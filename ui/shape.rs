use std::collections::HashMap;
use std::*;

use OpenGL::GL::*;
fn GLArray<T0, RT>(points: T0) -> RT {
    let vpoints = (GLfloat * (3 * points.len()))();
    let mut i = 0;
    for point in points {
        for j in (0..3) {
            vpoints[(i + j)] = point[j];
        }
        i += 3;
    }
    return vpoints;
}
struct Shape {
    array: ST0,
}

impl Shape {
    fn __init__<T0>(&self, vertexes: T0) {
        self.array = GLArray(vertexes);
    }
    fn draw(&self) {
        glEnableClientState(GL_VERTEX_ARRAY);
        glVertexPointer(3, GL_FLOAT, 0, self.array);
        glDrawArrays(GL_LINE_STRIP, 0, i32::from((self.array.len() / 3)));
        glDisableClientState(GL_VERTEX_ARRAY);
    }
}
struct Spherical {}

impl Spherical {
    fn __init__<T0, T1, T2, T3>(&self, beta: T0, f: T1, lons: T2, lats: T3) {
        let mut lastPoints = false;
        let mut vertexes = vec![];
        for lat in (0..lats) {
            let flat = ((-(math.pi) / 2) + ((math.pi / (lats - 1)) * lat));
            let mut points = vec![];
            for lon in (0..lons) {
                let flon = (-(math.pi) + (((2 * math.pi) / (lons - 1)) * lon));
                let x = (math.cos(flat) * math.cos(flon));
                let y = (math.cos(flat) * math.sin(flon));
                let z = math.sin(flat);
                let v = (beta[3] * numpy.array(vec![x, y, z]));
                points.push(v);
            }
            if lastPoints {
                let mut l_lp = lastPoints[0];
                let mut l_p = points[0];
                for i in (1..points.len()) {
                    let lp = lastPoints[i];
                    let p = points[i];
                    vertexes += vec![l_lp, l_p, p, lp];
                    l_lp = lp;
                    l_p = p;
                }
            }
            lastPoints = points;
        }
        super(Spherical, self).__init__(vertexes);
    }
}
struct Conical {}

impl Conical {
    fn __init__<T0, T1, T2>(&self, beta: T0, lons: T1, rs: T2) {
        let mut lastPoints = false;
        let mut vertexes = vec![];
        let dip = math.radians(beta[4]);
        for r in (0..rs) {
            let fr = ((beta[3] * r) / rs);
            let mut points = vec![];
            for lon in (0..lons) {
                let flon = (-(math.pi) + (((2 * math.pi) / (lons - 1)) * lon));
                let x = ((fr * math.cos(dip)) * math.cos(flon));
                let y = ((fr * math.cos(dip)) * math.sin(flon));
                let z = (fr * math.sin(dip));
                let v = numpy.array(vec![x, y, z]);
                points.push(v);
            }
            if lastPoints {
                let mut l_lp = lastPoints[0];
                let mut l_p = points[0];
                for i in (1..points.len()) {
                    let lp = lastPoints[i];
                    let p = points[i];
                    vertexes += vec![l_lp, l_p, p, lp];
                    l_lp = lp;
                    l_p = p;
                }
            }
            lastPoints = points;
        }
        super(Conical, self).__init__(vertexes);
    }
}
struct Plane {}

impl Plane {
    fn __init__<T0, T1, RT>(&self, plane_fit: T0, gridsize: T1) -> RT {
        let plane = numpy.array(plane_fit);
        let origin = (-(plane) / numpy.dot(plane, plane));
        let n = numpy.array(vec![plane[1], plane[2], plane[0]]);
        let u = numpy.cross(plane, n);
        let v = numpy.cross(plane, u);
        u /= numpy.linalg.norm(u);
        v /= numpy.linalg.norm(v);
        fn project_point<T0, RT>(point: T0) -> RT {
            return ((origin + (point[0] * u)) + (point[1] * v));
        }
        let mut vertexes = vec![];
        for x in ((-(gridsize) + 1)..gridsize) {
            for y in ((-(gridsize) + 1)..gridsize) {
                vertexes += vec![
                    project_point(((x - 1), (y - 1))),
                    project_point((x, (y - 1))),
                    project_point((x, y)),
                    project_point(((x - 1), y)),
                ];
            }
        }
        super(self, Plane).__init__(vertexes);
    }
}
