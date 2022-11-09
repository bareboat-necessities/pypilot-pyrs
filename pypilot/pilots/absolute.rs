use std::collections::HashMap;
use std::*;

use pilot::AutopilotPilot;
struct AbsolutePilot {
    gains: HashMap<_, _>,
}

impl AbsolutePilot {
    fn __init__<T0>(&self, ap: T0) {
        super(AbsolutePilot, self).__init__("absolute", ap);
        self.gains = HashMap::new();
        self.PosGain("P", 0.05, 2);
        self.PosGain("I", 0, 0.05);
        self.PosGain("D", 0.2, 2);
        self.PosGain("DD", 0, 1);
    }
    fn process(&self) {
        let ap = self.ap;
        if type_(ap.sensors.rudder.angle.value) == type_(false) {
            ap.pilot.set("basic");
            return;
        }
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
        let command = self.Compute(gain_values);
        if ap.enabled.value {
            ap.servo.position_command.set(command);
        }
    }
}
const pilot: _ = AbsolutePilot;
