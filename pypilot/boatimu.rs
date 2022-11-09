use std::*;
use std::collections::HashMap;

sys.path.append(os.path.dirname(os.path.abspath(__file__)));
let try_dummy = { //unsupported
use client::{pypilotClient};
use values::{*};
use nonblockingpipe::{NonBlockingPipe};
};
let except!() = { //unsupported
};
let try_dummy = { //unsupported
};
let except!(ImportError) = { //unsupported
const RTIMU: _ = false;
println!("{:?} ",_("RTIMU library not detected, please install it"));
};
struct IMU {
client: ST0,
multiprocessing: ST1,
process: ST2,
gyrobias: ST3,
lastgyrobiastime: ST4,
s: ST5,
imu_detect_time: ST6,
rtimu: bool,
lastdata: bool,
rate: ST7,
avggyro: ST8,
compass_calibration_updated: bool,
}

impl IMU {
fn __init__<T0>(&self, server: T0)  {
self.client = pypilotClient(server);
self.multiprocessing = server.multiprocessing;
if self.multiprocessing {
let (self.pipe, pipe) = NonBlockingPipe("imu pipe", self.multiprocessing);
self.process = multiprocessing.Process(self.process, (pipe), true);
self.process.start();
return;
}
self.process = false;
self.setup();
}
fn setup(&self)  {
self.client.watch("imu.accel.calibration");
self.client.watch("imu.compass.calibration");
self.client.watch("imu.rate");
self.gyrobias = self.client.register(SensorValue("imu.gyrobias", true));
self.lastgyrobiastime = time.monotonic();
let SETTINGS_FILE = "RTIMULib";
println!("{:?} ",(((_("Using settings file") + " ") + SETTINGS_FILE) + ".ini"));
let s = RTIMU.Settings(SETTINGS_FILE);
s.FusionType = 1;
s.CompassCalValid = false;
s.CompassCalEllipsoidOffset = (0, 0, 0);
s.CompassCalEllipsoidValid = true;
s.MPU925xAccelFsr = 0;
s.MPU925xGyroFsr = 0;
let rate = 100;
s.MPU925xGyroAccelSampleRate = rate;
s.MPU925xCompassSampleRate = rate;
s.AccelCalValid = true;
s.AccelCalMin = (-1, -1, -1);
s.AccelCalMax = (1, 1, 1);
s.GyroBiasValid = false;
s.GyroBias = (0, 0, 0);
let (s.KalmanRk, s.KalmanQ) = (0.002, 0.001);
self.s = s;
self.imu_detect_time = 0;
self.rtimu = true;
self.init();
self.lastdata = false;
self.rate = 10;
}
fn init(&self)  {
let t0 = time.monotonic();
self.s.IMUType = 0;
if (t0 - self.imu_detect_time) < 1 {
return;
}
self.imu_detect_time = t0;
let rtimu = RTIMU.RTIMU(self.s);
if rtimu.IMUName() == "Null IMU" {
if self.rtimu {
println!("{:?} {:?} ",_("ERROR: No IMU Detected"), t0);
}
self.s.IMUType = 0;
self.rtimu = false;
return;
}
println!("{:?} ",("IMU Name: " + rtimu.IMUName()));
if !rtimu.IMUInit() {
println!("{:?} {:?} ",_("ERROR: IMU Init Failed, no inertial data available"), t0);
self.s.IMUType = 0;
return;
}
rtimu.setSlerpPower(0.01);
rtimu.setGyroEnable(true);
rtimu.setAccelEnable(true);
rtimu.setCompassEnable(true);
time.sleep(0.1);
self.rtimu = rtimu;
self.avggyro = vec![0, 0, 0];
self.compass_calibration_updated = false;
}
fn process<T0>(&self, pipe: T0)  {
println!("{:?} {:?} ","imu process", os.getpid());
if !RTIMU {
while true {
time.sleep(10);
}
}
if os.system(("sudo chrt -pf 2 %d 2>&1 > /dev/null" % os.getpid())) {
println!("{:?} ",_("warning, failed to make imu process realtime"));
} else {
println!("{:?} ",_("made imu process realtime"));
}
self.setup();
while true {
let t0 = time.monotonic();
let data = self.read();
let t1 = time.monotonic();
pipe.send(data, !data);
let t2 = time.monotonic();
if !self.s.GyroBiasValid {
if self.gyrobias.value {
println!("{:?} {:?} ",_("setting initial gyro bias"), self.gyrobias.value);
self.s.GyroBias = tuple(self.gyrobias.value.iter().map(math.radians));
self.s.GyroBiasValid = true;
}
}
if (t0 - self.lastgyrobiastime) > 30 {
self.gyrobias.set(self.s.GyroBias.iter().map(math.degrees).collect::<Vec<_>>());
self.lastgyrobiastime = t0;
self.s.GyroBiasValid = true;
}
self.poll();
let t3 = time.monotonic();
let dt = (time.monotonic() - t0);
let period = (1/self.rate);
let t = (period - dt);
if t > 0&&t < period {
time.sleep(t);
} else {
println!("{:?} {:?} {:?} {:?} {:?} {:?} ",_("imu process failed to keep time"), dt, t0, t1, t2, t3);
}
}
}
fn read<RT>(&self) -> RT {
let t0 = time.monotonic();
if !self.s.IMUType {
self.init();
return false;
}
if !self.rtimu.IMURead() {
println!("{:?} {:?} ",_("failed to read IMU!"), t0);
self.init();
return false;
}
let data = self.rtimu.getIMUData();
data["accel.residuals"] = self.rtimu.getAccelResiduals().collect::<Vec<_>>();
data["timestamp"] = t0;
if self.compass_calibration_updated {
data["compass_calibration_updated"] = true;
self.compass_calibration_updated = false;
}
self.lastdata = (data["gyro"].collect::<Vec<_>>(), data["compass"].collect::<Vec<_>>());
return data;
}
fn poll(&self)  {
let msgs = self.client.receive();
for name in msgs {
let value = msgs[name];
if name == "imu.accel.calibration" {
self.s.AccelCalValid = true;
let (b, t) = (value[0][..3], value[0][3]);
self.s.AccelCalMin = ((b[0] - t), (b[1] - t), (b[2] - t));
self.s.AccelCalMax = ((b[0] + t), (b[1] + t), (b[2] + t));
} else {
if name == "imu.compass.calibration" {
self.compass_calibration_updated = true;
self.s.CompassCalEllipsoidValid = true;
self.s.CompassCalEllipsoidOffset = tuple(value[0][..3]);
} else {
if name == "imu.rate" {
self.rate = value;
println!("{:?} {:?} ",_("imu rate set to rate"), value);
}
}
}
}
if !self.lastdata {
return;
}
let (gyro, compass) = self.lastdata;
self.lastdata = false;
let d = (0.05/self.rate);
for i in (0..3) {
self.avggyro[i] = (((1 - d)*self.avggyro[i]) + (d*gyro[i]));
}
if vector.norm(self.avggyro) > 0.8 {
println!("{:?} {:?} {:?} ",_("too high standing gyro bias, resetting sensors"), gyro, self.avggyro);
self.init();
}
if any(compass.iter().map(|x| abs(x) > 1000)) {
println!("{:?} {:?} ",_("compass out of range, resetting"), compass);
self.init();
}
} 
}
struct FrequencyValue {
loopc: ST0,
t0: ST1,
}

impl FrequencyValue {
fn __init__<T0>(&self, name: T0)  {
super(FrequencyValue, self).__init__(name);
self.loopc = 0;
self.t0 = time.monotonic();
}
fn strobe(&self)  {
self.loopc += 1;
if self.loopc == 4 {
let t1 = time.monotonic();
self.set((self.loopc/(t1 - self.t0)));
self.t0 = t1;
self.loopc = 0;
}
} 
}
fn readable_timespan<T0, RT>(total: T0) -> RT {
let mods = vec![("s", 1), ("m", 60), ("h", 60), ("d", 24), ("y", 365.24)];
fn loop<T0, T1, RT>(i: T0, mod: T1) -> RT {
if i == mods.len()||i32::from((total/(mods[i][1]*mod))) == 0&&i > 0 {
return "";
}
if i < (mods.len() - 1) {
let div = ((mods[i][1]*mods[(i + 1)][1])*mod);
t = i32::from((total % i32::from(div)));
} else {
t = total;
}
return (loop_((i + 1), (mods[i][1]*mod)) + ((("%d" + mods[i][0]) + " ") % (t/(mods[i][1]*mod))));
}
return loop_(0, 1);
}
struct TimeValue {
lastupdate_value: ST0,
lastage_value: ST1,
stopped: bool,
total: ST2,
start: ST3,
value: ST4,
lastage: ST5,
}

impl TimeValue {
fn __init__<T0>(&self, name: T0)  {
super(TimeValue, self).__init__(name, 0, kwargs);
self.lastupdate_value = 0;
self.lastage_value = -100;
self.stopped = true;
self.total = self.value;
}
fn reset(&self)  {
self.lastupdate_value = 0;
self.total = 0;
self.start = time.monotonic();
self.set(0);
}
fn update(&self)  {
let t = time.monotonic();
if self.stopped {
self.stopped = false;
self.start = t;
}
self.value = ((self.total + t) - self.start);
if abs((self.value - self.lastupdate_value)) > 1 {
self.lastupdate_value = self.value;
self.set(self.value);
}
}
fn stop(&self)  {
if self.stopped {
return;
}
self.total += (time.monotonic() - self.start);
self.stopped = true;
}
fn get_msg<RT>(&self) -> RT {
if abs((self.value - self.lastage_value)) > 1 {
self.lastage_value = self.value;
self.lastage = readable_timespan(self.value);
}
return (("\"" + self.lastage) + "\"");
} 
}
struct QuaternionValue {

}

impl QuaternionValue {
fn __init__<T0, T1>(&self, name: T0, initial: T1)  {
super(QuaternionValue, self).__init__(name, initial, kwargs);
}
fn set<T0>(&self, value: T0)  {
if value {
value = quaternion.normalize(value);
}
super(QuaternionValue, self).set(value);
} 
}
fn heading_filter<T0, T1, T2, RT>(lp: T0, a: T1, b: T2) -> RT {
if !a {
return b;
}
if !b {
return a;
}
if (a - b) > 180 {
a -= 360;
} else {
if (b - a) > 180 {
b -= 360;
}
}
let mut result = ((lp*a) + ((1 - lp)*b));
if result < 0 {
result += 360;
}
return result;
}
fn CalibrationProcess<T0, T1>(cal_pipe: T0, client: T1)  {
if os.system(("sudo chrt -po 0 %d 2> /dev/null > /dev/null" % os.getpid())) {
println!("{:?} ",_("warning, failed to make calibration process other"));
}
if os.system(("sudo chrt -pi 0 %d 2> /dev/null > /dev/null" % os.getpid())) {
println!("{:?} ",_("warning, failed to make calibration process idle, trying renice"));
if os.system(("renice 20 %d" % os.getpid())) {
println!("{:?} ",_("warning, failed to renice calibration process"));
}
}
time.sleep(4);
while true {
let try_dummy = { //unsupported
println!("{:?} {:?} ",_("calibration loaded, starting"), os.getpid());
cal_pipe.send("ready");
break;
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ",_("failed import calibration fit"), e);
time.sleep(30);
};
}
calibration_fit.CalibrationProcess(cal_pipe, client);
}
struct AutomaticCalibrationProcess {
client: ST0,
process: ST1,
cal_ready: bool,
}

impl AutomaticCalibrationProcess {
fn __init__<T0>(&self, server: T0)  {
if true {
let (self.cal_pipe, self.cal_pipe_process) = NonBlockingPipe("cal pipe", true, true);
} else {
let (self.cal_pipe, self.cal_pipe_process) = (false, false);
}
self.client = pypilotClient(server);
self.process = multiprocessing.Process(CalibrationProcess, (self.cal_pipe_process, self.client), true);
self.process.start();
self.cal_ready = false;
}
fn calibration_ready<RT>(&self) -> RT {
if self.cal_ready {
return true;
}
if self.cal_pipe.recv() {
self.cal_ready = true;
return true;
}
return false;
}
fn __del__(&self)  {
self.process.terminate();
} 
}
struct BoatIMU {
client: ST0,
rate: ST1,
frequency: ST2,
alignmentQ: ST3,
heading_off: ST4,
alignmentCounter: ST5,
last_alignmentCounter: bool,
uptime: ST6,
auto_cal: ST7,
lasttimestamp: ST8,
headingrate: ST9,
heading_lowpass_constant: ST10,
headingrate_lowpass_constant: ST11,
headingraterate_lowpass_constant: ST12,
SensorValues: HashMap<_,_>,
imu: ST13,
last_imuread: ST14,
cal_data: bool,
alignmentPose: ST15,
}

impl BoatIMU {
fn __init__<T0>(&self, client: T0)  {
self.client = client;
self.rate = self.register(EnumProperty, "rate", 20, vec![10, 20], true);
self.frequency = self.register(FrequencyValue, "frequency");
self.alignmentQ = self.register(QuaternionValue, "alignmentQ", vec![1, 0, 0, 0], true);
self.alignmentQ.last = false;
self.heading_off = self.register(RangeProperty, "heading_offset", 0, -180, 180, true);
self.heading_off.last = 3000;
self.alignmentCounter = self.register(Property, "alignmentCounter", 0);
self.last_alignmentCounter = false;
self.uptime = self.register(TimeValue, "uptime");
self.auto_cal = AutomaticCalibrationProcess(client.server);
self.lasttimestamp = 0;
self.headingrate = 0;
self.heading_lowpass_constant = self.register(RangeProperty, "heading_lowpass_constant", 0.2, 0.05, 0.3);
self.headingrate_lowpass_constant = self.register(RangeProperty, "headingrate_lowpass_constant", 0.2, 0.05, 0.3);
self.headingraterate_lowpass_constant = self.register(RangeProperty, "headingraterate_lowpass_constant", 0.1, 0.05, 0.3);
let mut sensornames = vec!["accel", "gyro", "compass", "accel.residuals", "pitch", "roll"];
sensornames += vec!["pitchrate", "rollrate", "headingrate", "headingraterate", "heel"];
sensornames += vec!["headingrate_lowpass", "headingraterate_lowpass"];
let directional_sensornames = vec!["heading", "heading_lowpass"];
sensornames += directional_sensornames;
self.SensorValues = HashMap::new();
for name in sensornames {
self.SensorValues[name] = self.register(SensorValue, name, directional_sensornames.iter().any(|&x| x == name));
}
self.SensorValues["fusionQPose"] = self.register(SensorValue, "fusionQPose", "%.8f");
self.imu = IMU(client.server);
self.last_imuread = (time.monotonic() + 4);
self.cal_data = false;
}
fn __del__(&self)  {
/*pass*/
}
fn register<T0, T1, RT>(&self, _type: T0, name: T1) -> RT {
let value = _type(starred!((vec![("imu." + name)] + args.collect::<Vec<_>>()))/*unsupported*/, kwargs);
return self.client.register(value);
}
fn update_alignment<T0>(&self, q: T0)  {
let a2 = (2*math.atan2(q[3], q[0]));
let heading_offset = ((a2*180)/math.pi);
let off = (self.heading_off.value - heading_offset);
let o = quaternion.angvec2quat(((off*math.pi)/180), vec![0, 0, 1]);
self.alignmentQ.update(quaternion.normalize(quaternion.multiply(q, o)));
}
fn IMUread<RT>(&self) -> RT {
if self.imu.multiprocessing {
let mut lastdata = false;
while true {
let data = self.imu.pipe.recv();
if !data {
return lastdata;
}
lastdata = data;
}
}
return self.imu.read();
}
fn poll(&self)  {
if !self.imu.multiprocessing {
self.imu.poll();
}
}
fn read<RT>(&self) -> RT {
let data = self.IMUread();
if !data {
if (time.monotonic() - self.last_imuread) > 1&&self.frequency.value {
println!("{:?} ","IMURead failed!");
self.frequency.set(false);
for name in self.SensorValues {
self.SensorValues[name].set(false);
}
self.uptime.reset();
}
return false;
}
if vector.norm(data["accel"]) == 0 {
println!("{:?} {:?} ",_("accel values invalid"), data["accel"]);
return false;
}
self.last_imuread = time.monotonic();
self.frequency.strobe();
let mut aligned = quaternion.multiply(data["fusionQPose"], self.alignmentQ.value);
aligned = quaternion.normalize(aligned);
let (data["roll"], data["pitch"], data["heading"]) = quaternion.toeuler(aligned).iter().map(math.degrees);
if data["heading"] < 0 {
data["heading"] += 360;
}
let gyro_q = quaternion.rotvecquat(data["gyro"], data["fusionQPose"]);
let (ur, vr, data["headingrate"]) = gyro_q.iter().map(math.degrees);
let rh = math.radians(data["heading"]);
let srh = math.sin(rh);
let crh = math.cos(rh);
data["rollrate"] = ((ur*crh) + (vr*srh));
data["pitchrate"] = ((vr*crh) - (ur*srh));
let dt = (data["timestamp"] - self.lasttimestamp);
self.lasttimestamp = data["timestamp"];
if dt > 0.01&&dt < 0.2 {
data["headingraterate"] = ((data["headingrate"] - self.headingrate)/dt);
} else {
data["headingraterate"] = 0;
}
self.headingrate = data["headingrate"];
data["heel"] = ((data["roll"]*0.03) + (self.heel*0.97));
data["gyro"] = data["gyro"].iter().map(math.degrees).collect::<Vec<_>>();
let mut llp = self.heading_lowpass_constant.value;
data["heading_lowpass"] = heading_filter(llp, data["heading"], self.SensorValues["heading_lowpass"].value);
llp = self.headingrate_lowpass_constant.value;
data["headingrate_lowpass"] = ((llp*data["headingrate"]) + ((1 - llp)*self.SensorValues["headingrate_lowpass"].value));
llp = self.headingraterate_lowpass_constant.value;
data["headingraterate_lowpass"] = ((llp*data["headingraterate"]) + ((1 - llp)*self.SensorValues["headingraterate_lowpass"].value));
for name in self.SensorValues {
self.SensorValues[name].set(data[name]);
}
self.uptime.update();
if self.alignmentCounter.value != self.last_alignmentCounter {
self.alignmentPose = vec![0, 0, 0, 0];
}
if self.alignmentCounter.value > 0 {
self.alignmentPose = self.alignmentPose.iter().map(|x, y| (x + y)).collect::<Vec<_>>();
self.alignmentCounter.set((self.alignmentCounter.value - 1));
if self.alignmentCounter.value == 0 {
self.alignmentPose = quaternion.normalize(self.alignmentPose);
let adown = quaternion.rotvecquat(vec![0, 0, 1], quaternion.conjugate(self.alignmentPose));
let mut alignment = vec![];
alignment = quaternion.vec2vec2quat(vec![0, 0, 1], adown);
alignment = quaternion.multiply(self.alignmentQ.value, alignment);
if alignment.len() {
self.update_alignment(alignment);
}
}
self.last_alignmentCounter = self.alignmentCounter.value;
}
if self.heading_off.value != self.heading_off.last||self.alignmentQ.value != self.alignmentQ.last {
self.update_alignment(self.alignmentQ.value);
self.heading_off.last = self.heading_off.value;
self.alignmentQ.last = self.alignmentQ.value;
}
self.cal_data = [("accel", data["accel"]), ("compass", data["compass"]), ("fusionQPose", data["fusionQPose"])].iter().cloned().collect::<HashMap<_,_>>();
return data;
}
fn send_cal_data(&self)  {
if self.auto_cal.calibration_ready()&&self.cal_data {
self.auto_cal.cal_pipe.send(self.cal_data);
}
} 
}
fn printline()  {
for a in args {
sys.stdout.write(String::from(a));
sys.stdout.write(" ");
}
sys.stdout.write("");
sys.stdout.flush();
}
fn main()  {
use server::{pypilotServer};
let server = pypilotServer();
let client = pypilotClient(server);
let boatimu = BoatIMU(client);
let quiet = sys.argv.iter().any(|&x| x == "-q");
let mut lastprint = 0;
while true {
let t0 = time.monotonic();
server.poll();
client.poll();
let data = boatimu.read();
if data&&!quiet {
if (t0 - lastprint) > 0.25 {
printline("pitch", data["pitch"], "roll", data["roll"], "heading", data["heading"]);
lastprint = t0;
}
}
boatimu.poll();
while true {
let dt = ((1/boatimu.rate.value) - (time.monotonic() - t0));
if dt < 0 {
break;
}
if dt > 0 {
time.sleep(dt);
}
}
}
}
fn main() {
main();
}