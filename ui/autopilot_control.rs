use std::collections::HashMap;
use std::*;

use pypilot::client::*;
use pypilot::ui::autopilot_control_ui;

struct AutopilotControl {
    sliderlabels: ST0,
    fgGains: ST1,
    host: bool,
    client: bool,
    mode: ST2,
    heading_command: ST3,
    heading: ST4,
    lastcommand: bool,
    recv: HashMap<_, _>,
    rudder: bool,
    apenabled: bool,
    tackstate: bool,
    timer: ST5,
    gains: HashMap<_, _>,
    enumerated: bool,
}

impl AutopilotControl {
    const ID_MESSAGES: _ = 1000;
    fn __init__(&self) {
        super(AutopilotControl, self).__init__(None);
        self.sliderlabels = vec![-120, -40, -10, -5, 0, 5, 10, 40, 120];
        self.fgGains = self.swGains.GetSizer();
        self.host = false;
        if sys.argv.len() > 1 {
            self.host = sys.argv[1];
        }
        self.client = false;
        self.mode = "compass";
        self.heading_command = 0;
        self.heading = 0;
        self.lastcommand = false;
        self.recv = HashMap::new();
        self.rudder = false;
        self.apenabled = false;
        self.tackstate = false;
        self.timer = wx.Timer(self, self.ID_MESSAGES);
        self.timer.Start(100);
        self.Bind(wx.EVT_TIMER, self.receive_messages, self.ID_MESSAGES);
        self.init();
    }
    fn init(&self) {
        self.stStatus.SetLabel("No Connection");
        self.client = pypilotClient(self.host);
        self.client.connect(true);
        self.gains = HashMap::new();
        self.enumerated = false;
        let watchlist = vec![
            "ap.enabled",
            "ap.mode",
            "ap.heading_command",
            "ap.tack.state",
            "ap.tack.timeout",
            "ap.tack.direction",
            "ap.heading",
            "ap.pilot",
            "gps.source",
            "wind.source",
            "servo.controller",
            "servo.engaged",
            "servo.flags",
            "rudder.angle",
        ];
        for name in watchlist {
            self.client.watch(name);
        }
    }
    fn servo_command<T0>(&self, command: T0) {
        if self.lastcommand != command || command != 0 {
            self.lastcommand = command;
            self.client.set("servo.command", command);
        }
    }
    fn send_gain<T0, T1>(&self, name: T0, gain: T1) {
        let slidervalue =
            (((gain["slider"].GetValue() / 1000.0) * (gain["max"] - gain["min"])) + gain["min"]);
        self.client.set(name, slidervalue);
    }
    fn set_mode_color(&self) {
        let modecolors = [
            ("compass", wx.GREEN),
            ("gps", wx.YELLOW),
            ("wind", wx.BLUE),
            ("true wind", wx.CYAN),
        ]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
        if self.tbAP.GetValue() && modecolors.iter().any(|&x| x == self.mode) {
            color = modecolors[self.mode];
        } else {
            color = wx.RED;
        }
        self.tbAP.SetForegroundColour(color);
    }
    fn enumerate_controls<T0, RT>(&self, value_list: T0) -> RT {
        self.tbAP.SetValue(false);
        self.set_mode_color();
        self.fgGains.Clear(true);
        self.gains = HashMap::new();
        let pilots = HashMap::new();
        for name in value_list {
            let sname = name.split(".");
            if sname.len() > 2 && sname[0] == "ap" && sname[1] == "pilot" {
                pilots[sname[2]] = true;
            }
            if value_list[name].iter().any(|&x| x == "AutopilotGain") {
                let sizer = wx.FlexGridSizer(0, 1, 0, 0);
                sizer.AddGrowableRow(2);
                sizer.SetFlexibleDirection(wx.VERTICAL);
                self.client.watch(name);
                self.client.watch((name + "gain"));
                let lname = name;
                sname = name.split(".");
                if sname.len() > 3 && sname[0] == "ap" && sname[1] == "pilot" {
                    lname = sname[3];
                }
                let stname = wx.StaticText(self.swGains, wx.ID_ANY, lname);
                sizer.Add(stname, 0, wx.ALL, 5);
                let stvalue = wx.StaticText(self.swGains, wx.ID_ANY, "   N/A   ");
                sizer.Add(stvalue, 0, wx.ALL, 5);
                let hsizer = wx.FlexGridSizer(1, 0, 0, 0);
                hsizer.AddGrowableRow(0);
                hsizer.SetFlexibleDirection(wx.VERTICAL);
                let gauge = wx.Gauge(
                    self.swGains,
                    wx.ID_ANY,
                    1000,
                    wx.DefaultPosition,
                    wx.Size(-1, -1),
                    wx.SL_VERTICAL,
                );
                hsizer.Add(gauge, 0, (wx.ALL | wx.EXPAND), 5);
                let slider = wx.Slider(
                    self.swGains,
                    wx.ID_ANY,
                    0,
                    0,
                    1000,
                    wx.DefaultPosition,
                    wx.Size(-1, -1),
                    (wx.SL_VERTICAL | wx.SL_INVERSE),
                );
                hsizer.Add(slider, 0, (wx.ALL | wx.EXPAND), 5);
                sizer.Add(hsizer, 1, wx.EXPAND, 5);
                let (min_val, max_val) = (value_list[name]["min"], value_list[name]["max"]);
                let gain = [
                    ("stname", stname),
                    ("stvalue", stvalue),
                    ("gauge", gauge),
                    ("slider", slider),
                    ("min", min_val),
                    ("max", max_val),
                    ("need_update", false),
                    ("last_change", 0),
                    ("sliderval", 0),
                    ("sizer", sizer),
                ]
                    .iter()
                    .cloned()
                    .collect::<HashMap<_, _>>();
                self.gains[name] = gain;
                fn make_ongain<T0, RT>(gain: T0) -> RT {
                    fn do_gain<T0>(event: T0) {
                        gain["need_update"] = true;
                        gain["last_change"] = time.monotonic();
                    }
                    return do_gain;
                }
                slider.Bind(wx.EVT_SCROLL, make_ongain(gain));
            }
        }
        self.enumerate_gains();
        self.cPilot.Clear();
        for pilot in pilots {
            self.cPilot.Append(pilot);
        }
        self.GetSizer().Fit(self);
        self.SetSize(wx.Size(570, 420));
    }
    fn receive_messages<T0>(&self, event: T0) {
        if !self.enumerated && self.client.connection {
            let value_list = self.client.list_values(10);
            if value_list {
                self.enumerate_controls(value_list);
                self.enumerated = true;
            }
            return;
        }
        let mut command = self.sCommand.GetValue();
        if command != 0 {
            if self.tbAP.GetValue() {
                self.heading_command += self.apply_command(command);
                self.client.set("ap.heading_command", self.heading_command);
                self.sCommand.SetValue(0);
            } else {
                if true {
                    if command > 0 {
                        command -= 1;
                    } else {
                        if command < 0 {
                            command += 1;
                        }
                    }
                } else {
                    if abs(command) < 3 {
                        command = 0;
                    }
                }
                self.sCommand.SetValue(command);
                self.servo_command((-(command) / 100.0));
            }
        }
        for gain_name in self.gains {
            let mut gain = self.gains[gain_name];
            if gain["need_update"] {
                self.send_gain(gain_name, gain);
                gain["need_update"] = false;
            }
            if gain["slider"].GetValue() != gain["sliderval"]
                && (time.monotonic() - gain["last_change"]) > 1
            {
                gain["slider"].SetValue(i32::from(gain["sliderval"]));
            }
        }
        let msgs = self.client.receive();
        for name in msgs {
            let mut value = msgs[name];
            self.recv[name] = true;
            let mut found = false;
            for gain_name in self.gains {
                let mut gain = self.gains[gain_name];
                if name == gain_name {
                    gain["stvalue"].SetLabel(("%.5f" % value));
                    gain["sliderval"] =
                        (((value - gain["min"]) * 1000) / (gain["max"] - gain["min"]));
                    found = true;
                } else {
                    if name == (gain_name + "gain") {
                        let v = (abs(value) * 1000.0);
                        if v < gain["gauge"].GetRange() {
                            gain["gauge"].SetValue(i32::from(v));
                            if value > 0 {
                                gain["gauge"].SetBackgroundColour(wx.RED);
                            } else {
                                if value < 0 {
                                    gain["gauge"].SetBackgroundColour(wx.GREEN);
                                } else {
                                    gain["gauge"].SetBackgroundColour(wx.LIGHT_GREY);
                                }
                            }
                        } else {
                            gain["gauge"].SetValue(0);
                            gain["gauge"].SetBackgroundColour(wx.BLUE);
                        }
                        found = true;
                    }
                }
            }
            if found {
                /*pass*/
            } else {
                if name == "ap.enabled" {
                    self.tbAP.SetValue(i32::from(value));
                    self.set_mode_color();
                    self.apenabled = value;
                    self.bCenter.Show(!self.apenabled && self.rudder);
                } else {
                    if name == "rudder.angle" {
                        let try_dummy = {
                            //unsupported
                            value = round(value, 1);
                        };
                        let except!() = { //unsupported
                            /*pass*/
                        };
                        self.rudder = value;
                        if !!self.apenabled && self.rudder == self.bCenter.IsShown() {
                            self.bCenter.Show(!self.bCenter.IsShown());
                        }
                        self.stRudder.SetLabel(String::from(value));
                    } else {
                        if name == "ap.mode" {
                            let rb = [
                                ("compass", self.rbCompass),
                                ("gps", self.rbGPS),
                                ("wind", self.rbWind),
                                ("true wind", self.rbTrueWind),
                            ]
                                .iter()
                                .cloned()
                                .collect::<HashMap<_, _>>();
                            rb[value].SetValue(true);
                            self.mode = value;
                            self.set_mode_color();
                        } else {
                            if name == "ap.heading_command" {
                                self.stHeadingCommand.SetLabel(("%.1f" % value));
                                if command == 0 {
                                    self.heading_command = value;
                                }
                            } else {
                                if name == "ap.pilot" {
                                    self.cPilot.SetStringSelection(value);
                                    self.enumerate_gains();
                                } else {
                                    if name == "gps.source" {
                                        self.rbGPS.Enable(value != "none");
                                        self.rbTrueWind
                                            .Enable(value != "none" && self.rbWind.IsEnabled());
                                    } else {
                                        if name == "wind.source" {
                                            self.rbWind.Enable(value != "none");
                                            self.rbTrueWind
                                                .Enable(value != "none" && self.rbGPS.IsEnabled());
                                        } else {
                                            if name == "ap.heading" {
                                                self.stHeading.SetLabel(("%.1f" % value));
                                                self.heading = value;
                                            } else {
                                                if name == "ap.tack.state" {
                                                    self.stTackState.SetLabel(value);
                                                    self.bTack.SetLabel(if value == "none" {
                                                        "Tack"
                                                    } else {
                                                        "Cancel"
                                                    });
                                                    self.tackstate = value;
                                                } else {
                                                    if name == "ap.tack.timeout" {
                                                        if self.tackstate == "waiting" {
                                                            self.stTackState
                                                                .SetLabel(String::from(value));
                                                        }
                                                    } else {
                                                        if name == "ap.tack.direction" {
                                                            self.cTackDirection
                                                                .SetSelection(value == "starboard");
                                                        } else {
                                                            if name == "servo.engaged" {
                                                                self.stEngaged.SetLabel(if value {
                                                                    "Engaged"
                                                                } else {
                                                                    "Disengaged"
                                                                });
                                                            } else {
                                                                if name == "servo.flags" {
                                                                    self.stStatus.SetLabel(value);
                                                                } else {
                                                                    if name == "servo.controller" {
                                                                        self.stController
                                                                            .SetLabel(value);
                                                                    } else {
                                                                        if name == "servo.current" {
                                                                            /*pass*/
                                                                        } else {
                                                                            if name.iter().any(
                                                                                |&x| {
                                                                                    x == "ap.pilot."
                                                                                },
                                                                            ) {
                                                                                /*pass*/
                                                                            } else {
                                                                                println!("{:?} ",(_("warning: unhandled message") + (" \"%s\"" % name)));
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn onAP<T0>(&self, event: T0) {
        if self.tbAP.GetValue() {
            self.client.set("ap.heading_command", self.heading);
            self.client.set("ap.enabled", true);
        } else {
            self.client.set("ap.enabled", false);
        }
    }
    fn onMode<T0>(&self, event: T0) {
        if self.rbGPS.GetValue() {
            let mut mode = "gps";
        } else {
            if self.rbWind.GetValue() {
                mode = "wind";
            } else {
                if self.rbTrueWind.GetValue() {
                    mode = "true wind";
                } else {
                    mode = "compass";
                }
            }
        }
        self.client.set("ap.mode", mode);
    }
    fn onPilot<T0>(&self, event: T0) {
        self.client
            .set("ap.pilot", self.cPilot.GetStringSelection());
    }
    fn onTack<T0>(&self, event: T0) {
        if self.bTack.GetLabel() == "Tack" {
            self.tackstate = "begin";
        } else {
            self.tackstate = "none";
        }
        self.client.set("ap.tack.state", self.tackstate);
    }
    fn onTackDirection<T0>(&self, event: T0) {
        self.client.set(
            "ap.tack.direction",
            if self.cTackDirection.GetSelection() {
                "starboard"
            } else {
                "port"
            },
        );
    }
    fn onPaintControlSlider<T0>(&self, event: T0) {
        return;
        if wx.version().iter().any(|&x| x == "gtk3") {
            return;
        }
        let dc = wx.PaintDC(self.sCommand);
        let s = self.sCommand.GetSize();
        dc.SetPen(wx.Pen(wx.BLACK));
        dc.SetBrush(wx.TRANSPARENT_BRUSH);
        let y = 10;
        let mut x = 0;
        for l in self.sliderlabels {
            let t = String::from(abs(l));
            let mut tx = x;
            if l > 0 {
                tx -= dc.GetTextExtent(t)[0];
            }
            dc.DrawText(t, tx, y);
            dc.DrawLine(x, 0, x, s.y);
            x += (s.x / (self.sliderlabels.len() - 1));
        }
    }
    fn enumerate_gains(&self) {
        while !self.fgGains.IsEmpty() {
            self.fgGains.Detach(0);
        }
        let pilot = self.cPilot.GetStringSelection();
        for name in self.gains {
            if name.iter().any(|&x| x == pilot) || !name.iter().any(|&x| x == "ap.pilot.") {
                self.gains[name]["sizer"].ShowItems(true);
                self.fgGains.Add(self.gains[name]["sizer"], 1, wx.EXPAND, 5);
            } else {
                self.gains[name]["sizer"].ShowItems(false);
            }
        }
        let s = self.GetSize();
        self.Fit();
        self.SetSize(s);
    }
    fn apply_command<T0, RT>(&self, command: T0) -> RT {
        let r = ((self.sCommand.GetMax() - self.sCommand.GetMin()) + 1.0);
        let p = (((self.sliderlabels.len() - 1) * (command - self.sCommand.GetMin())) / r);
        let l0 = self.sliderlabels[i32::from(p)];
        let l1 = self.sliderlabels[(i32::from(p) + 1)];
        let v = (((p - i32::from(p)) * (l1 - l0)) + l0);
        return v;
    }
    fn onCommand<T0>(&self, event: T0) {
        if wx.GetMouseState().LeftIsDown() {
            let x = self.sCommand.ScreenToClient(wx.GetMousePosition()).x;
            let val = (self.sCommand.GetMin()
                + (((self.sCommand.GetMax() - self.sCommand.GetMin()) * x)
                / self.sCommand.GetSize().x));
            self.sCommand.SetValue(val);
        }
    }
    fn onCenter<T0>(&self, event: T0) {
        self.client.set("servo.position_command", 0);
    }
    fn onScope<T0>(&self, event: T0) {
        subprocess.Popen((vec!["pypilot_scope"] + sys.argv[1..]));
    }
    fn onClient<T0>(&self, event: T0) {
        subprocess.Popen((vec!["pypilot_client_wx"] + sys.argv[1..]));
    }
    fn onCalibration<T0>(&self, event: T0) {
        subprocess.Popen((vec!["pypilot_calibration"] + sys.argv[1..]));
    }
    fn onClose<T0>(&self, event: T0) {
        self.Close();
    }
}

fn main() {
    let app = wx.App();
    AutopilotControl().Show();
    app.MainLoop();
}

fn main() {
    main();
}
