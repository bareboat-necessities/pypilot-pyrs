use std::*;
use std::collections::HashMap;

use PIL::{Image};
use PIL::{Image};
use PIL::{ImageDraw};
use PIL::{ImageFont};
const ifont: _ = ImageFont::truetype("../font.ttf", 14);
const version: _ = pypilot.version.strversion;
println!("{:?} {:?} ","using version", version);
const size: _ = ifont.getsize(version);
const image: _ = Image::new("RGBA", size);
const draw: _ = ImageDraw::Draw(image);
draw.text((0, 0), version, ifont);
const data: _ = image.getdata().collect::<Vec<_>>();
println!("{:?} {:?} {:?} ","len", data.len(), size);
const f: _ = open("pypilot_version.h", "w");
f.write(("static unsigned int width = %d;
" % size[0]));
f.write(("static unsigned int height = %d;

" % size[1]));
f.write("#define HEADER_PIXEL(data,pixel) {\
");
f.write("pixel[0] = data[0]; \
");
f.write("pixel[1] = data[0]; \
");
f.write("pixel[2] = data[0]; \
");
f.write("data ++; }

");
f.write("static char header_data[] = {");
for i in (0..data.len()) {
let d = data[i];
if d[3] < 128 {
f.write("255,");
} else {
f.write("0,");
}
}
f.write("};
");
f.close();