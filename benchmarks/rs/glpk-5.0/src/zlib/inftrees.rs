use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: libc::c_uchar,
    pub bits: libc::c_uchar,
    pub val: libc::c_ushort,
}
pub type codetype = libc::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
pub static mut _glp_zlib_inflate_copyright: [libc::c_char; 47] = unsafe {
    *::std::mem::transmute::<
        &[u8; 47],
        &[libc::c_char; 47],
    >(b" inflate 1.2.5 Copyright 1995-2010 Mark Adler \0")
};
pub unsafe extern "C" fn _glp_zlib_inflate_table(
    mut type_0: codetype,
    mut lens: *mut libc::c_ushort,
    mut codes: libc::c_uint,
    mut table: *mut *mut code,
    mut bits: *mut libc::c_uint,
    mut work: *mut libc::c_ushort,
) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    let mut sym: libc::c_uint = 0;
    let mut min: libc::c_uint = 0;
    let mut max: libc::c_uint = 0;
    let mut root: libc::c_uint = 0;
    let mut curr: libc::c_uint = 0;
    let mut drop_0: libc::c_uint = 0;
    let mut left: libc::c_int = 0;
    let mut used: libc::c_uint = 0;
    let mut huff: libc::c_uint = 0;
    let mut incr: libc::c_uint = 0;
    let mut fill: libc::c_uint = 0;
    let mut low: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0;
    let mut here: code = code { op: 0, bits: 0, val: 0 };
    let mut next: *mut code = 0 as *mut code;
    let mut base: *const libc::c_ushort = 0 as *const libc::c_ushort;
    let mut extra: *const libc::c_ushort = 0 as *const libc::c_ushort;
    let mut end: libc::c_int = 0;
    let mut count: [libc::c_ushort; 16] = [0; 16];
    let mut offs: [libc::c_ushort; 16] = [0; 16];
    static mut lbase: [libc::c_ushort; 31] = [
        3 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        5 as libc::c_int as libc::c_ushort,
        6 as libc::c_int as libc::c_ushort,
        7 as libc::c_int as libc::c_ushort,
        8 as libc::c_int as libc::c_ushort,
        9 as libc::c_int as libc::c_ushort,
        10 as libc::c_int as libc::c_ushort,
        11 as libc::c_int as libc::c_ushort,
        13 as libc::c_int as libc::c_ushort,
        15 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        23 as libc::c_int as libc::c_ushort,
        27 as libc::c_int as libc::c_ushort,
        31 as libc::c_int as libc::c_ushort,
        35 as libc::c_int as libc::c_ushort,
        43 as libc::c_int as libc::c_ushort,
        51 as libc::c_int as libc::c_ushort,
        59 as libc::c_int as libc::c_ushort,
        67 as libc::c_int as libc::c_ushort,
        83 as libc::c_int as libc::c_ushort,
        99 as libc::c_int as libc::c_ushort,
        115 as libc::c_int as libc::c_ushort,
        131 as libc::c_int as libc::c_ushort,
        163 as libc::c_int as libc::c_ushort,
        195 as libc::c_int as libc::c_ushort,
        227 as libc::c_int as libc::c_ushort,
        258 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
    ];
    static mut lext: [libc::c_ushort; 31] = [
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        73 as libc::c_int as libc::c_ushort,
        195 as libc::c_int as libc::c_ushort,
    ];
    static mut dbase: [libc::c_ushort; 32] = [
        1 as libc::c_int as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        5 as libc::c_int as libc::c_ushort,
        7 as libc::c_int as libc::c_ushort,
        9 as libc::c_int as libc::c_ushort,
        13 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        25 as libc::c_int as libc::c_ushort,
        33 as libc::c_int as libc::c_ushort,
        49 as libc::c_int as libc::c_ushort,
        65 as libc::c_int as libc::c_ushort,
        97 as libc::c_int as libc::c_ushort,
        129 as libc::c_int as libc::c_ushort,
        193 as libc::c_int as libc::c_ushort,
        257 as libc::c_int as libc::c_ushort,
        385 as libc::c_int as libc::c_ushort,
        513 as libc::c_int as libc::c_ushort,
        769 as libc::c_int as libc::c_ushort,
        1025 as libc::c_int as libc::c_ushort,
        1537 as libc::c_int as libc::c_ushort,
        2049 as libc::c_int as libc::c_ushort,
        3073 as libc::c_int as libc::c_ushort,
        4097 as libc::c_int as libc::c_ushort,
        6145 as libc::c_int as libc::c_ushort,
        8193 as libc::c_int as libc::c_ushort,
        12289 as libc::c_int as libc::c_ushort,
        16385 as libc::c_int as libc::c_ushort,
        24577 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
    ];
    static mut dext: [libc::c_ushort; 32] = [
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        16 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        19 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        20 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        21 as libc::c_int as libc::c_ushort,
        22 as libc::c_int as libc::c_ushort,
        22 as libc::c_int as libc::c_ushort,
        23 as libc::c_int as libc::c_ushort,
        23 as libc::c_int as libc::c_ushort,
        24 as libc::c_int as libc::c_ushort,
        24 as libc::c_int as libc::c_ushort,
        25 as libc::c_int as libc::c_ushort,
        25 as libc::c_int as libc::c_ushort,
        26 as libc::c_int as libc::c_ushort,
        26 as libc::c_int as libc::c_ushort,
        27 as libc::c_int as libc::c_ushort,
        27 as libc::c_int as libc::c_ushort,
        28 as libc::c_int as libc::c_ushort,
        28 as libc::c_int as libc::c_ushort,
        29 as libc::c_int as libc::c_ushort,
        29 as libc::c_int as libc::c_ushort,
        64 as libc::c_int as libc::c_ushort,
        64 as libc::c_int as libc::c_ushort,
    ];
    len = 0 as libc::c_int as libc::c_uint;
    while len <= 15 as libc::c_int as libc::c_uint {
        count[len as usize] = 0 as libc::c_int as libc::c_ushort;
        len = len.wrapping_add(1);
        len;
    }
    sym = 0 as libc::c_int as libc::c_uint;
    while sym < codes {
        count[*lens.offset(sym as isize)
            as usize] = (count[*lens.offset(sym as isize) as usize]).wrapping_add(1);
        count[*lens.offset(sym as isize) as usize];
        sym = sym.wrapping_add(1);
        sym;
    }
    root = *bits;
    max = 15 as libc::c_int as libc::c_uint;
    while max >= 1 as libc::c_int as libc::c_uint {
        if count[max as usize] as libc::c_int != 0 as libc::c_int {
            break;
        }
        max = max.wrapping_sub(1);
        max;
    }
    if root > max {
        root = max;
    }
    if max == 0 as libc::c_int as libc::c_uint {
        here.op = 64 as libc::c_int as libc::c_uchar;
        here.bits = 1 as libc::c_int as libc::c_uchar;
        here.val = 0 as libc::c_int as libc::c_ushort;
        let fresh0 = *table;
        *table = (*table).offset(1);
        *fresh0 = here;
        let fresh1 = *table;
        *table = (*table).offset(1);
        *fresh1 = here;
        *bits = 1 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int;
    }
    min = 1 as libc::c_int as libc::c_uint;
    while min < max {
        if count[min as usize] as libc::c_int != 0 as libc::c_int {
            break;
        }
        min = min.wrapping_add(1);
        min;
    }
    if root < min {
        root = min;
    }
    left = 1 as libc::c_int;
    len = 1 as libc::c_int as libc::c_uint;
    while len <= 15 as libc::c_int as libc::c_uint {
        left <<= 1 as libc::c_int;
        left -= count[len as usize] as libc::c_int;
        if left < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        len = len.wrapping_add(1);
        len;
    }
    if left > 0 as libc::c_int
        && (type_0 as libc::c_uint == CODES as libc::c_int as libc::c_uint
            || max != 1 as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    offs[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    len = 1 as libc::c_int as libc::c_uint;
    while len < 15 as libc::c_int as libc::c_uint {
        offs[len.wrapping_add(1 as libc::c_int as libc::c_uint)
            as usize] = (offs[len as usize] as libc::c_int
            + count[len as usize] as libc::c_int) as libc::c_ushort;
        len = len.wrapping_add(1);
        len;
    }
    sym = 0 as libc::c_int as libc::c_uint;
    while sym < codes {
        if *lens.offset(sym as isize) as libc::c_int != 0 as libc::c_int {
            let fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize)
                as usize] = (offs[*lens.offset(sym as isize) as usize]).wrapping_add(1);
            *work.offset(fresh2 as isize) = sym as libc::c_ushort;
        }
        sym = sym.wrapping_add(1);
        sym;
    }
    match type_0 as libc::c_uint {
        0 => {
            extra = work;
            base = extra;
            end = 19 as libc::c_int;
        }
        1 => {
            base = lbase.as_ptr();
            base = base.offset(-(257 as libc::c_int as isize));
            extra = lext.as_ptr();
            extra = extra.offset(-(257 as libc::c_int as isize));
            end = 256 as libc::c_int;
        }
        _ => {
            base = dbase.as_ptr();
            extra = dext.as_ptr();
            end = -(1 as libc::c_int);
        }
    }
    huff = 0 as libc::c_int as libc::c_uint;
    sym = 0 as libc::c_int as libc::c_uint;
    len = min;
    next = *table;
    curr = root;
    drop_0 = 0 as libc::c_int as libc::c_uint;
    low = -(1 as libc::c_int) as libc::c_uint;
    used = (1 as libc::c_uint) << root;
    mask = used.wrapping_sub(1 as libc::c_int as libc::c_uint);
    if type_0 as libc::c_uint == LENS as libc::c_int as libc::c_uint
        && used >= 852 as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == DISTS as libc::c_int as libc::c_uint
            && used >= 592 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    loop {
        here.bits = len.wrapping_sub(drop_0) as libc::c_uchar;
        if (*work.offset(sym as isize) as libc::c_int) < end {
            here.op = 0 as libc::c_int as libc::c_uchar;
            here.val = *work.offset(sym as isize);
        } else if *work.offset(sym as isize) as libc::c_int > end {
            here
                .op = *extra.offset(*work.offset(sym as isize) as isize)
                as libc::c_uchar;
            here.val = *base.offset(*work.offset(sym as isize) as isize);
        } else {
            here.op = (32 as libc::c_int + 64 as libc::c_int) as libc::c_uchar;
            here.val = 0 as libc::c_int as libc::c_ushort;
        }
        incr = (1 as libc::c_uint) << len.wrapping_sub(drop_0);
        fill = (1 as libc::c_uint) << curr;
        min = fill;
        loop {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = here;
            if !(fill != 0 as libc::c_int as libc::c_uint) {
                break;
            }
        }
        incr = (1 as libc::c_uint) << len.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while huff & incr != 0 {
            incr >>= 1 as libc::c_int;
        }
        if incr != 0 as libc::c_int as libc::c_uint {
            huff &= incr.wrapping_sub(1 as libc::c_int as libc::c_uint);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0 as libc::c_int as libc::c_uint;
        }
        sym = sym.wrapping_add(1);
        sym;
        count[len as usize] = (count[len as usize]).wrapping_sub(1);
        if count[len as usize] as libc::c_int == 0 as libc::c_int {
            if len == max {
                break;
            }
            len = *lens.offset(*work.offset(sym as isize) as isize) as libc::c_uint;
        }
        if len > root && huff & mask != low {
            if drop_0 == 0 as libc::c_int as libc::c_uint {
                drop_0 = root;
            }
            next = next.offset(min as isize);
            curr = len.wrapping_sub(drop_0);
            left = (1 as libc::c_int) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -= count[curr.wrapping_add(drop_0) as usize] as libc::c_int;
                if left <= 0 as libc::c_int {
                    break;
                }
                curr = curr.wrapping_add(1);
                curr;
                left <<= 1 as libc::c_int;
            }
            used = used.wrapping_add((1 as libc::c_uint) << curr);
            if type_0 as libc::c_uint == LENS as libc::c_int as libc::c_uint
                && used >= 852 as libc::c_int as libc::c_uint
                || type_0 as libc::c_uint == DISTS as libc::c_int as libc::c_uint
                    && used >= 592 as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as libc::c_uchar;
            (*(*table).offset(low as isize)).bits = root as libc::c_uchar;
            (*(*table).offset(low as isize))
                .val = next.offset_from(*table) as libc::c_long as libc::c_ushort;
        }
    }
    here.op = 64 as libc::c_int as libc::c_uchar;
    here.bits = len.wrapping_sub(drop_0) as libc::c_uchar;
    here.val = 0 as libc::c_int as libc::c_ushort;
    while huff != 0 as libc::c_int as libc::c_uint {
        if drop_0 != 0 as libc::c_int as libc::c_uint && huff & mask != low {
            drop_0 = 0 as libc::c_int as libc::c_uint;
            len = root;
            next = *table;
            here.bits = len as libc::c_uchar;
        }
        *next.offset((huff >> drop_0) as isize) = here;
        incr = (1 as libc::c_uint) << len.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while huff & incr != 0 {
            incr >>= 1 as libc::c_int;
        }
        if incr != 0 as libc::c_int as libc::c_uint {
            huff &= incr.wrapping_sub(1 as libc::c_int as libc::c_uint);
            huff = huff.wrapping_add(incr);
        } else {
            huff = 0 as libc::c_int as libc::c_uint;
        }
    }
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as libc::c_int;
}
