use std::*;
use std::collections::HashMap;

use values::{*};
use resolv::{*};
use client::{pypilotClient};
use nonblockingpipe::{NonBlockingPipe};
let try_dummy = { //unsupported
};
let except!() = { //unsupported
println!("{:?} ","world magnetic model not available");
const wmm2020: _ = false;
};
const earth_radius: _ = 6378137.0;
const earth_md: _ = (((earth_radius*2)*math.pi)/360);
fn ll_to_xy<T0, T1, T2, T3, RT>(lat: T0, lon: T1, lat0: T2, lon0: T3) -> RT {
let cs = math.cos(math.radians(lat0));
let yc = (lat - lat0);
let xc = resolv((lon - lon0));
return (((earth_md*xc)*cs), (earth_md*yc));
}
fn xy_to_ll<T0, T1, T2, T3, RT>(x: T0, y: T1, lat0: T2, lon0: T3) -> RT {
let cs = math.cos(math.radians(lat0));
let xc = ((x/earth_md)/cs);
let yc = (y/earth_md);
return ((yc + lat0), (xc + lon0));
}
struct GPSFilterProcess {
client: ST0,
process: ST1,
output: ST2,
}

impl GPSFilterProcess {
fn __init__<T0>(&self, client: T0)  {
self.client = pypilotClient(client.server);
self.process = self;
self.output = client.register(BooleanProperty("gps.filtered.output", false, true));
let (self.pipe, pipe) = NonBlockingPipe("gps filter pipe", true);
super(GPSFilterProcess, self).__init__(self.filter_process, (pipe), true);
self.start();
}
fn predict<T0, T1, T2>(&self, accel: T0, fusionQpose_ned_magnetic: T1, t: T2)  {
self.pipe.send(("predict", (accel, fusionQpose_ned_magnetic, t)));
}
fn update<T0, T1>(&self, gps: T0, t: T1)  {
self.pipe.send(("update", (gps, t)));
}
fn filter_process<T0>(&self, pipe: T0)  {
while true {
let try_dummy = { //unsupported
//global np
break;
};
let except!() = { //unsupported
/*pass*/
};
time.sleep(20);
}
println!("{:?} {:?} ","gps filter process", os.getpid());
let f = GPSFilter(self.client);
while true {
while true {
let inp = pipe.recv();
if !inp {
break;
}
let (cmd, args) = inp;
if cmd == "predict" {
f.predict(starred!(args)/*unsupported*/);
} else {
if cmd == "update" {
f.update(starred!(args)/*unsupported*/);
}
}
}
let msgs = self.client.receive(0.1);
for msg in msgs {
self.values[msg] = msgs[msg];
}
}
} 
}
struct GPSFilter {
client: ST0,
gps_system_time_offset: ST1,
stale_count: ST2,
use3d: bool,
R: ST3,
enabled: ST4,
declination: ST5,
declination_time: ST6,
gps_time_offset: ST7,
compass_offset: ST8,
fix: ST9,
speed: ST10,
track: ST11,
Q: ST12,
predict_t: ST13,
X: bool,
P: ST14,
history: Vec<_>,
lastll: bool,
}

impl GPSFilter {
fn __init__<T0>(&self, client: T0)  {
self.client = client;
self.gps_system_time_offset = 0;
self.stale_count = 0;
self.use3d = false;
let posSigma = 10;
let velSigma = 0.25;
if self.use3d {
self.R = np.diag(vec![posSigma, posSigma, (posSigma*2), velSigma, velSigma, (velSigma*2)]);
} else {
self.R = np.diag(vec![posSigma, posSigma, velSigma, velSigma]);
}
self.enabled = self.register(BooleanProperty, "enabled", false, true);
self.declination = self.register(SensorValue, "declination");
self.declination_time = 0;
self.gps_time_offset = self.register(SensorValue, "time_offset");
self.gps_time_offset.update(0.7);
self.compass_offset = self.register(SensorValue, "compass_offset");
self.fix = self.register(JSONValue, "fix", false);
self.speed = self.register(SensorValue, "speed");
self.track = self.register(SensorValue, "track", true);
let posDev = 30;
let velDev = 3;
let c = if self.use3d { 3 } else { 2 };
let pos = np.diag((vec![posDev.pow(2)]*c));
let vel = np.diag((vec![velDev.pow(2)]*c));
let cov = np.diag((vec![(posDev*velDev)]*c));
self.Q = np.vstack((np.hstack((pos, cov)), np.hstack((cov, vel))));
self.predict_t = 0;
self.reset();
}
fn register<T0, T1, RT>(&self, _type: T0, name: T1) -> RT {
return self.client.register(_type(starred!((vec![("gps.filtered." + name)] + args.collect::<Vec<_>>()))/*unsupported*/, kwargs));
}
fn reset(&self)  {
let c = if self.use3d { 3 } else { 2 };
self.X = false;
self.P = np.identity((2*c));
self.history = vec![];
self.lastll = false;
}
fn predict<T0, T1, T2>(&self, accel: T0, fusionQPose: T1, t: T2)  {
if !self.enabled.value {
return;
}
let ta = time.monotonic();
let accel_ned_magnetic = quaternion.rotvecquat(accel, fusionQPose);
let residual_accel_magnetic = vector.sub(accel_ned_magnetic, vec![0, 0, 1]);
let decl_q = quaternion.angvec2quat(self.declination.value, vec![0, 0, 1]);
let accel_true = quaternion.rotvecquat(residual_accel_magnetic, decl_q);
let error_q = quaternion.angvec2quat(self.compass_offset.value, vec![0, 0, 1]);
let accel_ned = quaternion.rotvecquat(accel_true, error_q);
let mut U = (9.81*np.array(accel_ned));
if !self.use3d {
U = U[..2];
}
let dt = (t - self.predict_t);
self.predict_t = t;
if dt < 0||dt > 0.5 {
println!("{:?} {:?} ","gpsfilter reset", dt);
self.reset();
}
if type_(self.X) == bool&&!self.X {
return;
}
self.apply_prediction(dt, U);
self.history.append([("t", t), ("dt", dt), ("U", U), ("X", self.X), ("P", self.P)].iter().cloned().collect::<HashMap<_,_>>());
let ll = xy_to_ll(self.X[0], self.X[1], starred!(self.lastll)/*unsupported*/);
let c = if self.use3d { 3 } else { 2 };
let vx = self.X[c];
let vy = self.X[(c + 1)];
let speed = (math.hypot(vx, vy)*1.94);
self.speed.set(speed);
let track = resolv(math.degrees(math.atan2(vx, vy)), 180);
self.track.set(track);
let fix = [("lat", ll[0]), ("lon", ll[1]), ("speed", speed), ("track", track), ("timestamp", time.time())].iter().cloned().collect::<HashMap<_,_>>();
if self.use3d {
fix["alt"] = self.X[2];
fix["climb"] = self.X[5];
}
self.fix.set(fix);
let tb = time.monotonic();
}
fn apply_prediction<T0, T1>(&self, dt: T0, U: T1)  {
dt = dt.iter().max().unwrap().iter().min().unwrap();
let dt2 = ((dt*dt)/2);
let c = if self.use3d { 3 } else { 2 };
let i = np.identity(c);
let B = np.vstack(((dt2*i), (dt*i)));
let F = np.vstack((np.hstack((i, (dt*i))), np.hstack((np.zeros(vec![c, c]), i))));
self.X = ((F None self.X) + (B None U));
self.P = (((F None self.P) None F.transpose()) + self.Q);
}
fn update<T0, T1>(&self, data: T0, t: T1)  {
if !self.enabled.value {
return;
}
let mut ts = data["timestamp"];
let mut dt = ((t - ts) + self.gps_system_time_offset);
if dt > 5 {
println!("{:?} {:?} ","gpsfilter stale time", dt);
self.stale_count += 1;
if self.stale_count > 5 {
self.gps_system_time_offset = (ts - t);
self.reset();
}
} else {
self.stale_count = 0;
if dt < 0 {
println!("{:?} ","gpsfilter reset time");
self.gps_system_time_offset = (ts - t);
self.reset();
}
}
ts -= self.gps_system_time_offset;
ts -= self.gps_time_offset.value;
let ll = (data["lat"], data["lon"]);
if type_(self.X) != bool {
let pll = xy_to_ll(self.X[0], self.X[1], starred!(self.lastll)/*unsupported*/);
let (self.X[0], self.X[1]) = ll_to_xy(pll[0], pll[1], starred!(ll)/*unsupported*/);
}
self.lastll = ll;
if (t - self.declination_time) > (3600*4) {
self.declination_time = t;
if wmm2020 {
let year = datetime.date.today().year;
self.declination.update(wmm2020.wmm(self.lastll[0], self.lastll[1], 0, year).decl);
}
}
let c = if self.use3d { 3 } else { 2 };
let try_dummy = { //unsupported
let xy = (0, 0);
let speed = (data["speed"]/1.944);
let track = data["track"];
let mut Z = xy.collect::<Vec<_>>();
if self.use3d {
Z.append(if data.iter().any(|&x| x == "alt") { data["alt"] } else { 0 });
}
Z += vec![(speed*math.sin(math.radians(track))), (speed*math.cos(math.radians(track)))];
if self.use3d {
Z.append(if data.iter().any(|&x| x == "climb") { data["climb"] } else { 0 });
}
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","gps filter update failed", e);
return;
};
let mut t0 = false;
let i = 0;
for i in (0..self.history.len()) {
let h = self.history[(-(i) - 1)];
t0 = h["t"];
if t0 < ts {
self.X = h["X"];
self.P = h["P"];
break;
}
}
if i > 0&&0 {
let a0 = math.degrees(math.atan2(starred!(self.history[(-(i) - 1)]["X"][c..(c + 2)])/*unsupported*/));
let a1 = math.degrees(math.atan2(starred!(self.history[-(i)]["X"][c..(c + 2)])/*unsupported*/));
let t1 = self.history[-(i)]["t"];
let da = resolv((a1 - a0));
let db = resolv((track - a0));
let mut nts = ((db/da)*(t1 - t0));
nts = nts.iter().max().unwrap().iter().min().unwrap();
dt = (nts - ts);
let mut to = self.gps_time_offset.value;
to += (dt*0.0005);
to = t0.iter().max().unwrap().iter().min().unwrap();
self.gps_time_offset.update(to);
}
if type_(self.X) == bool&&!self.X {
self.X = Z;
}
let H = np.identity((2*c));
let Y = (Z - (H None self.X));
let S = (((H None self.P) None H.transpose()) + self.R);
let try_dummy = { //unsupported
let invS = np.linalg.inv(S);
};
let except!() = { //unsupported
println!("{:?} ","gps filter failed to invert S");
return;
};
let K = ((self.P None H.transpose()) None invS);
let curX = self.X.copy();
self.X += (K None Y);
self.P = ((np.identity((2*c)) - (K None H)) None self.P);
if 0&&speed > 2 {
let (ax, ay) = curX[c..(c + 2)];
let (cx, cy) = self.X[c..(c + 2)];
let (ad, cd) = (math.hypot(ax, ay), math.hypot(cx, cy));
let s = (ad/cd);
if s > 0.8&&s < 1.2 {
let mut comp_adj = -math.degrees(math.asin((((ax*cy) - (ay*cx))/(ad*cd))));
comp_adj = comp_adj.iter().max().unwrap().iter().min().unwrap();
let d = 0.0005;
self.compass_offset.set(resolv((self.compass_offset.value + (d*comp_adj))));
}
}
for h in self.history[-(i)..] {
self.apply_prediction(h["dt"], h["U"]);
}
self.history = self.history[-(i)..];
} 
}