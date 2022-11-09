use std::collections::HashMap;
use std::*;

use pilot::{AutopilotGain, AutopilotPilot};
use pypilot::autopilot::{resolv, HeadingOffset};
use pypilot::values::*;
struct WindPilot {
    gps_wind_offset: ST0,
    last_wind_speed: ST1,
    gains: HashMap<_, _>,
}

impl WindPilot {
    fn __init__<T0>(&self, ap: T0) {
        super(WindPilot, self).__init__("wind", ap);
        self.gps_wind_offset = HeadingOffset();
        self.last_wind_speed = 0;
        self.gains = HashMap::new();
        self.PosGain("P", 0.003, 0.02);
        self.PosGain("I", 0, 0.1);
        self.PosGain("D", 0.1, 1.0);
        self.PosGain("DD", 0.05, 1.0);
        self.Gain("WG", 0, -0.1, 0.1);
    }
    fn compute_heading(&self) {
        let ap = self.ap;
        let mut compass = self.ap.boatimu.SensorValues["heading_lowpass"].value;
        let sensors = self.ap.sensors;
        let wind = sensors.wind.direction.value;
        if sensors.gps.source.value != "none" {
            let gps_track = sensors.gps.track.value;
            if ap.gps_speed > 1 {
                let d = (0.005 * math.log((ap.gps_speed + 1)));
                self.gps_wind_offset.update((wind + gps_track), d);
            }
        }
        let mode = ap.mode.value;
        if mode == "compass" {
            compass = resolv((ap.wind_compass_offset.value - wind), 180);
            ap.heading.set(compass);
        } else {
            if mode == "gps" {
                let gps = resolv((self.gps_wind_offset.value - wind), 180);
                ap.heading.set(gps);
            } else {
                if mode == "true wind" {
                    if ap.true_wind_sensor.value == "water" {
                        let mut boat_speed = sensors.water.speed;
                    } else {
                        if ap.true_wind_sensor.value == "gps" {
                            boat_speed = ap.gps_speed;
                        } else {
                            boat_speed = 0;
                        }
                    }
                    let true_wind =
                        autopilot.compute_true_wind(boat_speed, sensors.wind.speed, wind);
                    ap.heading.set(true_wind);
                } else {
                    if mode == "wind" {
                        ap.heading.set(wind);
                    }
                }
            }
        }
    }
    fn best_mode<T0, RT>(&self, mode: T0) -> RT {
        let sensors = self.ap.sensors;
        let nocompass = self.ap.boatimu.SensorValues["compass"] == false;
        let nogps = sensors.gps.source.value == "none";
        let nowater = sensors.water.source.value == "none";
        if mode == "compass" {
            if nocompass {
                return "wind";
            }
        } else {
            if nogps && nowater {
                return "wind";
            }
        }
        return mode;
    }
    fn process(&self) {
        let ap = self.ap;
        if ap.sensors.wind.source.value == "none" {
            ap.pilot.set("basic");
            return;
        }
        let headingrate = ap.boatimu.SensorValues["headingrate_lowpass"].value;
        let headingraterate = ap.boatimu.SensorValues["headingraterate_lowpass"].value;
        let mut windgust = (ap.sensors.wind.speed - self.last_wind_speed);
        self.last_wind_speed = ap.sensors.wind.speed;
        if ap.sensors.wind.direction < 0 {
            windgust = -(windgust);
        }
        let gain_values = [
            ("P", self.heading_error.value),
            ("I", self.heading_error_int.value),
            ("D", headingrate),
            ("DD", headingraterate),
            ("WG", windgust),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        let command = self.Compute(gain_values);
        if ap.enabled.value {
            ap.servo.command.set(command);
        }
    }
}
const pilot: _ = WindPilot;
