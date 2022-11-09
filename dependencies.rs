use std::*;
use std::collections::HashMap;

const success: _ = true;

struct dep {
    name: ST0,
}

impl dep {
    fn __init__<T0>(&self, name: T0) {
        self.name = name;
    }
}

struct py_dep {
    pip_only: ST0,
}

impl py_dep {
    fn __init__<T0, T1>(&self, name: T0, pip_only: T1) {
        self.pip_only = pip_only;
        super(py_dep, self).__init__(name);
    }
    fn test<RT>(&self) -> RT {
        let remap = [("pil", "PIL"), ("gevent-websocket", "geventwebsocket"), ("flask-socketio", "flask_socketio"), ("flask-babel", "flask_babel"), ("python-socketio", "socketio"), ("opengl", "OpenGL")].iter().cloned().collect::<HashMap<_, _>>();
        let name = if remap.iter().any(|&x| x == self.name) { remap[self.name] } else { self.name };
        let try_dummy = { //unsupported
            __import__(name);
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ", "failed to import", self.name, " ");
            return false;
        };
        return true;
    }
    fn install<RT>(&self) -> RT {
        let apt_name = ("python3-" + self.name);
        if self.pip_only {
            os.system(("sudo apt remove " + apt_name));
            ret = true;
        } else {
            ret = os.system(("sudo apt install -y " + apt_name));
        }
        if ret {
            println!("{:?} {:?} ", "failed to install via apt, trying with pip", self.name);
            if self.name == "pil" {
                let mut name = "pillow";
            } else {
                if self.name == "flask-socketio" {
                    name = "flask-socketio==5";
                } else {
                    name = self.name;
                }
            }
            let mut ret = os.system(("sudo python3 -m pip install " + name));
            if ret {
                println!("{:?} {:?} ", "failed to install dependency", name);
                return false;
            }
        }
        return true;
    }
}

struct sys_dep {}

impl sys_dep {
    fn __init__<T0>(&self, name: T0) {
        super(sys_dep, self).__init__(name);
    }
    fn test<RT>(&self) -> RT {
        let try_dummy = { //unsupported
            return apt.Cache()[self.name].is_installed;
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ", "Failed to detect package", self.name, i);
        };
        return false;
    }
    fn install<RT>(&self) -> RT {
        let ret = os.system(("sudo apt install -y " + self.name));
        if ret {
            return false;
        }
        return true;
    }
}

struct wiringpi_dep {}

impl wiringpi_dep {
    fn __init__<T0>(&self, name: T0) {
        super(wiringpi_dep, self).__init__(name);
    }
    fn install<RT>(&self) -> RT {
        os.system("git clone https://github.com/wiringPi/wiringPi");
        if os.system("cd wiringPi; ./build") {
            return false;
        }
        return true;
    }
    fn test<T0, RT>(&self, check: T0) -> RT {
        let try_dummy = { //unsupported
            let f = open("/sys/firmware/devicetree/base/model");
            let pi = f.readline().iter().any(|&x| x == "Raspberry Pi");
            f.close();
            if pi {
                println!("{:?} {:?} ", "detected", pi);
                let try_dummy = { //unsupported
                    let output = subprocess.check_output(vec!["gpio", "-v"]);
                    let version = output.split(b"\n")[0].strip(b"gpio version: ");
                    return float(version) >= 2.6;
                };
                let except!(Exception) = { //unsupported
                    println!("{:?} {:?} ", "failed to run gpio command!", e);
                    return false;
                };
                return super(wiringpi_dep, self).test();
            }
        };
        let except!() = { //unsupported
            /*pass*/
        };
        return true;
    }
}

struct RTIMULIB2_dep {}

impl RTIMULIB2_dep {
    fn __init__(&self) {
        super(RTIMULIB2_dep, self).__init__("RTIMULIB2");
    }
    fn test<T0, RT>(&self, check: T0) -> RT {
        let try_dummy = { //unsupported
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ", "failed to import", self.name);
            return false;
        };
        use importlib_metadata::{version};
        let v = version("RTIMULib").split(".");
        let n = ((((i32::from(v[0]) * 1000) + i32::from(v[1])) * 1000) + i32::from(v[2]));
        if n < 8001000 {
            println!("{:?} ", "RTIMULib version out of date");
            return false;
        }
        return true;
    }
    fn install<RT>(&self) -> RT {
        return os.system(". scripts/install_rtimulib.sh") == 0;
    }
}

struct data_dep {}

impl data_dep {
    fn __init__(&self) {
        super(data_dep, self).__init__("data");
    }
    fn test<T0, RT>(&self, check: T0) -> RT {
        return os.path.exists("ui/compass.png");
    }
    fn install<RT>(&self) -> RT {
        return os.system(". scripts/install_data.sh") == 0;
    }
}

struct subsystem {
    name: ST0,
    info: ST1,
    deps: ST2,
    summary: ST3,
}

impl subsystem {
    fn __init__<T0, T1, T2>(&self, name: T0, info: T1, deps: T2) {
        self.name = name;
        self.info = info;
        self.deps = deps;
        self.summary = "SUCCESS";
    }
    fn install(&self) {
        let allok = true;
        for dep in self.deps {
            sys.stdout.write((("checking for " + dep::name) + "... "));
            if dep::test() {
                println!("{:?} ", "done");
            } else {
//global success
                println!("{:?} {:?} ", dep::name, "not found");
                if !dep::install() {
                    println!("{:?} {:?} ", "failed to install", dep::name);
                    self.summary = ("failed to install " + dep::name);
                    success = false;
                } else {
                    if dep::test() {
                        println!("{:?} {:?} {:?} ", "install dependency", dep::name, "success");
                    } else {
                        println!("{:?} {:?} ", "dependency failed to install", dep::name);
                        self.summary = ("failed to detect after installing " + dep::name);
                        success = false;
                    }
                }
            }
        }
    }
    fn result<RT>(&self) -> RT {
        return ((((self.name + " ") + self.info) + ":") + self.summary);
    }
}
let subsystems = vec![];
fn ss() {
    subsystems.push(subsystem(starred!(cargs)/*unsupported*/));
}
ss("dependencies", "dependency script dependencies", vec![py_dep("importlib_metadata")]);
ss("autopilot", "core autopilot or imu-only mode", vec![RTIMULIB2_dep(), py_dep("serial"), py_dep("numpy"), py_dep("scipy"), sys_dep("libpython3-dev"), sys_dep("swig")]);
ss("optimize", "(recommended) core autopilot operations", vec![py_dep("ujson"), py_dep("pyudev")]);
ss("signalk", "communicate with signalk-node-server distributed with openploter", vec![py_dep("zeroconf"), py_dep("requests"), py_dep("websocket")]);
ss("hat", "SPI lcd keypad, and remote control interface", vec![py_dep("pil"), wiringpi_dep("wiringpi")]);
ss("web", "web browser control", vec![py_dep("flask"), py_dep("gevent-websocket"), py_dep("python-socketio"), py_dep("flask-socketio", true), py_dep("flask-babel")]);
ss("python_gui", "python scripts for control and configuration", vec![sys_dep("python3-wxgtk4.0"), py_dep("opengl"), py_dep("pyglet"), py_dep("pywavefront")]);
ss("data", "data files used by various pypilot components", vec![data_dep(), sys_dep("gettext")]);
if os.path.basename(os.path.abspath(os.curdir)) != "pypilot" {
println ! ("{:?} ", "please run this script from the pypilot directory");
exit(1);
}
for s in subsystems {
s.install();
}
println!("{:?} ", "");
println!("{:?} ", "");
println!("{:?} ", "summary of pypilot dependencies");
for s in subsystems {
let r = s.result();
println! ("{:?} ", r);
}
println!("{:?} ", "");
println!("{:?} ", "");
if success {
let f = open("deps", "w");
for s in subsystems {
let r = s.result();
f.write((r + "
"));
}
f.close();
}