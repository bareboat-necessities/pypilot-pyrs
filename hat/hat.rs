use std::*;
use std::collections::HashMap;

println!("{:?} {:?} ","hat start", time.monotonic());
use pypilot::{pyjson};
use pypilot::client::{pypilotClient};
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)));
println!("{:?} {:?} ","hat import done", time.monotonic());
struct Action {
hat: ST0,
name: ST1,
}

impl Action {
fn __init__<T0, T1>(&self, hat: T0, name: T1)  {
self.hat = hat;
self.name = name;
} 
}
struct ActionNone {

}

impl ActionNone {
fn __init__(&self)  {
super(ActionNone, self).__init__(None, "none");
}
fn trigger<T0>(&self, count: T0)  {
/*pass*/
} 
}
struct ActionKeypad {
lcd: ST0,
index: ST1,
}

impl ActionKeypad {
fn __init__<T0, T1, T2>(&self, lcd: T0, index: T1, name: T2)  {
super(ActionKeypad, self).__init__(None, name);
self.lcd = lcd;
self.index = index;
}
fn trigger<T0>(&self, count: T0)  {
self.lcd.keypad(self.index, count);
} 
}
struct ActionPypilot {
pypilot_name: ST0,
value: ST1,
}

impl ActionPypilot {
fn __init__<T0, T1, T2, T3>(&self, hat: T0, name: T1, pypilot_name: T2, pypilot_value: T3)  {
super(ActionPypilot, self).__init__(hat, name);
self.pypilot_name = pypilot_name;
self.value = pypilot_value;
}
fn trigger<T0>(&self, count: T0)  {
if self.hat.client&&!count {
self.hat.client.set(self.pypilot_name, self.value);
}
} 
}
struct ActionEngage {

}

impl ActionEngage {
fn __init__<T0>(&self, hat: T0)  {
super(ActionEngage, self).__init__(hat, "engage", "ap.enabled", true);
}
fn trigger<T0>(&self, count: T0)  {
super(ActionEngage, self).trigger(count);
if self.hat.client&&!count&&self.hat.last_msg.iter().any(|&x| x == "ap.heading") {
self.hat.client.set("ap.heading_command", self.hat.last_msg["ap.heading"]);
}
} 
}
struct ActionHeading {
offset: ST0,
servo_timeout: ST1,
}

impl ActionHeading {
fn __init__<T0, T1>(&self, hat: T0, offset: T1)  {
super(ActionHeading, self).__init__(hat, String::from(offset));
self.offset = offset;
}
fn trigger<T0>(&self, count: T0)  {
if !self.hat.client {
return;
}
if self.hat.last_msg["ap.enabled"] {
if !count {
if self.hat.last_msg["ap.mode"].iter().any(|&x| x == "wind") {
let sign = -(sign);
}
self.hat.client.set("ap.heading_command", (self.hat.last_msg["ap.heading_command"] + self.offset));
}
} else {
self.servo_timeout = (time.monotonic() + (abs(self.offset).pow(0.5)/2));
self.hat.client.set("servo.command", if self.offset > 0 { 1 } else { -1 });
self.hat.client.poll();
}
} 
}
struct ActionTack {
direction: ST0,
}

impl ActionTack {
fn __init__<T0, T1, T2>(&self, hat: T0, name: T1, direction: T2)  {
super(ActionTack, self).__init__(hat, name, "ap.tack.state", "begin");
self.direction = direction;
}
fn trigger(&self)  {
if self.hat.client {
self.hat.client.set("ap.tack.direction", self.direction);
}
super(ActionTack, self).trigger();
} 
}
struct Process {
hat: ST0,
process: ST1,
}

impl Process {
fn __init__<T0>(&self, hat: T0)  {
self.hat = hat;
self.create();
}
fn send<T0>(&self, value: T0)  {
if self.process {
self.pipe.send(value, 0.1);
}
}
fn create<T0>(&self, process: T0)  {
use pypilot::nonblockingpipe::{NonBlockingPipe};
let (self.pipe, pipe) = NonBlockingPipe(String::from(self), true);
self.process = multiprocessing.Process(process, (pipe, self.hat.config), true);
self.process.start();
} 
}
struct Web {
status: ST0,
}

impl Web {
fn __init__<T0>(&self, hat: T0)  {
self.status = "Not Connected";
super(Web, self).__init__(hat);
}
fn set_status<T0>(&self, value: T0)  {
if self.status == value {
return;
}
self.status = value;
self.send([("status", value)].iter().cloned().collect::<HashMap<_,_>>());
}
fn create(&self)  {
fn process<T0, T1>(pipe: T0, config: T1)  {
while true {
if os.system(("sudo chrt -pi 0 %d 2> /dev/null > /dev/null" % os.getpid())) {
println!("{:?} ","warning, failed to make hat web process idle, trying renice");
}
if os.system(("renice 20 %d" % os.getpid())) {
println!("{:?} ","warning, failed to renice hat web process");
}
if os.getenv("USER") == "tc"&&time.monotonic() < 360 {
time.sleep(30);
} else {
time.sleep(5);
}
let try_dummy = { //unsupported
web.web_process(pipe, config);
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","web failed to run process:", e);
};
}
}
super(Web, self).create(process);
self.send([("status", self.status)].iter().cloned().collect::<HashMap<_,_>>());
}
fn poll(&self)  {
let msg = self.pipe.recv();
if msg {
for name in msg {
let value = msg[name];
self.hat.update_config(name, value);
}
self.hat.write_config();
if msg.iter().any(|&x| x == "host") {
println!("{:?} {:?} ","host changed, exiting", msg["host"]);
exit(0);
}
}
} 
}
struct Arduino {
voltage: ST0,
status: ST1,
}

impl Arduino {
fn __init__<T0>(&self, hat: T0)  {
super(Arduino, self).__init__(hat);
self.voltage = [("vcc", 5), ("vin", 3.3)].iter().cloned().collect::<HashMap<_,_>>();
self.status = "Not Connected";
}
fn config<T0, T1>(&self, name: T0, value: T1)  {
self.send((name, value));
}
fn create(&self)  {
fn process<T0, T1>(pipe: T0, config: T1)  {
println!("{:?} {:?} ","arduino process on", os.getpid());
if os.system(("renice -5 %d" % os.getpid())) {
println!("{:?} ","warning, failed to renice hat arduino process");
}
while true {
arduino.arduino_process(pipe, config);
time.sleep(15);
}
}
super(Arduino, self).create(process);
}
fn poll<RT>(&self) -> RT {
let mut ret = vec![];
while true {
let msgs = self.pipe.recv();
if !msgs {
break;
}
for msg in msgs {
let (key, code) = msg;
if key == "baudrate" {
self.hat.web.send([("baudrate", code)].iter().cloned().collect::<HashMap<_,_>>());
} else {
if key == "voltage" {
self.hat.web.send([("voltage", ("5v = %.3f, 3.3v = %.3f" % (code["vcc"], code["vin"])))].iter().cloned().collect::<HashMap<_,_>>());
self.hat.lcd.send(msg);
} else {
ret.push(msg);
}
}
}
}
return ret;
} 
}
struct LCD {
lcd: ST0,
}

impl LCD {
fn __init__<T0>(&self, hat: T0)  {
super(LCD, self).__init__(hat);
}
fn create(&self)  {
fn process<T0, T1>(pipe: T0, config: T1)  {
println!("{:?} {:?} ","lcd process on", os.getpid());
self.lcd = lcd.LCD(self.hat.config);
self.lcd.pipe = pipe;
if self.lcd.use_glut {
use OpenGL::GLUT::{glutMainLoop, glutIdleFunc};
glutIdleFunc(self.lcd.poll);
glutMainLoop();
} else {
while true {
self.lcd.poll();
}
}
}
super(LCD, self).create(process);
}
fn keypad<T0, T1>(&self, index: T0, count: T1)  {
self.send((index, count));
}
fn poll(&self)  {
let ret = vec![];
while true {
let msg = self.pipe.recv();
if !msg {
break;
}
let (key, code) = msg;
if key == "write_config" {
self.hat.config["lcd"] = code;
self.hat.write_config();
} else {
if key == "buzzer"||key == "backlight" {
if self.hat.arduino {
self.hat.arduino.send(msg);
}
}
}
}
} 
}
struct Hat {
config: ST0,
configfilename: ST1,
servo_timeout: ST2,
last_msg: HashMap<_,_>,
poller: ST3,
gpio: ST4,
lcd: ST5,
client: ST6,
watchlist: ST7,
arduino: ST8,
lirc: ST9,
keytimes: HashMap<_,_>,
keytimeouts: HashMap<_,_>,
inputs: ST10,
actions: Vec<_>,
web: ST11,
}

impl Hat {
fn __init__(&self)  {
self.config = [("host", "localhost"), ("actions", HashMap::new()), ("pi.ir", true), ("arduino.ir", false), ("arduino.nmea.in", false), ("arduino.nmea.out", false), ("arduino.nmea.baud", 4800), ("lcd", HashMap::new())].iter().cloned().collect::<HashMap<_,_>>();
self.configfilename = (os.getenv("HOME") + "/.pypilot/hat.conf");
println!("{:?} {:?} ","loading config file:", self.configfilename);
let try_dummy = { //unsupported
let file = open(self.configfilename);
let config = pyjson::loads(file.read());
file.close();
for name in config {
self.config[name] = config[name];
}
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","config failed:", e);
};
let try_dummy = { //unsupported
let configfile = "/proc/device-tree/hat/custom_0";
let f = open(configfile);
let hat_config = pyjson::loads(f.read());
f.close();
println!("{:?} ","loaded device tree hat config");
if !self.config.iter().any(|&x| x == "hat")||hat_config != self.config["hat"] {
self.config["hat"] = hat_config;
println!("{:?} ","writing device tree hat to hat.conf");
self.write_config();
}
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} {:?} {:?} ","failed to load", configfile, ":", e);
};
if !self.config.iter().any(|&x| x == "hat") {
println!("{:?} ","assuming original 26 pin tinypilot with nokia5110 display");
self.config["hat"] = [("lcd", [("driver", "default"), ("port", "/dev/spidev0.0")].iter().cloned().collect::<HashMap<_,_>>()), ("lirc", "gpio4")].iter().cloned().collect::<HashMap<_,_>>();
self.write_config();
}
self.servo_timeout = (time.monotonic() + 1);
self.last_msg = HashMap::new();
self.last_msg["ap.enabled"] = false;
self.last_msg["ap.heading_command"] = 0;
self.last_msg["ap.mode"] = "";
if sys.argv.len() > 1 {
self.config["host"] = sys.argv[1];
self.write_config();
}
let host = self.config["host"];
println!("{:?} {:?} ","host", host);
if self.config["hat"].iter().any(|&x| x == "arduino") {
use arduino;
arduino.arduino(self.config).firmware();
}
self.poller = select.poll();
self.gpio = gpio.gpio();
self.lcd = LCD(self);
time.sleep(1);
self.client = pypilotClient(host);
self.client.registered = false;
self.watchlist = vec!["ap.enabled", "ap.heading_command", "ap.mode"];
for name in self.watchlist {
self.client.watch(name);
}
self.lcd.poll();
if self.config["hat"].iter().any(|&x| x == "arduino") {
self.arduino = Arduino(self);
self.poller.register(self.arduino.pipe, select.POLLIN);
} else {
self.arduino = false;
}
self.lirc = lircd.lirc(self.config);
self.lirc.registered = false;
self.keytimes = HashMap::new();
self.keytimeouts = HashMap::new();
self.inputs = vec![self.gpio, self.arduino, self.lirc];
self.actions = vec![];
let keypadnames = vec!["auto", "menu", "port1", "starboard1", "select", "port10", "starboard10", "tack", "dodge_port", "dodge_starboard"];
for i in (0..keypadnames.len()) {
self.actions.append(ActionKeypad(self.lcd, i, keypadnames[i]));
}
self.actions += vec![ActionEngage(self), ActionPypilot(self, "disengage", "ap.enabled", false), ActionHeading(self, 1), ActionHeading(self, -1), ActionHeading(self, 2), ActionHeading(self, -2), ActionHeading(self, 5), ActionHeading(self, -5), ActionHeading(self, 10), ActionHeading(self, -10), ActionPypilot(self, "compassmode", "ap.mode", "compass"), ActionPypilot(self, "gpsmode", "ap.mode", "gps"), ActionPypilot(self, "windmode", "ap.mode", "wind"), ActionPypilot(self, "truewindmode", "ap.mode", "truewind"), ActionPypilot(self, "center", "servo.position", 0), ActionTack(self, "tackport", "port"), ActionTack(self, "tackstarboard", "starboard")];
for name in self.config["actions"] {
if name.startswith("pilot_") {
self.actions.append(ActionPypilot(self, name, "ap.pilot", name.replace("pilot_", "", 1)));
}
}
self.actions.append(ActionNone());
for action in self.actions {
if !self.config["actions"].iter().any(|&x| x == action.name) {
self.config["actions"][action.name] = vec![];
}
}
self.web = Web(self);
fn cleanup<T0, T1>(signal_number: T0, frame: T1)  {
println!("{:?} {:?} {:?} {:?} ","got signal", signal_number, "cleaning up", os.getpid());
let mut childpids = vec![];
let processes = vec![self.arduino, self.web, self.lcd];
for process in processes {
if process&&process.process {
childpids.push(process.process.pid);
}
}
if signal_number == signal.SIGCHLD {
let mut pid = os.waitpid(-1, os.WNOHANG);
if !childpids.iter().any(|&x| x == pid[0]) {
println!("{:?} {:?} {:?} ","subprocess returned", pid, childpids);
return;
}
println!("{:?} {:?} {:?} ","child process", pid, childpids);
}
while childpids {
let mut pid = childpids.pop();
let try_dummy = { //unsupported
os.kill(pid, signal.SIGTERM);
};
let except!(ProcessLookupError) = { //unsupported
/*pass*/
};
let try_dummy = { //unsupported
sys.stdout.flush();
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","failed to flush stdout", e);
};
}
for process in processes {
if process {
process.process = false;
}
}
raise!(KeyboardInterrupt); //unsupported
let try_dummy = { //unsupported
sys.stdout.flush();
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","failed to flush stdout2", e);
};
}
for s in (1..16) {
if s != 9&&s != 13 {
signal.signal(s, cleanup);
}
}
signal.signal(signal.SIGCHLD, cleanup);
}
fn write_config(&self)  {
let try_dummy = { //unsupported
let f = open(self.configfilename, "w");
f.write((pyjson::dumps(self.config) + "
"));
f.close();
};
let except!(IOError) = { //unsupported
println!("{:?} {:?} ","failed to save config file:", self.configfilename);
};
}
fn update_config<T0, T1>(&self, name: T0, value: T1)  {
if self.config.iter().any(|&x| x == name)&&self.config[name] == value {
return;
}
if name.startswith("arduino.")&&self.arduino {
self.arduino.config(name, value);
}
self.config[name] = value;
}
fn apply_code<T0, T1>(&self, key: T0, count: T1)  {
if self.keytimeouts.iter().any(|&x| x == key) {
let timeoutcount = self.keytimeouts[key];
if count > timeoutcount {
return;
}
self.keytimeouts[key].drop();
if count == 0 {
return;
}
}
self.web.send([("key", key)].iter().cloned().collect::<HashMap<_,_>>());
let actions = self.config["actions"];
for action in self.actions {
if !actions.iter().any(|&x| x == action.name) {
actions[action.name] = vec![];
}
let keys = actions[action.name];
if keys.iter().any(|&x| x == key) {
if !count {
self.web.send([("action", action.name)].iter().cloned().collect::<HashMap<_,_>>());
if self.keytimes.iter().any(|&x| x == key) {
self.keytimes[key].drop();
}
} else {
self.keytimes[key] = (time.monotonic(), count);
}
action.trigger(count);
return;
}
}
self.web.send([("action", "none")].iter().cloned().collect::<HashMap<_,_>>());
}
fn update_values(&self)  {
let values = self.client.list_values();
if values {
if values.iter().any(|&x| x == "ap.pilot") {
let pilots = values["ap.pilot"]["choices"];
let mut update = false;
for pilot in pilots {
let name = ("pilot_" + pilot);
if !self.config["actions"].iter().any(|&x| x == name) {
println!("{:?} {:?} ","adding pilot", pilot);
self.config["actions"][name] = vec![];
update = true;
}
}
for name in self.config["actions"].collect::<Vec<_>>() {
if name.startswith("pilot_") {
let pilot = name.replace("pilot_", "", 1);
if !pilots.iter().any(|&x| x == pilot) {
println!("{:?} {:?} ","removing pilot", pilot);
self.config["actions"][name].drop();
update = true;
}
}
}
if update {
self.write_config();
println!("{:?} ","shutting down since pilots updated");
exit(0);
}
}
}
}
fn poll(&self)  {
let t0 = time.monotonic();
let keycounts = HashMap::new();
for i in self.inputs {
let try_dummy = { //unsupported
if !i {
continue;
}
let events = i.poll();
for event in events {
let (key, count) = event;
keycounts[key] = count;
}
};
let except!(Exception) = { //unsupported
self.inputs.remove(i);
println!("{:?} {:?} {:?} ","WARNING, failed to poll!!", e, i);
i.drop();
return;
};
}
let mut key = "";
let mut count = 0;
for (k, c) in keycounts.items() {
if c {
if key {
key += ("_" + k);
count = count.iter().min().unwrap();
} else {
key = k;
count = c;
}
}
}
if count {
self.apply_code(key, count);
}
for (k, c) in keycounts.items() {
if c == 0 {
self.apply_code(k, 0);
}
}
let t1 = time.monotonic();
let msgs = self.client.receive();
let t2 = time.monotonic();
for (name, value) in msgs.items() {
self.last_msg[name] = value;
}
for i in vec![self.lcd, self.web] {
i.poll();
}
let t3 = time.monotonic();
for (key, tc) in self.keytimes.items() {
let (t, c) = tc;
let mut dt = (t3 - t);
if dt > 0.6 {
println!("{:?} {:?} {:?} {:?} ","keyup event lost, releasing key from timeout", key, t3, dt);
self.apply_code(key, 0);
self.keytimeouts[key] = c;
break;
}
}
self.client.watch("ap.heading", if self.last_msg["ap.enabled"] { false } else { 1 });
if self.servo_timeout {
if time.monotonic() > self.servo_timeout {
if self.client {
self.client.set("servo.command", 0);
}
self.servo_timeout = 0;
}
}
if self.client.connection {
self.web.set_status("connected");
if !self.client.registered {
self.client.registered = true;
}
self.update_values();
} else {
self.client.registered = false;
self.web.set_status("disconnected");
}
let t4 = time.monotonic();
let mut dt = (t3 - t0);
let period = (1 - dt).iter().max().unwrap();
if !self.lirc.registered {
let fileno = self.lirc.fileno();
if fileno {
self.poller.register(fileno, select.POLLIN);
self.lirc.registered = true;
}
}
let e = self.poller.poll((1000*period));
} 
}
fn main()  {
let hat = Hat();
println!("{:?} {:?} ","hat init complete", time.monotonic());
while true {
hat.poll();
}
}
fn main() {
main();
}