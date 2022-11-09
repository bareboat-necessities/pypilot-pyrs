use std::collections::HashMap;
use std::*;

use intellect::*;
use pilot::{AutopilotGain, AutopilotPilot};

const disabled: _ = true;

fn build_actions<T0, T1, T2, RT>(current: T0, period_count: T1, count: T2) -> RT {
    if count <= 0 {
        return vec![];
    }
    let mut ret = vec![];
    fn actions<T0, RT>(command: T0) -> RT {
        if count <= period_count {
            return vec![(vec![command] * count)];
        }
        return build_actions(command, period_count, (count - period_count))
            .iter()
            .map(|acts| ((vec![command] * period_count) + acts))
            .collect::<Vec<_>>();
    }
    if current <= 0 {
        ret += actions(-1);
    }
    if current >= 0 {
        ret += actions(1);
    }
    ret += actions(0);
    return ret;
}

struct TFliteModel {
    conf: ST0,
    interpreter: ST1,
    history: ST2,
    start_time: ST3,
}

impl TFliteModel {
    fn __init__(&self) {
        super(TFliteModel, self).__init__();
    }
    fn load<T0>(&self, state: T0) {
        let filename = model_filename(state);
        let try_dummy = {
            //unsupported
            let f = open((filename + ".conf"));
            self.conf = json.loads(f.read());
            f.close();
            let t0 = time.monotonic();
            let interpreter = tf.lite.Interpreter((filename + ".tflite_model"));
            let t1 = time.monotonic();
            interpreter.allocate_tensors();
            let t2 = time.monotonic();
            let input_details = interpreter.get_input_details();
            let output_details = interpreter.get_output_details();
            let t3 = time.monotonic();
            let input_shape = input_details[0]["shape"];
            println!("{:?} {:?} ", "input details", input_details);
            let t4 = time.monotonic();
            let t5 = time.monotonic();
            let t6 = time.monotonic();
            println!(
                "{:?} {:?} {:?} {:?} {:?} {:?} {:?} ",
                "interpreter timings",
                (t1 - t0),
                (t2 - t1),
                (t3 - t2),
                (t4 - t3),
                (t5 - t4),
                (t6 - t5)
            );
            self.interpreter = interpreter;
            self.history = History(self.conf, state);
        };
        let except!(Exception) = {
            //unsupported
            self.start_time = time.monotonic();
            println!("{:?} {:?} ", "failed to load model", filename);
            self.interpreter = false;
        };
    }
    fn predict<T0, RT>(&self, loss: T0) -> RT {
        let p = self.present();
        let sensors_data = intellect.inputs(self.history.data, self.conf["sensors"]);
        let count = (self.history.samples - self.present());
        let rate = state["imu.rate"];
        let current = self.servo.command.value;
        let period = 0.4;
        let actions = self.build_actions(current, (period / rate), count);
        let inputs = actions
            .iter()
            .map(|action| (sensors_data + action))
            .collect::<Vec<_>>();
        self.interpreter
            .set_tensor(input_details[0]["index"], np.array(inputs));
        self.interpreter.invoke();
        let outputs = interpreter.get_tensor(output_details[0]["index"]);
        let pnames = self.conf["predictions"];
        let mut besti = false;
        for i in inputs.len() {
            let output = outputs[i].reshape(-1, pnames.len());
            let mut weight = 0;
            for j in output.len() {
                let prediction = HashMap::new();
                for k in pnames.len() {
                    prediction[pnames[j]] = (output[j][k], self.conf["accuracy"][j][k]);
                }
                let action = actions[i][j];
                weight += loss(prediction, action);
            }
            if !besti || weight < best {
                besti = i;
                let best = weight;
            }
        }
        return actions[besti];
    }
}

struct LearningPilot {
    P: ST0,
    D: ST1,
    W: ST2,
    state: bool,
    start_time: ST3,
}

impl LearningPilot {
    fn __init__<T0>(&self, ap: T0) {
        super(LearningPilot, self).__init__("learning", ap);
        self.P = self.register(AutopilotGain, "P", 0.001, 0.0001, 0.01);
        self.D = self.register(AutopilotGain, "D", 0.03, 0.01, 0.1);
        self.W = self.register(AutopilotGain, "W", 0, 0, 0.1);
        self.state = false;
        self.start_time = time.monotonic();
    }
    fn loss<T0, T1, RT>(predictions: T0, action: T1) -> RT {
        let (heading, heading_accuracy) = predictions["imu.heading_error"];
        let (headingrate, headingrate_accuracy) = predictions["imu.headingrate_lowpass"];
        return (((self.P.value * heading) + (self.D.value * headingrate)).pow(2)
            + (self.W.value * action.pow(2)));
    }
    fn process<T0>(&self, reset: T0) {
        let ap = self.ap;
        let state = ["ap.mode", ap.mode.value, "imu.rate", ap.imu.rate.value]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        if self.state != state || !self.model {
            if (time.monotonic() - self.start_time) < 2 {
                return;
            }
            self.load(state);
            if !self.model {
                ap.pilot.set("basic");
                return;
            }
        }
        let data = HashMap::new();
        for sensor in (self.conf["sensors"] + self.conf["predictions"]) {
            data[sensor] = self.ap.client.values[sensor].value;
        }
        self.model.history.put(data);
        if !self.history.full() {
            self.ap.pilots["basic"].process(reset);
            return;
        }
        if ap.enabled.value {
            let actions = self.model.predict(self.loss);
            ap.servo.command.set(actions[0]);
        }
    }
}

const pilot: _ = LearningPilot;
