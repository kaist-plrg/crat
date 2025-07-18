use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut libc::c_uchar,
}
pub unsafe extern "C" fn BitStream_new() -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    bstream = malloc(::std::mem::size_of::<BitStream>() as libc::c_ulong)
        as *mut BitStream;
    if bstream.is_null() {
        return 0 as *mut BitStream;
    }
    (*bstream).length = 0 as libc::c_int as size_t;
    (*bstream).data = malloc(128 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    if ((*bstream).data).is_null() {
        free(bstream as *mut libc::c_void);
        return 0 as *mut BitStream;
    }
    (*bstream).datasize = 128 as libc::c_int as size_t;
    return bstream;
}
pub unsafe extern "C" fn BitStream_newWithBits(
    mut size: size_t,
    mut bits: *mut libc::c_uchar,
) -> *mut BitStream {
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    if size == 0 as libc::c_int as libc::c_ulong {
        return BitStream_new();
    }
    bstream = malloc(::std::mem::size_of::<BitStream>() as libc::c_ulong)
        as *mut BitStream;
    if bstream.is_null() {
        return 0 as *mut BitStream;
    }
    (*bstream).data = malloc(size) as *mut libc::c_uchar;
    if ((*bstream).data).is_null() {
        free(bstream as *mut libc::c_void);
        return 0 as *mut BitStream;
    }
    (*bstream).length = size;
    (*bstream).datasize = size;
    memcpy((*bstream).data as *mut libc::c_void, bits as *const libc::c_void, size);
    return bstream;
}
unsafe extern "C" fn BitStream_expand(mut bstream: *mut BitStream) -> libc::c_int {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    data = realloc(
        (*bstream).data as *mut libc::c_void,
        ((*bstream).datasize).wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if data.is_null() {
        return -(1 as libc::c_int);
    }
    (*bstream).data = data;
    (*bstream)
        .datasize = ((*bstream).datasize as libc::c_ulong)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn BitStream_writeNum(
    mut dest: *mut libc::c_uchar,
    mut bits: size_t,
    mut num: libc::c_uint,
) {
    let mut mask: libc::c_uint = 0;
    let mut i: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = dest;
    mask = (1 as libc::c_uint) << bits.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int as size_t;
    while i < bits {
        if num & mask != 0 {
            *p = 1 as libc::c_int as libc::c_uchar;
        } else {
            *p = 0 as libc::c_int as libc::c_uchar;
        }
        p = p.offset(1);
        p;
        mask = mask >> 1 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn BitStream_writeBytes(
    mut dest: *mut libc::c_uchar,
    mut size: size_t,
    mut data: *mut libc::c_uchar,
) {
    let mut mask: libc::c_uchar = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = dest;
    i = 0 as libc::c_int as size_t;
    while i < size {
        mask = 0x80 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int as size_t;
        while j < 8 as libc::c_int as libc::c_ulong {
            if *data.offset(i as isize) as libc::c_int & mask as libc::c_int != 0 {
                *p = 1 as libc::c_int as libc::c_uchar;
            } else {
                *p = 0 as libc::c_int as libc::c_uchar;
            }
            p = p.offset(1);
            p;
            mask = (mask as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn BitStream_append(
    mut bstream: *mut BitStream,
    mut arg: *mut BitStream,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if arg.is_null() {
        return -(1 as libc::c_int);
    }
    if (*arg).length == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ((*bstream).length).wrapping_add((*arg).length) > (*bstream).datasize {
        ret = BitStream_expand(bstream);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    memcpy(
        ((*bstream).data).offset((*bstream).length as isize) as *mut libc::c_void,
        (*arg).data as *const libc::c_void,
        (*arg).length,
    );
    (*bstream)
        .length = ((*bstream).length as libc::c_ulong).wrapping_add((*arg).length)
        as size_t as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_appendNum(
    mut bstream: *mut BitStream,
    mut bits: size_t,
    mut num: libc::c_uint,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if bits == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ((*bstream).datasize).wrapping_sub((*bstream).length) < bits {
        ret = BitStream_expand(bstream);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    BitStream_writeNum(((*bstream).data).offset((*bstream).length as isize), bits, num);
    (*bstream)
        .length = ((*bstream).length as libc::c_ulong).wrapping_add(bits) as size_t
        as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_appendBytes(
    mut bstream: *mut BitStream,
    mut size: size_t,
    mut data: *mut libc::c_uchar,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while ((*bstream).datasize).wrapping_sub((*bstream).length)
        < size.wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        ret = BitStream_expand(bstream);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    BitStream_writeBytes(
        ((*bstream).data).offset((*bstream).length as isize),
        size,
        data,
    );
    (*bstream)
        .length = ((*bstream).length as libc::c_ulong)
        .wrapping_add(size.wrapping_mul(8 as libc::c_int as libc::c_ulong)) as size_t
        as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn BitStream_toByte(
    mut bstream: *mut BitStream,
) -> *mut libc::c_uchar {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut size: size_t = 0;
    let mut bytes: size_t = 0;
    let mut oddbits: size_t = 0;
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut v: libc::c_uchar = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    size = (*bstream).length;
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_uchar;
    }
    data = malloc(
        size
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            .wrapping_div(8 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if data.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    bytes = size.wrapping_div(8 as libc::c_int as libc::c_ulong);
    p = (*bstream).data;
    i = 0 as libc::c_int as size_t;
    while i < bytes {
        v = 0 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int as size_t;
        while j < 8 as libc::c_int as libc::c_ulong {
            v = ((v as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
            v = (v as libc::c_int | *p as libc::c_int) as libc::c_uchar;
            p = p.offset(1);
            p;
            j = j.wrapping_add(1);
            j;
        }
        *data.offset(i as isize) = v;
        i = i.wrapping_add(1);
        i;
    }
    oddbits = size & 7 as libc::c_int as libc::c_ulong;
    if oddbits > 0 as libc::c_int as libc::c_ulong {
        v = 0 as libc::c_int as libc::c_uchar;
        j = 0 as libc::c_int as size_t;
        while j < oddbits {
            v = ((v as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
            v = (v as libc::c_int | *p as libc::c_int) as libc::c_uchar;
            p = p.offset(1);
            p;
            j = j.wrapping_add(1);
            j;
        }
        *data
            .offset(
                bytes as isize,
            ) = ((v as libc::c_int)
            << (8 as libc::c_int as libc::c_ulong).wrapping_sub(oddbits))
            as libc::c_uchar;
    }
    return data;
}
pub unsafe extern "C" fn BitStream_free(mut bstream: *mut BitStream) {
    if !bstream.is_null() {
        free((*bstream).data as *mut libc::c_void);
        free(bstream as *mut libc::c_void);
    }
}
