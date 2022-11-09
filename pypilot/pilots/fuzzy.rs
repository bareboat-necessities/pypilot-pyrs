use std::*;
use std::collections::HashMap;

use pilot::{AutopilotPilot};
use pypilot::values::{*};
use pypilot::{vector};
const disabled: _ = true;
const matrixfilepath: _ = ((os.getenv("HOME") + "/.pypilot/") + "fuzzy.json");
fn fuzzy_defaults<T0, T1, RT>(dimensions: T0, c: T1) -> RT {
if dimensions {
let ret = [("N/A", fuzzy_defaults(dimensions[1..]))].iter().cloned().collect::<HashMap<_,_>>();
let dimension = dimensions[0];
let (name, value, step) = dimension;
if name == "heading error" {
for i in (-5..5) {
let mut v = (i*step);
ret[i] = fuzzy_defaults(dimensions[1..], ((v*0.003) + c));
}
} else {
if name == "heading rate" {
for i in (-5..5) {
let mut v = (i*step);
ret[i] = fuzzy_defaults(dimensions[1..], ((v*0.09) + c));
}
}
}
return ret;
}
return c;
}
fn fuzzy_matrix<T0, T1, RT>(index: T0, matrix: T1) -> RT {
while !matrix.iter().any(|&x| x == index) {
if index < 0 {
index += 1;
} else {
if index > 0 {
index -= 1;
} else {
index = "N/A";
}
}
}
return matrix[index];
}
fn fuzzy_compute<T0, T1, T2, RT>(di: T0, dimensions: T1, matrix: T2) -> RT {
if di == dimensions.len() {
return matrix;
}
let dimension = dimensions[di];
let (name, sensor, step) = dimension;
let value = sensor.value;
if value == false {
return fuzzy_compute((di + 1), dimensions, matrix["N/A"]);
}
let index = (value/step);
let indexl = math.floor(index);
let indexh = (indexl + 1);
let matrixl = fuzzy_matrix(indexl, matrix);
let matrixh = fuzzy_matrix(indexh, matrix);
let l = fuzzy_compute((di + 1), dimensions, matrixl);
if matrixl == matrixh {
return l;
}
let h = fuzzy_compute((di + 1), dimensions, matrixh);
let d = (index - indexl);
return (((1 - d)*l) + (d*h));
}
fn fuzzy_get<T0, T1, RT>(matrix: T0, indicies: T1) -> RT {
if !indicies {
return matrix;
}
return fuzzy_get(fuzzy_matrix(indicies[0], matrix), indicies[1..]);
}
fn fuzzy_set<T0, T1, T2>(matrix: T0, indicies: T1, update: T2)  {
if indicies.len() == 1 {
matrix[indicies[0]] = update;
} else {
if !matrix.iter().any(|&x| x == indicies[0]) {
matrix[indicies[0]] = matrix["N/A"].copy();
}
fuzzy_set(matrix[indicies[0]], indicies[1..], update);
}
}
fn fuzzy_train<T0, T1, T2, T3>(dimensions: T0, matrix: T1, state: T2, error: T3)  {
let mut indicies = vec![];
for i in (0..state.len()) {
let dimension = dimensions[i];
let value = state[i];
let (name, sensor, step) = dimension;
if value == false {
index = "N/A";
} else {
index = round((value/step));
}
indicies.push(index);
}
let current = fuzzy_get(matrix, indicies);
let d = 0.002;
let mut update = (current + (d*error));
update = update.iter().max().unwrap().iter().min().unwrap();
fuzzy_set(matrix, indicies, update);
}
struct FuzzyPilot {
gains: HashMap<_,_>,
learningP: ST0,
learningD: ST1,
seastate: ST2,
accelm: ST3,
history_count: ST4,
history: Vec<_>,
history_time: ST5,
dimensions: ST6,
matrix: ST7,
matrix_time: ST8,
}

impl FuzzyPilot {
fn __init__<T0>(&self, ap: T0)  {
super(FuzzyPilot, self).__init__("fuzzy", ap);
self.gains = HashMap::new();
self.learningP = self.register(RangeProperty, "learningP", 0.003, 0, 0.02);
self.learningD = self.register(RangeProperty, "learningD", 0.09, 0, 1);
self.seastate = self.register(SensorValue, "seastate");
self.accelm = 1;
self.history_count = 40;
self.history = vec![];
self.history_time = 0;
self.dimensions = vec![("ground speed", ap.sensors.gps.speed, 2), ("wind speed", ap.sensors.wind.speed, 5), ("wind direction", ap.sensors.wind.direction, 10), ("rudder angle", ap.sensors.rudder.angle, 5), ("heel", ap.boatimu.SensorValues["heel"], 5), ("sea state", self.seastate, 0.1), ("heading error", ap.heading_error, 3), ("heading rate", ap.boatimu.SensorValues["headingrate_lowpass"], 2)];
self.matrix = fuzzy_defaults(self.dimensions);
self.load();
self.matrix_time = 0;
}
fn store(&self)  {
let try_dummy = { //unsupported
let f = open(matrixfilepath, "w");
f.write(pyjson.dumps(self.matrix));
f.close();
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","failed to store fuzzy data", e);
};
}
fn load(&self)  {
let try_dummy = { //unsupported
let f = open(matrixfilepath);
self.matrix = pyjson.loads(f.read());
f.close();
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","failed to load fuzzy data", e);
};
}
fn process(&self)  {
let t0 = time.monotonic();
let ap = self.ap;
let accel = ap.boatimu.SensorValues["accel"].value;
if accel {
self.accelm = ((0.1*vector::norm(accel)) + (0.9*self.accelm));
self.seastate.set((self.seastate.value*0.99).iter().max().unwrap());
}
if !ap.enabled.value {
return;
}
let t1 = time.monotonic();
let command = fuzzy_compute(0, self.dimensions, self.matrix);
let t2 = time.monotonic();
ap.servo.command.set(command);
let t3 = time.monotonic();
let P = ap.heading_error.value;
let D = ap.boatimu.SensorValues["headingrate_lowpass"].value;
let error = ((P*self.learningP.value) + (D*self.learningD.value));
let state = self.dimensions.iter().map(|x| x[1].value).collect::<Vec<_>>();
let t = time.monotonic();
if (t - self.history_time) > 0.2 {
self.history.clear();
}
self.history_time = t;
self.history.append((state, error));
let t4 = time.monotonic();
if self.history.len() == self.history_count {
let (prev, self.history) = (self.history[0], self.history[1..]);
let (prev_state, prev_error) = prev;
fuzzy_train(self.dimensions, self.matrix, prev_state, error);
let t5 = time.monotonic();
if (t - self.matrix_time) > 600 {
println!("{:?} ","fuzzy store");
self.store();
self.matrix_time = t;
}
}
} 
}
const pilot: _ = FuzzyPilot;