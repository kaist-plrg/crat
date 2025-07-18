use ::libc;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub unsafe extern "C" fn _glp_zlib_adler32(
    mut adler: uLong,
    mut buf: *const Bytef,
    mut len: uInt,
) -> uLong {
    let mut sum2: libc::c_ulong = 0;
    let mut n: libc::c_uint = 0;
    sum2 = adler >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong;
    adler &= 0xffff as libc::c_int as libc::c_ulong;
    if len == 1 as libc::c_int as libc::c_uint {
        adler = (adler as libc::c_ulong)
            .wrapping_add(*buf.offset(0 as libc::c_int as isize) as libc::c_ulong)
            as uLong as uLong;
        if adler >= 65521 as libc::c_ulong {
            adler = (adler as libc::c_ulong).wrapping_sub(65521 as libc::c_ulong)
                as uLong as uLong;
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= 65521 as libc::c_ulong {
            sum2 = sum2.wrapping_sub(65521 as libc::c_ulong);
        }
        return adler | sum2 << 16 as libc::c_int;
    }
    if buf.is_null() {
        return 1 as libc::c_long as uLong;
    }
    if len < 16 as libc::c_int as libc::c_uint {
        loop {
            let fresh0 = len;
            len = len.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = buf;
            buf = buf.offset(1);
            adler = (adler as libc::c_ulong).wrapping_add(*fresh1 as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
        }
        if adler >= 65521 as libc::c_ulong {
            adler = (adler as libc::c_ulong).wrapping_sub(65521 as libc::c_ulong)
                as uLong as uLong;
        }
        sum2 = sum2.wrapping_rem(65521 as libc::c_ulong);
        return adler | sum2 << 16 as libc::c_int;
    }
    while len >= 5552 as libc::c_int as libc::c_uint {
        len = (len as libc::c_uint).wrapping_sub(5552 as libc::c_int as libc::c_uint)
            as uInt as uInt;
        n = (5552 as libc::c_int / 16 as libc::c_int) as libc::c_uint;
        loop {
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(0 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((0 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int
                                + 1 as libc::c_int) as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(8 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((8 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((8 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((8 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int
                                + 1 as libc::c_int) as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            buf = buf.offset(16 as libc::c_int as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        adler = (adler as libc::c_ulong).wrapping_rem(65521 as libc::c_ulong) as uLong
            as uLong;
        sum2 = sum2.wrapping_rem(65521 as libc::c_ulong);
    }
    if len != 0 {
        while len >= 16 as libc::c_int as libc::c_uint {
            len = (len as libc::c_uint).wrapping_sub(16 as libc::c_int as libc::c_uint)
                as uInt as uInt;
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(0 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((0 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int
                                + 1 as libc::c_int) as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(8 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((8 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((8 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf.offset((8 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int)
                                as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(
                    *buf
                        .offset(
                            (8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int
                                + 1 as libc::c_int) as isize,
                        ) as libc::c_ulong,
                ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            buf = buf.offset(16 as libc::c_int as isize);
        }
        loop {
            let fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            let fresh3 = buf;
            buf = buf.offset(1);
            adler = (adler as libc::c_ulong).wrapping_add(*fresh3 as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
        }
        adler = (adler as libc::c_ulong).wrapping_rem(65521 as libc::c_ulong) as uLong
            as uLong;
        sum2 = sum2.wrapping_rem(65521 as libc::c_ulong);
    }
    return adler | sum2 << 16 as libc::c_int;
}
unsafe extern "C" fn adler32_combine_(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: libc::c_long,
) -> uLong {
    let mut sum1: libc::c_ulong = 0;
    let mut sum2: libc::c_ulong = 0;
    let mut rem: libc::c_uint = 0;
    rem = (len2 as libc::c_ulong).wrapping_rem(65521 as libc::c_ulong) as libc::c_uint;
    sum1 = adler1 & 0xffff as libc::c_int as libc::c_ulong;
    sum2 = (rem as libc::c_ulong).wrapping_mul(sum1);
    sum2 = sum2.wrapping_rem(65521 as libc::c_ulong);
    sum1 = sum1
        .wrapping_add(
            (adler2 & 0xffff as libc::c_int as libc::c_ulong)
                .wrapping_add(65521 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    sum2 = sum2
        .wrapping_add(
            (adler1 >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    adler2 >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong,
                )
                .wrapping_add(65521 as libc::c_ulong)
                .wrapping_sub(rem as libc::c_ulong),
        );
    if sum1 >= 65521 as libc::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as libc::c_ulong);
    }
    if sum1 >= 65521 as libc::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as libc::c_ulong);
    }
    if sum2 >= (65521 as libc::c_ulong) << 1 as libc::c_int {
        sum2 = sum2.wrapping_sub((65521 as libc::c_ulong) << 1 as libc::c_int);
    }
    if sum2 >= 65521 as libc::c_ulong {
        sum2 = sum2.wrapping_sub(65521 as libc::c_ulong);
    }
    return sum1 | sum2 << 16 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_adler32_combine(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: libc::c_long,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
pub unsafe extern "C" fn _glp_zlib_adler32_combine64(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: libc::c_long,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
