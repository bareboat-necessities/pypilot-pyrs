use std::*;
use std::collections::HashMap;

const pypilot_dir: _ = (os.getenv("HOME") + "/.pypilot/");
fn debug()  {
/*pass*/
}
fn read_config<T0, T1, RT>(filename: T0, fail: T1) -> RT {
let mut devices = vec![];
if os.path.exists((pypilot_dir + filename)) {
let try_dummy = { //unsupported
let f = open((pypilot_dir + filename), "r");
while true {
let device = f.readline();
if !device {
break;
}
devices.append(device.strip());
}
f.close();
return devices;
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ",_("error reading"), (pypilot_dir + filename));
};
}
return fail;
}
const blacklist_serial_ports: _ = "init";
fn read_blacklist<RT>() -> RT {
//global blacklist_serial_ports
if blacklist_serial_ports == "init" {
blacklist_serial_ports = read_config("blacklist_serial_ports", vec![]);
}
return blacklist_serial_ports;
}
const allowed_serial_ports: _ = "init";
fn read_allowed<RT>() -> RT {
//global allowed_serial_ports
if allowed_serial_ports == "init" {
allowed_serial_ports = read_config("serial_ports", "any");
}
return allowed_serial_ports;
}
const probes: _ = HashMap::new();
fn new_probe<T0>(name: T0)  {
//global probes
probes[name] = [("time", 0), ("device", false), ("probelast", true), ("lastdevice", false), ("lastworking", false)].iter().cloned().collect::<HashMap<_,_>>();
}
fn read_last_working_devices()  {
//global probes
for filename in os.listdir(pypilot_dir) {
if filename.endswith("device") {
let name = filename[..-6];
if name {
let try_dummy = { //unsupported
let file = open((pypilot_dir + filename), "r");
let lastdevice = pyjson.loads(file.readline().rstrip());
file.close();
if !probes.iter().any(|&x| x == name) {
new_probe(name);
}
probes[name]["lastworking"] = (lastdevice[0], lastdevice[1]);
};
let except!() = { //unsupported
/*pass*/
};
}
}
}
}
fn scan_devices<RT>() -> RT {
let devices = HashMap::new();
let mut devicesp = vec!["ttyAMA"];
let by_id = "/dev/serial/by-id";
let by_path = "/dev/serial/by-path";
let mut by = by_id;
if os.path.exists(by_id) {
let mut paths = os.listdir(by_id);
debug("serialprobe scan by-id", paths);
if os.path.exists(by_path) {
let by_path_paths = os.listdir(by_path);
if by_path_paths.len() > paths.len() {
println!("{:?} ",("serialprobe " + _("found more devices by path")));
paths = by_path_paths;
by = by_path;
}
}
for device_path in paths {
let full_path = os.path.join(by, device_path);
let mut realpath = os.path.realpath(full_path);
devices[full_path] = [("realpath", realpath)].iter().cloned().collect::<HashMap<_,_>>();
}
} else {
devicesp = (vec!["ttyUSB", "ttyACM"] + devicesp);
}
let mut devgpsdevices = vec![];
for dev in os.listdir("/dev") {
let devicesd = vec![];
if dev.startswith("gps") {
let mut path = ("/dev/" + dev);
let mut realpath = os.path.realpath(path);
devgpsdevices.push(realpath);
}
for p in devicesp {
if dev.startswith(p) {
let mut path = ("/dev/" + dev);
let mut realpath = os.path.realpath(path);
for device in devices {
if device[1] == realpath {
break;
}
}
}
}
}
for device in devices.collect::<Vec<_>>() {
if devgpsdevices.iter().any(|&x| x == devices[device]["realpath"]) {
println!("{:?} {:?} ",("serialprobe " + _("removing gps device")), device);
devices[device].drop();
}
}
blacklist_serial_ports = read_blacklist();
for path in blacklist_serial_ports {
let mut realpath = os.path.realpath(path);
for device in devices.collect::<Vec<_>>() {
if devices[device]["realpath"] == realpath {
devices[device].drop();
}
}
}
allowed_serial_ports = read_allowed();
if allowed_serial_ports == "any" {
return devices;
}
let allowed_devices = HashMap::new();
for path in allowed_serial_ports {
for device in devices {
let mut realpath = devices[device]["realpath"];
if os.path.realpath(path) == realpath {
allowed_devices[device] = devices[device];
}
}
}
for path in allowed_serial_ports {
let mut realpath = os.path.realpath(path);
for device in allowed_devices {
if devices[device]["realpath"] == realpath {
break;
}
}
}
return allowed_devices;
}
const devices: _ = HashMap::new();
let gpsdevices = vec![];
const enumstate: _ = "init";
fn enumerate_devices<RT>() -> RT {
//global devices
//global enumstate
let t0 = time.monotonic();
if enumstate == "init" {
enumstate = [("monitor", false), ("starttime", t0), ("scantime", 0), ("retries", 0), ("pyudevwarning", false)].iter().cloned().collect::<HashMap<_,_>>();
devices = HashMap::new();
read_last_working_devices();
}
if enumstate["monitor"] {
let mut ret = enumstate["monitor"].poll(0);
if ret {
enumstate["scantime"] = t0;
enumstate["retries"] = 5;
while ret {
debug("serialprobe pyudev monitor", ret);
ret = enumstate["monitor"].poll(0);
}
}
if enumstate["retries"] == 0||t0 < enumstate["scantime"] {
return false;
}
} else {
if t0 > (enumstate["starttime"] + 5) {
use signal;
let cursigchld_handler = signal.getsignal(signal.SIGCHLD);
signal.signal(signal.SIGCHLD, signal.SIG_IGN);
let try_dummy = { //unsupported
let context = pyudev.Context();
enumstate["monitor"] = pyudev.Monitor.from_netlink(context);
enumstate["monitor"].filter_by("usb");
};
let except!(Exception) = { //unsupported
enumstate["starttime"] = (time.monotonic() + 10);
if !enumstate["pyudevwarning"] {
println!("{:?} {:?} ",_("no pyudev module! will scan usb devices often!"), e);
enumstate["pyudevwarning"] = true;
}
};
signal.signal(signal.SIGCHLD, cursigchld_handler);
}
if t0 < enumstate["scantime"] {
return false;
}
enumstate["scantime"] = (t0 + 20);
}
let scanned_devices = scan_devices();
if enumstate["monitor"] {
let prev_devices = HashMap::new();
for name in devices {
prev_devices[name] = [("realpath", devices[name]["realpath"])].iter().cloned().collect::<HashMap<_,_>>();
}
if prev_devices == scanned_devices {
if enumstate["retries"] > 0 {
enumstate["scantime"] += 2;
enumstate["retries"] -= 1;
}
return false;
} else {
if enumstate["monitor"] {
debug("serialprobe pyudev found it", devices, scanned_devices);
enumstate["retries"] = 0;
}
}
}
debug("serialprobe scan", scanned_devices);
for device in devices.collect::<Vec<_>>() {
if !scanned_devices.iter().any(|&x| x == device) {
devices[device].drop();
}
}
for device in scanned_devices {
if !devices.iter().any(|&x| x == device) {
devices[device] = scanned_devices[device];
devices[device]["time"] = t0;
}
}
return true;
}
fn relinquish<T0>(name: T0)  {
if probes.iter().any(|&x| x == name) {
probes[name]["device"] = false;
}
}
fn probe<T0, T1, T2, RT>(name: T0, bauds: T1, timeout: T2) -> RT {
//global devices
//global probes
let t0 = time.monotonic();
if enumerate_devices() {
for (n, probe) in probes.items() {
let mut device = probe["device"];
if device&&!devices.iter().any(|&x| x == device) {
probe["device"] = false;
}
}
}
if !probes.iter().any(|&x| x == name) {
new_probe(name);
}
let probe = probes[name];
if probe["device"] {
probe["bauds"] = probe["bauds"][1..];
if probe["bauds"] {
return (probe["device"], probe["bauds"][0]);
}
probe["device"] = false;
}
if (t0 - probe["time"]) < timeout {
probe["device"] = false;
return false;
}
debug("serialprobe PROBE", name, probe);
let device_list = devices.collect::<Vec<_>>();
if probe["probelast"] {
probe["time"] = t0;
probe["probelast"] = false;
if !probe["lastworking"] {
debug("serialprobe no last working, abort", name);
return false;
}
let (last_device, last_baud) = probe["lastworking"];
bauds = vec![last_baud];
for index in (0..device_list.len()) {
let mut device = device_list[index];
if device == last_device {
break;
}
}
} else {
probe["probelast"] = true;
for index in (0..device_list.len()) {
let mut device = device_list[index];
if device == probe["lastdevice"] {
index += 1;
break;
}
}
}
while index < device_list.len() {
let mut device = device_list[index];
for p in probes {
let probe_device = probes[p]["device"];
if probe_device == device {
if !probe["probelast"] {
debug("serialprobe ret1");
return false;
}
index += 1;
break;
}
}
}
if probe["probelast"] {
probe["lastdevice"] = device;
}
if device == "/dev/ttyAMA0"&&name != "servo" {
debug("serial probe abort", name, "reserved for servo");
return false;
}
if gpsdevices.iter().any(|&x| x == devices[device]["realpath"]) {
debug("serial probe abort", name, "device", device, "is a gps device");
return false;
}
probe["device"] = device;
probe["bauds"] = bauds;
debug("serial probing", name, device, bauds[0]);
return (device, bauds[0]);
}
fn gpsddevices<T0>(devices: T0)  {
//global gpsdevices
gpsdevices = vec![];
for device in devices {
let realpath = os.path.realpath(device);
gpsdevices.push(realpath);
}
}
fn success<T0, T1>(name: T0, device: T1)  {
//global probes
let filename = ((pypilot_dir + name) + "device");
println!("{:?} {:?} {:?} ",(("serialprobe " + _("success")) + ":"), filename, device);
probes[name]["lastworking"] = device;
let try_dummy = { //unsupported
let file = open(filename, "w");
file.write((pyjson.dumps(device) + "
"));
file.close();
};
let except!() = { //unsupported
println!("{:?} {:?} ",("serialprobe " + _("failed to record device")), name);
};
}
fn main() {
println!("{:?} ","testing serial probe");
while true {
let t0 = time.monotonic();
let device = probe("test", vec![9600], 2);
if device {
println!("{:?} {:?} {:?} ","return", device, (time.monotonic() - t0));
}
time.sleep(1);
}
}