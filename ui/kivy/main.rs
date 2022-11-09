use std::*;
use std::collections::HashMap;

const __version__: _ = "0.0.1";
"
Autopilot Control
============

pypilot kivy control app
";
use kivy::app::{App};
use kivy::uix::tabbedpanel::{TabbedPanel};
use kivy::uix::floatlayout::{FloatLayout};
use kivy::uix::gridlayout::{GridLayout};
use kivy::uix::spinner::{Spinner};
use kivy::uix::widget::{Widget};
use kivy::uix::image::{Image};
use kivy::animation::{Animation};
use kivy::clock::{Clock};
use client::{pypilotClient};

struct AutopilotControl {}

impl AutopilotControl {
    /*pass*/
}

struct AutopilotControlApp {
    client: bool,
    enabled: bool,
    mode: ST0,
    heading_command: ST1,
    heading: ST2,
    _anim: Option<_>,
    texture: ST3,
    control: ST4,
}

impl AutopilotControlApp {
    fn build<RT>(&self) -> RT {
        self.client = false;
        self.connect(3);
        Clock::schedule_interval(self.connect, 3);
        Clock::schedule_interval(self.update, 0.1);
        self.enabled = false;
        self.mode = "compass";
        self.heading_command = 0;
        self.heading = 0;
        self._anim = None;
        self.texture = Image("compass.png").texture;
        self.control = AutopilotControl();
        return self.control;
    }
    fn connect<T0>(&self, dt: T0) {
        if self.client {
            return;
        }
        let watchlist = vec!["ap.enabled", "ap.mode", "ap.heading", "ap.heading_command"];
        fn on_con<T0>(client: T0) {
            for name in watchlist {
                client.watch(name);
            }
        }
        let try_dummy = { //unsupported
            self.client = pypilotClient(on_con, true);
            /*pass*/
        };
        let except!() = { //unsupported
            return;
        };
    }
    fn update<T0>(&self, dt: T0) {
        if !self.client {
            return;
        }
        let result = self.client.receive();
        for msg in result {
            let value = result[msg]["value"];
            if msg == "ap.enabled" || msg == "ap.mode" {
                if msg == "ap.enabled" {
                    self.enabled = value;
                } else {
                    self.mode = value;
                }
                let mut color = "ff0000";
                if self.enabled {
                    let colors = [("compass", "00ff00"), ("gps", "0000ff"), ("wind", "ffff00")].iter().cloned().collect::<HashMap<_, _>>();
                    color = colors[self.mode];
                }
                self.control.ap.text = (("[color=" + color) + "]AP");
            } else {
                if msg == "ap.heading" {
                    self.control.heading_label.text = String::from(value);
                    self.control.compass.heading = value;
                } else {
                    if msg == "ap.heading_command" {
                        self.control.heading_command_label.text = String::from(value);
                        self.heading_command = value;
                    }
                }
            }
        }
    }
    fn onAP(&self) {
        if !self.client {
            return;
        }
        self.client.set("servo.raw_command", false);
        if self.enabled {
            self.client.set("ap.heading_command", self.heading);
            self.client.set("ap.enabled", true);
        } else {
            self.client.set("servo.command", 0);
            self.client.set("ap.enabled", false);
        }
    }
}

fn main() {
    AutopilotControlApp().run();
}