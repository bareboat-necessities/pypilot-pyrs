use std::*;
use std::collections::HashMap;

use __future__::{print_function};
println!("{:?} ",_("pypilot failed to import required modules.  Did you forget to run sudo python3 setup.py install?"));
if sys.version_info[0] < 3 {
println ! ("{:?} {:?} ", "pypilot requires python version 3.  python version is", sys.version);
println ! ("{:?} ", "I will now attempt to re-run the command using python 3");
let cmd = "python3 ";
for arg in sys.argv {
cmd += (arg + " ");
}
use os;
os.system(cmd);
}
exit(1);