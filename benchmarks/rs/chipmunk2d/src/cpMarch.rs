use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type cpFloat = libc::c_double;
pub type cpBool = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
pub type cpMarchSampleFunc = Option::<
    unsafe extern "C" fn(cpVect, *mut libc::c_void) -> cpFloat,
>;
pub type cpMarchSegmentFunc = Option::<
    unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> (),
>;
pub type cpMarchCellFunc = Option::<
    unsafe extern "C" fn(
        cpFloat,
        cpFloat,
        cpFloat,
        cpFloat,
        cpFloat,
        cpFloat,
        cpFloat,
        cpFloat,
        cpFloat,
        cpMarchSegmentFunc,
        *mut libc::c_void,
    ) -> (),
>;
#[inline]
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = {
        let mut init = cpVect { x: x, y: y };
        init
    };
    return v;
}
#[inline]
unsafe extern "C" fn cpveql(v1: cpVect, v2: cpVect) -> cpBool {
    return (v1.x == v2.x && v1.y == v2.y) as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn cpflerp(
    mut f1: cpFloat,
    mut f2: cpFloat,
    mut t: cpFloat,
) -> cpFloat {
    return f1 * (1.0f32 as libc::c_double - t) + f2 * t;
}
unsafe extern "C" fn cpMarchCells(
    mut bb: cpBB,
    mut x_samples: libc::c_ulong,
    mut y_samples: libc::c_ulong,
    mut t: cpFloat,
    mut segment: cpMarchSegmentFunc,
    mut segment_data: *mut libc::c_void,
    mut sample: cpMarchSampleFunc,
    mut sample_data: *mut libc::c_void,
    mut cell: cpMarchCellFunc,
) {
    let mut x_denom: cpFloat = 1.0f64
        / x_samples.wrapping_sub(1 as libc::c_int as libc::c_ulong) as cpFloat;
    let mut y_denom: cpFloat = 1.0f64
        / y_samples.wrapping_sub(1 as libc::c_int as libc::c_ulong) as cpFloat;
    let mut buffer: *mut cpFloat = calloc(
        x_samples,
        ::std::mem::size_of::<cpFloat>() as libc::c_ulong,
    ) as *mut cpFloat;
    let mut i: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while i < x_samples {
        *buffer
            .offset(
                i as isize,
            ) = sample
            .unwrap()(
            cpv(cpflerp(bb.l, bb.r, i as libc::c_double * x_denom), bb.b),
            sample_data,
        );
        i = i.wrapping_add(1);
        i;
    }
    let mut j: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while j < y_samples.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut y0: cpFloat = cpflerp(
            bb.b,
            bb.t,
            j.wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_double * y_denom,
        );
        let mut y1: cpFloat = cpflerp(
            bb.b,
            bb.t,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double * y_denom,
        );
        let mut a: cpFloat = 0.;
        let mut b: cpFloat = *buffer.offset(0 as libc::c_int as isize);
        let mut c: cpFloat = 0.;
        let mut d: cpFloat = sample.unwrap()(cpv(bb.l, y1), sample_data);
        *buffer.offset(0 as libc::c_int as isize) = d;
        let mut i_0: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        while i_0 < x_samples.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut x0: cpFloat = cpflerp(
                bb.l,
                bb.r,
                i_0.wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_double
                    * x_denom,
            );
            let mut x1: cpFloat = cpflerp(
                bb.l,
                bb.r,
                i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * x_denom,
            );
            a = b;
            b = *buffer
                .offset(i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            c = d;
            d = sample.unwrap()(cpv(x1, y1), sample_data);
            *buffer
                .offset(
                    i_0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = d;
            cell.unwrap()(t, a, b, c, d, x0, x1, y0, y1, segment, segment_data);
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        j = j.wrapping_add(1);
        j;
    }
    free(buffer as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn seg(
    mut v0: cpVect,
    mut v1: cpVect,
    mut f: cpMarchSegmentFunc,
    mut data: *mut libc::c_void,
) {
    if cpveql(v0, v1) == 0 {
        f.unwrap()(v1, v0, data);
    }
}
#[inline]
unsafe extern "C" fn midlerp(
    mut x0: cpFloat,
    mut x1: cpFloat,
    mut s0: cpFloat,
    mut s1: cpFloat,
    mut t: cpFloat,
) -> cpFloat {
    return cpflerp(x0, x1, (t - s0) / (s1 - s0));
}
unsafe extern "C" fn cpMarchCellSoft(
    mut t: cpFloat,
    mut a: cpFloat,
    mut b: cpFloat,
    mut c: cpFloat,
    mut d: cpFloat,
    mut x0: cpFloat,
    mut x1: cpFloat,
    mut y0: cpFloat,
    mut y1: cpFloat,
    mut segment: cpMarchSegmentFunc,
    mut segment_data: *mut libc::c_void,
) {
    match ((a > t) as libc::c_int) << 0 as libc::c_int
        | ((b > t) as libc::c_int) << 1 as libc::c_int
        | ((c > t) as libc::c_int) << 2 as libc::c_int
        | ((d > t) as libc::c_int) << 3 as libc::c_int
    {
        1 => {
            seg(
                cpv(x0, midlerp(y0, y1, a, c, t)),
                cpv(midlerp(x0, x1, a, b, t), y0),
                segment,
                segment_data,
            );
        }
        2 => {
            seg(
                cpv(midlerp(x0, x1, a, b, t), y0),
                cpv(x1, midlerp(y0, y1, b, d, t)),
                segment,
                segment_data,
            );
        }
        3 => {
            seg(
                cpv(x0, midlerp(y0, y1, a, c, t)),
                cpv(x1, midlerp(y0, y1, b, d, t)),
                segment,
                segment_data,
            );
        }
        4 => {
            seg(
                cpv(midlerp(x0, x1, c, d, t), y1),
                cpv(x0, midlerp(y0, y1, a, c, t)),
                segment,
                segment_data,
            );
        }
        5 => {
            seg(
                cpv(midlerp(x0, x1, c, d, t), y1),
                cpv(midlerp(x0, x1, a, b, t), y0),
                segment,
                segment_data,
            );
        }
        6 => {
            seg(
                cpv(midlerp(x0, x1, a, b, t), y0),
                cpv(x1, midlerp(y0, y1, b, d, t)),
                segment,
                segment_data,
            );
            seg(
                cpv(midlerp(x0, x1, c, d, t), y1),
                cpv(x0, midlerp(y0, y1, a, c, t)),
                segment,
                segment_data,
            );
        }
        7 => {
            seg(
                cpv(midlerp(x0, x1, c, d, t), y1),
                cpv(x1, midlerp(y0, y1, b, d, t)),
                segment,
                segment_data,
            );
        }
        8 => {
            seg(
                cpv(x1, midlerp(y0, y1, b, d, t)),
                cpv(midlerp(x0, x1, c, d, t), y1),
                segment,
                segment_data,
            );
        }
        9 => {
            seg(
                cpv(x0, midlerp(y0, y1, a, c, t)),
                cpv(midlerp(x0, x1, a, b, t), y0),
                segment,
                segment_data,
            );
            seg(
                cpv(x1, midlerp(y0, y1, b, d, t)),
                cpv(midlerp(x0, x1, c, d, t), y1),
                segment,
                segment_data,
            );
        }
        10 => {
            seg(
                cpv(midlerp(x0, x1, a, b, t), y0),
                cpv(midlerp(x0, x1, c, d, t), y1),
                segment,
                segment_data,
            );
        }
        11 => {
            seg(
                cpv(x0, midlerp(y0, y1, a, c, t)),
                cpv(midlerp(x0, x1, c, d, t), y1),
                segment,
                segment_data,
            );
        }
        12 => {
            seg(
                cpv(x1, midlerp(y0, y1, b, d, t)),
                cpv(x0, midlerp(y0, y1, a, c, t)),
                segment,
                segment_data,
            );
        }
        13 => {
            seg(
                cpv(x1, midlerp(y0, y1, b, d, t)),
                cpv(midlerp(x0, x1, a, b, t), y0),
                segment,
                segment_data,
            );
        }
        14 => {
            seg(
                cpv(midlerp(x0, x1, a, b, t), y0),
                cpv(x0, midlerp(y0, y1, a, c, t)),
                segment,
                segment_data,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn cpMarchSoft(
    mut bb: cpBB,
    mut x_samples: libc::c_ulong,
    mut y_samples: libc::c_ulong,
    mut t: cpFloat,
    mut segment: cpMarchSegmentFunc,
    mut segment_data: *mut libc::c_void,
    mut sample: cpMarchSampleFunc,
    mut sample_data: *mut libc::c_void,
) {
    cpMarchCells(
        bb,
        x_samples,
        y_samples,
        t,
        segment,
        segment_data,
        sample,
        sample_data,
        Some(
            cpMarchCellSoft
                as unsafe extern "C" fn(
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpMarchSegmentFunc,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
#[inline]
unsafe extern "C" fn segs(
    mut a: cpVect,
    mut b: cpVect,
    mut c: cpVect,
    mut f: cpMarchSegmentFunc,
    mut data: *mut libc::c_void,
) {
    seg(b, c, f, data);
    seg(a, b, f, data);
}
unsafe extern "C" fn cpMarchCellHard(
    mut t: cpFloat,
    mut a: cpFloat,
    mut b: cpFloat,
    mut c: cpFloat,
    mut d: cpFloat,
    mut x0: cpFloat,
    mut x1: cpFloat,
    mut y0: cpFloat,
    mut y1: cpFloat,
    mut segment: cpMarchSegmentFunc,
    mut segment_data: *mut libc::c_void,
) {
    let mut xm: cpFloat = cpflerp(x0, x1, 0.5f32 as cpFloat);
    let mut ym: cpFloat = cpflerp(y0, y1, 0.5f32 as cpFloat);
    match ((a > t) as libc::c_int) << 0 as libc::c_int
        | ((b > t) as libc::c_int) << 1 as libc::c_int
        | ((c > t) as libc::c_int) << 2 as libc::c_int
        | ((d > t) as libc::c_int) << 3 as libc::c_int
    {
        1 => {
            segs(cpv(x0, ym), cpv(xm, ym), cpv(xm, y0), segment, segment_data);
        }
        2 => {
            segs(cpv(xm, y0), cpv(xm, ym), cpv(x1, ym), segment, segment_data);
        }
        3 => {
            seg(cpv(x0, ym), cpv(x1, ym), segment, segment_data);
        }
        4 => {
            segs(cpv(xm, y1), cpv(xm, ym), cpv(x0, ym), segment, segment_data);
        }
        5 => {
            seg(cpv(xm, y1), cpv(xm, y0), segment, segment_data);
        }
        6 => {
            segs(cpv(xm, y0), cpv(xm, ym), cpv(x0, ym), segment, segment_data);
            segs(cpv(xm, y1), cpv(xm, ym), cpv(x1, ym), segment, segment_data);
        }
        7 => {
            segs(cpv(xm, y1), cpv(xm, ym), cpv(x1, ym), segment, segment_data);
        }
        8 => {
            segs(cpv(x1, ym), cpv(xm, ym), cpv(xm, y1), segment, segment_data);
        }
        9 => {
            segs(cpv(x1, ym), cpv(xm, ym), cpv(xm, y0), segment, segment_data);
            segs(cpv(x0, ym), cpv(xm, ym), cpv(xm, y1), segment, segment_data);
        }
        10 => {
            seg(cpv(xm, y0), cpv(xm, y1), segment, segment_data);
        }
        11 => {
            segs(cpv(x0, ym), cpv(xm, ym), cpv(xm, y1), segment, segment_data);
        }
        12 => {
            seg(cpv(x1, ym), cpv(x0, ym), segment, segment_data);
        }
        13 => {
            segs(cpv(x1, ym), cpv(xm, ym), cpv(xm, y0), segment, segment_data);
        }
        14 => {
            segs(cpv(xm, y0), cpv(xm, ym), cpv(x0, ym), segment, segment_data);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn cpMarchHard(
    mut bb: cpBB,
    mut x_samples: libc::c_ulong,
    mut y_samples: libc::c_ulong,
    mut t: cpFloat,
    mut segment: cpMarchSegmentFunc,
    mut segment_data: *mut libc::c_void,
    mut sample: cpMarchSampleFunc,
    mut sample_data: *mut libc::c_void,
) {
    cpMarchCells(
        bb,
        x_samples,
        y_samples,
        t,
        segment,
        segment_data,
        sample,
        sample_data,
        Some(
            cpMarchCellHard
                as unsafe extern "C" fn(
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpMarchSegmentFunc,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
