use std::collections::HashMap;
use std::*;

use pypilot::hat::ugfx::ugfx;
use OpenGL::GL::*;
use OpenGL::GLU::*;
use OpenGL::GLUT::*;
struct screen {}

impl screen {
    fn __init__<T0>(&self, size: T0) {
        super(screen, self).__init__(size[0], size[1], 4, None);
        self.fill(0);
        glutInit(sys.argv);
        glutInitWindowPosition(250, 0);
        glutInitWindowSize(i32::from(((640 * size[0]) / size[1])), 640);
        glutInitDisplayMode((GLUT_DOUBLE | GLUT_RGB));
        glutCreateWindow("virtual lcd screen as glut window");
        fn display() {
            glEnable(GL_TEXTURE_2D);
            glBindTexture(GL_TEXTURE_2D, 0);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            let mut p = vec![];
            for y in (0..size[1]) {
                for x in (0..size[0]) {
                    let v = i32::from(self.getpixel(x, y));
                    p.push(((v >> 16) & 255));
                    p.push(((v >> 8) & 255));
                    p.push((v & 255));
                    p.push(0);
                }
            }
            let data = numpy.array(p.collect::<Vec<_>>(), numpy.int8);
            OpenGL.raw.GL.VERSION.GL_1_1.glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA,
                size[0],
                size[1],
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                data,
            );
            glBegin(GL_QUADS);
            (glTexCoord2f(0, 1), glVertex2f(0, 0));
            (glTexCoord2f(1, 1), glVertex2f(1, 0));
            (glTexCoord2f(1, 0), glVertex2f(1, 1));
            (glTexCoord2f(0, 0), glVertex2f(0, 1));
            glEnd();
            glDisable(GL_TEXTURE_2D);
            glutSwapBuffers();
        }
        fn reshape<T0, T1>(w: T0, h: T1) {
            glViewport(0, 0, w, h);
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();
            gluOrtho2D(0, 1, 0, 1);
        }
        fn key<T0, T1, T2>(k: T0, x: T1, y: T2) {
            if k == "q" || k == 27 {
                exit(0);
            }
        }
        glutDisplayFunc(display);
        glutReshapeFunc(reshape);
        glutKeyboardFunc(key);
    }
    fn refresh(&self) {
        OpenGL.GLUT.glutPostRedisplay();
    }
}
fn main() {
    let s = screen((128, 160));
    s.invert(10, 10, 20, 20);
    glutMainLoop();
}
