use std::*;
use std::collections::HashMap;

use pypilot::client::{pypilotClient};
use pypilot::{quaternion};
use pypilot::{vector};
use OpenGL::GLUT::{*};
use OpenGL::GLU::{*};
use OpenGL::GL::{*};

const history_point_count: _ = 200;
const recent_point_count: _ = 20;

use shape::{*};

fn TranslateAfter<T0, T1, T2>(x: T0, y: T1, z: T2) {
    let m = glGetFloatv(GL_MODELVIEW_MATRIX);
    glLoadIdentity();
    glTranslatef(x, y, z);
    glMultMatrixf(m);
}

fn RotateAfter<T0, T1, T2, T3>(ang: T0, x: T1, y: T2, z: T3) {
    let m = glGetFloatv(GL_MODELVIEW_MATRIX);
    glLoadIdentity();
    glRotatef(ang, x, y, z);
    glMultMatrixf(m);
}

fn rotate_mouse<T0, T1>(dx: T0, dy: T1) {
    RotateAfter((dx.pow(2) + dy.pow(2)).pow(0.1), dy, dx, 0);
}

struct CalibrationPlot {
    name: ST0,
    mode: ST1,
    fusionQPose: ST2,
    alignmentQ: ST3,
    recentpoints: Vec<_>,
    historypoints: Vec<_>,
    sigmapoints: Vec<_>,
    points: Vec<_>,
    uncalibrated_view: ST4,
    dim: ST5,
}

impl CalibrationPlot {
    fn __init__<T0>(&self, name: T0) {
        self.name = name;
        self.mode = GL_LINE;
        self.fusionQPose = vec![1, 0, 0, 0];
        self.alignmentQ = vec![1, 0, 0, 0];
        self.recentpoints = vec![];
        self.historypoints = vec![];
        self.sigmapoints = vec![];
        self.points = vec![];
    }
    fn add_point<T0>(&self, value: T0) {
        if !value {
            return;
        }
        self.recentpoints.append(value);
        if self.recentpoints.len() > (recent_point_count * 2) {
            let avg = vec![0, 0, 0];
            for point in self.recentpoints[..recent_point_count] {
                for i in (0..3) {
                    avg[i] += point[i];
                }
            }
            for i in (0..3) {
                avg[i] /= recent_point_count;
            }
            self.historypoints.append(avg);
            self.recentpoints = self.recentpoints[recent_point_count..];
            if self.historypoints.len() > history_point_count {
                self.historypoints = self.historypoints[1..];
            }
        }
    }
    fn read_data_plot<T0>(&self, msg: T0) {
        let (name, value) = msg;
        if name == "imu.fusionQPose" {
            self.fusionQPose = value;
        } else {
            if name == "imu.alignmentQ" {
                self.alignmentQ = value;
            } else {
                if name == ("imu." + self.name) {
                    self.add_point(value);
                } else {
                    if name == (("imu." + self.name) + ".calibration.sigmapoints") {
                        self.sigmapoints = value;
                    } else {
                        if name == (("imu." + self.name) + ".calibration.points") {
                            self.points = value;
                        }
                    }
                }
            }
        }
    }
    fn display_setup<RT>(&self) -> RT {
        let (width, height) = self.dim;
        let ar = (float(width) / float(height));
        glViewport(0, 0, width, height);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        let fac = 0.05;
        glFrustum((-(fac) * ar), (fac * ar), -(fac), fac, 0.1, 15);
        glMatrixMode(GL_MODELVIEW);
        glClearColor(0, 0, 0, 0);
        glClear((GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT));
        glPushMatrix();
        let s = self.userscale;
        glScalef(s, s, s);
        TranslateAfter(0, 0, -1);
        glPolygonMode(GL_FRONT_AND_BACK, self.mode);
        glLineWidth(1);
        glPushMatrix();
        if !self.fusionQPose {
            return vec![0, 0, 1];
        }
        let q = quaternion::multiply(self.fusionQPose, self.alignmentQ);
        let down = quaternion::rotvecquat(vec![0, 0, 1], quaternion::conjugate(q));
        glRotatef(-math.degrees(quaternion::angle(q)), starred!(q[1..])/*unsupported*/);
        return down;
    }
    fn draw_points(&self) {
        glPointSize(2);
        glColor3f(1, 0.3, 0.3);
        glBegin(GL_POINTS);
        for p in self.recentpoints {
            glVertex3fv(p);
        }
        glEnd();
        glPointSize(2);
        glColor3f(0, 1, 0);
        glBegin(GL_POINTS);
        for p in self.historypoints {
            glVertex3fv(p);
        }
        glEnd();
        glColor3f(1, 1, 0);
        glPointSize(4);
        glBegin(GL_POINTS);
        if self.sigmapoints {
            for p in self.sigmapoints {
                glVertex3fv(p[..3]);
            }
        }
        glEnd();
        glColor3f(0, 1, 1);
        glPointSize(4);
        glBegin(GL_POINTS);
        if self.points {
            for p in self.points {
                glVertex3fv(p[..3]);
            }
        }
        glEnd();
        glPopMatrix();
    }
    fn special<T0, T1, T2>(&self, key: T0, x: T1, y: T2) {
        let step = 5;
        if key == GLUT_KEY_UP {
            RotateAfter(step, 1, 0, 0);
        } else {
            if key == GLUT_KEY_DOWN {
                RotateAfter(step, -1, 0, 0);
            } else {
                if key == GLUT_KEY_LEFT {
                    RotateAfter(step, 0, 1, 0);
                } else {
                    if key == GLUT_KEY_RIGHT {
                        RotateAfter(step, 0, -1, 0);
                    } else {
                        if key == GLUT_KEY_PAGE_UP {
                            self.userscale /= 0.9;
                        } else {
                            if key == GLUT_KEY_PAGE_DOWN {
                                self.userscale *= 0.9;
                            } else {
                                if key == GLUT_KEY_INSERT {
                                    RotateAfter(step, 0, 0, 1);
                                } else {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        glutPostRedisplay();
    }
    fn key<T0, T1, T2>(&self, k: T0, x: T1, y: T2) {
        let step = 5;
        if k == "" {
            RotateAfter(step, 0, 0, -1);
        } else {
            if k == "+" || k == "=" {
                self.userscale /= 0.9;
            } else {
                if k == "-" || k == "_" {
                    self.userscale *= 0.9;
                } else {
                    if k == "f" {
                        glutFullScreen();
                    } else {
                        if k == "m" {
                            if self.mode == GL_LINE {
                                self.mode = GL_FILL;
                            } else {
                                self.mode = GL_LINE;
                            }
                        } else {
                            if k == "v" {
                                self.uncalibrated_view = !self.uncalibrated_view;
                            } else {
                                if k == 27 || k == "q" {
                                    exit(0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn reshape<T0, T1>(&self, width: T0, height: T1) {
        glEnable(GL_DEPTH_TEST);
        self.dim = (width, height);
    }
}

struct AccelCalibrationPlot {
    userscale: ST0,
    cal_sphere: ST1,
    fit_sphere: bool,
}

impl AccelCalibrationPlot {
    const default_radius: _ = 1;
    fn __init__(&self) {
        super(AccelCalibrationPlot, self).__init__("accel");
        self.userscale = 0.3;
        self.cal_sphere = vec![0, 0, 0, 1];
        self.fit_sphere = false;
    }
    fn read_data<T0, RT>(&self, msg: T0) -> RT {
        self.read_data_plot(msg);
        let (name, value) = msg;
        if name == "imu.accel.calibration" && value {
            fn fsphere<T0, T1, RT>(beta: T0, x: T1) -> RT {
                return ((beta[3] * x) + beta[..3]);
            }
            self.cal_sphere = value[0];
            self.fit_sphere = Spherical(self.cal_sphere, fsphere, 32, 16);
        }
    }
    fn display(&self) {
        self.display_setup();
        let cal_sphere = self.cal_sphere;
        if self.fit_sphere {
            glColor3f(0, 0.3, 0.8);
            self.fit_sphere.draw();
        }
        glPopMatrix();
        glTranslatef(-(cal_sphere[0]), -(cal_sphere[1]), -(cal_sphere[2]));
        self.draw_points();
    }
}

struct CompassCalibrationPlot {
    userscale: ST0,
    unit_sphere: ST1,
    mag_fit_new_bias: bool,
    mag_fit_sphere: bool,
    mag_cal_new_bias: ST2,
    mag_cal_new_sphere: ST3,
    mag_cal_sphere: ST4,
    accel: ST5,
    heading: ST6,
    apoints: Vec<_>,
    avg: ST7,
    mag_fit_cone: ST8,
}

impl CompassCalibrationPlot {
    default_radius = 30;
    fn __init__(&self) {
        super(CompassCalibrationPlot, self).__init__("compass");
        self.userscale = 0.005;
        self.unit_sphere = Spherical(vec![0, 0, 0, 1], |beta, x| x, 32, 16);
        self.mag_fit_new_bias = false;
        self.mag_fit_sphere = false;
        self.mag_cal_new_bias = vec![0, 0, 0, 30, 0];
        self.mag_cal_new_sphere = vec![0, 0, 0, 30, 0];
        self.mag_cal_sphere = vec![0, 0, 0, 30];
        self.accel = vec![0, 0, 0];
        self.heading = 0;
        self.apoints = vec![];
        self.avg = vec![0, 0, 0];
    }
    fn read_data<T0, RT>(&self, msg: T0) -> RT {
        self.read_data_plot(msg);
        let (name, value) = msg;
        if name == "imu.accel" {
            self.accel = value;
        } else {
            if name == "imu.heading" {
                self.heading = value;
            } else {
                if name == "imu.compass.calibration" && value {
                    fn fsphere<T0, T1, RT>(beta: T0, x: T1) -> RT {
                        return ((beta[3] * x) + beta[..3]);
                    }
                    self.mag_cal_sphere = value[0];
                    self.mag_fit_sphere = Spherical(self.mag_cal_sphere, fsphere, 32, 16);
                    self.mag_fit_cone = Conical(self.mag_cal_sphere, 32, 16);
                }
            }
        }
    }
    fn display(&self) {
        let down = self.display_setup();
        let cal_new_bias = self.mag_cal_new_bias;
        let cal_new_sphere = self.mag_cal_new_sphere;
        let cal_sphere = self.mag_cal_sphere;
        if self.mag_fit_new_bias {
            glColor3f(1, 0, 0);
            self.mag_fit_new_bias.draw();
        }
        if self.mag_fit_new_sphere {
            glColor3f(1, 0, 1);
            self.mag_fit_new_sphere.draw();
        }
        if self.mag_fit_sphere {
            glColor3f(0, 0, 1);
            self.mag_fit_sphere.draw();
        }
        if self.mag_fit_cone {
            glColor3f(1, 0, 0);
            self.mag_fit_cone.draw();
        }
        glPopMatrix();
        glTranslatef(-(cal_sphere[0]), -(cal_sphere[1]), -(cal_sphere[2]));
        glColor3f(1, 1, 1);
        glLineWidth(3.8);
        glBegin(GL_LINES);
        let try_dummy = { //unsupported
            glColor3f(0.8, 0.8, 0.8);
            glVertex3fv(down.iter().map(|x, y| ((-(x) * cal_sphere[3]) + y)).collect::<Vec<_>>());
            glVertex3fv(down.iter().map(|x, y| ((x * cal_sphere[3]) + y)).collect::<Vec<_>>());
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} {:?} ", "ERROR!", down, cal_sphere, e);
        };
        glEnd();
        self.draw_points();
    }
}

fn main() {
    let host = false;
    if sys.argv.len() > 1 {
        host = sys.argv[1];
    }
    let watchlist = vec!["imu.accel", "imu.compass", "imu.compass.calibration", "imu.compass.calibration", "imu.compass.calibration.sigmapoints", "imu.fusionQPose"];
    let client = pypilotClient(host);
    for name in watchlist {
        client.watch(name);
    }
    let plot = CompassCalibrationPlot();
    fn display() {
        plot.display();
        glutSwapBuffers();
    }
    let last = false;
    fn mouse<T0, T1, T2, T3>(button: T0, state: T1, x: T2, y: T3) {
        if button == GLUT_LEFT_BUTTON && state == GLUT_DOWN {
//global last
            last = (x, y);
        }
    }
    fn motion<T0, T1>(x: T0, y: T1) {
//global last
        rotate_mouse((x - last[0]), (y - last[1]));
        glutPostRedisplay();
        last = (x, y);
    }
    let n = 0;
    fn idle() {
        client.poll();
        while true {
            let result = client.receive_single();
            if !result {
                time.sleep(0.01);
                return;
            }
            if plot.read_data(result) {
                glutPostRedisplay();
            }
        }
    }
    glutInit(sys.argv);
    glutInitWindowPosition(0, 0);
    glutInitWindowSize(600, 500);
    glutInitDisplayMode(((GLUT_DOUBLE | GLUT_RGB) | GLUT_DEPTH));
    glutCreateWindow(sys.argv[0]);
    glutIdleFunc(idle);
    glutReshapeFunc(plot.reshape);
    glutKeyboardFunc(|| apply(plot.key, a), glutPostRedisplay());
    glutSpecialFunc(|| apply(plot.special, a), glutPostRedisplay());
    glutDisplayFunc(display);
    glutMouseFunc(mouse);
    glutMotionFunc(motion);
    glutMainLoop();
}