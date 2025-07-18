use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn binn_version() -> *mut libc::c_char;
    fn binn_new(
        type_0: libc::c_int,
        size: libc::c_int,
        buffer: *mut libc::c_void,
    ) -> *mut binn;
    fn binn_list() -> *mut binn;
    fn binn_map() -> *mut binn;
    fn binn_object() -> *mut binn;
    fn binn_list_add(
        list: *mut binn,
        type_0: libc::c_int,
        pvalue: *mut libc::c_void,
        size: libc::c_int,
    ) -> BOOL;
    fn binn_map_set(
        map: *mut binn,
        id: libc::c_int,
        type_0: libc::c_int,
        pvalue: *mut libc::c_void,
        size: libc::c_int,
    ) -> BOOL;
    fn binn_object_set(
        obj: *mut binn,
        key: *const libc::c_char,
        type_0: libc::c_int,
        pvalue: *mut libc::c_void,
        size: libc::c_int,
    ) -> BOOL;
    fn binn_free(item: *mut binn);
    fn binn_ptr(ptr: *mut libc::c_void) -> *mut libc::c_void;
    fn binn_size(ptr: *mut libc::c_void) -> libc::c_int;
    fn binn_type(ptr: *mut libc::c_void) -> libc::c_int;
    fn binn_count(ptr: *mut libc::c_void) -> libc::c_int;
    fn binn_is_valid(
        ptr: *mut libc::c_void,
        ptype: *mut libc::c_int,
        pcount: *mut libc::c_int,
        psize: *mut libc::c_int,
    ) -> BOOL;
    fn binn_is_valid_ex(
        ptr: *mut libc::c_void,
        ptype: *mut libc::c_int,
        pcount: *mut libc::c_int,
        psize: *mut libc::c_int,
    ) -> BOOL;
    fn binn_list_int32(list: *mut libc::c_void, pos: libc::c_int) -> libc::c_int;
    fn binn_list_double(list: *mut libc::c_void, pos: libc::c_int) -> libc::c_double;
    fn binn_list_bool(list: *mut libc::c_void, pos: libc::c_int) -> BOOL;
    fn binn_list_str(list: *mut libc::c_void, pos: libc::c_int) -> *mut libc::c_char;
    fn binn_list_blob(
        list: *mut libc::c_void,
        pos: libc::c_int,
        psize: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn binn_map_int32(map: *mut libc::c_void, id: libc::c_int) -> libc::c_int;
    fn binn_map_double(map: *mut libc::c_void, id: libc::c_int) -> libc::c_double;
    fn binn_map_bool(map: *mut libc::c_void, id: libc::c_int) -> BOOL;
    fn binn_map_str(map: *mut libc::c_void, id: libc::c_int) -> *mut libc::c_char;
    fn binn_map_blob(
        map: *mut libc::c_void,
        id: libc::c_int,
        psize: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn binn_object_int32(
        obj: *mut libc::c_void,
        key: *const libc::c_char,
    ) -> libc::c_int;
    fn binn_object_double(
        obj: *mut libc::c_void,
        key: *const libc::c_char,
    ) -> libc::c_double;
    fn binn_object_bool(obj: *mut libc::c_void, key: *const libc::c_char) -> BOOL;
    fn binn_object_str(
        obj: *mut libc::c_void,
        key: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn binn_object_blob(
        obj: *mut libc::c_void,
        key: *const libc::c_char,
        psize: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn binn_list_get_value(
        list: *mut libc::c_void,
        pos: libc::c_int,
        value: *mut binn,
    ) -> BOOL;
    fn binn_map_get_value(
        map: *mut libc::c_void,
        id: libc::c_int,
        value: *mut binn,
    ) -> BOOL;
    fn binn_object_get_value(
        obj: *mut libc::c_void,
        key: *const libc::c_char,
        value: *mut binn,
    ) -> BOOL;
    fn binn_list_get(
        list: *mut libc::c_void,
        pos: libc::c_int,
        type_0: libc::c_int,
        pvalue: *mut libc::c_void,
        psize: *mut libc::c_int,
    ) -> BOOL;
    fn binn_map_get(
        map: *mut libc::c_void,
        id: libc::c_int,
        type_0: libc::c_int,
        pvalue: *mut libc::c_void,
        psize: *mut libc::c_int,
    ) -> BOOL;
    fn binn_object_get(
        obj: *mut libc::c_void,
        key: *const libc::c_char,
        type_0: libc::c_int,
        pvalue: *mut libc::c_void,
        psize: *mut libc::c_int,
    ) -> BOOL;
    fn binn_list_read(
        list: *mut libc::c_void,
        pos: libc::c_int,
        ptype: *mut libc::c_int,
        psize: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn binn_map_read(
        map: *mut libc::c_void,
        id: libc::c_int,
        ptype: *mut libc::c_int,
        psize: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn binn_object_read(
        obj: *mut libc::c_void,
        key: *const libc::c_char,
        ptype: *mut libc::c_int,
        psize: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn binn_map_get_pair(
        map: *mut libc::c_void,
        pos: libc::c_int,
        pid: *mut libc::c_int,
        value: *mut binn,
    ) -> BOOL;
    fn binn_object_get_pair(
        obj: *mut libc::c_void,
        pos: libc::c_int,
        pkey: *mut libc::c_char,
        value: *mut binn,
    ) -> BOOL;
    fn test_binn2();
    fn IsValidBinnHeader(
        _: *mut libc::c_void,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> BOOL;
    fn CalcAllocation(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn copy_be16(pdest: *mut u16_0, psource: *mut u16_0);
    fn copy_be32(pdest: *mut u32_0, psource: *mut u32_0);
    fn copy_be64(pdest: *mut u64_0, psource: *mut u64_0);
}
pub type BOOL = libc::c_int;
pub type int64 = libc::c_longlong;
pub type uint64 = libc::c_ulonglong;
pub type binn_mem_free = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_struct {
    pub header: libc::c_int,
    pub allocated: BOOL,
    pub writable: BOOL,
    pub dirty: BOOL,
    pub pbuf: *mut libc::c_void,
    pub pre_allocated: BOOL,
    pub alloc_size: libc::c_int,
    pub used_size: libc::c_int,
    pub type_0: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub freefn: binn_mem_free,
    pub c2rust_unnamed: C2RustUnnamed,
    pub disable_int_compression: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub vint8: libc::c_schar,
    pub vint16: libc::c_short,
    pub vint32: libc::c_int,
    pub vint64: int64,
    pub vuint8: libc::c_uchar,
    pub vuint16: libc::c_ushort,
    pub vuint32: libc::c_uint,
    pub vuint64: uint64,
    pub vchar: libc::c_schar,
    pub vuchar: libc::c_uchar,
    pub vshort: libc::c_short,
    pub vushort: libc::c_ushort,
    pub vint: libc::c_int,
    pub vuint: libc::c_uint,
    pub vfloat: libc::c_float,
    pub vdouble: libc::c_double,
    pub vbool: BOOL,
}
pub type binn = binn_struct;
pub type u16_0 = libc::c_ushort;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
#[inline(always)]
unsafe extern "C" fn binn_list_add_int32(
    mut list: *mut binn,
    mut value: libc::c_int,
) -> BOOL {
    return binn_list_add(
        list,
        0x61 as libc::c_int,
        &mut value as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_add_double(
    mut list: *mut binn,
    mut value: libc::c_double,
) -> BOOL {
    return binn_list_add(
        list,
        0x82 as libc::c_int,
        &mut value as *mut libc::c_double as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_add_bool(mut list: *mut binn, mut value: BOOL) -> BOOL {
    return binn_list_add(
        list,
        0x80061 as libc::c_int,
        &mut value as *mut BOOL as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_add_str(
    mut list: *mut binn,
    mut str: *mut libc::c_char,
) -> BOOL {
    return binn_list_add(
        list,
        0xa0 as libc::c_int,
        str as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_add_blob(
    mut list: *mut binn,
    mut ptr: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    return binn_list_add(list, 0xc0 as libc::c_int, ptr, size);
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_int32(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut value: libc::c_int,
) -> BOOL {
    return binn_map_set(
        map,
        id,
        0x61 as libc::c_int,
        &mut value as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_double(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut value: libc::c_double,
) -> BOOL {
    return binn_map_set(
        map,
        id,
        0x82 as libc::c_int,
        &mut value as *mut libc::c_double as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_bool(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut value: BOOL,
) -> BOOL {
    return binn_map_set(
        map,
        id,
        0x80061 as libc::c_int,
        &mut value as *mut BOOL as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_str(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut str: *mut libc::c_char,
) -> BOOL {
    return binn_map_set(
        map,
        id,
        0xa0 as libc::c_int,
        str as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_blob(
    mut map: *mut binn,
    mut id: libc::c_int,
    mut ptr: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    return binn_map_set(map, id, 0xc0 as libc::c_int, ptr, size);
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_int32(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut value: libc::c_int,
) -> BOOL {
    return binn_object_set(
        obj,
        key,
        0x61 as libc::c_int,
        &mut value as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_double(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut value: libc::c_double,
) -> BOOL {
    return binn_object_set(
        obj,
        key,
        0x82 as libc::c_int,
        &mut value as *mut libc::c_double as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_bool(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut value: BOOL,
) -> BOOL {
    return binn_object_set(
        obj,
        key,
        0x80061 as libc::c_int,
        &mut value as *mut BOOL as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_str(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut str: *mut libc::c_char,
) -> BOOL {
    return binn_object_set(
        obj,
        key,
        0xa0 as libc::c_int,
        str as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_blob(
    mut obj: *mut binn,
    mut key: *const libc::c_char,
    mut ptr: *mut libc::c_void,
    mut size: libc::c_int,
) -> BOOL {
    return binn_object_set(obj, key, 0xc0 as libc::c_int, ptr, size);
}
#[inline(always)]
unsafe extern "C" fn binn_list_get_int32(
    mut list: *mut libc::c_void,
    mut pos: libc::c_int,
    mut pvalue: *mut libc::c_int,
) -> BOOL {
    return binn_list_get(
        list,
        pos,
        0x61 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_get_double(
    mut list: *mut libc::c_void,
    mut pos: libc::c_int,
    mut pvalue: *mut libc::c_double,
) -> BOOL {
    return binn_list_get(
        list,
        pos,
        0x82 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_get_bool(
    mut list: *mut libc::c_void,
    mut pos: libc::c_int,
    mut pvalue: *mut BOOL,
) -> BOOL {
    return binn_list_get(
        list,
        pos,
        0x80061 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_get_str(
    mut list: *mut libc::c_void,
    mut pos: libc::c_int,
    mut pvalue: *mut *mut libc::c_char,
) -> BOOL {
    return binn_list_get(
        list,
        pos,
        0xa0 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_list_get_blob(
    mut list: *mut libc::c_void,
    mut pos: libc::c_int,
    mut pvalue: *mut *mut libc::c_void,
    mut psize: *mut libc::c_int,
) -> BOOL {
    return binn_list_get(
        list,
        pos,
        0xc0 as libc::c_int,
        pvalue as *mut libc::c_void,
        psize,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_get_int32(
    mut map: *mut libc::c_void,
    mut id: libc::c_int,
    mut pvalue: *mut libc::c_int,
) -> BOOL {
    return binn_map_get(
        map,
        id,
        0x61 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_get_double(
    mut map: *mut libc::c_void,
    mut id: libc::c_int,
    mut pvalue: *mut libc::c_double,
) -> BOOL {
    return binn_map_get(
        map,
        id,
        0x82 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_get_bool(
    mut map: *mut libc::c_void,
    mut id: libc::c_int,
    mut pvalue: *mut BOOL,
) -> BOOL {
    return binn_map_get(
        map,
        id,
        0x80061 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_get_str(
    mut map: *mut libc::c_void,
    mut id: libc::c_int,
    mut pvalue: *mut *mut libc::c_char,
) -> BOOL {
    return binn_map_get(
        map,
        id,
        0xa0 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_get_blob(
    mut map: *mut libc::c_void,
    mut id: libc::c_int,
    mut pvalue: *mut *mut libc::c_void,
    mut psize: *mut libc::c_int,
) -> BOOL {
    return binn_map_get(
        map,
        id,
        0xc0 as libc::c_int,
        pvalue as *mut libc::c_void,
        psize,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_get_int32(
    mut obj: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut pvalue: *mut libc::c_int,
) -> BOOL {
    return binn_object_get(
        obj,
        key,
        0x61 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_get_double(
    mut obj: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut pvalue: *mut libc::c_double,
) -> BOOL {
    return binn_object_get(
        obj,
        key,
        0x82 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_get_bool(
    mut obj: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut pvalue: *mut BOOL,
) -> BOOL {
    return binn_object_get(
        obj,
        key,
        0x80061 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_get_str(
    mut obj: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut pvalue: *mut *mut libc::c_char,
) -> BOOL {
    return binn_object_get(
        obj,
        key,
        0xa0 as libc::c_int,
        pvalue as *mut libc::c_void,
        0 as *mut libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_get_blob(
    mut obj: *mut libc::c_void,
    mut key: *const libc::c_char,
    mut pvalue: *mut *mut libc::c_void,
    mut psize: *mut libc::c_int,
) -> BOOL {
    return binn_object_get(
        obj,
        key,
        0xc0 as libc::c_int,
        pvalue as *mut libc::c_void,
        psize,
    );
}
pub unsafe extern "C" fn test_binn_version() {
    let mut version: *mut libc::c_char = binn_version();
    if !version.is_null() {} else {
        __assert_fail(
            b"version\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_binn_version()\0"))
                .as_ptr(),
        );
    }
    'c_5963: {
        if !version.is_null() {} else {
            __assert_fail(
                b"version\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_binn_version()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(version, b"3.0.0\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(version,\"3.0.0\")==0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_binn_version()\0"))
                .as_ptr(),
        );
    }
    'c_5903: {
        if strcmp(version, b"3.0.0\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(version,\"3.0.0\")==0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                31 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_binn_version()\0"))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn test_endianess() {
    let mut vshort1: u16_0 = 0;
    let mut vshort2: u16_0 = 0;
    let mut vshort3: u16_0 = 0;
    let mut vint1: u32_0 = 0;
    let mut vint2: u32_0 = 0;
    let mut vint3: u32_0 = 0;
    let mut value1: u64_0 = 0;
    let mut value2: u64_0 = 0;
    let mut value3: u64_0 = 0;
    printf(b"testing endianess... \0" as *const u8 as *const libc::c_char);
    vshort1 = 0x1122 as libc::c_int as u16_0;
    copy_be16(&mut vshort2, &mut vshort1);
    if vshort2 as libc::c_int == 0x2211 as libc::c_int {} else {
        __assert_fail(
            b"vshort2 == 0x2211\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6699: {
        if vshort2 as libc::c_int == 0x2211 as libc::c_int {} else {
            __assert_fail(
                b"vshort2 == 0x2211\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    copy_be16(&mut vshort3, &mut vshort2);
    if vshort3 as libc::c_int == vshort1 as libc::c_int {} else {
        __assert_fail(
            b"vshort3 == vshort1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6650: {
        if vshort3 as libc::c_int == vshort1 as libc::c_int {} else {
            __assert_fail(
                b"vshort3 == vshort1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    vshort1 = 0xf123 as libc::c_int as u16_0;
    copy_be16(&mut vshort2, &mut vshort1);
    if vshort2 as libc::c_int == 0x23f1 as libc::c_int {} else {
        __assert_fail(
            b"vshort2 == 0x23F1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6601: {
        if vshort2 as libc::c_int == 0x23f1 as libc::c_int {} else {
            __assert_fail(
                b"vshort2 == 0x23F1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    copy_be16(&mut vshort3, &mut vshort2);
    if vshort3 as libc::c_int == vshort1 as libc::c_int {} else {
        __assert_fail(
            b"vshort3 == vshort1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6552: {
        if vshort3 as libc::c_int == vshort1 as libc::c_int {} else {
            __assert_fail(
                b"vshort3 == vshort1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    vshort1 = 0x123 as libc::c_int as u16_0;
    copy_be16(&mut vshort2, &mut vshort1);
    if vshort2 as libc::c_int == 0x2301 as libc::c_int {} else {
        __assert_fail(
            b"vshort2 == 0x2301\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6502: {
        if vshort2 as libc::c_int == 0x2301 as libc::c_int {} else {
            __assert_fail(
                b"vshort2 == 0x2301\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    copy_be16(&mut vshort3, &mut vshort2);
    if vshort3 as libc::c_int == vshort1 as libc::c_int {} else {
        __assert_fail(
            b"vshort3 == vshort1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6448: {
        if vshort3 as libc::c_int == vshort1 as libc::c_int {} else {
            __assert_fail(
                b"vshort3 == vshort1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    vint1 = 0x11223344 as libc::c_int as u32_0;
    copy_be32(&mut vint2, &mut vint1);
    if vint2 == 0x44332211 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"vint2 == 0x44332211\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6399: {
        if vint2 == 0x44332211 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"vint2 == 0x44332211\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    copy_be32(&mut vint3, &mut vint2);
    if vint3 == vint1 {} else {
        __assert_fail(
            b"vint3 == vint1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6354: {
        if vint3 == vint1 {} else {
            __assert_fail(
                b"vint3 == vint1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    vint1 = 0xf1234580 as libc::c_uint;
    copy_be32(&mut vint2, &mut vint1);
    if vint2 == 0x804523f1 as libc::c_uint {} else {
        __assert_fail(
            b"vint2 == 0x804523F1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6308: {
        if vint2 == 0x804523f1 as libc::c_uint {} else {
            __assert_fail(
                b"vint2 == 0x804523F1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    copy_be32(&mut vint3, &mut vint2);
    if vint3 == vint1 {} else {
        __assert_fail(
            b"vint3 == vint1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6263: {
        if vint3 == vint1 {} else {
            __assert_fail(
                b"vint3 == vint1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    vint1 = 0x112233 as libc::c_int as u32_0;
    copy_be32(&mut vint2, &mut vint1);
    if vint2 == 0x33221100 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"vint2 == 0x33221100\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6213: {
        if vint2 == 0x33221100 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"vint2 == 0x33221100\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    copy_be32(&mut vint3, &mut vint2);
    if vint3 == vint1 {} else {
        __assert_fail(
            b"vint3 == vint1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6163: {
        if vint3 == vint1 {} else {
            __assert_fail(
                b"vint3 == vint1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    value1 = 0x1122334455667788 as libc::c_long as u64_0;
    copy_be64(&mut value2, &mut value1);
    if value2 == 0x8877665544332211 as libc::c_ulong as libc::c_ulonglong {} else {
        __assert_fail(
            b"value2 == 0x8877665544332211\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6113: {
        if value2 == 0x8877665544332211 as libc::c_ulong as libc::c_ulonglong {} else {
            __assert_fail(
                b"value2 == 0x8877665544332211\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    copy_be64(&mut value3, &mut value2);
    if value3 == value1 {} else {
        __assert_fail(
            b"value3 == value1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_endianess()\0"))
                .as_ptr(),
        );
    }
    'c_6063: {
        if value3 == value1 {} else {
            __assert_fail(
                b"value3 == value1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void test_endianess()\0"))
                    .as_ptr(),
            );
        }
    };
    printf(b"OK\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn memdup(
    mut src: *mut libc::c_void,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    let mut dest: *mut libc::c_void = 0 as *mut libc::c_void;
    if src.is_null() || size <= 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    dest = malloc(size as libc::c_ulong);
    if dest.is_null() {
        return 0 as *mut libc::c_void;
    }
    memcpy(dest, src, size as libc::c_ulong);
    return dest;
}
pub unsafe extern "C" fn i64toa(
    mut value: int64,
    mut buf: *mut libc::c_char,
    mut radix: libc::c_int,
) -> *mut libc::c_char {
    match radix {
        10 => {
            snprintf(
                buf,
                64 as libc::c_int as libc::c_ulong,
                b"%lli\0" as *const u8 as *const libc::c_char,
                value,
            );
        }
        16 => {
            snprintf(
                buf,
                64 as libc::c_int as libc::c_ulong,
                b"%llx\0" as *const u8 as *const libc::c_char,
                value,
            );
        }
        _ => {
            *buf.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        }
    }
    return buf;
}
pub unsafe extern "C" fn pass_int64(mut a: int64) {
    if a == 9223372036854775807 as libc::c_long as libc::c_longlong {} else {
        __assert_fail(
            b"a == 9223372036854775807\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void pass_int64(int64)\0"))
                .as_ptr(),
        );
    }
    'c_6914: {
        if a == 9223372036854775807 as libc::c_long as libc::c_longlong {} else {
            __assert_fail(
                b"a == 9223372036854775807\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                163 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void pass_int64(int64)\0"))
                    .as_ptr(),
            );
        }
    };
    if a > 9223372036854775806 as libc::c_long as libc::c_longlong {} else {
        __assert_fail(
            b"a > 9223372036854775806\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void pass_int64(int64)\0"))
                .as_ptr(),
        );
    }
    'c_6874: {
        if a > 9223372036854775806 as libc::c_long as libc::c_longlong {} else {
            __assert_fail(
                b"a > 9223372036854775806\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                164 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void pass_int64(int64)\0"))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn return_int64() -> int64 {
    return 9223372036854775807 as libc::c_long as int64;
}
pub unsafe extern "C" fn return_passed_int64(mut a: int64) -> int64 {
    return a;
}
pub unsafe extern "C" fn test_int64() {
    let mut i64: int64 = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    printf(b"testing int64... \0" as *const u8 as *const libc::c_char);
    pass_int64(9223372036854775807 as libc::c_long as int64);
    i64 = return_int64();
    if i64 == 9223372036854775807 as libc::c_long as libc::c_longlong {} else {
        __assert_fail(
            b"i64 == 9223372036854775807\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"void test_int64()\0"))
                .as_ptr(),
        );
    }
    'c_7151: {
        if i64 == 9223372036854775807 as libc::c_long as libc::c_longlong {} else {
            __assert_fail(
                b"i64 == 9223372036854775807\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void test_int64()\0"))
                    .as_ptr(),
            );
        }
    };
    i64toa(i64, buf.as_mut_ptr(), 10 as libc::c_int);
    if strcmp(
        buf.as_mut_ptr(),
        b"9223372036854775807\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(buf, \"9223372036854775807\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"void test_int64()\0"))
                .as_ptr(),
        );
    }
    'c_7095: {
        if strcmp(
            buf.as_mut_ptr(),
            b"9223372036854775807\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(buf, \"9223372036854775807\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                207 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void test_int64()\0"))
                    .as_ptr(),
            );
        }
    };
    i64 = return_passed_int64(-(987654321987654321 as libc::c_long) as int64);
    if i64 == -(987654321987654321 as libc::c_long) as libc::c_longlong {} else {
        __assert_fail(
            b"i64 == -987654321987654321\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"void test_int64()\0"))
                .as_ptr(),
        );
    }
    'c_7046: {
        if i64 == -(987654321987654321 as libc::c_long) as libc::c_longlong {} else {
            __assert_fail(
                b"i64 == -987654321987654321\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void test_int64()\0"))
                    .as_ptr(),
            );
        }
    };
    i64toa(i64, buf.as_mut_ptr(), 10 as libc::c_int);
    if strcmp(
        buf.as_mut_ptr(),
        b"-987654321987654321\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(buf, \"-987654321987654321\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"void test_int64()\0"))
                .as_ptr(),
        );
    }
    'c_6985: {
        if strcmp(
            buf.as_mut_ptr(),
            b"-987654321987654321\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(buf, \"-987654321987654321\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                214 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"void test_int64()\0"))
                    .as_ptr(),
            );
        }
    };
    printf(b"OK\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn AlmostEqualFloats(
    mut A: libc::c_float,
    mut B: libc::c_float,
    mut maxUlps: libc::c_int,
) -> BOOL {
    let mut aInt: libc::c_int = 0;
    let mut bInt: libc::c_int = 0;
    let mut intDiff: libc::c_int = 0;
    if maxUlps > 0 as libc::c_int
        && maxUlps < 4 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
    {} else {
        __assert_fail(
            b"maxUlps > 0 && maxUlps < 4 * 1024 * 1024\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"BOOL AlmostEqualFloats(float, float, int)\0"))
                .as_ptr(),
        );
    }
    'c_7285: {
        if maxUlps > 0 as libc::c_int
            && maxUlps < 4 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
        {} else {
            __assert_fail(
                b"maxUlps > 0 && maxUlps < 4 * 1024 * 1024\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                228 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"BOOL AlmostEqualFloats(float, float, int)\0"))
                    .as_ptr(),
            );
        }
    };
    aInt = *(&mut A as *mut libc::c_float as *mut libc::c_int);
    bInt = *(&mut B as *mut libc::c_float as *mut libc::c_int);
    if aInt < 0 as libc::c_int {
        aInt = (0x80000000 as libc::c_uint).wrapping_sub(aInt as libc::c_uint)
            as libc::c_int;
    }
    if bInt < 0 as libc::c_int {
        bInt = (0x80000000 as libc::c_uint).wrapping_sub(bInt as libc::c_uint)
            as libc::c_int;
    }
    intDiff = abs(aInt - bInt);
    if intDiff <= maxUlps {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn AlmostEqualDoubles(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> BOOL {
    let mut absDiff: libc::c_double = 0.;
    let mut maxAbs: libc::c_double = 0.;
    let mut absA: libc::c_double = 0.;
    let mut absB: libc::c_double = 0.;
    absDiff = fabs(a - b);
    if absDiff < 1.0E-150f64 {
        return 1 as libc::c_int;
    }
    absA = fabs(a);
    absB = fabs(b);
    maxAbs = if absA > absB { absA } else { absB };
    if absDiff / maxAbs < 1.0E-8f64 {
        return 1 as libc::c_int;
    }
    printf(b"a=%g b=%g\n\0" as *const u8 as *const libc::c_char, a, b);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn test_floating_point_numbers() {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut f1: libc::c_float = 0.;
    let mut d1: libc::c_double = 0.;
    printf(b"testing floating point... \0" as *const u8 as *const libc::c_char);
    f1 = 1.25f64 as libc::c_float;
    if f1 as libc::c_double == 1.25f64 {} else {
        __assert_fail(
            b"f1 == 1.25\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8390: {
        if f1 as libc::c_double == 1.25f64 {} else {
            __assert_fail(
                b"f1 == 1.25\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                277 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    d1 = 1.25f64;
    if d1 == 1.25f64 {} else {
        __assert_fail(
            b"d1 == 1.25\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8351: {
        if d1 == 1.25f64 {} else {
            __assert_fail(
                b"d1 == 1.25\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    d1 = 0 as libc::c_int as libc::c_double;
    d1 = f1 as libc::c_double;
    if d1 == 1.25f64 {} else {
        __assert_fail(
            b"d1 == 1.25\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            283 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8306: {
        if d1 == 1.25f64 {} else {
            __assert_fail(
                b"d1 == 1.25\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                283 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    f1 = 0 as libc::c_int as libc::c_float;
    f1 = d1 as libc::c_float;
    if f1 as libc::c_double == 1.25f64 {} else {
        __assert_fail(
            b"f1 == 1.25\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8259: {
        if f1 as libc::c_double == 1.25f64 {} else {
            __assert_fail(
                b"f1 == 1.25\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                286 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    d1 = 1.234f64;
    if AlmostEqualDoubles(d1, 1.234f64) == 1 as libc::c_int {} else {
        __assert_fail(
            b"AlmostEqualDoubles(d1, 1.234) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            289 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8212: {
        if AlmostEqualDoubles(d1, 1.234f64) == 1 as libc::c_int {} else {
            __assert_fail(
                b"AlmostEqualDoubles(d1, 1.234) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                289 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    f1 = d1 as libc::c_float;
    if AlmostEqualFloats(f1, 1.234f64 as libc::c_float, 2 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"AlmostEqualFloats(f1, 1.234, 2) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8159: {
        if AlmostEqualFloats(f1, 1.234f64 as libc::c_float, 2 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"AlmostEqualFloats(f1, 1.234, 2) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                291 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    d1 = 1.2345f64;
    if AlmostEqualDoubles(d1, 1.2345f64) == 1 as libc::c_int {} else {
        __assert_fail(
            b"AlmostEqualDoubles(d1, 1.2345) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8111: {
        if AlmostEqualDoubles(d1, 1.2345f64) == 1 as libc::c_int {} else {
            __assert_fail(
                b"AlmostEqualDoubles(d1, 1.2345) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                294 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    f1 = d1 as libc::c_float;
    if AlmostEqualFloats(f1, 1.2345f64 as libc::c_float, 2 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"AlmostEqualFloats(f1, 1.2345, 2) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8058: {
        if AlmostEqualFloats(f1, 1.2345f64 as libc::c_float, 2 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"AlmostEqualFloats(f1, 1.2345, 2) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                296 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    d1 = atof(b"1.234\0" as *const u8 as *const libc::c_char);
    if AlmostEqualDoubles(d1, 1.234f64) == 1 as libc::c_int {} else {
        __assert_fail(
            b"AlmostEqualDoubles(d1, 1.234) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_8006: {
        if AlmostEqualDoubles(d1, 1.234f64) == 1 as libc::c_int {} else {
            __assert_fail(
                b"AlmostEqualDoubles(d1, 1.234) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    f1 = d1 as libc::c_float;
    if AlmostEqualFloats(f1, 1.234f64 as libc::c_float, 2 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"AlmostEqualFloats(f1, 1.234, 2) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7953: {
        if AlmostEqualFloats(f1, 1.234f64 as libc::c_float, 2 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"AlmostEqualFloats(f1, 1.234, 2) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                304 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    sprintf(buf.as_mut_ptr(), b"%g\0" as *const u8 as *const libc::c_char, d1);
    if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {} else {
        __assert_fail(
            b"buf[0] != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7900: {
        if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {} else {
            __assert_fail(
                b"buf[0] != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                313 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(buf.as_mut_ptr(), b"1.234\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(buf, \"1.234\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            314 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7852: {
        if strcmp(buf.as_mut_ptr(), b"1.234\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(buf, \"1.234\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                314 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    d1 = atof(b"12.34\0" as *const u8 as *const libc::c_char);
    if d1 == 12.34f64 {} else {
        __assert_fail(
            b"d1 == 12.34\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7809: {
        if d1 == 12.34f64 {} else {
            __assert_fail(
                b"d1 == 12.34\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    f1 = d1 as libc::c_float;
    if AlmostEqualFloats(f1, 12.34f64 as libc::c_float, 2 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"AlmostEqualFloats(f1, 12.34, 2) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7756: {
        if AlmostEqualFloats(f1, 12.34f64 as libc::c_float, 2 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"AlmostEqualFloats(f1, 12.34, 2) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                320 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    sprintf(buf.as_mut_ptr(), b"%g\0" as *const u8 as *const libc::c_char, d1);
    if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {} else {
        __assert_fail(
            b"buf[0] != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7703: {
        if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {} else {
            __assert_fail(
                b"buf[0] != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                329 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(buf.as_mut_ptr(), b"12.34\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(buf, \"12.34\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7654: {
        if strcmp(buf.as_mut_ptr(), b"12.34\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(buf, \"12.34\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    d1 = atof(b"1.234e25\0" as *const u8 as *const libc::c_char);
    if AlmostEqualDoubles(d1, 1.234e25f64) == 1 as libc::c_int {} else {
        __assert_fail(
            b"AlmostEqualDoubles(d1, 1.234e25) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7600: {
        if AlmostEqualDoubles(d1, 1.234e25f64) == 1 as libc::c_int {} else {
            __assert_fail(
                b"AlmostEqualDoubles(d1, 1.234e25) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                334 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    f1 = d1 as libc::c_float;
    if AlmostEqualFloats(f1, 1.234e25f64 as libc::c_float, 2 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"AlmostEqualFloats(f1, 1.234e25, 2) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7544: {
        if AlmostEqualFloats(f1, 1.234e25f64 as libc::c_float, 2 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"AlmostEqualFloats(f1, 1.234e25, 2) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                336 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    sprintf(buf.as_mut_ptr(), b"%g\0" as *const u8 as *const libc::c_char, d1);
    if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {} else {
        __assert_fail(
            b"buf[0] != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_floating_point_numbers()\0"))
                .as_ptr(),
        );
    }
    'c_7484: {
        if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {} else {
            __assert_fail(
                b"buf[0] != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                339 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_floating_point_numbers()\0"))
                    .as_ptr(),
            );
        }
    };
    printf(b"OK\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn print_binn(mut map: *mut binn) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    p = binn_ptr(map as *mut libc::c_void) as *mut libc::c_uchar;
    size = binn_size(map as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < size {
        printf(
            b"%02x \0" as *const u8 as *const libc::c_char,
            *p.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
    puts(b"\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn test1() {
    static mut fix_size: libc::c_int = 512 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut blobsize: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obj1: *mut binn = 0 as *mut binn;
    let mut list: *mut binn = 0 as *mut binn;
    let mut map: *mut binn = 0 as *mut binn;
    let mut obj: *mut binn = 0 as *mut binn;
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut vbyte: libc::c_char = 0;
    let mut pblob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vint16: libc::c_short = 0;
    let mut vuint16: libc::c_ushort = 0;
    let mut vint32: libc::c_int = 0;
    let mut vuint32: libc::c_uint = 0;
    let mut vint64: libc::c_longlong = 0;
    let mut vuint64: libc::c_ulonglong = 0;
    printf(b"testing binn 1... \0" as *const u8 as *const libc::c_char);
    if CalcAllocation(512 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(512, 512) == 512\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            382 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18635: {
        if CalcAllocation(512 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(512, 512) == 512\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                382 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(510 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(510, 512) == 512\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            383 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18592: {
        if CalcAllocation(510 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(510, 512) == 512\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                383 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(1 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(1, 512) == 512\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18550: {
        if CalcAllocation(1 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(1, 512) == 512\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                384 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(0 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(0, 512) == 512\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            385 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18508: {
        if CalcAllocation(0 as libc::c_int, 512 as libc::c_int) == 512 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(0, 512) == 512\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                385 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(513 as libc::c_int, 512 as libc::c_int) == 1024 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(513, 512) == 1024\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18465: {
        if CalcAllocation(513 as libc::c_int, 512 as libc::c_int) == 1024 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(513, 512) == 1024\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                387 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(512 as libc::c_int + 256 as libc::c_int, 512 as libc::c_int)
        == 1024 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(512 + CHUNK_SIZE, 512) == 1024\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18419: {
        if CalcAllocation(512 as libc::c_int + 256 as libc::c_int, 512 as libc::c_int)
            == 1024 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(512 + CHUNK_SIZE, 512) == 1024\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                388 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(1025 as libc::c_int, 512 as libc::c_int) == 2048 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(1025, 512) == 2048\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18377: {
        if CalcAllocation(1025 as libc::c_int, 512 as libc::c_int) == 2048 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(1025, 512) == 2048\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                389 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(1025 as libc::c_int, 1024 as libc::c_int) == 2048 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(1025, 1024) == 2048\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            390 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18335: {
        if CalcAllocation(1025 as libc::c_int, 1024 as libc::c_int)
            == 2048 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(1025, 1024) == 2048\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                390 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if CalcAllocation(2100 as libc::c_int, 1024 as libc::c_int) == 4096 as libc::c_int
    {} else {
        __assert_fail(
            b"CalcAllocation(2100, 1024) == 4096\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            391 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18292: {
        if CalcAllocation(2100 as libc::c_int, 1024 as libc::c_int)
            == 4096 as libc::c_int
        {} else {
            __assert_fail(
                b"CalcAllocation(2100, 1024) == 4096\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                391 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_new(-(1 as libc::c_int), 0 as libc::c_int, 0 as *mut libc::c_void))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_new(-1, 0, NULL) == INVALID_BINN\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18240: {
        if (binn_new(-(1 as libc::c_int), 0 as libc::c_int, 0 as *mut libc::c_void))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_new(-1, 0, NULL) == INVALID_BINN\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                399 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_new(0 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void)).is_null()
    {} else {
        __assert_fail(
            b"binn_new(0, 0, NULL) == INVALID_BINN\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            400 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18190: {
        if (binn_new(0 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_new(0, 0, NULL) == INVALID_BINN\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                400 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_new(5 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void)).is_null()
    {} else {
        __assert_fail(
            b"binn_new(5, 0, NULL) == INVALID_BINN\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            401 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18139: {
        if (binn_new(5 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_new(5, 0, NULL) == INVALID_BINN\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                401 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_new(0xe1 as libc::c_int, -(1 as libc::c_int), 0 as *mut libc::c_void))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_new(BINN_MAP, -1, NULL) == INVALID_BINN\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            402 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18087: {
        if (binn_new(0xe1 as libc::c_int, -(1 as libc::c_int), 0 as *mut libc::c_void))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_new(BINN_MAP, -1, NULL) == INVALID_BINN\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                402 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = &mut obj1 as *mut *mut binn as *mut libc::c_char;
    if (binn_new(0xe1 as libc::c_int, -(1 as libc::c_int), ptr as *mut libc::c_void))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_new(BINN_MAP, -1, ptr) == INVALID_BINN\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            404 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_18029: {
        if (binn_new(0xe1 as libc::c_int, -(1 as libc::c_int), ptr as *mut libc::c_void))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_new(BINN_MAP, -1, ptr) == INVALID_BINN\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                404 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_new(
        0xe1 as libc::c_int,
        3 as libc::c_int - 1 as libc::c_int,
        ptr as *mut libc::c_void,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_new(BINN_MAP, MIN_BINN_SIZE-1, ptr) == INVALID_BINN\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17975: {
        if (binn_new(
            0xe1 as libc::c_int,
            3 as libc::c_int - 1 as libc::c_int,
            ptr as *mut libc::c_void,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_new(BINN_MAP, MIN_BINN_SIZE-1, ptr) == INVALID_BINN\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                405 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    obj1 = binn_new(0xe0 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void);
    if !obj1.is_null() {} else {
        __assert_fail(
            b"obj1 != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17927: {
        if !obj1.is_null() {} else {
            __assert_fail(
                b"obj1 != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"obj1->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            411 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17887: {
        if (*obj1).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"obj1->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                411 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).type_0 == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"obj1->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17847: {
        if (*obj1).type_0 == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"obj1->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj1->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17807: {
        if (*obj1).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj1->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                413 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*obj1).pbuf).is_null() {} else {
        __assert_fail(
            b"obj1->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17763: {
        if !((*obj1).pbuf).is_null() {} else {
            __assert_fail(
                b"obj1->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                414 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"obj1->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            415 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17723: {
        if (*obj1).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"obj1->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                415 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"obj1->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            416 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17683: {
        if (*obj1).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"obj1->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                416 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj1->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            417 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17643: {
        if (*obj1).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj1->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                417 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    binn_free(obj1);
    list = binn_new(0xe0 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void);
    if !list.is_null() {} else {
        __assert_fail(
            b"list != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17590: {
        if !list.is_null() {} else {
            __assert_fail(
                b"list != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                424 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    map = binn_new(0xe1 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void);
    if !map.is_null() {} else {
        __assert_fail(
            b"map != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            428 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17542: {
        if !map.is_null() {} else {
            __assert_fail(
                b"map != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                428 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    obj = binn_new(0xe2 as libc::c_int, 0 as libc::c_int, 0 as *mut libc::c_void);
    if !obj.is_null() {} else {
        __assert_fail(
            b"obj != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            432 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17494: {
        if !obj.is_null() {} else {
            __assert_fail(
                b"obj != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                432 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"list->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17454: {
        if (*list).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"list->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                434 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).type_0 == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"list->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            435 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17414: {
        if (*list).type_0 == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"list->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                435 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"list->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            436 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17374: {
        if (*list).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"list->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                436 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*list).pbuf).is_null() {} else {
        __assert_fail(
            b"list->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            437 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17330: {
        if !((*list).pbuf).is_null() {} else {
            __assert_fail(
                b"list->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                437 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"list->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17290: {
        if (*list).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"list->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                438 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"list->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17250: {
        if (*list).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"list->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                439 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"list->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17210: {
        if (*list).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"list->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                440 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"map->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            442 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17170: {
        if (*map).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"map->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                442 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).type_0 == 0xe1 as libc::c_int {} else {
        __assert_fail(
            b"map->type == BINN_MAP\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17130: {
        if (*map).type_0 == 0xe1 as libc::c_int {} else {
            __assert_fail(
                b"map->type == BINN_MAP\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                443 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"map->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            444 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17090: {
        if (*map).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"map->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                444 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*map).pbuf).is_null() {} else {
        __assert_fail(
            b"map->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            445 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17046: {
        if !((*map).pbuf).is_null() {} else {
            __assert_fail(
                b"map->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                445 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"map->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            446 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_17006: {
        if (*map).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"map->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                446 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"map->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            447 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16966: {
        if (*map).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"map->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                447 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"map->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            448 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16926: {
        if (*map).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"map->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                448 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"obj->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            450 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16886: {
        if (*obj).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"obj->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                450 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).type_0 == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"obj->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            451 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16846: {
        if (*obj).type_0 == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"obj->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                451 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            452 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16805: {
        if (*obj).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                452 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*obj).pbuf).is_null() {} else {
        __assert_fail(
            b"obj->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16761: {
        if !((*obj).pbuf).is_null() {} else {
            __assert_fail(
                b"obj->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                453 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"obj->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16721: {
        if (*obj).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"obj->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                454 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"obj->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            455 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16680: {
        if (*obj).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"obj->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                455 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            456 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16640: {
        if (*obj).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                456 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = malloc(fix_size as libc::c_ulong) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            462 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16589: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                462 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    obj1 = binn_new(0xe2 as libc::c_int, fix_size, ptr as *mut libc::c_void);
    if !obj1.is_null() {} else {
        __assert_fail(
            b"obj1 != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            465 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16539: {
        if !obj1.is_null() {} else {
            __assert_fail(
                b"obj1 != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                465 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"obj1->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16499: {
        if (*obj1).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"obj1->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                467 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).type_0 == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"obj1->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            468 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16459: {
        if (*obj1).type_0 == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"obj1->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                468 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj1->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            469 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16419: {
        if (*obj1).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj1->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                469 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*obj1).pbuf).is_null() {} else {
        __assert_fail(
            b"obj1->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            470 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16375: {
        if !((*obj1).pbuf).is_null() {} else {
            __assert_fail(
                b"obj1->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                470 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).alloc_size == fix_size {} else {
        __assert_fail(
            b"obj1->alloc_size == fix_size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            471 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16333: {
        if (*obj1).alloc_size == fix_size {} else {
            __assert_fail(
                b"obj1->alloc_size == fix_size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                471 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"obj1->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            472 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16293: {
        if (*obj1).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"obj1->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                472 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).pre_allocated == 1 as libc::c_int {} else {
        __assert_fail(
            b"obj1->pre_allocated == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16253: {
        if (*obj1).pre_allocated == 1 as libc::c_int {} else {
            __assert_fail(
                b"obj1->pre_allocated == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                473 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        list,
        55001 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(list, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            478 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16199: {
        if binn_map_set(
            list,
            55001 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(list, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                478 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        list,
        b"test\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(list, \"test\", BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16143: {
        if binn_object_set(
            list,
            b"test\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(list, \"test\", BINN_INT32, &i, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                479 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        map,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(map, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16091: {
        if binn_list_add(
            map,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(map, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                481 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        map,
        b"test\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(map, \"test\", BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_16035: {
        if binn_object_set(
            map,
            b"test\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(map, \"test\", BINN_INT32, &i, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                482 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        obj,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(obj, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15983: {
        if binn_list_add(
            obj,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(obj, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                484 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        obj,
        55001 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(obj, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            485 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15929: {
        if binn_map_set(
            obj,
            55001 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(obj, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                485 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        -(1 as libc::c_int),
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, -1, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15875: {
        if binn_list_add(
            list,
            -(1 as libc::c_int),
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, -1, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                488 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x1ffff as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, 0x1FFFF, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15823: {
        if binn_list_add(
            list,
            0x1ffff as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, 0x1FFFF, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                489 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        -(1 as libc::c_int),
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, -1, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            490 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15766: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            -(1 as libc::c_int),
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, -1, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                490 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x1ffff as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, 0x1FFFF, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            491 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15712: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x1ffff as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, 0x1FFFF, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                491 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", -1, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            492 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15654: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", -1, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x1ffff as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", 0x1FFFF, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15598: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x1ffff as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", 0x1FFFF, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                493 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(list, 0x21 as libc::c_int, 0 as *mut libc::c_void, 0 as libc::c_int)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT8, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            496 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15546: {
        if binn_list_add(
            list,
            0x21 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT8, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                496 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(list, 0x41 as libc::c_int, 0 as *mut libc::c_void, 0 as libc::c_int)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT16, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15494: {
        if binn_list_add(
            list,
            0x41 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT16, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                497 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(list, 0x61 as libc::c_int, 0 as *mut libc::c_void, 0 as libc::c_int)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT32, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            498 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15442: {
        if binn_list_add(
            list,
            0x61 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT32, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                498 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(list, 0x81 as libc::c_int, 0 as *mut libc::c_void, 0 as libc::c_int)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT64, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15390: {
        if binn_list_add(
            list,
            0x81 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT64, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                499 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x21 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT8, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            501 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15336: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x21 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT8, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                501 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x41 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT16, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            502 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15282: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x41 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT16, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                502 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x61 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT32, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15228: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x61 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT32, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x81 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT64, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            504 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15174: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x81 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT64, NULL, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                504 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x21 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT8, NULL, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            506 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15118: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x21 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT8, NULL, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                506 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x41 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT16, NULL, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            507 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15062: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x41 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT16, NULL, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                507 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT32, NULL, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            508 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_15006: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT32, NULL, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                508 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x81 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT64, NULL, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14950: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x81 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT64, NULL, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                509 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xc0 as libc::c_int,
        0 as *mut libc::c_void,
        -(1 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, NULL, -1) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14896: {
        if binn_list_add(
            list,
            0xc0 as libc::c_int,
            0 as *mut libc::c_void,
            -(1 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, NULL, -1) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                513 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xc0 as libc::c_int,
        0 as *mut libc::c_void,
        10 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, NULL, 10) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            514 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14844: {
        if binn_list_add(
            list,
            0xc0 as libc::c_int,
            0 as *mut libc::c_void,
            10 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, NULL, 10) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                514 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0xc0 as libc::c_int,
        0 as *mut libc::c_void,
        -(1 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_BLOB, NULL, -1) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            515 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14788: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0xc0 as libc::c_int,
            0 as *mut libc::c_void,
            -(1 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_BLOB, NULL, -1) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                515 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0xc0 as libc::c_int,
        0 as *mut libc::c_void,
        10 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_BLOB, NULL, 10) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14734: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0xc0 as libc::c_int,
            0 as *mut libc::c_void,
            10 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_BLOB, NULL, 10) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                516 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0xc0 as libc::c_int,
        0 as *mut libc::c_void,
        -(1 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_BLOB, NULL, -1) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            517 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14676: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0xc0 as libc::c_int,
            0 as *mut libc::c_void,
            -(1 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_BLOB, NULL, -1) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                517 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0xc0 as libc::c_int,
        0 as *mut libc::c_void,
        10 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_BLOB, NULL, 10) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            518 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14620: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0xc0 as libc::c_int,
            0 as *mut libc::c_void,
            10 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_BLOB, NULL, 10) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                518 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xc0 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        -(1 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, &i, -1) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            521 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14566: {
        if binn_list_add(
            list,
            0xc0 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            -(1 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, &i, -1) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                521 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xc0 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        -(15 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, &i, -15) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            522 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14511: {
        if binn_list_add(
            list,
            0xc0 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            -(15 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, &i, -15) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                522 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0xc0 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        -(1 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_BLOB, &i, -1) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            523 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14455: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0xc0 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            -(1 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_BLOB, &i, -1) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                523 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0xc0 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        -(15 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_BLOB, &i, -15) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            524 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14398: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0xc0 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            -(15 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_BLOB, &i, -15) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                524 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0xc0 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        -(1 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_BLOB, &i, -1) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            525 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14340: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0xc0 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            -(1 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_BLOB, &i, -1) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                525 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0xc0 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        -(15 as libc::c_int),
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_BLOB, &i, -15) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14282: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0xc0 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            -(15 as libc::c_int),
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_BLOB, &i, -15) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                526 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(list as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            533 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14231: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                533 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 0 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 0, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            534 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14181: {
        if binn_list_get_value(ptr as *mut libc::c_void, 0 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 0, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                534 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 1 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 1, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            535 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14131: {
        if binn_list_get_value(ptr as *mut libc::c_void, 1 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 1, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                535 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 2 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 2, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            536 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14081: {
        if binn_list_get_value(ptr as *mut libc::c_void, 2 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 2, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                536 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, -(1 as libc::c_int), &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, -1, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            537 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_14029: {
        if binn_list_get_value(ptr as *mut libc::c_void, -(1 as libc::c_int), &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, -1, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                537 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(map as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            540 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13978: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                540 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 0 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 0, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            541 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13928: {
        if binn_list_get_value(ptr as *mut libc::c_void, 0 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 0, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                541 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 1 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 1, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13878: {
        if binn_list_get_value(ptr as *mut libc::c_void, 1 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 1, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                542 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 2 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 2, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            543 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13828: {
        if binn_list_get_value(ptr as *mut libc::c_void, 2 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 2, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, -(1 as libc::c_int), &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, -1, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            544 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13776: {
        if binn_list_get_value(ptr as *mut libc::c_void, -(1 as libc::c_int), &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, -1, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                544 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(obj as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            547 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13725: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                547 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 0 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 0, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            548 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13675: {
        if binn_list_get_value(ptr as *mut libc::c_void, 0 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 0, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                548 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 1 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 1, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            549 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13625: {
        if binn_list_get_value(ptr as *mut libc::c_void, 1 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 1, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                549 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, 2 as libc::c_int, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, 2, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13575: {
        if binn_list_get_value(ptr as *mut libc::c_void, 2 as libc::c_int, &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, 2, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                550 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(ptr as *mut libc::c_void, -(1 as libc::c_int), &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(ptr, -1, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            551 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13520: {
        if binn_list_get_value(ptr as *mut libc::c_void, -(1 as libc::c_int), &mut value)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(ptr, -1, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                551 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0x1234 as libc::c_int;
    if binn_list_add(
        list,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            558 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13465: {
        if binn_list_add(
            list,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                558 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            559 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13411: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                559 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            560 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13357: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                560 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            561 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13301: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                561 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13245: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                562 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    vbyte = 255 as libc::c_int as libc::c_char;
    vint16 = -(32000 as libc::c_int) as libc::c_short;
    vuint16 = 65000 as libc::c_int as libc::c_ushort;
    vint32 = -(65000000 as libc::c_int);
    vuint32 = 65000000 as libc::c_int as libc::c_uint;
    vint64 = -(6500000000000000 as libc::c_long) as libc::c_longlong;
    vuint64 = 6500000000000000 as libc::c_long as libc::c_ulonglong;
    blobsize = 150 as libc::c_int;
    pblob = malloc(blobsize as libc::c_ulong) as *mut libc::c_char;
    if !pblob.is_null() {} else {
        __assert_fail(
            b"pblob != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            573 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13160: {
        if !pblob.is_null() {} else {
            __assert_fail(
                b"pblob != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                573 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    memset(pblob as *mut libc::c_void, 55 as libc::c_int, blobsize as libc::c_ulong);
    if binn_list_add(list, 0 as libc::c_int, 0 as *mut libc::c_void, 0 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            576 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13098: {
        if binn_list_add(
            list,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                576 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            577 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_13045: {
        if binn_list_add(
            list,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                577 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x41 as libc::c_int,
        &mut vint16 as *mut libc::c_short as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT16, &vint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            578 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12993: {
        if binn_list_add(
            list,
            0x41 as libc::c_int,
            &mut vint16 as *mut libc::c_short as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT16, &vint16, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                578 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x40 as libc::c_int,
        &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT16, &vuint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            579 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12941: {
        if binn_list_add(
            list,
            0x40 as libc::c_int,
            &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT16, &vuint16, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                579 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x61 as libc::c_int,
        &mut vint32 as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT32, &vint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            580 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12889: {
        if binn_list_add(
            list,
            0x61 as libc::c_int,
            &mut vint32 as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT32, &vint32, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                580 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT32, &vuint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12837: {
        if binn_list_add(
            list,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT32, &vuint32, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                581 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x81 as libc::c_int,
        &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT64, &vint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            582 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12784: {
        if binn_list_add(
            list,
            0x81 as libc::c_int,
            &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT64, &vint64, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                582 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x80 as libc::c_int,
        &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT64, &vuint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            583 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12732: {
        if binn_list_add(
            list,
            0x80 as libc::c_int,
            &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT64, &vuint64, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                583 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xa0 as libc::c_int,
        b"this is the string\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            584 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12679: {
        if binn_list_add(
            list,
            0xa0 as libc::c_int,
            b"this is the string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                584 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(list, 0xc0 as libc::c_int, pblob as *mut libc::c_void, blobsize)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, pblob, blobsize) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            585 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12625: {
        if binn_list_add(list, 0xc0 as libc::c_int, pblob as *mut libc::c_void, blobsize)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, pblob, blobsize) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                585 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99000 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99000, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            587 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12572: {
        if binn_map_set(
            map,
            99000 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99000, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                587 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99001 as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99001, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            588 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12518: {
        if binn_map_set(
            map,
            99001 as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99001, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                588 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99002 as libc::c_int,
        0x41 as libc::c_int,
        &mut vint16 as *mut libc::c_short as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99002, BINN_INT16, &vint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            589 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12464: {
        if binn_map_set(
            map,
            99002 as libc::c_int,
            0x41 as libc::c_int,
            &mut vint16 as *mut libc::c_short as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99002, BINN_INT16, &vint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                589 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99003 as libc::c_int,
        0x40 as libc::c_int,
        &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99003, BINN_UINT16, &vuint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            590 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12410: {
        if binn_map_set(
            map,
            99003 as libc::c_int,
            0x40 as libc::c_int,
            &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99003, BINN_UINT16, &vuint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                590 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99004 as libc::c_int,
        0x61 as libc::c_int,
        &mut vint32 as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99004, BINN_INT32, &vint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            591 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12356: {
        if binn_map_set(
            map,
            99004 as libc::c_int,
            0x61 as libc::c_int,
            &mut vint32 as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99004, BINN_INT32, &vint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                591 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99005 as libc::c_int,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99005, BINN_UINT32, &vuint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            592 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12302: {
        if binn_map_set(
            map,
            99005 as libc::c_int,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99005, BINN_UINT32, &vuint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                592 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99006 as libc::c_int,
        0x81 as libc::c_int,
        &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99006, BINN_INT64, &vint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            593 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12248: {
        if binn_map_set(
            map,
            99006 as libc::c_int,
            0x81 as libc::c_int,
            &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99006, BINN_INT64, &vint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                593 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99007 as libc::c_int,
        0x80 as libc::c_int,
        &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99007, BINN_UINT64, &vuint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            594 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12194: {
        if binn_map_set(
            map,
            99007 as libc::c_int,
            0x80 as libc::c_int,
            &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99007, BINN_UINT64, &vuint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                594 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99008 as libc::c_int,
        0xa0 as libc::c_int,
        b"this is the string\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99008, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            595 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12139: {
        if binn_map_set(
            map,
            99008 as libc::c_int,
            0xa0 as libc::c_int,
            b"this is the string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99008, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                595 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99009 as libc::c_int,
        0xc0 as libc::c_int,
        pblob as *mut libc::c_void,
        blobsize,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99009, BINN_BLOB, pblob, blobsize) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            596 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12083: {
        if binn_map_set(
            map,
            99009 as libc::c_int,
            0xc0 as libc::c_int,
            pblob as *mut libc::c_void,
            blobsize,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99009, BINN_BLOB, pblob, blobsize) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                596 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key0\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key0\", BINN_NULL, 0, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            598 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_12029: {
        if binn_object_set(
            obj,
            b"key0\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key0\", BINN_NULL, 0, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                598 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key1\0" as *const u8 as *const libc::c_char,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key1\", BINN_UINT8, &vbyte, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            599 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11973: {
        if binn_object_set(
            obj,
            b"key1\0" as *const u8 as *const libc::c_char,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key1\", BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                599 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key2\0" as *const u8 as *const libc::c_char,
        0x41 as libc::c_int,
        &mut vint16 as *mut libc::c_short as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key2\", BINN_INT16, &vint16, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            600 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11916: {
        if binn_object_set(
            obj,
            b"key2\0" as *const u8 as *const libc::c_char,
            0x41 as libc::c_int,
            &mut vint16 as *mut libc::c_short as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key2\", BINN_INT16, &vint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                600 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key3\0" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
        &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key3\", BINN_UINT16, &vuint16, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11859: {
        if binn_object_set(
            obj,
            b"key3\0" as *const u8 as *const libc::c_char,
            0x40 as libc::c_int,
            &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key3\", BINN_UINT16, &vuint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                601 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key4\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut vint32 as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key4\", BINN_INT32, &vint32, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            602 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11802: {
        if binn_object_set(
            obj,
            b"key4\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut vint32 as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key4\", BINN_INT32, &vint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                602 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key5\0" as *const u8 as *const libc::c_char,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key5\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            603 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11746: {
        if binn_object_set(
            obj,
            b"key5\0" as *const u8 as *const libc::c_char,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key5\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                603 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key6\0" as *const u8 as *const libc::c_char,
        0x81 as libc::c_int,
        &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key6\", BINN_INT64, &vint64, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            604 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11688: {
        if binn_object_set(
            obj,
            b"key6\0" as *const u8 as *const libc::c_char,
            0x81 as libc::c_int,
            &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key6\", BINN_INT64, &vint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                604 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key7\0" as *const u8 as *const libc::c_char,
        0x80 as libc::c_int,
        &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key7\", BINN_UINT64, &vuint64, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            605 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11630: {
        if binn_object_set(
            obj,
            b"key7\0" as *const u8 as *const libc::c_char,
            0x80 as libc::c_int,
            &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key7\", BINN_UINT64, &vuint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                605 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key8\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"this is the string\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key8\", BINN_STRING, \"this is the string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            606 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11574: {
        if binn_object_set(
            obj,
            b"key8\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"this is the string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key8\", BINN_STRING, \"this is the string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                606 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key9\0" as *const u8 as *const libc::c_char,
        0xc0 as libc::c_int,
        pblob as *mut libc::c_void,
        blobsize,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key9\", BINN_BLOB, pblob, blobsize) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            607 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11514: {
        if binn_object_set(
            obj,
            b"key9\0" as *const u8 as *const libc::c_char,
            0xc0 as libc::c_int,
            pblob as *mut libc::c_void,
            blobsize,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key9\", BINN_BLOB, pblob, blobsize) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                607 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xc0 as libc::c_int,
        ptr as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, ptr, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            610 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11461: {
        if binn_list_add(
            list,
            0xc0 as libc::c_int,
            ptr as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, ptr, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                610 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xa0 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_STRING, \"\", 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            611 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11409: {
        if binn_list_add(
            list,
            0xa0 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_STRING, \"\", 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                611 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xa0 as libc::c_int,
        b"after the empty items\0" as *const u8 as *const libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_STRING, \"after the empty items\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            612 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11356: {
        if binn_list_add(
            list,
            0xa0 as libc::c_int,
            b"after the empty items\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_STRING, \"after the empty items\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                612 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x9 as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x09, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            618 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11302: {
        if binn_map_set(
            map,
            0x9 as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x09, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                618 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x3f as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x3F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            619 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11248: {
        if binn_map_set(
            map,
            0x3f as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x3F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                619 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x4f as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x4F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            620 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11193: {
        if binn_map_set(
            map,
            0x4f as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x4F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                620 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0xfff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0xFFF, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            621 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11139: {
        if binn_map_set(
            map,
            0xfff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0xFFF, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                621 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0xfffff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0xFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            622 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11085: {
        if binn_map_set(
            map,
            0xfffff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0xFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                622 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0xfffffff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0xFFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            623 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_11031: {
        if binn_map_set(
            map,
            0xfffffff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0xFFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                623 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x7fffffff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            624 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10977: {
        if binn_map_set(
            map,
            0x7fffffff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                624 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x9 as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x09, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            626 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10921: {
        if binn_map_set(
            map,
            -(0x9 as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x09, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                626 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x3f as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x3F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            627 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10865: {
        if binn_map_set(
            map,
            -(0x3f as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x3F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                627 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x4f as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x4F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            628 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10809: {
        if binn_map_set(
            map,
            -(0x4f as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x4F, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                628 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0xfff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0xFFF, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            629 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10753: {
        if binn_map_set(
            map,
            -(0xfff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0xFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                629 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0xfffff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0xFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            630 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10697: {
        if binn_map_set(
            map,
            -(0xfffff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0xFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                630 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0xfffffff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0xFFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            631 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10641: {
        if binn_map_set(
            map,
            -(0xfffffff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0xFFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                631 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x7fffffff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            632 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10585: {
        if binn_map_set(
            map,
            -(0x7fffffff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                632 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x9 as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x09, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            635 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10531: {
        if binn_map_set(
            map,
            0x9 as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x09, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                635 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x3f as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x3F, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            636 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10477: {
        if binn_map_set(
            map,
            0x3f as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x3F, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                636 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x4f as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x4F, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            637 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10423: {
        if binn_map_set(
            map,
            0x4f as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x4F, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                637 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0xfff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0xFFF, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            638 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10369: {
        if binn_map_set(
            map,
            0xfff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0xFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                638 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0xfffff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0xFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            639 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10314: {
        if binn_map_set(
            map,
            0xfffff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0xFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                639 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0xfffffff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0xFFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            640 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10259: {
        if binn_map_set(
            map,
            0xfffffff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0xFFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                640 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        0x7fffffff as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            641 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10205: {
        if binn_map_set(
            map,
            0x7fffffff as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                641 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x9 as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x09, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            643 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10149: {
        if binn_map_set(
            map,
            -(0x9 as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x09, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                643 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x3f as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x3F, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            644 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10093: {
        if binn_map_set(
            map,
            -(0x3f as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x3F, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                644 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x4f as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x4F, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            645 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_10036: {
        if binn_map_set(
            map,
            -(0x4f as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x4F, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                645 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0xfff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0xFFF, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            646 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9980: {
        if binn_map_set(
            map,
            -(0xfff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0xFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                646 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0xfffff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0xFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            647 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9923: {
        if binn_map_set(
            map,
            -(0xfffff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0xFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                647 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0xfffffff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0xFFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            648 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9866: {
        if binn_map_set(
            map,
            -(0xfffffff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0xFFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                648 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        -(0x7fffffff as libc::c_int),
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, -0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            649 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9808: {
        if binn_map_set(
            map,
            -(0x7fffffff as libc::c_int),
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, -0x7FFFFFFF, BINN_UINT8, &vbyte, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                649 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        obj1,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(obj1, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            654 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9755: {
        if binn_list_add(
            obj1,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(obj1, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                654 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        obj1,
        55001 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(obj1, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            655 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9700: {
        if binn_map_set(
            obj1,
            55001 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(obj1, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                655 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"test\0" as *const u8 as *const libc::c_char,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            657 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9643: {
        if binn_object_set(
            obj1,
            b"test\0" as *const u8 as *const libc::c_char,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                657 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"test\0" as *const u8 as *const libc::c_char,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            658 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9585: {
        if binn_object_set(
            obj1,
            b"test\0" as *const u8 as *const libc::c_char,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                658 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"key1\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"this is the value\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"key1\", BINN_STRING, \"this is the value\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            660 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9528: {
        if binn_object_set(
            obj1,
            b"key1\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"this is the value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"key1\", BINN_STRING, \"this is the value\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                660 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"key2\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"the second value\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"key2\", BINN_STRING, \"the second value\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            661 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9471: {
        if binn_object_set(
            obj1,
            b"key2\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"the second value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"key2\", BINN_STRING, \"the second value\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                661 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = malloc(fix_size as libc::c_ulong) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            666 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9420: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                666 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    p2 = ptr;
    i = 0 as libc::c_int;
    while i < fix_size - 1 as libc::c_int {
        *p2 = 'A' as i32 as libc::c_char;
        p2 = p2.offset(1);
        p2;
        i += 1;
        i;
    }
    *p2 = '\0' as i32 as libc::c_char;
    if strlen(ptr) == (fix_size - 1 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"strlen(ptr) == fix_size - 1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            672 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9329: {
        if strlen(ptr) == (fix_size - 1 as libc::c_int) as libc::c_ulong {} else {
            __assert_fail(
                b"strlen(ptr) == fix_size - 1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                672 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"v2\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        ptr as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"v2\", BINN_STRING, ptr, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9272: {
        if binn_object_set(
            obj1,
            b"v2\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            ptr as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"v2\", BINN_STRING, ptr, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                674 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"v2\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        ptr as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"v2\", BINN_STRING, ptr, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            676 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9215: {
        if binn_object_set(
            obj,
            b"v2\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            ptr as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"v2\", BINN_STRING, ptr, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                676 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"Key00\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"after the big string\0" as *const u8 as *const libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"Key00\", BINN_STRING, \"after the big string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            677 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9158: {
        if binn_object_set(
            obj,
            b"Key00\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"after the big string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"Key00\", BINN_STRING, \"after the big string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                677 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    free(ptr as *mut libc::c_void);
    ptr = 0 as *mut libc::c_char;
    if binn_object_set(
        obj,
        b"list\0" as *const u8 as *const libc::c_char,
        0xe0 as libc::c_int,
        binn_ptr(list as *mut libc::c_void),
        binn_size(list as *mut libc::c_void),
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"list\", BINN_LIST, binn_ptr(list), binn_size(list)) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            681 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9073: {
        if binn_object_set(
            obj,
            b"list\0" as *const u8 as *const libc::c_char,
            0xe0 as libc::c_int,
            binn_ptr(list as *mut libc::c_void),
            binn_size(list as *mut libc::c_void),
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"list\", BINN_LIST, binn_ptr(list), binn_size(list)) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                681 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"Key10\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"after the list\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"Key10\", BINN_STRING, \"after the list\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            682 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_9016: {
        if binn_object_set(
            obj,
            b"Key10\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"after the list\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"Key10\", BINN_STRING, \"after the list\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                682 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(0 as *mut libc::c_void) == 0 as libc::c_int {} else {
        __assert_fail(
            b"binn_size(NULL) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            698 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_8971: {
        if binn_size(0 as *mut libc::c_void) == 0 as libc::c_int {} else {
            __assert_fail(
                b"binn_size(NULL) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                698 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(list as *mut libc::c_void) == (*list).size {} else {
        __assert_fail(
            b"binn_size(list) == list->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            700 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_8921: {
        if binn_size(list as *mut libc::c_void) == (*list).size {} else {
            __assert_fail(
                b"binn_size(list) == list->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                700 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(map as *mut libc::c_void) == (*map).size {} else {
        __assert_fail(
            b"binn_size(map) == map->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            701 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_8871: {
        if binn_size(map as *mut libc::c_void) == (*map).size {} else {
            __assert_fail(
                b"binn_size(map) == map->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                701 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(obj as *mut libc::c_void) == (*obj).size {} else {
        __assert_fail(
            b"binn_size(obj) == obj->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            702 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_8820: {
        if binn_size(obj as *mut libc::c_void) == (*obj).size {} else {
            __assert_fail(
                b"binn_size(obj) == obj->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                702 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(obj1 as *mut libc::c_void) == (*obj1).size {} else {
        __assert_fail(
            b"binn_size(obj1) == obj1->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            703 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test1()\0"))
                .as_ptr(),
        );
    }
    'c_8768: {
        if binn_size(obj1 as *mut libc::c_void) == (*obj1).size {} else {
            __assert_fail(
                b"binn_size(obj1) == obj1->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                703 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test1()\0"))
                    .as_ptr(),
            );
        }
    };
    binn_free(list);
    binn_free(map);
    binn_free(obj);
    binn_free(obj1);
    printf(b"OK\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn test2(mut use_int_compression: BOOL) {
    let mut list: *mut binn = 0 as *mut binn;
    let mut map: *mut binn = 0 as *mut binn;
    let mut obj: *mut binn = 0 as *mut binn;
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut vbool: BOOL = 0;
    let mut blobsize: libc::c_int = 0;
    let mut pblob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vint32: libc::c_int = 0;
    let mut vdouble: libc::c_double = 0.;
    let mut str_list: *mut libc::c_char = b"test list\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut str_map: *mut libc::c_char = b"test map\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut str_obj: *mut libc::c_char = b"test object\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    printf(
        b"testing binn 2 (use_int_compression = %d)... \0" as *const u8
            as *const libc::c_char,
        use_int_compression,
    );
    blobsize = 150 as libc::c_int;
    pblob = malloc(blobsize as libc::c_ulong) as *mut libc::c_char;
    if !pblob.is_null() {} else {
        __assert_fail(
            b"pblob != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            737 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_30271: {
        if !pblob.is_null() {} else {
            __assert_fail(
                b"pblob != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                737 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(pblob as *mut libc::c_void, 55 as libc::c_int, blobsize as libc::c_ulong);
    if list.is_null() {} else {
        __assert_fail(
            b"list == INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            740 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_30223: {
        if list.is_null() {} else {
            __assert_fail(
                b"list == INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                740 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if map.is_null() {} else {
        __assert_fail(
            b"map == INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            741 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_30185: {
        if map.is_null() {} else {
            __assert_fail(
                b"map == INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                741 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if obj.is_null() {} else {
        __assert_fail(
            b"obj == INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            742 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_30147: {
        if obj.is_null() {} else {
            __assert_fail(
                b"obj == INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                742 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add_int32(list, 123 as libc::c_int) == 0 as libc::c_int {} else {
        __assert_fail(
            b"binn_list_add_int32(list, 123) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            746 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_30103: {
        if binn_list_add_int32(list, 123 as libc::c_int) == 0 as libc::c_int {} else {
            __assert_fail(
                b"binn_list_add_int32(list, 123) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                746 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set_int32(map, 1001 as libc::c_int, 456 as libc::c_int)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set_int32(map, 1001, 456) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            747 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_30057: {
        if binn_map_set_int32(map, 1001 as libc::c_int, 456 as libc::c_int)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set_int32(map, 1001, 456) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                747 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set_int32(
        obj,
        b"int\0" as *const u8 as *const libc::c_char,
        789 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set_int32(obj, \"int\", 789) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            748 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_30009: {
        if binn_object_set_int32(
            obj,
            b"int\0" as *const u8 as *const libc::c_char,
            789 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set_int32(obj, \"int\", 789) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                748 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    list = binn_list();
    map = binn_map();
    obj = binn_object();
    if !list.is_null() {} else {
        __assert_fail(
            b"list != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            756 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29955: {
        if !list.is_null() {} else {
            __assert_fail(
                b"list != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                756 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !map.is_null() {} else {
        __assert_fail(
            b"map != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            757 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29917: {
        if !map.is_null() {} else {
            __assert_fail(
                b"map != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                757 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !obj.is_null() {} else {
        __assert_fail(
            b"obj != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            758 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29879: {
        if !obj.is_null() {} else {
            __assert_fail(
                b"obj != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                758 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if use_int_compression == 0 as libc::c_int {
        (*list).disable_int_compression = 1 as libc::c_int;
        (*map).disable_int_compression = 1 as libc::c_int;
        (*obj).disable_int_compression = 1 as libc::c_int;
    }
    if binn_list_add_int32(list, 123 as libc::c_int) == 1 as libc::c_int {} else {
        __assert_fail(
            b"binn_list_add_int32(list, 123) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            768 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29811: {
        if binn_list_add_int32(list, 123 as libc::c_int) == 1 as libc::c_int {} else {
            __assert_fail(
                b"binn_list_add_int32(list, 123) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                768 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set_int32(map, 1001 as libc::c_int, 456 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set_int32(map, 1001, 456) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            769 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29764: {
        if binn_map_set_int32(map, 1001 as libc::c_int, 456 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set_int32(map, 1001, 456) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                769 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set_int32(
        obj,
        b"int\0" as *const u8 as *const libc::c_char,
        789 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set_int32(obj, \"int\", 789) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            770 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29715: {
        if binn_object_set_int32(
            obj,
            b"int\0" as *const u8 as *const libc::c_char,
            789 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set_int32(obj, \"int\", 789) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                770 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"list->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            774 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29675: {
        if (*list).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"list->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                774 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).type_0 == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"list->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            775 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29635: {
        if (*list).type_0 == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"list->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                775 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).count == 1 as libc::c_int {} else {
        __assert_fail(
            b"list->count == 1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            776 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29595: {
        if (*list).count == 1 as libc::c_int {} else {
            __assert_fail(
                b"list->count == 1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                776 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*list).pbuf).is_null() {} else {
        __assert_fail(
            b"list->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            777 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29551: {
        if !((*list).pbuf).is_null() {} else {
            __assert_fail(
                b"list->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                777 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"list->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            778 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29511: {
        if (*list).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"list->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                778 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).used_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"list->used_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            779 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29471: {
        if (*list).used_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"list->used_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                779 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"list->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            780 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29431: {
        if (*list).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"list->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                780 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"map->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            782 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29391: {
        if (*map).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"map->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                782 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).type_0 == 0xe1 as libc::c_int {} else {
        __assert_fail(
            b"map->type == BINN_MAP\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            783 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29351: {
        if (*map).type_0 == 0xe1 as libc::c_int {} else {
            __assert_fail(
                b"map->type == BINN_MAP\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                783 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).count == 1 as libc::c_int {} else {
        __assert_fail(
            b"map->count == 1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            784 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29311: {
        if (*map).count == 1 as libc::c_int {} else {
            __assert_fail(
                b"map->count == 1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                784 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*map).pbuf).is_null() {} else {
        __assert_fail(
            b"map->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            785 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29267: {
        if !((*map).pbuf).is_null() {} else {
            __assert_fail(
                b"map->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                785 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"map->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            786 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29227: {
        if (*map).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"map->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                786 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).used_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"map->used_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            787 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29187: {
        if (*map).used_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"map->used_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                787 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"map->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            788 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29147: {
        if (*map).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"map->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                788 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"obj->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            790 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29107: {
        if (*obj).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"obj->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                790 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).type_0 == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"obj->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            791 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29067: {
        if (*obj).type_0 == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"obj->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                791 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).count == 1 as libc::c_int {} else {
        __assert_fail(
            b"obj->count == 1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            792 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_29027: {
        if (*obj).count == 1 as libc::c_int {} else {
            __assert_fail(
                b"obj->count == 1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                792 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*obj).pbuf).is_null() {} else {
        __assert_fail(
            b"obj->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            793 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28983: {
        if !((*obj).pbuf).is_null() {} else {
            __assert_fail(
                b"obj->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                793 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"obj->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            794 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28943: {
        if (*obj).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"obj->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                794 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).used_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"obj->used_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            795 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28903: {
        if (*obj).used_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"obj->used_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                795 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            796 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28863: {
        if (*obj).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                796 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add_double(list, 1.23f64) == 1 as libc::c_int {} else {
        __assert_fail(
            b"binn_list_add_double(list, 1.23) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            801 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28818: {
        if binn_list_add_double(list, 1.23f64) == 1 as libc::c_int {} else {
            __assert_fail(
                b"binn_list_add_double(list, 1.23) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                801 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set_double(map, 1002 as libc::c_int, 4.56f64) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set_double(map, 1002, 4.56) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            802 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28771: {
        if binn_map_set_double(map, 1002 as libc::c_int, 4.56f64) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set_double(map, 1002, 4.56) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                802 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set_double(
        obj,
        b"double\0" as *const u8 as *const libc::c_char,
        7.89f64,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set_double(obj, \"double\", 7.89) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            803 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28722: {
        if binn_object_set_double(
            obj,
            b"double\0" as *const u8 as *const libc::c_char,
            7.89f64,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set_double(obj, \"double\", 7.89) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                803 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).count == 2 as libc::c_int {} else {
        __assert_fail(
            b"list->count == 2\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            805 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28682: {
        if (*list).count == 2 as libc::c_int {} else {
            __assert_fail(
                b"list->count == 2\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                805 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).count == 2 as libc::c_int {} else {
        __assert_fail(
            b"map->count == 2\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            806 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28642: {
        if (*map).count == 2 as libc::c_int {} else {
            __assert_fail(
                b"map->count == 2\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                806 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).count == 2 as libc::c_int {} else {
        __assert_fail(
            b"obj->count == 2\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            807 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28602: {
        if (*obj).count == 2 as libc::c_int {} else {
            __assert_fail(
                b"obj->count == 2\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                807 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add_bool(list, 1 as libc::c_int) == 1 as libc::c_int {} else {
        __assert_fail(
            b"binn_list_add_bool(list, TRUE) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            809 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28557: {
        if binn_list_add_bool(list, 1 as libc::c_int) == 1 as libc::c_int {} else {
            __assert_fail(
                b"binn_list_add_bool(list, TRUE) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                809 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set_bool(map, 1003 as libc::c_int, 1 as libc::c_int) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set_bool(map, 1003, TRUE) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            810 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28510: {
        if binn_map_set_bool(map, 1003 as libc::c_int, 1 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set_bool(map, 1003, TRUE) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                810 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set_bool(
        obj,
        b"bool\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set_bool(obj, \"bool\", TRUE) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            811 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28461: {
        if binn_object_set_bool(
            obj,
            b"bool\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set_bool(obj, \"bool\", TRUE) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                811 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).count == 3 as libc::c_int {} else {
        __assert_fail(
            b"list->count == 3\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            813 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28421: {
        if (*list).count == 3 as libc::c_int {} else {
            __assert_fail(
                b"list->count == 3\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                813 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).count == 3 as libc::c_int {} else {
        __assert_fail(
            b"map->count == 3\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            814 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28381: {
        if (*map).count == 3 as libc::c_int {} else {
            __assert_fail(
                b"map->count == 3\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                814 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).count == 3 as libc::c_int {} else {
        __assert_fail(
            b"obj->count == 3\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            815 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28341: {
        if (*obj).count == 3 as libc::c_int {} else {
            __assert_fail(
                b"obj->count == 3\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                815 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add_str(list, str_list) == 1 as libc::c_int {} else {
        __assert_fail(
            b"binn_list_add_str(list, str_list) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            817 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28294: {
        if binn_list_add_str(list, str_list) == 1 as libc::c_int {} else {
            __assert_fail(
                b"binn_list_add_str(list, str_list) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                817 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set_str(map, 1004 as libc::c_int, str_map) == 1 as libc::c_int {} else {
        __assert_fail(
            b"binn_map_set_str(map, 1004, str_map) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            818 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28245: {
        if binn_map_set_str(map, 1004 as libc::c_int, str_map) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set_str(map, 1004, str_map) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                818 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set_str(obj, b"text\0" as *const u8 as *const libc::c_char, str_obj)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set_str(obj, \"text\", str_obj) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            819 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28194: {
        if binn_object_set_str(
            obj,
            b"text\0" as *const u8 as *const libc::c_char,
            str_obj,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set_str(obj, \"text\", str_obj) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                819 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).count == 4 as libc::c_int {} else {
        __assert_fail(
            b"list->count == 4\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            821 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28154: {
        if (*list).count == 4 as libc::c_int {} else {
            __assert_fail(
                b"list->count == 4\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                821 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).count == 4 as libc::c_int {} else {
        __assert_fail(
            b"map->count == 4\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            822 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28114: {
        if (*map).count == 4 as libc::c_int {} else {
            __assert_fail(
                b"map->count == 4\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                822 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).count == 4 as libc::c_int {} else {
        __assert_fail(
            b"obj->count == 4\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            823 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28074: {
        if (*obj).count == 4 as libc::c_int {} else {
            __assert_fail(
                b"obj->count == 4\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                823 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add_blob(list, pblob as *mut libc::c_void, blobsize) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add_blob(list, pblob, blobsize) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            825 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_28021: {
        if binn_list_add_blob(list, pblob as *mut libc::c_void, blobsize)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add_blob(list, pblob, blobsize) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                825 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set_blob(map, 1005 as libc::c_int, pblob as *mut libc::c_void, blobsize)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set_blob(map, 1005, pblob, blobsize) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            826 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27967: {
        if binn_map_set_blob(
            map,
            1005 as libc::c_int,
            pblob as *mut libc::c_void,
            blobsize,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set_blob(map, 1005, pblob, blobsize) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                826 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set_blob(
        obj,
        b"blob\0" as *const u8 as *const libc::c_char,
        pblob as *mut libc::c_void,
        blobsize,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set_blob(obj, \"blob\", pblob, blobsize) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            827 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27910: {
        if binn_object_set_blob(
            obj,
            b"blob\0" as *const u8 as *const libc::c_char,
            pblob as *mut libc::c_void,
            blobsize,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set_blob(obj, \"blob\", pblob, blobsize) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                827 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).count == 5 as libc::c_int {} else {
        __assert_fail(
            b"list->count == 5\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            829 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27870: {
        if (*list).count == 5 as libc::c_int {} else {
            __assert_fail(
                b"list->count == 5\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                829 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).count == 5 as libc::c_int {} else {
        __assert_fail(
            b"map->count == 5\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            830 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27830: {
        if (*map).count == 5 as libc::c_int {} else {
            __assert_fail(
                b"map->count == 5\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                830 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).count == 5 as libc::c_int {} else {
        __assert_fail(
            b"obj->count == 5\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            831 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27790: {
        if (*obj).count == 5 as libc::c_int {} else {
            __assert_fail(
                b"obj->count == 5\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                831 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_count(list as *mut libc::c_void) == 5 as libc::c_int {} else {
        __assert_fail(
            b"binn_count(list) == 5\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            833 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27746: {
        if binn_count(list as *mut libc::c_void) == 5 as libc::c_int {} else {
            __assert_fail(
                b"binn_count(list) == 5\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                833 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_count(map as *mut libc::c_void) == 5 as libc::c_int {} else {
        __assert_fail(
            b"binn_count(map) == 5\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            834 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27702: {
        if binn_count(map as *mut libc::c_void) == 5 as libc::c_int {} else {
            __assert_fail(
                b"binn_count(map) == 5\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                834 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_count(obj as *mut libc::c_void) == 5 as libc::c_int {} else {
        __assert_fail(
            b"binn_count(obj) == 5\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            835 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27658: {
        if binn_count(obj as *mut libc::c_void) == 5 as libc::c_int {} else {
            __assert_fail(
                b"binn_count(obj) == 5\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                835 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(list as *mut libc::c_void) == (*list).size {} else {
        __assert_fail(
            b"binn_size(list) == list->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            837 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27608: {
        if binn_size(list as *mut libc::c_void) == (*list).size {} else {
            __assert_fail(
                b"binn_size(list) == list->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                837 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(map as *mut libc::c_void) == (*map).size {} else {
        __assert_fail(
            b"binn_size(map) == map->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            838 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27558: {
        if binn_size(map as *mut libc::c_void) == (*map).size {} else {
            __assert_fail(
                b"binn_size(map) == map->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                838 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(obj as *mut libc::c_void) == (*obj).size {} else {
        __assert_fail(
            b"binn_size(obj) == obj->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            839 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27508: {
        if binn_size(obj as *mut libc::c_void) == (*obj).size {} else {
            __assert_fail(
                b"binn_size(obj) == obj->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                839 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_type(list as *mut libc::c_void) == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"binn_type(list) == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            841 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27464: {
        if binn_type(list as *mut libc::c_void) == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"binn_type(list) == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                841 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_type(map as *mut libc::c_void) == 0xe1 as libc::c_int {} else {
        __assert_fail(
            b"binn_type(map) == BINN_MAP\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            842 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27420: {
        if binn_type(map as *mut libc::c_void) == 0xe1 as libc::c_int {} else {
            __assert_fail(
                b"binn_type(map) == BINN_MAP\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                842 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_type(obj as *mut libc::c_void) == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"binn_type(obj) == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            843 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27376: {
        if binn_type(obj as *mut libc::c_void) == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"binn_type(obj) == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                843 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_value(list as *mut libc::c_void, 1 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(list, 1, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            850 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27326: {
        if binn_list_get_value(list as *mut libc::c_void, 1 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(list, 1, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                850 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            852 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27288: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                852 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            853 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27250: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                853 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            854 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_27212: {
        if value.allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                854 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if use_int_compression != 0 {
        if value.type_0 == 0x20 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                856 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_27173: {
            if value.type_0 == 0x20 as libc::c_int {} else {
                __assert_fail(
                    b"value.type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    856 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
        if value.ptr
            != &mut value.c2rust_unnamed.vuint8 as *mut libc::c_uchar
                as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr != &value.vuint8\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                857 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_27127: {
            if value.ptr
                != &mut value.c2rust_unnamed.vuint8 as *mut libc::c_uchar
                    as *mut libc::c_void
            {} else {
                __assert_fail(
                    b"value.ptr != &value.vuint8\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    857 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
    } else {
        if value.type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                859 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_27087: {
            if value.type_0 == 0x61 as libc::c_int {} else {
                __assert_fail(
                    b"value.type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    859 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                860 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_27041: {
            if value.ptr
                == &mut value.c2rust_unnamed.vint as *mut libc::c_int
                    as *mut libc::c_void
            {} else {
                __assert_fail(
                    b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    860 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            862 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26998: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                862 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            863 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26960: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                863 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vint == 123 as libc::c_int {} else {
        __assert_fail(
            b"value.vint == 123\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            864 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26920: {
        if value.c2rust_unnamed.vint == 123 as libc::c_int {} else {
            __assert_fail(
                b"value.vint == 123\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                864 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_map_get_value(map as *mut libc::c_void, 1001 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_value(map, 1001, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            868 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26862: {
        if binn_map_get_value(map as *mut libc::c_void, 1001 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_value(map, 1001, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                868 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            870 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26824: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                870 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            871 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26786: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                871 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if use_int_compression != 0 {
        if value.type_0 == 0x40 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                873 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26747: {
            if value.type_0 == 0x40 as libc::c_int {} else {
                __assert_fail(
                    b"value.type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    873 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
        if value.ptr
            == &mut value.c2rust_unnamed.vuint16 as *mut libc::c_ushort
                as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vuint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                874 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26701: {
            if value.ptr
                == &mut value.c2rust_unnamed.vuint16 as *mut libc::c_ushort
                    as *mut libc::c_void
            {} else {
                __assert_fail(
                    b"value.ptr == &value.vuint16\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    874 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
    } else {
        if value.type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                876 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26661: {
            if value.type_0 == 0x61 as libc::c_int {} else {
                __assert_fail(
                    b"value.type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    876 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                877 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26615: {
            if value.ptr
                == &mut value.c2rust_unnamed.vint as *mut libc::c_int
                    as *mut libc::c_void
            {} else {
                __assert_fail(
                    b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    877 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            879 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26572: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                879 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            880 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26534: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                880 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vint == 456 as libc::c_int {} else {
        __assert_fail(
            b"value.vint == 456\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            881 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26494: {
        if value.c2rust_unnamed.vint == 456 as libc::c_int {} else {
            __assert_fail(
                b"value.vint == 456\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                881 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_object_get_value(
        obj as *mut libc::c_void,
        b"int\0" as *const u8 as *const libc::c_char,
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_value(obj, \"int\", &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            885 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26434: {
        if binn_object_get_value(
            obj as *mut libc::c_void,
            b"int\0" as *const u8 as *const libc::c_char,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_value(obj, \"int\", &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                885 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            887 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26396: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                887 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            888 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26358: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                888 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if use_int_compression != 0 {
        if value.type_0 == 0x40 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                890 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26319: {
            if value.type_0 == 0x40 as libc::c_int {} else {
                __assert_fail(
                    b"value.type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    890 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
        if value.ptr
            == &mut value.c2rust_unnamed.vuint16 as *mut libc::c_ushort
                as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vuint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                891 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26273: {
            if value.ptr
                == &mut value.c2rust_unnamed.vuint16 as *mut libc::c_ushort
                    as *mut libc::c_void
            {} else {
                __assert_fail(
                    b"value.ptr == &value.vuint16\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    891 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
    } else {
        if value.type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                893 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26233: {
            if value.type_0 == 0x61 as libc::c_int {} else {
                __assert_fail(
                    b"value.type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    893 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                894 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
        'c_26187: {
            if value.ptr
                == &mut value.c2rust_unnamed.vint as *mut libc::c_int
                    as *mut libc::c_void
            {} else {
                __assert_fail(
                    b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    894 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"void test2(BOOL)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            896 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26144: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                896 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            897 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26106: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                897 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vint == 789 as libc::c_int {} else {
        __assert_fail(
            b"value.vint == 789\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            898 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26066: {
        if value.c2rust_unnamed.vint == 789 as libc::c_int {} else {
            __assert_fail(
                b"value.vint == 789\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                898 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_list_get_value(list as *mut libc::c_void, 2 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(list, 2, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            905 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_26008: {
        if binn_list_get_value(list as *mut libc::c_void, 2 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(list, 2, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                905 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            907 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25970: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                907 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            908 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25932: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                908 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0x82 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_FLOAT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            909 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25894: {
        if value.type_0 == 0x82 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_FLOAT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                909 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.ptr
        == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            910 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25848: {
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                910 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            911 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25810: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                911 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            912 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25772: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                912 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vdouble == 1.23f64 {} else {
        __assert_fail(
            b"value.vdouble == 1.23\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            913 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25732: {
        if value.c2rust_unnamed.vdouble == 1.23f64 {} else {
            __assert_fail(
                b"value.vdouble == 1.23\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                913 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_map_get_value(map as *mut libc::c_void, 1002 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_value(map, 1002, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            917 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25674: {
        if binn_map_get_value(map as *mut libc::c_void, 1002 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_value(map, 1002, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                917 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            919 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25636: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                919 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            920 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25598: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                920 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0x82 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_FLOAT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            921 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25560: {
        if value.type_0 == 0x82 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_FLOAT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                921 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.ptr
        == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            922 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25514: {
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                922 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            923 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25476: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                923 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            924 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25438: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                924 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vdouble == 4.56f64 {} else {
        __assert_fail(
            b"value.vdouble == 4.56\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            925 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25398: {
        if value.c2rust_unnamed.vdouble == 4.56f64 {} else {
            __assert_fail(
                b"value.vdouble == 4.56\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                925 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_object_get_value(
        obj as *mut libc::c_void,
        b"double\0" as *const u8 as *const libc::c_char,
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_value(obj, \"double\", &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            929 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25338: {
        if binn_object_get_value(
            obj as *mut libc::c_void,
            b"double\0" as *const u8 as *const libc::c_char,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_value(obj, \"double\", &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                929 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            931 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25300: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                931 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            932 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25262: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                932 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0x82 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_FLOAT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            933 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25224: {
        if value.type_0 == 0x82 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_FLOAT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                933 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.ptr
        == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            934 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25178: {
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                934 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            935 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25140: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                935 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            936 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25102: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                936 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vdouble == 7.89f64 {} else {
        __assert_fail(
            b"value.vdouble == 7.89\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            937 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25062: {
        if value.c2rust_unnamed.vdouble == 7.89f64 {} else {
            __assert_fail(
                b"value.vdouble == 7.89\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                937 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_list_get_value(list as *mut libc::c_void, 3 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(list, 3, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            944 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_25004: {
        if binn_list_get_value(list as *mut libc::c_void, 3 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(list, 3, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                944 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            946 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24966: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                946 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            947 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24928: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                947 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0x80061 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_BOOL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            948 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24890: {
        if value.type_0 == 0x80061 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_BOOL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                948 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.ptr
        == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            949 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24844: {
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                949 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            950 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24806: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                950 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            951 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24768: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                951 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vbool == 1 as libc::c_int {} else {
        __assert_fail(
            b"value.vbool == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            952 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24728: {
        if value.c2rust_unnamed.vbool == 1 as libc::c_int {} else {
            __assert_fail(
                b"value.vbool == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                952 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_map_get_value(map as *mut libc::c_void, 1003 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_value(map, 1003, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            956 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24670: {
        if binn_map_get_value(map as *mut libc::c_void, 1003 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_value(map, 1003, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                956 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            958 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24632: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                958 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            959 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24594: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                959 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0x80061 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_BOOL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            960 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24556: {
        if value.type_0 == 0x80061 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_BOOL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                960 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.ptr
        == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            961 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24510: {
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                961 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            962 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24472: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                962 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            963 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24434: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                963 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vbool == 1 as libc::c_int {} else {
        __assert_fail(
            b"value.vbool == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            964 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24394: {
        if value.c2rust_unnamed.vbool == 1 as libc::c_int {} else {
            __assert_fail(
                b"value.vbool == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                964 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_value(
        obj as *mut libc::c_void,
        b"bool\0" as *const u8 as *const libc::c_char,
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_value(obj, \"bool\", &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            966 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24342: {
        if binn_object_get_value(
            obj as *mut libc::c_void,
            b"bool\0" as *const u8 as *const libc::c_char,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_value(obj, \"bool\", &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                966 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            968 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24304: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                968 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            969 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24266: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                969 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0x80061 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_BOOL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            970 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24228: {
        if value.type_0 == 0x80061 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_BOOL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                970 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.ptr
        == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            971 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24182: {
        if value.ptr
            == &mut value.c2rust_unnamed.vint as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"value.ptr == &value.vint\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                971 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            972 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24144: {
        if value.size == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                972 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            973 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24106: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                973 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.c2rust_unnamed.vbool == 1 as libc::c_int {} else {
        __assert_fail(
            b"value.vbool == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            974 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24066: {
        if value.c2rust_unnamed.vbool == 1 as libc::c_int {} else {
            __assert_fail(
                b"value.vbool == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                974 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_list_get_value(list as *mut libc::c_void, 4 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(list, 4, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            981 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_24008: {
        if binn_list_get_value(list as *mut libc::c_void, 4 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(list, 4, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                981 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            983 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23970: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                983 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            984 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23932: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                984 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            985 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23894: {
        if value.type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                985 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            986 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23854: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                986 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size as libc::c_ulong == strlen(str_list) {} else {
        __assert_fail(
            b"value.size == strlen(str_list)\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            987 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23803: {
        if value.size as libc::c_ulong == strlen(str_list) {} else {
            __assert_fail(
                b"value.size == strlen(str_list)\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                987 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(value.ptr as *const libc::c_char, str_list) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(value.ptr, str_list) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            988 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23751: {
        if strcmp(value.ptr as *const libc::c_char, str_list) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(value.ptr, str_list) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                988 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            989 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23713: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                989 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_map_get_value(map as *mut libc::c_void, 1004 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_value(map, 1004, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            993 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23655: {
        if binn_map_get_value(map as *mut libc::c_void, 1004 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_value(map, 1004, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                993 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            995 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23617: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                995 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            996 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23579: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                996 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            997 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23541: {
        if value.type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                997 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size as libc::c_ulong == strlen(str_map) {} else {
        __assert_fail(
            b"value.size == strlen(str_map)\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            998 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23491: {
        if value.size as libc::c_ulong == strlen(str_map) {} else {
            __assert_fail(
                b"value.size == strlen(str_map)\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                998 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(value.ptr as *const libc::c_char, str_map) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(value.ptr, str_map) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            999 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23439: {
        if strcmp(value.ptr as *const libc::c_char, str_map) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(value.ptr, str_map) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                999 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1000 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23401: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1000 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_object_get_value(
        obj as *mut libc::c_void,
        b"text\0" as *const u8 as *const libc::c_char,
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_value(obj, \"text\", &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1004 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23341: {
        if binn_object_get_value(
            obj as *mut libc::c_void,
            b"text\0" as *const u8 as *const libc::c_char,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_value(obj, \"text\", &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1004 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1006 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23303: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1006 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1007 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23265: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1007 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1008 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23227: {
        if value.type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1008 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size as libc::c_ulong == strlen(str_obj) {} else {
        __assert_fail(
            b"value.size == strlen(str_obj)\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1009 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23177: {
        if value.size as libc::c_ulong == strlen(str_obj) {} else {
            __assert_fail(
                b"value.size == strlen(str_obj)\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1009 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(value.ptr as *const libc::c_char, str_obj) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(value.ptr, str_obj) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1010 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23125: {
        if strcmp(value.ptr as *const libc::c_char, str_obj) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(value.ptr, str_obj) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1010 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1011 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23087: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1011 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_list_get_value(list as *mut libc::c_void, 5 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_value(list, 5, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1018 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_23029: {
        if binn_list_get_value(list as *mut libc::c_void, 5 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_value(list, 5, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1018 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1020 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22991: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1020 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1021 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22953: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1021 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0xc0 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1022 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22915: {
        if value.type_0 == 0xc0 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1022 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1023 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22875: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1023 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1024 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22835: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1024 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1025 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22777: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1025 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1026 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22739: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1026 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_map_get_value(map as *mut libc::c_void, 1005 as libc::c_int, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_value(map, 1005, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1030 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22681: {
        if binn_map_get_value(map as *mut libc::c_void, 1005 as libc::c_int, &mut value)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_value(map, 1005, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1030 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1032 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22643: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1032 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1033 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22605: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1033 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0xc0 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1034 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22567: {
        if value.type_0 == 0xc0 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1034 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1035 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22527: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1035 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1036 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22487: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1036 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1037 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22429: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1037 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1038 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22391: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1038 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_object_get_value(
        obj as *mut libc::c_void,
        b"blob\0" as *const u8 as *const libc::c_char,
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_value(obj, \"blob\", &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1042 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22330: {
        if binn_object_get_value(
            obj as *mut libc::c_void,
            b"blob\0" as *const u8 as *const libc::c_char,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_value(obj, \"blob\", &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1042 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1044 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22292: {
        if value.header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"value.header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1044 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.writable == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1045 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22254: {
        if value.writable == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.writable == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1045 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.type_0 == 0xc0 as libc::c_int {} else {
        __assert_fail(
            b"value.type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1046 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22216: {
        if value.type_0 == 0xc0 as libc::c_int {} else {
            __assert_fail(
                b"value.type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1046 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1047 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22176: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1047 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1048 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22136: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1048 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1049 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22078: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1049 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.count == 0 as libc::c_int {} else {
        __assert_fail(
            b"value.count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1050 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_22040: {
        if value.count == 0 as libc::c_int {} else {
            __assert_fail(
                b"value.count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1050 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut value as *mut binn as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    if binn_list_get_int32(list as *mut libc::c_void, 1 as libc::c_int, &mut vint32)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_int32(list, 1, &vint32) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1058 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21982: {
        if binn_list_get_int32(list as *mut libc::c_void, 1 as libc::c_int, &mut vint32)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_int32(list, 1, &vint32) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1058 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vint32 == 123 as libc::c_int {} else {
        __assert_fail(
            b"vint32 == 123\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1059 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21946: {
        if vint32 == 123 as libc::c_int {} else {
            __assert_fail(
                b"vint32 == 123\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1059 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_int32(map as *mut libc::c_void, 1001 as libc::c_int, &mut vint32)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_int32(map, 1001, &vint32) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1061 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21895: {
        if binn_map_get_int32(map as *mut libc::c_void, 1001 as libc::c_int, &mut vint32)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_int32(map, 1001, &vint32) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1061 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vint32 == 456 as libc::c_int {} else {
        __assert_fail(
            b"vint32 == 456\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1062 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21859: {
        if vint32 == 456 as libc::c_int {} else {
            __assert_fail(
                b"vint32 == 456\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1062 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_int32(
        obj as *mut libc::c_void,
        b"int\0" as *const u8 as *const libc::c_char,
        &mut vint32,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_int32(obj, \"int\", &vint32) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1064 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21806: {
        if binn_object_get_int32(
            obj as *mut libc::c_void,
            b"int\0" as *const u8 as *const libc::c_char,
            &mut vint32,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_int32(obj, \"int\", &vint32) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1064 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vint32 == 789 as libc::c_int {} else {
        __assert_fail(
            b"vint32 == 789\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1065 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21769: {
        if vint32 == 789 as libc::c_int {} else {
            __assert_fail(
                b"vint32 == 789\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1065 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_double(list as *mut libc::c_void, 2 as libc::c_int, &mut vdouble)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_double(list, 2, &vdouble) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1069 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21719: {
        if binn_list_get_double(
            list as *mut libc::c_void,
            2 as libc::c_int,
            &mut vdouble,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_double(list, 2, &vdouble) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1069 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vdouble == 1.23f64 {} else {
        __assert_fail(
            b"vdouble == 1.23\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1070 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21683: {
        if vdouble == 1.23f64 {} else {
            __assert_fail(
                b"vdouble == 1.23\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1070 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_double(map as *mut libc::c_void, 1002 as libc::c_int, &mut vdouble)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_double(map, 1002, &vdouble) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1072 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21632: {
        if binn_map_get_double(
            map as *mut libc::c_void,
            1002 as libc::c_int,
            &mut vdouble,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_double(map, 1002, &vdouble) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1072 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vdouble == 4.56f64 {} else {
        __assert_fail(
            b"vdouble == 4.56\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1073 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21596: {
        if vdouble == 4.56f64 {} else {
            __assert_fail(
                b"vdouble == 4.56\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1073 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_double(
        obj as *mut libc::c_void,
        b"double\0" as *const u8 as *const libc::c_char,
        &mut vdouble,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_double(obj, \"double\", &vdouble) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1075 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21543: {
        if binn_object_get_double(
            obj as *mut libc::c_void,
            b"double\0" as *const u8 as *const libc::c_char,
            &mut vdouble,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_double(obj, \"double\", &vdouble) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1075 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vdouble == 7.89f64 {} else {
        __assert_fail(
            b"vdouble == 7.89\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1076 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21506: {
        if vdouble == 7.89f64 {} else {
            __assert_fail(
                b"vdouble == 7.89\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1076 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_bool(list as *mut libc::c_void, 3 as libc::c_int, &mut vbool)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_bool(list, 3, &vbool) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1080 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21456: {
        if binn_list_get_bool(list as *mut libc::c_void, 3 as libc::c_int, &mut vbool)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_bool(list, 3, &vbool) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1080 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vbool == 1 as libc::c_int {} else {
        __assert_fail(
            b"vbool == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1081 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21420: {
        if vbool == 1 as libc::c_int {} else {
            __assert_fail(
                b"vbool == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1081 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_bool(map as *mut libc::c_void, 1003 as libc::c_int, &mut vbool)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_bool(map, 1003, &vbool) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1083 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21369: {
        if binn_map_get_bool(map as *mut libc::c_void, 1003 as libc::c_int, &mut vbool)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_bool(map, 1003, &vbool) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1083 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vbool == 1 as libc::c_int {} else {
        __assert_fail(
            b"vbool == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1084 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21333: {
        if vbool == 1 as libc::c_int {} else {
            __assert_fail(
                b"vbool == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1084 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_bool(
        obj as *mut libc::c_void,
        b"bool\0" as *const u8 as *const libc::c_char,
        &mut vbool,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_bool(obj, \"bool\", &vbool) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1086 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21280: {
        if binn_object_get_bool(
            obj as *mut libc::c_void,
            b"bool\0" as *const u8 as *const libc::c_char,
            &mut vbool,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_bool(obj, \"bool\", &vbool) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1086 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if vbool == 1 as libc::c_int {} else {
        __assert_fail(
            b"vbool == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1087 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21243: {
        if vbool == 1 as libc::c_int {} else {
            __assert_fail(
                b"vbool == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1087 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_get_str(list as *mut libc::c_void, 4 as libc::c_int, &mut pstr)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_str(list, 4, &pstr) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1091 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21193: {
        if binn_list_get_str(list as *mut libc::c_void, 4 as libc::c_int, &mut pstr)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_str(list, 4, &pstr) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1091 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1092 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21155: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1092 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, str_list) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(pstr, str_list) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1093 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21105: {
        if strcmp(pstr, str_list) == 0 as libc::c_int {} else {
            __assert_fail(
                b"strcmp(pstr, str_list) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1093 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_str(map as *mut libc::c_void, 1004 as libc::c_int, &mut pstr)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_str(map, 1004, &pstr) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1095 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21054: {
        if binn_map_get_str(map as *mut libc::c_void, 1004 as libc::c_int, &mut pstr)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_str(map, 1004, &pstr) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1095 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1096 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_21016: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1096 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, str_map) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(pstr, str_map) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1097 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20966: {
        if strcmp(pstr, str_map) == 0 as libc::c_int {} else {
            __assert_fail(
                b"strcmp(pstr, str_map) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1097 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_str(
        obj as *mut libc::c_void,
        b"text\0" as *const u8 as *const libc::c_char,
        &mut pstr,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_str(obj, \"text\", &pstr) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1099 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20913: {
        if binn_object_get_str(
            obj as *mut libc::c_void,
            b"text\0" as *const u8 as *const libc::c_char,
            &mut pstr,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_str(obj, \"text\", &pstr) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1099 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1100 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20875: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1100 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, str_obj) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(pstr, str_obj) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1101 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20825: {
        if strcmp(pstr, str_obj) == 0 as libc::c_int {} else {
            __assert_fail(
                b"strcmp(pstr, str_obj) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1101 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    value.ptr = 0 as *mut libc::c_void;
    value.size = 0 as libc::c_int;
    if binn_list_get_blob(
        list as *mut libc::c_void,
        5 as libc::c_int,
        &mut value.ptr,
        &mut value.size,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_get_blob(list, 5, &value.ptr, &value.size) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1107 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20758: {
        if binn_list_get_blob(
            list as *mut libc::c_void,
            5 as libc::c_int,
            &mut value.ptr,
            &mut value.size,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_get_blob(list, 5, &value.ptr, &value.size) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1107 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1108 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20718: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1108 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1109 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20678: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1109 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1110 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20620: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1110 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    value.ptr = 0 as *mut libc::c_void;
    value.size = 0 as libc::c_int;
    if binn_map_get_blob(
        map as *mut libc::c_void,
        1005 as libc::c_int,
        &mut value.ptr,
        &mut value.size,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_blob(map, 1005, &value.ptr, &value.size) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1114 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20552: {
        if binn_map_get_blob(
            map as *mut libc::c_void,
            1005 as libc::c_int,
            &mut value.ptr,
            &mut value.size,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_blob(map, 1005, &value.ptr, &value.size) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1114 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20512: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1115 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1116 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20472: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1116 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1117 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20414: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1117 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    value.ptr = 0 as *mut libc::c_void;
    value.size = 0 as libc::c_int;
    if binn_object_get_blob(
        obj as *mut libc::c_void,
        b"blob\0" as *const u8 as *const libc::c_char,
        &mut value.ptr,
        &mut value.size,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_blob(obj, \"blob\", &value.ptr, &value.size) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1121 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20343: {
        if binn_object_get_blob(
            obj as *mut libc::c_void,
            b"blob\0" as *const u8 as *const libc::c_char,
            &mut value.ptr,
            &mut value.size,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_blob(obj, \"blob\", &value.ptr, &value.size) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1121 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20303: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1122 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1123 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20263: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1123 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1124 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20205: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1124 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_int32(list as *mut libc::c_void, 1 as libc::c_int) == 123 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_int32(list, 1) == 123\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1130 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20159: {
        if binn_list_int32(list as *mut libc::c_void, 1 as libc::c_int)
            == 123 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_int32(list, 1) == 123\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1130 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_int32(map as *mut libc::c_void, 1001 as libc::c_int)
        == 456 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_int32(map, 1001) == 456\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1131 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20112: {
        if binn_map_int32(map as *mut libc::c_void, 1001 as libc::c_int)
            == 456 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_int32(map, 1001) == 456\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1131 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_int32(
        obj as *mut libc::c_void,
        b"int\0" as *const u8 as *const libc::c_char,
    ) == 789 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_int32(obj, \"int\") == 789\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1132 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20063: {
        if binn_object_int32(
            obj as *mut libc::c_void,
            b"int\0" as *const u8 as *const libc::c_char,
        ) == 789 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_int32(obj, \"int\") == 789\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1132 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_double(list as *mut libc::c_void, 2 as libc::c_int) == 1.23f64 {} else {
        __assert_fail(
            b"binn_list_double(list, 2) == 1.23\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1136 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_20017: {
        if binn_list_double(list as *mut libc::c_void, 2 as libc::c_int) == 1.23f64
        {} else {
            __assert_fail(
                b"binn_list_double(list, 2) == 1.23\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1136 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_double(map as *mut libc::c_void, 1002 as libc::c_int) == 4.56f64
    {} else {
        __assert_fail(
            b"binn_map_double(map, 1002) == 4.56\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1137 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19970: {
        if binn_map_double(map as *mut libc::c_void, 1002 as libc::c_int) == 4.56f64
        {} else {
            __assert_fail(
                b"binn_map_double(map, 1002) == 4.56\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1137 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_double(
        obj as *mut libc::c_void,
        b"double\0" as *const u8 as *const libc::c_char,
    ) == 7.89f64
    {} else {
        __assert_fail(
            b"binn_object_double(obj, \"double\") == 7.89\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19920: {
        if binn_object_double(
            obj as *mut libc::c_void,
            b"double\0" as *const u8 as *const libc::c_char,
        ) == 7.89f64
        {} else {
            __assert_fail(
                b"binn_object_double(obj, \"double\") == 7.89\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1138 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_bool(list as *mut libc::c_void, 3 as libc::c_int) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_bool(list, 3) == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1142 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19874: {
        if binn_list_bool(list as *mut libc::c_void, 3 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_bool(list, 3) == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1142 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_bool(map as *mut libc::c_void, 1003 as libc::c_int) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_bool(map, 1003) == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1143 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19827: {
        if binn_map_bool(map as *mut libc::c_void, 1003 as libc::c_int)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_bool(map, 1003) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1143 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_bool(
        obj as *mut libc::c_void,
        b"bool\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_bool(obj, \"bool\") == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1144 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19778: {
        if binn_object_bool(
            obj as *mut libc::c_void,
            b"bool\0" as *const u8 as *const libc::c_char,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_bool(obj, \"bool\") == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1144 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    pstr = binn_list_str(list as *mut libc::c_void, 4 as libc::c_int);
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1149 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19731: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1149 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, str_list) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(pstr, str_list) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1150 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19677: {
        if strcmp(pstr, str_list) == 0 as libc::c_int {} else {
            __assert_fail(
                b"strcmp(pstr, str_list) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1150 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    pstr = binn_map_str(map as *mut libc::c_void, 1004 as libc::c_int);
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1153 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19629: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1153 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, str_map) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(pstr, str_map) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19575: {
        if strcmp(pstr, str_map) == 0 as libc::c_int {} else {
            __assert_fail(
                b"strcmp(pstr, str_map) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1154 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    pstr = binn_object_str(
        obj as *mut libc::c_void,
        b"text\0" as *const u8 as *const libc::c_char,
    );
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1157 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19525: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1157 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, str_obj) == 0 as libc::c_int {} else {
        __assert_fail(
            b"strcmp(pstr, str_obj) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1158 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19470: {
        if strcmp(pstr, str_obj) == 0 as libc::c_int {} else {
            __assert_fail(
                b"strcmp(pstr, str_obj) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1158 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    value
        .ptr = binn_list_blob(
        list as *mut libc::c_void,
        5 as libc::c_int,
        &mut value.size,
    );
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1163 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19417: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1163 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1164 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19377: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1164 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1165 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19319: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1165 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    value
        .ptr = binn_map_blob(
        map as *mut libc::c_void,
        1005 as libc::c_int,
        &mut value.size,
    );
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1168 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19265: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1168 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1169 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19225: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1169 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1170 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19167: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1170 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    value
        .ptr = binn_object_blob(
        obj as *mut libc::c_void,
        b"blob\0" as *const u8 as *const libc::c_char,
        &mut value.size,
    );
    if !(value.ptr).is_null() {} else {
        __assert_fail(
            b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19112: {
        if !(value.ptr).is_null() {} else {
            __assert_fail(
                b"value.ptr != 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1173 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if value.size == blobsize {} else {
        __assert_fail(
            b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19072: {
        if value.size == blobsize {} else {
            __assert_fail(
                b"value.size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1174 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1175 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test2(BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_19010: {
        if memcmp(value.ptr, pblob as *const libc::c_void, blobsize as libc::c_ulong)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(value.ptr, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1175 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"void test2(BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    binn_free(list);
    binn_free(map);
    binn_free(obj);
    printf(b"OK\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn test3() {
    static mut fix_size: libc::c_int = 512 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut header_size: libc::c_int = 0;
    let mut blobsize: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 256] = [0; 256];
    let mut list: *mut binn = 0 as *mut binn;
    let mut map: *mut binn = 0 as *mut binn;
    let mut obj: *mut binn = 0 as *mut binn;
    let mut obj1: *mut binn = 0 as *mut binn;
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *mut libc::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *mut libc::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut vbyte: libc::c_char = 0;
    let mut pblob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vint16: libc::c_short = 0;
    let mut pint16: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut vuint16: libc::c_ushort = 0;
    let mut puint16: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut vint32: libc::c_int = 0;
    let mut pint32: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vuint32: libc::c_uint = 0;
    let mut puint32: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut vint64: libc::c_longlong = 0;
    let mut pint64: *mut libc::c_longlong = 0 as *mut libc::c_longlong;
    let mut vuint64: libc::c_ulonglong = 0;
    let mut puint64: *mut libc::c_ulonglong = 0 as *mut libc::c_ulonglong;
    printf(b"testing binn 3... \0" as *const u8 as *const libc::c_char);
    list = binn_list();
    if !list.is_null() {} else {
        __assert_fail(
            b"list != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1206 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46804: {
        if !list.is_null() {} else {
            __assert_fail(
                b"list != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1206 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    map = binn_map();
    if !map.is_null() {} else {
        __assert_fail(
            b"map != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1209 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46761: {
        if !map.is_null() {} else {
            __assert_fail(
                b"map != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1209 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    obj = binn_object();
    if !obj.is_null() {} else {
        __assert_fail(
            b"obj != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46718: {
        if !obj.is_null() {} else {
            __assert_fail(
                b"obj != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1212 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"list->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1214 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46678: {
        if (*list).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"list->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1214 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).type_0 == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"list->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1215 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46638: {
        if (*list).type_0 == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"list->type == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1215 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"list->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1216 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46598: {
        if (*list).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"list->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1216 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*list).pbuf).is_null() {} else {
        __assert_fail(
            b"list->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1217 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46554: {
        if !((*list).pbuf).is_null() {} else {
            __assert_fail(
                b"list->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1217 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"list->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1218 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46514: {
        if (*list).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"list->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1218 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"list->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1219 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46474: {
        if (*list).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"list->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1219 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*list).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"list->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46434: {
        if (*list).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"list->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1220 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"map->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46394: {
        if (*map).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"map->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1222 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).type_0 == 0xe1 as libc::c_int {} else {
        __assert_fail(
            b"map->type == BINN_MAP\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1223 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46354: {
        if (*map).type_0 == 0xe1 as libc::c_int {} else {
            __assert_fail(
                b"map->type == BINN_MAP\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1223 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"map->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1224 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46314: {
        if (*map).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"map->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1224 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*map).pbuf).is_null() {} else {
        __assert_fail(
            b"map->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1225 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46270: {
        if !((*map).pbuf).is_null() {} else {
            __assert_fail(
                b"map->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1225 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"map->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1226 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46230: {
        if (*map).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"map->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1226 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"map->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1227 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46190: {
        if (*map).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"map->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1227 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*map).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"map->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1228 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46150: {
        if (*map).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"map->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1228 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"obj->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1230 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46110: {
        if (*obj).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"obj->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1230 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).type_0 == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"obj->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1231 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46070: {
        if (*obj).type_0 == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"obj->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1231 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1232 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_46030: {
        if (*obj).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1232 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*obj).pbuf).is_null() {} else {
        __assert_fail(
            b"obj->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1233 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45986: {
        if !((*obj).pbuf).is_null() {} else {
            __assert_fail(
                b"obj->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1233 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).alloc_size > 9 as libc::c_int {} else {
        __assert_fail(
            b"obj->alloc_size > MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1234 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45946: {
        if (*obj).alloc_size > 9 as libc::c_int {} else {
            __assert_fail(
                b"obj->alloc_size > MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1234 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"obj->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1235 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45906: {
        if (*obj).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"obj->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1235 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj).pre_allocated == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1236 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45866: {
        if (*obj).pre_allocated == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj->pre_allocated == FALSE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1236 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = malloc(fix_size as libc::c_ulong) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1242 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45815: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1242 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    obj1 = binn_new(0xe2 as libc::c_int, fix_size, ptr as *mut libc::c_void);
    if !obj1.is_null() {} else {
        __assert_fail(
            b"obj1 != INVALID_BINN\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1245 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45766: {
        if !obj1.is_null() {} else {
            __assert_fail(
                b"obj1 != INVALID_BINN\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1245 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).header == 0x1f22b11f as libc::c_int {} else {
        __assert_fail(
            b"obj1->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1247 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45726: {
        if (*obj1).header == 0x1f22b11f as libc::c_int {} else {
            __assert_fail(
                b"obj1->header == BINN_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1247 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).type_0 == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"obj1->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1248 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45686: {
        if (*obj1).type_0 == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"obj1->type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1248 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).count == 0 as libc::c_int {} else {
        __assert_fail(
            b"obj1->count == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1249 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45646: {
        if (*obj1).count == 0 as libc::c_int {} else {
            __assert_fail(
                b"obj1->count == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1249 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*obj1).pbuf).is_null() {} else {
        __assert_fail(
            b"obj1->pbuf != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1250 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45602: {
        if !((*obj1).pbuf).is_null() {} else {
            __assert_fail(
                b"obj1->pbuf != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1250 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).alloc_size == fix_size {} else {
        __assert_fail(
            b"obj1->alloc_size == fix_size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1251 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45560: {
        if (*obj1).alloc_size == fix_size {} else {
            __assert_fail(
                b"obj1->alloc_size == fix_size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1251 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).used_size == 9 as libc::c_int {} else {
        __assert_fail(
            b"obj1->used_size == MAX_BINN_HEADER\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1252 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45520: {
        if (*obj1).used_size == 9 as libc::c_int {} else {
            __assert_fail(
                b"obj1->used_size == MAX_BINN_HEADER\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1252 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (*obj1).pre_allocated == 1 as libc::c_int {} else {
        __assert_fail(
            b"obj1->pre_allocated == TRUE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1253 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45480: {
        if (*obj1).pre_allocated == 1 as libc::c_int {} else {
            __assert_fail(
                b"obj1->pre_allocated == TRUE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1253 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(list as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45429: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1263 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_list_read(
        ptr as *mut libc::c_void,
        0 as libc::c_int,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_list_read(ptr, 0, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1264 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45371: {
        if (binn_list_read(
            ptr as *mut libc::c_void,
            0 as libc::c_int,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_list_read(ptr, 0, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1264 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_list_read(
        ptr as *mut libc::c_void,
        1 as libc::c_int,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_list_read(ptr, 1, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1265 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45313: {
        if (binn_list_read(
            ptr as *mut libc::c_void,
            1 as libc::c_int,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_list_read(ptr, 1, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1265 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_list_read(
        ptr as *mut libc::c_void,
        2 as libc::c_int,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_list_read(ptr, 2, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1266 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45255: {
        if (binn_list_read(
            ptr as *mut libc::c_void,
            2 as libc::c_int,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_list_read(ptr, 2, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1266 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_list_read(
        ptr as *mut libc::c_void,
        -(1 as libc::c_int),
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_list_read(ptr, -1, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1267 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45195: {
        if (binn_list_read(
            ptr as *mut libc::c_void,
            -(1 as libc::c_int),
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_list_read(ptr, -1, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1267 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(map as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1270 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45144: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1270 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_map_read(
        ptr as *mut libc::c_void,
        0 as libc::c_int,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_map_read(ptr, 0, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1271 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45086: {
        if (binn_map_read(
            ptr as *mut libc::c_void,
            0 as libc::c_int,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_map_read(ptr, 0, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1271 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_map_read(
        ptr as *mut libc::c_void,
        55001 as libc::c_int,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_map_read(ptr, 55001, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1272 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_45028: {
        if (binn_map_read(
            ptr as *mut libc::c_void,
            55001 as libc::c_int,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_map_read(ptr, 55001, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1272 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_map_read(
        ptr as *mut libc::c_void,
        -(1 as libc::c_int),
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_map_read(ptr, -1, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1273 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44968: {
        if (binn_map_read(
            ptr as *mut libc::c_void,
            -(1 as libc::c_int),
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_map_read(ptr, -1, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1273 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(obj as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1276 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44917: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1276 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_object_read(
        ptr as *mut libc::c_void,
        0 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_object_read(ptr, NULL, &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1277 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44853: {
        if (binn_object_read(
            ptr as *mut libc::c_void,
            0 as *const libc::c_char,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_object_read(ptr, NULL, &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1277 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_object_read(
        ptr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_object_read(ptr, \"\", &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1278 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44793: {
        if (binn_object_read(
            ptr as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_object_read(ptr, \"\", &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1278 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_object_read(
        ptr as *mut libc::c_void,
        b"test\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binn_object_read(ptr, \"test\", &type, &size) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1279 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44733: {
        if (binn_object_read(
            ptr as *mut libc::c_void,
            b"test\0" as *const u8 as *const libc::c_char,
            &mut type_0,
            &mut size,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binn_object_read(ptr, \"test\", &type, &size) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1279 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1284 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44681: {
        if binn_list_add(
            list,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1284 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1285 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44627: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1285 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        5501 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1286 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44573: {
        if binn_map_set(
            map,
            5501 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 5501, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1286 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1287 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44517: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1287 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"test\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44461: {
        if binn_object_set(
            obj,
            b"test\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"test\", BINN_INT32, &i, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1288 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    vbyte = 255 as libc::c_int as libc::c_char;
    vint16 = -(32000 as libc::c_int) as libc::c_short;
    vuint16 = 65000 as libc::c_int as libc::c_ushort;
    vint32 = -(65000000 as libc::c_int);
    vuint32 = 65000000 as libc::c_int as libc::c_uint;
    vint64 = -(6500000000000000 as libc::c_long) as libc::c_longlong;
    vuint64 = 6500000000000000 as libc::c_long as libc::c_ulonglong;
    blobsize = 150 as libc::c_int;
    pblob = malloc(blobsize as libc::c_ulong) as *mut libc::c_char;
    if !pblob.is_null() {} else {
        __assert_fail(
            b"pblob != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1299 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44377: {
        if !pblob.is_null() {} else {
            __assert_fail(
                b"pblob != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1299 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    memset(pblob as *mut libc::c_void, 55 as libc::c_int, blobsize as libc::c_ulong);
    if binn_list_add(list, 0 as libc::c_int, 0 as *mut libc::c_void, 0 as libc::c_int)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1302 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44317: {
        if binn_list_add(
            list,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1302 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1303 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44265: {
        if binn_list_add(
            list,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1303 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x41 as libc::c_int,
        &mut vint16 as *mut libc::c_short as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT16, &vint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44213: {
        if binn_list_add(
            list,
            0x41 as libc::c_int,
            &mut vint16 as *mut libc::c_short as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT16, &vint16, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1304 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x40 as libc::c_int,
        &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT16, &vuint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1305 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44161: {
        if binn_list_add(
            list,
            0x40 as libc::c_int,
            &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT16, &vuint16, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1305 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x61 as libc::c_int,
        &mut vint32 as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT32, &vint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1306 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44109: {
        if binn_list_add(
            list,
            0x61 as libc::c_int,
            &mut vint32 as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT32, &vint32, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1306 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT32, &vuint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1307 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44057: {
        if binn_list_add(
            list,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT32, &vuint32, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1307 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x81 as libc::c_int,
        &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_INT64, &vint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1308 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_44005: {
        if binn_list_add(
            list,
            0x81 as libc::c_int,
            &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_INT64, &vint64, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1308 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0x80 as libc::c_int,
        &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_UINT64, &vuint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1309 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43953: {
        if binn_list_add(
            list,
            0x80 as libc::c_int,
            &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_UINT64, &vuint64, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1309 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xa0 as libc::c_int,
        b"this is the string\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1310 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43901: {
        if binn_list_add(
            list,
            0xa0 as libc::c_int,
            b"this is the string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1310 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(list, 0xc0 as libc::c_int, pblob as *mut libc::c_void, blobsize)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, pblob, blobsize) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1311 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43847: {
        if binn_list_add(list, 0xc0 as libc::c_int, pblob as *mut libc::c_void, blobsize)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, pblob, blobsize) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1311 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99000 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99000, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1313 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43795: {
        if binn_map_set(
            map,
            99000 as libc::c_int,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99000, BINN_NULL, 0, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1313 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99001 as libc::c_int,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99001, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1314 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43741: {
        if binn_map_set(
            map,
            99001 as libc::c_int,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99001, BINN_UINT8, &vbyte, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1314 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99002 as libc::c_int,
        0x41 as libc::c_int,
        &mut vint16 as *mut libc::c_short as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99002, BINN_INT16, &vint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1315 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43687: {
        if binn_map_set(
            map,
            99002 as libc::c_int,
            0x41 as libc::c_int,
            &mut vint16 as *mut libc::c_short as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99002, BINN_INT16, &vint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1315 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99003 as libc::c_int,
        0x40 as libc::c_int,
        &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99003, BINN_UINT16, &vuint16, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1316 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43633: {
        if binn_map_set(
            map,
            99003 as libc::c_int,
            0x40 as libc::c_int,
            &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99003, BINN_UINT16, &vuint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1316 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99004 as libc::c_int,
        0x61 as libc::c_int,
        &mut vint32 as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99004, BINN_INT32, &vint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1317 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43579: {
        if binn_map_set(
            map,
            99004 as libc::c_int,
            0x61 as libc::c_int,
            &mut vint32 as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99004, BINN_INT32, &vint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1317 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99005 as libc::c_int,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99005, BINN_UINT32, &vuint32, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1318 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43525: {
        if binn_map_set(
            map,
            99005 as libc::c_int,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99005, BINN_UINT32, &vuint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1318 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99006 as libc::c_int,
        0x81 as libc::c_int,
        &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99006, BINN_INT64, &vint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1319 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43471: {
        if binn_map_set(
            map,
            99006 as libc::c_int,
            0x81 as libc::c_int,
            &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99006, BINN_INT64, &vint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1319 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99007 as libc::c_int,
        0x80 as libc::c_int,
        &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99007, BINN_UINT64, &vuint64, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43417: {
        if binn_map_set(
            map,
            99007 as libc::c_int,
            0x80 as libc::c_int,
            &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99007, BINN_UINT64, &vuint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1320 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99008 as libc::c_int,
        0xa0 as libc::c_int,
        b"this is the string\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99008, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1321 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43363: {
        if binn_map_set(
            map,
            99008 as libc::c_int,
            0xa0 as libc::c_int,
            b"this is the string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99008, BINN_STRING, \"this is the string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1321 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        map,
        99009 as libc::c_int,
        0xc0 as libc::c_int,
        pblob as *mut libc::c_void,
        blobsize,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(map, 99009, BINN_BLOB, pblob, blobsize) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1322 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43307: {
        if binn_map_set(
            map,
            99009 as libc::c_int,
            0xc0 as libc::c_int,
            pblob as *mut libc::c_void,
            blobsize,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(map, 99009, BINN_BLOB, pblob, blobsize) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1322 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key0\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key0\", BINN_NULL, 0, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1324 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43253: {
        if binn_object_set(
            obj,
            b"key0\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            0 as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key0\", BINN_NULL, 0, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1324 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key1\0" as *const u8 as *const libc::c_char,
        0x20 as libc::c_int,
        &mut vbyte as *mut libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key1\", BINN_UINT8, &vbyte, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1325 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43197: {
        if binn_object_set(
            obj,
            b"key1\0" as *const u8 as *const libc::c_char,
            0x20 as libc::c_int,
            &mut vbyte as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key1\", BINN_UINT8, &vbyte, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1325 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key2\0" as *const u8 as *const libc::c_char,
        0x41 as libc::c_int,
        &mut vint16 as *mut libc::c_short as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key2\", BINN_INT16, &vint16, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1326 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43141: {
        if binn_object_set(
            obj,
            b"key2\0" as *const u8 as *const libc::c_char,
            0x41 as libc::c_int,
            &mut vint16 as *mut libc::c_short as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key2\", BINN_INT16, &vint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1326 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key3\0" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
        &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key3\", BINN_UINT16, &vuint16, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1327 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43085: {
        if binn_object_set(
            obj,
            b"key3\0" as *const u8 as *const libc::c_char,
            0x40 as libc::c_int,
            &mut vuint16 as *mut libc::c_ushort as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key3\", BINN_UINT16, &vuint16, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1327 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key4\0" as *const u8 as *const libc::c_char,
        0x61 as libc::c_int,
        &mut vint32 as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key4\", BINN_INT32, &vint32, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1328 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_43029: {
        if binn_object_set(
            obj,
            b"key4\0" as *const u8 as *const libc::c_char,
            0x61 as libc::c_int,
            &mut vint32 as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key4\", BINN_INT32, &vint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1328 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key5\0" as *const u8 as *const libc::c_char,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key5\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42973: {
        if binn_object_set(
            obj,
            b"key5\0" as *const u8 as *const libc::c_char,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key5\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1329 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key6\0" as *const u8 as *const libc::c_char,
        0x81 as libc::c_int,
        &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key6\", BINN_INT64, &vint64, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42917: {
        if binn_object_set(
            obj,
            b"key6\0" as *const u8 as *const libc::c_char,
            0x81 as libc::c_int,
            &mut vint64 as *mut libc::c_longlong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key6\", BINN_INT64, &vint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1330 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key7\0" as *const u8 as *const libc::c_char,
        0x80 as libc::c_int,
        &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key7\", BINN_UINT64, &vuint64, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1331 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42861: {
        if binn_object_set(
            obj,
            b"key7\0" as *const u8 as *const libc::c_char,
            0x80 as libc::c_int,
            &mut vuint64 as *mut libc::c_ulonglong as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key7\", BINN_UINT64, &vuint64, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1331 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key8\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"this is the string\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key8\", BINN_STRING, \"this is the string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1332 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42805: {
        if binn_object_set(
            obj,
            b"key8\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"this is the string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key8\", BINN_STRING, \"this is the string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1332 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"key9\0" as *const u8 as *const libc::c_char,
        0xc0 as libc::c_int,
        pblob as *mut libc::c_void,
        blobsize,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"key9\", BINN_BLOB, pblob, blobsize) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1333 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42747: {
        if binn_object_set(
            obj,
            b"key9\0" as *const u8 as *const libc::c_char,
            0xc0 as libc::c_int,
            pblob as *mut libc::c_void,
            blobsize,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"key9\", BINN_BLOB, pblob, blobsize) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1333 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xc0 as libc::c_int,
        ptr as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_BLOB, ptr, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42695: {
        if binn_list_add(
            list,
            0xc0 as libc::c_int,
            ptr as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_BLOB, ptr, 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1336 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xa0 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_STRING, \"\", 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1337 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42643: {
        if binn_list_add(
            list,
            0xa0 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_STRING, \"\", 0) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1337 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        list,
        0xa0 as libc::c_int,
        b"after the empty items\0" as *const u8 as *const libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(list, BINN_STRING, \"after the empty items\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1338 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42591: {
        if binn_list_add(
            list,
            0xa0 as libc::c_int,
            b"after the empty items\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(list, BINN_STRING, \"after the empty items\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1338 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_list_add(
        obj1,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_list_add(obj1, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1343 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42539: {
        if binn_list_add(
            obj1,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_list_add(obj1, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1343 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_set(
        obj1,
        55001 as libc::c_int,
        0x61 as libc::c_int,
        &mut i as *mut libc::c_int as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_set(obj1, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42485: {
        if binn_map_set(
            obj1,
            55001 as libc::c_int,
            0x61 as libc::c_int,
            &mut i as *mut libc::c_int as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_set(obj1, 55001, BINN_INT32, &i, 0) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1344 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"test\0" as *const u8 as *const libc::c_char,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1346 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42429: {
        if binn_object_set(
            obj1,
            b"test\0" as *const u8 as *const libc::c_char,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1346 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"test\0" as *const u8 as *const libc::c_char,
        0x60 as libc::c_int,
        &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == FALSE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1347 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42373: {
        if binn_object_set(
            obj1,
            b"test\0" as *const u8 as *const libc::c_char,
            0x60 as libc::c_int,
            &mut vuint32 as *mut libc::c_uint as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"test\", BINN_UINT32, &vuint32, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1347 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"key1\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"this is the value\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"key1\", BINN_STRING, \"this is the value\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1349 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42317: {
        if binn_object_set(
            obj1,
            b"key1\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"this is the value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"key1\", BINN_STRING, \"this is the value\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1349 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"key2\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"the second value\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"key2\", BINN_STRING, \"the second value\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1350 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42261: {
        if binn_object_set(
            obj1,
            b"key2\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"the second value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"key2\", BINN_STRING, \"the second value\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1350 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = malloc(fix_size as libc::c_ulong) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1355 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42210: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1355 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    p2 = ptr;
    i = 0 as libc::c_int;
    while i < fix_size - 1 as libc::c_int {
        *p2 = 'A' as i32 as libc::c_char;
        p2 = p2.offset(1);
        p2;
        i += 1;
        i;
    }
    *p2 = '\0' as i32 as libc::c_char;
    if strlen(ptr) == (fix_size - 1 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"strlen(ptr) == fix_size - 1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42123: {
        if strlen(ptr) == (fix_size - 1 as libc::c_int) as libc::c_ulong {} else {
            __assert_fail(
                b"strlen(ptr) == fix_size - 1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1361 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj1,
        b"v2\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        ptr as *mut libc::c_void,
        0 as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj1, \"v2\", BINN_STRING, ptr, 0) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1363 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42067: {
        if binn_object_set(
            obj1,
            b"v2\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            ptr as *mut libc::c_void,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj1, \"v2\", BINN_STRING, ptr, 0) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1363 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"v2\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        ptr as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"v2\", BINN_STRING, ptr, 0) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1365 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_42011: {
        if binn_object_set(
            obj,
            b"v2\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            ptr as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"v2\", BINN_STRING, ptr, 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1365 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"Key00\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"after the big string\0" as *const u8 as *const libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"Key00\", BINN_STRING, \"after the big string\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1366 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41955: {
        if binn_object_set(
            obj,
            b"Key00\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"after the big string\0" as *const u8 as *const libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"Key00\", BINN_STRING, \"after the big string\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1366 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    free(ptr as *mut libc::c_void);
    ptr = 0 as *mut libc::c_char;
    if binn_object_set(
        obj,
        b"list\0" as *const u8 as *const libc::c_char,
        0xe0 as libc::c_int,
        binn_ptr(list as *mut libc::c_void),
        binn_size(list as *mut libc::c_void),
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"list\", BINN_LIST, binn_ptr(list), binn_size(list)) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1370 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41873: {
        if binn_object_set(
            obj,
            b"list\0" as *const u8 as *const libc::c_char,
            0xe0 as libc::c_int,
            binn_ptr(list as *mut libc::c_void),
            binn_size(list as *mut libc::c_void),
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"list\", BINN_LIST, binn_ptr(list), binn_size(list)) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1370 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_set(
        obj,
        b"Key10\0" as *const u8 as *const libc::c_char,
        0xa0 as libc::c_int,
        b"after the list\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        0 as libc::c_int,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_set(obj, \"Key10\", BINN_STRING, \"after the list\", 0) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1371 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41817: {
        if binn_object_set(
            obj,
            b"Key10\0" as *const u8 as *const libc::c_char,
            0xa0 as libc::c_int,
            b"after the list\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_set(obj, \"Key10\", BINN_STRING, \"after the list\", 0) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1371 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(map as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1384 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41766: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1384 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_pair(
        ptr as *mut libc::c_void,
        -(1 as libc::c_int),
        &mut id,
        &mut value,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_pair(ptr, -1, &id, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1386 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41710: {
        if binn_map_get_pair(
            ptr as *mut libc::c_void,
            -(1 as libc::c_int),
            &mut id,
            &mut value,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_pair(ptr, -1, &id, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1386 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_pair(ptr as *mut libc::c_void, 0 as libc::c_int, &mut id, &mut value)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_pair(ptr, 0, &id, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1387 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41656: {
        if binn_map_get_pair(
            ptr as *mut libc::c_void,
            0 as libc::c_int,
            &mut id,
            &mut value,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_pair(ptr, 0, &id, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1387 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_pair(ptr as *mut libc::c_void, 1 as libc::c_int, &mut id, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_pair(ptr, 1, &id, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1389 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41602: {
        if binn_map_get_pair(
            ptr as *mut libc::c_void,
            1 as libc::c_int,
            &mut id,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_pair(ptr, 1, &id, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1389 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if id == 5501 as libc::c_int {} else {
        __assert_fail(
            b"id == 5501\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1390 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41566: {
        if id == 5501 as libc::c_int {} else {
            __assert_fail(
                b"id == 5501\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1390 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_pair(ptr as *mut libc::c_void, 2 as libc::c_int, &mut id, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_pair(ptr, 2, &id, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1391 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41512: {
        if binn_map_get_pair(
            ptr as *mut libc::c_void,
            2 as libc::c_int,
            &mut id,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_pair(ptr, 2, &id, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1391 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if id == 99000 as libc::c_int {} else {
        __assert_fail(
            b"id == 99000\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1392 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41476: {
        if id == 99000 as libc::c_int {} else {
            __assert_fail(
                b"id == 99000\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1392 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_pair(ptr as *mut libc::c_void, 3 as libc::c_int, &mut id, &mut value)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_pair(ptr, 3, &id, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1393 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41422: {
        if binn_map_get_pair(
            ptr as *mut libc::c_void,
            3 as libc::c_int,
            &mut id,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_pair(ptr, 3, &id, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1393 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if id == 99001 as libc::c_int {} else {
        __assert_fail(
            b"id == 99001\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1394 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41386: {
        if id == 99001 as libc::c_int {} else {
            __assert_fail(
                b"id == 99001\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1394 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_pair(
        ptr as *mut libc::c_void,
        10 as libc::c_int,
        &mut id,
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_pair(ptr, 10, &id, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1395 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41332: {
        if binn_map_get_pair(
            ptr as *mut libc::c_void,
            10 as libc::c_int,
            &mut id,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_pair(ptr, 10, &id, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1395 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if id == 99008 as libc::c_int {} else {
        __assert_fail(
            b"id == 99008\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41296: {
        if id == 99008 as libc::c_int {} else {
            __assert_fail(
                b"id == 99008\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1396 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_map_get_pair(
        ptr as *mut libc::c_void,
        11 as libc::c_int,
        &mut id,
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_map_get_pair(ptr, 11, &id, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1397 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41241: {
        if binn_map_get_pair(
            ptr as *mut libc::c_void,
            11 as libc::c_int,
            &mut id,
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_map_get_pair(ptr, 11, &id, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1397 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if id == 99009 as libc::c_int {} else {
        __assert_fail(
            b"id == 99009\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41204: {
        if id == 99009 as libc::c_int {} else {
            __assert_fail(
                b"id == 99009\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1398 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(obj as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1402 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41153: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1402 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_pair(
        ptr as *mut libc::c_void,
        -(1 as libc::c_int),
        key.as_mut_ptr(),
        &mut value,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_pair(ptr, -1, key, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1404 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41097: {
        if binn_object_get_pair(
            ptr as *mut libc::c_void,
            -(1 as libc::c_int),
            key.as_mut_ptr(),
            &mut value,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_pair(ptr, -1, key, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1404 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_pair(
        ptr as *mut libc::c_void,
        0 as libc::c_int,
        key.as_mut_ptr(),
        &mut value,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_pair(ptr, 0, key, &value) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1405 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_41043: {
        if binn_object_get_pair(
            ptr as *mut libc::c_void,
            0 as libc::c_int,
            key.as_mut_ptr(),
            &mut value,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_pair(ptr, 0, key, &value) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1405 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_pair(
        ptr as *mut libc::c_void,
        1 as libc::c_int,
        key.as_mut_ptr(),
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_pair(ptr, 1, key, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1407 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40989: {
        if binn_object_get_pair(
            ptr as *mut libc::c_void,
            1 as libc::c_int,
            key.as_mut_ptr(),
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_pair(ptr, 1, key, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1407 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(key.as_mut_ptr(), b"test\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(key, \"test\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1408 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40941: {
        if strcmp(key.as_mut_ptr(), b"test\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(key, \"test\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1408 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_pair(
        ptr as *mut libc::c_void,
        2 as libc::c_int,
        key.as_mut_ptr(),
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_pair(ptr, 2, key, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1409 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40887: {
        if binn_object_get_pair(
            ptr as *mut libc::c_void,
            2 as libc::c_int,
            key.as_mut_ptr(),
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_pair(ptr, 2, key, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1409 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(key.as_mut_ptr(), b"key0\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(key, \"key0\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1410 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40839: {
        if strcmp(key.as_mut_ptr(), b"key0\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(key, \"key0\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1410 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_pair(
        ptr as *mut libc::c_void,
        3 as libc::c_int,
        key.as_mut_ptr(),
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_pair(ptr, 3, key, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1411 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40785: {
        if binn_object_get_pair(
            ptr as *mut libc::c_void,
            3 as libc::c_int,
            key.as_mut_ptr(),
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_pair(ptr, 3, key, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1411 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(key.as_mut_ptr(), b"key1\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(key, \"key1\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1412 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40737: {
        if strcmp(key.as_mut_ptr(), b"key1\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(key, \"key1\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1412 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_pair(
        ptr as *mut libc::c_void,
        10 as libc::c_int,
        key.as_mut_ptr(),
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_pair(ptr, 10, key, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1413 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40683: {
        if binn_object_get_pair(
            ptr as *mut libc::c_void,
            10 as libc::c_int,
            key.as_mut_ptr(),
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_pair(ptr, 10, key, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1413 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(key.as_mut_ptr(), b"key8\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(key, \"key8\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1414 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40635: {
        if strcmp(key.as_mut_ptr(), b"key8\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(key, \"key8\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1414 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_object_get_pair(
        ptr as *mut libc::c_void,
        11 as libc::c_int,
        key.as_mut_ptr(),
        &mut value,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_object_get_pair(ptr, 11, key, &value) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1415 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40579: {
        if binn_object_get_pair(
            ptr as *mut libc::c_void,
            11 as libc::c_int,
            key.as_mut_ptr(),
            &mut value,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_object_get_pair(ptr, 11, key, &value) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1415 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(key.as_mut_ptr(), b"key9\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(key, \"key9\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1416 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40530: {
        if strcmp(key.as_mut_ptr(), b"key9\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(key, \"key9\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1416 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(obj1 as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1423 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40479: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1423 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_object_read(
        ptr as *mut libc::c_void,
        b"key1\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1427 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40416: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1427 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1428 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40380: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1428 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1429 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40344: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1429 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"this is the value\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"this is the value\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1430 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40296: {
        if strcmp(pstr, b"this is the value\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"this is the value\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1430 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_object_read(
        ptr as *mut libc::c_void,
        b"key2\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1434 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40233: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1434 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1435 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40197: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1435 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1436 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40161: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1436 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"the second value\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"the second value\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1437 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40113: {
        if strcmp(pstr, b"the second value\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"the second value\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1437 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint32 = binn_object_read(
        ptr as *mut libc::c_void,
        b"test\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_int;
    if !pint32.is_null() {} else {
        __assert_fail(
            b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1441 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40050: {
        if !pint32.is_null() {} else {
            __assert_fail(
                b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1441 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x60 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1442 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_40014: {
        if type_0 == 0x60 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1442 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint32 as libc::c_uint == vuint32 {} else {
        __assert_fail(
            b"*pint32 == vuint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1444 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39970: {
        if *pint32 as libc::c_uint == vuint32 {} else {
            __assert_fail(
                b"*pint32 == vuint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1444 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(list as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1449 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39919: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1449 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_list_read(
        ptr as *mut libc::c_void,
        2 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1453 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39857: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1453 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1454 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39821: {
        if type_0 == 0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1454 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_list_read(
        ptr as *mut libc::c_void,
        3 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1460 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39759: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1460 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x20 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1461 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39723: {
        if type_0 == 0x20 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1461 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *p2 as libc::c_int == vbyte as libc::c_int {} else {
        __assert_fail(
            b"*p2 == vbyte\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1462 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39677: {
        if *p2 as libc::c_int == vbyte as libc::c_int {} else {
            __assert_fail(
                b"*p2 == vbyte\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1462 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint16 = binn_list_read(
        ptr as *mut libc::c_void,
        4 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_short;
    if !pint16.is_null() {} else {
        __assert_fail(
            b"pint16 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1466 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39615: {
        if !pint16.is_null() {} else {
            __assert_fail(
                b"pint16 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1466 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x41 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1467 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39579: {
        if type_0 == 0x41 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1467 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint16 as libc::c_int == vint16 as libc::c_int {} else {
        __assert_fail(
            b"*pint16 == vint16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1468 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39533: {
        if *pint16 as libc::c_int == vint16 as libc::c_int {} else {
            __assert_fail(
                b"*pint16 == vint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1468 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint16 = binn_list_read(
        ptr as *mut libc::c_void,
        5 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_ushort;
    if !puint16.is_null() {} else {
        __assert_fail(
            b"puint16 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1472 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39471: {
        if !puint16.is_null() {} else {
            __assert_fail(
                b"puint16 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1472 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x40 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1473 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39435: {
        if type_0 == 0x40 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1473 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint16 as libc::c_int == vuint16 as libc::c_int {} else {
        __assert_fail(
            b"*puint16 == vuint16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1474 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39389: {
        if *puint16 as libc::c_int == vuint16 as libc::c_int {} else {
            __assert_fail(
                b"*puint16 == vuint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1474 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint32 = binn_list_read(
        ptr as *mut libc::c_void,
        6 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_int;
    if !pint32.is_null() {} else {
        __assert_fail(
            b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1478 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39327: {
        if !pint32.is_null() {} else {
            __assert_fail(
                b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1478 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x61 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1479 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39291: {
        if type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1479 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint32 == vint32 {} else {
        __assert_fail(
            b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1480 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39249: {
        if *pint32 == vint32 {} else {
            __assert_fail(
                b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1480 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint32 = binn_list_read(
        ptr as *mut libc::c_void,
        6 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_int;
    if !pint32.is_null() {} else {
        __assert_fail(
            b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1484 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39187: {
        if !pint32.is_null() {} else {
            __assert_fail(
                b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1484 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x61 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1485 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39151: {
        if type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1485 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint32 == vint32 {} else {
        __assert_fail(
            b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1486 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39109: {
        if *pint32 == vint32 {} else {
            __assert_fail(
                b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint32 = binn_list_read(
        ptr as *mut libc::c_void,
        7 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_uint;
    if !puint32.is_null() {} else {
        __assert_fail(
            b"puint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1490 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39047: {
        if !puint32.is_null() {} else {
            __assert_fail(
                b"puint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1490 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x60 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1491 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_39011: {
        if type_0 == 0x60 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1491 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint32 == vuint32 {} else {
        __assert_fail(
            b"*puint32 == vuint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1492 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38969: {
        if *puint32 == vuint32 {} else {
            __assert_fail(
                b"*puint32 == vuint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1492 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint64 = binn_list_read(
        ptr as *mut libc::c_void,
        8 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_longlong;
    if !pint64.is_null() {} else {
        __assert_fail(
            b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1496 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38907: {
        if !pint64.is_null() {} else {
            __assert_fail(
                b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1496 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x81 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1497 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38871: {
        if type_0 == 0x81 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1497 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint64 == vint64 {} else {
        __assert_fail(
            b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1498 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38829: {
        if *pint64 == vint64 {} else {
            __assert_fail(
                b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1498 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint64 = binn_list_read(
        ptr as *mut libc::c_void,
        8 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_longlong;
    if !pint64.is_null() {} else {
        __assert_fail(
            b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1502 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38767: {
        if !pint64.is_null() {} else {
            __assert_fail(
                b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1502 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x81 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1503 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38731: {
        if type_0 == 0x81 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1503 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint64 == vint64 {} else {
        __assert_fail(
            b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1504 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38689: {
        if *pint64 == vint64 {} else {
            __assert_fail(
                b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1504 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint64 = binn_list_read(
        ptr as *mut libc::c_void,
        9 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_ulonglong;
    if !puint64.is_null() {} else {
        __assert_fail(
            b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1508 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38627: {
        if !puint64.is_null() {} else {
            __assert_fail(
                b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1508 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x80 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1509 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38591: {
        if type_0 == 0x80 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1509 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint64 == vuint64 {} else {
        __assert_fail(
            b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1510 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38549: {
        if *puint64 == vuint64 {} else {
            __assert_fail(
                b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1510 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_list_read(
        ptr as *mut libc::c_void,
        10 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1514 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38487: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1514 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1515 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38451: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1515 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1516 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38415: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1516 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1517 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38367: {
        if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1517 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_list_read(
        ptr as *mut libc::c_void,
        11 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1521 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38305: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1521 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xc0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1522 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38269: {
        if type_0 == 0xc0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1522 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == blobsize {} else {
        __assert_fail(
            b"size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1523 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38231: {
        if size == blobsize {} else {
            __assert_fail(
                b"size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1523 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(
        p2 as *const libc::c_void,
        pblob as *const libc::c_void,
        blobsize as libc::c_ulong,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(p2, pblob, blobsize) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1524 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38175: {
        if memcmp(
            p2 as *const libc::c_void,
            pblob as *const libc::c_void,
            blobsize as libc::c_ulong,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(p2, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1524 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(map as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1530 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38124: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1530 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_map_read(
        ptr as *mut libc::c_void,
        99000 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1534 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38062: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1534 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1535 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_38026: {
        if type_0 == 0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1535 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_map_read(
        ptr as *mut libc::c_void,
        99001 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1541 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37964: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1541 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x20 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1542 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37928: {
        if type_0 == 0x20 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1542 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *p2 as libc::c_int == vbyte as libc::c_int {} else {
        __assert_fail(
            b"*p2 == vbyte\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1543 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37882: {
        if *p2 as libc::c_int == vbyte as libc::c_int {} else {
            __assert_fail(
                b"*p2 == vbyte\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1543 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint16 = binn_map_read(
        ptr as *mut libc::c_void,
        99002 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_short;
    if !pint16.is_null() {} else {
        __assert_fail(
            b"pint16 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1547 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37820: {
        if !pint16.is_null() {} else {
            __assert_fail(
                b"pint16 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1547 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x41 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1548 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37784: {
        if type_0 == 0x41 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1548 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint16 as libc::c_int == vint16 as libc::c_int {} else {
        __assert_fail(
            b"*pint16 == vint16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1549 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37738: {
        if *pint16 as libc::c_int == vint16 as libc::c_int {} else {
            __assert_fail(
                b"*pint16 == vint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1549 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint16 = binn_map_read(
        ptr as *mut libc::c_void,
        99003 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_ushort;
    if !puint16.is_null() {} else {
        __assert_fail(
            b"puint16 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1553 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37676: {
        if !puint16.is_null() {} else {
            __assert_fail(
                b"puint16 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1553 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x40 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1554 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37640: {
        if type_0 == 0x40 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1554 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint16 as libc::c_int == vuint16 as libc::c_int {} else {
        __assert_fail(
            b"*puint16 == vuint16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1555 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37594: {
        if *puint16 as libc::c_int == vuint16 as libc::c_int {} else {
            __assert_fail(
                b"*puint16 == vuint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1555 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint32 = binn_map_read(
        ptr as *mut libc::c_void,
        99004 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_int;
    if !pint32.is_null() {} else {
        __assert_fail(
            b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1559 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37532: {
        if !pint32.is_null() {} else {
            __assert_fail(
                b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1559 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x61 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1560 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37496: {
        if type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1560 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint32 == vint32 {} else {
        __assert_fail(
            b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37454: {
        if *pint32 == vint32 {} else {
            __assert_fail(
                b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1561 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint32 = binn_map_read(
        ptr as *mut libc::c_void,
        99004 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_int;
    if !pint32.is_null() {} else {
        __assert_fail(
            b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1565 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37392: {
        if !pint32.is_null() {} else {
            __assert_fail(
                b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1565 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x61 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1566 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37356: {
        if type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1566 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint32 == vint32 {} else {
        __assert_fail(
            b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1567 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37314: {
        if *pint32 == vint32 {} else {
            __assert_fail(
                b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1567 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint32 = binn_map_read(
        ptr as *mut libc::c_void,
        99005 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_uint;
    if !puint32.is_null() {} else {
        __assert_fail(
            b"puint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1571 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37252: {
        if !puint32.is_null() {} else {
            __assert_fail(
                b"puint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1571 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x60 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1572 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37216: {
        if type_0 == 0x60 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1572 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint32 == vuint32 {} else {
        __assert_fail(
            b"*puint32 == vuint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1573 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37174: {
        if *puint32 == vuint32 {} else {
            __assert_fail(
                b"*puint32 == vuint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1573 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint64 = binn_map_read(
        ptr as *mut libc::c_void,
        99006 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_longlong;
    if !pint64.is_null() {} else {
        __assert_fail(
            b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1577 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37112: {
        if !pint64.is_null() {} else {
            __assert_fail(
                b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1577 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x81 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1578 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37076: {
        if type_0 == 0x81 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1578 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint64 == vint64 {} else {
        __assert_fail(
            b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1579 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_37034: {
        if *pint64 == vint64 {} else {
            __assert_fail(
                b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1579 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint64 = binn_map_read(
        ptr as *mut libc::c_void,
        99006 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_longlong;
    if !pint64.is_null() {} else {
        __assert_fail(
            b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1583 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36972: {
        if !pint64.is_null() {} else {
            __assert_fail(
                b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1583 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x81 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1584 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36936: {
        if type_0 == 0x81 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1584 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint64 == vint64 {} else {
        __assert_fail(
            b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1585 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36894: {
        if *pint64 == vint64 {} else {
            __assert_fail(
                b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1585 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint64 = binn_map_read(
        ptr as *mut libc::c_void,
        99007 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_ulonglong;
    if !puint64.is_null() {} else {
        __assert_fail(
            b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1589 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36832: {
        if !puint64.is_null() {} else {
            __assert_fail(
                b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1589 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x80 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1590 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36796: {
        if type_0 == 0x80 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1590 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint64 == vuint64 {} else {
        __assert_fail(
            b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1591 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36754: {
        if *puint64 == vuint64 {} else {
            __assert_fail(
                b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1591 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_map_read(
        ptr as *mut libc::c_void,
        99008 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1595 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36692: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1595 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1596 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36656: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1596 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1597 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36620: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1597 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1598 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36572: {
        if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1598 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_map_read(
        ptr as *mut libc::c_void,
        99009 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1602 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36510: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1602 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xc0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1603 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36474: {
        if type_0 == 0xc0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1603 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == blobsize {} else {
        __assert_fail(
            b"size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1604 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36436: {
        if size == blobsize {} else {
            __assert_fail(
                b"size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1604 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(
        p2 as *const libc::c_void,
        pblob as *const libc::c_void,
        blobsize as libc::c_ulong,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(p2, pblob, blobsize) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1605 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36380: {
        if memcmp(
            p2 as *const libc::c_void,
            pblob as *const libc::c_void,
            blobsize as libc::c_ulong,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(p2, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1605 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(obj as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1611 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36329: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1611 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_object_read(
        ptr as *mut libc::c_void,
        b"key0\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1615 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36266: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1615 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1616 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36230: {
        if type_0 == 0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1616 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key1\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1622 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36167: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1622 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x20 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1623 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36131: {
        if type_0 == 0x20 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT8\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1623 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *p2 as libc::c_int == vbyte as libc::c_int {} else {
        __assert_fail(
            b"*p2 == vbyte\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1624 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36084: {
        if *p2 as libc::c_int == vbyte as libc::c_int {} else {
            __assert_fail(
                b"*p2 == vbyte\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1624 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint16 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key2\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_short;
    if !pint16.is_null() {} else {
        __assert_fail(
            b"pint16 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1628 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_36021: {
        if !pint16.is_null() {} else {
            __assert_fail(
                b"pint16 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1628 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x41 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1629 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35985: {
        if type_0 == 0x41 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1629 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint16 as libc::c_int == vint16 as libc::c_int {} else {
        __assert_fail(
            b"*pint16 == vint16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1630 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35937: {
        if *pint16 as libc::c_int == vint16 as libc::c_int {} else {
            __assert_fail(
                b"*pint16 == vint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1630 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint16 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key3\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_ushort;
    if !puint16.is_null() {} else {
        __assert_fail(
            b"puint16 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1634 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35874: {
        if !puint16.is_null() {} else {
            __assert_fail(
                b"puint16 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1634 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x40 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1635 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35838: {
        if type_0 == 0x40 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1635 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint16 as libc::c_int == vuint16 as libc::c_int {} else {
        __assert_fail(
            b"*puint16 == vuint16\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1636 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35790: {
        if *puint16 as libc::c_int == vuint16 as libc::c_int {} else {
            __assert_fail(
                b"*puint16 == vuint16\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1636 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint32 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key4\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_int;
    if !pint32.is_null() {} else {
        __assert_fail(
            b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1640 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35727: {
        if !pint32.is_null() {} else {
            __assert_fail(
                b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1640 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x61 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1641 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35691: {
        if type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1641 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint32 == vint32 {} else {
        __assert_fail(
            b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1642 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35649: {
        if *pint32 == vint32 {} else {
            __assert_fail(
                b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1642 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint32 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key4\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_int;
    if !pint32.is_null() {} else {
        __assert_fail(
            b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1646 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35586: {
        if !pint32.is_null() {} else {
            __assert_fail(
                b"pint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1646 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x61 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1647 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35550: {
        if type_0 == 0x61 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1647 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint32 == vint32 {} else {
        __assert_fail(
            b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1648 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35506: {
        if *pint32 == vint32 {} else {
            __assert_fail(
                b"*pint32 == vint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1648 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint32 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key5\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_uint;
    if !puint32.is_null() {} else {
        __assert_fail(
            b"puint32 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1652 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35443: {
        if !puint32.is_null() {} else {
            __assert_fail(
                b"puint32 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1652 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x60 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1653 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35407: {
        if type_0 == 0x60 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1653 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint32 == vuint32 {} else {
        __assert_fail(
            b"*puint32 == vuint32\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1654 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35363: {
        if *puint32 == vuint32 {} else {
            __assert_fail(
                b"*puint32 == vuint32\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1654 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint64 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key6\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_longlong;
    if !pint64.is_null() {} else {
        __assert_fail(
            b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1658 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35300: {
        if !pint64.is_null() {} else {
            __assert_fail(
                b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1658 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x81 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1659 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35264: {
        if type_0 == 0x81 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1659 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint64 == vint64 {} else {
        __assert_fail(
            b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1660 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35222: {
        if *pint64 == vint64 {} else {
            __assert_fail(
                b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1660 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pint64 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key6\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_longlong;
    if !pint64.is_null() {} else {
        __assert_fail(
            b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1664 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35159: {
        if !pint64.is_null() {} else {
            __assert_fail(
                b"pint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1664 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x81 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1665 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35123: {
        if type_0 == 0x81 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_INT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1665 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *pint64 == vint64 {} else {
        __assert_fail(
            b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1666 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35079: {
        if *pint64 == vint64 {} else {
            __assert_fail(
                b"*pint64 == vint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1666 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint64 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key7\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_ulonglong;
    if !puint64.is_null() {} else {
        __assert_fail(
            b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1670 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_35016: {
        if !puint64.is_null() {} else {
            __assert_fail(
                b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1670 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x80 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1671 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34980: {
        if type_0 == 0x80 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1671 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint64 == vuint64 {} else {
        __assert_fail(
            b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1672 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34938: {
        if *puint64 == vuint64 {} else {
            __assert_fail(
                b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1672 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_object_read(
        ptr as *mut libc::c_void,
        b"key8\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1676 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34875: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1676 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1677 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34839: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1677 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1678 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34803: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1678 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1679 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34755: {
        if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1679 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_object_read(
        ptr as *mut libc::c_void,
        b"key9\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1683 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34692: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1683 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xc0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1684 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34656: {
        if type_0 == 0xc0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1684 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == blobsize {} else {
        __assert_fail(
            b"size == blobsize\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1685 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34618: {
        if size == blobsize {} else {
            __assert_fail(
                b"size == blobsize\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1685 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if memcmp(
        p2 as *const libc::c_void,
        pblob as *const libc::c_void,
        blobsize as libc::c_ulong,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"memcmp(p2, pblob, blobsize) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1686 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34560: {
        if memcmp(
            p2 as *const libc::c_void,
            pblob as *const libc::c_void,
            blobsize as libc::c_ulong,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"memcmp(p2, pblob, blobsize) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1686 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_object_read(
        ptr as *mut libc::c_void,
        b"v2\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1690 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34497: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1690 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1691 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34461: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1691 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == fix_size - 1 as libc::c_int {} else {
        __assert_fail(
            b"size == fix_size - 1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1692 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34419: {
        if size == fix_size - 1 as libc::c_int {} else {
            __assert_fail(
                b"size == fix_size - 1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1692 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strlen(p2) == (fix_size - 1 as libc::c_int) as libc::c_ulong {} else {
        __assert_fail(
            b"strlen(p2) == fix_size - 1\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1693 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34367: {
        if strlen(p2) == (fix_size - 1 as libc::c_int) as libc::c_ulong {} else {
            __assert_fail(
                b"strlen(p2) == fix_size - 1\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1693 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *p2.offset(0 as libc::c_int as isize) as libc::c_int == 'A' as i32 {} else {
        __assert_fail(
            b"p2[0] == 'A'\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1694 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34323: {
        if *p2.offset(0 as libc::c_int as isize) as libc::c_int == 'A' as i32 {} else {
            __assert_fail(
                b"p2[0] == 'A'\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1694 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *p2.offset(1 as libc::c_int as isize) as libc::c_int == 'A' as i32 {} else {
        __assert_fail(
            b"p2[1] == 'A'\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1695 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34279: {
        if *p2.offset(1 as libc::c_int as isize) as libc::c_int == 'A' as i32 {} else {
            __assert_fail(
                b"p2[1] == 'A'\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1695 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *p2.offset(500 as libc::c_int as isize) as libc::c_int == 'A' as i32 {} else {
        __assert_fail(
            b"p2[500] == 'A'\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1696 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34235: {
        if *p2.offset(500 as libc::c_int as isize) as libc::c_int == 'A' as i32 {} else {
            __assert_fail(
                b"p2[500] == 'A'\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1696 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *p2.offset((fix_size - 1 as libc::c_int) as isize) as libc::c_int
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"p2[fix_size-1] == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1697 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34183: {
        if *p2.offset((fix_size - 1 as libc::c_int) as isize) as libc::c_int
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"p2[fix_size-1] == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1697 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_object_read(
        ptr as *mut libc::c_void,
        b"key00\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1701 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34120: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1701 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1702 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34084: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1702 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1703 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34048: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1703 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"after the big string\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"after the big string\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1704 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_34000: {
        if strcmp(pstr, b"after the big string\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"after the big string\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1704 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    p2 = binn_object_read(
        ptr as *mut libc::c_void,
        b"list\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !p2.is_null() {} else {
        __assert_fail(
            b"p2 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1708 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33937: {
        if !p2.is_null() {} else {
            __assert_fail(
                b"p2 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1708 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1709 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33901: {
        if type_0 == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1709 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1710 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33865: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1710 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    puint64 = binn_list_read(
        p2 as *mut libc::c_void,
        9 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_ulonglong;
    if !puint64.is_null() {} else {
        __assert_fail(
            b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1714 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33803: {
        if !puint64.is_null() {} else {
            __assert_fail(
                b"puint64 != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1714 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0x80 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1715 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33767: {
        if type_0 == 0x80 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_UINT64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1715 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if *puint64 == vuint64 {} else {
        __assert_fail(
            b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1716 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33723: {
        if *puint64 == vuint64 {} else {
            __assert_fail(
                b"*puint64 == vuint64\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1716 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_list_read(
        p2 as *mut libc::c_void,
        10 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1720 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33661: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1720 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1721 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33625: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1721 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1722 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33589: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1722 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1723 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33541: {
        if strcmp(pstr, b"this is the string\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"this is the string\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1723 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_list_read(
        p2 as *mut libc::c_void,
        12 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1727 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33479: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1727 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xc0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1728 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33443: {
        if type_0 == 0xc0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_BLOB\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1728 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == 0 as libc::c_int {} else {
        __assert_fail(
            b"size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1729 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33407: {
        if size == 0 as libc::c_int {} else {
            __assert_fail(
                b"size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1729 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_list_read(
        p2 as *mut libc::c_void,
        13 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1733 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33345: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1733 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1734 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33309: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1734 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == 0 as libc::c_int {} else {
        __assert_fail(
            b"size == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1735 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33273: {
        if size == 0 as libc::c_int {} else {
            __assert_fail(
                b"size == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1735 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1736 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33225: {
        if strcmp(pstr, b"\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"\") == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1736 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_list_read(
        p2 as *mut libc::c_void,
        14 as libc::c_int,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1740 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33161: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1740 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1741 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33125: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1741 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1742 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33089: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1742 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"after the empty items\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"after the empty items\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1743 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_33041: {
        if strcmp(pstr, b"after the empty items\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"after the empty items\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1743 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    type_0 = 0 as libc::c_int;
    size = 0 as libc::c_int;
    pstr = binn_object_read(
        ptr as *mut libc::c_void,
        b"key10\0" as *const u8 as *const libc::c_char,
        &mut type_0,
        &mut size,
    ) as *mut libc::c_char;
    if !pstr.is_null() {} else {
        __assert_fail(
            b"pstr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1747 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32977: {
        if !pstr.is_null() {} else {
            __assert_fail(
                b"pstr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1747 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xa0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1748 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32941: {
        if type_0 == 0xa0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_STRING\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1748 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as libc::c_int {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1749 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32905: {
        if size > 0 as libc::c_int {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1749 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if strcmp(pstr, b"after the list\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(pstr, \"after the list\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1750 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32855: {
        if strcmp(pstr, b"after the list\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(pstr, \"after the list\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1750 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if (binn_ptr(0 as *mut libc::c_void)).is_null() {} else {
        __assert_fail(
            b"binn_ptr(NULL) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1758 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32807: {
        if (binn_ptr(0 as *mut libc::c_void)).is_null() {} else {
            __assert_fail(
                b"binn_ptr(NULL) == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1758 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if IsValidBinnHeader(
        0 as *mut libc::c_void,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"IsValidBinnHeader(NULL, NULL, NULL, NULL, NULL) == FALSE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1764 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32731: {
        if IsValidBinnHeader(
            0 as *mut libc::c_void,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"IsValidBinnHeader(NULL, NULL, NULL, NULL, NULL) == FALSE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1764 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(obj as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1767 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32680: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1767 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    size = 0 as libc::c_int;
    if IsValidBinnHeader(
        ptr as *mut libc::c_void,
        &mut type_0,
        &mut count,
        &mut size,
        &mut header_size,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"IsValidBinnHeader(ptr, &type, &count, &size, &header_size) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1770 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32617: {
        if IsValidBinnHeader(
            ptr as *mut libc::c_void,
            &mut type_0,
            &mut count,
            &mut size,
            &mut header_size,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"IsValidBinnHeader(ptr, &type, &count, &size, &header_size) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1770 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1771 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32581: {
        if type_0 == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1771 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if count == 15 as libc::c_int {} else {
        __assert_fail(
            b"count == 15\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1772 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32545: {
        if count == 15 as libc::c_int {} else {
            __assert_fail(
                b"count == 15\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1772 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
        __assert_fail(
            b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1773 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32499: {
        if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
            __assert_fail(
                b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1773 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 3 as libc::c_int {} else {
        __assert_fail(
            b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1774 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32463: {
        if size > 3 as libc::c_int {} else {
            __assert_fail(
                b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1774 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == (*obj).size {} else {
        __assert_fail(
            b"size == obj->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1775 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32421: {
        if size == (*obj).size {} else {
            __assert_fail(
                b"size == obj->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1775 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_is_valid(ptr as *mut libc::c_void, &mut type_0, &mut count, &mut size)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_is_valid(ptr, &type, &count, &size) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1777 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32365: {
        if binn_is_valid(ptr as *mut libc::c_void, &mut type_0, &mut count, &mut size)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_is_valid(ptr, &type, &count, &size) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1777 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xe2 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1778 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32329: {
        if type_0 == 0xe2 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_OBJECT\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1778 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if count == 15 as libc::c_int {} else {
        __assert_fail(
            b"count == 15\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1779 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32293: {
        if count == 15 as libc::c_int {} else {
            __assert_fail(
                b"count == 15\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1779 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 3 as libc::c_int {} else {
        __assert_fail(
            b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1780 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32257: {
        if size > 3 as libc::c_int {} else {
            __assert_fail(
                b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1780 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == (*obj).size {} else {
        __assert_fail(
            b"size == obj->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1781 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32215: {
        if size == (*obj).size {} else {
            __assert_fail(
                b"size == obj->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1781 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(map as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1784 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32164: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1784 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    size = 0 as libc::c_int;
    if IsValidBinnHeader(
        ptr as *mut libc::c_void,
        &mut type_0,
        &mut count,
        &mut size,
        &mut header_size,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"IsValidBinnHeader(ptr, &type, &count, &size, &header_size) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1787 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32101: {
        if IsValidBinnHeader(
            ptr as *mut libc::c_void,
            &mut type_0,
            &mut count,
            &mut size,
            &mut header_size,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"IsValidBinnHeader(ptr, &type, &count, &size, &header_size) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1787 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xe1 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_MAP\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1788 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32065: {
        if type_0 == 0xe1 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_MAP\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1788 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if count == 11 as libc::c_int {} else {
        __assert_fail(
            b"count == 11\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1789 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_32029: {
        if count == 11 as libc::c_int {} else {
            __assert_fail(
                b"count == 11\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1789 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
        __assert_fail(
            b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1790 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31983: {
        if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
            __assert_fail(
                b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1790 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 3 as libc::c_int {} else {
        __assert_fail(
            b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1791 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31947: {
        if size > 3 as libc::c_int {} else {
            __assert_fail(
                b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1791 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == (*map).size {} else {
        __assert_fail(
            b"size == map->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1792 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31905: {
        if size == (*map).size {} else {
            __assert_fail(
                b"size == map->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1792 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_is_valid(ptr as *mut libc::c_void, &mut type_0, &mut count, &mut size)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_is_valid(ptr, &type, &count, &size) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1794 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31849: {
        if binn_is_valid(ptr as *mut libc::c_void, &mut type_0, &mut count, &mut size)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_is_valid(ptr, &type, &count, &size) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1794 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xe1 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_MAP\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1795 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31813: {
        if type_0 == 0xe1 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_MAP\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1795 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if count == 11 as libc::c_int {} else {
        __assert_fail(
            b"count == 11\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1796 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31777: {
        if count == 11 as libc::c_int {} else {
            __assert_fail(
                b"count == 11\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1796 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 3 as libc::c_int {} else {
        __assert_fail(
            b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1797 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31741: {
        if size > 3 as libc::c_int {} else {
            __assert_fail(
                b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1797 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == (*map).size {} else {
        __assert_fail(
            b"size == map->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1798 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31699: {
        if size == (*map).size {} else {
            __assert_fail(
                b"size == map->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1798 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    ptr = binn_ptr(list as *mut libc::c_void) as *mut libc::c_char;
    if !ptr.is_null() {} else {
        __assert_fail(
            b"ptr != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1801 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31648: {
        if !ptr.is_null() {} else {
            __assert_fail(
                b"ptr != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1801 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    size = 0 as libc::c_int;
    if IsValidBinnHeader(
        ptr as *mut libc::c_void,
        &mut type_0,
        &mut count,
        &mut size,
        &mut header_size,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"IsValidBinnHeader(ptr, &type, &count, &size, &header_size) == TRUE\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1804 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31584: {
        if IsValidBinnHeader(
            ptr as *mut libc::c_void,
            &mut type_0,
            &mut count,
            &mut size,
            &mut header_size,
        ) == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"IsValidBinnHeader(ptr, &type, &count, &size, &header_size) == TRUE\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1804 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1805 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31548: {
        if type_0 == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1805 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if count == 14 as libc::c_int {} else {
        __assert_fail(
            b"count == 14\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1806 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31512: {
        if count == 14 as libc::c_int {} else {
            __assert_fail(
                b"count == 14\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1806 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
        __assert_fail(
            b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1807 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31466: {
        if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
            __assert_fail(
                b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1807 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 3 as libc::c_int {} else {
        __assert_fail(
            b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1808 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31430: {
        if size > 3 as libc::c_int {} else {
            __assert_fail(
                b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1808 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == (*list).size {} else {
        __assert_fail(
            b"size == list->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1809 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31388: {
        if size == (*list).size {} else {
            __assert_fail(
                b"size == list->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1809 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_is_valid(ptr as *mut libc::c_void, &mut type_0, &mut count, &mut size)
        == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"binn_is_valid(ptr, &type, &count, &size) == TRUE\0" as *const u8
                as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1811 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31330: {
        if binn_is_valid(ptr as *mut libc::c_void, &mut type_0, &mut count, &mut size)
            == 1 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_is_valid(ptr, &type, &count, &size) == TRUE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1811 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if type_0 == 0xe0 as libc::c_int {} else {
        __assert_fail(
            b"type == BINN_LIST\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1812 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31293: {
        if type_0 == 0xe0 as libc::c_int {} else {
            __assert_fail(
                b"type == BINN_LIST\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1812 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if count == 14 as libc::c_int {} else {
        __assert_fail(
            b"count == 14\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1813 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31256: {
        if count == 14 as libc::c_int {} else {
            __assert_fail(
                b"count == 14\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1813 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
        __assert_fail(
            b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1814 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31209: {
        if header_size >= 3 as libc::c_int && header_size <= 9 as libc::c_int {} else {
            __assert_fail(
                b"header_size >= MIN_BINN_SIZE && header_size <= MAX_BINN_HEADER\0"
                    as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1814 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size > 3 as libc::c_int {} else {
        __assert_fail(
            b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1815 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31173: {
        if size > 3 as libc::c_int {} else {
            __assert_fail(
                b"size > MIN_BINN_SIZE\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1815 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if size == (*list).size {} else {
        __assert_fail(
            b"size == list->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1816 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31130: {
        if size == (*list).size {} else {
            __assert_fail(
                b"size == list->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1816 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(0 as *mut libc::c_void) == 0 as libc::c_int {} else {
        __assert_fail(
            b"binn_size(NULL) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1822 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31086: {
        if binn_size(0 as *mut libc::c_void) == 0 as libc::c_int {} else {
            __assert_fail(
                b"binn_size(NULL) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1822 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(list as *mut libc::c_void) == (*list).size {} else {
        __assert_fail(
            b"binn_size(list) == list->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1824 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_31036: {
        if binn_size(list as *mut libc::c_void) == (*list).size {} else {
            __assert_fail(
                b"binn_size(list) == list->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1824 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(map as *mut libc::c_void) == (*map).size {} else {
        __assert_fail(
            b"binn_size(map) == map->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1825 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_30986: {
        if binn_size(map as *mut libc::c_void) == (*map).size {} else {
            __assert_fail(
                b"binn_size(map) == map->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1825 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(obj as *mut libc::c_void) == (*obj).size {} else {
        __assert_fail(
            b"binn_size(obj) == obj->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1826 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_30936: {
        if binn_size(obj as *mut libc::c_void) == (*obj).size {} else {
            __assert_fail(
                b"binn_size(obj) == obj->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1826 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    if binn_size(obj1 as *mut libc::c_void) == (*obj1).size {} else {
        __assert_fail(
            b"binn_size(obj1) == obj1->size\0" as *const u8 as *const libc::c_char,
            b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
            1827 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"void test3()\0"))
                .as_ptr(),
        );
    }
    'c_30885: {
        if binn_size(obj1 as *mut libc::c_void) == (*obj1).size {} else {
            __assert_fail(
                b"binn_size(obj1) == obj1->size\0" as *const u8 as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1827 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 13],
                    &[libc::c_char; 13],
                >(b"void test3()\0"))
                    .as_ptr(),
            );
        }
    };
    binn_free(list);
    binn_free(map);
    binn_free(obj);
    printf(b"OK\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn test_invalid_binn() {
    let mut buffers: [[libc::c_char; 20]; 36] = [
        [
            0xe0 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0x1 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x1 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0x1 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x1 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x7e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0x1 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0x7f as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x1 as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0xe0 as libc::c_int as libc::c_char,
            0x8e as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0x12 as libc::c_int as libc::c_char,
            0x34 as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0xff as libc::c_int as libc::c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ];
    let mut count: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    puts(b"testing invalid binn buffers...\0" as *const u8 as *const libc::c_char);
    count = (::std::mem::size_of::<[[libc::c_char; 20]; 36]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        ptr = (buffers[i as usize]).as_mut_ptr();
        size = strlen(ptr) as libc::c_int;
        printf(
            b"checking invalid binn #%d   size: %d bytes\n\0" as *const u8
                as *const libc::c_char,
            i,
            size,
        );
        if binn_is_valid_ex(
            ptr as *mut libc::c_void,
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
            &mut size,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binn_is_valid_ex(ptr, NULL, NULL, &size) == FALSE\0" as *const u8
                    as *const libc::c_char,
                b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                1894 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_invalid_binn()\0"))
                    .as_ptr(),
            );
        }
        'c_46874: {
            if binn_is_valid_ex(
                ptr as *mut libc::c_void,
                0 as *mut libc::c_int,
                0 as *mut libc::c_int,
                &mut size,
            ) == 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"binn_is_valid_ex(ptr, NULL, NULL, &size) == FALSE\0" as *const u8
                        as *const libc::c_char,
                    b"test/test_binn.c\0" as *const u8 as *const libc::c_char,
                    1894 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void test_invalid_binn()\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    puts(b"OK\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    puts(
        b"\nStarting the unit/regression tests...\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"sizeof(binn) = %d\n\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<binn>() as libc::c_ulong,
    );
    test_binn_version();
    test_endianess();
    test_int64();
    test_floating_point_numbers();
    test1();
    test2(0 as libc::c_int);
    test2(1 as libc::c_int);
    test_binn2();
    test3();
    test_invalid_binn();
    puts(b"\nAll tests pass! :)\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
