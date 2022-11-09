use std::collections::HashMap;
use std::*;

use pypilot::values::*;
use sensors::Sensor;

struct Rudder {
    angle: ST0,
    speed: ST1,
    last: ST2,
    last_time: ST3,
    offset: ST4,
    scale: ST5,
    nonlinearity: ST6,
    calibration_state: ST7,
    calibration_raw: HashMap<_, _>,
    range: ST8,
    lastrange: ST9,
    minmax: ST10,
    autogain_state: ST11,
    raw: ST12,
    lastdevice: bool,
    autogain_movetime: ST13,
    autogain_time: ST14,
}

impl Rudder {
    fn __init__<T0>(&self, client: T0) {
        super(Rudder, self).__init__(client, "rudder");
        self.angle = self.register(SensorValue, "angle");
        self.speed = self.register(SensorValue, "speed");
        self.last = 0;
        self.last_time = time.monotonic();
        self.offset = self.register(Value, "offset", 0.0, true);
        self.scale = self.register(Value, "scale", 100.0, true);
        self.nonlinearity = self.register(Value, "nonlinearity", 0.0, true);
        self.calibration_state = self.register(
            EnumProperty,
            "calibration_state",
            "idle",
            vec![
                "idle",
                "reset",
                "centered",
                "starboard range",
                "port range",
                "auto gain",
            ],
        );
        self.calibration_raw = HashMap::new();
        self.range = self.register(RangeProperty, "range", 45, 10, 100, true);
        self.lastrange = 0;
        self.minmax = (-0.5, 0.5);
        self.autogain_state = "idle";
        self.raw = 0;
        self.lastdevice = false;
    }
    fn update_minmax(&self) {
        let mut scale = self.scale.value;
        let mut offset = self.offset.value;
        let range = float(self.range.value);
        let oldminmax = self.minmax;
        self.minmax = (((-(range) - offset) / scale), ((range - offset) / scale));
        if self.lastrange && self.lastrange != self.range.value {
            let nonlinearity = self.nonlinearity.value;
            let (min, max) = oldminmax;
            let B = (scale - (nonlinearity * (min + max)));
            let C = (offset + ((nonlinearity * min) * max));
            let (min, max) = self.minmax;
            scale = (B + (nonlinearity * (min + max)));
            offset = (C - ((nonlinearity * min) * max));
            self.scale.update(scale);
            self.offset.update(offset);
        }
        self.lastrange = self.range.value;
    }
    fn calibration<T0>(&self, command: T0) {
        if command == "reset" {
            self.nonlinearity.update(0.0);
            self.scale.update(100.0);
            self.offset.update(0.0);
            self.update_minmax();
            self.calibration_raw = HashMap::new();
            return;
        } else {
            if command == "centered" {
                let mut true_angle = 0;
            } else {
                if command == "port range" {
                    true_angle = self.range.value;
                } else {
                    if command == "starboard range" {
                        true_angle = -(self.range.value);
                    } else {
                        println!("{:?} {:?} ", "unhandled rudder_calibration", command);
                        return;
                    }
                }
            }
        }
        self.calibration_raw[command] = [("raw", self.raw), ("rudder", true_angle)]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
        let mut scale = self.scale.value;
        let mut offset = self.offset.value;
        let mut nonlinearity = self.nonlinearity.value;
        let mut p = vec![];
        for c in vec!["starboard range", "centered", "port range"] {
            if self.calibration_raw.iter().any(|&x| x == c) {
                p.push(self.calibration_raw[c]);
            }
        }
        let l = p.len();
        if l == 1 {
            let rudder = p[0]["rudder"];
            let raw = p[0]["raw"];
            offset = ((rudder - (scale * raw))
                - ((nonlinearity * (self.minmax[0] - raw)) * (self.minmax[1] - raw)));
        } else {
            if l == 2 {
                let (rudder0, rudder1) = (p[0]["rudder"], p[1]["rudder"]);
                let (raw0, raw1) = (p[0]["raw"], p[1]["raw"]);
                if abs((raw1 - raw0)) > 0.001 {
                    scale = ((rudder1 - rudder0) / (raw1 - raw0));
                }
                offset = (rudder1 - (scale * raw1));
                nonlinearity = 0;
            }
        }
        if l == 3 {
            let (rudder0, rudder1, rudder2) = (p[0]["rudder"], p[1]["rudder"], p[2]["rudder"]);
            let (raw0, raw1, raw2) = (p[0]["raw"], p[1]["raw"], p[2]["raw"]);
            if abs((raw1 - raw0)).iter().min().unwrap() > 0.001 {
                scale = ((rudder2 - rudder0) / (raw2 - raw0));
                offset = (rudder0 - (scale * raw0));
                nonlinearity =
                    ((((rudder1 - (scale * raw1)) - offset) / (raw0 - raw1)) / (raw2 - raw1));
            } else {
                println!(
                    "{:?} {:?} ",
                    _("bad rudder calibration"),
                    self.calibration_raw
                );
            }
        }
        if abs(scale) <= 0.01 {
            println!(
                "{:?} {:?} {:?} ",
                _("bad servo rudder calibration"),
                scale,
                nonlinearity
            );
            while self.calibration_raw.len() > 1 {
                for c in self.calibration_raw {
                    if c != command {
                        self.calibration_raw[c].drop();
                        break;
                    }
                }
            }
            return;
        }
        self.offset.update(offset);
        self.scale.update(scale);
        self.nonlinearity.update(nonlinearity);
        self.update_minmax();
    }
    fn invalid<RT>(&self) -> RT {
        return type_(self.angle.value) == type_(false);
    }
    fn poll(&self) {
        if self.lastrange != self.range.value {
            self.update_minmax();
        }
        if self.calibration_state.value == "idle" {
            return;
        }
        if self.calibration_state.value == "auto gain" {
            return;
            fn idle() {
                self.autogain_state = "idle";
                self.calibration_state.set("idle");
            }
            let t = time.monotonic();
            if self.autogain_state == "idle" {
                self.gain.set(1);
                self.autogain_state = "fwd";
                self.autogain_movetime = t;
            }
            if type_(self.value) == type_(false) {
                idle();
            }
            let rng = self.range.value;
            if self.autogain_state == "fwd" {
                self.command.set(1);
                if abs(self.angle.value) >= rng {
                    self.autogain_state = "center";
                    self.autogain_time = t;
                }
            }
            if self.autogain_state == "center" {
                self.command.set(-1);
                if abs(self.angle.value) < (rng - 1) {
                    self.autogain_state = "rev";
                }
            }
            if self.autogain_state == "rev" {
                self.command.set(-1);
                if abs(self.value) >= rng {
                    let dt = (time.monotonic() - self.autogain_time);
                    let mut gain = ((5 * dt) / rng).iter().max().unwrap().iter().min().unwrap();
                    if self.angle.value < 0 {
                        gain = -(gain);
                    }
                    self.gain.set(gain);
                    idle();
                }
            }
            if self.current.value {
                self.autogain_movetime = t;
            }
            if (t - self.autogain_movetime) > 3 {
                println!("{:?} ", _("servo rudder autogain failed"));
                idle();
            }
        } else {
            self.calibration(self.calibration_state.value);
            self.calibration_state.set("idle");
        }
    }
    fn update<T0>(&self, data: T0) {
        if !data {
            self.angle.update(false);
            return;
        }
        if data["device"] != self.lastdevice {
            self.lastdevice = data["device"];
            self.angle.update(false);
            return;
        }
        self.raw = data["angle"];
        if math.isnan(self.raw) {
            self.angle.update(false);
            return;
        }
        let scale = self.scale.value;
        let offset = self.offset.value;
        let nonlinearity = self.nonlinearity.value;
        let raw = self.raw;
        let mut angle = (((scale * raw) + offset)
            + ((nonlinearity * (self.minmax[0] - raw)) * (self.minmax[1] - raw)));
        angle = round(angle, 2);
        self.angle.set(angle);
        let t = time.monotonic();
        let mut dt = (t - self.last_time);
        if dt > 1 {
            dt = 1;
        }
        if dt > 0 {
            let speed = ((self.angle.value - self.last) / dt);
            self.last_time = t;
            self.last = self.angle.value;
            self.speed.set(((0.9 * self.speed.value) + (0.1 * speed)));
        }
    }
    fn reset(&self) {
        self.angle.set(false);
    }
}
