use std::*;
use std::collections::HashMap;

LIRC.init("pypilot");
while true {
const code: _ = LIRC.nextcode(1);
if code {
println ! ("{:?} ", code);
}
time.sleep(0.1);
}