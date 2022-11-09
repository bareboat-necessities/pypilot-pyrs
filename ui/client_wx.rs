use std::collections::HashMap;
use std::*;

use pypilot::client::pypilotClient;
fn round3<T0, RT>(value: T0) -> RT {
    if type_(value) == type_(vec![]) {
        return value.iter().map(round3).collect::<Vec<_>>();
    } else {
        if type_(value) == type_(HashMap::new()) {
            let ret = HashMap::new();
            for each in value {
                ret[round3(each)] = round3(value[each]);
            }
            return ret;
        } else {
            if type_(value) == type_(1.0) {
                return round(value, 3);
            }
        }
    }
    return value;
}
struct MainFrame {
    client: ST0,
    connected: bool,
    scrolledWindow: ST1,
    bRefresh: ST2,
    bScope: ST3,
    bClose: ST4,
    timer: ST5,
    values: HashMap<_, _>,
    controls: HashMap<_, _>,
    sliderrange: HashMap<_, _>,
}

impl MainFrame {
    fn __init__(&self) {
        wx.Frame.__init__(self, None, "pypilot client", (1000, 600));
        let mut host = "";
        if sys.argv.len() > 1 {
            host = sys.argv[1];
        }
        self.client = pypilotClient(host);
        self.connected = false;
        let ssizer = wx.FlexGridSizer(0, 1, 0, 0);
        ssizer.AddGrowableRow(0);
        ssizer.AddGrowableCol(0);
        ssizer.SetFlexibleDirection(wx.BOTH);
        ssizer.SetNonFlexibleGrowMode(wx.FLEX_GROWMODE_SPECIFIED);
        self.scrolledWindow = wx.ScrolledWindow(
            self,
            wx.ID_ANY,
            wx.DefaultPosition,
            wx.DefaultSize,
            (wx.HSCROLL | wx.VSCROLL),
        );
        self.scrolledWindow.SetScrollRate(5, 5);
        self.Refresh(None);
        ssizer.Add(self.scrolledWindow, 1, (wx.EXPAND | wx.ALL), 5);
        let bsizer = wx.FlexGridSizer(1, 0, 0, 0);
        self.bRefresh = wx.Button(self, wx.ID_ANY, _("Refresh"));
        self.bRefresh.Bind(wx.EVT_BUTTON, self.Refresh);
        bsizer.Add(self.bRefresh);
        self.bScope = wx.Button(self, wx.ID_ANY, _("Scope"));
        self.bScope.Bind(wx.EVT_BUTTON, |event| {
            subprocess.Popen(
                (vec![
                    "python",
                    ((os.path.abspath(os.path.dirname(__file__)) + "/") + "scope_wx.py"),
                ] + sys.argv[1..]),
            )
        });
        bsizer.Add(self.bScope);
        self.bClose = wx.Button(self, wx.ID_ANY, "Close");
        self.bClose.Bind(wx.EVT_BUTTON, exit);
        bsizer.Add(self.bClose);
        ssizer.Add(bsizer, 1, wx.EXPAND);
        self.SetSizer(ssizer);
        self.Layout();
        self.timer = wx.Timer(self, wx.ID_ANY);
        self.timer.Start(500);
        self.Bind(wx.EVT_TIMER, self.receive_messages, wx.ID_ANY);
    }
    fn layout_widgets<T0>(&self, value_list: T0) {
        let sizer = self.scrolledWindow.GetSizer();
        if !sizer {
            sizer = wx.FlexGridSizer(0, 3, 0, 0);
            sizer.AddGrowableCol(2);
            sizer.SetFlexibleDirection(wx.BOTH);
            sizer.SetNonFlexibleGrowMode(wx.FLEX_GROWMODE_SPECIFIED);
        }
        for name in sorted(value_list) {
            let t = value_list[name]["type"];
            let watch = true;
            if t == "SensorValue" {
                watch = 10;
            }
            self.client.watch(name, watch);
        }
        for name in sorted(value_list) {
            if self.values.iter().any(|&x| x == name) {
                continue;
            }
            sizer.Add(
                wx.StaticText(self.scrolledWindow, wx.ID_ANY, name),
                0,
                wx.ALL,
                5,
            );
            self.values[name] = wx.StaticText(self.scrolledWindow, wx.ID_ANY);
            sizer.Add(self.values[name], 0, wx.ALL, 5);
            let t = value_list[name]["type"];
            if t == "Property" {
                let tb = wx.TextCtrl(self.scrolledWindow, wx.ID_ANY);
                sizer.Add(tb);
                self.controls[name] = tb;
            } else {
                if t == "BooleanProperty" {
                    fn proc() {
                        let cb = wx.CheckBox(self.scrolledWindow, wx.ID_ANY, "");
                        sizer.Add(cb, 0, wx.EXPAND);
                        self.controls[name] = cb;
                        let cbname = name;
                        fn oncheck<T0>(event: T0) {
                            self.client.set(cbname, cb.GetValue());
                        }
                        cb.Bind(wx.EVT_CHECKBOX, oncheck);
                    }
                    proc();
                } else {
                    if t == "RangeProperty" || t == "RangeSetting" {
                        let useSlider = true;
                        fn proc() {
                            let r = (value_list[name]["min"], value_list[name]["max"]);
                            if useSlider {
                                s = wx.Slider(self.scrolledWindow);
                                s.SetRange(0, 1000);
                            } else {
                                s = wx.SpinCtrlDouble(self.scrolledWindow);
                                s.SetRange(r[0], r[1]);
                                s.SetIncrement(1.iter().min().unwrap());
                                s.SetDigits(((-math.log(s.GetIncrement()) / math.log(10)) + 1));
                            }
                            sizer.Add(s, 0, wx.EXPAND);
                            self.controls[name] = s;
                            let sname = name;
                            fn onspin<T0>(event: T0) {
                                if useSlider {
                                    let v = (((s.GetValue() / 1000.0) * (r[1] - r[0])) + r[0]);
                                    self.client.set(sname, v);
                                } else {
                                    self.client.set(sname, s.GetValue());
                                }
                            }
                            if useSlider {
                                s.Bind(wx.EVT_SLIDER, onspin);
                                self.sliderrange[name] = r;
                            } else {
                                s.Bind(wx.EVT_SPINCTRLDOUBLE, onspin);
                            }
                        }
                        proc();
                    } else {
                        if t == "EnumProperty" {
                            fn proc() {
                                let c = wx.Choice(self.scrolledWindow, wx.ID_ANY);
                                for choice in value_list[name]["choices"] {
                                    c.Append(String::from(choice));
                                }
                                sizer.Add(c, 0, wx.EXPAND);
                                self.controls[name] = c;
                                let cname = name;
                                fn onchoice<T0>(event: T0) {
                                    self.client.set(cname, String::from(c.GetStringSelection()));
                                }
                                c.Bind(wx.EVT_CHOICE, onchoice);
                            }
                            proc();
                        } else {
                            if t == "ResettableValue" {
                                fn proc() {
                                    let b = wx.Button(self.scrolledWindow, wx.ID_ANY, _("Reset"));
                                    sizer.Add(b, 0, wx.EXPAND);
                                    let bname = name;
                                    fn onclick<T0>(event: T0) {
                                        self.client.set(bname, 0);
                                    }
                                    b.Bind(wx.EVT_BUTTON, onclick);
                                }
                                proc();
                            } else {
                                sizer.Add(wx.StaticText(self.scrolledWindow, wx.ID_ANY, ""));
                            }
                        }
                    }
                }
            }
        }
        self.scrolledWindow.SetSizer(sizer);
        self.scrolledWindow.Layout();
        sizer.Fit(self.scrolledWindow);
    }
    fn Refresh<T0>(&self, event: T0) {
        if self.client.connection {
            self.client.disconnect();
        }
        let sizer = self.scrolledWindow.GetSizer();
        if sizer {
            sizer.Clear(true);
        }
        self.values = HashMap::new();
        self.controls = HashMap::new();
        self.sliderrange = HashMap::new();
    }
    fn receive_messages<T0>(&self, event: T0) {
        if self.client.connection != self.connected {
            self.connected = self.client.connection;
            if self.connected {
                self.SetTitle(("pypilot client - " + _("Connected")));
            } else {
                self.SetTitle(("pypilot client - " + _("Disconnected")));
            }
        }
        let value_list = self.client.list_values();
        if value_list {
            self.layout_widgets(value_list);
            let size = self.GetSize();
            self.Fit();
            self.SetSize(size);
        }
        while true {
            let result = self.client.receive();
            if !result {
                break;
            }
            for name in result {
                let value = round3(result[name]);
                let mut strvalue = String::from(value);
                if strvalue.len() > 50 {
                    strvalue = (strvalue[..47] + "...");
                }
                self.values[name].SetLabel(strvalue);
                if self.controls.iter().any(|&x| x == name) {
                    let try_dummy = {
                        //unsupported
                        let t = String::from(type_(self.controls[name]));
                        if t == "<class 'wx._controls.Choice'>" || t == "<class 'wx._core.Choice'>"
                        {
                            if !self.controls[name].SetStringSelection(String::from(value)) {
                                println!("{:?} ", _("warning, invalid choice value specified"));
                            }
                        } else {
                            if t == "<class 'wx._controls.Slider'>"
                                || t == "<class 'wx._core.Slider'>"
                            {
                                let r = self.sliderrange[name];
                                self.controls[name].SetValue(i32::from(
                                    ((float((value - r[0])) / (r[1] - r[0])) * 1000),
                                ));
                            } else {
                                self.controls[name].SetValue(value);
                            }
                        }
                    };
                    let except!() = {
                        //unsupported
                        self.controls[name].SetValue(String::from(value));
                    };
                }
            }
        }
    }
}
fn main() {
    let app = wx.App();
    MainFrame().Show();
    app.MainLoop();
}
fn main() {
    main();
}
