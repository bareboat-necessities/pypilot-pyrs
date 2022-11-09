use std::*;
use std::collections::HashMap;

const white: _ = 16777215;
const black: _ = 0;
let (AUTO, MENU, SMALL_PORT, SMALL_STARBOARD, SELECT, BIG_PORT, BIG_STARBOARD, TACK, NUDGE_PORT, NUDGE_STARBOARD, NUM_KEYS) = (0..11);
struct rectangle {

}

impl rectangle {
fn __init__<T0, T1, T2, T3>(&self, x: T0, y: T1, width: T2, height: T3)  {
let (self.x, self.y, self.width, self.height) = (x, y, width, height);
} 
}
const translate: _ = |x| x;
const no_translation: _ = translate;
fn _<T0, RT>(x: T0) -> RT {
return translate(x);
}
const locale_d: _ = "";
let try_dummy = { //unsupported
fn test_wifi<RT>() -> RT {
return wifi_esp32.connected[0];
}
fn gettime<RT>() -> RT {
return (time.ticks_ms()/1000.0);
}
let try_dummy = { //unsupported
};
let except!() = { //unsupported
println!("{:?} ","failed to import gettext");
};
};
let except!() = { //unsupported
fn gettime<RT>() -> RT {
return time.monotonic();
}
fn test_wifi<RT>() -> RT {
let try_dummy = { //unsupported
let wlan0 = open("/sys/class/net/wlan0/operstate");
let line = wlan0.readline().rstrip();
wlan0.close();
if line == "up" {
return true;
}
};
let except!() = { //unsupported
/*pass*/
};
return false;
}
let try_dummy = { //unsupported
locale_d = (os.path.abspath(os.path.dirname(__file__)) + "/");
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","failed to import gettext", e);
};
};
fn set_language<T0>(lang: T0)  {
println!("{:?} {:?} ","set language", lang);
let try_dummy = { //unsupported
let language = gettext.translation("pypilot_hat", (locale_d + "locale"), vec![lang], true);
//global translate
translate = language.gettext;
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} {:?} ","no language", lang, e);
};
}
struct page {
name: ST0,
frameperiod: ST1,
watches: HashMap<_,_>,
fittext_cache: Vec<_>,
}

impl page {
fn __init__<T0, T1>(&self, name: T0, frameperiod: T1)  {
self.name = name;
self.frameperiod = frameperiod;
self.watches = HashMap::new();
self.fittext_cache = vec![];
}
fn fill<T0>(&self, color: T0)  {
self.lcd.surface.fill(color);
}
fn text<T0, T1, T2, T3, RT>(&self, pos: T0, text: T1, size: T2, crop: T3) -> RT {
let surface = self.lcd.surface;
pos = (i32::from((pos[0]*surface.width)), i32::from((pos[1]*surface.height)));
size = i32::from(((size*surface.width)/48));
size = font.draw(surface, pos, text, size, self.lcd.bw, crop);
return ((float(size[0])/surface.width), (float(size[1])/surface.height));
}
fn fittextsizewordwrap<T0, T1, T2, T3, T4, RT>(&self, rect: T0, text: T1, metric_size: T2, bw: T3, surface: T4) -> RT {
let t0 = time.time();
let words = text.split(" ");
if !words||!words[0] {
return (0, "");
}
let spacewidth = font.draw(surface, false, " ", metric_size, self.lcd.bw)[0];
let mut metrics = vec![];
for word in words {
metrics.push((word, font.draw(surface, false, word, metric_size, bw)));
self.lcd.receive();
}
let t1 = time.time();
let widths = metrics.iter().map(|metric| metric[1][0]).collect::<Vec<_>>();
let maxwordwidth = starred!((widths + vec![0]))/*unsupported*/.iter().max().unwrap();
let totalwidth = (widths.iter().sum() + (spacewidth*(words.len() - 1)));
let t2 = time.time();
let mut size = 0;
let mut wrappos = maxwordwidth;
while true {
let (posx, posy) = (0, 0);
let mut curtext = "";
let mut lineheight = 0;
let mut maxw = 0;
let mut minfirstwidth = maxwordwidth;
for metric in metrics {
let (word, (width, height)) = metric;
if posx > 0 {
width += spacewidth;
}
if (posx + width) > wrappos {
curtext += "
";
let mut posx = 0;
posy += lineheight;
if width < minfirstwidth {
minfirstwidth = width;
}
lineheight = 0;
}
if posx > 0 {
curtext += " ";
}
curtext += word;
lineheight = lineheight.iter().max().unwrap();
posx += width;
maxw = maxw.iter().max().unwrap();
}
let maxh = (posy + lineheight);
if maxw == 0||maxh == 0 {
return (0, "");
}
let sw = ((surface.width*float(rect.width))/maxw);
let sh = ((surface.height*float(rect.height))/maxh);
let cursize = i32::from((sw*metric_size).iter().min().unwrap());
if cursize < size {
break;
}
size = cursize;
text = curtext;
if posy == 0 {
break;
}
wrappos += minfirstwidth;
}
let t3 = time.time();
return (size, text);
}
fn fittext<T0, T1, T2, T3, RT>(&self, rect: T0, text: T1, wordwrap: T2, fill: T3) -> RT {
let surface = self.lcd.surface;
let bw = self.lcd.bw;
if fill != "none" {
surface.box(starred!((self.convrect(rect) + vec![fill]))/*unsupported*/);
}
let metric_size = 16;
let mut ptext = text;
let spaces = text.iter().any(|&x| x == " ");
if spaces {
ntext = text;
} else {
ntext = "";
for c in text {
if c.isdigit() {
ntext += "0";
} else {
ntext += c;
}
}
}
for t in self.fittext_cache {
if t[0] == ntext {
let (t0, size, r, ptext) = t;
self.fittext_cache.remove(t);
if r.width == rect.width&&r.height == rect.height {
self.fittext_cache.append(t);
break;
}
}
}
let pos = (i32::from((rect.x*surface.width)), i32::from((rect.y*surface.height)));
if !ptext {
ptext = text;
}
let mut size = font.draw(surface, pos, ptext, size, bw);
return ((float(size[0])/surface.width), (float(size[1])/surface.height));
}
fn line<T0, T1, T2, T3>(&self, x1: T0, y1: T1, x2: T2, y2: T3)  {
let surface = self.lcd.surface;
let (w, h) = ((surface.width - 1), (surface.height - 1));
surface.line(i32::from((x1*w)), i32::from((y1*h)), i32::from(((x2*w) + 0.5)), i32::from(((y2*h) + 0.5)), white);
}
fn convbox<T0, T1, T2, T3, RT>(&self, x1: T0, y1: T1, x2: T2, y2: T3) -> RT {
fn bound<T0, RT>(x: T0) -> RT {
return x.iter().max().unwrap().iter().min().unwrap();
}
x1 = bound(x1);
y1 = bound(y1);
x2 = bound(x2);
y2 = bound(y2);
let surface = self.lcd.surface;
let (w, h) = ((surface.width - 1), (surface.height - 1));
return vec![i32::from(round((x1*w))), i32::from(round((y1*h))), i32::from(round((x2*w))), i32::from(round((y2*h)))];
}
fn invertrectangle<T0>(&self, rect: T0)  {
self.lcd.surface.invert(starred!(self.convbox(rect.x, rect.y, (rect.x + rect.width), (rect.y + rect.height)))/*unsupported*/);
}
fn convrect<T0, RT>(&self, rect: T0) -> RT {
return self.convbox(rect.x, rect.y, (rect.x + rect.width), (rect.y + rect.height));
}
fn rectangle<T0, T1>(&self, rect: T0, width: T1)  {
let surface = self.lcd.surface;
if !width {
surface.box(starred!((self.convrect(rect) + vec![white]))/*unsupported*/);
} else {
let box = self.convrect(rect);
surface.invert(starred!(box)/*unsupported*/);
if width {
let (w, h) = ((surface.width - 1), (surface.height - 1));
let px_width = i32::from(1.iter().max().unwrap());
surface.invert((box[0] + px_width), (box[1] + px_width), (box[2] - px_width), (box[3] - px_width));
}
}
}
fn box<T0, T1>(&self, rect: T0, color: T1)  {
let surface = self.lcd.surface;
surface.box(starred!((self.convrect(rect) + vec![color]))/*unsupported*/);
}
fn last_val<T0, T1, T2, RT>(&self, name: T0, period: T1, default: T2) -> RT {
if period == -1 {
period = self.frameperiod;
}
self.watches[name] = period;
if self.lcd.last_msg.iter().any(|&x| x == name) {
return self.lcd.last_msg[name];
}
return default;
}
fn round_last_val<T0, T1, RT>(&self, name: T0, places: T1) -> RT {
let v = self.last_val(name);
let try_dummy = { //unsupported
let n = 10.pow(places);
return String::from((round((v*n))/n));
};
let except!() = { //unsupported
return v;
};
}
fn testkeydown<T0, RT>(&self, key: T0) -> RT {
let k = self.lcd.keypad[key];
if k.down {
if self.lcd.config["buzzer"] > 1 {
self.lcd.send("buzzer", (1, 0.1));
}
k.down -= 1;
return true;
}
return false;
}
fn testkeyup<T0, RT>(&self, key: T0) -> RT {
let k = self.lcd.keypad[key];
if k.up {
k.up = false;
return true;
}
return false;
}
fn speed_of_keys<RT>(&self) -> RT {
let keypad = self.lcd.keypad;
let ss = (keypad[SMALL_STARBOARD].dt()*10);
let sp = (keypad[SMALL_PORT].dt()*10);
let bp = (keypad[BIG_PORT].dt()*10);
let bs = (keypad[BIG_STARBOARD].dt()*10);
let mut speed = 0;
let mut sign = 0;
if sp||ss {
speed = 0.6.iter().min().unwrap();
}
if bp||bs {
speed = 0.4.iter().max().unwrap();
}
if ss||bs {
sign = -1;
} else {
if sp||bp {
sign = 1;
}
}
return (sign*speed);
}
fn set<T0, T1>(&self, name: T0, value: T1)  {
self.lcd.client.set(name, value);
self.lcd.client.poll();
}
fn display<T0>(&self, refresh: T0)  {
/*pass*/
}
fn process<RT>(&self) -> RT {
if self.testkeydown(AUTO) {
return control(self.lcd);
}
if self.testkeydown(MENU) {
return self.lcd.getmenu();
}
if self.testkeydown(SELECT) {
if self.prev {
return self.prev;
}
return control(self.lcd);
}
let lcd = self.lcd;
if lcd.keypad[NUDGE_PORT].dt() {
lcd.client.set("servo.command", -1);
} else {
if lcd.keypad[NUDGE_STARBOARD].dt() {
lcd.client.set("servo.command", 1);
} else {
if self.testkeyup(NUDGE_PORT)||self.testkeyup(NUDGE_STARBOARD) {
lcd.client.set("servo.command", 0);
}
}
}
} 
}
struct info {
num_pages: ST0,
page: ST1,
watches: HashMap<_,_>,
}

impl info {
fn __init__<T0>(&self, num_pages: T0)  {
super(info, self).__init__(_("info"));
self.num_pages = num_pages;
self.page = 0;
}
fn bound_page(&self)  {
if self.page >= self.num_pages {
self.page = 0;
} else {
if self.page < 0 {
self.page = (self.num_pages - 1);
}
}
}
fn display<T0>(&self, refresh: T0)  {
self.bound_page();
self.watches = HashMap::new();
self.fill(black);
self.fittext(rectangle(0, 0, 1, 0.2), _("Info"));
let mut y = 0.2;
if self.page == 0 {
let mut spacing = 0.11;
let mut v = self.round_last_val("servo.watts", 3);
let runtime = self.last_val("ap.runtime")[..7];
let ah = self.round_last_val("servo.amp_hours", 3);
let mut items = vec![_("Watts"), v, _("Amp Hours"), ah, _("runtime"), runtime];
} else {
if self.page == 1 {
spacing = 0.11;
v = self.round_last_val("servo.voltage", 3);
let rate = self.round_last_val("imu.frequency", 2);
let uptime = self.last_val("imu.uptime")[..7];
items = vec![_("voltage"), v, _("rate"), rate, _("uptime"), uptime];
} else {
if self.page == 2 {
spacing = 0.11;
let ct = self.round_last_val("servo.controller_temp", 2);
let mt = self.round_last_val("servo.motor_temp", 2);
items = vec![_("cont temp"), ct, _("motor temp"), mt];
if self.lcd.battery_voltage {
items += vec![_("battery"), ("%.3f" % self.lcd.battery_voltage)];
} else {
items += vec![_("faults"), self.round_last_val("servo.faults", 0)];
}
} else {
spacing = 0.18;
let ver = self.last_val("ap.version");
items = vec![_("version"), ver, _("author"), "Sean D'Epagnier"];
}
}
}
let (even, odd) = (0, 0.05);
for item in items {
self.fittext(rectangle(0, y, 1, (spacing + even)), item, false);
y += (spacing + even);
let (even, odd) = (odd, even);
}
}
fn process<RT>(&self) -> RT {
if self.testkeydown(SMALL_PORT)||self.testkeydown(BIG_STARBOARD) {
self.page += 1;
}
if self.testkeydown(SMALL_STARBOARD)||self.testkeydown(BIG_PORT) {
self.page -= 1;
}
return super(info, self).process();
} 
}
struct calibrate_info {

}

impl calibrate_info {
fn __init__(&self)  {
super(calibrate_info, self).__init__(3);
}
fn display<T0>(&self, refresh: T0)  {
self.bound_page();
self.fill(black);
self.fittext(rectangle(0, 0, 1, 0.24), _("Calibrate Info"), true);
if self.page == 0 {
let mut deviation = vec!["N/A", "N/A"];
let mut deviationstr = "N/A";
let mut dim = "?";
let try_dummy = { //unsupported
let mut cal = self.last_val("imu.compass.calibration");
deviation = vec![("%.2f" % cal[1][0]), ("%.2f" % cal[1][1])];
dim = String::from(i32::from(cal[2]));
let names = vec![(0, _("incomplete")), (0.01, _("excellent")), (0.02, _("good")), (0.04, _("fair")), (0.06, _("poor")), (1000, _("bad"))];
for n in names {
if cal[1][0] <= n[0] {
deviationstr = n[1];
break;
}
}
};
let except!() = { //unsupported
/*pass*/
};
self.fittext(rectangle(0, 0.3, 1, 0.15), _("compass"));
self.fittext(rectangle(0, 0.42, 1, 0.23), deviationstr);
self.fittext(rectangle(0, 0.66, 1, 0.14), (((deviation[0] + " ") + dim) + "d"));
self.fittext(rectangle(0, 0.8, 1, 0.2), self.last_val("imu.compass.calibration.age")[..7]);
} else {
if self.page == 1 {
let try_dummy = { //unsupported
cal = self.last_val("imu.compass.calibration");
let mut raw = "";
for c in cal[0] {
raw += ("%.1f
" % c);
}
};
let except!() = { //unsupported
raw = "N/A";
};
self.fittext(rectangle(0, 0.3, 1, 0.7), raw);
} else {
use math;
let mod = (i32::from((time.time() % 11))/3);
self.fittext(rectangle(0, 0.24, 1, 0.15), "sigma plot");
cal = self.last_val("imu.compass.calibration")[0];
if cal.len() >= 5 {
let mut m = cal[3];
let dip = math.radians(cal[4]);
} else {
let (m, dip) = (0, 0);
}
if mod == 1 {
m *= math.cos(dip);
}
let try_dummy = { //unsupported
use pypilot::{quaternion};
let mut p = self.last_val("imu.compass.calibration.sigmapoints");
let mut q = self.last_val("imu.alignmentQ");
p = p.iter().map(|p0| p0[..3].iter().map(|x0, c| ((x0 - c)/m)));
let (x, y, r) = (24, 56, 20);
if mod > 1 {
if mod == 3 {
let (x1, y1) = (i32::from((r*math.cos(dip))), i32::from((r*math.sin(dip))));
self.surface.line(x, y, (x + x1), (y + y1), white);
self.surface.line(x, y, (x - x1), (y + y1), white);
}
q = quaternion.multiply(q, quaternion.angvec2quat(math.radians(90), vec![0, 1, 0]));
}
p = p.iter().map(|p0| quaternion.rotvecquat(p0, q));
for p0 in p {
self.surface.putpixel(i32::from(((r*p0[0]) + x)), i32::from(((r*p0[1]) + y)), white);
}
};
let except!() = { //unsupported
self.fittext(rectangle(0, 0.3, 1, 0.7), "N/A");
};
}
}
} 
}
struct controlbase {
lcd: ST0,
batt: bool,
wifi: bool,
pilot: bool,
charging_blink: bool,
charging_blink_time: ST1,
}

impl controlbase {
fn __init__<T0, T1>(&self, lcd: T0, frameperiod: T1)  {
super(controlbase, self).__init__(frameperiod);
self.lcd = lcd;
self.batt = false;
self.wifi = false;
self.pilot = false;
self.charging_blink = false;
self.charging_blink_time = 0;
}
fn display<T0>(&self, refresh: T0)  {
if refresh {
self.box(rectangle(0, 0.92, 1, 0.1), black);
self.wifi = false;
self.pilot = false;
}
if self.lcd.battery_voltage {
let mut battrect = rectangle(0.03, 0.93, 0.25, 0.06);
let batt = (self.lcd.battery_voltage - 3.2).iter().max().unwrap().iter().min().unwrap();
if batt != self.batt||refresh {
self.batt = batt;
self.lcd.surface.box(starred!((self.convrect(battrect) + vec![black]))/*unsupported*/);
self.rectangle(battrect, 0.015);
self.rectangle(rectangle(0.28, 0.95, 0.03, 0.02));
if batt&&!self.charging_blink {
battrect = rectangle(0.06, 0.95, (0.19*float(batt)), 0.02);
self.box(battrect, white);
}
}
if self.lcd.battery_voltage > 4.5 {
let t = gettime();
if (t - self.charging_blink_time) > 1 {
self.charging_blink_time = t;
self.charging_blink = !self.charging_blink;
self.batt = 0;
}
} else {
self.charging_blink = false;
}
}
let pilot = self.last_val("ap.pilot");
if self.pilot != pilot {
self.pilot = pilot;
let pilotrect = rectangle(0, 0.92, 0.6, 0.09);
self.fittext(pilotrect, pilot[..6]);
}
let wifi = test_wifi();
if self.wifi == wifi&&!refresh {
return;
}
self.wifi = wifi;
let wifirect = rectangle(0.65, 0.92, 0.3, 0.09);
if wifi {
let mut text = "W";
if self.lcd.host != "localhost" {
text += "R";
}
self.fittext(wifirect, text);
} else {
self.lcd.surface.box(starred!((self.convrect(wifirect) + vec![black]))/*unsupported*/);
}
} 
}
struct control {
modes_list: ST0,
control: HashMap<_,_>,
lastspeed: ST1,
lasttime: ST2,
ap_heading_command_time: ST3,
ap_heading_command: ST4,
tack_hint: ST5,
manualkeystate: ST6,
}

impl control {
fn __init__<T0>(&self, lcd: T0)  {
super(control, self).__init__(lcd, 0.25);
self.modes_list = vec!["compass", "gps", "wind", "true wind"];
self.control = HashMap::new();
self.lastspeed = 0;
self.lasttime = 0;
self.ap_heading_command_time = (gettime() - 5);
self.ap_heading_command = 0;
self.resetmanualkeystate();
self.tack_hint = (0, "");
}
fn get_ap_heading_command<RT>(&self) -> RT {
if (gettime() - self.ap_heading_command_time) < 5 {
return self.ap_heading_command;
}
return self.last_val("ap.heading_command");
}
fn set_ap_heading_command<T0>(&self, command: T0)  {
if self.control["mode"].iter().any(|&x| x == "wind") {
d = -180;
} else {
d = 0;
}
while command < d {
command += 360;
}
while command >= (d + 360) {
command -= 360;
}
self.set("ap.heading_command", command);
self.ap_heading_command = command;
self.ap_heading_command_time = gettime();
}
fn resetmanualkeystate<T0>(&self, k: T0)  {
self.manualkeystate = [("key", k), ("command", self.get_ap_heading_command()), ("change", 0)].iter().cloned().collect::<HashMap<_,_>>();
}
fn have_compass<RT>(&self) -> RT {
return true;
}
fn have_gps<RT>(&self) -> RT {
return self.last_val("gps.source") != "none";
}
fn have_wind<RT>(&self) -> RT {
return self.last_val("wind.source") != "none";
}
fn have_true_wind<RT>(&self) -> RT {
return self.last_val("truewind.source") != "none";
}
fn display_mode(&self)  {
let mode = self.last_val("ap.mode");
let mut modes = vec![self.have_compass(), self.have_gps(), self.have_wind(), self.have_true_wind()];
if self.control["mode"] == mode&&self.control["modes"] == modes {
return;
}
self.control["mode"] = mode;
self.control["modes"] = modes;
modes = [("compass", ("C", self.have_compass, rectangle(0, 0.74, 0.22, 0.16))), ("gps", ("G", self.have_gps, rectangle(0.22, 0.74, 0.25, 0.16))), ("wind", ("W", self.have_wind, rectangle(0.47, 0.74, 0.3, 0.16))), ("true wind", ("T", self.have_true_wind, rectangle(0.77, 0.74, 0.23, 0.16)))].iter().cloned().collect::<HashMap<_,_>>();
let marg = 0.02;
self.lcd.surface.box(starred!((self.convrect(rectangle(0, 0.74, 1, (0.16 + marg))) + vec![black]))/*unsupported*/);
for mode in modes {
if modes[mode][1]() {
let ret = self.fittext(modes[mode][2], modes[mode][0]);
}
}
for mode in modes {
if self.last_val("ap.mode") == mode {
let r = modes[mode][2];
self.rectangle(rectangle(r.x, (r.y + marg), r.width, r.height), 0.015);
}
}
}
fn display<T0, RT>(&self, refresh: T0) -> RT {
if !self.control {
self.control = [("heading", false), ("heading_command", false), ("mode", false), ("modes", vec![])].iter().cloned().collect::<HashMap<_,_>>();
}
fn nr<T0, RT>(x: T0) -> RT {
let try_dummy = { //unsupported
let mut s = String::from(i32::from(round(abs(x))));
while s.len() < 3 {
s = (" " + s);
}
return s;
};
let except!() = { //unsupported
return x;
};
}
fn draw_big_number<T0, T1, T2>(pos: T0, num: T1, lastnum: T2)  {
if num == "N/A"&&lastnum != num {
let r = rectangle(0, pos, 1, 0.4);
self.fittext(r, num, false, black);
return;
}
if self.lcd.surface.width < 120 {
size = 34;
} else {
size = 32;
}
for i in (0..3) {
let try_dummy = { //unsupported
if num[i] == lastnum[i] {
continue;
}
};
let except!() = { //unsupported
/*pass*/
};
let x = (float(i)*0.33);
self.box(rectangle(x, pos, 0.34, 0.4), black);
self.text((x, pos), num[i], size, true);
}
}
fn draw_heading<T0, T1, T2>(pos: T0, value: T1, lastvalue: T2)  {
let (heading, mode, num) = value;
let try_dummy = { //unsupported
let (lastheading, lastmode, lastnum) = lastvalue;
};
let except!() = { //unsupported
let lastmode = false;
};
let windmode = mode.iter().any(|&x| x == "wind");
if mode != lastmode {
let mut lastnum = "XXX";
} else {
if windmode&&lastheading != "N/A"&&(heading*lastheading) <= 0 {
lastnum = "XXX";
}
}
draw_big_number(pos, num, lastnum);
if windmode {
if heading > 0 {
self.box(rectangle(0.7, (pos + 0.3), 0.3, 0.025), white);
} else {
if heading < 0 {
self.box(rectangle(0, (pos + 0.3), 0.3, 0.025), white);
}
}
}
}
if self.last_val("imu.frequency", 1) == false {
let r = rectangle(0, 0, 1, 0.8);
self.fittext(r, ((_("ERROR") + "
") + _("compass or gyro failure!")), true, black);
self.control["heading"] = "no imu";
self.control["heading_command"] = "no imu";
super(control, self).display(refresh);
return;
}
let t0 = gettime();
let mode = self.last_val("ap.mode");
let ap_heading = self.last_val("ap.heading");
let ap_heading_command = self.get_ap_heading_command();
let heading = (ap_heading, mode, nr(ap_heading));
if self.control["heading"]&&heading == self.control["heading"]&&self.control["heading_command"] == ap_heading_command {
if (t0 - self.lasttime) < 0.8&&!refresh {
return true;
}
}
self.lasttime = t0;
draw_heading(0, heading, self.control["heading"]);
self.control["heading"] = heading;
let flags = self.last_val("servo.flags").split();
let mut warning = "";
let buzz = false;
for flag in flags {
if flag.endswith("_FAULT") {
warning += (flag[..-6].replace("_", " ") + " ");
if flag.iter().any(|&x| x == "OVER") {
buzz = true;
}
}
}
if warning {
if buzz&&self.lcd.config["buzzer"] > 0 {
self.lcd.send("buzzer", (1, 0.1));
}
warning = warning.lower();
warning += "fault";
if self.control["heading_command"] != warning {
self.fittext(rectangle(0, 0.4, 1, 0.4), _(warning), true, black);
self.control["heading_command"] = warning;
self.control["mode"] = warning;
}
} else {
if mode == "gps"&&!self.have_gps() {
if self.control["heading_command"] != "no gps" {
self.fittext(rectangle(0, 0.4, 1, 0.35), _("GPS not detected"), true, black);
self.control["heading_command"] = "no gps";
}
} else {
if mode == "wind"&&!self.have_wind() {
if self.control["heading_command"] != "no wind" {
self.fittext(rectangle(0, 0.4, 1, 0.35), _("WIND not detected"), true, black);
self.control["heading_command"] = "no wind";
}
} else {
if mode == "true wind"&&!self.have_true_wind() {
if self.control["heading_command"] != "no wind" {
self.fittext(rectangle(0, 0.4, 1, 0.35), _("WIND not detected"), true, black);
self.control["heading_command"] = "no wind";
}
} else {
if self.last_val("servo.controller") == "none" {
if self.control["heading_command"] != "no controller" {
self.fittext(rectangle(0, 0.4, 1, 0.35), _("WARNING no motor controller"), true, black);
self.control["heading_command"] = "no controller";
}
} else {
if self.lcd.check_voltage() {
let msg = self.lcd.check_voltage();
if self.control["heading_command"] != msg {
self.fittext(rectangle(0, 0.4, 1, 0.34), msg, true, black);
self.control["heading_command"] = msg;
}
} else {
if self.last_val("ap.enabled") != true {
if self.control["heading_command"] != "standby" {
let r = rectangle(0, 0.4, 1, 0.34);
self.fittext(r, _("standby"), false, black);
self.control["heading_command"] = "standby";
}
} else {
if self.last_val("ap.tack.state") != "none" {
let r = rectangle(0, 0.4, 1, 0.34);
let d = self.last_val("ap.tack.direction");
if self.last_val("ap.tack.state") == "waiting" {
msg = ((_("tack") + " ") + String::from(self.last_val("ap.tack.timeout")));
} else {
msg = ((_("tacking") + " ") + d[0].upper());
}
if self.control["heading_command"] != msg {
self.fittext(r, msg, false, black);
self.control["heading_command"] = msg;
}
} else {
if self.control["heading_command"] != ap_heading_command {
let heading_command = (ap_heading_command, mode, nr(ap_heading_command));
draw_heading(0.4, heading_command, self.control["heading_command"]);
self.control["heading_command"] = heading_command;
self.control["mode"] = false;
}
}
}
}
}
}
}
}
}
if mode == "compass" {
let cal = self.last_val("imu.compass.calibration");
if cal == "N/A" {
ndeviation = 0;
} else {
ndeviation = cal[1][0];
}
fn warncal<T0>(s: T0)  {
let r = rectangle(0, 0.75, 1, 0.15);
self.fittext(r, s, true, white);
self.invertrectangle(r);
self.control["mode"] = "warning";
}
if ndeviation == 0&&false {
warncal(_("No Cal"));
warning = true;
}
if ndeviation > 6 {
warncal(_("Bad Cal"));
warning = true;
}
}
if !warning {
self.display_mode();
}
super(control, self).display(refresh);
}
fn process<RT>(&self) -> RT {
if !self.lcd.client.connection {
return connecting(self.lcd);
}
if self.testkeydown(AUTO) {
self.lcd.reset_keys();
if self.last_val("ap.enabled") == false {
self.set_ap_heading_command(self.last_val("ap.heading"));
self.set("ap.enabled", true);
} else {
self.set("servo.command", 0);
self.set("ap.enabled", false);
}
}
if self.testkeydown(SELECT) {
let have_mode = [("compass", self.have_compass), ("gps", self.have_gps), ("wind", self.have_wind), ("true wind", self.have_true_wind)].iter().cloned().collect::<HashMap<_,_>>();
for t in (0..self.modes_list.len()) {
self.modes_list = (self.modes_list[1..] + vec![self.modes_list[0]]);
let next_mode = self.modes_list[0];
if next_mode != self.last_val("ap.mode")&&have_mode[next_mode]() {
self.set("ap.mode", next_mode);
break;
}
}
return;
}
if self.testkeydown(TACK) {
if self.last_val("ap.tack.state") == "none" {
let (t, direction) = self.tack_hint;
if (time.monotonic() - t) < 3 {
self.set("ap.tack.direction", if direction > 0 { "starboard" } else { "port" });
}
self.set("ap.tack.state", "begin");
} else {
self.set("ap.tack.state", "none");
}
return;
}
if self.last_val("ap.enabled") {
let keys = [(SMALL_STARBOARD, (0, 1)), (SMALL_PORT, (0, -1)), (BIG_PORT, (1, -1)), (BIG_STARBOARD, (1, 1))].iter().cloned().collect::<HashMap<_,_>>();
let mut key = None;
let mut dt = 0;
for k in keys {
if self.testkeydown(k) {
self.resetmanualkeystate(k);
key = k;
dt = 0.1;
break;
}
}
if !dt {
self.resetmanualkeystate(0);
} else {
let mut speed = keys[key][0];
if speed {
change = self.lcd.config["bigstep"];
} else {
change = self.lcd.config["smallstep"];
}
if !speed {
if dt > 1 {
let mut change = (self.lcd.config["bigstep"]*i32::from(dt));
}
}
if self.manualkeystate["change"] != change {
self.manualkeystate["change"] = change;
let mut sign = keys[key][1];
if self.control["mode"].iter().any(|&x| x == "wind") {
sign = -(sign);
}
let mut change = float(change);
let cmd = (self.manualkeystate["command"] + (sign*change));
self.tack_hint = (time.monotonic(), sign);
self.set_ap_heading_command(cmd);
}
}
} else {
let mut speed = self.speed_of_keys();
if speed {
self.set("servo.command", speed);
} else {
if self.lastspeed {
self.set("servo.command", 0);
}
}
self.lastspeed = speed;
}
return super(control, self).process();
} 
}
struct connecting {
connecting_dots: ST0,
drawn_text: bool,
}

impl connecting {
fn __init__<T0>(&self, lcd: T0)  {
super(connecting, self).__init__(lcd);
self.connecting_dots = 0;
}
fn display<T0>(&self, refresh: T0)  {
if refresh {
self.box(rectangle(0, 0, 1, 0.4), black);
self.fittext(rectangle(0, 0, 1, 0.4), _("connect to server"), true);
self.drawn_text = true;
}
self.box(rectangle(0, 0.4, 1, 0.52), black);
let mut dots = "";
for i in (0..self.connecting_dots) {
dots += ".";
}
let size = self.text((0, 0.4), dots, 12);
self.connecting_dots += 1;
if size[0] >= 1||self.connecting_dots > 20 {
self.connecting_dots = 0;
}
super(connecting, self).display(refresh);
}
fn process<RT>(&self) -> RT {
if self.lcd.client.connection {
return control(self.lcd);
}
if self.testkeydown(MENU) {
return self.lcd.getmenu();
}
} 
}