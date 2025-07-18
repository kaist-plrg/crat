use ::libc;
static mut proot: libc::c_int = 0x11d as libc::c_int;
static mut alpha: [libc::c_uchar; 256] = [0; 256];
static mut aindex: [libc::c_uchar; 256] = [0; 256];
pub unsafe extern "C" fn RSECC_decoder_init() {
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    alpha[(((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_uchar;
    aindex[0 as libc::c_int
        as usize] = (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
        as libc::c_uchar;
    b = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int {
        alpha[i as usize] = b as libc::c_uchar;
        aindex[b as usize] = i as libc::c_uchar;
        b <<= 1 as libc::c_int;
        if b
            & ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
                + 1 as libc::c_int != 0
        {
            b ^= proot;
        }
        b &= ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn RSECC_decoder_checkSyndrome(
    mut dl: libc::c_int,
    mut data: *mut libc::c_uchar,
    mut el: libc::c_int,
    mut ecc: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < el {
        s = *data.offset(0 as libc::c_int as isize) as libc::c_int;
        j = 1 as libc::c_int;
        while j < dl {
            if s == 0 as libc::c_int {
                s = *data.offset(j as isize) as libc::c_int;
            } else {
                s = *data.offset(j as isize) as libc::c_int
                    ^ alpha[((aindex[s as usize] as libc::c_int + i)
                        % (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int))
                        as usize] as libc::c_int;
            }
            j += 1;
            j;
        }
        j = 0 as libc::c_int;
        while j < el {
            if s == 0 as libc::c_int {
                s = *ecc.offset(j as isize) as libc::c_int;
            } else {
                s = *ecc.offset(j as isize) as libc::c_int
                    ^ alpha[((aindex[s as usize] as libc::c_int + i)
                        % (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int))
                        as usize] as libc::c_int;
            }
            j += 1;
            j;
        }
        if s != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
