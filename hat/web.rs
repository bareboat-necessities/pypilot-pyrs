use std::*;
use std::collections::HashMap;

use flask::{Flask, render_template, request, Markup};
use flask_socketio::{SocketIO, Namespace, emit, disconnect};

const app: _ = Flask(__name__);
app.config["SECRET_KEY"] = "secret!";
const socketio: _ = SocketIO(app, None);
const web_port: _ = 33333;
const default_actions: _ = [("auto", vec!["ir030C1000", "ir030C1800", "KEY_POWER", "gpio17", "rf7E1C2950", "rf7E0C2950"]), ("menu", vec!["ir030D1000", "ir030D1800", "KEY_MUTE", "gpio23", "rf7D1C2950", "rf7D0C2950"]), ("port1", vec!["ir03201800", "ir03201000", "KEY_UP", "gpio27", "rf771C2950", "rf770C2950"]), ("starboard1", vec!["ir03211800", "ir03211000", "KEY_DOWN", "gpio22", "rf7B1C2950", "rf7B0C2950"]), ("select", vec!["ir030B1000", "ir030B1800", "KEY_SELECT", "gpio18", "rf6F1C2950", "rf6F0C2950"]), ("port10", vec!["ir03111800", "ir03111000", "KEY_LEFT", "gpio6", "rf3F1C2950", "rf3F0C2950"]), ("starboard10", vec!["ir03101800", "ir03101000", "KEY_RIGHT", "gpio5", "rf5F1C2950", "rf5F0C2950"]), ("tack", vec!["gpio26", "rf7F1C2910", "rf7F0C2910"])].iter().cloned().collect::<HashMap<_, _>>();
let try_dummy = { //unsupported
use flask_babel::{Babel, gettext};
const babel: _ = Babel(app);
const LANGUAGES: _ = os.listdir((os.path.dirname(os.path.abspath(__file__)) + "/translations"));
fn get_locale < RT > () -> RT {
return request::accept_languages.best_match(LANGUAGES);
}
};
let except!(Exception) = { //unsupported
println ! ("{:?} {:?} ", "failed to import flask_babel, translations not possible!!", e);
fn _ < T0, RT > (x: T0) -> RT {
return x;
}
app.jinja_env.globals.update(_);
babel = None;
};
struct WebConfig {
    pipe: ST0,
    config: ST1,
    status: ST2,
    last_key: bool,
}

impl WebConfig {
    fn __init__<T0, T1, T2, RT>(&self, name: T0, pipe: T1, config: T2) -> RT {
        super(Namespace, self).__init__(name);
        socketio.start_background_task(self.background_thread);
        self.pipe = pipe;
        self.config = config;
        self.status = "N/A";
        self.last_key = false;
        let ind = 0;
        let acts = vec!["", ""];
        let names = Markup("[");
        let cols = 1;
        let col = 0;
        acts[ind] += Markup("<table border=0>");
        let i = 0;
        let actions = config["actions"];
        for name in actions {
            if i == 8 {
                acts[ind] += Markup("</tr></table>");
                ind = 1;
                acts[ind] += Markup("<table border=0>");
                col = 0;
            }
            i += 1;
            if col == 0 {
                acts[ind] += Markup("<tr>");
            }
            acts[ind] += Markup((((((("<td><button id=\"action_" + name) + "\">") + name) + "</button></td><td><span id=\"action") + name) + "keys\"></span></td>"));
            if col == (cols - 1) {
                acts[ind] += Markup("</tr>");
                col = 0;
            } else {
                col += 1;
            }
            names += Markup((("\"" + name) + "\", "));
        }
        acts[ind] += Markup("</table>");
        names += Markup("\"\"]");
        let ir = Markup("<input type=\"radio\" id=\"pi_ir\" name=\"ir\"");
        if config["pi.ir"] {
            ir += Markup(" checked");
        }
        ir += Markup(" /> raspberry");
        ir += Markup("<input type=\"radio\" id=\"arduino_ir\" name=\"ir\"");
        if config["arduino.ir"] {
            ir += Markup(" checked");
        }
        ir += Markup(" /> arduino");
        let nmea = Markup("<input type=\"checkbox\" id=\"arduino_nmea_in\"");
        if config["arduino.nmea.in"] {
            nmea += Markup(" checked");
        }
        nmea += Markup("/> Input<input type=\"checkbox\" id=\"arduino_nmea_out\"");
        if config["arduino.nmea.out"] {
            nmea += Markup(" checked");
        }
        nmea += Markup("/> Output<select id=\"arduino_nmea_baud\">");
        for baud in vec![4800, 38400] {
            nmea += Markup(("<option value=" + String::from(baud)));
            if baud == config["arduino.nmea.baud"] {
                nmea += Markup(" selected");
            }
            nmea += Markup(((">" + String::from(baud)) + "</option>"));
        }
        nmea += Markup("</select>");
        let remote = Markup("<input type=\"checkbox\" id=\"remote\"");
        if config["host"] != "localhost" {
            remote += Markup(" checked");
        }
        remote += Markup((("/><input type=\"text\" id=\"host\" value=\"" + config["host"]) + "\">"));
        fn index<RT>() -> RT {
            return render_template("index.html", socketio.async_mode, web_port, acts, names, ir, nmea, remote);
        }
    }
    fn on_ping(&self) {
        emit("pong");
    }
    fn on_keys<T0>(&self, command: T0) {
        let actions = self.config["actions"];
        if command == "clear" {
            for name in actions {
                actions[name] = vec![];
            }
            self.emit_keys();
            return;
        }
        if command == "default" {
            for name in actions {
                actions[name] = vec![];
            }
            for (name, keys) in default_actions.items() {
                actions[name] = keys.copy();
            }
            self.emit_keys();
            return;
        }
        if !self.last_key {
            return;
        }
        for (name, keys) in actions.items() {
            while keys.iter().any(|&x| x == self.last_key) {
                keys.remove(self.last_key);
            }
        }
        actions[command].append(self.last_key);
        self.emit_keys();
    }
    fn on_config<T0>(&self, config: T0) {
        self.pipe.send(config);
    }
    fn emit_keys(&self) {
        let actions = self.config["actions"];
        for (name, keys) in actions.items() {
            let keys = [("name", name), ("keys", keys)].iter().cloned().collect::<HashMap<_, _>>();
            socketio.emit("action_keys", keys);
        }
        self.pipe.send([("actions", actions)].iter().cloned().collect::<HashMap<_, _>>());
    }
    fn on_connect(&self) {
        self.emit_keys();
        println!("{:?} {:?} ", "web client connected", request::sid);
        socketio.emit("status", self.status);
    }
    fn on_disconnect(&self) {
        println!("{:?} {:?} ", "web client disconnected", request::sid);
    }
    fn background_thread(&self) {
        println!("{:?} {:?} ", "web process on port", web_port);
        let mut last_key_time = time.monotonic();
        let x = 0;
        let polls_sent = HashMap::new();
        while true {
            socketio.sleep(0.5);
            if self.last_key {
                let dtc = (time.monotonic() - last_key_time);
                if dtc > 8 {
                    self.last_key = false;
                    socketio.emit("key", "N/A");
                    socketio.emit("action", "");
                }
            }
            if !self.pipe {
                continue;
            }
            while true {
                let msg = self.pipe.recv();
                if !msg {
                    break;
                }
                if msg.iter().any(|&x| x == "key") {
                    self.last_key = msg["key"];
                    last_key_time = time.monotonic();
                }
                for name in msg {
                    socketio.emit(name, String::from(msg[name]));
                }
                if msg.iter().any(|&x| x == "status") {
                    self.status = msg["status"];
                    socketio.emit("status", self.status);
                }
            }
        }
    }
}

fn web_process<T0, T1>(pipe: T0, config: T1) {
    println!("{:?} {:?} ", "web process", os.getpid());
    let path = os.path.dirname(__file__);
    os.chdir(os.path.abspath(path));
    socketio.on_namespace(WebConfig("", pipe, config));
    socketio.run(app, false, "0.0.0.0", web_port);
}

fn main() {
    let config = [("host", "localhost"), ("actions", HashMap::new()), ("pi.ir", true), ("arduino.ir", false), ("arduino.nmea.in", false), ("arduino.nmea.out", false), ("arduino.nmea.baud", 4800), ("lcd", HashMap::new()), ("actions", default_actions.copy())].iter().cloned().collect::<HashMap<_, _>>();
    web_process(None, config);
}