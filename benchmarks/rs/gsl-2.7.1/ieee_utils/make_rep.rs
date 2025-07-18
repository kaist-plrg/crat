use ::libc;
pub type C2RustUnnamed = libc::c_uint;
pub const GSL_IEEE_TYPE_ZERO: C2RustUnnamed = 5;
pub const GSL_IEEE_TYPE_DENORMAL: C2RustUnnamed = 4;
pub const GSL_IEEE_TYPE_NORMAL: C2RustUnnamed = 3;
pub const GSL_IEEE_TYPE_INF: C2RustUnnamed = 2;
pub const GSL_IEEE_TYPE_NAN: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_float_rep {
    pub sign: libc::c_int,
    pub mantissa: [libc::c_char; 24],
    pub exponent: libc::c_int,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_double_rep {
    pub sign: libc::c_int,
    pub mantissa: [libc::c_char; 53],
    pub exponent: libc::c_int,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub byte: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub f: libc::c_float,
    pub ieee: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub f: libc::c_float,
    pub b: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub l: libc::c_long,
    pub c: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub byte: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub d: libc::c_double,
    pub ieee: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub d: libc::c_double,
    pub b: [libc::c_uchar; 8],
}
unsafe extern "C" fn little_endian_p() -> libc::c_int {
    let mut u: C2RustUnnamed_3 = C2RustUnnamed_3 { l: 0 };
    u.l = 1 as libc::c_int as libc::c_long;
    return (u
        .c[(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as libc::c_int
        == 1 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn make_float_bigendian(mut x: *mut libc::c_float) {
    let mut u: C2RustUnnamed_2 = C2RustUnnamed_2 { f: 0. };
    let mut v: C2RustUnnamed_2 = C2RustUnnamed_2 { f: 0. };
    u.f = *x;
    v.b[0 as libc::c_int as usize] = u.b[3 as libc::c_int as usize];
    v.b[1 as libc::c_int as usize] = u.b[2 as libc::c_int as usize];
    v.b[2 as libc::c_int as usize] = u.b[1 as libc::c_int as usize];
    v.b[3 as libc::c_int as usize] = u.b[0 as libc::c_int as usize];
    *x = v.f;
}
unsafe extern "C" fn make_double_bigendian(mut x: *mut libc::c_double) {
    let mut u: C2RustUnnamed_6 = C2RustUnnamed_6 { d: 0. };
    let mut v: C2RustUnnamed_6 = C2RustUnnamed_6 { d: 0. };
    u.d = *x;
    v.b[0 as libc::c_int as usize] = u.b[7 as libc::c_int as usize];
    v.b[1 as libc::c_int as usize] = u.b[6 as libc::c_int as usize];
    v.b[2 as libc::c_int as usize] = u.b[5 as libc::c_int as usize];
    v.b[3 as libc::c_int as usize] = u.b[4 as libc::c_int as usize];
    v.b[4 as libc::c_int as usize] = u.b[3 as libc::c_int as usize];
    v.b[5 as libc::c_int as usize] = u.b[2 as libc::c_int as usize];
    v.b[6 as libc::c_int as usize] = u.b[1 as libc::c_int as usize];
    v.b[7 as libc::c_int as usize] = u.b[0 as libc::c_int as usize];
    *x = v.d;
}
pub unsafe extern "C" fn gsl_ieee_float_to_rep(
    mut x: *const libc::c_float,
    mut r: *mut gsl_ieee_float_rep,
) {
    let mut e: libc::c_int = 0;
    let mut non_zero: libc::c_int = 0;
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { f: 0. };
    u.f = *x;
    if little_endian_p() != 0 {
        make_float_bigendian(&mut u.f);
    }
    if u.ieee.byte[3 as libc::c_int as usize] as libc::c_int >> 7 as libc::c_int != 0 {
        (*r).sign = 1 as libc::c_int;
    } else {
        (*r).sign = 0 as libc::c_int;
    }
    e = (u.ieee.byte[3 as libc::c_int as usize] as libc::c_int & 0x7f as libc::c_int)
        << 1 as libc::c_int
        | (u.ieee.byte[2 as libc::c_int as usize] as libc::c_int & 0x80 as libc::c_int)
            >> 7 as libc::c_int;
    (*r).exponent = e - 127 as libc::c_int;
    sprint_byte(
        (u.ieee.byte[2 as libc::c_int as usize] as libc::c_int & 0x7f as libc::c_int)
            << 1 as libc::c_int,
        ((*r).mantissa).as_mut_ptr(),
    );
    sprint_byte(
        u.ieee.byte[1 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(7 as libc::c_int as isize),
    );
    sprint_byte(
        u.ieee.byte[0 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(15 as libc::c_int as isize),
    );
    (*r).mantissa[23 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    non_zero = (u.ieee.byte[0 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[1 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[2 as libc::c_int as usize] as libc::c_int & 0x7f as libc::c_int
            != 0) as libc::c_int;
    (*r).type_0 = determine_ieee_type(non_zero, e, 255 as libc::c_int);
}
pub unsafe extern "C" fn gsl_ieee_double_to_rep(
    mut x: *const libc::c_double,
    mut r: *mut gsl_ieee_double_rep,
) {
    let mut e: libc::c_int = 0;
    let mut non_zero: libc::c_int = 0;
    let mut u: C2RustUnnamed_5 = C2RustUnnamed_5 { d: 0. };
    u.d = *x;
    if little_endian_p() != 0 {
        make_double_bigendian(&mut u.d);
    }
    if u.ieee.byte[7 as libc::c_int as usize] as libc::c_int >> 7 as libc::c_int != 0 {
        (*r).sign = 1 as libc::c_int;
    } else {
        (*r).sign = 0 as libc::c_int;
    }
    e = (u.ieee.byte[7 as libc::c_int as usize] as libc::c_int & 0x7f as libc::c_int)
        << 4 as libc::c_int
        ^ (u.ieee.byte[6 as libc::c_int as usize] as libc::c_int & 0xf0 as libc::c_int)
            >> 4 as libc::c_int;
    (*r).exponent = e - 1023 as libc::c_int;
    sprint_nybble(
        u.ieee.byte[6 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr(),
    );
    sprint_byte(
        u.ieee.byte[5 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(4 as libc::c_int as isize),
    );
    sprint_byte(
        u.ieee.byte[4 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(12 as libc::c_int as isize),
    );
    sprint_byte(
        u.ieee.byte[3 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(20 as libc::c_int as isize),
    );
    sprint_byte(
        u.ieee.byte[2 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(28 as libc::c_int as isize),
    );
    sprint_byte(
        u.ieee.byte[1 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(36 as libc::c_int as isize),
    );
    sprint_byte(
        u.ieee.byte[0 as libc::c_int as usize] as libc::c_int,
        ((*r).mantissa).as_mut_ptr().offset(44 as libc::c_int as isize),
    );
    (*r).mantissa[52 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    non_zero = (u.ieee.byte[0 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[1 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[2 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[3 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[4 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[5 as libc::c_int as usize] as libc::c_int != 0
        || u.ieee.byte[6 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int
            != 0) as libc::c_int;
    (*r).type_0 = determine_ieee_type(non_zero, e, 2047 as libc::c_int);
}
static mut nybble: [[libc::c_char; 5]; 16] = unsafe {
    [
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0000\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0001\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0010\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0011\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0100\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0101\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0110\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"0111\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1000\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1001\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1010\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1011\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1100\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1101\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1110\0"),
        *::std::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"1111\0"),
    ]
};
unsafe extern "C" fn sprint_nybble(mut i: libc::c_int, mut s: *mut libc::c_char) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    c = (nybble[(i & 0xf as libc::c_int) as usize]).as_mut_ptr();
    *s = *c.offset(0 as libc::c_int as isize);
    *s.offset(1 as libc::c_int as isize) = *c.offset(1 as libc::c_int as isize);
    *s.offset(2 as libc::c_int as isize) = *c.offset(2 as libc::c_int as isize);
    *s.offset(3 as libc::c_int as isize) = *c.offset(3 as libc::c_int as isize);
}
unsafe extern "C" fn sprint_byte(mut i: libc::c_int, mut s: *mut libc::c_char) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    c = (nybble[((i & 0xf0 as libc::c_int) >> 4 as libc::c_int) as usize]).as_mut_ptr();
    *s = *c.offset(0 as libc::c_int as isize);
    *s.offset(1 as libc::c_int as isize) = *c.offset(1 as libc::c_int as isize);
    *s.offset(2 as libc::c_int as isize) = *c.offset(2 as libc::c_int as isize);
    *s.offset(3 as libc::c_int as isize) = *c.offset(3 as libc::c_int as isize);
    c = (nybble[(i & 0xf as libc::c_int) as usize]).as_mut_ptr();
    *s.offset(4 as libc::c_int as isize) = *c.offset(0 as libc::c_int as isize);
    *s.offset(5 as libc::c_int as isize) = *c.offset(1 as libc::c_int as isize);
    *s.offset(6 as libc::c_int as isize) = *c.offset(2 as libc::c_int as isize);
    *s.offset(7 as libc::c_int as isize) = *c.offset(3 as libc::c_int as isize);
}
unsafe extern "C" fn determine_ieee_type(
    mut non_zero: libc::c_int,
    mut exponent: libc::c_int,
    mut max_exponent: libc::c_int,
) -> libc::c_int {
    if exponent == max_exponent {
        if non_zero != 0 {
            return GSL_IEEE_TYPE_NAN as libc::c_int
        } else {
            return GSL_IEEE_TYPE_INF as libc::c_int
        }
    } else if exponent == 0 as libc::c_int {
        if non_zero != 0 {
            return GSL_IEEE_TYPE_DENORMAL as libc::c_int
        } else {
            return GSL_IEEE_TYPE_ZERO as libc::c_int
        }
    } else {
        return GSL_IEEE_TYPE_NORMAL as libc::c_int
    };
}
