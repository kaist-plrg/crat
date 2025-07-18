use ::libc;
extern "C" {
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
}
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __ssize_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub unsafe extern "C" fn print_raw_out(
    mut bars_count: libc::c_int,
    mut fd: libc::c_int,
    mut is_binary: libc::c_int,
    mut bit_format: libc::c_int,
    mut ascii_range: libc::c_int,
    mut bar_delim: libc::c_char,
    mut frame_delim: libc::c_char,
    mut f: *const libc::c_int,
) -> libc::c_int {
    let mut buf_16: int16_t = 0;
    let mut buf_8: int8_t = 0;
    if is_binary != 0 {
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
            match bit_format {
                16 => {
                    buf_16 = f_limited as int16_t;
                    write(
                        fd,
                        &mut buf_16 as *mut int16_t as *const libc::c_void,
                        ::std::mem::size_of::<int16_t>() as libc::c_ulong,
                    );
                }
                8 => {
                    buf_8 = f_limited as int8_t;
                    write(
                        fd,
                        &mut buf_8 as *mut int8_t as *const libc::c_void,
                        ::std::mem::size_of::<int8_t>() as libc::c_ulong,
                    );
                }
                _ => {}
            }
            i += 1;
            i;
        }
    } else {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < bars_count {
            let mut f_ranged: libc::c_int = *f.offset(i_0 as isize);
            if f_ranged > ascii_range {
                f_ranged = ascii_range;
            }
            let mut bar_height_size: libc::c_int = 2 as libc::c_int;
            if f_ranged != 0 as libc::c_int {
                bar_height_size = (bar_height_size as libc::c_double
                    + floor(log10(f_ranged as libc::c_double))) as libc::c_int;
            }
            let vla = bar_height_size as usize;
            let mut bar_height: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
            snprintf(
                bar_height.as_mut_ptr(),
                bar_height_size as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                f_ranged,
            );
            write(
                fd,
                bar_height.as_mut_ptr() as *const libc::c_void,
                (bar_height_size - 1 as libc::c_int) as size_t,
            );
            write(
                fd,
                &mut bar_delim as *mut libc::c_char as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            );
            i_0 += 1;
            i_0;
        }
        write(
            fd,
            &mut frame_delim as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
