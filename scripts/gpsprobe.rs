use std::collections::HashMap;
use std::*;

struct GpsProbe {
    lastgpsdevice: ST0,
    gpsd: ST1,
}

impl GpsProbe {
    fn __init__(&self) {
        self.lastgpsdevice = "";
    }
    fn connect(&self) {
        while true {
            let try_dummy = {
                //unsupported
                self.gpsd = gps.gps(gps.WATCH_ENABLE);
                self.gpsd.next();
                self.gpsd.activated = false;
                return;
            };
            let except!() = {
                //unsupported
                time.sleep(3);
            };
        }
    }
    fn probe<RT>(&self) -> RT {
        if !os.system("timeout -s KILL -t 5 gpsctl 2> /dev/null") {
            return true;
        }
        let devicesp = vec!["/dev/gps", "/dev/ttyUSB", "/dev/ttyAMA", "/dev/ttyS"];
        let mut devices = vec![self.lastgpsdevice];
        for devicep in devicesp {
            for i in (0..4) {
                devices.push((devicep + ("%d" % i)));
            }
        }
        for device in devices {
            if !os.path.exists(device) {
                continue;
            }
            if !os.system((("timeout -s KILL -t 5 gpsctl -f " + device) + " 2> /dev/null")) {
                os.environ["GPSD_SOCKET"] = "/tmp/gpsd.sock";
                os.system(("gpsdctl add " + device));
                println!("{:?} ", ("GPS found: " + device));
                self.lastgpsdevice = device;
                return true;
            }
            sys.stdout.flush();
        }
        return false;
    }
    fn verify(&self) {
        while true {
            let try_dummy = {
                //unsupported
                let result = self.gpsd.next();
                if result.iter().any(|&x| x == "devices") {
                    let mut activated = result["devices"].len() > 0;
                    if activated != self.gpsd.activated {
                        println!(
                            "{:?} ",
                            (("GPS " + if activated { "" } else { "de" }) + "activated")
                        );
                    }
                    self.gpsd.activated = activated;
                    break;
                }
            };
            let except!(StopIteration) = {
                //unsupported
                println!("{:?} ", "GPS lost gpsd");
                return;
            };
        }
        if !self.gpsd.activated {
            if self.probe() {
                self.gpsd.activated = true;
                println!("{:?} ", "GPS probe success");
            }
        }
        let mut activated = self.gpsd.activated;
        self.gpsd.drop();
        if activated {
            time.sleep(60);
        } else {
            time.sleep(15);
        }
    }
}
fn main() {
    let gpsprobe = GpsProbe();
    let try_dummy = {
        //unsupported
        while true {
            gpsprobe.connect();
            gpsprobe.verify();
        }
    };
    let except!(KeyboardInterrupt) = {
        //unsupported
        println!("{:?} ", "Keyboard interrupt, gpsprobe exit");
    };
}
fn main() {
    main();
}
