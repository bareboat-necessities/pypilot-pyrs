use std::*;
use std::collections::HashMap;

const RF: _ = 1;
const IR: _ = 2;
const GP: _ = 3;
const VOLTAGE: _ = 4;
const SET_BACKLIGHT: _ = 22;
const SET_BUZZER: _ = 23;
const SET_BAUD: _ = 24;
const PACKET_LEN: _ = 6;
let try_dummy = { //unsupported
};
let except!() = { //unsupported
const GPIO: _ = false;
};
struct arduino {
    spi: bool,
    nmea_socket: bool,
    nmea_connect_time: ST0,
    pollt0: ST1,
    config: ST2,
    debug: ST3,
    hatconfig: bool,
    backlight_polarity: bool,
    lasttime: ST4,
    sent_count: ST5,
    sent_start: ST6,
    socketdata: ST7,
    serial_in_count: ST8,
    serial_out_count: ST9,
    serial_time: ST10,
    packetout_data: Vec<_>,
    packetin_data: Vec<_>,
    resetpin: ST11,
    nmea_socket_poller: ST12,
}

impl arduino {
    fn __init__<T0>(&self, config: T0) {
        self.spi = false;
        self.nmea_socket = false;
        self.nmea_connect_time = time.monotonic();
        self.pollt0 = vec![0, time.monotonic()];
        self.config = config;
        if config.iter().any(|&x| x == "arduino.debug") && config["arduino.debug"] {
            self.debug = print;
        } else {
            self.debug = || None;
        }
        self.hatconfig = false;
        self.backlight_polarity = false;
        if config.iter().any(|&x| x == "hat") {
            let hatconfig = config["hat"];
            if hatconfig && hatconfig.iter().any(|&x| x == "arduino") {
                self.hatconfig = hatconfig["arduino"];
            }
            if hatconfig && hatconfig.iter().any(|&x| x == "lcd") && hatconfig["lcd"].iter().any(|&x| x == "driver") {
                self.backlight_polarity = hatconfig["lcd"]["driver"] == "nokia5110";
            }
        }
        if !self.hatconfig {
            println!("{:?} ", "No hat config, arduino not found");
        }
        self.lasttime = 0;
        self.sent_count = 0;
        self.sent_start = time.monotonic();
        self.socketdata = b"";
        self.serial_in_count = 0;
        self.serial_out_count = 0;
        self.serial_time = (self.sent_start + 2);
        self.packetout_data = vec![];
        self.packetin_data = vec![];
    }
    fn firmware(&self) {
        if !self.hatconfig {
            return;
        }
        let device = self.hatconfig["device"];
        if !device {
            return;
        }
        if device.startswith("/dev/spidev") {
            let filename = (os.getenv("HOME") + "/.pypilot/hat.hex");
            if !os.path.exists(filename) {
                println!("{:?} {:?} ", "hat firmware not in", filename);
                println!("{:?} ", "skipping verification");
            } else {
                if !self.verify(filename) && !self.verify(filename) {
                    if !self.write(filename) || !self.verify(filename) {
                        println!("{:?} {:?} ", "failed to verify or upload", filename);
                    }
                }
            }
            self.resetpin = self.hatconfig["resetpin"];
            let try_dummy = { //unsupported
                GPIO.setmode(GPIO.BCM);
                GPIO.setup(i32::from(self.resetpin), GPIO.IN, GPIO.PUD_UP);
                /*pass*/
            };
            let except!(Exception) = { //unsupported
                println!("{:?} {:?} ", "arduino failed to set reset pin high", e);
            };
        }
    }
    fn open(&self) {
        if !self.hatconfig {
            return;
        }
        let device = self.hatconfig["device"];
        if !device {
            return;
        }
        let try_dummy = { //unsupported
            let (port, slave) = (i32::from(device[11]), i32::from(device[13]));
            println!("{:?} ", ("arduino on spidev%d.%d" % (port, slave)));
            if false {
                use spidev;
                self.spi = spidev.SpiDev();
                self.spi.open(port, slave);
                self.spi.max_speed_hz = 100000;
            } else {
                use pypilot::hat::spireader::{spireader};
                self.spi = spireader.spireader(10, 10);
                if self.spi.open(port, slave, 100000) == -1 {
                    self.close();
                }
            }
            if self.config.iter().any(|&x| x == "lcd") && self.config["lcd"].iter().any(|&x| x == "backlight") {
                self.set_backlight(self.config["lcd"]["backlight"]);
            }
            self.set_baud(self.config["arduino.nmea.baud"]);
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ", "failed to communicate with arduino", device, e);
            self.hatconfig = false;
            self.spi = false;
        };
    }
    fn close<T0>(&self, e: T0) {
        println!("{:?} {:?} ", "failed to read spi:", e);
        self.spi.close();
        self.spi = false;
    }
    fn xfer<T0, RT>(&self, x: T0) -> RT {
        return self.spi.xfer(vec![x])[0];
    }
    fn send<T0, T1>(&self, id: T0, data: T1) {
        let mut p = id;
        self.packetout_data += bytes(vec![(ord("$") | 128), (id | 128)]);
        for i in (0..(PACKET_LEN - 1)) {
            if i < data.len() {
                d = data[i];
            } else {
                d = 0;
            }
            p ^= d;
            self.packetout_data += bytes(vec![(d | 128)]);
        }
        self.packetout_data += bytes(vec![(p | 128)]);
    }
    fn set_backlight<T0>(&self, value: T0) {
        value = i32::from((value * 3)).iter().max().unwrap().iter().min().unwrap();
        let backlight = vec![value, self.backlight_polarity];
        self.send(SET_BACKLIGHT, backlight);
    }
    fn set_baud<T0>(&self, baud: T0) {
        let try_dummy = { //unsupported
            baud = i32::from(baud);
        };
        let except!() = { //unsupported
            baud = 38400;
        };
        if baud == 4800 {
            let mut d = vec![5];
        } else {
            if baud == 38400 {
                d = vec![8];
            } else {
                println!("{:?} {:?} ", "invalid baud", baud);
                d = vec![8];
            }
        }
        self.debug("nmea set baud", d);
        self.send(SET_BAUD, d);
    }
    fn set_buzzer<T0, T1>(&self, mode: T0, duration: T1) {
        duration = i32::from((duration.iter().max().unwrap().iter().min().unwrap() * 100));
        self.send(SET_BUZZER, (mode, duration));
    }
    fn get_baud_rate<RT>(&self) -> RT {
        let t = time.monotonic();
        let dt = (t - self.serial_time);
        if dt < 10 {
            return false;
        }
        self.serial_time = t;
        let rate_in = ((self.serial_in_count * 10) / dt);
        let rate_out = ((self.serial_out_count * 10) / dt);
        self.serial_in_count = 0;
        return ("TX: %.1f  RX %.1f" % (rate_in, rate_out));
    }
    fn poll<RT>(&self) -> RT {
        let t0 = time.monotonic();
        if !self.spi {
            self.open();
            return vec![];
        }
        let mut events = vec![];
        let mut serial_data = vec![];
        self.open_nmea();
        let baud = (i32::from(self.config["arduino.nmea.baud"]) * 0.9);
        let t1 = time.monotonic();
        while true {
            if self.nmea_socket && self.socketdata.len() < 100 && self.nmea_socket_poller.poll(0) {
                let try_dummy = { //unsupported
                    let b = self.nmea_socket.recv(40);
                    if self.config["arduino.nmea.out"] {
                        self.socketdata += b;
                        self.serial_in_count += b.len();
                    }
                };
                let except!(Exception) = { //unsupported
                    if e.args[0] == errno.EWOULDBLOCK {
                        /*pass*/
                    } else {
                        println!("{:?} {:?} ", "nmea socket exception reading", e);
                        self.nmea_socket.close();
                        self.nmea_socket = false;
                    }
                };
            }
            let mut i = 0;
            if self.socketdata {
                let (count, t0) = self.pollt0;
                let dt = (time.monotonic() - t0);
                let rate = ((10 * count) / dt);
                if rate < baud {
                    if self.socketdata[0] && self.socketdata[0] < 128 {
                        i = self.socketdata[0];
                        self.pollt0[0] += 1;
                        self.socketdata = self.socketdata[1..];
                    }
                }
            } else {
                self.pollt0 = vec![0, time.monotonic()];
            }
            if !i && self.packetout_data {
                i = self.packetout_data[0];
                self.packetout_data = self.packetout_data[1..];
            }
            let mut o = self.spi.xfer(i, !i && self.packetin_data.len() < (PACKET_LEN + 3));
            if !i && !o {
                break;
            }
            if o > 127 {
                o &= 127;
                self.packetin_data.append(o);
            } else {
                if o {
                    serial_data.push(o);
                }
            }
        }
        let t2 = time.monotonic();
        while self.packetin_data.len() >= (PACKET_LEN + 3) {
            if self.packetin_data[0] != ord("$") {
                self.packetin_data = self.packetin_data[1..];
                continue;
            }
            let cmd = self.packetin_data[1];
            let d = self.packetin_data[2..(PACKET_LEN + 2)];
            let parity = self.packetin_data[(PACKET_LEN + 2)];
            let mut p = 0;
            for x in d {
                p ^= x;
            }
            if p != parity {
                self.packetin_data = self.packetin_data[1..];
                continue;
            }
            self.packetin_data = self.packetin_data[(3 + PACKET_LEN)..];
            let mut key = ("%02X%02X%02X%02X" % (d[0], d[1], d[2], d[3]));
            let count = d[4];
            if cmd == RF {
                key = ("rf" + key);
            } else {
                if cmd == IR {
                    if !self.config["arduino.ir"] {
                        continue;
                    }
                    key = ("ir" + key);
                } else {
                    if cmd == GP {
                        key = ("gpio_ext" + key);
                    } else {
                        if cmd == VOLTAGE {
                            let vcc = ((d[0] + (d[1] << 7)) / 1000.0);
                            let vin = ((d[2] + (d[3] << 7)) / 1000.0);
                            events.push(vec!["voltage", [("vcc", vcc), ("vin", vin)].iter().cloned().collect::<HashMap<_, _>>()]);
                            continue;
                        } else {
                            println!("{:?} {:?} {:?} ", "unknown message", cmd, d);
                            continue;
                        }
                    }
                }
            }
            events.push(vec![key, count]);
        }
        let t3 = time.monotonic();
        if serial_data {
            self.debug("nmea>", bytes(serial_data));
            if self.nmea_socket && self.config["arduino.nmea.in"] {
                let try_dummy = { //unsupported
                    self.nmea_socket.send(bytes(serial_data));
                };
                let except!(Exception) = { //unsupported
                    println!("{:?} {:?} ", "nmea socket exception sending", e);
                    self.nmea_socket.close();
                    self.nmea_socket = false;
                };
            }
            self.serial_out_count += serial_data.len();
        }
        let t4 = time.monotonic();
        return events;
    }
    fn open_nmea(&self) {
        let c = self.config;
        if !c["arduino.nmea.in"] && !c["arduino.nmea.out"] {
            if self.nmea_socket {
                self.nmea_socket.close();
                self.nmea_socket = false;
            }
            return;
        }
        if self.nmea_socket {
            return;
        }
        if time.monotonic() < self.nmea_connect_time {
            return;
        }
        self.nmea_connect_time += 8;
        self.socketdata = b"";
        let try_dummy = { //unsupported
            self.nmea_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM);
            self.nmea_socket.setblocking(0);
            self.nmea_socket.connect((self.config["host"], 20220));
        };
        let except!(OSError) = { //unsupported
            if e.args[0] == errno.EINPROGRESS {
                self.nmea_socket_poller = select.poll();
                self.nmea_socket_poller.register(self.nmea_socket, select.POLLIN);
                let try_dummy = { //unsupported
                    self.nmea_socket.send(bytes("$PYPBS*48
", "utf-8"));
                };
                let except!(Exception) = { //unsupported
                    println!("{:?} {:?} ", "nmea socket exception sending", e);
                    self.nmea_socket.close();
                    self.nmea_socket = false;
                };
            } else {
                println!("{:?} {:?} ", "os error", e);
                self.nmea_socket = false;
            }
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ", "exception", e);
            self.nmea_socket = false;
        };
        let except!() = { //unsupported
            println!("{:?} ", "MOOOOOOOOOOOOOOORE");
        };
    }
    fn flash<T0, T1, RT>(&self, filename: T0, c: T1) -> RT {
//global GPIO
        if !GPIO {
            return false;
        }
        self.resetpin = self.hatconfig["resetpin"];
        let try_dummy = { //unsupported
            GPIO.setmode(GPIO.BCM);
            GPIO.setup(i32::from(self.resetpin), GPIO.OUT);
            GPIO.output(i32::from(self.resetpin), 0);
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ", "failed to setup gpio reset pin for arduino", e);
            GPIO = false;
            return false;
        };
        let command = (((((("avrdude -P " + self.hatconfig["device"]) + " -u -p atmega328p -c linuxspi -U f:") + c) + ":") + filename) + " -b 500000");
        println!("{:?} {:?} ", "cmd", command);
        let ret = os.system(command);
        GPIO.output(i32::from(self.resetpin), 1);
        GPIO.setup(i32::from(self.resetpin), GPIO.IN);
        return !ret;
    }
    fn verify<T0, RT>(&self, filename: T0) -> RT {
        return self.flash(filename, "v");
    }
    fn write<T0, RT>(&self, filename: T0) -> RT {
        return self.flash(filename, "w");
    }
}

fn arduino_process<T0, T1>(pipe: T0, config: T1) {
    let start = time.monotonic();
    let a = arduino(config);
    let mut period = 0.05;
    let mut periodtime = 0;
    while true {
        let t0 = time.monotonic();
        let mut events = a.poll();
        let t1 = time.monotonic();
        let baud_rate = a.get_baud_rate();
        if baud_rate {
            a.debug("nmea baud rate", baud_rate);
            if a.nmea_socket {
                events.append(vec!["baudrate", baud_rate]);
            } else {
                events.append(vec!["baudrate", "ERROR: no connection to server for nmea"]);
            }
        }
        if events && (t0 - start) > 2 {
            pipe.send(events);
            period = 0.05;
            periodtime = t0;
        } else {
            if (periodtime - t0) > 5 {
                period = 0.2;
            }
        }
        period = 0.01;
        while true {
            let try_dummy = { //unsupported
                let msg = pipe.recv();
                if !msg {
                    break;
                }
                let (name, value) = msg;
            };
            let except!(Exception) = { //unsupported
                println!("{:?} ", "pipe recv failed!!
");
                return;
            };
            config[name] = value;
            if name == "backlight" {
                a.set_backlight(value);
            } else {
                if name == "buzzer" {
                    a.set_buzzer(starred!(value)/*unsupported*/);
                } else {
                    if name == "arduino.nmea.baud" {
                        a.set_baud(value);
                    }
                }
            }
        }
        let t2 = time.monotonic();
        let dt = (period - (t2 - t0));
        if dt > 0 {
            time.sleep(dt);
        }
    }
}

fn main() {
    println!("{:?} ", "initializing arduino");
    let config = [("host", "localhost"), ("hat", [("arduino", [("device", "/dev/spidev0.1"), ("resetpin", "16")].iter().cloned().collect::<HashMap<_, _>>())].iter().cloned().collect::<HashMap<_, _>>()), ("arduino.nmea.baud", 4800), ("arduino.nmea.in", false), ("arduino.nmea.out", false), ("arduino.ir", true), ("arduino.debug", true)].iter().cloned().collect::<HashMap<_, _>>();
    let a = arduino(config);
    let dt = 0;
    let mut lt = 0;
    while true {
        let t0 = time.monotonic();
        let events = a.poll();
        if events {
            println!("{:?} {:?} {:?} ", events, dt, (t0 - lt));
            lt = t0;
        }
        let baud_rate = a.get_baud_rate();
        if baud_rate {
            println!("{:?} {:?} ", "baud rate", baud_rate);
        }
    }
}

fn main() {
    main();
}