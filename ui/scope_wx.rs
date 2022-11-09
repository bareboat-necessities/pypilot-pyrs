use std::*;
use std::collections::HashMap;

use OpenGL::GL::{*};
use pypilot::client::{pypilotClientFromArgs};
sys.path.append(os.path.dirname(os.path.abspath(__file__)));
use scope_ui::{pypilotScopeBase};
use scope::{pypilotPlot};
fn wxglutkeypress<T0, T1, T2>(event: T0, special: T1, key: T2)  {
let translation = [(wx.WXK_UP, GLUT_KEY_UP), (wx.WXK_DOWN, GLUT_KEY_DOWN), (wx.WXK_LEFT, GLUT_KEY_LEFT), (wx.WXK_RIGHT, GLUT_KEY_RIGHT), (wx.WXK_INSERT, GLUT_KEY_INSERT), (wx.WXK_DELETE, GLUT_KEY_DELETE)].iter().cloned().collect::<HashMap<_,_>>();
if translation.iter().any(|&x| x == event.GetKeyCode()) {
special(translation[event.GetKeyCode()], event.GetPosition().x, event.GetPosition().y);
} else {
let code = event.GetKeyCode();
if code < 255 {
let mut k = ("%c" % code);
if !(event.GetModifiers() & wx.MOD_SHIFT) {
k = k.lower();
}
key(k, event.GetPosition().x, event.GetPosition().y);
}
}
}
struct pypilotScope {
plot: ST0,
glContext: ST1,
args: Vec<_>,
client: ST2,
watches: HashMap<_,_>,
timer: ST3,
plot_reshape: bool,
lastmouse: ST4,
}

impl pypilotScope {
fn __init__(&self)  {
super(pypilotScope, self).__init__(None);
self.plot = pypilotPlot();
self.glContext = wx.glcanvas.GLContext(self.glArea);
let host = false;
self.args = vec![];
if sys.argv.len() > 1 {
let (host, self.args) = (sys.argv[1], sys.argv[2..]);
}
self.client = pypilotClientFromArgs(self.args, host);
self.client.watch("timestamp");
self.watches = HashMap::new();
self.timer = wx.Timer(self, wx.ID_ANY);
self.Bind(wx.EVT_TIMER, self.receive_messages, wx.ID_ANY);
self.timer.Start(100);
self.sTime.SetValue(self.plot.disptime);
self.plot_reshape = false;
}
fn enumerate_values<T0>(&self, value_list: T0)  {
let watches = self.args;
for name in sorted(value_list) {
if value_list[name]["type"] != "SensorValue"||name == "timestamp" {
continue;
}
let i = self.clValues.Append(name);
self.watches[name] = false;
for arg in watches {
if arg == name {
self.clValues.Check(i, true);
self.watches[name] = true;
watches.remove(name);
break;
}
}
}
if watches {
println!("{:?} {:?} ",_("values not found:"), watches);
}
}
fn receive_messages<T0>(&self, event: T0)  {
if !self.client {
let try_dummy = { //unsupported
let (host, port) = self.host_port;
self.client = pypilotClient(self.on_con, host, port, false);
self.timer.Start(50);
};
let except!(socket.error) = { //unsupported
self.timer.Start(1000);
return;
};
}
if !self.plot.value_list {
let value_list = self.client.list_values();
if value_list {
self.enumerate_values(value_list);
self.plot.init(value_list);
}
return;
}
let mut refresh = false;
self.client.poll();
if !self.client.connection {
self.plot.add_blank();
return;
}
while true {
let result = self.client.receive_single();
if !result {
break;
}
let (name, value) = result;
if name == "timestamp"||self.watches[name] {
if self.plot.read_data(result) {
refresh = true;
}
}
}
if refresh {
self.glArea.Refresh();
}
}
fn onValueSelected<T0>(&self, event: T0)  {
self.plot.select(self.clValues.GetStringSelection());
}
fn onValueToggled<T0>(&self, event: T0)  {
let value = self.clValues.IsChecked(event.GetInt());
self.watches[event.GetString()] = value;
self.client.watch(event.GetString(), value);
self.plot.add_blank(event.GetString());
}
fn onPaintGL<T0>(&self, event: T0)  {
let dc = wx.PaintDC(self.glArea);
self.glArea.SetCurrent(self.glContext);
self.plot.fft_on = self.cbfftw.GetValue();
if self.plot_reshape {
self.plot.reshape(starred!(self.plot_reshape)/*unsupported*/);
self.plot_reshape = false;
}
self.plot.display();
self.glArea.SwapBuffers();
}
fn onSizeGL<T0>(&self, event: T0)  {
self.plot_reshape = (event.GetSize().x, event.GetSize().y);
}
fn onMouseEvents<T0>(&self, event: T0)  {
self.glArea.SetFocus();
let pos = event.GetPosition();
if event.LeftDown() {
self.lastmouse = pos;
}
if event.RightDown() {
self.plot.curtrace.center();
self.glArea.Refresh();
}
if event.Dragging() {
let offset = (pos[1] - self.lastmouse[1]);
self.plot.adjustoffset(offset, self.glArea.GetSize().y);
self.lastmouse = pos;
self.glArea.Refresh();
}
let rotation = (event.GetWheelRotation()/60);
if rotation {
if rotation > 0 {
self.plot.increasescale();
} else {
self.plot.decreasescale();
}
self.glArea.Refresh();
}
}
fn onKeyPress<T0>(&self, event: T0)  {
wxglutkeypress(event, self.plot.special, self.plot.key);
self.cbfftw.SetValue(self.plot.fft_on);
self.glArea.Refresh();
}
fn onZero<T0>(&self, event: T0)  {
if self.plot.curtrace {
self.plot.curtrace.offset = 0;
self.glArea.Refresh();
}
}
fn onCenter<T0>(&self, event: T0)  {
if self.plot.curtrace {
self.plot.curtrace.center();
self.glArea.Refresh();
}
}
fn onScalePlus<T0>(&self, event: T0)  {
self.plot.increasescale();
self.glArea.Refresh();
}
fn onScaleMinus<T0>(&self, event: T0)  {
self.plot.decreasescale();
self.glArea.Refresh();
}
fn onOffsetPlus<T0>(&self, event: T0)  {
self.plot.curtrace.offset -= (self.plot.scale/10.0);
self.glArea.Refresh();
}
fn onOffsetMinus<T0>(&self, event: T0)  {
self.plot.curtrace.offset += (self.plot.scale/10.0);
self.glArea.Refresh();
}
fn onFreeze<T0>(&self, event: T0)  {
self.plot.freeze = event.IsChecked();
self.glArea.Refresh();
}
fn onReset<T0>(&self, event: T0)  {
self.plot.reset();
self.glArea.Refresh();
}
fn onTime<T0>(&self, event: T0)  {
self.plot.disptime = self.sTime.GetValue();
self.glArea.Refresh();
}
fn onClose<T0>(&self, event: T0)  {
self.Close();
} 
}
use OpenGL::GLUT::{*};
fn main()  {
glutInit(sys.argv);
let app = wx.App();
pypilotScope().Show();
app.MainLoop();
}
fn main() {
main();
}