use std::*;
use std::collections::HashMap;

const e: _ = sys.stdout.encoding.lower();
if e.startswith("utf") {
use gettext;
use os;
let locale_d = (os.path.abspath(os.path.dirname(__file__)) + "/locale");
gettext.translation("pypilot", locale_d, true).install();
} else {
use builtins;
builtins.__dict__["_"] = | x | x;
}