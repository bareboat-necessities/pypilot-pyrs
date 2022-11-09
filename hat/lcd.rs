use std::*;
use std::collections::HashMap;

use page::{*};
use page::{_};
let try_dummy = { //unsupported
use upy_client::{pypilotClient};
fn gettime < RT > () -> RT {
return (time.ticks_ms() / 1000.0);
}
};
let except!() = { //unsupported
use pypilot::client::{pypilotClient};
fn gettime < RT > () -> RT {
return time.monotonic();
}
use pypilot::hat::ugfx::{ugfx};
const micropython: _ = false;
};
const driver: _ = "default";
for pdriver in vec!["nokia5110", "jlx12864", "glut", "framebuffer", "tft", "none"] {
if sys.argv.iter().any( | & x | x == pdriver) {
println ! ("{:?} {:?} {:?} {:?} ", "overriding driver", driver, "to command line", pdriver);
driver = pdriver;
sys.argv.remove(driver);
break;
}
}
struct Key {
    time: ST0,
    down: ST1,
    up: bool,
}

impl Key {
    fn __init__(&self) {
        self.time = 0;
        self.down = 0;
        self.up = false;
    }
    fn update<T0, T1>(&self, down: T0, count: T1) {
        if down {
            if !self.time {
                self.down += 1;
                self.time = gettime();
            }
            if count {
                let t0 = (gettime() - (count * 0.1));
                self.time = t0.iter().min().unwrap();
            }
        } else {
            if self.time {
                self.up = true;
                self.time = 0;
            }
        }
    }
    fn dt<RT>(&self) -> RT {
        let t0 = gettime();
        let dt = (t0 - self.time);
        if self.time {
            if dt > 10 {
                self.time = (t0 - 10);
            }
            return dt;
        }
        return 0;
    }
}

struct LCD {
    config: ST0,
    pipe: bool,
    poller: bool,
    voltage: bool,
    host: ST1,
    battery_voltage: ST2,
    keypress: bool,
    surface: Option<_>,
    use_glut: bool,
    bw: Option<_>,
    glutkeytime: bool,
    mag: ST3,
    magsurface: ST4,
    invsurface: ST5,
    screen: ST6,
    client: bool,
    menu: bool,
    page: ST7,
    need_refresh: bool,
    keypad: Vec<_>,
    blink: ST8,
    data_update: bool,
    last_msg: HashMap<_, _>,
    blinktime: ST9,
    registered: bool,
}

impl LCD {
    fn __init__<T0>(&self, config: T0) {
        if config {
            self.config = config["lcd"];
        } else {
            self.config = HashMap::new();
        }
        self.pipe = false;
        self.poller = false;
        self.voltage = false;
        let default = [("contrast", 60), ("invert", false), ("backlight", 20), ("hue", 214), ("flip", false), ("language", "en"), ("bigstep", 10), ("smallstep", 1), ("buzzer", 2)].iter().cloned().collect::<HashMap<_, _>>();
        for name in default {
            if !self.config.iter().any(|&x| x == name) {
                self.config[name] = default[name];
            }
        }
//global driver
        if config && config.iter().any(|&x| x == "hat") {
            if driver == "default" {
                driver = config["hat"]["lcd"]["driver"];
            }
            self.host = config["host"];
        } else {
            self.host = false;
        }
        self.battery_voltage = 0;
        let use_tft = if micropython { true } else { false };
        self.keypress = false;
        let use_glut = !use_tft && os.environ.iter().any(|&x| x == "DISPLAY");
        self.surface = None;
        self.use_glut = false;
        println!("{:?} {:?} {:?} {:?} ", "lcd driver", driver, use_tft, use_glut);
        if driver == "none" {
            let page = None;
            let mut screen = None;
            self.bw = None;
        } else {
            if driver == "tft" || driver == "default" && use_tft {
                use gc;
                if gc.mem_free() > 1000000.0 {
                    screen = ugfx::surface(240, 320, 1);
                } else {
                    screen = ugfx::surface(136, 240, 1);
                }
                self.surface = screen;
            } else {
                if driver == "nokia5110" || driver == "default" && !use_glut {
                    screen = ugfx::spiscreen(0);
                } else {
                    if driver == "jlx12864" {
                        screen = ugfx::spiscreen(1);
                    } else {
                        if driver == "glut" || driver == "default" && use_glut {
                            self.use_glut = true;
                            println!("{:?} ", "using glut");
                            use glut;
                            screen = glut.screen((136, 240));
                            use OpenGL::GLUT::{glutKeyboardFunc, glutKeyboardUpFunc};
                            use OpenGL::GLUT::{glutSpecialFunc, glutSpecialUpFunc};
                            glutKeyboardFunc(self.glutkeydown);
                            glutKeyboardUpFunc(self.glutkeyup);
                            glutSpecialFunc(self.glutspecialdown);
                            glutSpecialUpFunc(self.glutspecialup);
                            self.glutkeytime = false;
                        } else {
                            if driver == "framebuffer" {
                                println!("{:?} ", "using framebuffer");
                                screen = ugfx::screen("/dev/fb0");
                                if screen.width > 480 {
                                    println!("{:?} ", "warning huge width");
                                }
                            }
                        }
                    }
                }
            }
        }
        if screen {
            self.bw = if screen.width < 120 { 1 } else { false };
            self.mag = 1;
            if !self.surface {
                let (w, h) = (screen.width, screen.height);
                self.surface = ugfx::surface(w, h, screen.bypp, None);
                self.mag = (screen.width / self.surface.width).iter().min().unwrap();
                if self.mag != 1 {
                    println!("{:?} ", "magnifying lcd surface to fit screen");
                    self.magsurface = ugfx::surface(screen);
                }
                self.invsurface = ugfx::surface(self.surface);
            }
        } else {
            self.surface = None;
        }
        self.screen = screen;
        set_language(self.config["language"]);
        self.client = false;
        self.connect();
        self.menu = false;
        self.page = connecting(self);
        self.need_refresh = true;
        self.keypad = vec![];
        for i in (0..NUM_KEYS) {
            self.keypad.append(Key());
        }
        self.blink = (black, white);
        self.data_update = false;
    }
    fn getmenu<RT>(&self) -> RT {
        if !self.menu {
            use menu::{mainmenu};
            self.menu = mainmenu(self);
        }
        return self.menu;
    }
    fn set_language<T0>(&self, lang: T0) {
        set_language(lang);
        self.config["language"] = lang;
        self.write_config();
    }
    fn connect(&self) {
        self.last_msg = HashMap::new();
        self.last_msg["gps.source"] = "none";
        self.last_msg["wind.source"] = "none";
        self.last_msg["truewind.source"] = "none";
        self.last_msg["ap.heading_command"] = 0;
        if self.client {
            self.client.disconnect();
        }
        self.client = pypilotClient(self.host);
    }
    fn send<T0, T1>(&self, key: T0, code: T1) {
        if self.pipe {
            self.pipe.send((key, code));
        }
    }
    fn write_config(&self) {
        if micropython {
            use config_esp32::{write_config};
            write_config(self.config);
        } else {
            self.send("write_config", self.config);
        }
    }
    fn get_values<RT>(&self) -> RT {
        return self.client.get_values();
    }
    fn key<T0, T1>(&self, k: T0, down: T1) {
        if k < 0 || k >= self.keypad.len() {
            return;
        }
        self.keypad[k].update(down);
    }
    fn glutkeydown<T0, T1, T2>(&self, k: T0, x: T1, y: T2) {
        self.glutkey(k);
    }
    fn glutkeyup<T0, T1, T2>(&self, k: T0, x: T1, y: T2) {
        self.glutkey(k, false);
    }
    fn glutkey<T0, T1>(&self, k: T0, down: T1) {
        k = k.decode();
        if k == "q" || ord(k) == 27 {
            exit(0);
        }
        if k == " " {
            let mut key = AUTO;
        } else {
            if k == "
" {
                key = MENU;
            } else {
                if k == "	" {
                    key = SELECT;
                } else {
                    key = (ord(k) - ord("1"));
                }
            }
        }
        self.key(key, down);
    }
    fn glutspecialdown<T0, T1, T2>(&self, k: T0, x: T1, y: T2) {
        self.glutspecial(k);
    }
    fn glutspecialup<T0, T1, T2>(&self, k: T0, x: T1, y: T2) {
        self.glutspecial(k, false);
    }
    fn glutspecial<T0, T1>(&self, k: T0, down: T1) {
        use OpenGL::{GLUT};
        if k == glut.GLUT_KEY_UP {
            self.key(SMALL_PORT, down);
        } else {
            if k == glut.GLUT_KEY_DOWN {
                self.key(SMALL_STARBOARD, down);
            } else {
                if k == glut.GLUT_KEY_LEFT {
                    self.key(BIG_PORT, down);
                } else {
                    if k == glut.GLUT_KEY_RIGHT {
                        self.key(BIG_STARBOARD, down);
                    }
                }
            }
        }
    }
    fn display(&self) {
        let t0 = gettime();
        if micropython {
            self.page.watches["ap.heading"] = 1;
        }
        if self.page.display(self.need_refresh) {
            return;
        }
        let t1 = gettime();
        self.need_refresh = false;
        let mut surface = self.surface;
        let try_dummy = { //unsupported
            if (t0 - self.blinktime) > 0.8 {
                if self.data_update {
                    self.blink = (self.blink[1], self.blink[0]);
                    self.data_update = false;
                }
                self.blinktime = t0;
            }
        };
        let except!() = { //unsupported
            self.blinktime = 0;
        };
        let (w, h) = (self.surface.width, self.surface.height);
        let size = (h / 40);
        self.surface.
        box (((w - size) - 1), ((h - size) - 1), (w - 1), (h - 1), self.blink[0]);
        if self.screen != surface {
            if self.config["invert"] {
                self.invsurface.blit(surface, 0, 0);
                surface = self.invsurface;
                surface.invert(0, 0, surface.width, surface.height);
            }
            if self.mag != 1 {
                self.magsurface.magnify(surface, self.mag);
                surface = magsurface;
            }
            self.screen.blit(surface, 0, 0, self.config["flip"]);
        }
        if self.config.iter().any(|&x| x == "contrast") {
            self.screen.contrast = (i32::from(((float(self.config["contrast"]) * 9) / 12)) + 10);
        }
        if micropython {
            self.screen.hue = i32::from(float(self.config["hue"]));
        }
        let t2 = gettime();
        self.screen.refresh();
        let t3 = gettime();
    }
    fn update_watches(&self) {
        for name in self.client.watches.collect::<Vec<_>>() {
            if name != "values" && !self.page.watches.collect::<Vec<_>>().iter().any(|&x| x == name) {
                self.client.watch(name, false);
            }
        }
        for (name, period) in self.page.watches.items() {
            self.client.watch(name, period);
        }
    }
    fn receive(&self) {
        let msgs = self.client.receive();
        if msgs {
            self.data_update = true;
            for (name, value) in msgs.items() {
                self.last_msg[name] = value;
            }
        }
        if self.pipe {
            if !self.poller {
                self.poller = select.poll();
                self.poller.register(self.pipe, select.POLLIN);
                self.registered = true;
            }
            while true {
                let msg = self.pipe.recv();
                if !msg {
                    break;
                }
                let (index, count) = msg;
                if index == "voltage" {
                    self.voltage = count;
                } else {
                    self.keypad[index].update(count, count);
                }
            }
        }
    }
    fn reset_keys(&self) {
        for key in self.keypad {
            key.down = 0;
            key.up = false;
        }
    }
    fn check_voltage<RT>(&self) -> RT {
        if !self.voltage {
            return false;
        }
        let (vin, vcc) = (self.voltage["vin"], self.voltage["vcc"]);
        if vin < 3 || vin > 3.6 {
            return ("3v3 Voltage Bad" + (": %.2f" % vin));
        }
        if vcc < 4.5 || vcc > 5.5 {
            return ("5v Voltage Bad" + (": %.2f" % vcc));
        }
        return false;
    }
    fn poll(&self) {
        if self.screen == None {
            return;
        }
        let t0 = gettime();
        self.receive();
        let t1 = gettime();
        if !self.page {
            frameperiod = 1;
        } else {
            frameperiod = self.page.frameperiod;
        }
        let t = gettime();
        let next_page = self.page.process();
        if next_page && next_page != self.page {
            self.page = next_page;
            self.update_watches();
            self.reset_keys();
            self.need_refresh = true;
        }
        let t2 = gettime();
        self.display();
        self.update_watches();
        let t3 = gettime();
        if self.poller {
            self.poller.poll((1000 * frameperiod));
        } else {
            time.sleep(frameperiod);
        }
    }
}

fn main() {
    let lcd = LCD(false);
    fn idle() {
        lcd.poll();
        if lcd.glutkeytime {
            let (k, t) = lcd.glutkeytime;
            let dt = (gettime() - t);
            if dt > 0.5 {
                lcd.keypad[k].update(false);
                lcd.glutkeytime = false;
            }
        }
        time.sleep(0.1);
    }
    if lcd.use_glut {
        use OpenGL::GLUT::{glutMainLoop, glutIdleFunc};
        glutIdleFunc(idle);
        glutMainLoop();
    } else {
        while true {
            lcd.poll();
            time.sleep(0.1);
        }
    }
}

fn main() {
    main();
}