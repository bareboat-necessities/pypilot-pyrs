use std::*;
use std::collections::HashMap;

let default = vec![];
sys.path.append(os.path.dirname(os.path.abspath(__file__)));
for module in os.listdir(os.path.dirname(__file__)) {
if module == "__init__.py"||module[-3..] != ".py"||module.startswith(".") {
continue;
}
if module == "pilot.py" {
continue;
}
let try_dummy = { //unsupported
let mod = importlib.import_module(("pilots." + module[..-3]));
};
let except!(Exception) = { //unsupported
let try_dummy = { //unsupported
mod = importlib.import_module(module[..-3]);
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} {:?} {:?} {:?} ",_("ERROR loading"), module, e1, " ", e2);
continue;
};
};
let try_dummy = { //unsupported
if mod.disabled {
continue;
}
};
let except!(Exception) = { //unsupported
/*pass*/
};
let try_dummy = { //unsupported
default.push(mod.pilot);
};
let except!() = { //unsupported
/*pass*/
};
}