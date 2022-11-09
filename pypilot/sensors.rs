use std::collections::HashMap;
use std::*;

use client::*;
use gps_filter::*;
use gpsd::gpsd;
use resolv::resolv;
use values::*;
const source_priority: _ = [
    ("gpsd", 1),
    ("servo", 1),
    ("serial", 2),
    ("tcp", 3),
    ("signalk", 4),
    ("gps+wind", 5),
    ("water+wind", 6),
    ("none", 7),
]
.iter()
.cloned()
.collect::<HashMap<_, _>>();
struct Sensor {
    source: ST0,
    rate: ST1,
    lastupdate: ST2,
    device: Option<_>,
    name: ST3,
    client: ST4,
}

impl Sensor {
    fn __init__<T0, T1>(&self, client: T0, name: T1) {
        self.source = client.register(StringValue((name + ".source"), "none"));
        if name != "apb" {
            self.rate = client.register(RangeProperty((name + ".rate"), 4, 0, 50));
        }
        self.lastupdate = 0;
        self.device = None;
        self.name = name;
        self.client = client;
    }
    fn write<T0, T1, RT>(&self, data: T0, source: T1) -> RT {
        if source_priority[self.source.value] < source_priority[source] {
            return false;
        }
        if source_priority[self.source.value] == source_priority[source]
            && data["device"] != self.device
        {
            return false;
        }
        self.update(data);
        if self.source.value != source {
            println!(
                "{:?} {:?} {:?} {:?} {:?} ",
                _("sensor found"),
                self.name,
                source,
                data["device"],
                time.asctime(time.localtime(time.time()))
            );
            self.source.set(source);
            self.device = data["device"];
        }
        self.lastupdate = time.monotonic();
        return true;
    }
    fn reset(&self) {
        raise!("reset should be overloaded"); //unsupported
    }
    fn update<T0>(&self, data: T0) {
        raise!("update should be overloaded"); //unsupported
    }
    fn register<T0, T1, RT>(&self, _type: T0, name: T1) -> RT {
        return self.client.register(_type(
            starred!((vec![((self.name + ".") + name)] + args.collect::<Vec<_>>())), /*unsupported*/
            kwargs,
        ));
    }
}
struct BaseWind {
    boatimu: ST0,
    direction: ST1,
    speed: ST2,
    offset: ST3,
    compensation_height: ST4,
    wspeed: ST5,
    wdirection: ST6,
    wfactor: ST7,
}

impl BaseWind {
    fn __init__<T0, T1, T2>(&self, client: T0, name: T1, boatimu: T2) {
        super(BaseWind, self).__init__(client, name);
        self.boatimu = boatimu;
        self.direction = self.register(SensorValue, "direction", true);
        self.speed = self.register(SensorValue, "speed");
        self.offset = self.register(RangeSetting, "offset", 0, -180, 180, "deg");
        self.compensation_height = self.register(RangeProperty, "sensors_height", 0, 0, 100, true);
        self.wspeed = 0;
        self.wdirection = 0;
        self.wfactor = 0;
    }
    fn update<T0>(&self, data: T0) {
        if data.iter().any(|&x| x == "direction") {
            data["direction"] += self.offset.value;
        }
        if self.compensation_height.value
            && data.iter().any(|&x| x == "direction")
            && data.iter().any(|&x| x == "speed")
            && self.boatimu
        {
            let speed = data["speed"];
            let direction = data["direction"];
            let mut dx = (speed * math.sin(math.radians(direction)));
            let mut dy = (speed * math.cos(math.radians(direction)));
            let m = math.radians(self.compensation_height.value);
            dx -= (m * self.boatimu.SensorValues["rollrate"].value);
            dy -= (m * self.boatimu.SensorValues["pitchrate"].value);
            data["speed"] = math.hypot(dx, dy);
            data["direction"] = math.degrees(math.atan2(dx, dy));
        }
        if data.iter().any(|&x| x == "direction") {
            self.direction.set(resolv(data["direction"]));
        }
        if data.iter().any(|&x| x == "speed") {
            self.speed.set(data["speed"]);
        }
        self.weight();
    }
    fn reset(&self) {
        self.direction.set(false);
        self.speed.set(false);
    }
    fn weight(&self) {
        let mut d = 0.005;
        let wspeed = self.speed.value;
        self.wspeed = (((1 - d) * self.wspeed) + (d * wspeed));
        d = (0.05 * math.log(((wspeed / 5.0) + 1.2)));
        let mut wdirection = resolv(self.direction.value, self.wdirection);
        wdirection = (((1 - d) * self.wdirection) + (d * wdirection));
        self.wdirection = resolv(wdirection);
        self.wfactor = d;
    }
}
struct Wind {}

impl Wind {
    fn __init__<T0, T1>(&self, client: T0, boatimu: T1) {
        super(Wind, self).__init__(client, "wind", boatimu);
    }
}
struct TrueWind {
    wdirection: ST0,
    wfactor: ST1,
    lastupdate: ST2,
}

impl TrueWind {
    fn __init__<T0, T1>(&self, client: T0, boatimu: T1) {
        super(TrueWind, self).__init__(client, "truewind", boatimu);
    }
    fn compute_true_wind<T0, T1, T2, RT>(
        water_speed: T0,
        wind_speed: T1,
        wind_direction: T2,
    ) -> RT {
        let rd = math.radians(wind_direction);
        let windv = (
            (wind_speed * math.sin(rd)),
            ((wind_speed * math.cos(rd)) - water_speed),
        );
        return math.hypot(starred!(windv) /*unsupported*/);
    }
    fn compute_true_wind_speed<T0, T1, T2, RT>(
        water_speed: T0,
        wind_speed: T1,
        wind_direction: T2,
    ) -> RT {
        let rd = math.radians(wind_direction);
        let windv = (
            (wind_speed * math.sin(rd)),
            ((wind_speed * math.cos(rd)) - water_speed),
        );
        return math.hypot(starred!(windv) /*unsupported*/);
    }
    fn update_from_apparent<T0, T1, T2>(&self, boat_speed: T0, wind_speed: T1, wind_direction: T2) {
        if self.source.value == "water+wind" || self.source.value == "gps+wind" {
            self.direction.set(TrueWind::compute_true_wind(
                boat_speed,
                wind_speed,
                wind_direction,
            ));
            self.wdirection = self.direction.value;
            self.wfactor = 0.05;
            self.lastupdate = time.monotonic();
        }
    }
}
struct APB {
    track: ST0,
    xte: ST1,
    gain: ST2,
    last_time: ST3,
}

impl APB {
    fn __init__<T0>(&self, client: T0) {
        super(APB, self).__init__(client, "apb");
        self.track = self.register(SensorValue, "track", true);
        self.xte = self.register(SensorValue, "xte");
        self.gain = self.register(RangeProperty, "xte.gain", 300, 0, 3000, true);
        self.last_time = time.monotonic();
    }
    fn reset(&self) {
        self.xte.update(0);
    }
    fn update<T0>(&self, data: T0) {
        let t = time.monotonic();
        if (t - self.last_time) < 0.5 {
            return;
        }
        self.last_time = t;
        self.track.update(data["track"]);
        if data.iter().any(|&x| x == "xte") {
            xte = data["xte"];
            self.xte.update(xte);
        } else {
            xte = 0;
        }
        if !self.client.values.values["ap.enabled"].value {
            return;
        }
        let data_mode = if data.iter().any(|&x| x == "mode") {
            data["mode"]
        } else {
            "gps"
        };
        let mode = self.client.values.values["ap.mode"];
        if mode.value != data_mode {
            if data.iter().any(|&x| x == "senderid") && data["senderid"] != "GP" {
                mode.set(data_mode);
            } else {
                return;
            }
        }
        let command = (data["track"] + (self.gain.value * xte));
        let heading_command = self.client.values.values["ap.heading_command"];
        if abs((heading_command.value - command)) > 0.1 {
            heading_command.set(command);
        }
    }
}
struct gps {
    track: ST0,
    speed: ST1,
    fix: ST2,
    leeway_ground: ST3,
    compass_error: ST4,
    filtered: ST5,
    lastpredictt: ST6,
}

impl gps {
    fn __init__<T0>(&self, client: T0) {
        super(gps, self).__init__(client, "gps");
        self.track = self.register(SensorValue, "track", true);
        self.speed = self.register(SensorValue, "speed");
        self.fix = self.register(JSONValue, "fix", false);
        self.leeway_ground = self.register(SensorValue, "leeway_ground");
        self.compass_error = self.register(SensorValue, "compass_error");
        self.filtered = GPSFilterProcess(client);
        self.lastpredictt = time.monotonic();
        self.rate.set(1.0);
    }
    fn update<T0>(&self, data: T0) {
        if data.iter().any(|&x| x == "fix") {
            data.update(data["fix"]);
            data["fix"].drop();
        }
        self.speed.set(data["speed"]);
        if data.iter().any(|&x| x == "track") {
            self.track.set(data["track"]);
        }
        self.fix.set(data);
        self.filtered.update(data, time.monotonic());
    }
    fn predict<T0>(&self, ap: T0) {
        if self.source.value == "none" {
            return;
        }
        let accel = ap.boatimu.SensorValues["accel"].value;
        let fusionQPose = ap.boatimu.SensorValues["fusionQPose"].value;
        if accel && fusionQPose {
            self.filtered.predict(accel, fusionQPose, time.monotonic());
        }
    }
    fn reset(&self) {
        self.track.set(false);
        self.speed.set(false);
    }
}
struct Water {
    speed: ST0,
    leeway: ST1,
    last_leeway_measurement: ST2,
    current_speed: ST3,
    current_direction: ST4,
    water_wind_speed: ST5,
    water_wind_direction: ST6,
}

impl Water {
    fn __init__<T0>(&self, client: T0) {
        super(Water, self).__init__(client, "water");
        self.speed = self.register(SensorValue, "speed");
        self.leeway = self.register(SensorValue, "leeway");
        self.leeway.source = self.register(Value, "leeway.source", "none");
        self.last_leeway_measurement = 0;
        self.current_speed = self.register(SensorValue, "current.speed");
        self.current_direction = self.register(SensorValue, "current.direction");
        self.water_wind_speed = self.register(SensorValue, "wind.speed");
        self.water_wind_direction = self.register(SensorValue, "wind.direction");
    }
    fn update<T0>(&self, data: T0) {
        let t = time.monotonic();
        if data.iter().any(|&x| x == "leeway") {
            self.leeway.set(data["leeway"]);
            self.leeway_source.update("sensor");
            self.last_leeway_measurement = t;
        }
        if data.iter().any(|&x| x == "speed") {
            self.speed.set(data["speed"]);
        }
    }
    fn compute<T0>(&self, ap: T0) {
        if self.source.value == "none" {
            self.leeway.source.update("none");
            return;
        }
        return;
        let t = time.monotonic();
        if (t - self.last_leeway_measurement) > 3 {
            let heel = ap.boatimu.heel;
            let K = 5;
            let spd2 = self.speed.value.pow(2);
            if spd2 > 2 {
                self.leeway.set(((K * heel) / spd2));
            }
            self.leeway.source.update("computed");
        }
        let gps = ap.sensors.gps;
        if gps::source.value != "none" {
            let speed = gps::filtered.speed;
            let rtrack = math.radians(gps::filtered.track);
            let vg_north = (speed * math.cos(rtrack));
            let vg_east = (speed * math.sin(rtrack));
            let mut heading = ap.boatimu.SensorValues["heading_lowpass"].value;
            let mut declination = gps::filtered.declination.value;
            let mut compass_error = gps::filtered.compass_error.value;
            let direction_true = (((heading + declination) + compass_error) + self.leeway.value);
            let rdirection = math.radians(direction_true);
            let water_speed = self.speed.value;
            let vw_north = (water_speed * math.cos(rdirection));
            let vw_east = (water_speed * math.sin(rdirection));
            let c_north = (vg_north - vw_north);
            let c_east = (vg_east - vw_east);
            self.current_speed.set(math.hypot(c_north, c_east));
            self.current_direction
                .set(resolv(math.degrees(math.atan2(c_north, c_east)), 180));
        }
        let wind = ap.sensors.wind;
        if wind.source.value != "none" {
            let awa = wind.direction.value;
            let aws = wind.speed.value;
            let mut heading = ap.boatimu.SensorValues["heading_lowpass"].value;
            let mut declination = gps::filtered.declination.value;
            let mut compass_error = gps::filtered.compass_error.value;
            let ra = math.radians((awa - self.leeway.value));
            let vya = ((aws * math.cos(ra)) - self.speed.value);
            let vxa = (aws * math.sin(ra));
            self.water_wind_speed.set(math.hypot(vya, vxa));
            self.water_wind_direction
                .set(math.degrees(math.atan2(vya, vxa)));
        }
    }
    fn reset(&self) {
        self.direction.set(false);
        self.speed.set(false);
    }
}
struct Sensors {
    client: ST0,
    nmea: ST1,
    signalk: ST2,
    gpsd: ST3,
    gps: ST4,
    wind: ST5,
    truewind: ST6,
    rudder: ST7,
    apb: ST8,
    water: ST9,
    sensors: ST10,
}

impl Sensors {
    fn __init__<T0, T1>(&self, client: T0, boatimu: T1) {
        use nmea::Nmea;
        use rudder::Rudder;
        use signalk::signalk;
        self.client = client;
        self.nmea = Nmea(self);
        self.signalk = signalk(self);
        self.gpsd = gpsd(self);
        self.gps = gps(client);
        self.wind = Wind(client, boatimu);
        self.truewind = TrueWind(client, boatimu);
        self.rudder = Rudder(client);
        self.apb = APB(client);
        self.water = Water(client);
        self.sensors = [
            ("gps", self.gps),
            ("wind", self.wind),
            ("truewind", self.truewind),
            ("rudder", self.rudder),
            ("apb", self.apb),
            ("water", self.water),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
    }
    fn poll(&self) {
        self.nmea.poll();
        self.signalk.poll();
        self.gpsd.poll();
        self.rudder.poll();
        let t = time.monotonic();
        for name in self.sensors {
            let sensor = self.sensors[name];
            if sensor.source.value == "none" {
                continue;
            }
            if (t - sensor.lastupdate) > 8 {
                self.lostsensor(sensor);
            }
        }
    }
    fn lostsensor<T0>(&self, sensor: T0) {
        println!(
            "{:?} {:?} {:?} {:?} {:?} ",
            "sensor lost",
            sensor.name,
            sensor.source.value,
            sensor.device,
            time.asctime(time.localtime(time.time()))
        );
        sensor.source.set("none");
        sensor.reset();
        sensor.device = None;
    }
    fn lostgpsd(&self) {
        if self.gps.source.value == "gpsd" {
            self.lostsensor(self.gps);
        }
    }
    fn write<T0, T1, T2>(&self, sensor: T0, data: T1, source: T2) {
        if !self.sensors.iter().any(|&x| x == sensor) {
            println!("{:?} {:?} ", _("unknown data parsed!"), sensor);
            return;
        }
        self.sensors[sensor].write(data, source);
    }
    fn lostdevice<T0>(&self, device: T0) {
        for name in self.sensors {
            let sensor = self.sensors[name];
            if sensor.device && sensor.device[2..] == device {
                self.lostsensor(sensor);
            }
        }
    }
}
