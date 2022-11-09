use std::collections::HashMap;
use std::*;

use pypilot::client::pypilotClient;
use servo::*;
fn fit<T0, T1, RT>(x: T0, n: T1) -> RT {
    let try_dummy = { //unsupported
    };
    let except!() = {
        //unsupported
        println!(
            "{:?} ",
            "failed to load scientific library, cannot perform calibration update!"
        );
        return false;
    };
    fn func<T0, T1, T2, RT>(b: T0, x: T1, n: T2) -> RT {
        let mut res = -(x[1]);
        for o in (0..(n + 1)) {
            res = (res + (b[o] * x[0].pow(o)));
        }
        return res;
    }
    let a = numpy.array(x);
    let b = scipy.optimize.leastsq(func, (vec![0] * (n + 1)), (a, n))[0];
    let mut t = 0;
    for v in func(b, a, n) {
        t += v.pow(2);
    }
    return (b.collect::<Vec<_>>(), t.pow(0.5));
}
fn fit_str<T0, RT>(fit: T0) -> RT {
    let mut s = "";
    for o in (0..fit.len()) {
        if o {
            s += " + ";
        }
        s += (("%.3f*x**" % fit[o]) + String::from(o));
    }
    return s;
}
fn FitCalibration<T0, RT>(cal: T0) -> RT {
    let mut speeds = vec![];
    let mut commands = vec![];
    for speed in cal {
        let (raw_cmd, idle_current, stall_current, idle_voltage, dt, power) = cal[speed];
        speeds.push(speed);
        commands.push(raw_cmd);
    }
    let fits = HashMap::new();
    println!("{:?} {:?} ", "speeds", speeds.len());
    println!("{:?} ", "plot");
    for val in zip(speeds, commands) {
        println!("{:?} {:?} ", val[0], val[1]);
    }
    for n in vec![1, 3, 5] {
        if speeds.len() > (n + 1) {
            fits[n] = fit(vec![speeds, commands], n);
        } else {
            fits[n] = false;
        }
        println!("{:?} {:?} {:?} ", "fit order", n, fits[n]);
        if fits[n] {
            println!("{:?} ", fit_str(fits[n][0]));
        }
    }
    if fits[1] {
        return fits[1][0].iter().map(float);
    } else {
        return false;
    }
}
const period_speed: _ = 0.6;
const printconsole: _ = false;
fn ServoCalibrationThread<T0, RT>(calibration: T0) -> RT {
    let servo = calibration.servo;
    fn console() {
        let mut c = "";
        for t in text {
            c += (t + " ");
        }
        calibration.console.set(c);
        if printconsole {
            println!("{:?} ", c);
        }
    }
    fn command<T0>(value: T0) {
        if self.fwd_fault && value < 0 {
            servo.fwd_fault = false;
        } else {
            if self.rev_fault && value > 0 {
                servo.rev_fault = false;
            }
        }
        engage();
        calibration.raw_command.set(value);
    }
    fn stop() {
        command(0);
    }
    fn reset(&self) {
        self.stop();
        if !self.waitnofault(7) {
            println!(
                "{:?} {:?} {:?} ",
                "servo reset failed",
                self.fault(),
                self.data["servo.flags"]
            );
            exit(1);
        }
    }
    fn average_power<T0, RT>(&self, timeout: T0) -> RT {
        let start = time.monotonic();
        self.log = vec![];
        time.sleep(timeout);
        let (power, avgc, avgv) = (0, 0, 0);
        if self.log.len() {
            let lasttime = log[0][2];
            for l in self.log {
                let (voltage, current, time) = l;
                let dt = (time - lasttime);
                avgc += (dt * current);
                avgv += (dt * voltage);
                power += ((dt * voltage) * current);
            }
            avgv /= self.log.len();
            avgc /= self.log.len();
        }
        return (avgc, avgv, power);
    }
    fn calibrate_period<T0, T1, T2, RT>(raw_cmd: T0, period: T1, idle_current: T2) -> RT {
        reset();
        for t in (0..20) {
            sys.stdout.write(("%d " % t));
            sys.stdout.flush();
            self.average_power(1);
        }
        println!("{:?} ", "start");
        let t0 = time.monotonic();
        let transitions = 0;
        while true {
            fn period_command<T0, RT>(raw_cmd: T0) -> RT {
                let t = time.monotonic();
                while (time.monotonic() - t) <= (period - 0.05) {
                    command(raw_cmd);
                    if self.fault() && (time.monotonic() - t0) > 3 {
                        return true;
                    }
                    if (time.monotonic() - t0) > 3
                        && self.data["servo.current"] > (1.6 * idle_current)
                    {
                        return true;
                    }
                    time.sleep(0.1);
                }
                return false;
            }
            let t1 = time.monotonic();
            transitions += 1;
            if period_command(raw_cmd) {
                break;
            }
            period_command(0);
        }
        let dt = ((t1 - t0) + ((time.monotonic() - t1) * 2));
        let (current, voltage, t) = self.average_power(0);
        let power = ((current * voltage) * dt);
        let mut truespeed = (1 / dt);
        println!(
            "{:?} {:?} {:?} ",
            transitions,
            truespeed,
            if raw_cmd > 0 { "plota" } else { "plotb" }
        );
        return (current, voltage, transitions, dt, power);
    }
    fn calibrate_speed<T0, RT>(raw_cmd: T0) -> RT {
        self.reset();
        for t in (0..10) {
            sys.stdout.write(("%d " % t));
            sys.stdout.flush();
            self.average_power(1);
        }
        servo.max_current.set(10);
        let mut t0 = false;
        let mut idle_current = false;
        let mut stall_current = false;
        let mut prevcurrent = false;
        let mut lp_current = 0;
        let mut count = 0;
        let mut power = 0;
        while true {
            command(raw_cmd);
            let avgtime = 0.3;
            let (voltage, current, p) = average_power(avgtime);
            power += p;
            if !t0 {
                t0 = t;
            }
            if (t - t0) >= self.timeout {
                break;
            }
            if idle_current && current > (idle_current * 1.3) || lp_current > (idle_current * 1.1) {
                console("found stall current", current, lp_current);
                let (voltage, current, t1) = self.average_power(0.1);
                console("settled to stall current", current);
                stall_current = current;
            } else {
                if fault() {
                    if !idle_current {
                        console("motor fault without finding idle current:", raw_cmd);
                        return false;
                    }
                    console("stall current above max current for raw_cmd:", raw_cmd);
                    stall_current = servo.max_current;
                } else {
                    if idle_current && current > servo.max_current {
                        console("servo failed to stop overcurrent!!!!", current);
                        stall_current = servo.max_current.value;
                    }
                }
            }
            if stall_current {
                return vec![
                    raw_cmd,
                    idle_current,
                    stall_current,
                    idle_voltage,
                    (t - t0),
                    power,
                ];
            }
            if !idle_current
                && count > 4
                && current > 0
                && prevcurrent
                && abs((prevcurrent - current)) < 0.03
            {
                console("found idle current", current);
                servo.max_current((current * 1.8));
                idle_current = current;
                idle_voltage = voltage;
            }
            prevcurrent = current;
            lp_current = ((0.9 * lp_current) + (0.1 * current));
            count += 1;
        }
        println!("{:?} {:?} ", "timeout calibrating raw_cmd", raw_cmd);
        return false;
    }
    fn safe_raw_cmd<T0, RT>(&self, d: T0) -> RT {
        if self.brake_hack && d > 0 {
            d *= 1.4;
        }
        return (period_speed * d);
    }
    fn search_end<T0, RT>(sign: T0) -> RT {
        self.reset();
        self.waitfault(1);
        console("moving away from end");
        self.reset();
        self.command(self.safe_raw_cmd(-(sign)));
        self.waitfault(4);
        self.reset();
        self.waitfault(1);
        console("continuing to end at", self.safe_raw_cmd(sign));
        calibration = self.calibrate_speed(self.safe_raw_cmd(sign));
        if !calibration {
            console("failed to reach end at safe speed");
            return false;
        }
        console("reached end");
        return calibration;
    }
    let brake_hack = false;
    servo.brake_hack.set(brake_hack);
    servo.max_current.set(10);
    servo.disengage_on_timeout.set(false);
    calibration.raw_command(0);
    let timeout = 40;
    let mut cal = self.search_end(-1);
    if !cal {
        console("failed to reset servo position to start");
        console("Trying with brake hack");
        self.brake_hack = true;
        self.client.set("servo.brake_hack", self.brake_hack);
        cal = self.search_end(-1);
        if !cal {
            console("failed to reset servo position to start");
            exit(1);
        }
    }
    println!("{:?} {:?} ", "initial cal", cal);
    let (command, idle_current, stall_current, cal_voltage, dt, power) = cal;
    let mut truespeed = (1 / dt);
    let max_current = (idle_current + (0.75 * (stall_current - idle_current)));
    reset();
    console("max current found", max_current);
    console("found start");
    if false {
        let mut period = 0.2;
        for d in (0..12) {
            println!("{:?} {:?} ", "period", period);
            cal = self.calibrate_period(period_speed, period, idle_current);
            println!("{:?} {:?} ", "fwd", cal);
            cal = self.calibrate_period(-(period_speed), period, idle_current);
            println!("{:?} {:?} ", "rev", cal);
            period *= 1.5;
        }
        exit(0);
    }
    calibration = HashMap::new();
    let complete = vec![false, false];
    let lastspeed = vec![0, 0];
    let steps = 14;
    let mincmd = 400;
    let maxcmd = 750;
    let mut stepi = 0;
    for abs_raw_cmd in (mincmd..maxcmd).step_by(((maxcmd - mincmd) / (steps - 1))) {
        for signi in vec![0, 1] {
            let sign = (1 - (2 * signi));
            let raw_cmd = ((sign * abs_raw_cmd) / 1000.0);
            self.stop();
            console(
                ("%.1f%%" % (((stepi * 100.0) / 2) / steps)),
                "step",
                stepi,
                "of",
                (2 * steps),
                "raw command",
                raw_cmd,
            );
            stepi += 1;
            cal = self.calibrate_speed(raw_cmd);
            if cal {
                let (command, idle_current, stall_current, cal_voltage, dt, power) = cal;
                truespeed = (1 / dt);
                println!(
                    "{:?} {:?} {:?} {:?} {:?} {:?} ",
                    "truespeed", truespeed, "lastspeed", lastspeed[signi], "power", power
                );
                if lastspeed[signi] && (truespeed / lastspeed[signi]) < (1 + (0.1 / steps)) {
                    complete[signi] += 1;
                    console(
                        "completed this direction when counter >= 3:",
                        complete[signi],
                    );
                } else {
                    complete[signi] = 0;
                    calibration[(sign * truespeed)] = cal;
                    lastspeed[signi] = truespeed.iter().max().unwrap();
                }
            } else {
                if !self.search_end(sign) {
                    println!("{:?} ", "failed to find end");
                    exit(0);
                }
            }
        }
        if complete[0] >= 3 && complete[1] >= 3 {
            console("higher commands do not yield higher speeds: finished");
            break;
        }
    }
    if complete[0] < 3 || complete[1] < 3 {
        console("did not reach highest speed");
    }
    self.stop();
    self.client.set("servo.raw_command", 0);
    let speeds = calibration.collect::<Vec<_>>();
    let max_fwd_speed = speeds.iter().max().unwrap();
    let max_rev_speed = -speeds.iter().min().unwrap();
    let ratio = (max_fwd_speed / max_rev_speed);
    println!("{:?} {:?} ", "fwd/rev", ratio);
    if ratio > 1.1 || ratio < 0.9 {
        println!(
            "{:?} ",
            "warning: very unbalanced ratio in forward/reverse speed"
        );
    }
    let max_speed = max_rev_speed.iter().min().unwrap();
    println!("{:?} {:?} ", "max speed", max_speed);
    let fwd_calibration = HashMap::new();
    let rev_calibration = HashMap::new();
    for truespeed in calibration {
        if abs(truespeed) <= (max_speed * 0.99) || true {
            let speed = (truespeed / max_speed);
            cal = calibration[truespeed];
            if truespeed > 0 {
                fwd_calibration[speed] = cal;
            } else {
                rev_calibration[-(speed)] = (vec![-(cal[0])] + cal[1..]);
            }
        } else {
            println!(
                "{:?} {:?} ",
                "discarding speed, outside of range", truespeed
            );
        }
    }
    let min_fwd_speed = fwd_calibration.iter().min().unwrap();
    let min_rev_speed = rev_calibration.iter().min().unwrap();
    let min_speed = min_fwd_speed.iter().max().unwrap();
    let fwdfit = FitCalibration(fwd_calibration);
    let revfit = FitCalibration(rev_calibration);
    cal = [
        ("forward", fwdfit),
        ("reverse", revfit),
        ("Min Speed", min_speed),
        ("Brake Hack", self.brake_hack),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
    let f = open((os.getenv("HOME") + "/.pypilot/servocalibration"), "w");
    f.write(json.dumps(cal));
    console("calibration complete");
}
struct ServoCalibration {
    server: ST0,
    run: ST1,
    rawcommand: ST2,
    console: ST3,
    current_total: ST4,
    servo: ST5,
    thread: ST6,
    force_engaged: bool,
    command: ST7,
    raw_command: ST8,
    log: Vec<_>,
    state: ST9,
    fwd_fault: bool,
    rev_fault: bool,
}

impl ServoCalibration {
    fn __init__<T0>(&self, servo: T0) {
        self.server = servo.server;
        self.run = self.Register(BooleanProperty, "run", false);
        self.rawcommand = self.Register(SensorValue, "raw_command");
        self.console = self.Register(Value, "console", "");
        self.current_total = (0, 0);
        self.servo = servo;
        self.thread = threading.Thread(ServoCalibrationThread, (self));
        self.rawcommand.set(0);
    }
    fn raw_command<T0>(&self, value: T0) {
        self.rawcommand.set(value);
    }
    fn Register<T0, T1, RT>(&self, _type: T0, name: T1) -> RT {
        return self.server.Register(_type(starred!((vec![("servo.calibration." + name)] + args.collect::<Vec<_>>()))/*unsupported*/, kwargs));
        fn fault<RT>(&self) -> RT {
            return ((ServoFlags.OVERCURRENT_FAULT | ServoFlags.FALTPIN) & self.servo.flags.value)
                || !self.servo.engaged.value;
        }
    }
    fn poll(&self) {
        if !self.thread.is_alive() {
            if self.run.value {
                self.force_engaged = true;
                self.command = self.servo.command.value;
                self.raw_command = self.servo.raw_command.value;
                self.servo.brake_hack.set(false);
                self.log = vec![];
                self.state = 0;
                self.thread = thread.start();
            } else {
                return;
            }
        }
        if !self.run.value || self.ap.enabled.value {
            self.thread.exit();
            return;
        }
        if self.command != self.servo.command.value {
            console("servo command received, aborting");
            console("ensure the autopilot is not active and");
            console("no manual servo commands during calibration!");
            self.command = self.servo.command.value;
            self.thread.exit(0);
        }
        self.log.append(vec![
            self.servo.voltage.value,
            self.servo.current.value,
            self.servo.current.time,
        ]);
        if self.fwd_fault && self.rawcommand.value < 0 {
            self.fwd_fault = false;
        } else {
            if self.rev_fault && self.rawcommand.value > 0 {
                self.rev_fault = false;
            }
        }
        self.servo.engage();
        self.servo.raw_command(self.rawcommand.value);
    }
    fn stop(&self) {
        if self.thread.is_alive() {
            self.thread.exit();
        }
    }
}
fn round_any<T0, T1, RT>(x: T0, n: T1) -> RT {
    if type_(x) == type_(HashMap::new()) {
        let r = HashMap::new();
        for v in x {
            r[v] = round_any(x[v], n);
        }
        return r;
    } else {
        if type_(x) == type_(vec![]) {
            return x.iter().map(|v| round_any(v, n));
        } else {
            if type_(x) == type_(0.0) {
                return round(x, n);
            } else {
                return x;
            }
        }
    }
}
fn main() {
    use serialprobe;
    println!("{:?} ", "Servo Server");
    printconsole = true;
    let server = pypilotServer();
    let serial_probe = serialprobe.SerialProbe();
    let servo = Servo(server, serial_probe);
    servo.servo_calibration.run = true;
    while true {
        servo.poll();
        servo.send_command();
        server.HandleRequests();
        time.sleep(0.1);
    }
}
