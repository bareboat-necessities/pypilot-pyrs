use std::collections::HashMap;
use std::*;

struct tobject {
    d: HashMap<_, _>,
}

impl tobject {
    fn __init__<T0>(&self, path: T0) {
        self.d = HashMap::new();
        if !path {
            return;
        }
        let f = open(path);
        let mut id = None;
        while true {
            let l = f.readline();
            if !l {
                break;
            }
            if l.startswith("msgid \"") {
                id = l[7..-2];
            } else {
                if l.startswith("msgstr \"") && id {
                    let str = l[8..-2];
                    self.d[id] = str;
                }
            }
        }
    }
    fn gettext<T0, RT>(&self, id: T0) -> RT {
        if self.d.iter().any(|&x| x == id) {
            return self.d[id];
        }
        return id;
    }
}
fn translation<T0, T1, T2, T3, RT>(name: T0, path: T1, languages: T2, fallback: T3) -> RT {
    if languages[0] == "en" {
        return tobject(None);
    }
    return tobject((((((path + "/") + languages[0]) + "/LC_MESSAGES/") + name) + ".po"));
}
