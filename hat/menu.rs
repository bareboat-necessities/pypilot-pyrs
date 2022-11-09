use std::*;
use std::collections::HashMap;

use page::{*};
use page::{_};
let try_dummy = { //unsupported
};
let except!() = { //unsupported
const micropython: _ = false;
};
struct menu {
    selection: ST0,
    items: ST1,
    prev: bool,
    last_selection: ST2,
    menu_values: HashMap<_, _>,
}

impl menu {
    fn __init__<T0, T1>(&self, name: T0, items: T1) {
        super(menu, self).__init__(name);
        self.selection = 0;
        self.items = items;
        self.prev = false;
        self.last_selection = -1;
        self.menu_values = HashMap::new();
    }
    fn find_parents(&self) {
        for p in self.items {
            p.lcd = self.lcd;
            p.prev = self;
            if isinstance(p, menu) {
                p.find_parents();
            }
        }
    }
    fn mainmenu<RT>(&self) -> RT {
        if self.prev {
            return self.prev.mainmenu();
        }
        return self;
    }
    fn display<T0>(&self, refresh: T0) {
        self.lcd.menu = self;
        if !refresh && self.selection == self.last_selection {
            for item in self.items {
                if isinstance(item, ValueCheck) || isinstance(item, RangeEdit) {
                    let path = item.pypilot_path;
                    let mut val = self.last_val(path);
                    if !refresh {
                        refresh = !self.menu_values.iter().any(|&x| x == path) || self.menu_values[path] != val;
                    }
                    self.menu_values[path] = val;
                }
            }
            if !refresh {
                return;
            }
        }
        self.last_selection = self.selection;
        self.fill(black);
        let fit = self.fittext(rectangle(0, 0, 1, 0.25), self.name);
        let sy = (fit[1] + 0.03);
        let items = i32::from(((1 - y) / 0.15)).iter().min().unwrap();
        let mut scroll = (self.selection - i32::from((items / 2))).iter().max().unwrap();
        scroll = scroll.iter().min().unwrap();
        let mut maxsizeslider = 0;
        let mut sliders = vec![];
        for item in self.items[scroll..] {
            let size = self.fittext(rectangle(0, y, 1, 0.15), item.name);
            if isinstance(item, ValueCheck) {
                let mut val = self.last_val(item.pypilot_path);
                if val == true {
                    self.invertrectangle(rectangle(0.8, (y + 0.07), 0.1, 0.07));
                }
            } else {
                if isinstance(item, RangeEdit) && size[0] < 0.8 {
                    maxsizeslider = (size[0] + 0.02).iter().max().unwrap();
                    sliders.push((item, y));
                }
            }
            y += 0.15;
            if y >= 0.9 {
                break;
            }
        }
        for (item, y) in sliders {
            let sliderarea = rectangle(maxsizeslider, (y + 0.05), (1 - maxsizeslider), 0.07);
            self.rectangle(sliderarea, 0.015);
            let try_dummy = { //unsupported
                let values = self.lcd.client.get_values();
                let name = item.pypilot_path;
                let minv = values[name]["min"];
                let maxv = values[name]["max"];
                let mut val = ((self.last_val(name, 0, 0) - minv) / (maxv - minv));
                if val <= 0 {
                    continue;
                }
                sliderarea.width *= val;
                self.rectangle(sliderarea);
            };
            let except!(Exception) = { //unsupported
                /*pass*/
            };
        }
        if self.selection >= 0 {
            y = ((0.15 * (self.selection - scroll)) + sy);
            if y < 0.85 {
                self.invertrectangle(rectangle(0, (y + 0.01), 1, 0.14));
            }
        }
    }
    fn process<RT>(&self) -> RT {
        if self.testkeydown(AUTO) {
            self.lcd.menu = self.lcd.menu.mainmenu();
            return control(self.lcd);
        }
        if self.testkeydown(SMALL_PORT) || self.testkeydown(BIG_PORT) {
            self.selection -= 1;
            if self.selection < 0 {
                self.selection = (self.items.len() - 1);
            }
        } else {
            if self.testkeydown(SMALL_STARBOARD) || self.testkeydown(BIG_STARBOARD) {
                self.selection += 1;
                if self.selection == self.items.len() {
                    self.selection = 0;
                }
            } else {
                if self.testkeydown(MENU) {
                    if self.selection >= 0 && self.selection < self.items.len() {
                        return self.items[self.selection];
                    }
                    return;
                }
            }
        }
        if self.selection >= self.items.len() {
            self.selection = (self.items.len() - 1);
        }
        return super(menu, self).process();
    }
}

struct RangeEdit {
    name: ST0,
    desc: ST1,
    id: ST2,
    pypilot_path: ST3,
    range: ST4,
    lastmovetime: ST5,
    value: Option<_>,
}

impl RangeEdit {
    fn __init__<T0, T1, T2, T3, T4, T5>(&self, name: T0, desc: T1, id: T2, pypilot_path: T3, minval: T4, maxval: T5) {
        self.name = name;
        if type_(desc) == type_("") || type_(desc) == type_("") {
            self.desc = || desc;
        } else {
            self.desc = desc;
        }
        self.id = id;
        self.pypilot_path = pypilot_path;
        self.range = (minval, maxval);
        self.lastmovetime = 0;
        self.value = None;
        super(RangeEdit, self).__init__(name);
    }
    fn display<T0>(&self, refresh: T0) {
        if refresh {
            self.fill(black);
            self.fittext(rectangle(0, 0, 1, 0.3), self.name, true);
            self.fittext(rectangle(0, 0.3, 1, 0.3), self.desc(), true);
        } else {
            self.
            box (rectangle(0, 0.6, 1, 0.4), black);
        }
        if (gettime() - self.lastmovetime) > 1 {
            if self.pypilot_path {
                self.value = self.last_val(self.pypilot_path);
            }
        }
        if !self.pypilot_path && !self.value {
            self.value = self.lcd.config[self.id];
        }
        if self.value == false {
            return;
        }
        let mut v = self.value;
        if self.pypilot_path {
            let try_dummy = { //unsupported
                v = String::from((round((10000 * v)) / 10000));
                while v.len() < 6 {
                    v += "0";
                }
            };
            let except!() = { //unsupported
                /*pass*/
            };
        } else {
            v = String::from(round(v));
        }
        self.fittext(rectangle(0, 0.6, 1, 0.18), v);
        let sliderarea = rectangle(0, 0.8, 1, 0.1);
        let try_dummy = { //unsupported
            self.rectangle(sliderarea, 0.015);
            let xp = ((float(v) - self.range[0]) / (self.range[1] - self.range[0]));
            sliderarea.width *= xp;
            self.rectangle(sliderarea);
        };
        let except!() = { //unsupported
            /*pass*/
        };
    }
    fn move < T0>( & self , delta: T0)  {
    if self.value == false {
    return;
    }
    let step = (( self.range[1] - self.range[0]) /500.0);
    if self.pypilot_path {
    v = self.value;
    let try_dummy = { //unsupported
    v += (delta * step);
    };
    let except ! () = { //unsupported
    /*pass*/
    };
    } else {
    v = ((delta * step) + self.value);
    }
    let mut v = v.iter().min().unwrap();
    v = v.iter().max().unwrap();
    self.value = v;
    if self.pypilot_path {
    self.lcd.client.set( self.pypilot_path, v);
    } else {
    self.lcd.config[ self.id] = v;
    }
    self.lastmovetime = gettime();
    }
    fn process<RT>(&self) -> RT {
        if self.testkeydown(MENU) {
            if !self.pypilot_path {
                self.lcd.write_config();
            }
            return self.prev;
        }
        let keypad = self.lcd.keypad;
        fn spd<T0, RT>(k: T0) -> RT {
            let dt = (keypad[k].dt() * 2);
            if dt || self.testkeydown(k) {
                return (dt + 1);
            }
            return 0;
        }
        let ss = spd(SMALL_STARBOARD);
        let sp = spd(SMALL_PORT);
        let bp = spd(BIG_PORT);
        let bs = spd(BIG_STARBOARD);
        let mut speed = 0;
        let mut sign = 0;
        if sp || ss {
            speed = sp.iter().max().unwrap();
        }
        if bp || bs {
            speed = (bp.iter().max().unwrap() * 3);
        }
        if ss || bs {
            sign = 1;
        } else {
            if sp || bp {
                sign = -1;
            }
        }
        speed = (sign * speed);
        if speed {
            self. move (speed);
        } else {
            return super(RangeEdit, self).process();
        }
    }
}

fn ConfigEdit<T0, T1, T2, T3, T4, RT>(name: T0, desc: T1, config_name: T2, min: T3, max: T4) -> RT {
    return RangeEdit(name, desc, config_name, false, min, max);
}

struct ValueEdit {
    range: bool,
}

impl ValueEdit {
    fn __init__<T0, T1, T2, T3>(&self, name: T0, desc: T1, pypilot_path: T2, value: T3) {
        super(ValueEdit, self).__init__(name, desc, false, pypilot_path, 0, 1);
        self.range = false;
    }
    fn display<T0>(&self, refresh: T0) {
        if !self.range {
            let values = self.lcd.get_values();
            if values.iter().any(|&x| x == self.pypilot_path) {
                info = values[self.pypilot_path];
            } else {
                info = [("min", 0), ("max", 0)].iter().cloned().collect::<HashMap<_, _>>();
            }
            self.range = (info["min"], info["max"]);
        }
        super(ValueEdit, self).display(refresh);
    }
}

struct ValueCheck {
    pypilot_path: ST0,
}

impl ValueCheck {
    fn __init__<T0, T1>(&self, name: T0, pypilot_path: T1) {
        super(ValueCheck, self).__init__(name);
        self.pypilot_path = pypilot_path;
    }
    fn process<RT>(&self) -> RT {
        self.set(self.pypilot_path, !self.last_val(self.pypilot_path));
        return self.lcd.menu;
    }
}

struct ValueEnumSelect {
    lcd: ST0,
    pypilot_path: ST1,
}

impl ValueEnumSelect {
    fn __init__<T0, T1, T2>(&self, lcd: T0, name: T1, pypilot_path: T2) {
        super(ValueEnumSelect, self).__init__(name);
        self.lcd = lcd;
        self.pypilot_path = pypilot_path;
    }
    fn process<RT>(&self) -> RT {
        self.set(self.pypilot_path, self.name);
        return control(self.lcd);
    }
}

struct ValueEnum {
    pypilot_path: ST0,
    selection: ST1,
    items: ST2,
}

impl ValueEnum {
    fn __init__<T0, T1>(&self, name: T0, pypilot_path: T1) {
        super(ValueEnum, self).__init__(name, vec![]);
        self.pypilot_path = pypilot_path;
        self.selection = -1;
    }
    fn process<RT>(&self) -> RT {
        if !self.items {
            let try_dummy = { //unsupported
                let values = self.lcd.client.get_values();
                if values {
                    let info = values[self.pypilot_path];
                    choices = info["choices"];
                } else {
                    choices = vec![];
                }
                self.items = choices.iter().map(|choice| ValueEnumSelect(self.lcd, choice, self.pypilot_path)).collect::<Vec<_>>();
            };
            let except!(Exception) = { //unsupported
                println!("{:?} {:?} ", "failed choices", e);
            };
        }
        if self.selection < 0 {
            let val = self.last_val(self.pypilot_path);
            for i in (0..self.items.len()) {
                if self.items[i].name == val {
                    self.selection = i;
                }
            }
        }
        return super(ValueEnum, self).process();
    }
}

fn GainEdit<T0, RT>(gain: T0) -> RT {
    let n = gain[(gain::rfind(".") + 1)..];
    return ValueEdit(n, n, gain, true);
}

struct gain {
    last_pilot: bool,
    items: ST0,
    selection: ST1,
}

impl gain {
    fn __init__(&self) {
        self.last_pilot = false;
        super(gain, self).__init__(_("gain"), vec![]);
    }
    fn curgains<RT>(&self) -> RT {
        let mut ret = vec![];
        for (name, value) in self.lcd.get_values().items() {
            if value.iter().any(|&x| x == "AutopilotGain") {
                if name.iter().any(|&x| x == "ap.pilot.") {
                    let s = name.split(".");
                    if self.last_pilot == s[2] {
                        ret.push(name);
                    }
                }
            }
        }
        ret.sort();
        ret.reverse();
        return ret;
    }
    fn process<RT>(&self) -> RT {
        let pilot = self.last_val("ap.pilot");
        if pilot != self.last_pilot {
            self.last_pilot = pilot;
            self.items = self.curgains().iter().map(GainEdit).collect::<Vec<_>>();
            self.find_parents();
            self.lcd.need_refresh = true;
            if self.selection < 0 {
                self.selection = 0;
            }
        }
        return super(gain, self).process();
    }
}

struct level {}

impl level {
    fn process<RT>(&self) -> RT {
        self.set("imu.alignmentCounter", 100);
        return self.lcd.menu;
    }
}

struct calibrate_rudder_state {
    value: ST0,
}

impl calibrate_rudder_state {
    fn __init__<T0>(&self, name: T0) {
        self.value = name;
        super(calibrate_rudder_state, self).__init__(_(name));
    }
    fn process<RT>(&self) -> RT {
        self.set(_("rudder.calibration_state"), self.value);
        return self.lcd.menu;
    }
}

struct calibrate_rudder_feedback {}

impl calibrate_rudder_feedback {
    fn __init__(&self) {
        let items = vec![calibrate_rudder_state("reset"), calibrate_rudder_state("centered"), calibrate_rudder_state("starboard range"), calibrate_rudder_state("port range"), ValueEdit(_("range"), _("degrees"), "rudder.range")];
        super(calibrate_rudder_feedback, self).__init__(_("rudder"), items);
    }
    fn process<RT>(&self) -> RT {
        if self.last_val("rudder.angle") == false {
            if self.testkeydown(MENU) {
                return self.prev;
            }
        }
        return super(calibrate_rudder_feedback, self).process();
    }
    fn display<T0>(&self, refresh: T0) {
        if self.last_val("rudder.angle") == false {
            self.
            box (rectangle(0, 0.18, 1, 0.82), black);
            self.fittext(rectangle(0, 0.4, 1, 0.4), "No Rudder Feedback Detected", true);
            return;
        }
        super(calibrate_rudder_feedback, self).display(true);
        self.fittext(rectangle(0, 0.9, 1, 0.1), String::from(self.last_val("rudder.angle")));
    }
}

struct calibrate {
    lastcounter: ST0,
}

impl calibrate {
    fn __init__(&self) {
        super(calibrate, self).__init__(_("calibrate"), vec![level(_("level")), ValueEdit(_("heading"), self.getheading, "imu.heading_offset"), ValueCheck(_("lock"), "imu.compass.calibration.locked"), calibrate_rudder_feedback(), calibrate_info()]);
        self.lastcounter = 0;
    }
    fn getheading<RT>(&self) -> RT {
        let try_dummy = { //unsupported
            return ("%.1f" % self.last_val("imu.heading"));
        };
        let except!() = { //unsupported
            return String::from(self.last_val("imu.heading"));
        };
    }
    fn display<T0>(&self, refresh: T0) {
        let counter = self.last_val("imu.alignmentCounter", 0);
        super(calibrate, self).display(refresh || counter != self.lastcounter);
        self.lastcounter = counter;
        if counter {
            let r = rectangle(0, 0, 1, 0.15);
            r.height = 0.2;
            self.fittext(r, (" %d%%" % (100 - counter)), false, black);
            r.width = (1 - (float(counter) / 100));
            r.height = 0.25;
            self.invertrectangle(r);
        }
        self.fittext(rectangle(0, 0.9, 0.5, 0.11), self.round_last_val("imu.pitch", 1));
        self.fittext(rectangle(0.5, 0.9, 0.5, 0.11), self.round_last_val("imu.heel", 1));
    }
}

struct motor {}

impl motor {
    fn __init__(&self) {
        super(motor, self).__init__(_("motor"), vec![ValueEdit(_("min speed"), _("relative"), "servo.speed.min"), ValueEdit(_("max speed"), _("relative"), "servo.speed.max"), ValueEdit(_("max current"), _("amps"), "servo.max_current"), ValueEdit(_("period"), _("seconds"), "servo.period"), ValueEdit(_("clutch pwm"), _("percent"), "servo.clutch_pwm")]);
    }
}

const networking: _ = "/home/tc/.pypilot/networking.txt";
const default_network: _ = [("mode", "Master"), ("ssid", "pypilot"), ("key", ""), ("client_ssid", "openplotter"), ("client_key", "12345678")].iter().cloned().collect::<HashMap<_, _>>();

struct select_wifi {
    wifi_settings: ST0,
}

impl select_wifi {
    fn __init__<T0, T1>(&self, name: T0, wifi_settings: T1) {
        self.wifi_settings = wifi_settings;
        super(select_wifi, self).__init__(name);
    }
    fn setup_network(&self) {
        let try_dummy = { //unsupported
            let f = open(networking, "w");
            for setting in self.wifi_settings {
                f.write((((setting + "=") + self.wifi_settings[setting]) + "
"));
            }
            f.close();
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} {:?} ", "exception writing", networking, ":", e);
        };
        os.system("/opt/networking.sh");
    }
}

struct select_wifi_ap_toggle {}

impl select_wifi_ap_toggle {
    fn process<RT>(&self) -> RT {
        let ap = self.wifi_settings["mode"] == "Master";
        self.wifi_settings["mode"] = if ap { "Managed" } else { "Master" };
        self.setup_network();
        return self.lcd.menu;
    }
}

struct select_wifi_defaults {}

impl select_wifi_defaults {
    fn process<RT>(&self) -> RT {
        for (n, v) in default_network.items() {
            self.wifi_settings[n] = v;
        }
        self.setup_network();
        return self.lcd.menu;
    }
}

struct wifi {
    wifi_settings: bool,
    have_wifi: bool,
    mtime: bool,
    wifi_updatetime: ST0,
}

impl wifi {
    fn __init__(&self) {
        self.wifi_settings = false;
        self.have_wifi = false;
        self.mtime = false;
        self.wifi_updatetime = gettime();
        if self.read_networking() {
            items = vec![select_wifi_ap_toggle("AP/Client", self.wifi_settings), select_wifi_defaults(_("defaults"), self.wifi_settings)];
        } else {
            items = vec![];
        }
        super(wifi, self).__init__("WIFI", items);
    }
    fn read_networking<RT>(&self) -> RT {
        let try_dummy = { //unsupported
            let mtime = os.path.getmtime(networking);
            if mtime == self.mtime {
                return false;
            }
            self.wifi_settings = default_network.copy();
            let f = open(networking, "r");
            while true {
                let l = f.readline();
                if !l {
                    break;
                }
                let parsed = l.rstrip().split("=", 1);
                if parsed.len() == 2 {
                    let (setting, value) = parsed;
                    self.wifi_settings[setting] = value;
                }
            }
            f.close();
            return true;
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ", "failed to read", networking, e);
            return false;
        };
    }
    fn display<T0>(&self, refresh: T0) {
        if !self.wifi_settings {
            self.fill(black);
            self.fittext(rectangle(0, 0, 1, 1), _("Wifi not managed"), true);
            return;
        }
        super(wifi, self).display(refresh);
        self.have_wifi = test_wifi();
        if !self.have_wifi {
            let mut info = _("No Connetion");
            self.fittext(rectangle(0, 0, 1, 0.2), info);
        }
        if self.wifi_settings["mode"] == "Master" {
            info = "mode: AP
";
            ssid = "ssid";
            key = "key";
        } else {
            info = "mode: Client
";
            ssid = "client_ssid";
            key = "client_key";
        }
        self.fittext(rectangle(0, 0.6, 1, 0.13), info);
        let mut info = self.wifi_settings[ssid];
        self.fittext(rectangle(0, 0.73, 1, 0.14), info);
        if self.wifi_settings[key] {
            info = self.wifi_settings[key];
            self.fittext(rectangle(0, 0.87, 1, 0.13), info);
        }
    }
    fn process<RT>(&self) -> RT {
        let have_wifi = test_wifi();
        if have_wifi != self.have_wifi {
            self.lcd.need_refresh = true;
        }
        let t = gettime();
        if (t - self.wifi_updatetime) > 1 {
            self.wifi_updatetime = t;
            if self.read_networking() {
                self.lcd.need_refresh = true;
            }
        }
        return super(wifi, self).process();
    }
}

struct control_menu {}

impl control_menu {
    fn __init__(&self) {
        super(control_menu, self).__init__(_("control"), vec![wifi(), ConfigEdit(_("small step"), _("degrees"), "smallstep", 1, 5), ConfigEdit(_("big step"), _("degrees"), "bigstep", 5, 20)]);
    }
}

struct invert {}

impl invert {
    fn process<RT>(&self) -> RT {
        self.lcd.config["invert"] = !self.lcd.config["invert"];
        self.lcd.write_config();
        return self.lcd.menu;
    }
}

struct flip {}

impl flip {
    fn process<RT>(&self) -> RT {
        self.lcd.config["flip"] = !self.lcd.config["flip"];
        self.lcd.write_config();
        return self.lcd.menu;
    }
}

struct BacklightEdit {}

impl BacklightEdit {
    fn __init__(&self) {
        super(BacklightEdit, self).__init__(_("backlight"), "", "backlight", false, 0, 40);
    }
    fn move < T0>( & self , delta: T0)  {
    super (BacklightEdit, self ).move (delta);
    self.lcd.send("backlight", self.value);
    }
}

struct display {}

impl display {
    fn __init__(&self) {
        if micropython {
            bl = vec![ConfigEdit(_("hue"), "", "hue", 0, 255)];
        } else {
            bl = vec![BacklightEdit(), ConfigEdit(_("buzzer"), _("buzzer"), "buzzer", 0, 2)];
        }
        super(display, self).__init__(_("display"), (vec![ConfigEdit(_("contrast"), "", "contrast", 0, 120), invert(_("invert")), flip(_("flip"))] + bl));
    }
}

struct select_language {
    lang: ST0,
}

impl select_language {
    fn __init__<T0>(&self, lang: T0) {
        super(select_language, self).__init__(lang[0]);
        self.lang = lang[1];
    }
    fn process<RT>(&self) -> RT {
        self.lcd.set_language(self.lang);
        self.lcd.menu = mainmenu(self.lcd);
        return self.lcd.menu;
    }
}

struct language {
    selection: ST0,
}

impl language {
    let languages = vec![("català", "ca"), ("dansk", "da"), ("deutsch", "de"), ("Eλληνικά", "el"), ("english", "en"), ("español", "es"), ("suomalainen", "fi"), ("français", "fr"), ("italiano", "it"), ("nederlands", "nl"), ("norsk", "no"), ("polskie", "pl"), ("português", "pt"), ("pycкий", "ru"), ("svenska", "sv")];
    fn __init__(&self) {
        super(language, self).__init__(_("language"), self.languages.iter().map(select_language).collect::<Vec<_>>());
        self.selection = -1;
    }
    fn process<RT>(&self) -> RT {
        if self.selection < 0 {
            let (index, self.selection) = (0, 0);
            for lang in self.languages {
                if lang[1] == self.lcd.config["language"] {
                    self.selection = index;
                }
                index += 1;
            }
        }
        return super(language, self).process();
    }
}

struct settings {}

impl settings {
    fn __init__(&self) {
        if no_translation == translate {
            lang = vec![];
        } else {
            lang = vec![language()];
        }
        super(settings, self).__init__(_("settings"), (vec![ValueEnum(_("mode"), "ap.mode"), ValueEnum(_("pilot"), "ap.pilot"), motor(), control_menu(), display()] + lang));
    }
}

struct mainmenu {
    lcd: ST0,
    loadtime: ST1,
}

impl mainmenu {
    fn __init__<T0>(&self, lcd: T0) {
        super(mainmenu, self).__init__(_("Menu"), vec![gain(), calibrate(), settings(), info()]);
        self.lcd = lcd;
        self.find_parents();
        self.loadtime = 0;
    }
    fn display<T0>(&self, refresh: T0) {
        let values = self.lcd.get_values();
        if !values {
            if !self.loadtime {
                if self.lcd.client.connection {
                    self.loadtime = gettime();
                    self.lcd.client.list_values();
                } else {
                    self.loadtime = 0;
                }
            } else {
                let mut dt = (gettime() - self.loadtime);
                self.lcd.surface.fill(black);
                if dt > 0.2 {
                    self.fittext(rectangle(0, 0, 1, 0.4), _("Loading"));
                }
            }
        } else {
            if self.loadtime {
                refresh = true;
            }
            self.loadtime = 0;
        }
        if self.loadtime {
            let mut dt = (gettime() - self.loadtime);
            if dt > 11 {
                self.fittext(rectangle(0, 0.4, 1, 0.2), _("timeout"));
            } else {
                if dt > 10 {
                    self.loadtime = 0;
                } else {
                    if dt > 0.6 {
                        self.fittext(rectangle(0, 0.4, 1, 0.2), ("." * i32::from(((dt * 2) + 0.5))));
                    }
                }
            }
        } else {
            super(mainmenu, self).display(refresh);
        }
    }
}