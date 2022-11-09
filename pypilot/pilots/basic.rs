use std::collections::HashMap;
use std::*;

use pilot::AutopilotPilot;
use pypilot::values::*;
use resolv::resolv;

struct BasicPilot {
    gains: HashMap<_, _>,
}

impl BasicPilot {
    fn __init__<T0, T1>(&self, ap: T0, name: T1) {
        super(BasicPilot, self).__init__(name, ap);
        self.gains = HashMap::new();
        self.PosGain("P", 0.003, 0.02);
        self.PosGain("I", 0, 0.1);
        self.PosGain("D", 0.09, 0.8);
        self.PosGain("DD", 0.075, 0.8);
        self.PosGain("PR", 0.005, 0.02);
        self.PosGain("FF", 0.6, 5.0);
    }
    fn process(&self) {
        let t = time.monotonic();
        let ap = self.ap;
        let headingrate = ap.boatimu.SensorValues["headingrate_lowpass"].value;
        let headingraterate = ap.boatimu.SensorValues["headingraterate_lowpass"].value;
        let gain_values = [
            ("P", ap.heading_error.value),
            ("I", ap.heading_error_int.value),
            ("D", headingrate),
            ("DD", headingraterate),
            ("FF", ap.heading_command_rate.value),
        ]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
        let mut PR = math.sqrt(abs(gain_values["P"]));
        if gain_values["P"] < 0 {
            PR = -(PR);
        }
        gain_values["PR"] = PR;
        let command = self.Compute(gain_values);
        if ap.enabled.value {
            ap.servo.command.set(command);
        }
    }
}

const pilot: _ = BasicPilot;
