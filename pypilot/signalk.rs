use std::*;
use std::collections::HashMap;

use nonblockingpipe::{NonBlockingPipe};
use client::{pypilotClient};
use values::{Property, RangeProperty};
use sensors::{source_priority};

const signalk_priority: _ = source_priority["signalk"];
const radians: _ = (3.141592653589793 / 180);
const meters_s: _ = 0.5144456333854638;
const signalk_table: _ = [("wind", [(("environment.wind.speedApparent", meters_s), "speed"), (("environment.wind.angleApparent", radians), "direction")].iter().cloned().collect::<HashMap<_, _>>()), ("truewind", [(("environment.wind.speedTrue", meters_s), "speed"), (("environment.wind.angleTrue", radians), "direction")].iter().cloned().collect::<HashMap<_, _>>()), ("gps", [(("navigation.courseOverGroundTrue", radians), "track"), (("navigation.speedOverGround", meters_s), "speed"), (("navigation.position", 1), [("latitude", "lat"), ("longitude", "lon")].iter().cloned().collect::<HashMap<_, _>>())].iter().cloned().collect::<HashMap<_, _>>()), ("rudder", [(("steering.rudderAngle", radians), "angle")].iter().cloned().collect::<HashMap<_, _>>()), ("apb", [(("steering.autopilot.target.headingTrue", radians), "track")].iter().cloned().collect::<HashMap<_, _>>()), ("imu", [(("navigation.headingMagnetic", radians), "heading_lowpass"), (("navigation.attitude", radians), [("pitch", "pitch"), ("roll", "roll"), ("yaw", "heading_lowpass")].iter().cloned().collect::<HashMap<_, _>>()), (("navigation.rateOfTurn", radians), "headingrate_lowpass")].iter().cloned().collect::<HashMap<_, _>>()), ("water", [(("navigation.speedThroughWater", meters_s), "speed"), (("navigation.leewayAngle", radians), "leeway")].iter().cloned().collect::<HashMap<_, _>>())].iter().cloned().collect::<HashMap<_, _>>();
const token_path: _ = (os.getenv("HOME") + "/.pypilot/signalk-token");

fn debug() {
    /*pass*/
}

struct signalk {
    sensors: ST0,
    client: ST1,
    multiprocessing: bool,
    initialized: bool,
    missingzeroconfwarned: bool,
    signalk_access_url: bool,
    last_access_request_time: ST2,
    process: ST3,
    token: ST4,
    last_values: HashMap<_, _>,
    last_sources: HashMap<_, _>,
    signalk_last_msg_time: HashMap<_, _>,
    gps_filtered_output: bool,
    last_values_keys: HashMap<_, _>,
    period: ST5,
    last_period: bool,
    uid: ST6,
    signalk_host_port: bool,
    signalk_ws_url: bool,
    ws: bool,
    signalk: ST7,
    name_type: bool,
    subscribed: HashMap<_, _>,
    subscriptions: Vec<_>,
    signalk_values: HashMap<_, _>,
    keep_token: bool,
}

impl signalk {
    fn __init__<T0>(&self, sensors: T0) {
        self.sensors = sensors;
        if !sensors {
            self.client = pypilotClient();
            self.multiprocessing = false;
        } else {
            let server = sensors.client.server;
            self.multiprocessing = server.multiprocessing;
            self.client = pypilotClient(server);
        }
        self.initialized = false;
        self.missingzeroconfwarned = false;
        self.signalk_access_url = false;
        self.last_access_request_time = 0;
        let (self.sensors_pipe, self.sensors_pipe_out) = NonBlockingPipe("signalk pipe", self.multiprocessing);
        if self.multiprocessing {
            use multiprocessing;
            self.process = multiprocessing.Process(self.process, true);
            self.process.start();
        } else {
            self.process = false;
        }
    }
    fn setup(&self) {
        let try_dummy = { //unsupported
            let f = open(token_path);
            self.token = f.read();
            println!("{:?} {:?} ",("signalk" + _("read token")), self.token);
            f.close();
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ",("signalk " + _("failed to read token")), token_path);
            self.token = false;
        };
        let try_dummy = {
            //unsupported
            use zeroconf::{ServiceBrowser, ServiceStateChange, Zeroconf};
        };
        let except!(Exception) = { //unsupported
            if !self.missingzeroconfwarned {
                println!("{:?} ",((("signalk: " + _("failed to")) + " import zeroconf, ") + _("autodetection not possible")));
                println!("{:?} ",(((_("try") + " pip3 install zeroconf") + _("or")) + " apt install python3-zeroconf"));
                self.missingzeroconfwarned = true;
            }
            time.sleep(20);
            return;
        };
        self.last_values = HashMap::new();
        self.last_sources = HashMap::new();
        self.signalk_last_msg_time = HashMap::new();
        self.gps_filtered_output = false;
        self.last_values_keys = HashMap::new();
        for sensor in signalk_table {
            for (signalk_path_conversion, pypilot_path) in signalk_table[sensor].items() {
                let (signalk_path, signalk_conversion) = signalk_path_conversion;
                if type_(pypilot_path) == type_(HashMap::new()) {
                    self.last_values_keys[signalk_path] = HashMap::new();
                }
            }
        }
        self.period = self.client.register(RangeProperty("signalk.period", 0.5, 0.1, 2, true));
        self.last_period = false;
        self.uid = self.client.register(Property("signalk.uid", "pypilot", true));
        self.signalk_host_port = false;
        self.signalk_ws_url = false;
        self.ws = false;
        struct Listener {
            signalk: ST0,
            name_type: bool,
        }

        impl Listener {
            fn __init__<T0>(&self, signalk: T0) {
                self.signalk = signalk;
                self.name_type = false;
            }
            fn remove_service<T0, T1, T2>(&self, zeroconf: T0, type: T1, name: T2) {
                println!("{:?} {:?} {:?} ",("signalk zeroconf " + _("service removed")), name, type_);
                if self.name_type == (name, type_) {
                    self.signalk.signalk_host_port = false;
                    self.signalk.disconnect_signalk();
                    println!("{:?} ",("signalk " + _("server lost")));
                }
            }
            fn update_service<T0, T1, T2>(&self, zeroconf: T0, type: T1, name: T2) {
                self.add_service(zeroconf, type_, name);
            }
            fn add_service<T0, T1, T2>(&self, zeroconf: T0, type: T1, name: T2) {
                println!("{:?} {:?} {:?} ",("signalk zeroconf " + _("service add")), name, type_);
                self.name_type = (name, type_);
                let info = zeroconf.get_service_info(type_, name);
                if !info {
                    return;
                }
                let properties = HashMap::new();
                for (name, value) in info.properties.items() {
                    let try_dummy = { //unsupported
                        properties[name.decode()] = value.decode();
                    };
                    let except!(Exception) = { //unsupported
                        println!("{:?} {:?} {:?} {:?} ", "signalk zeroconf exception", e, name, value);
                    };
                }
                if properties.iter().any(|&x| x == "swname") && properties["swname"] == "signalk-server" {
                    let try_dummy = { //unsupported
                        let mut host_port = ((socket.inet_ntoa(info.addresses[0]) + ":") + String::from(info.port));
                    };
                    let except!(Exception) = { //unsupported
                        host_port = ((socket.inet_ntoa(info.address) + ":") + String::from(info.port));
                    };
                    self.signalk.signalk_host_port = host_port;
                    println!("{:?} {:?} ",("signalk " + _("server found")), host_port);
                }
            }
        }
        let zeroconf = Zeroconf();
        let listener = Listener(self);
        let browser = ServiceBrowser(zeroconf, "_http._tcp.local.", listener);
        self.initialized = true;
    }
    fn probe_signalk(&self) {
        println!("{:?} {:?} ",(("signalk " + _("probe")) + "..."), self.signalk_host_port);
        let try_dummy = { //unsupported
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ",(("signalk " + _("could not")) + " import requests"), e);
            println!("{:?} ",(((_("try") + " 'sudo apt install python3-requests' ") + _("or")) + " 'pip3 install requests'"));
            time.sleep(50);
            return;
        };
        let try_dummy = { //unsupported
            let r = requests.get((("http://" + self.signalk_host_port) + "/signalk"));
            let contents = pyjson.loads(r.content);
            self.signalk_ws_url = (contents["endpoints"]["v1"]["signalk-ws"] + "?subscribe=none");
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ",_("failed to retrieve/parse data from"), self.signalk_host_port, e);
            time.sleep(5);
            self.signalk_host_port = false;
            return;
        };
        println!("{:?} {:?} ",("signalk " + _("found")), self.signalk_ws_url);
    }
    fn request_access<RT>(&self) -> RT {
        if self.signalk_access_url {
            let dt = (time.monotonic() - self.last_access_request_time);
            if dt < 10 {
                return;
            }
            self.last_access_request_time = time.monotonic();
            let try_dummy = { //unsupported
                let r = requests.get(self.signalk_access_url);
                let contents = pyjson.loads(r.content);
                println!("{:?} {:?} {:?} ",("signalk " + _("see if token is ready")), self.signalk_access_url, contents);
                if contents["state"] == "COMPLETED" {
                    if contents.iter().any(|&x| x == "accessRequest") {
                        let access = contents["accessRequest"];
                        if access["permission"] == "APPROVED" {
                            self.token = access["token"];
                            println!("{:?} {:?} ",("signalk " + _("received token")), self.token);
                            let try_dummy = { //unsupported
                                let f = open(token_path, "w");
                                f.write(self.token);
                                f.close();
                            };
                            let except!(Exception) = { //unsupported
                                println!("{:?} {:?} ",("signalk " + _("failed to store token")), token_path);
                            };
                        }
                    }
                    self.signalk_access_url = false;
                }
            };
            let except!(Exception) = { //unsupported
                println!("{:?} {:?} ",("signalk " + _("error requesting access")), e);
                self.signalk_access_url = false;
            };
            return;
        }
        let try_dummy = {
            //unsupported
            fn random_number_string<T0, RT>(n: T0) -> RT {
                if n == 0 {
                    return "";
                }
                return (String::from(i32::from((random.random() * 10))) + random_number_string((n - 1)));
            }
            if self.uid.value == "pypilot" {
                self.uid.set(("pypilot-" + random_number_string(11)));
            }
            let r = requests.post((("http://" + self.signalk_host_port) + "/signalk/v1/access/requests"), [("clientId", self.uid.value), ("description", "pypilot")].iter().cloned().collect::<HashMap<_, _>>());
            let contents = pyjson.loads(r.content);
            println!("{:?} {:?} ", "signalk post", contents);
            if contents["statusCode"] == 202 || contents["statusCode"] == 400 {
                self.signalk_access_url = (("http://" + self.signalk_host_port) + contents["href"]);
                println!("{:?} {:?} ",("signalk " + _("request access url")), self.signalk_access_url);
            }
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ",("signalk " + _("error requesting access")), e);
            self.signalk_ws_url = false;
        };
    }
    fn connect_signalk(&self) {
        let try_dummy = {
            //unsupported
            use websocket::{create_connection, WebSocketBadStatusException};
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ",("signalk " + _("cannot create connection:")), e);
            println!("{:?} ",(((_("try") + " pip3 install websocket-client ") + _("or")) + " apt install python3-websocket"));
            self.signalk_host_port = false;
            return;
        };
        self.subscribed = HashMap::new();
        for sensor in signalk_table.collect::<Vec<_>>() {
            self.subscribed[sensor] = false;
        }
        self.subscriptions = vec![];
        self.signalk_values = HashMap::new();
        self.keep_token = false;
        let try_dummy = { //unsupported
            self.ws = create_connection(self.signalk_ws_url, [("Authorization", ("JWT " + self.token))].iter().cloned().collect::<HashMap<_, _>>());
            self.ws.settimeout(0);
        };
        let except!(WebSocketBadStatusException) = { //unsupported
            println!("{:?} ",("signalk " + _("bad status, rejecting token")));
            self.token = false;
            self.ws = false;
        };
        let except!(ConnectionRefusedError) = { //unsupported
            println!("{:?} ",("signalk " + _("connection refused")));
            self.signalk_ws_url = false;
            time.sleep(5);
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ",("signalk " + _("failed to connect")), e);
            self.signalk_ws_url = false;
            time.sleep(5);
        };
    }
    fn process(&self) {
        time.sleep(6);
        println!("{:?} {:?} ", "signalk process", os.getpid());
        self.process = false;
        while true {
            time.sleep(0.1);
            self.poll(1);
        }
    }
    fn setup_watches(&self) {
        let mut watches = vec!["imu.heading_lowpass", "imu.roll", "imu.pitch"];
        watches += vec!["gps.filtered.output"];
        for watch in watches {
            self.client.watch(watch, self.period.value);
        }
        self.client.watch("timestamp", (self.period.value / 2));
        for sensor in signalk_table {
            self.client.watch((sensor + ".source"));
        }
    }
    fn poll<T0>(&self, timeout: T0) {
        if self.process {
            let mut msg = self.sensors_pipe_out.recv();
            while msg {
                let (sensor, data) = msg;
                self.sensors.write(sensor, data, "signalk");
                let mut msg = self.sensors_pipe_out.recv();
            }
            return;
        }
        let t0 = time.monotonic();
        if !self.initialized {
            self.setup();
            return;
        }
        self.client.poll(timeout);
        if !self.signalk_host_port {
            return;
        }
        let t1 = time.monotonic();
        if !self.signalk_ws_url {
            self.probe_signalk();
            return;
        }
        let t2 = time.monotonic();
        if !self.token {
            self.request_access();
            return;
        }
        if !self.ws {
            self.connect_signalk();
            if !self.ws {
                return;
            }
            println!("{:?} {:?} ",("signalk " + _("connected to")), self.signalk_ws_url);
            self.setup_watches();
            self.last_period = self.period.value;
            return;
        }
        if self.last_period != self.period.value {
            self.disconnect_signalk();
            return;
        }
        while true {
            let mut msg = self.client.receive_single();
            if !msg {
                break;
            }
            let (name, value) = msg;
            if name == "timestamp" {
                self.send_signalk();
                self.last_values = HashMap::new();
            }
            if name.endswith(".source") {
                for sensor in signalk_table {
                    let source_name = (sensor + ".source");
                    if name == source_name {
                        self.update_sensor_source(sensor, value);
                    }
                }
                self.last_sources[name[..-7]] = value;
            } else {
                if name == "gps.filtered.output" {
                    self.gps_filtered_output = value;
                    self.client.watch("gps.fix", !value);
                    self.client.watch("gps.filtered.fix", value);
                } else {
                    self.last_values[name] = value;
                }
            }
        }
        let t3 = time.monotonic();
        let t4 = time.monotonic();
        while true {
            let try_dummy = { //unsupported
                msg = self.ws.recv();
            };
            let except!(Exception) = { //unsupported
                break;
            };
            if !msg {
                println!("{:?} ", "signalk server closed connection");
                if !self.keep_token {
                    println!("{:?} ", "signalk invalidating token");
                    self.token = false;
                }
                self.disconnect_signalk();
                return;
            }
            let try_dummy = { //unsupported
                self.receive_signalk(msg);
            };
            let except!(Exception) = { //unsupported
                debug("failed to parse signalk", msg, e);
                return;
            };
            self.keep_token = true;
        }
        let t5 = time.monotonic();
        for (sensor, sensor_table) in signalk_table.items() {
            for (source, values) in self.signalk_values.items() {
                let data = HashMap::new();
                for (signalk_path_conversion, pypilot_path) in sensor_table.items() {
                    let (signalk_path, signalk_conversion) = signalk_path_conversion;
                    if values.iter().any(|&x| x == signalk_path) {
                        let try_dummy = { //unsupported
                            if !data.iter().any(|&x| x == "timestamp") && self.signalk_last_msg_time.iter().any(|&x| x == signalk_path) {
                                let ts = time.strptime(self.signalk_last_msg_time[signalk_path], "%Y-%m-%dT%H:%M:%S.%fZ");
                                data["timestamp"] = time.mktime(ts);
                            }
                            let value = values[signalk_path];
                            if type_(pypilot_path) == dict {
                                for (signalk_key, pypilot_key) in pypilot_path.items() {
                                    if !value[signalk_key] == None {
                                        data[pypilot_key] = (value[signalk_key] / signalk_conversion);
                                    }
                                }
                            } else {
                                if !value == None {
                                    data[pypilot_path] = value;
                                    if signalk_conversion != 1 {
                                        data[pypilot_path] /= signalk_conversion;
                                    }
                                }
                            }
                        };
                        let except!(Exception) = { //unsupported
                            println!("{:?} {:?} {:?} ",_("Exception converting signalk->pypilot"), e, self.signalk_values);
                            break;
                        };
                    } else {
                        if signalk_conversion != 1 {
                            break;
                        }
                    }
                }
            }
        }
    }
    fn send_signalk(&self) {
        let mut updates = vec![];
        for sensor in signalk_table {
            if sensor != "imu" && !self.last_sources.iter().any(|&x| x == sensor) || source_priority[self.last_sources[sensor]] >= signalk_priority {
                continue;
            }
            let mut sensork = sensor;
            if sensor == "gps" && self.gps_filtered_output {
                sensork = "gps.filtered";
            }
            for (signalk_path_conversion, pypilot_path) in signalk_table[sensor].items() {
                let (signalk_path, signalk_conversion) = signalk_path_conversion;
                if type_(pypilot_path) == dict {
                    let keys = self.last_values_keys[signalk_path];
                    for (signalk_key, pypilot_key) in pypilot_path.items() {
                        let mut key = ((sensork + ".") + pypilot_key);
                        if sensor == "gps" {
                            let kf = (sensork + ".fix");
                            if self.last_values.get(kf) {
                                keys[key] = self.last_values[kf][pypilot_key];
                            }
                        } else {
                            if self.last_values.iter().any(|&x| x == key) {
                                keys[key] = self.last_values[key];
                            }
                        }
                    }
                    v = HashMap::new();
                    for (signalk_key, pypilot_key) in pypilot_path.items() {
                        let mut key = ((sensork + ".") + pypilot_key);
                        if !keys.iter().any(|&x| x == key) {
                            break;
                        }
                        v[signalk_key] = (keys[key] * signalk_conversion);
                    }
                } else {
                    v = None;
                    if sensor == "gps" {
                        key = (sensork + ".fix");
                        let kv = self.last_values.get(key);
                        if kv && kv.iter().any(|&x| x == pypilot_path) {
                            v = kv[pypilot_path];
                        }
                    } else {
                        key = ((sensor + ".") + pypilot_path);
                        if self.last_values.get(key) {
                            v = self.last_values[key];
                        }
                    }
                    if v != None {
                        v *= signalk_conversion;
                        updates.push([("path", signalk_path), ("value", v)].iter().cloned().collect::<HashMap<_, _>>());
                    }
                }
            }
        }
        if updates {
            let msg = [("updates", vec![[("$source", "pypilot"), ("values", updates)].iter().cloned().collect::<HashMap<_, _>>()])].iter().cloned().collect::<HashMap<_, _>>();
            debug("signalk updates", msg);
            let try_dummy = { //unsupported
                self.ws.send((pyjson.dumps(msg) + "
"));
            };
            let except!(Exception) = { //unsupported
                println!("{:?} {:?} ",("signalk " + _("failed to send updates")), e);
                self.disconnect_signalk();
            };
        }
    }
    fn disconnect_signalk(&self) {
        if self.ws {
            self.ws.close();
        }
        self.ws = false;
        self.client.clear_watches();
    }
    fn receive_signalk<T0>(&self, msg: T0) {
        let try_dummy = { //unsupported
            let data = pyjson.loads(msg);
        };
        let except!() = { //unsupported
            if msg {
                println!("{:?} {:?} ",("signalk " + _("failed to parse msg:")), msg);
            }
            return;
        };
        if data.iter().any(|&x| x == "updates") {
            let updates = data["updates"];
            for update in updates {
                let mut source = "unknown";
                if update.iter().any(|&x| x == "$source") {
                    source = update["$source"];
                } else {
                    if update.iter().any(|&x| x == "source") {
                        if update["source"].iter().any(|&x| x == "talker") {
                            source = update["source"]["talker"];
                        }
                    }
                }
                if update.iter().any(|&x| x == "timestamp") {
                    let timestamp = update["timestamp"];
                }
                if !self.signalk_values.iter().any(|&x| x == source) {
                    self.signalk_values[source] = HashMap::new();
                }
                if update.iter().any(|&x| x == "values") {
                    let mut values = update["values"];
                } else {
                    if update.iter().any(|&x| x == "meta") {
                        values = update["meta"];
                    } else {
                        debug("signalk message update contains no values or meta", update);
                        continue;
                    }
                }
                for value in values {
                    let path = value["path"];
                    if self.signalk_last_msg_time.iter().any(|&x| x == path) {
                        if self.signalk_last_msg_time[path] == timestamp {
                            debug("signalk skip duplicate timestamp", source, path, timestamp);
                            continue;
                        }
                        self.signalk_values[source][path] = value["value"];
                    } else {
                        debug("signalk skip initial message", source, path, timestamp);
                    }
                    self.signalk_last_msg_time[path] = timestamp;
                }
            }
        }
    }
    fn update_sensor_source<T0, T1>(&self, sensor: T0, source: T1) {
        let priority = source_priority[source];
        let mut watch = priority < signalk_priority;
        if watch {
            watch = self.period.value;
        }
        if sensor == "gps" {
            self.client.watch("gps.fix", watch);
        } else {
            for (signalk_path_conversion, pypilot_path) in signalk_table[sensor].items() {
                if type_(pypilot_path) == dict {
                    for (signalk_key, pypilot_key) in pypilot_path.items() {
                        let mut pypilot_path = ((sensor + ".") + pypilot_key);
                        if self.last_values.iter().any(|&x| x == pypilot_path) {
                            self.last_values[pypilot_path].drop();
                        }
                        self.client.watch(pypilot_path, watch);
                    }
                } else {
                    let mut pypilot_path = ((sensor + ".") + pypilot_path);
                    if self.last_values.iter().any(|&x| x == pypilot_path) {
                        self.last_values[pypilot_path].drop();
                    }
                    self.client.watch(pypilot_path, watch);
                }
            }
        }
        let subscribe = priority >= signalk_priority;
        if self.subscribed[sensor] == subscribe {
            return;
        }
        self.subscribed[sensor] = subscribe;
        if !subscribe {
            let mut subscription = [("context", "*"), ("unsubscribe", vec![[("path", "*")].iter().cloned().collect::<HashMap<_, _>>()])].iter().cloned().collect::<HashMap<_, _>>();
            debug("signalk unsubscribe", subscription);
            let try_dummy = { //unsupported
                self.ws.send((pyjson.dumps(subscription) + "
"));
            };
            let except!(Exception) = { //unsupported
                println!("{:?} {:?} ", "signalk failed to send", e);
                self.disconnect_signalk();
                return;
            };
        }
        let signalk_sensor = signalk_table[sensor];
        if subscribe {
            subscriptions = vec![];
            for signalk_path_conversion in signalk_sensor {
                let (signalk_path, signalk_conversion) = signalk_path_conversion;
                if self.signalk_last_msg_time.iter().any(|&x| x == signalk_path) {
                    self.signalk_last_msg_time[signalk_path].drop();
                }
                subscriptions.push([("path", signalk_path), ("minPeriod", (self.period.value * 1000)), ("format", "delta"), ("policy", "instant")].iter().cloned().collect::<HashMap<_, _>>());
            }
            self.subscriptions += subscriptions;
        } else {
            debug("signalk remove subs", signalk_sensor, self.subscriptions);
            subscriptions = vec![];
            for subscription in self.subscriptions {
                for signalk_path_conversion in signalk_sensor {
                    let (signalk_path, signalk_conversion) = signalk_path_conversion;
                    if subscription["path"] == signalk_path {
                        break;
                    }
                }
            }
            self.subscriptions = subscriptions;
            self.signalk_last_msg_time = HashMap::new();
        }
        let mut subscription = [("context", "vessels.self")].iter().cloned().collect::<HashMap<_, _>>();
        subscription["subscribe"] = subscriptions;
        debug("signalk subscribe", subscription);
        let try_dummy = { //unsupported
            self.ws.send((pyjson.dumps(subscription) + "
"));
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ", "signalk failed to send subscription", e);
            self.disconnect_signalk();
        };
    }
}

fn main() {
    let sk = signalk();
    while true {
        sk.poll(1);
    }
}

fn main() {
    main();
}