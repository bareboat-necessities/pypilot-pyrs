use std::collections::HashMap;
use std::*;

use pypilot::client::PypilotClient;

fn unique<T0, RT>(l: T0) -> RT {
    if l.len() < 2 {
        return l;
    }
    if l[0] == l[1] {
        return unique(l[1..]);
    }
    return (vec![l[0]] + unique(l[1..]));
}

fn frange<T0, T1, T2, RT>(min: T0, max: T1, step: T2) -> RT {
    fn each<T0, RT>(val: T0) -> RT {
        if val > max {
            return vec![];
        }
        return (vec![val] + each((val + step)));
    }
    return each(min);
}

struct autogain {
    search: ST0,
    variables: ST1,
    settle_period: ST2,
    period: ST3,
    watchlist: ST4,
    client: ST5,
    total: HashMap<_, _>,
    searchval: HashMap<_, _>,
    results: HashMap<_, _>,
}

impl autogain {
    fn __init__(&self) {
        self.search = vec![[
            ("name", "ap.D"),
            ("min", 0.07),
            ("max", 0.12),
            ("step", 0.001),
        ]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>()];
        self.variables = vec!["ap.heading_error", "servo.watts"];
        self.settle_period = 20;
        self.period = 120;
        self.watchlist = vec!["ap.enabled"];
        for var in self.search {
            self.watchlist.append(var["name"]);
        }
        for var in self.variables {
            self.watchlist.append(var);
        }
        fn on_con<T0>(client: T0) {
            for name in self.watchlist {
                client.watch(name);
            }
        }
        println!("{:?} ", "connecting to server...");
        let mut host = false;
        if sys.argv.len() > 1 {
            host = sys.argv[1];
        }
        while true {
            let try_dummy = {
                //unsupported
                self.client = PypilotClient(on_con, host, true);
                break;
            };
            let except!() = {
                //unsupported
                time.sleep(2);
            };
        }
        println!("{:?} ", "connected");
    }
    fn read_messages<T0>(&self, log: T0) {
        let msgs = self.client.receive();
        for name in msgs {
            let data = msgs[name];
            let value = data["value"];
            for var in self.search {
                if name == var {
                    name = name[3..];
                    if abs((value - self.gains[name].value)) > 1e-08 {
                        println!(
                            "{:?} {:?} {:?} ",
                            "external program adjusting search variable!!, abort", name, value
                        );
                        exit(0);
                    }
                }
            }
            if log {
                for var in self.variables {
                    if name == var {
                        self.total[name]["total"] += abs(value);
                        self.total[name]["count"] += 1;
                    }
                }
            }
            if name == "ap.enabled" && !value { /*pass*/ }
        }
    }
    fn set<T0, T1>(&self, name: T0, val: T1) {
        println!("{:?} {:?} {:?} {:?} ", "setting", name, "to", val);
        self.searchval[name] = val;
        self.client.set(name, val);
    }
    fn log(&self) {
        println!("{:?} {:?} ", "logging for", self.searchval);
        let t0 = time.monotonic();
        self.total = HashMap::new();
        for var in self.variables {
            self.total[var] = [("total", 0), ("count", 0)]
                .iter()
                .cloned()
                .collect::<HashMap<_, _>>();
        }
        while (time.monotonic() - t0) < self.period {
            self.read_messages((time.monotonic() - t0) > self.settle_period);
            time.sleep(0.05);
        }
        for var in self.variables {
            if !self.results.iter().any(|&x| x == var) {
                self.results[var] = vec![];
            }
            let count = self.total[var]["count"];
            if count {
                self.results[var]
                    .append((self.searchval.copy(), (self.total[var]["total"] / count)));
            } else {
                println!("{:?} {:?} ", "warning, no results for", var);
            }
        }
    }
    fn run_search<T0>(&self, search: T0) {
        if search {
            let s = search[0];
            for val in frange(s["min"], s["max"], s["step"]) {
                self.set(s["name"], val);
                self.run_search(search[1..]);
            }
        } else {
            self.log();
        }
    }
    fn result_range<T0, T1, RT>(&self, results: T0, name: T1) -> RT {
        let mut r = vec![];
        for result in results {
            let (vars, val) = result;
            r.push(vars[name]);
        }
        return unique(sorted(r));
    }
    fn result_value<T0, T1, RT>(&self, results: T0, vals: T1) -> RT {
        let mut values = vec![];
        for result in results {
            let (vars, val) = result;
            if vars == vals {
                values.push(val);
            }
        }
        if values.len() == 1 {
            return ("%.4f" % values[0]);
        }
        return values;
    }
    fn print_results<T0, T1, T2>(&self, results: T0, search: T1, vals: T2) {
        let l = search.len();
        let s = search[0];
        if l < 2 {
            println!("{:?} ", s["name"]);
            for val in self.result_range(results, s["name"]) {
                vals[s["name"]] = val;
                println!("{:?} {:?} ", val, self.result_value(results, vals));
            }
        } else {
            if l > 2 {
                for val in self.result_range(results, s["name"]) {
                    println!("{:?} {:?} {:?} ", s["name"], "=", val);
                    vals[s["name"]] = val;
                    self.print_results(results, search[1..], vals);
                }
            } else {
                if l == 2 {
                    let t = search[1];
                    println!("{:?} {:?} {:?} ", s["name"], "/", t["name"]);
                    let mut line = "	";
                    let s_range = self.result_range(results, s["name"]);
                    for val0 in s_range {
                        line += ("%.4f	" % val0);
                    }
                    println!("{:?} ", line);
                    for val1 in self.result_range(results, t["name"]) {
                        line = ("%.4f	" % val1);
                        vals[t["name"]] = val1;
                        for val0 in s_range {
                            vals[s["name"]] = val0;
                            line += (String::from(self.result_value(results, vals)) + "	");
                        }
                        println!("{:?} ", line);
                    }
                    println!("{:?} ", "");
                }
            }
        }
    }
    fn run(&self) {
        self.searchval = HashMap::new();
        self.results = HashMap::new();
        self.run_search(self.search);
        for var in self.variables {
            println!("{:?} {:?} ", "Results for", var);
            self.print_results(self.results[var], self.search, HashMap::new());
            println!("{:?} ", "");
        }
    }
}

fn main() {
    let ag = autogain();
    ag.run();
}
