use std::*;
use std::collections::HashMap;

let try_dummy = { //unsupported
};
let except!() = { //unsupported
use PIL::{Image};
};
use OpenGL::GLUT::{*};
use OpenGL::GLU::{*};
use OpenGL::GL::{*};
let try_dummy = { //unsupported
use pywavefront::{visualization};
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ",_("failed to load pywavefront:"), e);
const pywavefront: _ = false;
};
use pypilot::{quaternion};
struct BoatPlot {
Q: ST0,
Scale: ST1,
compasstex: ST2,
obj: bool,
texture_compass: bool,
dim: ST3,
}

impl BoatPlot {
fn __init__(&self)  {
self.Q = vec![-0.32060682, -0.32075041, 0.73081691, -0.51013437];
self.Scale = 3;
self.compasstex = 0;
self.obj = false;
self.texture_compass = true;
}
fn chdir(&self)  {
let path = os.path.dirname(__file__);
os.chdir(os.path.abspath(path));
}
fn display<T0>(&self, fusionQPose: T0)  {
let (width, height) = self.dim;
glViewport(0, 0, width, height);
glMatrixMode(GL_PROJECTION);
glLoadIdentity();
if width < 10||height < 10 {
println!("{:?} {:?} {:?} ",("boatplot: " + _("invalid display dimensions")), width, height);
return;
}
let ar = ((0.5*width)/height);
glFrustum(-(ar), ar, -0.5, 0.5, 2.0, 300.0);
glMatrixMode(GL_MODELVIEW);
glLoadIdentity();
glClearColor(0, 0.2, 0.7, 0);
glClearDepth(100);
glClear((GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT));
glPushMatrix();
fn glRotateQ<T0>(q: T0)  {
let try_dummy = { //unsupported
glRotatef(((quaternion::angle(q)*180)/math.pi), q[1], q[2], q[3]);
};
let except!() = { //unsupported
/*pass*/
};
}
let dist = 12;
glTranslatef(0, 0, -(dist));
glScalef(self.Scale, self.Scale, self.Scale);
glRotateQ(self.Q);
if self.obj {
glPushMatrix();
let q = fusionQPose;
glRotateQ(q);
let s = 0.2;
glScalef(s, s, s);
glRotatef(90, 0, 0, -1);
glRotatef(90, -1, 0, 0);
glEnable(GL_LIGHTING);
let lightfv = (ctypes.c_float*4);
glLightfv(GL_LIGHT0, GL_DIFFUSE, lightfv(1.0, 1.0, 1.0, 1.0));
glEnable(GL_LIGHT0);
visualization::draw(self.obj);
glDisable(GL_LIGHTING);
glPopMatrix();
} else {
if pywavefront {
self.chdir();
let try_dummy = { //unsupported
self.obj = pywavefront.Wavefront("Vagabond.obj");
};
let except!(Exception) = { //unsupported
println!("{:?} {:?} ",("Vagabond.obj " + _("failed to load")), e);
println!("{:?} ",_("Did you add the pypilot_data repository?"));
};
}
}
glEnable(GL_DEPTH_TEST);
if self.texture_compass {
self.draw_texture_compass();
} else {
self.draw_vector_compass();
}
glPopMatrix();
}
fn draw_vector_compass(&self)  {
let s = 1;
fn draw_string<T0, T1>(s: T0, pos: T1)  {
let viewport = glGetIntegerv(GL_VIEWPORT);
let proj = glGetDoublev(GL_PROJECTION_MATRIX);
let model = glGetDoublev(GL_MODELVIEW_MATRIX);
let winpos = gluProject(pos[0], pos[1], pos[2], model, proj, viewport);
glPushMatrix();
glLoadIdentity();
glMatrixMode(GL_PROJECTION);
glPushMatrix();
glLoadIdentity();
glRasterPos2d((((2*winpos[0])/viewport[2]) - 1), (((2*winpos[1])/viewport[3]) - 1));
for c in s {
glutBitmapCharacter(GLUT_BITMAP_9_BY_15, ctypes.c_int(ord(c)));
}
glPopMatrix();
glMatrixMode(GL_MODELVIEW);
glPopMatrix();
}
draw_string("N", vec![s, 0, 0]);
draw_string("S", vec![-(s), 0, 0]);
draw_string("E", vec![0, s, 0]);
draw_string("W", vec![0, -(s), 0]);
glBegin(GL_LINES);
let f = (0.9*s);
(glVertex2f(-(f), 0), glVertex2f(f, 0));
(glVertex2f(0, -(f)), glVertex2f(0, f));
glEnd();
}
fn draw_texture_compass(&self)  {
if self.compasstex == 0 {
self.chdir();
let try_dummy = { //unsupported
let img = Image::open("compass.png");
};
let except!() = { //unsupported
println!("{:?} ",("compass.png " + _("not found, texture compass cannot be used")));
self.texture_compass = false;
return;
};
self.compasstex = glGenTextures(1);
let data = numpy.array(img.getdata().collect::<Vec<_>>(), numpy.int8);
glBindTexture(GL_TEXTURE_2D, self.compasstex);
glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
gluBuild2DMipmaps(GL_TEXTURE_2D, GL_RGBA, img.size[0], img.size[1], GL_RGBA, GL_UNSIGNED_BYTE, data);
}
glEnable(GL_TEXTURE_2D);
glEnable(GL_BLEND);
glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
glBindTexture(GL_TEXTURE_2D, self.compasstex);
glBegin(GL_QUADS);
(glTexCoord2f(0, 0), glVertex3f(1, -1, 0));
(glTexCoord2f(1, 0), glVertex3f(1, 1, 0));
(glTexCoord2f(1, 1), glVertex3f(-1, 1, 0));
(glTexCoord2f(0, 1), glVertex3f(-1, -1, 0));
glEnd();
glDisable(GL_BLEND);
glDisable(GL_TEXTURE_2D);
}
fn reshape<T0, T1>(&self, width: T0, height: T1)  {
self.dim = (width, height);
} 
}
fn main() {
let plot = BoatPlot();
fn display()  {
plot.display(vec![1, 0, 0, 0]);
glutSwapBuffers();
}
let last = false;
fn mouse<T0, T1, T2, T3>(button: T0, state: T1, x: T2, y: T3)  {
if button == GLUT_LEFT_BUTTON&&state == GLUT_DOWN {
//global last
last = (x, y);
}
}
fn motion<T0, T1>(x: T0, y: T1)  {
//global last
let (dx, dy) = ((x - last[0]), (y - last[1]));
let q = quaternion::angvec2quat((((dx.pow(2) + dy.pow(2)).pow(0.4)/180)*math.pi), vec![dy, dx, 0]);
plot.Q = quaternion::multiply(q, plot.Q);
last = (x, y);
glutPostRedisplay();
}
fn keyboard<T0, T1, T2>(key: T0, x: T1, y: T2)  {
exit(0);
}
glutInit(sys.argv);
glutInitWindowPosition(0, 0);
glutInitWindowSize(600, 500);
glutInitDisplayMode(((GLUT_DOUBLE | GLUT_RGB) | GLUT_DEPTH));
glutCreateWindow(sys.argv[0]);
glutReshapeFunc(plot.reshape);
glutDisplayFunc(display);
glutKeyboardFunc(keyboard);
glutMouseFunc(mouse);
glutMotionFunc(motion);
glutMainLoop();
}