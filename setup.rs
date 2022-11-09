use std::*;
use std::collections::HashMap;

if sys.version_info[0] < 3 {
println ! ("{:?} {:?} ", "pypilot requires python version 3.  python version is", sys.version);
exit(1);
}
if os.system("which apt") {
println ! ("{:?} ", "system does not support apt, you can try running dependencies script and/or manually install needed packages");
} else {
if ! os.path.exists("deps") {
use dependencies;
}
}
let try_dummy = { //unsupported
use setuptools::{setup, Extension};
};
let except!(ImportError) = { //unsupported
println ! ("{:?} ", "ERROR, requires python setuptools package!");
exit(0);
};
use setuptools::command::install::{install};

struct build_ext_first {}

impl build_ext_first {
    fn run<RT>(&self) -> RT {
        self.run_command("build_ext");
        return install::run(self);
    }
}

const linebuffer_module: _ = Extension("pypilot/linebuffer/_linebuffer", vec!["pypilot/linebuffer/linebuffer.cpp", "pypilot/linebuffer/linebuffer.i"], vec!["-Wno-unused-result"], vec!["-c++"]);
const arduino_servo_module: _ = Extension("pypilot/arduino_servo/_arduino_servo", vec!["pypilot/arduino_servo/arduino_servo.cpp", "pypilot/arduino_servo/arduino_servo_eeprom.cpp", "pypilot/arduino_servo/arduino_servo.i"], vec!["-Wno-unused-result"], vec!["-c++"]);
let ugfx_defs = vec!["-DWIRINGPI"];
let try_dummy = { //unsupported
let ugfx_libraries = vec ! ["wiringPi"];
};
let except!() = { //unsupported
let try_dummy = { //unsupported
ugfx_libraries = vec ! ["wiringPi"];
};
let except ! () = { //unsupported
println ! ("{:?} ", "no RPi.GPIO library for ugfx");
ugfx_libraries = vec ! [];
ugfx_defs = vec ! [];
};
};
const ugfx_module: _ = Extension("pypilot/hat/ugfx/_ugfx", vec!["hat/ugfx/ugfx.cpp", "hat/ugfx/ugfx.i"], (vec!["-Wno-unused-result"] + ugfx_defs), ugfx_libraries, (vec!["-c++"] + ugfx_defs));
if ugfx_libraries {
spireader_module = Extension("pypilot/hat/spireader/_spireader", vec!["hat/spireader/spireader.cpp", "hat/spireader/spireader.i"], vec!["-Wno-unused-result"], ugfx_libraries, vec!["-c++"]);
} else {
spireader_module = None;
}
os.system("cd hat/locale;./translate.sh");
os.system("cd hat; pybabel compile -d translations");
os.system("cd pypilot/locale;./translate.sh");
os.system("cd web; pybabel compile -d translations");
fn find_locales<T0, T1, RT>(name: T0, dir: T1) -> RT {
    let mut locale_files = vec![];
    for walk in os.walk(((("./" + name) + "/") + dir)) {
        let (path, dirs, files) = walk;
        let path = path[(name.len() + 3)..];
        for file in files {
            if file[-3..] == ".mo" {
                locale_files.push(os.path.join(path, file));
            }
        }
    }
    return locale_files;
}

use pypilot::{version};
let packages = vec!["pypilot", "pypilot/pilots", "pypilot/arduino_servo", "ui", "hat", "web", "pypilot/linebuffer", "hat/ugfx", "hat/spireader"];
let try_dummy = { //unsupported
use setuptools::{find_packages};
packages = find_packages();
};
let except!() = { //unsupported
/*pass*/
};
const package_dirs: _ = HashMap::new();
for package in packages.collect::<Vec<_ > > () {
if ! package.startswith("pypilot") {
packages.remove(package);
packages.push(("pypilot." + package));
package_dirs[("pypilot." + package)] = package.replace(".", "/");
}
}
const package_data: _ = [("pypilot", find_locales("pypilot")), ("pypilot.hat", ((vec!["font.ttf", "static/*", "templates/*"] + find_locales("hat")) + find_locales("hat", "translations"))), ("pypilot.ui", vec!["*.png", "*.mtl", "*.obj"]), ("pypilot.web", ((vec!["static/*", "templates/*"] + vec!["pypilot_web.pot"]) + find_locales("web", "translations")))].iter().cloned().collect::<HashMap<_, _>>();
let ext_modules = vec![arduino_servo_module, linebuffer_module, ugfx_module];
if spireader_module {
ext_modules.push(spireader_module);
}
setup("pypilot", version::strversion, "pypilot sailboat autopilot", "GPLv3", "Sean D'Epagnier", "http://pypilot.org/", packages, package_dirs, ext_modules, package_data, [("install", build_ext_first)].iter().cloned().collect::<HashMap<_, _ > > (), [("console_scripts", vec!["pypilot=pypilot.autopilot:main", "pypilot_boatimu=pypilot.boatimu:main", "pypilot_servo=pypilot.servo:main", "pypilot_web=pypilot.web.web:main", "pypilot_hat=pypilot.hat.hat:main", "pypilot_control=pypilot.ui.autopilot_control:main", "pypilot_calibration=pypilot.ui.autopilot_calibration:main", "pypilot_client=pypilot.client:main", "pypilot_scope=pypilot.ui.scope_wx:main", "pypilot_client_wx=pypilot.ui.client_wx:main"])].iter().cloned().collect::<HashMap<_, _ > > ());