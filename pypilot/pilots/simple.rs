use std::collections::HashMap;
use std::*;

use pilot::AutopilotPilot;
const disabled: _ = true;
struct SimplePilot {
    gains: HashMap<_, _>,
}

impl SimplePilot {
    fn __init__<T0>(&self, ap: T0) {
        super(SimplePilot, self).__init__("simple", ap);
        self.gains = HashMap::new();
        self.Gain("P", 0.005, 0, 0.025);
        self.Gain("I", 0, 0, 0.05);
        self.Gain("D", 0.15, 0, 0.5);
    }
    fn process<T0>(&self, reset: T0) {
        let ap = self.ap;
        let headingrate = ap.boatimu.SensorValues["headingrate_lowpass"].value;
        let gain_values = [
            ("P", ap.heading_error.value),
            ("I", ap.heading_error_int.value),
            ("D", headingrate),
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
const pilot: _ = SimplePilot;
