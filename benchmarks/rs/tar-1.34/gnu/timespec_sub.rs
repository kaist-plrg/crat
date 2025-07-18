use ::libc;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const TIMESPEC_HZ: C2RustUnnamed = 1000000000;
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: libc::c_long) -> timespec {
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    r.tv_sec = s;
    r.tv_nsec = ns;
    return r;
}
pub unsafe extern "C" fn timespec_sub(mut a: timespec, mut b: timespec) -> timespec {
    let mut current_block: u64;
    let mut rs: time_t = a.tv_sec;
    let mut bs: time_t = b.tv_sec;
    let mut ns: libc::c_int = (a.tv_nsec - b.tv_nsec) as libc::c_int;
    let mut rns: libc::c_int = ns;
    if ns < 0 as libc::c_int {
        rns = ns + TIMESPEC_HZ as libc::c_int;
        let mut bs1: time_t = 0;
        let (fresh0, fresh1) = bs.overflowing_add((1 as libc::c_int).into());
        *(&mut bs1 as *mut time_t) = fresh0;
        if !fresh1 {
            bs = bs1;
            current_block = 11006700562992250127;
        } else if (-(!((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t)
            as libc::c_int) as libc::c_long) < rs
        {
            rs -= 1;
            rs;
            current_block = 11006700562992250127;
        } else {
            current_block = 7373911032283775906;
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            let (fresh2, fresh3) = rs.overflowing_sub(bs);
            *(&mut rs as *mut time_t) = fresh2;
            if fresh3 {
                if (0 as libc::c_int as libc::c_long) < bs {
                    current_block = 7373911032283775906;
                } else {
                    rs = if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t
                    {
                        -(1 as libc::c_int) as time_t
                    } else {
                        (((1 as libc::c_int as time_t)
                            << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    };
                    rns = TIMESPEC_HZ as libc::c_int - 1 as libc::c_int;
                    current_block = 4166486009154926805;
                }
            } else {
                current_block = 4166486009154926805;
            }
        }
        _ => {}
    }
    match current_block {
        7373911032283775906 => {
            rs = !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            };
            rns = 0 as libc::c_int;
        }
        _ => {}
    }
    return make_timespec(rs, rns as libc::c_long);
}
