use std::collections::HashMap;
use std::*;

use pilot::AutopilotPilot;
use pypilot::resolv::resolv;
const disabled: _ = true;
struct AutotunePilot {
    gains: HashMap<_, _>,
    p_search: ST0,
    d_search: ST1,
    search_angle: ST2,
    search_angle_change: ST3,
    search_time: ST4,
    search_count: ST5,
    cost: ST6,
    last_cost: ST7,
    last_cost_dt: ST8,
}

impl AutotunePilot {
    fn __init__<T0>(&self, ap: T0) {
        super(AutotunePilot, self).__init__("autotune", ap);
        self.gains = HashMap::new();
        self.PosGain("P", 0.003, 0.025);
        self.PosGain("D", 0.09, 0.5);
        self.PosGain("FF", 0.6, 3.0);
        self.p_search = (0.0015, 0.006, 0.0004);
        self.d_search = (0.05, 0.18, 0.01);
        self.search_angle = 0;
        self.search_angle_change = ((30 * 3.14) / 180);
        self.search_time = 0;
        self.search_count = 0;
        self.cost = 0;
        self.last_cost = 0;
        self.last_cost_dt = 0;
    }
    fn process(&self) {
        let ap = self.ap;
        let headingrate = ap.boatimu.SensorValues["headingrate_lowpass"].value;
        let headingraterate = ap.boatimu.SensorValues["headingraterate_lowpass"].value;
        let gain_values = [
            ("P", ap.heading_error.value),
            ("D", (headingrate + headingraterate)),
            ("FF", ap.heading_command_rate.value),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        let command = self.Compute(gain_values);
        if ap.enabled.value {
            ap.servo.command.set(command);
        } else {
            self.search_count = 0;
        }
        self.search_count += 1;
        let (P, D) = (self.gains["P"]["apgain"], self.gains["D"]["apgain"]);
        self.cost += ((ap.heading_error.value + (P.value * 4000)) + (D.value * 20));
        if self.search_count >= 600 {
            self.search_count = 0;
            let t = time.monotonic();
            let search_dt = (t - self.search_time);
            self.search_time = t;
            if search_dt < ((600 / self.boatimu.rate.value) * 1.05) {
                let cost_dt = (cost - self.last_cost);
                if cost_dt > 0 {
                    self.search_angle += 3.14;
                } else {
                    if cost_dt > self.last_cost_dt {
                        self.search_angle_change = -(self.search_angle_change);
                    }
                }
                self.last_cost_dt = cost_dt;
                self.search_angle += self.search_angle_change;
                self.search_angle = resolve(self.search_angle);
                let Pval = (P.value + (self.p_search[2] * math.sin(self.search_angle)));
                let Dval = (D.value + (self.d_search[2] * math.cos(self.search_angle)));
                P.set(Pval.iter().max().unwrap().iter().min().unwrap());
                D.set(Dval.iter().max().unwrap().iter().min().unwrap());
            }
            self.last_cost = cost;
            self.cost = 0;
        }
    }
}
const pilot: _ = AutotunePilot;
