use std::*;
use std::collections::HashMap;

while true {
while true {
let try_dummy = { //unsupported
const gpsd: _ = gps.gps(gps.WATCH_ENABLE);
break;
};
let except!() = { //unsupported
time.sleep(3);
};
}
while true {
let try_dummy = { //unsupported
gpsd.next();
};
let except!(KeyboardInterrupt) = { //unsupported
exit(1);
};
let except!() = { //unsupported
break;
};
if gpsd.utc.len() {
let (date, t) = gpsd.utc[..-5].split("T");
println!("{:?} {:?} {:?} ","Setting date to gps time", date, t);
sys.stdout.flush();
os.system((((("date -u -s \"" + date) + " ") + t) + "\""));
gpsd.drop();
time.sleep((((3*24)*60)*60));
break;
}
}
}