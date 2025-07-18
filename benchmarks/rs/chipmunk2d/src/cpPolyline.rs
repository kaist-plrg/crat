use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn cpConvexHull(
        count: libc::c_int,
        verts: *const cpVect,
        result: *mut cpVect,
        first: *mut libc::c_int,
        tol: cpFloat,
    ) -> libc::c_int;
    fn cpLoopIndexes(
        verts: *const cpVect,
        count: libc::c_int,
        start: *mut libc::c_int,
        end: *mut libc::c_int,
    );
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
pub struct cpPolyline {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub verts: [cpVect; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPolylineSet {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub lines: *mut *mut cpPolyline,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Notch {
    pub i: libc::c_int,
    pub d: cpFloat,
    pub v: cpVect,
    pub n: cpVect,
}
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
unsafe extern "C" fn cpvadd(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x + v2.x, v1.y + v2.y);
}
#[inline]
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x - v2.x, v1.y - v2.y);
}
#[inline]
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvcross(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.y - v1.y * v2.x;
}
#[inline]
unsafe extern "C" fn cpvperp(v: cpVect) -> cpVect {
    return cpv(-v.y, v.x);
}
#[inline]
unsafe extern "C" fn cpvrperp(v: cpVect) -> cpVect {
    return cpv(v.y, -v.x);
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    return cpvdot(v, v);
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
}
#[inline]
unsafe extern "C" fn cpvlerp(v1: cpVect, v2: cpVect, t: cpFloat) -> cpVect {
    return cpvadd(cpvmult(v1, 1.0f32 as libc::c_double - t), cpvmult(v2, t));
}
#[inline]
unsafe extern "C" fn cpvnormalize(v: cpVect) -> cpVect {
    return cpvmult(
        v,
        1.0f32 as libc::c_double / (cpvlength(v) + 2.2250738585072014e-308f64),
    );
}
#[inline]
unsafe extern "C" fn cpvdist(v1: cpVect, v2: cpVect) -> cpFloat {
    return cpvlength(cpvsub(v1, v2));
}
#[inline]
unsafe extern "C" fn cpvdistsq(v1: cpVect, v2: cpVect) -> cpFloat {
    return cpvlengthsq(cpvsub(v1, v2));
}
#[inline]
unsafe extern "C" fn cpvnear(v1: cpVect, v2: cpVect, dist: cpFloat) -> cpBool {
    return (cpvdistsq(v1, v2) < dist * dist) as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn Next(mut i: libc::c_int, mut count: libc::c_int) -> libc::c_int {
    return (i + 1 as libc::c_int) % count;
}
unsafe extern "C" fn cpPolylineSizeForCapacity(
    mut capacity: libc::c_int,
) -> libc::c_int {
    return (::std::mem::size_of::<cpPolyline>() as libc::c_ulong)
        .wrapping_add(
            (capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
        ) as libc::c_int;
}
unsafe extern "C" fn cpPolylineMake(mut capacity: libc::c_int) -> *mut cpPolyline {
    capacity = if capacity > 16 as libc::c_int { capacity } else { 16 as libc::c_int };
    let mut line: *mut cpPolyline = calloc(
        1 as libc::c_int as libc::c_ulong,
        cpPolylineSizeForCapacity(capacity) as libc::c_ulong,
    ) as *mut cpPolyline;
    (*line).count = 0 as libc::c_int;
    (*line).capacity = capacity;
    return line;
}
unsafe extern "C" fn cpPolylineMake2(
    mut capacity: libc::c_int,
    mut a: cpVect,
    mut b: cpVect,
) -> *mut cpPolyline {
    let mut line: *mut cpPolyline = cpPolylineMake(capacity);
    (*line).count = 2 as libc::c_int;
    *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize) = a;
    *((*line).verts).as_mut_ptr().offset(1 as libc::c_int as isize) = b;
    return line;
}
unsafe extern "C" fn cpPolylineShrink(mut line: *mut cpPolyline) -> *mut cpPolyline {
    (*line).capacity = (*line).count;
    return realloc(
        line as *mut libc::c_void,
        cpPolylineSizeForCapacity((*line).count) as libc::c_ulong,
    ) as *mut cpPolyline;
}
pub unsafe extern "C" fn cpPolylineFree(mut line: *mut cpPolyline) {
    free(line as *mut libc::c_void);
}
unsafe extern "C" fn cpPolylineGrow(
    mut line: *mut cpPolyline,
    mut count: libc::c_int,
) -> *mut cpPolyline {
    (*line).count += count;
    let mut capacity: libc::c_int = (*line).capacity;
    while (*line).count > capacity {
        capacity *= 2 as libc::c_int;
    }
    if (*line).capacity < capacity {
        (*line).capacity = capacity;
        line = realloc(
            line as *mut libc::c_void,
            cpPolylineSizeForCapacity(capacity) as libc::c_ulong,
        ) as *mut cpPolyline;
    }
    return line;
}
unsafe extern "C" fn cpPolylinePush(
    mut line: *mut cpPolyline,
    mut v: cpVect,
) -> *mut cpPolyline {
    let mut count: libc::c_int = (*line).count;
    line = cpPolylineGrow(line, 1 as libc::c_int);
    *((*line).verts).as_mut_ptr().offset(count as isize) = v;
    return line;
}
unsafe extern "C" fn cpPolylineEnqueue(
    mut line: *mut cpPolyline,
    mut v: cpVect,
) -> *mut cpPolyline {
    let mut count: libc::c_int = (*line).count;
    line = cpPolylineGrow(line, 1 as libc::c_int);
    memmove(
        ((*line).verts).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        ((*line).verts).as_mut_ptr() as *const libc::c_void,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
    );
    *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize) = v;
    return line;
}
pub unsafe extern "C" fn cpPolylineIsClosed(mut line: *mut cpPolyline) -> cpBool {
    return ((*line).count > 1 as libc::c_int
        && cpveql(
            *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
            *((*line).verts)
                .as_mut_ptr()
                .offset(((*line).count - 1 as libc::c_int) as isize),
        ) as libc::c_int != 0) as libc::c_int as cpBool;
}
unsafe extern "C" fn cpPolylineIsShort(
    mut points: *mut cpVect,
    mut count: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut min: cpFloat,
) -> cpBool {
    let mut length: cpFloat = 0.0f32 as cpFloat;
    let mut i: libc::c_int = start;
    while i != end {
        length
            += cpvdist(
                *points.offset(i as isize),
                *points.offset(Next(i, count) as isize),
            );
        if length > min {
            return 0 as libc::c_int as cpBool;
        }
        i = Next(i, count);
    }
    return 1 as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn Sharpness(mut a: cpVect, mut b: cpVect, mut c: cpVect) -> cpFloat {
    return cpvdot(cpvnormalize(cpvsub(a, b)), cpvnormalize(cpvsub(c, b)));
}
pub unsafe extern "C" fn cpPolylineSimplifyVertexes(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    let mut reduced: *mut cpPolyline = cpPolylineMake2(
        0 as libc::c_int,
        *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
        *((*line).verts).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    let mut minSharp: cpFloat = -cos(tol);
    let mut i: libc::c_int = 2 as libc::c_int;
    while i < (*line).count {
        let mut vert: cpVect = *((*line).verts).as_mut_ptr().offset(i as isize);
        let mut sharp: cpFloat = Sharpness(
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 2 as libc::c_int) as isize),
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 1 as libc::c_int) as isize),
            vert,
        );
        if sharp <= minSharp {
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 1 as libc::c_int) as isize) = vert;
        } else {
            reduced = cpPolylinePush(reduced, vert);
        }
        i += 1;
        i;
    }
    if cpPolylineIsClosed(line) as libc::c_int != 0
        && Sharpness(
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 2 as libc::c_int) as isize),
            *((*reduced).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
            *((*reduced).verts).as_mut_ptr().offset(1 as libc::c_int as isize),
        ) < minSharp
    {
        *((*reduced).verts)
            .as_mut_ptr()
            .offset(
                0 as libc::c_int as isize,
            ) = *((*reduced).verts)
            .as_mut_ptr()
            .offset(((*reduced).count - 2 as libc::c_int) as isize);
        (*reduced).count -= 1;
        (*reduced).count;
    }
    return reduced;
}
unsafe extern "C" fn DouglasPeucker(
    mut verts: *mut cpVect,
    mut reduced: *mut cpPolyline,
    mut length: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut min: cpFloat,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    if (end - start + length) % length < 2 as libc::c_int {
        return reduced;
    }
    let mut a: cpVect = *verts.offset(start as isize);
    let mut b: cpVect = *verts.offset(end as isize);
    if cpvnear(a, b, min) as libc::c_int != 0
        && cpPolylineIsShort(verts, length, start, end, min) as libc::c_int != 0
    {
        return reduced;
    }
    let mut max: cpFloat = 0.0f64;
    let mut maxi: libc::c_int = start;
    let mut n: cpVect = cpvnormalize(cpvperp(cpvsub(b, a)));
    let mut d: cpFloat = cpvdot(n, a);
    let mut i: libc::c_int = Next(start, length);
    while i != end {
        let mut dist: cpFloat = fabs(cpvdot(n, *verts.offset(i as isize)) - d);
        if dist > max {
            max = dist;
            maxi = i;
        }
        i = Next(i, length);
    }
    if max > tol {
        reduced = DouglasPeucker(verts, reduced, length, start, maxi, min, tol);
        reduced = cpPolylinePush(reduced, *verts.offset(maxi as isize));
        reduced = DouglasPeucker(verts, reduced, length, maxi, end, min, tol);
    }
    return reduced;
}
pub unsafe extern "C" fn cpPolylineSimplifyCurves(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    let mut reduced: *mut cpPolyline = cpPolylineMake((*line).count);
    let mut min: cpFloat = tol / 2.0f32 as libc::c_double;
    if cpPolylineIsClosed(line) != 0 {
        let mut start: libc::c_int = 0;
        let mut end: libc::c_int = 0;
        cpLoopIndexes(
            ((*line).verts).as_mut_ptr(),
            (*line).count - 1 as libc::c_int,
            &mut start,
            &mut end,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(start as isize),
        );
        reduced = DouglasPeucker(
            ((*line).verts).as_mut_ptr(),
            reduced,
            (*line).count - 1 as libc::c_int,
            start,
            end,
            min,
            tol,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(end as isize),
        );
        reduced = DouglasPeucker(
            ((*line).verts).as_mut_ptr(),
            reduced,
            (*line).count - 1 as libc::c_int,
            end,
            start,
            min,
            tol,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(start as isize),
        );
    } else {
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        reduced = DouglasPeucker(
            ((*line).verts).as_mut_ptr(),
            reduced,
            (*line).count,
            0 as libc::c_int,
            (*line).count - 1 as libc::c_int,
            min,
            tol,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts)
                .as_mut_ptr()
                .offset(((*line).count - 1 as libc::c_int) as isize),
        );
    }
    return cpPolylineShrink(reduced);
}
pub unsafe extern "C" fn cpPolylineSetAlloc() -> *mut cpPolylineSet {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpPolylineSet>() as libc::c_ulong,
    ) as *mut cpPolylineSet;
}
pub unsafe extern "C" fn cpPolylineSetInit(
    mut set: *mut cpPolylineSet,
) -> *mut cpPolylineSet {
    (*set).count = 0 as libc::c_int;
    (*set).capacity = 8 as libc::c_int;
    (*set)
        .lines = calloc(
        (*set).capacity as libc::c_ulong,
        ::std::mem::size_of::<cpPolyline>() as libc::c_ulong,
    ) as *mut *mut cpPolyline;
    return set;
}
pub unsafe extern "C" fn cpPolylineSetNew() -> *mut cpPolylineSet {
    return cpPolylineSetInit(cpPolylineSetAlloc());
}
pub unsafe extern "C" fn cpPolylineSetDestroy(
    mut set: *mut cpPolylineSet,
    mut freePolylines: cpBool,
) {
    if freePolylines != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*set).count {
            cpPolylineFree(*((*set).lines).offset(i as isize));
            i += 1;
            i;
        }
    }
    free((*set).lines as *mut libc::c_void);
}
pub unsafe extern "C" fn cpPolylineSetFree(
    mut set: *mut cpPolylineSet,
    mut freePolylines: cpBool,
) {
    if !set.is_null() {
        cpPolylineSetDestroy(set, freePolylines);
        free(set as *mut libc::c_void);
    }
}
unsafe extern "C" fn cpPolylineSetFindEnds(
    mut set: *mut cpPolylineSet,
    mut v: cpVect,
) -> libc::c_int {
    let mut count: libc::c_int = (*set).count;
    let mut lines: *mut *mut cpPolyline = (*set).lines;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut line: *mut cpPolyline = *lines.offset(i as isize);
        if cpveql(
            *((*line).verts)
                .as_mut_ptr()
                .offset(((*line).count - 1 as libc::c_int) as isize),
            v,
        ) != 0
        {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cpPolylineSetFindStarts(
    mut set: *mut cpPolylineSet,
    mut v: cpVect,
) -> libc::c_int {
    let mut count: libc::c_int = (*set).count;
    let mut lines: *mut *mut cpPolyline = (*set).lines;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        if cpveql(
            *((**lines.offset(i as isize)).verts)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            v,
        ) != 0
        {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cpPolylineSetPush(
    mut set: *mut cpPolylineSet,
    mut line: *mut cpPolyline,
) {
    (*set).count += 1;
    (*set).count;
    if (*set).count > (*set).capacity {
        (*set).capacity *= 2 as libc::c_int;
        (*set)
            .lines = realloc(
            (*set).lines as *mut libc::c_void,
            ((*set).capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpPolyline>() as libc::c_ulong),
        ) as *mut *mut cpPolyline;
    }
    let ref mut fresh0 = *((*set).lines)
        .offset(((*set).count - 1 as libc::c_int) as isize);
    *fresh0 = line;
}
unsafe extern "C" fn cpPolylineSetAdd(
    mut set: *mut cpPolylineSet,
    mut v0: cpVect,
    mut v1: cpVect,
) {
    cpPolylineSetPush(set, cpPolylineMake2(16 as libc::c_int, v0, v1));
}
unsafe extern "C" fn cpPolylineSetJoin(
    mut set: *mut cpPolylineSet,
    mut before: libc::c_int,
    mut after: libc::c_int,
) {
    let mut lbefore: *mut cpPolyline = *((*set).lines).offset(before as isize);
    let mut lafter: *mut cpPolyline = *((*set).lines).offset(after as isize);
    let mut count: libc::c_int = (*lbefore).count;
    lbefore = cpPolylineGrow(lbefore, (*lafter).count);
    memmove(
        ((*lbefore).verts).as_mut_ptr().offset(count as isize) as *mut libc::c_void,
        ((*lafter).verts).as_mut_ptr() as *const libc::c_void,
        ((*lafter).count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
    );
    let ref mut fresh1 = *((*set).lines).offset(before as isize);
    *fresh1 = lbefore;
    (*set).count -= 1;
    (*set).count;
    cpPolylineFree(*((*set).lines).offset(after as isize));
    let ref mut fresh2 = *((*set).lines).offset(after as isize);
    *fresh2 = *((*set).lines).offset((*set).count as isize);
}
pub unsafe extern "C" fn cpPolylineSetCollectSegment(
    mut v0: cpVect,
    mut v1: cpVect,
    mut lines: *mut cpPolylineSet,
) {
    let mut before: libc::c_int = cpPolylineSetFindEnds(lines, v0);
    let mut after: libc::c_int = cpPolylineSetFindStarts(lines, v1);
    if before >= 0 as libc::c_int && after >= 0 as libc::c_int {
        if before == after {
            let ref mut fresh3 = *((*lines).lines).offset(before as isize);
            *fresh3 = cpPolylinePush(*((*lines).lines).offset(before as isize), v1);
        } else {
            cpPolylineSetJoin(lines, before, after);
        }
    } else if before >= 0 as libc::c_int {
        let ref mut fresh4 = *((*lines).lines).offset(before as isize);
        *fresh4 = cpPolylinePush(*((*lines).lines).offset(before as isize), v1);
    } else if after >= 0 as libc::c_int {
        let ref mut fresh5 = *((*lines).lines).offset(after as isize);
        *fresh5 = cpPolylineEnqueue(*((*lines).lines).offset(after as isize), v0);
    } else {
        cpPolylineSetAdd(lines, v0, v1);
    };
}
pub unsafe extern "C" fn cpPolylineToConvexHull(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    let mut hull: *mut cpPolyline = cpPolylineMake((*line).count + 1 as libc::c_int);
    (*hull)
        .count = cpConvexHull(
        (*line).count,
        ((*line).verts).as_mut_ptr(),
        ((*hull).verts).as_mut_ptr(),
        0 as *mut libc::c_int,
        tol,
    );
    hull = cpPolylinePush(
        hull,
        *((*hull).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    return cpPolylineShrink(hull);
}
unsafe extern "C" fn FindSteiner(
    mut count: libc::c_int,
    mut verts: *mut cpVect,
    mut notch: Notch,
) -> cpFloat {
    let mut min: cpFloat = ::std::f32::INFINITY as cpFloat;
    let mut feature: cpFloat = -1.0f64;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < count - 1 as libc::c_int {
        let mut index: libc::c_int = (notch.i + i) % count;
        let mut seg_a: cpVect = *verts.offset(index as isize);
        let mut seg_b: cpVect = *verts.offset(Next(index, count) as isize);
        let mut thing_a: cpFloat = cpvcross(notch.n, cpvsub(seg_a, notch.v));
        let mut thing_b: cpFloat = cpvcross(notch.n, cpvsub(seg_b, notch.v));
        if thing_a * thing_b <= 0.0f64 {
            let mut t: cpFloat = thing_a / (thing_a - thing_b);
            let mut dist: cpFloat = cpvdot(
                notch.n,
                cpvsub(cpvlerp(seg_a, seg_b, t), notch.v),
            );
            if dist >= 0.0f64 && dist <= min {
                min = dist;
                feature = index as libc::c_double + t;
            }
        }
        i += 1;
        i;
    }
    return feature;
}
unsafe extern "C" fn DeepestNotch(
    mut count: libc::c_int,
    mut verts: *mut cpVect,
    mut hullCount: libc::c_int,
    mut hullVerts: *mut cpVect,
    mut first: libc::c_int,
    mut tol: cpFloat,
) -> Notch {
    let mut notch: Notch = {
        let mut init = Notch {
            i: 0,
            d: 0.,
            v: cpVect { x: 0., y: 0. },
            n: cpVect { x: 0., y: 0. },
        };
        init
    };
    let mut j: libc::c_int = Next(first, count);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < hullCount {
        let mut a: cpVect = *hullVerts.offset(i as isize);
        let mut b: cpVect = *hullVerts.offset(Next(i, hullCount) as isize);
        let mut n: cpVect = cpvnormalize(cpvrperp(cpvsub(a, b)));
        let mut d: cpFloat = cpvdot(n, a);
        let mut v: cpVect = *verts.offset(j as isize);
        while cpveql(v, b) == 0 {
            let mut depth: cpFloat = cpvdot(n, v) - d;
            if depth > notch.d {
                notch.d = depth;
                notch.i = j;
                notch.v = v;
                notch.n = n;
            }
            j = Next(j, count);
            v = *verts.offset(j as isize);
        }
        j = Next(j, count);
        i += 1;
        i;
    }
    return notch;
}
#[inline]
unsafe extern "C" fn IMAX(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
unsafe extern "C" fn ApproximateConcaveDecomposition(
    mut verts: *mut cpVect,
    mut count: libc::c_int,
    mut tol: cpFloat,
    mut set: *mut cpPolylineSet,
) {
    let mut first: libc::c_int = 0;
    let mut fresh6 = ::std::vec::from_elem(
        0,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    let mut hullVerts: *mut cpVect = fresh6.leak().as_mut_ptr() as *mut cpVect;
    let mut hullCount: libc::c_int = cpConvexHull(
        count,
        verts,
        hullVerts,
        &mut first,
        0.0f64,
    );
    if hullCount != count {
        let mut notch: Notch = DeepestNotch(
            count,
            verts,
            hullCount,
            hullVerts,
            first,
            tol,
        );
        if notch.d > tol {
            let mut steiner_it: cpFloat = FindSteiner(count, verts, notch);
            if steiner_it >= 0.0f64 {
                let mut steiner_i: libc::c_int = steiner_it as libc::c_int;
                let mut steiner: cpVect = cpvlerp(
                    *verts.offset(steiner_i as isize),
                    *verts.offset(Next(steiner_i, count) as isize),
                    steiner_it - steiner_i as libc::c_double,
                );
                let mut sub1_count: libc::c_int = (steiner_i - notch.i + count) % count
                    + 1 as libc::c_int;
                let mut sub2_count: libc::c_int = count
                    - (steiner_i - notch.i + count) % count;
                let mut fresh7 = ::std::vec::from_elem(
                    0,
                    ((IMAX(sub1_count, sub2_count) + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong)
                        as usize,
                );
                let mut scratch: *mut cpVect = fresh7.leak().as_mut_ptr() as *mut cpVect;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < sub1_count {
                    *scratch
                        .offset(
                            i as isize,
                        ) = *verts.offset(((notch.i + i) % count) as isize);
                    i += 1;
                    i;
                }
                *scratch.offset(sub1_count as isize) = steiner;
                ApproximateConcaveDecomposition(
                    scratch,
                    sub1_count + 1 as libc::c_int,
                    tol,
                    set,
                );
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < sub2_count {
                    *scratch
                        .offset(
                            i_0 as isize,
                        ) = *verts
                        .offset(((steiner_i + 1 as libc::c_int + i_0) % count) as isize);
                    i_0 += 1;
                    i_0;
                }
                *scratch.offset(sub2_count as isize) = steiner;
                ApproximateConcaveDecomposition(
                    scratch,
                    sub2_count + 1 as libc::c_int,
                    tol,
                    set,
                );
                return;
            }
        }
    }
    let mut hull: *mut cpPolyline = cpPolylineMake(hullCount + 1 as libc::c_int);
    memcpy(
        ((*hull).verts).as_mut_ptr() as *mut libc::c_void,
        hullVerts as *const libc::c_void,
        (hullCount as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
    );
    *((*hull).verts)
        .as_mut_ptr()
        .offset(hullCount as isize) = *hullVerts.offset(0 as libc::c_int as isize);
    (*hull).count = hullCount + 1 as libc::c_int;
    cpPolylineSetPush(set, hull);
}
pub unsafe extern "C" fn cpPolylineConvexDecomposition(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolylineSet {
    let mut set: *mut cpPolylineSet = cpPolylineSetNew();
    ApproximateConcaveDecomposition(
        ((*line).verts).as_mut_ptr(),
        (*line).count - 1 as libc::c_int,
        tol,
        set,
    );
    return set;
}
