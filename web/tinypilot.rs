use std::*;
use std::collections::HashMap;

const temp: _ = tempfile.mkstemp();
const p: _ = subprocess.Popen(vec!["uname", "-r"], temp[0], true);
p.wait();
const f: _ = os.fdopen(temp[0], "r");
f.seek(0);
const kernel_release: _ = f.readline().rstrip();
f.close();
const tinypilot: _ = kernel_release.iter().any(|&x| x == "piCore");
tinypilot = if tinypilot { 1 } else { 0 };