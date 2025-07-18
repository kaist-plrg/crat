use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type int8_t = __int8_t;
pub type uint64_t = __uint64_t;
pub unsafe extern "C" fn print_ntk_out(
    mut bars_count: libc::c_int,
    mut fd: libc::c_int,
    mut bit_format: libc::c_int,
    mut bar_width: libc::c_int,
    mut bar_spacing: libc::c_int,
    mut bar_height: libc::c_int,
    mut f: *const libc::c_int,
) -> libc::c_int {
    let mut buf_8: int8_t = 0;
    let mut val1: int8_t = 0;
    let mut bits: uint64_t = 0;
    let mut j: int8_t = 0;
    let mut k: int8_t = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < bars_count {
        let mut f_limited: libc::c_int = *f.offset(i as isize);
        if f_limited as libc::c_double
            > pow(2 as libc::c_int as libc::c_double, bit_format as libc::c_double)
                - 1 as libc::c_int as libc::c_double
        {
            f_limited = (pow(
                2 as libc::c_int as libc::c_double,
                bit_format as libc::c_double,
            ) - 1 as libc::c_int as libc::c_double) as libc::c_int;
        }
        val1 = (f_limited >> bar_height / 8 as libc::c_int - 1 as libc::c_int) as int8_t;
        bits = (pow(2 as libc::c_int as libc::c_double, val1 as libc::c_double)
            - 1 as libc::c_int as libc::c_double) as uint64_t;
        k = 0 as libc::c_int as int8_t;
        while (k as libc::c_int) < bar_width {
            j = 0 as libc::c_int as int8_t;
            while (j as libc::c_int) < bar_height / 8 as libc::c_int {
                buf_8 = (bits
                    >> 8 as libc::c_int
                        * (bar_height / 8 as libc::c_int - 1 as libc::c_int
                            - j as libc::c_int) & 0xff as libc::c_int as libc::c_ulong)
                    as int8_t;
                write(
                    fd,
                    &mut buf_8 as *mut int8_t as *const libc::c_void,
                    ::std::mem::size_of::<int8_t>() as libc::c_ulong,
                );
                j += 1;
                j;
            }
            k += 1;
            k;
        }
        buf_8 = 0 as libc::c_int as int8_t;
        j = 0 as libc::c_int as int8_t;
        while (j as libc::c_int) < bar_height / 8 as libc::c_int * bar_spacing {
            write(
                fd,
                &mut buf_8 as *mut int8_t as *const libc::c_void,
                ::std::mem::size_of::<int8_t>() as libc::c_ulong,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
