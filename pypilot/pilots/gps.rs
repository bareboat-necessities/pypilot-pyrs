use std::collections::HashMap;
use std::*;

use pilot::{AutopilotGain, AutopilotPilot};
use pypilot::autopilot::{resolv, HeadingOffset};
use pypilot::values::*;
const disabled: _ = true;
struct GPSPilot {
    wind_gps_offset: ST0,
    true_wind_gps_offset: ST1,
    gains: HashMap<_, _>,
}

impl GPSPilot {
    fn __init__<T0>(&self, ap: T0) {
        super(GPSPilot, self).__init__("gps", ap);
        self.wind_gps_offset = HeadingOffset();
        self.true_wind_gps_offset = HeadingOffset();
        self.gains = HashMap::new();
        self.PosGain("P", 0.003, 0.02);
        self.PosGain("D", 0.1, 1.0);
        self.PosGain("DD", 0.05, 1.0);
        self.PosGain("FF", 0.6, 3.0);
        self.wind_gps_offset = HeadingOffset();
        self.true_wind_gps_offset = HeadingOffset();
    }
    fn compute_heading(&self) {
        let ap = self.ap;
        let sensors = ap.sensors;
        let gps_course = ap.sensors.gps.track.value;
        if sensors.wind.source.value != "none" {
            let mut offset = resolv(
                (sensors.wind.wdirection + gps_course),
                self.wind_gps_offset.value,
            );
            self.wind_gps_offset.update(offset, sensors.wind.wfactor);
        }
        if sensors.truewind.source.value != "none" {
            let mut offset = resolv(
                (sensors.truewind.wdirection + gps_course),
                self.true_wind_gps_offset.value,
            );
            self.true_wind_gps_offset
                .update(offset, sensors.truewind.wfactor);
        }
        let mode = ap.mode.value;
        if mode == "compass" {
            let compass = ap.boatimu.SensorValues["heading_lowpass"].value;
            ap.heading.set(compass);
        }
        if mode == "gps" {
            ap.heading.set(gps_course);
        } else {
            if mode == "wind" {
                let wind = resolv((self.wind_gps_offset.value - gps_course), 180);
                ap.heading.set(wind);
            } else {
                if mode == "true wind" {
                    let true_wind = resolve((self.true_wind_gps_offset.value - gps_course), 180);
                    ap.heading.set(true_wind);
                }
            }
        }
    }
    fn best_mode<T0, RT>(&self, mode: T0) -> RT {
        let sensors = self.ap.sensors;
        let gps_speed = sensors.gps.speed.value;
        let nogps = sensors.gps.source.value == "none" || gps_speed < 1.2;
        let nowind = sensors.wind.source.value == "none";
        if nogps {
            return "compass";
        } else {
            if mode == "compass" || nowind {
                return "gps";
            }
        }
        return mode;
    }
    fn process(&self) {
        let ap = self.ap;
        let headingrate = ap.boatimu.SensorValues["headingrate_lowpass"].value;
        let headingraterate = ap.boatimu.SensorValues["headingraterate_lowpass"].value;
        let gain_values = [
            ("P", ap.heading_error.value),
            ("D", headingrate),
            ("DD", headingraterate),
            ("FF", ap.heading_command_rate.value),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        let command = self.Compute(gain_values);
        if ap.enabled.value {
            ap.servo.command.set(command);
        }
    }
}
const pilot: _ = GPSPilot;
