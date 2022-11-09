use std::collections::HashMap;
use std::*;

use pilot::AutopilotPilot;
const disabled: _ = true;
use pypilot::values::*;
use resolv::resolv;
struct vmgTable {
    t: ST0,
    command: ST1,
    table: HashMap<_, _>,
    previous: ST2,
    previous_table: HashMap<_, _>,
    current_command_time: ST3,
}

impl vmgTable {
    fn __init__(&self) {
        self.t = 0;
        self.reset();
    }
    fn reset(&self) {
        self.command = 400;
        self.table = HashMap::new();
        self.previous = 400;
        self.previous_table = HashMap::new();
    }
    fn update_time(&self) {
        let t = time.monotonic();
        if (t - self.t) > 0.5 {
            self.reset();
        }
        self.t = t;
    }
    fn add_measurement<T0, T1, T2, T3>(&self, heading: T0, command: T1, speed: T2, track: T3) {
        let mut t = time.monotonic();
        let dt = (t - self.current_command_time);
        if dt < 60 {
            return;
        }
        if command != self.command {
            self.table = HashMap::new();
            let diff = resolv((command - self.command));
            if diff > 60 {
                self.previous = self.command;
                self.previous_table = self.table;
            }
            self.current_command_time = t;
        }
        let headingi = i32::from(round(heading));
        let rtrack = math.radians(track);
        let vn = (speed * math.cos(rtrack));
        let ve = (speed * math.sin(rtrack));
        t = time.monotonic();
        if !self.table.iter().any(|&x| x == headingi) {
            let (vnt, vet, count, tt) = (vn, ve, 1, t);
        } else {
            let (vnt, vet, count, tt) = self.table[headingi];
            if (t - tt) > 30 {
                let (vnt, vet, count, tt) = (vn, ve, 1, t);
            }
        }
        let vnt = ((vn / (count + 1)) + (vnt / count));
        let vet = ((ve / (count + 1)) + (vet / count));
        count += 1;
        self.table[headingi] = (vnt, vet, count, t);
    }
    fn updated_command<T0>(&self, heading_command: T0) {
        let t = time.monotonic();
        let dt = (t - self.current_command_time);
        if dt > 120 {
            if self.table && self.previous_table { /*pass*/ }
        }
    }
}
struct VMGPilot {
    gains: HashMap<_, _>,
    vmg: HashMap<_, _>,
    noise: ST0,
    accel: ST1,
}

impl VMGPilot {
    fn __init__<T0, T1>(&self, ap: T0, name: T1) {
        super(VMGPilot, self).__init__(name, ap);
        self.gains = HashMap::new();
        self.PosGain("P", 0.003, 0.02);
        self.PosGain("D", 0.09, 0.8);
        self.PosGain("DD", 0.075, 0.8);
        self.vmg = HashMap::new();
        for mode in ap.mode.info["choices"] {
            self.vmg[mode] = vmgTable();
        }
        self.noise = 1;
    }
    fn process(&self) {
        let t = time.monotonic();
        let ap = self.ap;
        let vmg = self.vmg[ap.mode.value];
        vmg.update_time();
        if ap.sensors.wind.source.value == "none" {
            ap.pilot.set("basic");
            return;
        }
        let headingrate = ap.boatimu.SensorValues["headingrate_lowpass"].value;
        let headingraterate = ap.boatimu.SensorValues["headingraterate_lowpass"].value;
        let gain_values = [
            ("P", ap.heading_error.value),
            ("D", headingrate),
            ("DD", headingraterate),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        let command = self.Compute(gain_values);
        if !ap.enabled.value {
            return;
        }
        ap.servo.command.set(command);
        let accel = ap.boatimu.SensorValues["accel"].value;
        let noise = vector.dist(accel, self.accel);
        self.accel = accel;
        self.noise = ((0.1 * noise) + (0.9 * self.noise));
        if self.noise < 0.01 {
            vmg.add_measurement(
                ap.heading.value,
                ap.heading_comand.value,
                ap.sensors.gps.filtered.speed.value,
                ap.sensors.gps.filtered.track.value,
            );
        }
        vmg.update_command(ap.heading_command);
    }
}
const pilot: _ = VMGPilot;
