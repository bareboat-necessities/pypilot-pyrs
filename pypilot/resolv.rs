use std::collections::HashMap;
use std::*;

fn resolv<T0, T1, RT>(angle: T0, offset: T1) -> RT {
    while (offset - angle) > 180 {
        angle += 360;
    }
    while (offset - angle) <= -180 {
        angle -= 360;
    }
    return angle;
}
