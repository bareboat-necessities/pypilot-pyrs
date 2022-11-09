use std::*;
use std::collections::HashMap;

println!("{:?} {:?} ", "autopilot start", time.monotonic());
const pypilot_dir: _ = (os.getenv("HOME") + "/.pypilot/");
sys.path.append(os.path.dirname(os.path.abspath(__file__)));
use server::{pypilotServer};
use client::{pypilotClient};
use values::{*};
use boatimu::{*};
use resolv::{*};
use version::{strversion};
use sensors::{Sensors};

fn minmax<T0, T1, RT>(value: T0, r: T1) -> RT {
    return value.iter().max().unwrap().iter().min().unwrap();
}

struct ModeProperty {
    ap: bool,
}

impl ModeProperty {
    fn __init__<T0>(&self, name: T0) {
        self.ap = false;
        super(ModeProperty, self).__init__(name, "compass", vec!["compass", "gps", "wind", "true wind"], true);
    }
    fn set<T0>(&self, value: T0) {
        if self.ap {
            self.ap.preferred_mode.update(value);
        }
        self.set_internal(value);
    }
    fn set_internal<T0>(&self, value: T0) {
        super(ModeProperty, self).set(value);
    }
}

struct HeadingOffset {
    value: ST0,
}

impl HeadingOffset {
    fn __init__(&self) {
        self.value = 0;
    }
    fn update<T0, T1>(&self, offset: T0, d: T1) {
        offset = resolv(offset, self.value);
        self.value = resolv(((d * offset) + ((1 - d) * self.value)));
    }
}

struct HeadingProperty {
    mode: ST0,
}

impl HeadingProperty {
    fn __init__<T0, T1>(&self, name: T0, mode: T1) {
        self.mode = mode;
        super(HeadingProperty, self).__init__(name, 0, -180, 360);
    }
    fn set<T0>(&self, value: T0) {
        value = resolv(value, if self.mode.value.iter().any(|&x| x == "wind") { 0 } else { 180 });
        super(HeadingProperty, self).set(value);
    }
}

struct TimeStamp {}

impl TimeStamp {
    fn __init__(&self) {
        super(TimeStamp, self).__init__("timestamp", 0);
        self.info["type"] = "TimeStamp";
    }
}

struct TimedQueue {
    data: Vec<_>,
    length: ST0,
}

impl TimedQueue {
    fn __init__<T0>(&self, length: T0) {
        self.data = vec![];
        self.length = length;
    }
    fn add<T0>(&self, data: T0) {
        let t = time.monotonic();
        while self.data && self.data[0][1] < (t - self.length) {
            self.data = self.data[1..];
        }
        self.data.append((data, t));
    }
    fn take<T0, RT>(&self, t: T0) -> RT {
        while self.data && self.data[0][1] < t {
            self.data = self.data[1..];
        }
        if self.data {
            return self.data[0][0];
        }
        return 0;
    }
}

struct Autopilot {
    watchdog_device: bool,
    server: ST0,
    client: ST1,
    boatimu: ST2,
    sensors: ST3,
    servo: ST4,
    version: ST5,
    timestamp: ST6,
    starttime: ST7,
    mode: ST8,
    preferred_mode: ST9,
    lastmode: bool,
    dt: ST10,
    heading_command: ST11,
    enabled: ST12,
    lastenabled: bool,
    last_heading: bool,
    last_heading_off: ST13,
    heading: ST14,
    heading_error: ST15,
    heading_error_int: ST16,
    heading_error_int_time: ST17,
    heading_command_rate: ST18,
    pilots: HashMap<_, _>,
    pilot: ST19,
    tack: ST20,
    gps_compass_offset: ST21,
    gps_speed: ST22,
    wind_compass_offset: ST23,
    true_wind_compass_offset: ST24,
    runtime: ST25,
    timings: ST26,
    last_heading_mode: bool,
    lasttime: ST27,
    childprocesses: ST28,
    compass_change: ST29,
    last_heading_command: ST30,
}

impl Autopilot {
    fn __init__(&self) {
        super(Autopilot, self).__init__();
        self.watchdog_device = false;
        self.server = pypilotServer();
        self.client = pypilotClient(self.server);
        self.boatimu = BoatIMU(self.client);
        self.sensors = Sensors(self.client, self.boatimu);
        self.servo = servo.Servo(self.client, self.sensors);
        self.version = self.register(Value, "version", (("pypilot" + " ") + strversion));
        self.timestamp = self.client.register(TimeStamp());
        self.starttime = time.monotonic();
        self.mode = self.register(ModeProperty, "mode");
        self.preferred_mode = self.register(Value, "preferred_mode", "compass");
        self.lastmode = false;
        self.mode.ap = self;
        self.dt = 0;
        self.heading_command = self.register(HeadingProperty, "heading_command", self.mode);
        self.enabled = self.register(BooleanProperty, "enabled", false);
        self.lastenabled = false;
        self.last_heading = false;
        self.last_heading_off = self.boatimu.heading_off.value;
        self.heading = self.register(SensorValue, "heading", true);
        self.heading_error = self.register(SensorValue, "heading_error");
        self.heading_error_int = self.register(SensorValue, "heading_error_int");
        self.heading_error_int_time = time.monotonic();
        self.heading_command_rate = self.register(SensorValue, "heading_command_rate");
        self.heading_command_rate.time = 0;
        self.pilots = HashMap::new();
        for pilot_type in pilots.default {
            let try_dummy = { //unsupported
                let pilot = pilot_type(self);
                self.pilots[pilot.name] = pilot;
            };
            let except!(Exception) = { //unsupported
                println!("{:?} {:?} {:?} ",_("failed to load pilot"), pilot_type, e);
            };
        }
        let pilot_names = self.pilots.collect::<Vec<_>>();
        println!("{:?} {:?} ",(_("Available Pilots") + ":"), pilot_names);
        self.pilot = self.register(EnumProperty, "pilot", "basic", pilot_names, true);
        self.tack = tacking.Tack(self);
        self.gps_compass_offset = HeadingOffset();
        self.gps_speed = 0;
        self.wind_compass_offset = HeadingOffset();
        self.true_wind_compass_offset = HeadingOffset();
        self.runtime = self.register(TimeValue, "runtime");
        self.timings = self.register(SensorValue, "timings", false);
        self.last_heading_mode = false;
        let device = "/dev/watchdog0";
        let try_dummy = { //unsupported
            self.watchdog_device = open(device, "w");
        };
        let except!() = { //unsupported
            println!("{:?} {:?} {:?} ",_("warning: failed to open special file"), device, _("for writing"));
            println!("{:?} ",("         " + _("cannot stroke the watchdog")));
        };
        self.server.poll();
        if os.system(("sudo chrt -pf 1 %d 2>&1 > /dev/null" % os.getpid())) {
            println!("{:?} ",_("warning: failed to make autopilot process realtime"));
        }
        self.lasttime = time.monotonic();
        self.childprocesses = vec![self.boatimu.imu, self.boatimu.auto_cal, self.sensors.nmea, self.sensors.gpsd, self.sensors.gps.filtered, self.sensors.signalk, self.server];
        fn cleanup<T0, T1>(signal_number: T0, frame: T1) {
            if signal_number == signal.SIGCHLD {
                let mut pid = os.waitpid(-1, os.WNOHANG);
            }
            if signal_number != "atexit" {
                signal.signal(signal_number, signal.SIG_IGN);
            }
            while self.childprocesses {
                let process = self.childprocesses.pop().process;
                if process {
                    let mut pid = process.pid;
                    let try_dummy = { //unsupported
                        os.kill(pid, signal.SIGTERM);
                    };
                    let except!(Exception) = { //unsupported
                        /*pass*/
                    };
                }
            }
            sys.stdout.flush();
            if signal_number != "atexit" {
                raise!(KeyboardInterrupt); //unsupported
            }
        }
        fn printpipewarning<T0, T1>(signal_number: T0, frame: T1) {
            println!("{:?} ", "got SIGPIPE, ignoring");
        }
        for s in (1..16) {
            if s == 13 {
                signal.signal(s, printpipewarning);
            } else {
                if s != 9 {
                    signal.signal(s, cleanup);
                }
            }
        }
        signal.signal(signal.SIGCHLD, cleanup);
        atexit.register(|| cleanup("atexit"));
    }
    fn __del__(&self) {
        println!("{:?} ", "closing autopilot");
        self.server.__del__();
        if self.watchdog_device {
            println!("{:?} ", "close watchdog");
            self.watchdog_device.write("V");
            self.watchdog_device.close();
        }
    }
    fn register<T0, T1, RT>(&self, _type: T0, name: T1) -> RT {
        return self.client.register(_type(starred!((vec![("ap." + name)] + args.collect::<Vec<_>>()))/*unsupported*/, kwargs));
    }
    fn adjust_mode<T0>(&self, pilot: T0) {
        let newmode = pilot.best_mode(self.preferred_mode.value);
        if self.mode.value != newmode {
            self.mode.set_internal(newmode);
        }
    }
    fn compute_offsets(&self) {
        let compass = self.boatimu.SensorValues["heading_lowpass"].value;
        if self.sensors.gps.source.value != "none" {
            let mut d = 0.002;
            let gps_speed = self.sensors.gps.speed.value;
            self.gps_speed = (((1 - d) * self.gps_speed) + (d * gps_speed));
            if gps_speed > 1 {
                let gps_track = self.sensors.gps.track.value;
                d = (0.005 * math.log((self.gps_speed + 1)));
                self.gps_compass_offset.update((gps_track - compass), d);
            }
        }
        if self.sensors.wind.source.value != "none" {
            let mut offset = resolv((self.sensors.wind.wdirection + compass), self.wind_compass_offset.value);
            self.wind_compass_offset.update(offset, self.sensors.wind.wfactor);
            let mut boat_speed = None;
            if self.sensors.water.source.value != "none" {
                boat_speed = self.sensors.water.speed.value;
                if self.sensors.truewind.source.value == "none" {
                    self.sensors.truewind.source.update("water+wind");
                }
            } else {
                if self.sensors.gps.source.value != "none" {
                    boat_speed = self.gps_speed;
                    if self.sensors.truewind.source.value == "none" {
                        self.sensors.truewind.source.update("gps+wind");
                    }
                }
            }
            if boat_speed != None {
                self.sensors.truewind.update_from_apparent(boat_speed, self.sensors.wind.wspeed, self.sensors.wind.wdirection);
            }
        }
        if self.sensors.truewind.source.value != "none" {
            let mut offset = resolv((self.sensors.truewind.wdirection + compass), self.true_wind_compass_offset.value);
            self.true_wind_compass_offset.update(offset, self.sensors.truewind.wfactor);
        }
    }
    fn fix_compass_calibration_change<T0, T1>(&self, data: T0, t0: T1) {
        let headingrate = self.boatimu.SensorValues["headingrate_lowpass"].value;
        let dt = (t0 - self.lasttime).iter().min().unwrap();
        self.lasttime = t0;
        self.compass_change = 0;
        if data {
            if data.iter().any(|&x| x == "compass_calibration_updated") && self.last_heading {
                let last_heading = resolv(self.last_heading, data["heading"]);
                self.compass_change += ((data["heading"] - (headingrate * dt)) - last_heading);
            }
            self.last_heading = data["heading"];
        }
        if self.last_heading_off != self.boatimu.heading_off.value {
            self.last_heading_off = resolv(self.last_heading_off, self.boatimu.heading_off.value);
            self.compass_change += (self.boatimu.heading_off.value - self.last_heading_off);
            self.last_heading_off = self.boatimu.heading_off.value;
        }
        if self.compass_change {
            self.gps_compass_offset.value -= self.compass_change;
            self.wind_compass_offset.value += self.compass_change;
            self.true_wind_compass_offset.value += self.compass_change;
            if self.mode.value == "compass" {
                let heading_command = (self.heading_command.value + self.compass_change);
                self.heading_command.set(resolv(heading_command, 180));
            }
        }
    }
    fn compute_heading_error<T0>(&self, t: T0) {
        let heading = self.heading.value;
        let windmode = self.mode.value.iter().any(|&x| x == "wind");
        if self.mode.value != self.lastmode {
            let mut error = self.heading_error.value;
            if windmode {
                error = -(error);
            }
            self.heading_command.set((heading - error));
            self.lastmode = self.mode.value;
        }
        let heading_command = self.heading_command.value;
        let mut err = minmax(resolv((heading - heading_command)), 60);
        if self.mode.value.iter().any(|&x| x == "wind") {
            err = -(err);
        }
        self.heading_error.set(err);
        let mut dt = (t - self.heading_error_int_time);
        dt = dt.iter().min().unwrap();
        self.heading_error_int_time = t;
        self.heading_error_int.set(minmax((self.heading_error_int.value + ((self.heading_error.value / 1500) * dt)), 1));
    }
    fn iteration(&self) {
        let mut data = false;
        let t0 = time.monotonic();
        self.server.poll();
        let msgs = self.client.receive(self.dt);
        for msg in msgs {
            println!("{:?} {:?} {:?} ", "autopilot main process received:", msg, msgs[msg]);
        }
        if !self.enabled.value {
            if self.lastenabled {
                self.servo.command.set(0);
            }
            self.servo.poll();
        }
        let t1 = time.monotonic();
        let period = (1 / self.boatimu.rate.value);
        if (t1 - t0) > ((period / 2) + self.dt) {
            println!("{:?} {:?} ",_("server/client is running too _slowly_"), (t1 - t0));
        }
        self.sensors.poll();
        let t2 = time.monotonic();
        if (t2 - t1) > (period / 2) {
            println!("{:?} {:?} ",_("sensors is running too _slowly_"), (t2 - t1));
        }
        let mut sp = 0;
        for tries in (0..14) {
            let timu = time.monotonic();
            data = self.boatimu.read();
            if data {
                break;
            }
            let pd10 = (period / 10);
            sp += pd10;
            time.sleep(pd10);
        }
        let t3 = time.monotonic();
        if (t3 - t2) > ((period * 2) / 3) && data {
            println!("{:?} {:?} {:?} ", "read imu running too _slowly_", (t3 - t2), period);
        }
        self.fix_compass_calibration_change(data, t0);
        self.compute_offsets();
        let pilot = self.pilots[self.pilot.value];
        self.adjust_mode(pilot);
        pilot.compute_heading();
        self.compute_heading_error(t0);
        if self.enabled.value {
            self.runtime.update();
        } else {
            self.runtime.stop();
        }
        if self.enabled.value != self.lastenabled {
            self.lastenabled = self.enabled.value;
            if self.enabled.value {
                self.heading_error_int.set(0);
                self.heading_command_rate.set(0);
                self.last_heading_mode = false;
            }
        }
        if self.last_heading_mode != self.mode.value || (t0 - self.heading_command_rate.time) > 1 {
            self.last_heading_command = self.heading_command.value;
        }
        let mut heading_command_diff = resolv((self.heading_command.value - self.last_heading_command));
        if !self.mode.value.iter().any(|&x| x == "wind") {
            heading_command_diff = -(heading_command_diff);
        }
        self.last_heading_command = self.heading_command.value;
        self.heading_command_rate.time = t0;
        let lp = 0.1;
        let command_rate = (((1 - lp) * self.heading_command_rate.value) + (lp * heading_command_diff));
        self.heading_command_rate.update(command_rate);
        self.last_heading_mode = self.mode.value;
        if !self.tack.process() {
            let mut compute = true;
            if !self.enabled.value {
                for gain in pilot.gains {
                    if pilot.gains[gain]["sensor"].watch {
                        break;
                    }
                }
            }
            if compute {
                pilot.process();
            }
        }
        self.servo.force_engaged = self.enabled.value;
        let t4 = time.monotonic();
        if (t4 - t3) > (period / 2) {
            println!("{:?} {:?} ",_("autopilot routine is running too _slowly_"), (t4 - t3));
        }
        if self.enabled.value {
            self.servo.poll();
        }
        self.sensors.gps.predict(self);
        self.sensors.water.compute(self);
        self.boatimu.send_cal_data();
        let t5 = time.monotonic();
        if (t5 - t4) > (period / 2) && self.servo.driver {
            println!("{:?} {:?} ",_("servo is running too _slowly_"), (t5 - t4));
        }
        self.timings.set(vec![(t1 - t0), (t2 - t1), (t3 - t2), (t4 - t3), (t5 - t4), (t5 - t0)]);
        self.timestamp.set((t0 - self.starttime));
        if self.watchdog_device {
            self.watchdog_device.write("c");
        }
        let t6 = time.monotonic();
        if (t6 - t0) > period {
            println!("{:?} {:?} ",_("autopilot iteration running too slow"), (t6 - t0));
        }
        while true {
            let dt = ((period - (time.monotonic() - t0)) + sp);
            if dt >= period || dt <= 0 {
                break;
            }
            time.sleep(dt);
            self.dt = if self.enabled.value { 0 } else { (dt * 0.8) };
        }
    }
}

fn main() {
    let ap = Autopilot();
    println!("{:?} {:?} ", "autopilot init complete", time.monotonic());
    while true {
        ap.iteration();
    }
}

fn main() {
    main();
}