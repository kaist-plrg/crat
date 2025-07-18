use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type cpFloat = libc::c_double;
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
#[inline]
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    return if f < 0 as libc::c_int as libc::c_double { -f } else { f };
}
static mut cpvzero: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = {
        let mut init = cpVect { x: x, y: y };
        init
    };
    return v;
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
unsafe extern "C" fn cpvdist(v1: cpVect, v2: cpVect) -> cpFloat {
    return cpvlength(cpvsub(v1, v2));
}
pub unsafe extern "C" fn cpMessage(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut isError: libc::c_int,
    mut isHardError: libc::c_int,
    mut message: *const libc::c_char,
    mut args: ...
) {
    fprintf(
        stderr,
        if isError != 0 {
            b"Aborting due to Chipmunk error: \0" as *const u8 as *const libc::c_char
        } else {
            b"Chipmunk warning: \0" as *const u8 as *const libc::c_char
        },
    );
    let mut vargs: ::std::ffi::VaListImpl;
    vargs = args.clone();
    vfprintf(stderr, message, vargs.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"\tFailed condition: %s\n\0" as *const u8 as *const libc::c_char,
        condition,
    );
    fprintf(
        stderr,
        b"\tSource:%s:%d\n\0" as *const u8 as *const libc::c_char,
        file,
        line,
    );
}
pub static mut cpVersionString: *const libc::c_char = b"7.0.3\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn cpMomentForCircle(
    mut m: cpFloat,
    mut r1: cpFloat,
    mut r2: cpFloat,
    mut offset: cpVect,
) -> cpFloat {
    return m * (0.5f32 as libc::c_double * (r1 * r1 + r2 * r2) + cpvlengthsq(offset));
}
pub unsafe extern "C" fn cpAreaForCircle(mut r1: cpFloat, mut r2: cpFloat) -> cpFloat {
    return 3.14159265358979323846264338327950288f64 * cpfabs(r1 * r1 - r2 * r2);
}
pub unsafe extern "C" fn cpMomentForSegment(
    mut m: cpFloat,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> cpFloat {
    let mut offset: cpVect = cpvlerp(a, b, 0.5f32 as cpFloat);
    let mut length: cpFloat = cpvdist(b, a) + 2.0f32 as libc::c_double * r;
    return m
        * ((length * length + 4.0f32 as libc::c_double * r * r)
            / 12.0f32 as libc::c_double + cpvlengthsq(offset));
}
pub unsafe extern "C" fn cpAreaForSegment(
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> cpFloat {
    return r
        * (3.14159265358979323846264338327950288f64 * r
            + 2.0f32 as libc::c_double * cpvdist(a, b));
}
pub unsafe extern "C" fn cpMomentForPoly(
    mut m: cpFloat,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut offset: cpVect,
    mut r: cpFloat,
) -> cpFloat {
    if count == 2 as libc::c_int {
        return cpMomentForSegment(
            m,
            *verts.offset(0 as libc::c_int as isize),
            *verts.offset(1 as libc::c_int as isize),
            0.0f32 as cpFloat,
        );
    }
    let mut sum1: cpFloat = 0.0f32 as cpFloat;
    let mut sum2: cpFloat = 0.0f32 as cpFloat;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut v1: cpVect = cpvadd(*verts.offset(i as isize), offset);
        let mut v2: cpVect = cpvadd(
            *verts.offset(((i + 1 as libc::c_int) % count) as isize),
            offset,
        );
        let mut a: cpFloat = cpvcross(v2, v1);
        let mut b: cpFloat = cpvdot(v1, v1) + cpvdot(v1, v2) + cpvdot(v2, v2);
        sum1 += a * b;
        sum2 += a;
        i += 1;
        i;
    }
    return m * sum1 / (6.0f32 as libc::c_double * sum2);
}
pub unsafe extern "C" fn cpAreaForPoly(
    count: libc::c_int,
    mut verts: *const cpVect,
    mut r: cpFloat,
) -> cpFloat {
    let mut area: cpFloat = 0.0f32 as cpFloat;
    let mut perimeter: cpFloat = 0.0f32 as cpFloat;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut v1: cpVect = *verts.offset(i as isize);
        let mut v2: cpVect = *verts.offset(((i + 1 as libc::c_int) % count) as isize);
        area += cpvcross(v1, v2);
        perimeter += cpvdist(v1, v2);
        i += 1;
        i;
    }
    return r * (3.14159265358979323846264338327950288f64 * cpfabs(r) + perimeter)
        + area / 2.0f32 as libc::c_double;
}
pub unsafe extern "C" fn cpCentroidForPoly(
    count: libc::c_int,
    mut verts: *const cpVect,
) -> cpVect {
    let mut sum: cpFloat = 0.0f32 as cpFloat;
    let mut vsum: cpVect = cpvzero;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let mut v1: cpVect = *verts.offset(i as isize);
        let mut v2: cpVect = *verts.offset(((i + 1 as libc::c_int) % count) as isize);
        let mut cross: cpFloat = cpvcross(v1, v2);
        sum += cross;
        vsum = cpvadd(vsum, cpvmult(cpvadd(v1, v2), cross));
        i += 1;
        i;
    }
    return cpvmult(vsum, 1.0f32 as libc::c_double / (3.0f32 as libc::c_double * sum));
}
pub unsafe extern "C" fn cpMomentForBox(
    mut m: cpFloat,
    mut width: cpFloat,
    mut height: cpFloat,
) -> cpFloat {
    return m * (width * width + height * height) / 12.0f32 as libc::c_double;
}
pub unsafe extern "C" fn cpMomentForBox2(mut m: cpFloat, mut box_0: cpBB) -> cpFloat {
    let mut width: cpFloat = box_0.r - box_0.l;
    let mut height: cpFloat = box_0.t - box_0.b;
    let mut offset: cpVect = cpvmult(
        cpv(box_0.l + box_0.r, box_0.b + box_0.t),
        0.5f32 as cpFloat,
    );
    return cpMomentForBox(m, width, height) + m * cpvlengthsq(offset);
}
pub unsafe extern "C" fn cpLoopIndexes(
    mut verts: *const cpVect,
    mut count: libc::c_int,
    mut start: *mut libc::c_int,
    mut end: *mut libc::c_int,
) {
    *end = 0 as libc::c_int;
    *start = *end;
    let mut min: cpVect = *verts.offset(0 as libc::c_int as isize);
    let mut max: cpVect = min;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < count {
        let mut v: cpVect = *verts.offset(i as isize);
        if v.x < min.x || v.x == min.x && v.y < min.y {
            min = v;
            *start = i;
        } else if v.x > max.x || v.x == max.x && v.y > max.y {
            max = v;
            *end = i;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn QHullPartition(
    mut verts: *mut cpVect,
    mut count: libc::c_int,
    mut a: cpVect,
    mut b: cpVect,
    mut tol: cpFloat,
) -> libc::c_int {
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut max: cpFloat = 0 as libc::c_int as cpFloat;
    let mut pivot: libc::c_int = 0 as libc::c_int;
    let mut delta: cpVect = cpvsub(b, a);
    let mut valueTol: cpFloat = tol * cpvlength(delta);
    let mut head: libc::c_int = 0 as libc::c_int;
    let mut tail: libc::c_int = count - 1 as libc::c_int;
    while head <= tail {
        let mut value: cpFloat = cpvcross(
            cpvsub(*verts.offset(head as isize), a),
            delta,
        );
        if value > valueTol {
            if value > max {
                max = value;
                pivot = head;
            }
            head += 1;
            head;
        } else {
            let mut __TMP__: cpVect = *verts.offset(head as isize);
            *verts.offset(head as isize) = *verts.offset(tail as isize);
            *verts.offset(tail as isize) = __TMP__;
            tail -= 1;
            tail;
        }
    }
    if pivot != 0 as libc::c_int {
        let mut __TMP___0: cpVect = *verts.offset(0 as libc::c_int as isize);
        *verts.offset(0 as libc::c_int as isize) = *verts.offset(pivot as isize);
        *verts.offset(pivot as isize) = __TMP___0;
    }
    return head;
}
unsafe extern "C" fn QHullReduce(
    mut tol: cpFloat,
    mut verts: *mut cpVect,
    mut count: libc::c_int,
    mut a: cpVect,
    mut pivot: cpVect,
    mut b: cpVect,
    mut result: *mut cpVect,
) -> libc::c_int {
    if count < 0 as libc::c_int {
        return 0 as libc::c_int
    } else if count == 0 as libc::c_int {
        *result.offset(0 as libc::c_int as isize) = pivot;
        return 1 as libc::c_int;
    } else {
        let mut left_count: libc::c_int = QHullPartition(verts, count, a, pivot, tol);
        let mut index: libc::c_int = QHullReduce(
            tol,
            verts.offset(1 as libc::c_int as isize),
            left_count - 1 as libc::c_int,
            a,
            *verts.offset(0 as libc::c_int as isize),
            pivot,
            result,
        );
        let fresh0 = index;
        index = index + 1;
        *result.offset(fresh0 as isize) = pivot;
        let mut right_count: libc::c_int = QHullPartition(
            verts.offset(left_count as isize),
            count - left_count,
            pivot,
            b,
            tol,
        );
        return index
            + QHullReduce(
                tol,
                verts.offset(left_count as isize).offset(1 as libc::c_int as isize),
                right_count - 1 as libc::c_int,
                pivot,
                *verts.offset(left_count as isize),
                b,
                result.offset(index as isize),
            );
    };
}
pub unsafe extern "C" fn cpConvexHull(
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut result: *mut cpVect,
    mut first: *mut libc::c_int,
    mut tol: cpFloat,
) -> libc::c_int {
    if verts != result as *const cpVect {
        memcpy(
            result as *mut libc::c_void,
            verts as *const libc::c_void,
            (count as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
        );
    }
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    cpLoopIndexes(verts, count, &mut start, &mut end);
    if start == end {
        if !first.is_null() {
            *first = 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    let mut __TMP__: cpVect = *result.offset(0 as libc::c_int as isize);
    *result.offset(0 as libc::c_int as isize) = *result.offset(start as isize);
    *result.offset(start as isize) = __TMP__;
    let mut __TMP___0: cpVect = *result.offset(1 as libc::c_int as isize);
    *result
        .offset(
            1 as libc::c_int as isize,
        ) = *result.offset((if end == 0 as libc::c_int { start } else { end }) as isize);
    *result
        .offset(
            (if end == 0 as libc::c_int { start } else { end }) as isize,
        ) = __TMP___0;
    let mut a: cpVect = *result.offset(0 as libc::c_int as isize);
    let mut b: cpVect = *result.offset(1 as libc::c_int as isize);
    if !first.is_null() {
        *first = start;
    }
    return QHullReduce(
        tol,
        result.offset(2 as libc::c_int as isize),
        count - 2 as libc::c_int,
        a,
        b,
        a,
        result.offset(1 as libc::c_int as isize),
    ) + 1 as libc::c_int;
}
