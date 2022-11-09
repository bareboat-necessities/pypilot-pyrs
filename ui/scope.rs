use std::*;
use std::collections::HashMap;

use OpenGL::GLUT::{*};
use OpenGL::GLU::{*};
use OpenGL::GL::{*};
use pypilot::client::{pypilotClientFromArgs};
struct trace {
points: Vec<_>,
offset: ST0,
visible: bool,
timeoff: bool,
name: ST1,
group: ST2,
color: ST3,
directional: ST4,
}

impl trace {
let colors = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1], vec![1, 1, 1], vec![1, 0.5, 0], vec![0.5, 1, 0], vec![0.5, 0.5, 0.5], vec![0, 0.5, 0.5], vec![0.5, 0, 1]];
fn __init__<T0, T1, T2, T3>(&self, name: T0, group: T1, colorindex: T2, directional: T3)  {
self.points = vec![];
self.offset = 0;
self.visible = true;
self.timeoff = false;
self.name = name;
self.group = group;
self.color = self.colors[(colorindex % self.colors.len())];
self.directional = directional;
}
fn add<T0, T1, T2, RT>(&self, t: T0, data: T1, mindt: T2) -> RT {
if self.points&&math.isnan(self.points[0][1]) {
let dt = ((time.monotonic() - t) - self.timeoff);
self.timeoff = false;
for i in (0..self.points.len()) {
let point = self.points[i];
self.points[i] = ((point[0] - dt), point[1]);
}
}
if !self.timeoff||self.timeoff < (time.monotonic() - t)||self.timeoff > ((time.monotonic() - t) + 1) {
self.timeoff = (time.monotonic() - t);
} else {
if self.points&&(t - self.points[0][0]) < mindt {
return false;
}
}
self.points.insert(0, (t, data));
return true;
}
fn add_blank(&self)  {
if self.points {
self.points.insert(0, (self.points[0][0], float("nan")));
}
}
fn center(&self)  {
if self.points.len() > 0 {
self.offset = self.points[0][1];
}
}
fn noise<RT>(&self) -> RT {
let try_dummy = { //unsupported
let avg = (self.points.iter().map(|x| x[1]).iter().sum()/self.points.len());
return (math.sqrt(self.points.iter().map(|x| (avg - x[1]).pow(2)).iter().sum())/self.points.len());
};
let except!() = { //unsupported
return 0;
};
}
fn tracevertexes<T0, T1, T2>(&self, time: T0, plot: T1, gldrawtype: T2)  {
for i in (0..self.points.len()) {
if self.points[i][0] < (time - plot.disptime) {
self.points = self.points[..(i + 1)];
break;
}
}
glBegin(gldrawtype);
for point in self.points {
if math.isnan(point[1]) {
glEnd();
glBegin(gldrawtype);
} else {
let mut y = (point[1] - self.offset);
if self.directional {
if y >= 180 {
y -= 360;
} else {
if y < -180 {
y += 360;
}
}
}
glVertex2d((point[0] - time), y);
}
}
glEnd();
}
fn draw<T0>(&self, plot: T0)  {
if !self.visible||!self.timeoff {
return;
}
let t = (time.monotonic() - self.timeoff);
glPushMatrix();
glColor3dv(self.color);
self.tracevertexes(t, plot, GL_LINE_STRIP);
if plot.drawpoints {
glPointSize(8);
self.tracevertexes(t, plot, GL_POINTS);
}
glPopMatrix();
}
fn draw_fft(&self)  {
if self.points.len() < 1 {
return;
}
let pts = self.points.iter().map(|p| (p[1] - self.offset));
let out = numpy.fft.rfft(pts);
let c = out.len();
let mut norm = 0;
for i in (0..(c/2)) {
norm += (numpy.real(out[i]).pow(2) + numpy.imag(out[i]).pow(2));
}
norm = math.sqrt(norm);
if norm <= 0 {
return;
}
for i in (1..pypilotPlot::NUM_X_DIV) {
let x = (float(i)/pypilotPlot::NUM_X_DIV);
self.rasterpos(vec![x, 0.95]);
let period = (3/math.exp(x));
pypilotPlot::drawputs(String::from(period));
}
glPushMatrix();
glBegin(GL_LINE_STRIP);
for i in (0..(c/2)) {
glVertex2d(((float(i)*2)/(c - 2)), (abs(out[i])/norm));
}
glEnd();
glPopMatrix();
} 
}
struct pypilotPlot {
value_list: bool,
freeze: bool,
drawpoints: bool,
scale: ST0,
scalestate: ST1,
delay: ST2,
disptime: ST3,
curtrace: bool,
fft_on: bool,
starttime: ST4,
traces: Vec<_>,
timestamp: bool,
lastrasterpos: ST5,
width: ST6,
}

impl pypilotPlot {
const NUM_X_DIV: _ = 5;
const NUM_Y_DIV: _ = 6;
const FONT: _ = GLUT_BITMAP_TIMES_ROMAN_24;
fn __init__(&self)  {
self.value_list = false;
self.freeze = false;
self.drawpoints = false;
self.scale = 1.0;
self.scalestate = 0;
self.delay = 0;
self.disptime = 30;
self.curtrace = false;
self.fft_on = false;
self.starttime = 0;
self.reset();
}
fn reset(&self)  {
self.traces = vec![];
self.timestamp = false;
}
fn add_data<T0, T1, T2, T3, RT>(&self, name: T0, group: T1, timestamp: T2, value: T3) -> RT {
let mut t = false;
for tn in self.traces {
if tn.name == name {
t = tn;
break;
}
}
if !t {
for tn in self.traces {
if name == group&&tn.group == group {
return;
}
}
let directional = self.value_list.iter().any(|&x| x == name)&&self.value_list[name].iter().any(|&x| x == "directional")&&self.value_list[name]["directional"];
t = trace(name, group, self.traces.len(), directional);
self.traces.append(t);
self.curtrace = t;
}
let mindt = (self.disptime/float(self.width));
return t.add(timestamp, value, mindt)&&t.visible;
}
fn add_blank<T0>(&self, group: T0)  {
for t in self.traces {
if !group||group == t.group {
t.add_blank();
}
}
}
fn read_data<T0, RT>(&self, msg: T0) -> RT {
let (name, value) = msg;
if name == "timestamp" {
self.timestamp = value;
return;
}
if !self.timestamp {
return;
}
let timestamp = self.timestamp;
if type_(value) == type_(vec![]) {
let mut ret = false;
for i in (0..value.len()) {
let namei = (name + String::from(i));
ret = self.add_data(namei, name, timestamp, float(value[i]))||ret;
}
return ret;
} else {
if type_(value) == type_(true) {
if value {
value = 1;
} else {
value = 0;
}
}
return self.add_data(name, name, timestamp, float(value));
}
}
fn drawputs<T0>(str: T0)  {
for c in str {
glutBitmapCharacter(pypilotPlot::FONT, ctypes.c_int(ord(c)));
}
}
fn synccolor(&self)  {
let pos = glGetDoublev(GL_CURRENT_RASTER_POSITION)[..2];
let vp = glGetDoublev(GL_VIEWPORT);
self.lastrasterpos = ((pos[0]/vp[2]), (pos[1]/vp[3]));
glRasterPos2d(starred!(self.lastrasterpos)/*unsupported*/);
}
fn rasterpos<T0>(&self, pos: T0)  {
glRasterPos2d(starred!(pos)/*unsupported*/);
self.lastrasterpos = pos;
}
fn drawticks(&self)  {
glLineWidth(1);
glEnable(GL_LINE_STIPPLE);
glColor3d(0.6, 0.6, 0.6);
glLineStipple(1, 17);
glBegin(GL_LINES);
for i in (1..pypilotPlot::NUM_X_DIV) {
let x = (float(i)/pypilotPlot::NUM_X_DIV);
glVertex2d(x, 0);
glVertex2d(x, 1);
}
for i in (1..pypilotPlot::NUM_Y_DIV) {
let y = (float(i)/pypilotPlot::NUM_Y_DIV);
glVertex2d(0, y);
glVertex2d(1, y);
}
glEnd();
glDisable(GL_LINE_STIPPLE);
}
fn drawtext(&self)  {
if !self.curtrace {
return;
}
glColor3d(1, 1, 1);
self.rasterpos(vec![0, 0.01]);
let mut i = 1;
for t in self.traces {
glColor3dv(t.color);
self.synccolor();
pypilotPlot::drawputs(("%d " % i));
i += 1;
}
glColor3dv(self.curtrace.color);
self.synccolor();
let mut val = float("nan");
if self.curtrace.points.len() {
val = self.curtrace.points[0][1];
}
pypilotPlot::drawputs(("name: %s offset: %g  value: %g  visible: %s  " % (self.curtrace.name, self.curtrace.offset, val, if self.curtrace.visible { "T" } else { "F" })));
glColor3d(1, 1, 1);
pypilotPlot::drawputs(("scale: %g  time: %g  " % (self.scale, self.disptime)));
glColor3dv(self.curtrace.color);
pypilotPlot::drawputs(("noise: %g" % self.curtrace.noise()));
}
fn init<T0>(&self, value_list: T0)  {
glClearColor(0.0, 0.0, 0.0, 0.0);
glEnable(GL_BLEND);
glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
self.value_list = value_list;
}
fn display(&self)  {
if self.freeze {
return;
}
glClear(GL_COLOR_BUFFER_BIT);
self.drawticks();
if self.fft_on {
self.curtrace.draw_fft();
}
glPushMatrix();
glScaled((1.0/self.disptime), 0.5, 1);
glTranslated(self.disptime, 0.5, 0);
glTranslated(0, 0.5, 0);
glScaled(1, (2/(self.scale*pypilotPlot::NUM_Y_DIV)), 1);
glLineWidth(1);
for t in self.traces {
t.draw(self);
}
glPopMatrix();
self.drawtext();
}
fn reshape<T0, T1>(&self, w: T0, h: T1)  {
glViewport(0, 0, w, h);
glMatrixMode(GL_PROJECTION);
glLoadIdentity();
glMatrixMode(GL_MODELVIEW);
glLoadIdentity();
gluOrtho2D(0, 1, 0, 1);
self.width = w;
}
fn increasescale(&self)  {
if (self.scalestate % 3) == 1 {
self.scale *= 2.5;
} else {
self.scale *= 2;
}
self.scalestate += 1;
}
fn decreasescale(&self)  {
if (self.scalestate % 3) == 2 {
self.scale /= 2.5;
} else {
self.scale /= 2;
}
self.scalestate -= 1;
}
fn adjustoffset<T0, T1>(&self, offset: T0, y: T1)  {
self.curtrace.offset += (((offset*self.scale)*self.NUM_Y_DIV)/y);
}
fn key<T0, T1, T2>(&self, k: T0, x: T1, y: T2)  {
if k == "q"||k == 27 {
exit(0);
}
if !self.curtrace {
return;
}
if k >= "0"&&k <= "9" {
let mut ind = (i32::from(k) - 1);
if ind < 0 {
ind += 10;
}
if self.traces.len() <= ind {
return;
}
self.curtrace = self.traces[ind];
} else {
if k == "+"||k == "=" {
self.increasescale();
} else {
if k == "-"||k == "_" {
self.decreasescale();
} else {
if k == "f" {
self.freeze = !self.freeze;
} else {
if k == "p" {
self.drawpoints = !self.drawpoints;
} else {
if k == "c" {
self.curtrace.center();
} else {
if k == "C" {
for trace in self.traces {
trace::center();
}
} else {
if k == "v" {
self.curtrace.visible = !self.curtrace.visible;
} else {
if k == "V" {
let v = !self.curtrace.visible;
for trace in self.traces {
trace::visible = v;
}
} else {
if k == "z" {
self.curtrace.offset = 0;
} else {
if k == "Z" {
for trace in self.traces {
trace::offset = 0;
}
} else {
if k == "w" {
self.fft_on = !self.fft_on;
}
}
}
}
}
}
}
}
}
}
}
}
}
fn special<T0, T1, T2>(&self, key: T0, x: T1, y: T2)  {
if !self.curtrace {
return;
}
let dist = (self.scale/10.0);
if key == GLUT_KEY_DOWN {
self.curtrace.offset += dist;
} else {
if key == GLUT_KEY_UP {
self.curtrace.offset -= dist;
} else {
if key == GLUT_KEY_F11 {
glutFullScreen();
}
}
}
}
fn select<T0>(&self, name: T0)  {
for t in self.traces {
if t.name.iter().any(|&x| x == name) {
self.curtrace = t;
break;
}
}
} 
}
fn main()  {
let plot = pypilotPlot();
let (host, args) = (false, vec![]);
if sys.argv.len() > 1 {
let (host, args) = (sys.argv[1], sys.argv[2..]);
}
let client = pypilotClientFromArgs(args, host);
fn idle()  {
while true {
let try_dummy = { //unsupported
let result = client.receive_single();
if result {
plot.read_data(result);
} else {
time.sleep(0.01);
break;
}
};
let except!() = { //unsupported
/*pass*/
};
}
}
glutInit(sys.argv);
glutInitWindowPosition(250, 0);
glutInitWindowSize(1000, 500);
glutInitDisplayMode((GLUT_DOUBLE | GLUT_RGB));
glutCreateWindow("glplot");
fn display()  {
plot.display();
glutSwapBuffers();
}
glutDisplayFunc(display);
glutReshapeFunc(plot.reshape);
glutKeyboardFunc(plot.key);
glutSpecialFunc(plot.special);
glutIdleFunc(idle);
plot.init(client.list_values(10));
let fps = 30;
fn timeout<T0>(arg: T0)  {
glutPostRedisplay();
glutTimerFunc(i32::from((1000/fps)), timeout, arg);
}
glutTimerFunc(0, timeout, None);
glutMainLoop();
}
fn main() {
main();
}