use std::*;
use std::collections::HashMap;

use pypilot::client::{pypilotClient};
"
eye:

64x64x64                     2e29
cnn 4x4x4x32              2e11 2e11-2e13
max poll 16x16x16            2e25
cnn 4x4x4x128             2e13 2e18
max poll 4x4x4               2e21
cnn 4x4x4x512             2e14 2e22
max poll 1x1x1               2e20
dense 2048                   2e19
dense 256                    2e12
dense 16                     


cnn 4x4x4x512                2e15
max poll 4x4x4               2e18
flatten 4x4x4x1024           2e
dense 256                    2e18
dense 64                     2e14
dense 16                     2e10



256x256x64                    67108864
cnn 2x2x4                    16
max poll 128x128x32x4          67108864
cnn 2x2x8                    32
max poll 64x64x16x8            33554432
cnn 2x2x16                   64
max pool 32x32x8x16           16777216
cnn 2x2x32                   128
max poll 16x16x4x32           8388608
cnn 2x2x64                   256
max poll 8x8x2x64             4194304
cnn 2x2x128                  512
max poll 4x4x1x128            2097152
cnn 2x2x256                  1024
max poll 2x2x256              2097152
cnn 2x2x512                  2048
max poll 512                  65536   
dense    128                 65536
dense 8                      1024

flatten 1024
dense 256                     262144
dense 32                      8192
dense 8





can load and select between different models

uses compute nodes to send distributed learning jobs


model predicts future state
inputs - wind speed imu servo camera, future commands
outputs - boat heading, power consumed,  errors for all outputs


predict error of output
measure output error: if output error < error, reduce error slightly, otherwise increase error by a larger factor
predict output error: use smaller loss factor if < error, otherwise use larger loss

optimal trajectory is differential equation




support multiple models

tensorboard for analysis

if logging enabled, hdf5 format to store union of inputs and predictions compressed size per day?
   store in .pypilot with maximum total data size 4gb?   Make sure do not fill disk completely.


new layer type in tensorflow which uses parameters to change weights, otherwise it is same as dense
   can you embed model in model?   Can model do this rather than layer?
   seastate (multi dimensional) -  wind speed, wind direction, water speed (or gps speed), accelerometer/gyro frequency/amplitude??
   light (average intensity) for eye

language to define pypilot models??

models which learn to steer from limited inputs, for example, eye only

models include:
    tensor flow model architecture, knows if online/offline, some layers may only train offline
    filename calculation to store/load weights/bias from file, this file should include what data already trained it to avoid re-training
    function for optimal trajectory, or the function used to predict the output from the model which may be another tensorflow model
    data showing accuracy

visualizations:
    hyperspace of explored sea states in the model
    the realtime/replay plot of intended trajectory, actual trajectory (in playback), and the various predicted trajectories
    for each prediction variable

predictions may include wattage, wind direction, wind speed etc...

main proccess learning pilot:
    executes calculation for current model from inputs
    send all inputs and predictions to ai process

separate ai process includes
    settings for:
          ncpus to use (all idle priority)
          status to indicate if sufficient processing power, if keeping up with all models, only active or not keeping up
          max log size
          
    receive inputs and predictions
    log data to disk for future offline processing
    train active/enabled models

separate program
    to train all models from log data
    support tensorboard from here

";
fn model_filename<T0, RT>(state: T0) -> RT {
    let mut filename = (os.getenv("HOME") + "/.pypilot/model_");
    for (name, value) in state.items() {
        filename += ("_" + String::from(value));
    }
    return filename;
}

struct stopwatch {
    total: ST0,
    starttime: bool,
}

impl stopwatch {
    fn __init__(&self) {
        self.total = 0;
        self.starttime = false;
    }
    fn start(&self) {
        self.starttime = time.monotonic();
    }
    fn stop(&self) {
        self.total += (time.monotonic() - self.starttime);
    }
    fn time<RT>(&self) -> RT {
        if !self.starttime {
            return 0;
        }
        return ((self.total + time.monotonic()) - self.starttime);
    }
}

struct History {
    samples: ST0,
    data: Vec<_>,
}

impl History {
    fn __init__<T0, T1, T2>(&self, conf: T0, state: T1, future: T2) {
        future = if future { self.conf["future"] } else { 0 };
        let dt = ((self.conf["past"] + future) * self.state["imu.rate"]);
        self.samples = i32::from(math.ceil(dt));
        self.data = vec![];
    }
    fn put<T0>(&self, data: T0) {
        if self.full() {
            self.data = self.data[1..];
        }
        self.data.append(data);
    }
    fn clear(&self) {
        self.data = vec![];
    }
    fn full<RT>(&self) -> RT {
        return self.data.len() == self.samples;
    }
}

fn inputs<T0, T1, RT>(history: T0, names: T1) -> RT {
    fn select<T0, T1, RT>(values: T0, names: T1) -> RT {
        let mut data = vec![];
        for name in values {
            if !names.iter().any(|&x| x == name) {
                continue;
            }
            let value = values[name];
            if type_(value) == type_(vec![]) {
                data += value;
            } else {
                data.push(value);
            }
        }
        return data;
    }
    fn flatten<T0, RT>(values: T0) -> RT {
        if type_(values) != type_(vec![]) {
            return vec![float(values)];
        }
        let mut data = vec![];
        for value in values {
            data += flatten(value);
        }
        return data;
    }
    return flatten(history.iter().map(|input| select(input, names)).collect::<Vec<_>>());
}

fn norm_sensor<T0, T1, RT>(name: T0, value: T1) -> RT {
    let conversions = [("imu.accel", 1), ("imu.gyro", 0.1), ("servo.current", 1), ("servo.command", 1), ("ap.heading_error", 0.2), ("imu.headingrate_lowpass", 0.1)].iter().cloned().collect::<HashMap<_, _>>();
    let c = conversions[name];
    fn norm_value<T0, RT>(value: T0) -> RT {
        return math.tanh((c * value));
    }
    if type_(value) == type_(vec![]) {
        return value.iter().map(norm_value).collect::<Vec<_>>();
    }
    return norm_value(value);
}

struct Model {
    history: bool,
}

impl Model {
    fn __init__(&self) {
        self.history = false;
    }
    fn present<RT>(&self) -> RT {
        return (self.conf["state"]["imu.rate"] * self.conf["past"]);
    }
    fn receive<T0, T1>(&self, name: T0, value: T1) {
        if self.conf["sensors"].iter().any(|&x| x == name) && self.enabled {
            self.inputs[name] = norm_sensor(name, value);
        }
    }
}

struct KerasModel {
    host: ST0,
    inputs: HashMap<_, _>,
    conf: ST1,
    state: HashMap<_, _>,
    ap_enabled: bool,
    lasttimestamp: ST2,
    firsttimestamp: bool,
    record_file: bool,
    playback_file: bool,
    load_time: ST3,
    fit_time: ST4,
    total_time: ST5,
    model: ST6,
    history: ST7,
    client: ST8,
}

impl KerasModel {
    fn __init__<T0>(&self, host: T0) {
        super(KerasModel, self).__init__();
        self.host = host;
        let (self.train_x, self.train_y) = (vec![], vec![]);
        self.inputs = HashMap::new();
        self.conf = [("past", 5), ("future", 2), ("sensors", vec!["imu.accel", "imu.headingrate", "servo.current", "servo.command"]), ("actions", vec!["servo.command"]), ("predictions", vec!["ap.heading_error", "imu.headingrate_lowpass"]), ("state", vec!["ap.mode", "imu.rate"])].iter().cloned().collect::<HashMap<_, _>>();
        self.state = HashMap::new();
        self.ap_enabled = false;
        self.lasttimestamp = 0;
        self.firsttimestamp = false;
        self.record_file = false;
        self.playback_file = false;
        self.load_time = stopwatch();
        self.fit_time = stopwatch();
        self.total_time = stopwatch();
        self.total_time.start();
    }
    fn train(&self) {
        if self.history.data.len() != self.history.samples() {
            return;
        }
        let p = self.present();
        let sensors_data = inputs(self.history.data[..p], self.conf["sensors"]);
        let actions_data = inputs(self.history.data[p..], self.conf["actions"]);
        let predictions_data = inputs(self.history.data[p..], self.conf["predictions"]);
        if !self.model {
            let (self.train_x, self.train_y) = (vec![], vec![]);
        }
        let inputs = (sensors_data + actions_data);
        self.train_x.append(inputs);
        self.train_y.append(predictions_data);
        if !self.model {
            self.load_time.start();
            self.build(self.train_x[0].len(), self.train_y[0].len());
            let try_dummy = { //unsupported
                self.model.load_weights((model_filename(self.state) + "model"));
            };
            let except!() = { //unsupported
                println!("{:?} ", "failed to load model, starting from new");
            };
            self.load_time.stop();
        }
        let predict = self.model.predict(inputs);
        let pl = self.conf["predictions"].len();
        let a = (vec![1] * pl);
        let mut accuracy = vec![];
        for i in (0..predict.len()).step_by(pl) {
            for j in (0..pl) {
                let computed = predict[(i + j)];
                let measured = predictions_data[(i + j)];
                let square_error = (computed - measured).pow(2);
                a[j] *= (1 - square_error).iter().max().unwrap();
                let lp = 0.01;
                accuracy = self.conf["accuracy"];
                accuracy[i][j] = ((accuracy[i][j] * (1 - lp)) + (a[j] * lp));
            }
        }
        let pool_size = 6000;
        let l = self.train_x.len();
        if l < pool_size {
            if (l % 100) == 0 {
                sys.stdout.write((("pooling... " + String::from(l)) + "
"));
                sys.stdout.flush();
            }
            return;
        }
        println!("{:?} {:?} {:?} {:?} {:?} ", "fit", self.train_x.len(), self.train_x[0].len(), self.train_y.len(), self.train_y[0].len());
        self.fit_time.start();
        let history = self.model.fit(self.train_x, self.train_y, 8);
        self.fit_time.stop();
        let mse = history.history["mse"];
        println!("{:?} {:?} ", "mse", mse);
        let (self.train_x, self.train_y) = (vec![], vec![]);
    }
    fn build<T0, T1>(&self, input_size: T0, output_size: T1) {
        let conf = self.conf;
        println!("{:?} ", "load_time...");
        println!("{:?} ", "building...");
        let input = tf.keras.layers.Input((input_size), "input_layer");
        let hidden2 = tf.keras.layers.Dense(16, "relu")(input);
        let output = tf.keras.layers.Dense(output_size, "tanh")(hidden2);
        self.model = tf.keras.Model(input, output);
        self.model.compile("adam", "mean_squared_error", vec!["mse"]);
    }
    fn save(&self) {
        let filename = learning.model_filename(self.state);
        let converter = tf.lite.TFLiteConverter.from_keras_model(self.model);
        let tflite_model = converter.convert();
        let try_dummy = { //unsupported
            let mut f = open((filename + "conf"), "w");
            f.write(json.dumps(self.conf));
            f.close();
            f = open((filename + ".tflite_model"), "w");
            f.write(tflite_model);
            f.close();
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} ", "failed to save", f);
        };
    }
    fn receive_single<T0, T1>(&self, name: T0, value: T1) {
        if name == "ap.enabled" {
            if self.ap_enabled != value {
                self.model.history = false;
                self.ap_enabled = value;
            }
        } else {
            if self.conf["state"].iter().any(|&x| x == name) {
                if self.state[name] != value {
                    println!("{:?} {:?} ", "state changed:", self.state);
                    self.state[name] = value;
                    self.model = false;
                }
            } else {
                if name == "timestamp" {
                    let t0 = time.monotonic();
                    if !self.firsttimestamp {
                        self.firsttimestamp = (value, t0);
                    } else {
                        let (first_value, first_t0) = self.firsttimestamp;
                        let mut dt = (value - first_value);
                        let dtl = (t0 - first_t0);
                        if (dtl - dt) > 10.0 {
                            println!("{:?} {:?} ", "computation not keep up!!", (dtl - dt));
                        }
                    }
                    let mut dt = (value - self.lasttimestamp);
                    self.lasttimestamp = value;
                    let dte = abs((dt - (1.0 / float(rate(self.conf)))));
                    if dte > 0.05 {
                        self.history.clear();
                        return;
                    }
                    for s in self.conf["sensors"] {
                        if !self.inputs.iter().any(|&x| x == s) {
                            println!("{:?} {:?} ", "missing input", s);
                            return;
                        }
                    }
                    if !self.history {
                        self.history = History(self.conf, self.state, true);
                    }
                    self.history.put(self.inputs);
                    self.train();
                } else {
                    self.model.data(name, value);
                }
            }
        }
    }
    fn receive(&self) {
        if self.playback_file {
            let line = self.playback_file.readline();
            if !line {
                println!("{:?} ", "end of file");
                exit(0);
            }
            let msg = json.loads(line);
            for name in msg {
                self.receive_single(name, msg[name]);
            }
            return;
        }
        self.client.poll(1);
        for (name, value) in self.client.received {
            let (name, value) = msg;
            if self.record_file {
                self.record_data(name, value);
            } else {
                self.receive_single(name, value);
            }
        }
        self.client.received = vec![];
    }
    fn record_data<T0, T1>(&self, name: T0, value: T1) {
        self.record_file.write(json.dumps([(name, value)].iter().cloned().collect::<HashMap<_, _>>()));
        self.record_file.write("
");
    }
    fn record<T0>(&self, filename: T0) {
        let try_dummy = { //unsupported
            self.record_file = open(filename, "w");
            self.record_file.lines = 0;
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ", "unable to open for recording", filename, e);
        };
    }
    fn playback<T0>(&self, filename: T0) {
        let try_dummy = { //unsupported
            self.playback_file = open(filename, "rb");
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ", "failed to open replay file", filename, e);
        };
    }
    fn run(&self) {
        use signal::{signal};
        fn cleanup<T0, T1>(a: T0, b: T1) {
            println!("{:?} {:?} ", "time spent load time", self.load_time.time());
            println!("{:?} {:?} ", "time spent fit time", self.fit_time.time());
            println!("{:?} {:?} ", "time spent total", self.total_time.time());
            exit(0);
        }
        signal(2, cleanup);
        for p in self.conf["predictions"] {
            if !self.conf["sensors"].iter().any(|&x| x == p) {
                self.conf["sensors"].append(p);
            }
        }
        let t0 = time.monotonic();
        println!("{:?} {:?} ", "connecting to", self.host);
        self.client = pypilotClient(self.host);
        let watches = (((self.conf["sensors"] + self.conf["state"]) + "ap.enabled") + "timestamp");
        for name in watches {
            client.watch(name);
        }
        while true {
            self.receive();
            if (time.monotonic() - t0) > 600 {
                self.save();
            }
        }
    }
}

fn main() {
    let try_dummy = { //unsupported
        let (args, host) = getopt.getopt(sys.argv[1..], "p:r:h");
        if host {
            host = host[0];
        } else {
            host = "localhost";
        }
    };
    let except!(Exception) = { //unsupported
        println!("{:?} {:?} ", "failed to parse command line arguments:", e);
        return;
    };
    let kerasmodel = Kerasmodel(host);
    for arg in args {
        let (name, value) = arg;
        if name == "-h" {
            println!("{:?} ", (sys.argv[0] + " [ARGS] [HOST]
"));
            println!("{:?} ", "-p filename -- playback from filename instead of live");
            println!("{:?} ", "-r filename -- record to file data for playback, no processing");
            println!("{:?} ", "-h          -- Display this message");
            return;
        } else {
            if name == "-p" {
                kerasmodel.playback(value);
            } else {
                if name == "-r" {
                    kerasmodel.record(value);
                }
            }
        }
    }
    kerasmodel.run();
}

fn main() {
    main();
}