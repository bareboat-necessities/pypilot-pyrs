use std::*;
use std::collections::HashMap;

sys.path.append(os.path.dirname(os.path.abspath(__file__)));
use pypilot::{quaternion};
use pypilot::client::{pypilotClient};
use client_wx::{round3};
use OpenGL::GL::{*};
use OpenGL::GLU::{*};
use OpenGL::GLUT::{*};
struct CalibrationDialog {
host: ST0,
client: bool,
accel_calibration_plot: ST1,
accel_calibration_glContext: ST2,
compass_calibration_plot: ST3,
compass_calibration_glContext: ST4,
boat_plot: ST5,
boat_plot_glContext: ST6,
lastmouse: bool,
alignment_count: ST7,
timer: ST8,
heading_offset_timer: ST9,
have_rudder: bool,
fusionQPose: ST10,
alignmentQ: ST11,
controltimes: HashMap<_,_>,
settings: HashMap<_,_>,
pypilot_heading_offset: ST12,
}

impl CalibrationDialog {
const ID_MESSAGES: _ = 1000;
const ID_CALIBRATE_SERVO: _ = 1001;
const ID_HEADING_OFFSET: _ = 1002;
fn __init__(&self)  {
super(CalibrationDialog, self).__init__(None);
self.host = "";
if sys.argv.len() > 1 {
self.host = sys.argv[1];
}
self.client = false;
self.accel_calibration_plot = calibration_plot.AccelCalibrationPlot();
self.accel_calibration_glContext = wx.glcanvas.GLContext(self.AccelCalibration);
self.compass_calibration_plot = calibration_plot.CompassCalibrationPlot();
self.compass_calibration_glContext = wx.glcanvas.GLContext(self.CompassCalibration);
self.boat_plot = boatplot.BoatPlot();
self.boat_plot_glContext = wx.glcanvas.GLContext(self.BoatPlot);
self.lastmouse = false;
self.alignment_count = 0;
self.timer = wx.Timer(self, self.ID_MESSAGES);
self.timer.Start(50);
self.Bind(wx.EVT_TIMER, self.receive_messages, self.ID_MESSAGES);
self.heading_offset_timer = wx.Timer(self, self.ID_HEADING_OFFSET);
self.Bind(wx.EVT_TIMER, |e| self.sHeadingOffset.SetValue(round3(self.pypilot_heading_offset)), self.ID_HEADING_OFFSET);
self.have_rudder = false;
self.fusionQPose = vec![1, 0, 0, 0];
self.alignmentQ = vec![1, 0, 0, 0];
self.controltimes = HashMap::new();
self.client = pypilotClient(self.host);
self.accel_calibration_plot.points = vec![];
self.compass_calibration_plot.points = vec![];
self.settings = HashMap::new();
self.set_watches();
}
fn set_watches<RT>(&self) -> RT {
if !self.client {
return;
}
fn calwatch<T0, RT>(name: T0) -> RT {
name = ("imu." + name);
return vec![(name + ".calibration"), (name + ".calibration.age"), (name, 0.2), (name + ".calibration.sigmapoints"), (name + ".calibration.points"), (name + ".calibration.locked"), (name + ".calibration.log")];
}
let watchlist = vec![vec!["imu.fusionQPose", ("imu.alignmentCounter", 0.2), ("imu.heading", 0.5), ("imu.alignmentQ", 1), ("imu.pitch", 0.5), ("imu.roll", 0.5), ("imu.heel", 0.5), ("imu.heading_offset", 1)], calwatch("accel"), (calwatch("compass") + vec!["imu.fusionQPose"]), vec!["rudder.offset", "rudder.scale", "rudder.nonlinearity", ("rudder.angle", 1), "rudder.range", "servo.flags"], self.settings.collect::<Vec<_>>()];
let pageindex = self.m_notebook.GetSelection();
let watches = HashMap::new();
for i in (0..watchlist.len()) {
let pagelist = watchlist[i];
for name in watchlist[pageindex] {
if i == pageindex {
if type_(name) == type_(()) {
let (name, watch) = name;
} else {
let watch = true;
}
watches[name] = watch;
} else {
if self.client.watches.iter().any(|&x| x == name) {
self.client.watch(name, false);
}
}
}
}
for (name, watch) in watches.items() {
self.client.watch(name, watch);
}
}
fn enumerate_settings<T0>(&self, values: T0)  {
let fgSettings = self.m_pSettings.GetSizer();
if !fgSettings {
fgSettings = wx.FlexGridSizer(0, 3, 0, 0);
fgSettings.AddGrowableCol(1);
fgSettings.SetFlexibleDirection(wx.BOTH);
fgSettings.SetNonFlexibleGrowMode(wx.FLEX_GROWMODE_SPECIFIED);
self.m_pSettings.SetSizer(fgSettings);
}
let lvalues = values.collect::<Vec<_>>();
lvalues.sort();
for name in lvalues {
if self.settings.iter().any(|&x| x == name) {
continue;
}
if values[name].iter().any(|&x| x == "units") {
let v = values[name];
fn proc()  {
let s = wx.SpinCtrlDouble(self.m_pSettings, wx.ID_ANY);
s.SetRange(v["min"], v["max"]);
s.SetIncrement(1.iter().min().unwrap());
s.SetDigits(((-math.log(s.GetIncrement())/math.log(10)) + 1));
self.settings[name] = s;
fgSettings.Add(wx.StaticText(self.m_pSettings, wx.ID_ANY, name), 0, wx.ALL, 5);
fgSettings.Add(s, 0, (wx.ALL | wx.EXPAND), 5);
fgSettings.Add(wx.StaticText(self.m_pSettings, wx.ID_ANY, v["units"]), 0, wx.ALL, 5);
let sname = name;
fn onspin<T0>(event: T0)  {
self.client.set(sname, s.GetValue());
}
s.Bind(wx.EVT_SPINCTRLDOUBLE, onspin);
}
proc();
}
}
self.m_pSettings.Layout();
fgSettings.Fit(self.m_pSettings);
}
fn receive_messages<T0>(&self, event: T0)  {
self.client.poll();
let values_list = self.client.list_values();
if values_list {
self.enumerate_settings(values_list);
}
let mut msg = self.client.receive_single();
while msg {
self.receive_message(msg);
msg = self.client.receive_single();
}
self.timer.Start(50);
return;
let try_dummy = { //unsupported
msg = self.client.receive_single();
while msg {
self.receive_message(msg);
msg = self.client.receive_single();
}
self.timer.Start(50);
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ",_("exception in calibration:"), e);
};
}
fn UpdateControl<T0, T1>(&self, control: T0, update: T1)  {
let t = time.monotonic();
if !self.controltimes.iter().any(|&x| x == control)||(t - self.controltimes[control]) > 0.5 {
update();
self.controltimes[control] = t;
}
}
fn UpdateLabel<T0, T1>(&self, label: T0, value: T1)  {
self.UpdateControl(label, || label.SetLabel(String::from(value)));
}
fn UpdatedSpin<T0, T1>(&self, dspin: T0, value: T1)  {
self.UpdateControl(dspin, || dspin.SetValue(value));
}
fn receive_message<T0>(&self, msg: T0)  {
let (name, value) = msg;
if 1 {
if name == "imu.alignmentQ" {
self.stAlignment.SetLabel(((String::from(round3(value)) + " ") + String::from(math.degrees(quaternion::angle(value)))));
self.alignmentQ = value;
} else {
if name == "imu.fusionQPose" {
if !value {
return;
}
let aligned = quaternion::normalize(quaternion::multiply(value, self.alignmentQ));
let value = aligned;
if self.cCoords.GetSelection() == 1 {
self.boat_plot.Q = quaternion::multiply(self.boat_plot.Q, self.fusionQPose);
self.boat_plot.Q = quaternion::multiply(self.boat_plot.Q, quaternion::conjugate(aligned));
} else {
if self.cCoords.GetSelection() == 2 {
let ang = (quaternion::toeuler(self.fusionQPose)[2] - quaternion::toeuler(aligned)[2]);
self.boat_plot.Q = quaternion::multiply(self.boat_plot.Q, quaternion::angvec2quat(ang, vec![0, 0, 1]));
}
}
self.fusionQPose = value;
self.BoatPlot.Refresh();
} else {
if name == "imu.alignmentCounter" {
self.gAlignment.SetValue((100 - value));
let enable = value == 0;
self.bLevel.Enable(enable);
} else {
if name == "imu.pitch" {
self.stPitch.SetLabel(String::from(round3(value)));
} else {
if name == "imu.roll" {
self.stRoll.SetLabel(String::from(round3(value)));
} else {
if name == "imu.heel" {
self.stHeel.SetLabel(String::from(round3(value)));
} else {
if name == "imu.heading" {
self.stHeading.SetLabel(String::from(round3(value)));
} else {
if name == "imu.heading_offset" {
self.pypilot_heading_offset = value;
self.heading_offset_timer.Start(1000, true);
}
}
}
}
}
}
}
}
self.accel_calibration_plot.read_data(msg);
if name == "imu.accel" {
self.AccelCalibration.Refresh();
} else {
if name == "imu.accel.calibration" {
self.stAccelCal.SetLabel(String::from(round3(value)));
} else {
if name == "imu.accel.calibration.age" {
self.stAccelCalAge.SetLabel(String::from(value));
} else {
if name == "imu.accel.calibration.locked" {
self.cbAccelCalibrationLocked.SetValue(value);
} else {
if name == "imu.accel.calibration.log" {
self.tAccelCalibrationLog.WriteText((value + "
"));
}
}
}
}
}
self.compass_calibration_plot.read_data(msg);
if name == "imu.compass" {
self.CompassCalibration.Refresh();
} else {
if name == "imu.compass.calibration" {
self.stCompassCal.SetLabel(String::from(round3(value)));
} else {
if name == "imu.compass.calibration.age" {
self.stCompassCalAge.SetLabel(String::from(value));
} else {
if name == "imu.compass.calibration.locked" {
self.cbCompassCalibrationLocked.SetValue(value);
} else {
if name == "imu.compass.calibration.log" {
self.tCompassCalibrationLog.WriteText((value + "
"));
}
}
}
}
}
if name == "rudder.angle" {
self.UpdateLabel(self.stRudderAngle, String::from(round3(value)));
self.have_rudder = type_(value) != type_(bool);
} else {
if name == "rudder.offset" {
self.UpdateLabel(self.stRudderOffset, String::from(round3(value)));
} else {
if name == "rudder.scale" {
self.UpdateLabel(self.stRudderScale, String::from(round3(value)));
} else {
if name == "rudder.nonlinearity" {
self.UpdateLabel(self.stRudderNonlinearity, String::from(round3(value)));
} else {
if name == "rudder.range" {
self.UpdatedSpin(self.sRudderRange, value);
} else {
if name == "servo.flags" {
self.stServoFlags.SetLabel(value);
}
}
}
}
}
}
if self.settings.iter().any(|&x| x == name) {
self.UpdatedSpin(self.settings[name], value);
}
}
}
fn servo_console<T0>(&self, text: T0)  {
self.stServoCalibrationConsole.SetLabel(((self.stServoCalibrationConsole.GetLabel() + text) + "
"));
}
fn PageChanged<T0>(&self, event: T0)  {
self.set_watches();
}
fn onKeyPressAccel<T0>(&self, event: T0)  {
self.onKeyPress(event, self.compass_calibration_plot);
}
fn onKeyPressCompass<T0>(&self, event: T0)  {
self.onKeyPress(event, self.compass_calibration_plot);
}
fn onKeyPress<T0, T1>(&self, event: T0, plot: T1)  {
scope_wx.wxglutkeypress(event, plot.special, plot.key);
}
fn onClearAccel<T0>(&self, event: T0)  {
self.accel_calibration_plot.points = vec![];
}
fn onClearCompass<T0>(&self, event: T0)  {
self.compass_calibration_plot.points = vec![];
}
fn onAccelCalibrationLocked<T0>(&self, event: T0)  {
self.client.set("imu.accel.calibration.locked", self.cbAccelCalibrationLocked.GetValue());
}
fn onCompassCalibrationLocked<T0>(&self, event: T0)  {
self.client.set("imu.compass.calibration.locked", self.cbCompassCalibrationLocked.GetValue());
}
fn onCalibrationLocked<T0, T1>(&self, sensor: T0, ctrl: T1)  {
self.client.set((("imu." + sensor) + ".calibration.locked"), self.ctrl.GetValue());
}
fn onMouseEventsAccel<T0>(&self, event: T0)  {
self.AccelCalibration.SetFocus();
self.onMouseEvents(event, self.AccelCalibration, self.accel_calibration_plot);
}
fn onMouseEventsCompass<T0>(&self, event: T0)  {
self.CompassCalibration.SetFocus();
self.onMouseEvents(event, self.CompassCalibration, self.compass_calibration_plot);
}
fn onMouseEvents<T0, T1, T2>(&self, event: T0, canvas: T1, plot: T2)  {
let pos = event.GetPosition();
if event.LeftDown() {
self.lastmouse = pos;
}
if event.Dragging() {
if self.lastmouse {
calibration_plot.rotate_mouse((pos[0] - self.lastmouse[0]), (pos[1] - self.lastmouse[1]));
canvas.Refresh();
}
self.lastmouse = pos;
}
let mut rotation = (event.GetWheelRotation()/60);
if rotation {
canvas.Refresh();
while rotation > 0 {
plot.userscale /= 0.9;
rotation -= 1;
}
while rotation < 0 {
plot.userscale *= 0.9;
rotation += 1;
}
}
}
fn onPaintGLAccel<T0>(&self, event: T0)  {
self.onPaintGL(self.AccelCalibration, self.accel_calibration_plot, self.accel_calibration_glContext);
}
fn onPaintGLCompass<T0>(&self, event: T0)  {
self.onPaintGL(self.CompassCalibration, self.compass_calibration_plot, self.compass_calibration_glContext);
}
fn onPaintGL<T0, T1, T2>(&self, canvas: T0, plot: T1, context: T2)  {
wx.PaintDC(canvas);
canvas.SetCurrent(context);
plot.display();
canvas.SwapBuffers();
}
fn onSizeGLAccel<T0>(&self, event: T0)  {
self.accel_calibration_plot.reshape(event.GetSize().x, event.GetSize().y);
}
fn onSizeGLCompass<T0>(&self, event: T0)  {
self.compass_calibration_plot.reshape(event.GetSize().x, event.GetSize().y);
}
fn onResetAlignment<T0>(&self, event: T0)  {
self.client.set("imu.alignmentQ", false);
}
fn onLevel<T0>(&self, event: T0)  {
self.client.set("imu.alignmentCounter", 100);
}
fn onIMUHeadingOffset<T0>(&self, event: T0)  {
self.client.set("imu.heading_offset", self.sHeadingOffset.GetValue());
self.heading_offset_timer.Stop();
}
fn onKeyPressBoatPlot<T0>(&self, event: T0)  {
self.BoatPlot.SetFocus();
let mut k = ("%c" % (event.GetKeyCode() & 255));
if !(event.GetModifiers() & wx.MOD_SHIFT) {
k = k.lower();
}
self.BoatPlot.Refresh();
}
fn onMouseEventsBoatPlot<T0>(&self, event: T0)  {
self.BoatPlot.SetFocus();
let pos = event.GetPosition();
if event.LeftDown() {
self.lastmouse = pos;
}
if event.Dragging() {
if self.lastmouse {
self.BoatPlot.Refresh();
let (dx, dy) = ((pos[0] - self.lastmouse[0]), (pos[1] - self.lastmouse[1]));
let q = quaternion::angvec2quat((((dx.pow(2) + dy.pow(2)).pow(0.4)/180)*math.pi), vec![dy, dx, 0]);
self.boat_plot.Q = quaternion::multiply(q, self.boat_plot.Q);
}
self.lastmouse = pos;
}
let mut rotation = (event.GetWheelRotation()/60);
if rotation {
while rotation > 0 {
self.boat_plot.Scale /= 0.9;
rotation -= 1;
}
while rotation < 0 {
self.boat_plot.Scale *= 0.9;
rotation += 1;
}
self.BoatPlot.Refresh();
}
}
fn onPaintGLBoatPlot<T0>(&self, event: T0)  {
wx.PaintDC(self.BoatPlot);
self.BoatPlot.SetCurrent(self.boat_plot_glContext);
self.boat_plot.reshape(self.BoatPlot.GetSize().x, self.BoatPlot.GetSize().y);
self.boat_plot.display(self.fusionQPose);
self.BoatPlot.SwapBuffers();
}
fn onSizeGLBoatPlot<T0>(&self, event: T0)  {
self.boat_plot.reshape(event.GetSize().x, event.GetSize().y);
self.BoatPlot.Refresh();
}
fn onTextureCompass<T0>(&self, event: T0)  {
self.boat_plot.texture_compass = event.IsChecked();
self.BoatPlot.Refresh();
}
fn onIMUScope<T0>(&self, event: T0)  {
let host = self.client.config["host"];
let args = vec!["pypilot_scope", host, "imu.pitch", "imu.roll", "imu.heel", "imu.heading"];
subprocess.Popen(args);
}
fn onRudderResetCalibration<T0>(&self, event: T0)  {
self.client.set("rudder.calibration_state", "reset");
}
fn onRudderCentered<T0>(&self, event: T0)  {
self.client.set("rudder.calibration_state", "centered");
}
fn onRudderStarboardRange<T0>(&self, event: T0)  {
self.client.set("rudder.calibration_state", "starboard range");
}
fn onRudderPortRange<T0>(&self, event: T0)  {
self.client.set("rudder.calibration_state", "port range");
}
fn onRudderRange<T0>(&self, event: T0)  {
self.client.set("rudder.range", self.sRudderRange.GetValue());
} 
}
fn main()  {
glutInit(sys.argv);
let app = wx.App();
CalibrationDialog().ShowModal();
}
fn main() {
main();
}