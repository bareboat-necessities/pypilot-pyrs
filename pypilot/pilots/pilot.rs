use std::collections::HashMap;
use std::*;

use pypilot::resolv::resolv;
use pypilot::values::*;
struct AutopilotGain {}

impl AutopilotGain {
    fn __init__(&self) {
        super(AutopilotGain, self).__init__(starred!(cargs) /*unsupported*/, true);
        self.info["AutopilotGain"] = true;
    }
}
struct AutopilotPilot {
    name: ST0,
    ap: ST1,
}

impl AutopilotPilot {
    fn __init__<T0, T1>(&self, name: T0, ap: T1) {
        super(AutopilotPilot, self).__init__();
        self.name = name;
        self.ap = ap;
    }
    fn register<T0, T1, RT>(&self, _type: T0, name: T1) -> RT {
        return self.ap.client.register(_type(starred!((vec![((("ap.pilot." + self.name) + ".") + name)] + args.collect::<Vec<_>>()))/*unsupported*/, kwargs));
    }
    fn Gain<T0, T1, T2, T3, T4>(
        &self,
        name: T0,
        default: T1,
        min_val: T2,
        max_val: T3,
        compute: T4,
    ) {
        if !compute {
            compute = |value| (value * self.gains[name]["apgain"].value);
        }
        self.gains[name] = [
            (
                "apgain",
                self.register(AutopilotGain, name, default, min_val, max_val),
            ),
            ("sensor", self.register(SensorValue, (name + "gain"))),
            ("compute", compute),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
    }
    fn PosGain<T0, T1, T2>(&self, name: T0, default: T1, max_val: T2) {
        self.Gain(name, default, 0, max_val);
    }
    fn Compute<T0, RT>(&self, gain_values: T0) -> RT {
        let mut command = 0;
        for gain in self.gains {
            let value = gain_values[gain];
            let gains = self.gains[gain];
            gains["sensor"].set(gains["compute"](value));
            command += gains["sensor"].value;
        }
        return command;
    }
    fn compute_heading(&self) {
        let ap = self.ap;
        let compass = ap.boatimu.SensorValues["heading_lowpass"].value;
        if ap.mode.value == "true wind" {
            let true_wind = resolv((ap.true_wind_compass_offset.value - compass));
            ap.heading.set(true_wind);
        } else {
            if ap.mode.value == "wind" {
                let wind = resolv((ap.wind_compass_offset.value - compass));
                ap.heading.set(wind);
            } else {
                if ap.mode.value == "gps" {
                    let gps = resolv((compass + ap.gps_compass_offset.value), 180);
                    ap.heading.set(gps);
                } else {
                    if ap.mode.value == "compass" {
                        ap.heading.set(compass);
                    }
                }
            }
        }
    }
    fn best_mode<T0, RT>(&self, mode: T0) -> RT {
        let sensors = self.ap.sensors;
        let nowind = sensors.wind.source.value == "none";
        let notruewind = sensors.truewind.source.value == "none";
        let nogps = sensors.gps.source.value == "none";
        let nowater = sensors.water.source.value == "none";
        if mode == "true wind" && notruewind {
            mode = "wind";
        }
        if mode == "wind" && nowind {
            return "compass";
        }
        if mode == "gps" && nogps {
            return "compass";
        }
        return mode;
    }
}
