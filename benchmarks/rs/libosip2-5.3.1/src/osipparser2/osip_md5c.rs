use ::libc;
pub type POINTER = *mut libc::c_uchar;
pub type UINT4 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_MD5_CTX {
    pub state: [UINT4; 4],
    pub count: [UINT4; 2],
    pub buffer: [libc::c_uchar; 64],
}
static mut PADDING: [libc::c_uchar; 64] = [
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn osip_MD5Init(mut context: *mut osip_MD5_CTX) {
    (*context).count[1 as libc::c_int as usize] = 0 as libc::c_int as UINT4;
    (*context)
        .count[0 as libc::c_int as usize] = (*context).count[1 as libc::c_int as usize];
    (*context).state[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as UINT4;
    (*context).state[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*context).state[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*context).state[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as UINT4;
}
pub unsafe extern "C" fn osip_MD5Update(
    mut context: *mut osip_MD5_CTX,
    mut input: *mut libc::c_uchar,
    mut inputLen: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut index: libc::c_uint = 0;
    let mut partLen: libc::c_uint = 0;
    index = (*context).count[0 as libc::c_int as usize] >> 3 as libc::c_int
        & 0x3f as libc::c_int as libc::c_uint;
    (*context)
        .count[0 as libc::c_int
        as usize] = ((*context).count[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(inputLen << 3 as libc::c_int) as UINT4 as UINT4;
    if (*context).count[0 as libc::c_int as usize] < inputLen << 3 as libc::c_int {
        (*context)
            .count[1 as libc::c_int
            as usize] = ((*context).count[1 as libc::c_int as usize]).wrapping_add(1);
        (*context).count[1 as libc::c_int as usize];
    }
    (*context)
        .count[1 as libc::c_int
        as usize] = ((*context).count[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(inputLen >> 29 as libc::c_int) as UINT4 as UINT4;
    partLen = (64 as libc::c_int as libc::c_uint).wrapping_sub(index);
    if inputLen >= partLen {
        osip_MD5_memcpy(
            &mut *((*context).buffer).as_mut_ptr().offset(index as isize)
                as *mut libc::c_uchar,
            input,
            partLen,
        );
        osip_MD5Transform(
            ((*context).state).as_mut_ptr(),
            ((*context).buffer).as_mut_ptr(),
        );
        i = partLen;
        while i.wrapping_add(63 as libc::c_int as libc::c_uint) < inputLen {
            osip_MD5Transform(
                ((*context).state).as_mut_ptr(),
                &mut *input.offset(i as isize),
            );
            i = i.wrapping_add(64 as libc::c_int as libc::c_uint);
        }
        index = 0 as libc::c_int as libc::c_uint;
    } else {
        i = 0 as libc::c_int as libc::c_uint;
    }
    osip_MD5_memcpy(
        &mut *((*context).buffer).as_mut_ptr().offset(index as isize)
            as *mut libc::c_uchar,
        &mut *input.offset(i as isize) as *mut libc::c_uchar,
        inputLen.wrapping_sub(i),
    );
}
pub unsafe extern "C" fn osip_MD5Final(
    mut digest: *mut libc::c_uchar,
    mut context: *mut osip_MD5_CTX,
) {
    let mut bits: [libc::c_uchar; 8] = [0; 8];
    let mut index: libc::c_uint = 0;
    let mut padLen: libc::c_uint = 0;
    osip_Encode(
        bits.as_mut_ptr(),
        ((*context).count).as_mut_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    index = (*context).count[0 as libc::c_int as usize] >> 3 as libc::c_int
        & 0x3f as libc::c_int as libc::c_uint;
    padLen = if index < 56 as libc::c_int as libc::c_uint {
        (56 as libc::c_int as libc::c_uint).wrapping_sub(index)
    } else {
        (120 as libc::c_int as libc::c_uint).wrapping_sub(index)
    };
    osip_MD5Update(context, PADDING.as_mut_ptr(), padLen);
    osip_MD5Update(context, bits.as_mut_ptr(), 8 as libc::c_int as libc::c_uint);
    osip_Encode(
        digest,
        ((*context).state).as_mut_ptr(),
        16 as libc::c_int as libc::c_uint,
    );
    osip_MD5_memset(
        context as POINTER,
        0 as libc::c_int,
        ::std::mem::size_of::<osip_MD5_CTX>() as libc::c_ulong as libc::c_uint,
    );
}
unsafe extern "C" fn osip_MD5Transform(
    mut state: *mut UINT4,
    mut block: *mut libc::c_uchar,
) {
    let mut a: UINT4 = *state.offset(0 as libc::c_int as isize);
    let mut b: UINT4 = *state.offset(1 as libc::c_int as isize);
    let mut c: UINT4 = *state.offset(2 as libc::c_int as isize);
    let mut d: UINT4 = *state.offset(3 as libc::c_int as isize);
    let mut x: [UINT4; 16] = [0; 16];
    osip_Decode(x.as_mut_ptr(), block, 64 as libc::c_int as libc::c_uint);
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0xd76aa478 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0xe8c7b756 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0x242070db as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0xc1bdceee as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0xf57c0faf as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0x4787c62a as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0xa8304613 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0xfd469501 as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x698098d8 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0x8b44f7af as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0xffff5bb1 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0x895cd7be as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | !b & d)
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0x6b901122 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | !a & c)
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0xfd987193 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | !d & b)
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0xa679438e as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | !c & a)
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0x49b40821 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0xf61e2562 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0xc040b340 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0x265e5a51 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0xe9b6c7aa as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0xd62f105d as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0x2441453 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0xd8a1e681 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0xe7d3fbc8 as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0x21e1cde6 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0xc33707d6 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0xf4d50d87 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x455a14ed as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & d | c & !d)
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0xa9e3e905 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & c | b & !c)
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0xfcefa3f8 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & b | a & !b)
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0x676f02d9 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & a | d & !a)
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0x8d2a4c8a as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0xfffa3942 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x8771f681 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0x6d9d6122 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0xfde5380c as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0xa4beea44 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0x4bdecfa9 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0xf6bb4b60 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0xbebfbc70 as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0x289b7ec6 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0xeaa127fa as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0xd4ef3085 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0x4881d05 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0xd9d4d039 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0xe6db99e5 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0x1fa27cf8 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0xc4ac5665 as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0xf4292244 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0x432aff97 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0xab9423a7 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0xfc93a039 as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0x655b59c3 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0x8f0ccc92 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0xffeff47d as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0x85845dd1 as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x6fa87e4f as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0xfe2ce6e0 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0xa3014314 as libc::c_uint),
        ) as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0x4e0811a1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0xf7537e82 as libc::c_uint),
        ) as UINT4 as UINT4;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0xbd3af235 as libc::c_uint),
        ) as UINT4 as UINT4;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0x2ad7d2bb as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0xeb86d391 as libc::c_uint),
        ) as UINT4 as UINT4;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    let ref mut fresh0 = *state.offset(0 as libc::c_int as isize);
    *fresh0 = (*fresh0 as libc::c_uint).wrapping_add(a) as UINT4 as UINT4;
    let ref mut fresh1 = *state.offset(1 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(b) as UINT4 as UINT4;
    let ref mut fresh2 = *state.offset(2 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(c) as UINT4 as UINT4;
    let ref mut fresh3 = *state.offset(3 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(d) as UINT4 as UINT4;
    osip_MD5_memset(
        x.as_mut_ptr() as POINTER,
        0 as libc::c_int,
        ::std::mem::size_of::<[UINT4; 16]>() as libc::c_ulong as libc::c_uint,
    );
}
unsafe extern "C" fn osip_Encode(
    mut output: *mut libc::c_uchar,
    mut input: *mut UINT4,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while j < len {
        *output
            .offset(
                j as isize,
            ) = (*input.offset(i as isize) & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *output
            .offset(
                j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = (*input.offset(i as isize) >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *output
            .offset(
                j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) = (*input.offset(i as isize) >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *output
            .offset(
                j.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) = (*input.offset(i as isize) >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn osip_Decode(
    mut output: *mut UINT4,
    mut input: *mut libc::c_uchar,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while j < len {
        *output
            .offset(
                i as isize,
            ) = *input.offset(j as isize) as UINT4
            | (*input.offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                as UINT4) << 8 as libc::c_int
            | (*input.offset(j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                as UINT4) << 16 as libc::c_int
            | (*input.offset(j.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
                as UINT4) << 24 as libc::c_int;
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn osip_MD5_memcpy(
    mut output: POINTER,
    mut input: POINTER,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < len {
        *output.offset(i as isize) = *input.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn osip_MD5_memset(
    mut output: POINTER,
    mut value: libc::c_int,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < len {
        *(output as *mut libc::c_char).offset(i as isize) = value as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
}
