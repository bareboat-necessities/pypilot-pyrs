use std::collections::HashMap;
use std::*;

use resolv::*;
use values::*;

struct TackSensorLog {
    log: Vec<_>,
    time: ST0,
    threshold: ST1,
}

impl TackSensorLog {
    fn __init__<T0>(&self, threshold: T0) {
        self.log = vec![];
        self.time = time.monotonic();
        self.threshold = threshold;
    }
    fn update<T0, RT>(&self, value: T0) -> RT {
        let t = time.monotonic();
        let dt = (t - self.time);
        if dt < 0.25 {
            return;
        }
        self.time = t;
        if dt > 1 {
            self.log = vec![];
            return;
        }
        if self.log.len() < 20 {
            self.log.append(value);
            return;
        }
        self.log = (self.log[1..] + vec![value]);
        let (port, starboard) = (true, true);
        let mut avg = 0;
        for d in self.log {
            if d <= self.threshold {
                let starboard = false;
            }
            if d >= -(self.threshold) {
                let port = false;
            }
            avg += d;
        }
        avg /= self.log.len();
        if starboard {
            return "starboard";
        }
        if port {
            return "port";
        }
    }
}

struct TackDirection {
    auto: bool,
}

impl TackDirection {
    fn __init__<T0>(&self, name: T0) {
        super(TackDirection, self).__init__(name, "port", vec!["port", "starboard"]);
        self.auto = true;
    }
    fn set<T0>(&self, value: T0) {
        super(TackDirection, self).set(value);
        self.auto = false;
    }
    fn auto_update<T0>(&self, value: T0) {
        if self.auto && self.value != value {
            super(TackDirection, self).set(value);
        }
    }
    fn toggle(&self) {
        super(TackDirection, self).set(if self.value == "starboard" {
            "port"
        } else {
            "starboard"
        });
        self.auto = true;
    }
}

struct Tack {
    ap: ST0,
    state: ST1,
    timeout: ST2,
    delay: ST3,
    angle: ST4,
    rate: ST5,
    threshold: ST6,
    count: ST7,
    direction: ST8,
    current_direction: ST9,
    time: ST10,
    wind_log: ST11,
    heel_log: ST12,
    tack_angle: ST13,
}

impl Tack {
    fn __init__<T0>(&self, ap: T0) {
        self.ap = ap;
        self.state = self.register(
            EnumProperty,
            "state",
            "none",
            vec!["none", "begin", "waiting", "tacking"],
        );
        self.timeout = self.register(Value, "timeout", 0);
        self.delay = self.register(RangeSetting, "delay", 0, 0, 60, "sec");
        self.angle = self.register(RangeSetting, "angle", 100, 10, 180, "deg");
        self.rate = self.register(RangeSetting, "rate", 15, 1, 100, "deg/s");
        self.threshold = self.register(RangeSetting, "threshold", 50, 10, 100, "%");
        self.count = self.register(ResettableValue, "count", 0, true);
        self.direction = self.register(TackDirection, "direction");
        self.current_direction = "port";
        self.time = time.monotonic();
        self.wind_log = TackSensorLog(12);
        self.heel_log = TackSensorLog(5);
        self.tack_angle = self.angle.value;
    }
    fn register<T0, T1, RT>(&self, _type: T0, name: T1) -> RT {
        return self.ap.client.register(_type(
            starred!((vec![("ap.tack." + name)] + args.collect::<Vec<_>>())), /*unsupported*/
            kwargs,
        ));
    }
    fn process<RT>(&self) -> RT {
        let t = time.monotonic();
        let ap = self.ap;
        if !ap.enabled.value {
            self.state.update("none");
            let mut r = false;
            if ap.sensors.wind.source.value != "none" {
                let mut d = resolv(ap.sensors.wind.direction.value);
                r = self.wind_log.update(d);
            } else {
                if (t - self.time) > 30 {
                    r = self.heel_log.update(ap.boatimu.heel);
                }
            }
            if r {
                self.direction.auto_update(r);
            }
            return;
        }
        if self.state.value == "begin" {
            self.time = t;
            self.current_direction = self.direction.value;
            self.state.update("waiting");
        }
        if self.state.value == "waiting" {
            let remaining = round((self.delay.value - (t - self.time)), 1);
            if remaining > 0 {
                self.timeout.set(remaining);
                return;
            }
            self.timeout.set(0);
            self.state.update("tacking");
            self.tack_angle = self.angle.value;
        }
        if self.state.value == "tacking" {
            let mut command = ap.heading_command.value;
            let heading = ap.boatimu.SensorValues["heading_lowpass"].value;
            let headingrate = ap.boatimu.SensorValues["headingrate_lowpass"].value;
            let headingraterate = ap.boatimu.SensorValues["headingraterate_lowpass"].value;
            if ap.mode.value.iter().any(|&x| x == "wind") {
                d = (0.5 - ((2 * heading) / command));
                tack_heading = -(command);
                direction = if command < 0 { 1 } else { -1 };
            } else {
                direction = if self.current_direction == "port" {
                    1
                } else {
                    -1
                };
                tack_heading = (command - (direction * self.tack_angle));
                d = ((direction * (command - resolv(heading, command))) / self.tack_angle);
            }
            if (100 * d) > self.threshold.value {
                self.state.update("none");
                self.direction.toggle();
                ap.heading_command.set(tack_heading);
                return;
            }
            command = (((headingrate + (headingraterate / 2)) / self.rate.value) + direction);
            command = command.iter().max().unwrap().iter().min().unwrap();
            ap.servo.command.set(command);
            return true;
        }
    }
}
