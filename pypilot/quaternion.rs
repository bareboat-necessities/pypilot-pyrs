use std::collections::HashMap;
use std::*;

use pypilot::vector;

fn angvec2quat<T0, T1, RT>(angle: T0, v: T1) -> RT {
    let n = vector::norm(v);
    if n == 0 {
        fac = 0;
    } else {
        fac = (math.sin((angle / 2)) / n);
    }
    return vec![
        math.cos((angle / 2)),
        (v[0] * fac),
        (v[1] * fac),
        (v[2] * fac),
    ];
}

fn angle<T0, RT>(q: T0) -> RT {
    return (2 * math.acos(q[0]));
}

fn vec2vec2quat<T0, T1, RT>(a: T0, b: T1) -> RT {
    let n = vector::cross(a, b);
    let mut fac = ((vector::dot(a, b) / vector::norm(a)) / vector::norm(b));
    fac = fac.iter().max().unwrap().iter().min().unwrap();
    let ang = math.acos(fac);
    return angvec2quat(ang, n);
}

fn multiply<T0, T1, RT>(q1: T0, q2: T1) -> RT {
    return vec![
        ((((q1[0] * q2[0]) - (q1[1] * q2[1])) - (q1[2] * q2[2])) - (q1[3] * q2[3])),
        ((((q1[0] * q2[1]) + (q1[1] * q2[0])) + (q1[2] * q2[3])) - (q1[3] * q2[2])),
        ((((q1[0] * q2[2]) - (q1[1] * q2[3])) + (q1[2] * q2[0])) + (q1[3] * q2[1])),
        ((((q1[0] * q2[3]) + (q1[1] * q2[2])) - (q1[2] * q2[1])) + (q1[3] * q2[0])),
    ];
}

fn rotvecquat<T0, T1, RT>(v: T0, q: T1) -> RT {
    let w = vec![0, v[0], v[1], v[2]];
    let r = vec![q[0], -(q[1]), -(q[2]), -(q[3])];
    return multiply(multiply(q, w), r)[1..];
}

fn toeuler<T0, RT>(q: T0) -> RT {
    let roll = math.atan2(
        (2.0 * ((q[2] * q[3]) + (q[0] * q[1]))),
        (1 - (2.0 * ((q[1] * q[1]) + (q[2] * q[2])))),
    );
    let pitch = math.asin(
        (2.0 * ((q[0] * q[2]) - (q[1] * q[3])))
            .iter()
            .max()
            .unwrap()
            .iter()
            .min()
            .unwrap(),
    );
    let heading = math.atan2(
        (2.0 * ((q[1] * q[2]) + (q[0] * q[3]))),
        (1 - (2.0 * ((q[2] * q[2]) + (q[3] * q[3])))),
    );
    return (roll, pitch, heading);
}

fn conjugate<T0, RT>(q: T0) -> RT {
    return vec![q[0], -(q[1]), -(q[2]), -(q[3])];
}

fn normalize<T0, RT>(q: T0) -> RT {
    let mut total = 0;
    for v in q {
        total += (v * v);
    }
    let d = math.sqrt(total);
    return vec![(q[0] / d), (q[1] / d), (q[2] / d), (q[3] / d)];
}
