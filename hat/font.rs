use std::*;
use std::collections::HashMap;

let try_dummy = { //unsupported
const fontpath: _ = "";
const character: _ = ugfx::surface(76, 149, 1);
};
let except!() = { //unsupported
const micropython: _ = false;
use pypilot::hat::ugfx::{ugfx};
fontpath = (os.path.abspath((os.getenv("HOME") + "/.pypilot/ugfxfonts/")) + "/");
if !os.path.exists(fontpath) {
os.makedirs(fontpath);
}
if !os.path.isdir(fontpath) {
raise!("ugfxfonts should be a directory"); //unsupported
}
};
//global fonts
const fonts: _ = HashMap::new();
fn draw<T0, T1, T2, T3, T4, T5, RT>(surface: T0, pos: T1, text: T2, size: T3, bw: T4, crop: T5) -> RT {
if !fonts.iter().any(|&x| x == size) {
fonts[size] = HashMap::new();
}
let font = fonts[size];
if pos {
let (x, y) = pos;
} else {
let (x, y) = (0, 0);
}
let origx = x;
let mut width = 0;
let mut lineheight = 0;
let mut height = 0;
for c in text {
if c == "
" {
let mut x = origx;
y += lineheight;
height += lineheight;
lineheight = 0;
continue;
}
if font.iter().any(|&x| x == c) {
src = font[c];
} else {
src = None;
while size > 1 {
let mut filename = (fontpath + ("%03d%03d" % (size, ord(c))));
if micropython {
character.load(filename.encode("utf-8"), surface.bypp);
src = character;
} else {
if bw {
filename += "b";
}
if crop {
filename += "c";
}
src = ugfx::surface(filename.encode("utf-8"), surface.bypp);
}
if src.bypp == surface.bypp {
break;
}
if !micropython {
let try_dummy = { //unsupported
println!("{:?} {:?} {:?} {:?} {:?} ","create font charater", c, size, src.bypp, surface.bypp);
};
let except!() = { //unsupported
println!("{:?} {:?} {:?} {:?} ","create font charater", size, src.bypp, surface.bypp);
println!("{:?} ","unable to print unicode character to console");
};
src = create_character((os.path.abspath(os.path.dirname(__file__)) + "/font.ttf"), size, c, surface.bypp, crop, bw);
if src {
println!("{:?} {:?} ","store grey", filename);
src.store_grey(filename.encode("utf-8"));
}
break;
}
size -= 1;
}
}
if !src||src.bypp != surface.bypp {
println!("{:?} {:?} {:?} ","font dont have character", ord(c), size);
continue;
}
if pos {
surface.blit(src, x, y);
}
x += src.width;
width = width.iter().max().unwrap();
lineheight = lineheight.iter().max().unwrap();
if !micropython {
font[c] = src;
}
}
return (width, (height + lineheight));
}
fn create_character<T0, T1, T2, T3, T4, T5, RT>(fontpath: T0, size: T1, c: T2, bypp: T3, crop: T4, bpp: T5) -> RT {
let try_dummy = { //unsupported
use PIL::{Image};
use PIL::{ImageDraw};
use PIL::{ImageFont};
use PIL::{ImageChops};
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ","failed to load PIL to create fonts, aborting...", e);
return false;
time.sleep(3);
exit(0);
};
let ifont = ImageFont.truetype(fontpath, size);
size = ifont.getsize(c);
let mut image = Image.new("RGBA", size);
let draw = ImageDraw.Draw(image);
draw.text((0, 0), c, ifont);
if crop {
let bg = Image.new(image.mode, image.size, image.getpixel((0, 0)));
let diff = ImageChops.difference(image, bg);
let bbox = diff.getbbox();
if bbox {
image = image.crop(bbox);
}
}
if bpp {
let data = image.getdata().collect::<Vec<_>>();
for i in (0..data.len()) {
let d = (255/(1 << bpp));
let v = i32::from((round((data[i][3]/(255/(1 << bpp))))*(255/((1 << bpp) - 1))));
data[i] = (v, v, v, v);
}
image.putdata(data);
}
return ugfx::surface(image.size[0], image.size[1], bypp, image.tobytes());
}
fn main() {
println!("{:?} ","ugfx test program");
let screen = ugfx::screen("/dev/fb0".encode("utf-8"));
draw(screen, (0, 100), "1234567890", 28, false);
}