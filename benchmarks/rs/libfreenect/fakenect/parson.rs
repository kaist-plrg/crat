use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn rewind(__stream: *mut FILE);
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_t {
    pub wrapping_value: *mut JSON_Value,
    pub names: *mut *mut libc::c_char,
    pub values: *mut *mut JSON_Value,
    pub count: size_t,
    pub capacity: size_t,
}
pub type JSON_Value = json_value_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_t {
    pub parent: *mut JSON_Value,
    pub type_0: JSON_Value_Type,
    pub value: JSON_Value_Value,
}
pub type JSON_Value_Value = json_value_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub union json_value_value {
    pub string: *mut libc::c_char,
    pub number: libc::c_double,
    pub object: *mut JSON_Object,
    pub array: *mut JSON_Array,
    pub boolean: libc::c_int,
    pub null: libc::c_int,
}
pub type JSON_Array = json_array_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_t {
    pub wrapping_value: *mut JSON_Value,
    pub items: *mut *mut JSON_Value,
    pub count: size_t,
    pub capacity: size_t,
}
pub type JSON_Object = json_object_t;
pub type JSON_Value_Type = libc::c_int;
pub type json_value_type = libc::c_int;
pub const JSONBoolean: json_value_type = 6;
pub const JSONArray: json_value_type = 5;
pub const JSONObject: json_value_type = 4;
pub const JSONNumber: json_value_type = 3;
pub const JSONString: json_value_type = 2;
pub const JSONNull: json_value_type = 1;
pub const JSONError: json_value_type = -1;
pub type json_result_t = libc::c_int;
pub const JSONFailure: json_result_t = -1;
pub const JSONSuccess: json_result_t = 0;
pub type JSON_Status = libc::c_int;
pub type JSON_Malloc_Function = Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
>;
pub type JSON_Free_Function = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
static mut parson_malloc: JSON_Malloc_Function = unsafe {
    Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void)
};
static mut parson_free: JSON_Free_Function = unsafe {
    Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())
};
unsafe extern "C" fn parson_strndup(
    mut string: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut output_string: *mut libc::c_char = parson_malloc
        .unwrap()(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if output_string.is_null() {
        return 0 as *mut libc::c_char;
    }
    *output_string.offset(n as isize) = '\0' as i32 as libc::c_char;
    strncpy(output_string, string, n);
    return output_string;
}
unsafe extern "C" fn parson_strdup(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    return parson_strndup(string, strlen(string));
}
unsafe extern "C" fn hex_char_to_int(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return c as libc::c_int - '0' as i32
    } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_utf16_hex(
    mut s: *const libc::c_char,
    mut result: *mut libc::c_uint,
) -> libc::c_int {
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut x3: libc::c_int = 0;
    let mut x4: libc::c_int = 0;
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *s.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *s.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *s.offset(3 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    x1 = hex_char_to_int(*s.offset(0 as libc::c_int as isize));
    x2 = hex_char_to_int(*s.offset(1 as libc::c_int as isize));
    x3 = hex_char_to_int(*s.offset(2 as libc::c_int as isize));
    x4 = hex_char_to_int(*s.offset(3 as libc::c_int as isize));
    if x1 == -(1 as libc::c_int) || x2 == -(1 as libc::c_int)
        || x3 == -(1 as libc::c_int) || x4 == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    *result = (x1 << 12 as libc::c_int | x2 << 8 as libc::c_int | x3 << 4 as libc::c_int
        | x4) as libc::c_uint;
    return 1 as libc::c_int;
}
unsafe extern "C" fn num_bytes_in_utf8_sequence(mut c: libc::c_uchar) -> libc::c_int {
    if c as libc::c_int == 0xc0 as libc::c_int || c as libc::c_int == 0xc1 as libc::c_int
        || c as libc::c_int > 0xf4 as libc::c_int
        || c as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int
    {
        return 0 as libc::c_int
    } else if c as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int
    } else if c as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
        return 2 as libc::c_int
    } else if c as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
        return 3 as libc::c_int
    } else if c as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
        return 4 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn verify_utf8_sequence(
    mut string: *const libc::c_uchar,
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let mut cp: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    *len = num_bytes_in_utf8_sequence(*string.offset(0 as libc::c_int as isize));
    if *len == 1 as libc::c_int {
        cp = *string.offset(0 as libc::c_int as isize) as libc::c_uint;
    } else if *len == 2 as libc::c_int
        && *string.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
    {
        cp = (*string.offset(0 as libc::c_int as isize) as libc::c_int
            & 0x1f as libc::c_int) as libc::c_uint;
        cp = cp << 6 as libc::c_int
            | (*string.offset(1 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) as libc::c_uint;
    } else if *len == 3 as libc::c_int
        && *string.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        && *string.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
    {
        cp = (*string.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf as libc::c_int) as libc::c_uint;
        cp = cp << 6 as libc::c_int
            | (*string.offset(1 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) as libc::c_uint;
        cp = cp << 6 as libc::c_int
            | (*string.offset(2 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) as libc::c_uint;
    } else if *len == 4 as libc::c_int
        && *string.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        && *string.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
        && *string.offset(3 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
            == 0x80 as libc::c_int
    {
        cp = (*string.offset(0 as libc::c_int as isize) as libc::c_int
            & 0x7 as libc::c_int) as libc::c_uint;
        cp = cp << 6 as libc::c_int
            | (*string.offset(1 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) as libc::c_uint;
        cp = cp << 6 as libc::c_int
            | (*string.offset(2 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) as libc::c_uint;
        cp = cp << 6 as libc::c_int
            | (*string.offset(3 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int) as libc::c_uint;
    } else {
        return 0 as libc::c_int
    }
    if cp < 0x80 as libc::c_int as libc::c_uint && *len > 1 as libc::c_int
        || cp < 0x800 as libc::c_int as libc::c_uint && *len > 2 as libc::c_int
        || cp < 0x10000 as libc::c_int as libc::c_uint && *len > 3 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if cp > 0x10ffff as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if cp >= 0xd800 as libc::c_int as libc::c_uint
        && cp <= 0xdfff as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_valid_utf8(
    mut string: *const libc::c_char,
    mut string_len: size_t,
) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut string_end: *const libc::c_char = string.offset(string_len as isize);
    while string < string_end {
        if verify_utf8_sequence(string as *const libc::c_uchar, &mut len) == 0 {
            return 0 as libc::c_int;
        }
        string = string.offset(len as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_decimal(
    mut string: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    if length > 1 as libc::c_int as libc::c_ulong
        && *string.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && *string.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32
    {
        return 0 as libc::c_int;
    }
    if length > 2 as libc::c_int as libc::c_ulong
        && strncmp(
            string,
            b"-0\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 && *string.offset(2 as libc::c_int as isize) as libc::c_int != '.' as i32
    {
        return 0 as libc::c_int;
    }
    loop {
        let fresh0 = length;
        length = length.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        if !(strchr(
            b"xX\0" as *const u8 as *const libc::c_char,
            *string.offset(length as isize) as libc::c_int,
        ))
            .is_null()
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_file(mut filename: *const libc::c_char) -> *mut libc::c_char {
    let mut fp: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    let mut file_size: size_t = 0;
    let mut pos: libc::c_long = 0;
    let mut file_contents: *mut libc::c_char = 0 as *mut libc::c_char;
    if fp.is_null() {
        return 0 as *mut libc::c_char;
    }
    fseek(fp, 0 as libc::c_long, 2 as libc::c_int);
    pos = ftell(fp);
    if pos < 0 as libc::c_int as libc::c_long {
        fclose(fp);
        return 0 as *mut libc::c_char;
    }
    file_size = pos as size_t;
    rewind(fp);
    file_contents = parson_malloc
        .unwrap()(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(file_size.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_char;
    if file_contents.is_null() {
        fclose(fp);
        return 0 as *mut libc::c_char;
    }
    if fread(
        file_contents as *mut libc::c_void,
        file_size,
        1 as libc::c_int as libc::c_ulong,
        fp,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        if ferror(fp) != 0 {
            fclose(fp);
            parson_free.unwrap()(file_contents as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
    }
    fclose(fp);
    *file_contents.offset(file_size as isize) = '\0' as i32 as libc::c_char;
    return file_contents;
}
unsafe extern "C" fn remove_comments(
    mut string: *mut libc::c_char,
    mut start_token: *const libc::c_char,
    mut end_token: *const libc::c_char,
) {
    let mut in_string: libc::c_int = 0 as libc::c_int;
    let mut escaped: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_char: libc::c_char = 0;
    let mut start_token_len: size_t = strlen(start_token);
    let mut end_token_len: size_t = strlen(end_token);
    if start_token_len == 0 as libc::c_int as libc::c_ulong
        || end_token_len == 0 as libc::c_int as libc::c_ulong
    {
        return;
    }
    loop {
        current_char = *string;
        if !(current_char as libc::c_int != '\0' as i32) {
            break;
        }
        if current_char as libc::c_int == '\\' as i32 && escaped == 0 {
            escaped = 1 as libc::c_int;
            string = string.offset(1);
            string;
        } else {
            if current_char as libc::c_int == '"' as i32 && escaped == 0 {
                in_string = (in_string == 0) as libc::c_int;
            } else if in_string == 0
                && strncmp(string, start_token, start_token_len) == 0 as libc::c_int
            {
                i = 0 as libc::c_int as size_t;
                while i < start_token_len {
                    *string.offset(i as isize) = ' ' as i32 as libc::c_char;
                    i = i.wrapping_add(1);
                    i;
                }
                string = string.offset(start_token_len as isize);
                ptr = strstr(string, end_token);
                if ptr.is_null() {
                    return;
                }
                i = 0 as libc::c_int as size_t;
                while i
                    < (ptr.offset_from(string) as libc::c_long as libc::c_ulong)
                        .wrapping_add(end_token_len)
                {
                    *string.offset(i as isize) = ' ' as i32 as libc::c_char;
                    i = i.wrapping_add(1);
                    i;
                }
                string = ptr
                    .offset(end_token_len as isize)
                    .offset(-(1 as libc::c_int as isize));
            }
            escaped = 0 as libc::c_int;
            string = string.offset(1);
            string;
        }
    };
}
unsafe extern "C" fn json_object_init(
    mut wrapping_value: *mut JSON_Value,
) -> *mut JSON_Object {
    let mut new_obj: *mut JSON_Object = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Object>() as libc::c_ulong)
        as *mut JSON_Object;
    if new_obj.is_null() {
        return 0 as *mut JSON_Object;
    }
    (*new_obj).wrapping_value = wrapping_value;
    (*new_obj).names = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*new_obj).values = 0 as *mut libc::c_void as *mut *mut JSON_Value;
    (*new_obj).capacity = 0 as libc::c_int as size_t;
    (*new_obj).count = 0 as libc::c_int as size_t;
    return new_obj;
}
unsafe extern "C" fn json_object_add(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut value: *mut JSON_Value,
) -> JSON_Status {
    let mut index: size_t = 0 as libc::c_int as size_t;
    if object.is_null() || name.is_null() || value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if !(json_object_get_value(object, name)).is_null() {
        return JSONFailure as libc::c_int;
    }
    if (*object).count >= (*object).capacity {
        let mut new_capacity: size_t = if ((*object).capacity)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            > 16 as libc::c_int as libc::c_ulong
        {
            ((*object).capacity).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        } else {
            16 as libc::c_int as libc::c_ulong
        };
        if json_object_resize(object, new_capacity) == JSONFailure as libc::c_int {
            return JSONFailure as libc::c_int;
        }
    }
    index = (*object).count;
    let ref mut fresh1 = *((*object).names).offset(index as isize);
    *fresh1 = parson_strdup(name);
    if (*((*object).names).offset(index as isize)).is_null() {
        return JSONFailure as libc::c_int;
    }
    (*value).parent = json_object_get_wrapping_value(object);
    let ref mut fresh2 = *((*object).values).offset(index as isize);
    *fresh2 = value;
    (*object).count = ((*object).count).wrapping_add(1);
    (*object).count;
    return JSONSuccess as libc::c_int;
}
unsafe extern "C" fn json_object_resize(
    mut object: *mut JSON_Object,
    mut new_capacity: size_t,
) -> JSON_Status {
    let mut temp_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut temp_values: *mut *mut JSON_Value = 0 as *mut *mut JSON_Value;
    if ((*object).names).is_null() && !((*object).values).is_null()
        || !((*object).names).is_null() && ((*object).values).is_null()
        || new_capacity == 0 as libc::c_int as libc::c_ulong
    {
        return JSONFailure as libc::c_int;
    }
    temp_names = parson_malloc
        .unwrap()(
        new_capacity
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if temp_names.is_null() {
        return JSONFailure as libc::c_int;
    }
    temp_values = parson_malloc
        .unwrap()(
        new_capacity
            .wrapping_mul(::std::mem::size_of::<*mut JSON_Value>() as libc::c_ulong),
    ) as *mut *mut JSON_Value;
    if temp_values.is_null() {
        parson_free.unwrap()(temp_names as *mut libc::c_void);
        return JSONFailure as libc::c_int;
    }
    if !((*object).names).is_null() && !((*object).values).is_null()
        && (*object).count > 0 as libc::c_int as libc::c_ulong
    {
        memcpy(
            temp_names as *mut libc::c_void,
            (*object).names as *const libc::c_void,
            ((*object).count)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        );
        memcpy(
            temp_values as *mut libc::c_void,
            (*object).values as *const libc::c_void,
            ((*object).count)
                .wrapping_mul(::std::mem::size_of::<*mut JSON_Value>() as libc::c_ulong),
        );
    }
    parson_free.unwrap()((*object).names as *mut libc::c_void);
    parson_free.unwrap()((*object).values as *mut libc::c_void);
    (*object).names = temp_names;
    (*object).values = temp_values;
    (*object).capacity = new_capacity;
    return JSONSuccess as libc::c_int;
}
unsafe extern "C" fn json_object_nget_value(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
    mut n: size_t,
) -> *mut JSON_Value {
    let mut i: size_t = 0;
    let mut name_length: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < json_object_get_count(object) {
        name_length = strlen(*((*object).names).offset(i as isize));
        if !(name_length != n) {
            if strncmp(*((*object).names).offset(i as isize), name, n)
                == 0 as libc::c_int
            {
                return *((*object).values).offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut JSON_Value;
}
unsafe extern "C" fn json_object_free(mut object: *mut JSON_Object) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*object).count {
        parson_free.unwrap()(*((*object).names).offset(i as isize) as *mut libc::c_void);
        json_value_free(*((*object).values).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    parson_free.unwrap()((*object).names as *mut libc::c_void);
    parson_free.unwrap()((*object).values as *mut libc::c_void);
    parson_free.unwrap()(object as *mut libc::c_void);
}
unsafe extern "C" fn json_array_init(
    mut wrapping_value: *mut JSON_Value,
) -> *mut JSON_Array {
    let mut new_array: *mut JSON_Array = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Array>() as libc::c_ulong)
        as *mut JSON_Array;
    if new_array.is_null() {
        return 0 as *mut JSON_Array;
    }
    (*new_array).wrapping_value = wrapping_value;
    (*new_array).items = 0 as *mut libc::c_void as *mut *mut JSON_Value;
    (*new_array).capacity = 0 as libc::c_int as size_t;
    (*new_array).count = 0 as libc::c_int as size_t;
    return new_array;
}
unsafe extern "C" fn json_array_add(
    mut array: *mut JSON_Array,
    mut value: *mut JSON_Value,
) -> JSON_Status {
    if (*array).count >= (*array).capacity {
        let mut new_capacity: size_t = if ((*array).capacity)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            > 16 as libc::c_int as libc::c_ulong
        {
            ((*array).capacity).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        } else {
            16 as libc::c_int as libc::c_ulong
        };
        if json_array_resize(array, new_capacity) == JSONFailure as libc::c_int {
            return JSONFailure as libc::c_int;
        }
    }
    (*value).parent = json_array_get_wrapping_value(array);
    let ref mut fresh3 = *((*array).items).offset((*array).count as isize);
    *fresh3 = value;
    (*array).count = ((*array).count).wrapping_add(1);
    (*array).count;
    return JSONSuccess as libc::c_int;
}
unsafe extern "C" fn json_array_resize(
    mut array: *mut JSON_Array,
    mut new_capacity: size_t,
) -> JSON_Status {
    let mut new_items: *mut *mut JSON_Value = 0 as *mut *mut JSON_Value;
    if new_capacity == 0 as libc::c_int as libc::c_ulong {
        return JSONFailure as libc::c_int;
    }
    new_items = parson_malloc
        .unwrap()(
        new_capacity
            .wrapping_mul(::std::mem::size_of::<*mut JSON_Value>() as libc::c_ulong),
    ) as *mut *mut JSON_Value;
    if new_items.is_null() {
        return JSONFailure as libc::c_int;
    }
    if !((*array).items).is_null() && (*array).count > 0 as libc::c_int as libc::c_ulong
    {
        memcpy(
            new_items as *mut libc::c_void,
            (*array).items as *const libc::c_void,
            ((*array).count)
                .wrapping_mul(::std::mem::size_of::<*mut JSON_Value>() as libc::c_ulong),
        );
    }
    parson_free.unwrap()((*array).items as *mut libc::c_void);
    (*array).items = new_items;
    (*array).capacity = new_capacity;
    return JSONSuccess as libc::c_int;
}
unsafe extern "C" fn json_array_free(mut array: *mut JSON_Array) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*array).count {
        json_value_free(*((*array).items).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    parson_free.unwrap()((*array).items as *mut libc::c_void);
    parson_free.unwrap()(array as *mut libc::c_void);
}
unsafe extern "C" fn json_value_init_string_no_copy(
    mut string: *mut libc::c_char,
) -> *mut JSON_Value {
    let mut new_value: *mut JSON_Value = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Value>() as libc::c_ulong)
        as *mut JSON_Value;
    if new_value.is_null() {
        return 0 as *mut JSON_Value;
    }
    (*new_value).parent = 0 as *mut JSON_Value;
    (*new_value).type_0 = JSONString as libc::c_int;
    (*new_value).value.string = string;
    return new_value;
}
unsafe extern "C" fn skip_quotes(mut string: *mut *const libc::c_char) -> JSON_Status {
    if **string as libc::c_int != '"' as i32 {
        return JSONFailure as libc::c_int;
    }
    *string = (*string).offset(1);
    *string;
    while **string as libc::c_int != '"' as i32 {
        if **string as libc::c_int == '\0' as i32 {
            return JSONFailure as libc::c_int
        } else if **string as libc::c_int == '\\' as i32 {
            *string = (*string).offset(1);
            *string;
            if **string as libc::c_int == '\0' as i32 {
                return JSONFailure as libc::c_int;
            }
        }
        *string = (*string).offset(1);
        *string;
    }
    *string = (*string).offset(1);
    *string;
    return JSONSuccess as libc::c_int;
}
unsafe extern "C" fn parse_utf16(
    mut unprocessed: *mut *const libc::c_char,
    mut processed: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cp: libc::c_uint = 0;
    let mut lead: libc::c_uint = 0;
    let mut trail: libc::c_uint = 0;
    let mut parse_succeeded: libc::c_int = 0 as libc::c_int;
    let mut processed_ptr: *mut libc::c_char = *processed;
    let mut unprocessed_ptr: *const libc::c_char = *unprocessed;
    unprocessed_ptr = unprocessed_ptr.offset(1);
    unprocessed_ptr;
    parse_succeeded = parse_utf16_hex(unprocessed_ptr, &mut cp);
    if parse_succeeded == 0 {
        return JSONFailure as libc::c_int;
    }
    if cp < 0x80 as libc::c_int as libc::c_uint {
        *processed_ptr.offset(0 as libc::c_int as isize) = cp as libc::c_char;
    } else if cp < 0x800 as libc::c_int as libc::c_uint {
        *processed_ptr
            .offset(
                0 as libc::c_int as isize,
            ) = (cp >> 6 as libc::c_int & 0x1f as libc::c_int as libc::c_uint
            | 0xc0 as libc::c_int as libc::c_uint) as libc::c_char;
        *processed_ptr
            .offset(
                1 as libc::c_int as isize,
            ) = (cp & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        processed_ptr = processed_ptr.offset(1 as libc::c_int as isize);
    } else if cp < 0xd800 as libc::c_int as libc::c_uint
        || cp > 0xdfff as libc::c_int as libc::c_uint
    {
        *processed_ptr
            .offset(
                0 as libc::c_int as isize,
            ) = (cp >> 12 as libc::c_int & 0xf as libc::c_int as libc::c_uint
            | 0xe0 as libc::c_int as libc::c_uint) as libc::c_char;
        *processed_ptr
            .offset(
                1 as libc::c_int as isize,
            ) = (cp >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        *processed_ptr
            .offset(
                2 as libc::c_int as isize,
            ) = (cp & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        processed_ptr = processed_ptr.offset(2 as libc::c_int as isize);
    } else if cp >= 0xd800 as libc::c_int as libc::c_uint
        && cp <= 0xdbff as libc::c_int as libc::c_uint
    {
        lead = cp;
        unprocessed_ptr = unprocessed_ptr.offset(4 as libc::c_int as isize);
        let fresh4 = unprocessed_ptr;
        unprocessed_ptr = unprocessed_ptr.offset(1);
        if *fresh4 as libc::c_int != '\\' as i32
            || {
                let fresh5 = unprocessed_ptr;
                unprocessed_ptr = unprocessed_ptr.offset(1);
                *fresh5 as libc::c_int != 'u' as i32
            }
        {
            return JSONFailure as libc::c_int;
        }
        parse_succeeded = parse_utf16_hex(unprocessed_ptr, &mut trail);
        if parse_succeeded == 0 || trail < 0xdc00 as libc::c_int as libc::c_uint
            || trail > 0xdfff as libc::c_int as libc::c_uint
        {
            return JSONFailure as libc::c_int;
        }
        cp = ((lead.wrapping_sub(0xd800 as libc::c_int as libc::c_uint)
            & 0x3ff as libc::c_int as libc::c_uint) << 10 as libc::c_int
            | trail.wrapping_sub(0xdc00 as libc::c_int as libc::c_uint)
                & 0x3ff as libc::c_int as libc::c_uint)
            .wrapping_add(0x10000 as libc::c_int as libc::c_uint);
        *processed_ptr
            .offset(
                0 as libc::c_int as isize,
            ) = (cp >> 18 as libc::c_int & 0x7 as libc::c_int as libc::c_uint
            | 0xf0 as libc::c_int as libc::c_uint) as libc::c_char;
        *processed_ptr
            .offset(
                1 as libc::c_int as isize,
            ) = (cp >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        *processed_ptr
            .offset(
                2 as libc::c_int as isize,
            ) = (cp >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        *processed_ptr
            .offset(
                3 as libc::c_int as isize,
            ) = (cp & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        processed_ptr = processed_ptr.offset(3 as libc::c_int as isize);
    } else {
        return JSONFailure as libc::c_int
    }
    unprocessed_ptr = unprocessed_ptr.offset(3 as libc::c_int as isize);
    *processed = processed_ptr;
    *unprocessed = unprocessed_ptr;
    return JSONSuccess as libc::c_int;
}
unsafe extern "C" fn process_string(
    mut input: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut input_ptr: *const libc::c_char = input;
    let mut initial_size: size_t = len
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong);
    let mut final_size: size_t = 0 as libc::c_int as size_t;
    let mut output: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut output_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resized_output: *mut libc::c_char = 0 as *mut libc::c_char;
    output = parson_malloc.unwrap()(initial_size) as *mut libc::c_char;
    if !output.is_null() {
        output_ptr = output;
        loop {
            if !(*input_ptr as libc::c_int != '\0' as i32
                && (input_ptr.offset_from(input) as libc::c_long as size_t) < len)
            {
                current_block = 15125582407903384992;
                break;
            }
            if *input_ptr as libc::c_int == '\\' as i32 {
                input_ptr = input_ptr.offset(1);
                input_ptr;
                match *input_ptr as libc::c_int {
                    34 => {
                        *output_ptr = '"' as i32 as libc::c_char;
                    }
                    92 => {
                        *output_ptr = '\\' as i32 as libc::c_char;
                    }
                    47 => {
                        *output_ptr = '/' as i32 as libc::c_char;
                    }
                    98 => {
                        *output_ptr = '\u{8}' as i32 as libc::c_char;
                    }
                    102 => {
                        *output_ptr = '\u{c}' as i32 as libc::c_char;
                    }
                    110 => {
                        *output_ptr = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        *output_ptr = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        *output_ptr = '\t' as i32 as libc::c_char;
                    }
                    117 => {
                        if parse_utf16(&mut input_ptr, &mut output_ptr)
                            == JSONFailure as libc::c_int
                        {
                            current_block = 5651443946696651954;
                            break;
                        }
                    }
                    _ => {
                        current_block = 5651443946696651954;
                        break;
                    }
                }
            } else {
                if (*input_ptr as libc::c_uchar as libc::c_int) < 0x20 as libc::c_int {
                    current_block = 5651443946696651954;
                    break;
                }
                *output_ptr = *input_ptr;
            }
            output_ptr = output_ptr.offset(1);
            output_ptr;
            input_ptr = input_ptr.offset(1);
            input_ptr;
        }
        match current_block {
            5651443946696651954 => {}
            _ => {
                *output_ptr = '\0' as i32 as libc::c_char;
                final_size = (output_ptr.offset_from(output) as libc::c_long as size_t)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                resized_output = parson_malloc.unwrap()(final_size) as *mut libc::c_char;
                if !resized_output.is_null() {
                    memcpy(
                        resized_output as *mut libc::c_void,
                        output as *const libc::c_void,
                        final_size,
                    );
                    parson_free.unwrap()(output as *mut libc::c_void);
                    return resized_output;
                }
            }
        }
    }
    parson_free.unwrap()(output as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn get_quoted_string(
    mut string: *mut *const libc::c_char,
) -> *mut libc::c_char {
    let mut string_start: *const libc::c_char = *string;
    let mut string_len: size_t = 0 as libc::c_int as size_t;
    let mut status: JSON_Status = skip_quotes(string);
    if status != JSONSuccess as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    string_len = ((*string).offset_from(string_start) as libc::c_long
        - 2 as libc::c_int as libc::c_long) as size_t;
    return process_string(string_start.offset(1 as libc::c_int as isize), string_len);
}
unsafe extern "C" fn parse_value(
    mut string: *mut *const libc::c_char,
    mut nesting: size_t,
) -> *mut JSON_Value {
    if nesting > 2048 as libc::c_int as libc::c_ulong {
        return 0 as *mut JSON_Value;
    }
    while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *string = (*string).offset(1);
        *string;
    }
    match **string as libc::c_int {
        123 => {
            return parse_object_value(
                string,
                nesting.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        91 => {
            return parse_array_value(
                string,
                nesting.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        34 => return parse_string_value(string),
        102 | 116 => return parse_boolean_value(string),
        45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            return parse_number_value(string);
        }
        110 => return parse_null_value(string),
        _ => return 0 as *mut JSON_Value,
    };
}
unsafe extern "C" fn parse_object_value(
    mut string: *mut *const libc::c_char,
    mut nesting: size_t,
) -> *mut JSON_Value {
    let mut output_value: *mut JSON_Value = json_value_init_object();
    let mut new_value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut output_object: *mut JSON_Object = json_value_get_object(output_value);
    let mut new_key: *mut libc::c_char = 0 as *mut libc::c_char;
    if output_value.is_null() || **string as libc::c_int != '{' as i32 {
        return 0 as *mut JSON_Value;
    }
    *string = (*string).offset(1);
    *string;
    while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *string = (*string).offset(1);
        *string;
    }
    if **string as libc::c_int == '}' as i32 {
        *string = (*string).offset(1);
        *string;
        return output_value;
    }
    while **string as libc::c_int != '\0' as i32 {
        new_key = get_quoted_string(string);
        if new_key.is_null() {
            json_value_free(output_value);
            return 0 as *mut JSON_Value;
        }
        while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *string = (*string).offset(1);
            *string;
        }
        if **string as libc::c_int != ':' as i32 {
            parson_free.unwrap()(new_key as *mut libc::c_void);
            json_value_free(output_value);
            return 0 as *mut JSON_Value;
        }
        *string = (*string).offset(1);
        *string;
        new_value = parse_value(string, nesting);
        if new_value.is_null() {
            parson_free.unwrap()(new_key as *mut libc::c_void);
            json_value_free(output_value);
            return 0 as *mut JSON_Value;
        }
        if json_object_add(output_object, new_key, new_value)
            == JSONFailure as libc::c_int
        {
            parson_free.unwrap()(new_key as *mut libc::c_void);
            json_value_free(new_value);
            json_value_free(output_value);
            return 0 as *mut JSON_Value;
        }
        parson_free.unwrap()(new_key as *mut libc::c_void);
        while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *string = (*string).offset(1);
            *string;
        }
        if **string as libc::c_int != ',' as i32 {
            break;
        }
        *string = (*string).offset(1);
        *string;
        while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *string = (*string).offset(1);
            *string;
        }
    }
    while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *string = (*string).offset(1);
        *string;
    }
    if **string as libc::c_int != '}' as i32
        || json_object_resize(output_object, json_object_get_count(output_object))
            == JSONFailure as libc::c_int
    {
        json_value_free(output_value);
        return 0 as *mut JSON_Value;
    }
    *string = (*string).offset(1);
    *string;
    return output_value;
}
unsafe extern "C" fn parse_array_value(
    mut string: *mut *const libc::c_char,
    mut nesting: size_t,
) -> *mut JSON_Value {
    let mut output_value: *mut JSON_Value = json_value_init_array();
    let mut new_array_value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut output_array: *mut JSON_Array = json_value_get_array(output_value);
    if output_value.is_null() || **string as libc::c_int != '[' as i32 {
        return 0 as *mut JSON_Value;
    }
    *string = (*string).offset(1);
    *string;
    while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *string = (*string).offset(1);
        *string;
    }
    if **string as libc::c_int == ']' as i32 {
        *string = (*string).offset(1);
        *string;
        return output_value;
    }
    while **string as libc::c_int != '\0' as i32 {
        new_array_value = parse_value(string, nesting);
        if new_array_value.is_null() {
            json_value_free(output_value);
            return 0 as *mut JSON_Value;
        }
        if json_array_add(output_array, new_array_value) == JSONFailure as libc::c_int {
            json_value_free(new_array_value);
            json_value_free(output_value);
            return 0 as *mut JSON_Value;
        }
        while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *string = (*string).offset(1);
            *string;
        }
        if **string as libc::c_int != ',' as i32 {
            break;
        }
        *string = (*string).offset(1);
        *string;
        while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            *string = (*string).offset(1);
            *string;
        }
    }
    while *(*__ctype_b_loc()).offset(**string as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *string = (*string).offset(1);
        *string;
    }
    if **string as libc::c_int != ']' as i32
        || json_array_resize(output_array, json_array_get_count(output_array))
            == JSONFailure as libc::c_int
    {
        json_value_free(output_value);
        return 0 as *mut JSON_Value;
    }
    *string = (*string).offset(1);
    *string;
    return output_value;
}
unsafe extern "C" fn parse_string_value(
    mut string: *mut *const libc::c_char,
) -> *mut JSON_Value {
    let mut value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut new_string: *mut libc::c_char = get_quoted_string(string);
    if new_string.is_null() {
        return 0 as *mut JSON_Value;
    }
    value = json_value_init_string_no_copy(new_string);
    if value.is_null() {
        parson_free.unwrap()(new_string as *mut libc::c_void);
        return 0 as *mut JSON_Value;
    }
    return value;
}
unsafe extern "C" fn parse_boolean_value(
    mut string: *mut *const libc::c_char,
) -> *mut JSON_Value {
    let mut true_token_size: size_t = (::std::mem::size_of::<[libc::c_char; 5]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut false_token_size: size_t = (::std::mem::size_of::<[libc::c_char; 6]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if strncmp(b"true\0" as *const u8 as *const libc::c_char, *string, true_token_size)
        == 0 as libc::c_int
    {
        *string = (*string).offset(true_token_size as isize);
        return json_value_init_boolean(1 as libc::c_int);
    } else if strncmp(
        b"false\0" as *const u8 as *const libc::c_char,
        *string,
        false_token_size,
    ) == 0 as libc::c_int
    {
        *string = (*string).offset(false_token_size as isize);
        return json_value_init_boolean(0 as libc::c_int);
    }
    return 0 as *mut JSON_Value;
}
unsafe extern "C" fn parse_number_value(
    mut string: *mut *const libc::c_char,
) -> *mut JSON_Value {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut number: libc::c_double = 0 as libc::c_int as libc::c_double;
    *__errno_location() = 0 as libc::c_int;
    number = strtod(*string, &mut end);
    if *__errno_location() != 0
        || is_decimal(*string, end.offset_from(*string) as libc::c_long as size_t) == 0
    {
        return 0 as *mut JSON_Value;
    }
    *string = end;
    return json_value_init_number(number);
}
unsafe extern "C" fn parse_null_value(
    mut string: *mut *const libc::c_char,
) -> *mut JSON_Value {
    let mut token_size: size_t = (::std::mem::size_of::<[libc::c_char; 5]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if strncmp(b"null\0" as *const u8 as *const libc::c_char, *string, token_size)
        == 0 as libc::c_int
    {
        *string = (*string).offset(token_size as isize);
        return json_value_init_null();
    }
    return 0 as *mut JSON_Value;
}
unsafe extern "C" fn json_serialize_to_buffer_r(
    mut value: *const JSON_Value,
    mut buf: *mut libc::c_char,
    mut level: libc::c_int,
    mut is_pretty: libc::c_int,
    mut num_buf: *mut libc::c_char,
) -> libc::c_int {
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut string: *const libc::c_char = 0 as *const libc::c_char;
    let mut temp_value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut array: *mut JSON_Array = 0 as *mut JSON_Array;
    let mut object: *mut JSON_Object = 0 as *mut JSON_Object;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut num: libc::c_double = 0.0f64;
    let mut written: libc::c_int = -(1 as libc::c_int);
    let mut written_total: libc::c_int = 0 as libc::c_int;
    match json_value_get_type(value) {
        5 => {
            array = json_value_get_array(value);
            count = json_array_get_count(array);
            written = append_string(buf, b"[\0" as *const u8 as *const libc::c_char);
            if written < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !buf.is_null() {
                buf = buf.offset(written as isize);
            }
            written_total += written;
            if count > 0 as libc::c_int as libc::c_ulong && is_pretty != 0 {
                written = append_string(
                    buf,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            i = 0 as libc::c_int as size_t;
            while i < count {
                if is_pretty != 0 {
                    written = append_indent(buf, level + 1 as libc::c_int);
                    if written < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    if !buf.is_null() {
                        buf = buf.offset(written as isize);
                    }
                    written_total += written;
                }
                temp_value = json_array_get_value(array, i);
                written = json_serialize_to_buffer_r(
                    temp_value,
                    buf,
                    level + 1 as libc::c_int,
                    is_pretty,
                    num_buf,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
                if i < count.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    written = append_string(
                        buf,
                        b",\0" as *const u8 as *const libc::c_char,
                    );
                    if written < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    if !buf.is_null() {
                        buf = buf.offset(written as isize);
                    }
                    written_total += written;
                }
                if is_pretty != 0 {
                    written = append_string(
                        buf,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                    if written < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    if !buf.is_null() {
                        buf = buf.offset(written as isize);
                    }
                    written_total += written;
                }
                i = i.wrapping_add(1);
                i;
            }
            if count > 0 as libc::c_int as libc::c_ulong && is_pretty != 0 {
                written = append_indent(buf, level);
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            written = append_string(buf, b"]\0" as *const u8 as *const libc::c_char);
            if written < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !buf.is_null() {
                buf = buf.offset(written as isize);
            }
            written_total += written;
            return written_total;
        }
        4 => {
            object = json_value_get_object(value);
            count = json_object_get_count(object);
            written = append_string(buf, b"{\0" as *const u8 as *const libc::c_char);
            if written < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !buf.is_null() {
                buf = buf.offset(written as isize);
            }
            written_total += written;
            if count > 0 as libc::c_int as libc::c_ulong && is_pretty != 0 {
                written = append_string(
                    buf,
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            i = 0 as libc::c_int as size_t;
            while i < count {
                key = json_object_get_name(object, i);
                if key.is_null() {
                    return -(1 as libc::c_int);
                }
                if is_pretty != 0 {
                    written = append_indent(buf, level + 1 as libc::c_int);
                    if written < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    if !buf.is_null() {
                        buf = buf.offset(written as isize);
                    }
                    written_total += written;
                }
                written = json_serialize_string(key, buf);
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
                written = append_string(buf, b":\0" as *const u8 as *const libc::c_char);
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
                if is_pretty != 0 {
                    written = append_string(
                        buf,
                        b" \0" as *const u8 as *const libc::c_char,
                    );
                    if written < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    if !buf.is_null() {
                        buf = buf.offset(written as isize);
                    }
                    written_total += written;
                }
                temp_value = json_object_get_value(object, key);
                written = json_serialize_to_buffer_r(
                    temp_value,
                    buf,
                    level + 1 as libc::c_int,
                    is_pretty,
                    num_buf,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
                if i < count.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                    written = append_string(
                        buf,
                        b",\0" as *const u8 as *const libc::c_char,
                    );
                    if written < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    if !buf.is_null() {
                        buf = buf.offset(written as isize);
                    }
                    written_total += written;
                }
                if is_pretty != 0 {
                    written = append_string(
                        buf,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                    if written < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    if !buf.is_null() {
                        buf = buf.offset(written as isize);
                    }
                    written_total += written;
                }
                i = i.wrapping_add(1);
                i;
            }
            if count > 0 as libc::c_int as libc::c_ulong && is_pretty != 0 {
                written = append_indent(buf, level);
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            written = append_string(buf, b"}\0" as *const u8 as *const libc::c_char);
            if written < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !buf.is_null() {
                buf = buf.offset(written as isize);
            }
            written_total += written;
            return written_total;
        }
        2 => {
            string = json_value_get_string(value);
            if string.is_null() {
                return -(1 as libc::c_int);
            }
            written = json_serialize_string(string, buf);
            if written < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !buf.is_null() {
                buf = buf.offset(written as isize);
            }
            written_total += written;
            return written_total;
        }
        6 => {
            if json_value_get_boolean(value) != 0 {
                written = append_string(
                    buf,
                    b"true\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            } else {
                written = append_string(
                    buf,
                    b"false\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            return written_total;
        }
        3 => {
            num = json_value_get_number(value);
            if !buf.is_null() {
                num_buf = buf;
            }
            written = sprintf(
                num_buf,
                b"%1.17g\0" as *const u8 as *const libc::c_char,
                num,
            );
            if written < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !buf.is_null() {
                buf = buf.offset(written as isize);
            }
            written_total += written;
            return written_total;
        }
        1 => {
            written = append_string(buf, b"null\0" as *const u8 as *const libc::c_char);
            if written < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if !buf.is_null() {
                buf = buf.offset(written as isize);
            }
            written_total += written;
            return written_total;
        }
        -1 => return -(1 as libc::c_int),
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn json_serialize_string(
    mut string: *const libc::c_char,
    mut buf: *mut libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = strlen(string);
    let mut c: libc::c_char = '\0' as i32 as libc::c_char;
    let mut written: libc::c_int = -(1 as libc::c_int);
    let mut written_total: libc::c_int = 0 as libc::c_int;
    written = append_string(buf, b"\"\0" as *const u8 as *const libc::c_char);
    if written < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        buf = buf.offset(written as isize);
    }
    written_total += written;
    i = 0 as libc::c_int as size_t;
    while i < len {
        c = *string.offset(i as isize);
        match c as libc::c_int {
            34 => {
                written = append_string(
                    buf,
                    b"\\\"\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            92 => {
                written = append_string(
                    buf,
                    b"\\\\\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            47 => {
                written = append_string(
                    buf,
                    b"\\/\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            8 => {
                written = append_string(
                    buf,
                    b"\\b\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            12 => {
                written = append_string(
                    buf,
                    b"\\f\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            10 => {
                written = append_string(
                    buf,
                    b"\\n\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            13 => {
                written = append_string(
                    buf,
                    b"\\r\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            9 => {
                written = append_string(
                    buf,
                    b"\\t\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            0 => {
                written = append_string(
                    buf,
                    b"\\u0000\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            1 => {
                written = append_string(
                    buf,
                    b"\\u0001\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            2 => {
                written = append_string(
                    buf,
                    b"\\u0002\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            3 => {
                written = append_string(
                    buf,
                    b"\\u0003\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            4 => {
                written = append_string(
                    buf,
                    b"\\u0004\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            5 => {
                written = append_string(
                    buf,
                    b"\\u0005\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            6 => {
                written = append_string(
                    buf,
                    b"\\u0006\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            7 => {
                written = append_string(
                    buf,
                    b"\\u0007\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            11 => {
                written = append_string(
                    buf,
                    b"\\u000b\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            14 => {
                written = append_string(
                    buf,
                    b"\\u000e\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            15 => {
                written = append_string(
                    buf,
                    b"\\u000f\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            16 => {
                written = append_string(
                    buf,
                    b"\\u0010\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            17 => {
                written = append_string(
                    buf,
                    b"\\u0011\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            18 => {
                written = append_string(
                    buf,
                    b"\\u0012\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            19 => {
                written = append_string(
                    buf,
                    b"\\u0013\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            20 => {
                written = append_string(
                    buf,
                    b"\\u0014\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            21 => {
                written = append_string(
                    buf,
                    b"\\u0015\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            22 => {
                written = append_string(
                    buf,
                    b"\\u0016\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            23 => {
                written = append_string(
                    buf,
                    b"\\u0017\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            24 => {
                written = append_string(
                    buf,
                    b"\\u0018\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            25 => {
                written = append_string(
                    buf,
                    b"\\u0019\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            26 => {
                written = append_string(
                    buf,
                    b"\\u001a\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            27 => {
                written = append_string(
                    buf,
                    b"\\u001b\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            28 => {
                written = append_string(
                    buf,
                    b"\\u001c\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            29 => {
                written = append_string(
                    buf,
                    b"\\u001d\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            30 => {
                written = append_string(
                    buf,
                    b"\\u001e\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            31 => {
                written = append_string(
                    buf,
                    b"\\u001f\0" as *const u8 as *const libc::c_char,
                );
                if written < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if !buf.is_null() {
                    buf = buf.offset(written as isize);
                }
                written_total += written;
            }
            _ => {
                if !buf.is_null() {
                    *buf.offset(0 as libc::c_int as isize) = c;
                    buf = buf.offset(1 as libc::c_int as isize);
                }
                written_total += 1 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    written = append_string(buf, b"\"\0" as *const u8 as *const libc::c_char);
    if written < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !buf.is_null() {
        buf = buf.offset(written as isize);
    }
    written_total += written;
    return written_total;
}
unsafe extern "C" fn append_indent(
    mut buf: *mut libc::c_char,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut written: libc::c_int = -(1 as libc::c_int);
    let mut written_total: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < level {
        written = append_string(buf, b"    \0" as *const u8 as *const libc::c_char);
        if written < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if !buf.is_null() {
            buf = buf.offset(written as isize);
        }
        written_total += written;
        i += 1;
        i;
    }
    return written_total;
}
unsafe extern "C" fn append_string(
    mut buf: *mut libc::c_char,
    mut string: *const libc::c_char,
) -> libc::c_int {
    if buf.is_null() {
        return strlen(string) as libc::c_int;
    }
    return sprintf(buf, b"%s\0" as *const u8 as *const libc::c_char, string);
}
pub unsafe extern "C" fn json_parse_file(
    mut filename: *const libc::c_char,
) -> *mut JSON_Value {
    let mut file_contents: *mut libc::c_char = read_file(filename);
    let mut output_value: *mut JSON_Value = 0 as *mut JSON_Value;
    if file_contents.is_null() {
        return 0 as *mut JSON_Value;
    }
    output_value = json_parse_string(file_contents);
    parson_free.unwrap()(file_contents as *mut libc::c_void);
    return output_value;
}
pub unsafe extern "C" fn json_parse_file_with_comments(
    mut filename: *const libc::c_char,
) -> *mut JSON_Value {
    let mut file_contents: *mut libc::c_char = read_file(filename);
    let mut output_value: *mut JSON_Value = 0 as *mut JSON_Value;
    if file_contents.is_null() {
        return 0 as *mut JSON_Value;
    }
    output_value = json_parse_string_with_comments(file_contents);
    parson_free.unwrap()(file_contents as *mut libc::c_void);
    return output_value;
}
pub unsafe extern "C" fn json_parse_string(
    mut string: *const libc::c_char,
) -> *mut JSON_Value {
    if string.is_null() {
        return 0 as *mut JSON_Value;
    }
    if *string.offset(0 as libc::c_int as isize) as libc::c_int == -17i32
        && *string.offset(1 as libc::c_int as isize) as libc::c_int == -69i32
        && *string.offset(2 as libc::c_int as isize) as libc::c_int == -65i32
    {
        string = string.offset(3 as libc::c_int as isize);
    }
    return parse_value(
        &mut string as *mut *const libc::c_char,
        0 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn json_parse_string_with_comments(
    mut string: *const libc::c_char,
) -> *mut JSON_Value {
    let mut result: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut string_mutable_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string_mutable_copy_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    string_mutable_copy = parson_strdup(string);
    if string_mutable_copy.is_null() {
        return 0 as *mut JSON_Value;
    }
    remove_comments(
        string_mutable_copy,
        b"/*\0" as *const u8 as *const libc::c_char,
        b"*/\0" as *const u8 as *const libc::c_char,
    );
    remove_comments(
        string_mutable_copy,
        b"//\0" as *const u8 as *const libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    string_mutable_copy_ptr = string_mutable_copy;
    result = parse_value(
        &mut string_mutable_copy_ptr as *mut *mut libc::c_char
            as *mut *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    parson_free.unwrap()(string_mutable_copy as *mut libc::c_void);
    return result;
}
pub unsafe extern "C" fn json_object_get_value(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *mut JSON_Value {
    if object.is_null() || name.is_null() {
        return 0 as *mut JSON_Value;
    }
    return json_object_nget_value(object, name, strlen(name));
}
pub unsafe extern "C" fn json_object_get_string(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    return json_value_get_string(json_object_get_value(object, name));
}
pub unsafe extern "C" fn json_object_get_number(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> libc::c_double {
    return json_value_get_number(json_object_get_value(object, name));
}
pub unsafe extern "C" fn json_object_get_object(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *mut JSON_Object {
    return json_value_get_object(json_object_get_value(object, name));
}
pub unsafe extern "C" fn json_object_get_array(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *mut JSON_Array {
    return json_value_get_array(json_object_get_value(object, name));
}
pub unsafe extern "C" fn json_object_get_boolean(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return json_value_get_boolean(json_object_get_value(object, name));
}
pub unsafe extern "C" fn json_object_dotget_value(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *mut JSON_Value {
    let mut dot_position: *const libc::c_char = strchr(name, '.' as i32);
    if dot_position.is_null() {
        return json_object_get_value(object, name);
    }
    object = json_value_get_object(
        json_object_nget_value(
            object,
            name,
            dot_position.offset_from(name) as libc::c_long as size_t,
        ),
    );
    return json_object_dotget_value(
        object,
        dot_position.offset(1 as libc::c_int as isize),
    );
}
pub unsafe extern "C" fn json_object_dotget_string(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    return json_value_get_string(json_object_dotget_value(object, name));
}
pub unsafe extern "C" fn json_object_dotget_number(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> libc::c_double {
    return json_value_get_number(json_object_dotget_value(object, name));
}
pub unsafe extern "C" fn json_object_dotget_object(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *mut JSON_Object {
    return json_value_get_object(json_object_dotget_value(object, name));
}
pub unsafe extern "C" fn json_object_dotget_array(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> *mut JSON_Array {
    return json_value_get_array(json_object_dotget_value(object, name));
}
pub unsafe extern "C" fn json_object_dotget_boolean(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return json_value_get_boolean(json_object_dotget_value(object, name));
}
pub unsafe extern "C" fn json_object_get_count(
    mut object: *const JSON_Object,
) -> size_t {
    return if !object.is_null() {
        (*object).count
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
pub unsafe extern "C" fn json_object_get_name(
    mut object: *const JSON_Object,
    mut index: size_t,
) -> *const libc::c_char {
    if object.is_null() || index >= json_object_get_count(object) {
        return 0 as *const libc::c_char;
    }
    return *((*object).names).offset(index as isize);
}
pub unsafe extern "C" fn json_object_get_value_at(
    mut object: *const JSON_Object,
    mut index: size_t,
) -> *mut JSON_Value {
    if object.is_null() || index >= json_object_get_count(object) {
        return 0 as *mut JSON_Value;
    }
    return *((*object).values).offset(index as isize);
}
pub unsafe extern "C" fn json_object_get_wrapping_value(
    mut object: *const JSON_Object,
) -> *mut JSON_Value {
    return (*object).wrapping_value;
}
pub unsafe extern "C" fn json_object_has_value(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return (json_object_get_value(object, name)
        != 0 as *mut libc::c_void as *mut JSON_Value) as libc::c_int;
}
pub unsafe extern "C" fn json_object_has_value_of_type(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
    mut type_0: JSON_Value_Type,
) -> libc::c_int {
    let mut val: *mut JSON_Value = json_object_get_value(object, name);
    return (!val.is_null() && json_value_get_type(val) == type_0) as libc::c_int;
}
pub unsafe extern "C" fn json_object_dothas_value(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return (json_object_dotget_value(object, name)
        != 0 as *mut libc::c_void as *mut JSON_Value) as libc::c_int;
}
pub unsafe extern "C" fn json_object_dothas_value_of_type(
    mut object: *const JSON_Object,
    mut name: *const libc::c_char,
    mut type_0: JSON_Value_Type,
) -> libc::c_int {
    let mut val: *mut JSON_Value = json_object_dotget_value(object, name);
    return (!val.is_null() && json_value_get_type(val) == type_0) as libc::c_int;
}
pub unsafe extern "C" fn json_array_get_value(
    mut array: *const JSON_Array,
    mut index: size_t,
) -> *mut JSON_Value {
    if array.is_null() || index >= json_array_get_count(array) {
        return 0 as *mut JSON_Value;
    }
    return *((*array).items).offset(index as isize);
}
pub unsafe extern "C" fn json_array_get_string(
    mut array: *const JSON_Array,
    mut index: size_t,
) -> *const libc::c_char {
    return json_value_get_string(json_array_get_value(array, index));
}
pub unsafe extern "C" fn json_array_get_number(
    mut array: *const JSON_Array,
    mut index: size_t,
) -> libc::c_double {
    return json_value_get_number(json_array_get_value(array, index));
}
pub unsafe extern "C" fn json_array_get_object(
    mut array: *const JSON_Array,
    mut index: size_t,
) -> *mut JSON_Object {
    return json_value_get_object(json_array_get_value(array, index));
}
pub unsafe extern "C" fn json_array_get_array(
    mut array: *const JSON_Array,
    mut index: size_t,
) -> *mut JSON_Array {
    return json_value_get_array(json_array_get_value(array, index));
}
pub unsafe extern "C" fn json_array_get_boolean(
    mut array: *const JSON_Array,
    mut index: size_t,
) -> libc::c_int {
    return json_value_get_boolean(json_array_get_value(array, index));
}
pub unsafe extern "C" fn json_array_get_count(mut array: *const JSON_Array) -> size_t {
    return if !array.is_null() {
        (*array).count
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
pub unsafe extern "C" fn json_array_get_wrapping_value(
    mut array: *const JSON_Array,
) -> *mut JSON_Value {
    return (*array).wrapping_value;
}
pub unsafe extern "C" fn json_value_get_type(
    mut value: *const JSON_Value,
) -> JSON_Value_Type {
    return if !value.is_null() { (*value).type_0 } else { JSONError as libc::c_int };
}
pub unsafe extern "C" fn json_value_get_object(
    mut value: *const JSON_Value,
) -> *mut JSON_Object {
    return if json_value_get_type(value) == JSONObject as libc::c_int {
        (*value).value.object
    } else {
        0 as *mut JSON_Object
    };
}
pub unsafe extern "C" fn json_value_get_array(
    mut value: *const JSON_Value,
) -> *mut JSON_Array {
    return if json_value_get_type(value) == JSONArray as libc::c_int {
        (*value).value.array
    } else {
        0 as *mut JSON_Array
    };
}
pub unsafe extern "C" fn json_value_get_string(
    mut value: *const JSON_Value,
) -> *const libc::c_char {
    return if json_value_get_type(value) == JSONString as libc::c_int {
        (*value).value.string
    } else {
        0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn json_value_get_number(
    mut value: *const JSON_Value,
) -> libc::c_double {
    return if json_value_get_type(value) == JSONNumber as libc::c_int {
        (*value).value.number
    } else {
        0 as libc::c_int as libc::c_double
    };
}
pub unsafe extern "C" fn json_value_get_boolean(
    mut value: *const JSON_Value,
) -> libc::c_int {
    return if json_value_get_type(value) == JSONBoolean as libc::c_int {
        (*value).value.boolean
    } else {
        -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn json_value_get_parent(
    mut value: *const JSON_Value,
) -> *mut JSON_Value {
    return if !value.is_null() { (*value).parent } else { 0 as *mut JSON_Value };
}
pub unsafe extern "C" fn json_value_free(mut value: *mut JSON_Value) {
    match json_value_get_type(value) {
        4 => {
            json_object_free((*value).value.object);
        }
        2 => {
            parson_free.unwrap()((*value).value.string as *mut libc::c_void);
        }
        5 => {
            json_array_free((*value).value.array);
        }
        _ => {}
    }
    parson_free.unwrap()(value as *mut libc::c_void);
}
pub unsafe extern "C" fn json_value_init_object() -> *mut JSON_Value {
    let mut new_value: *mut JSON_Value = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Value>() as libc::c_ulong)
        as *mut JSON_Value;
    if new_value.is_null() {
        return 0 as *mut JSON_Value;
    }
    (*new_value).parent = 0 as *mut JSON_Value;
    (*new_value).type_0 = JSONObject as libc::c_int;
    (*new_value).value.object = json_object_init(new_value);
    if ((*new_value).value.object).is_null() {
        parson_free.unwrap()(new_value as *mut libc::c_void);
        return 0 as *mut JSON_Value;
    }
    return new_value;
}
pub unsafe extern "C" fn json_value_init_array() -> *mut JSON_Value {
    let mut new_value: *mut JSON_Value = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Value>() as libc::c_ulong)
        as *mut JSON_Value;
    if new_value.is_null() {
        return 0 as *mut JSON_Value;
    }
    (*new_value).parent = 0 as *mut JSON_Value;
    (*new_value).type_0 = JSONArray as libc::c_int;
    (*new_value).value.array = json_array_init(new_value);
    if ((*new_value).value.array).is_null() {
        parson_free.unwrap()(new_value as *mut libc::c_void);
        return 0 as *mut JSON_Value;
    }
    return new_value;
}
pub unsafe extern "C" fn json_value_init_string(
    mut string: *const libc::c_char,
) -> *mut JSON_Value {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut string_len: size_t = 0 as libc::c_int as size_t;
    if string.is_null() {
        return 0 as *mut JSON_Value;
    }
    string_len = strlen(string);
    if is_valid_utf8(string, string_len) == 0 {
        return 0 as *mut JSON_Value;
    }
    copy = parson_strndup(string, string_len);
    if copy.is_null() {
        return 0 as *mut JSON_Value;
    }
    value = json_value_init_string_no_copy(copy);
    if value.is_null() {
        parson_free.unwrap()(copy as *mut libc::c_void);
    }
    return value;
}
pub unsafe extern "C" fn json_value_init_number(
    mut number: libc::c_double,
) -> *mut JSON_Value {
    let mut new_value: *mut JSON_Value = 0 as *mut JSON_Value;
    if number * 0.0f64 != 0.0f64 {
        return 0 as *mut JSON_Value;
    }
    new_value = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Value>() as libc::c_ulong)
        as *mut JSON_Value;
    if new_value.is_null() {
        return 0 as *mut JSON_Value;
    }
    (*new_value).parent = 0 as *mut JSON_Value;
    (*new_value).type_0 = JSONNumber as libc::c_int;
    (*new_value).value.number = number;
    return new_value;
}
pub unsafe extern "C" fn json_value_init_boolean(
    mut boolean: libc::c_int,
) -> *mut JSON_Value {
    let mut new_value: *mut JSON_Value = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Value>() as libc::c_ulong)
        as *mut JSON_Value;
    if new_value.is_null() {
        return 0 as *mut JSON_Value;
    }
    (*new_value).parent = 0 as *mut JSON_Value;
    (*new_value).type_0 = JSONBoolean as libc::c_int;
    (*new_value)
        .value
        .boolean = if boolean != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
    return new_value;
}
pub unsafe extern "C" fn json_value_init_null() -> *mut JSON_Value {
    let mut new_value: *mut JSON_Value = parson_malloc
        .unwrap()(::std::mem::size_of::<JSON_Value>() as libc::c_ulong)
        as *mut JSON_Value;
    if new_value.is_null() {
        return 0 as *mut JSON_Value;
    }
    (*new_value).parent = 0 as *mut JSON_Value;
    (*new_value).type_0 = JSONNull as libc::c_int;
    return new_value;
}
pub unsafe extern "C" fn json_value_deep_copy(
    mut value: *const JSON_Value,
) -> *mut JSON_Value {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut return_value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut temp_value_copy: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut temp_value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut temp_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut temp_key: *const libc::c_char = 0 as *const libc::c_char;
    let mut temp_string_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_array: *mut JSON_Array = 0 as *mut JSON_Array;
    let mut temp_array_copy: *mut JSON_Array = 0 as *mut JSON_Array;
    let mut temp_object: *mut JSON_Object = 0 as *mut JSON_Object;
    let mut temp_object_copy: *mut JSON_Object = 0 as *mut JSON_Object;
    match json_value_get_type(value) {
        5 => {
            temp_array = json_value_get_array(value);
            return_value = json_value_init_array();
            if return_value.is_null() {
                return 0 as *mut JSON_Value;
            }
            temp_array_copy = json_value_get_array(return_value);
            i = 0 as libc::c_int as size_t;
            while i < json_array_get_count(temp_array) {
                temp_value = json_array_get_value(temp_array, i);
                temp_value_copy = json_value_deep_copy(temp_value);
                if temp_value_copy.is_null() {
                    json_value_free(return_value);
                    return 0 as *mut JSON_Value;
                }
                if json_array_add(temp_array_copy, temp_value_copy)
                    == JSONFailure as libc::c_int
                {
                    json_value_free(return_value);
                    json_value_free(temp_value_copy);
                    return 0 as *mut JSON_Value;
                }
                i = i.wrapping_add(1);
                i;
            }
            return return_value;
        }
        4 => {
            temp_object = json_value_get_object(value);
            return_value = json_value_init_object();
            if return_value.is_null() {
                return 0 as *mut JSON_Value;
            }
            temp_object_copy = json_value_get_object(return_value);
            i = 0 as libc::c_int as size_t;
            while i < json_object_get_count(temp_object) {
                temp_key = json_object_get_name(temp_object, i);
                temp_value = json_object_get_value(temp_object, temp_key);
                temp_value_copy = json_value_deep_copy(temp_value);
                if temp_value_copy.is_null() {
                    json_value_free(return_value);
                    return 0 as *mut JSON_Value;
                }
                if json_object_add(temp_object_copy, temp_key, temp_value_copy)
                    == JSONFailure as libc::c_int
                {
                    json_value_free(return_value);
                    json_value_free(temp_value_copy);
                    return 0 as *mut JSON_Value;
                }
                i = i.wrapping_add(1);
                i;
            }
            return return_value;
        }
        6 => return json_value_init_boolean(json_value_get_boolean(value)),
        3 => return json_value_init_number(json_value_get_number(value)),
        2 => {
            temp_string = json_value_get_string(value);
            if temp_string.is_null() {
                return 0 as *mut JSON_Value;
            }
            temp_string_copy = parson_strdup(temp_string);
            if temp_string_copy.is_null() {
                return 0 as *mut JSON_Value;
            }
            return_value = json_value_init_string_no_copy(temp_string_copy);
            if return_value.is_null() {
                parson_free.unwrap()(temp_string_copy as *mut libc::c_void);
            }
            return return_value;
        }
        1 => return json_value_init_null(),
        -1 => return 0 as *mut JSON_Value,
        _ => return 0 as *mut JSON_Value,
    };
}
pub unsafe extern "C" fn json_serialization_size(
    mut value: *const JSON_Value,
) -> size_t {
    let mut num_buf: [libc::c_char; 1100] = [0; 1100];
    let mut res: libc::c_int = json_serialize_to_buffer_r(
        value,
        0 as *mut libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        num_buf.as_mut_ptr(),
    );
    return if res < 0 as libc::c_int {
        0 as libc::c_int as libc::c_ulong
    } else {
        (res + 1 as libc::c_int) as size_t
    };
}
pub unsafe extern "C" fn json_serialize_to_buffer(
    mut value: *const JSON_Value,
    mut buf: *mut libc::c_char,
    mut buf_size_in_bytes: size_t,
) -> JSON_Status {
    let mut written: libc::c_int = -(1 as libc::c_int);
    let mut needed_size_in_bytes: size_t = json_serialization_size(value);
    if needed_size_in_bytes == 0 as libc::c_int as libc::c_ulong
        || buf_size_in_bytes < needed_size_in_bytes
    {
        return JSONFailure as libc::c_int;
    }
    written = json_serialize_to_buffer_r(
        value,
        buf,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_char,
    );
    if written < 0 as libc::c_int {
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_serialize_to_file(
    mut value: *const JSON_Value,
    mut filename: *const libc::c_char,
) -> JSON_Status {
    let mut return_code: JSON_Status = JSONSuccess as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut serialized_string: *mut libc::c_char = json_serialize_to_string(value);
    if serialized_string.is_null() {
        return JSONFailure as libc::c_int;
    }
    fp = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        json_free_serialized_string(serialized_string);
        return JSONFailure as libc::c_int;
    }
    if fputs(serialized_string, fp) == -(1 as libc::c_int) {
        return_code = JSONFailure as libc::c_int;
    }
    if fclose(fp) == -(1 as libc::c_int) {
        return_code = JSONFailure as libc::c_int;
    }
    json_free_serialized_string(serialized_string);
    return return_code;
}
pub unsafe extern "C" fn json_serialize_to_string(
    mut value: *const JSON_Value,
) -> *mut libc::c_char {
    let mut serialization_result: JSON_Status = JSONFailure as libc::c_int;
    let mut buf_size_bytes: size_t = json_serialization_size(value);
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if buf_size_bytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    buf = parson_malloc.unwrap()(buf_size_bytes) as *mut libc::c_char;
    if buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    serialization_result = json_serialize_to_buffer(value, buf, buf_size_bytes);
    if serialization_result == JSONFailure as libc::c_int {
        json_free_serialized_string(buf);
        return 0 as *mut libc::c_char;
    }
    return buf;
}
pub unsafe extern "C" fn json_serialization_size_pretty(
    mut value: *const JSON_Value,
) -> size_t {
    let mut num_buf: [libc::c_char; 1100] = [0; 1100];
    let mut res: libc::c_int = json_serialize_to_buffer_r(
        value,
        0 as *mut libc::c_char,
        0 as libc::c_int,
        1 as libc::c_int,
        num_buf.as_mut_ptr(),
    );
    return if res < 0 as libc::c_int {
        0 as libc::c_int as libc::c_ulong
    } else {
        (res + 1 as libc::c_int) as size_t
    };
}
pub unsafe extern "C" fn json_serialize_to_buffer_pretty(
    mut value: *const JSON_Value,
    mut buf: *mut libc::c_char,
    mut buf_size_in_bytes: size_t,
) -> JSON_Status {
    let mut written: libc::c_int = -(1 as libc::c_int);
    let mut needed_size_in_bytes: size_t = json_serialization_size_pretty(value);
    if needed_size_in_bytes == 0 as libc::c_int as libc::c_ulong
        || buf_size_in_bytes < needed_size_in_bytes
    {
        return JSONFailure as libc::c_int;
    }
    written = json_serialize_to_buffer_r(
        value,
        buf,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as *mut libc::c_char,
    );
    if written < 0 as libc::c_int {
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_serialize_to_file_pretty(
    mut value: *const JSON_Value,
    mut filename: *const libc::c_char,
) -> JSON_Status {
    let mut return_code: JSON_Status = JSONSuccess as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut serialized_string: *mut libc::c_char = json_serialize_to_string_pretty(
        value,
    );
    if serialized_string.is_null() {
        return JSONFailure as libc::c_int;
    }
    fp = fopen(filename, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        json_free_serialized_string(serialized_string);
        return JSONFailure as libc::c_int;
    }
    if fputs(serialized_string, fp) == -(1 as libc::c_int) {
        return_code = JSONFailure as libc::c_int;
    }
    if fclose(fp) == -(1 as libc::c_int) {
        return_code = JSONFailure as libc::c_int;
    }
    json_free_serialized_string(serialized_string);
    return return_code;
}
pub unsafe extern "C" fn json_serialize_to_string_pretty(
    mut value: *const JSON_Value,
) -> *mut libc::c_char {
    let mut serialization_result: JSON_Status = JSONFailure as libc::c_int;
    let mut buf_size_bytes: size_t = json_serialization_size_pretty(value);
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if buf_size_bytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    buf = parson_malloc.unwrap()(buf_size_bytes) as *mut libc::c_char;
    if buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    serialization_result = json_serialize_to_buffer_pretty(value, buf, buf_size_bytes);
    if serialization_result == JSONFailure as libc::c_int {
        json_free_serialized_string(buf);
        return 0 as *mut libc::c_char;
    }
    return buf;
}
pub unsafe extern "C" fn json_free_serialized_string(mut string: *mut libc::c_char) {
    parson_free.unwrap()(string as *mut libc::c_void);
}
pub unsafe extern "C" fn json_array_remove(
    mut array: *mut JSON_Array,
    mut ix: size_t,
) -> JSON_Status {
    let mut to_move_bytes: size_t = 0 as libc::c_int as size_t;
    if array.is_null() || ix >= json_array_get_count(array) {
        return JSONFailure as libc::c_int;
    }
    json_value_free(json_array_get_value(array, ix));
    to_move_bytes = (json_array_get_count(array))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_sub(ix)
        .wrapping_mul(::std::mem::size_of::<*mut JSON_Value>() as libc::c_ulong);
    memmove(
        ((*array).items).offset(ix as isize) as *mut libc::c_void,
        ((*array).items).offset(ix as isize).offset(1 as libc::c_int as isize)
            as *const libc::c_void,
        to_move_bytes,
    );
    (*array)
        .count = ((*array).count as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_replace_value(
    mut array: *mut JSON_Array,
    mut ix: size_t,
    mut value: *mut JSON_Value,
) -> JSON_Status {
    if array.is_null() || value.is_null() || !((*value).parent).is_null()
        || ix >= json_array_get_count(array)
    {
        return JSONFailure as libc::c_int;
    }
    json_value_free(json_array_get_value(array, ix));
    (*value).parent = json_array_get_wrapping_value(array);
    let ref mut fresh6 = *((*array).items).offset(ix as isize);
    *fresh6 = value;
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_replace_string(
    mut array: *mut JSON_Array,
    mut i: size_t,
    mut string: *const libc::c_char,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_string(string);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_replace_value(array, i, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_replace_number(
    mut array: *mut JSON_Array,
    mut i: size_t,
    mut number: libc::c_double,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_number(number);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_replace_value(array, i, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_replace_boolean(
    mut array: *mut JSON_Array,
    mut i: size_t,
    mut boolean: libc::c_int,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_boolean(boolean);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_replace_value(array, i, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_replace_null(
    mut array: *mut JSON_Array,
    mut i: size_t,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_null();
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_replace_value(array, i, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_clear(mut array: *mut JSON_Array) -> JSON_Status {
    let mut i: size_t = 0 as libc::c_int as size_t;
    if array.is_null() {
        return JSONFailure as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < json_array_get_count(array) {
        json_value_free(json_array_get_value(array, i));
        i = i.wrapping_add(1);
        i;
    }
    (*array).count = 0 as libc::c_int as size_t;
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_append_value(
    mut array: *mut JSON_Array,
    mut value: *mut JSON_Value,
) -> JSON_Status {
    if array.is_null() || value.is_null() || !((*value).parent).is_null() {
        return JSONFailure as libc::c_int;
    }
    return json_array_add(array, value);
}
pub unsafe extern "C" fn json_array_append_string(
    mut array: *mut JSON_Array,
    mut string: *const libc::c_char,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_string(string);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_append_value(array, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_append_number(
    mut array: *mut JSON_Array,
    mut number: libc::c_double,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_number(number);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_append_value(array, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_append_boolean(
    mut array: *mut JSON_Array,
    mut boolean: libc::c_int,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_boolean(boolean);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_append_value(array, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_array_append_null(
    mut array: *mut JSON_Array,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_null();
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_array_append_value(array, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_object_set_value(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut value: *mut JSON_Value,
) -> JSON_Status {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut old_value: *mut JSON_Value = 0 as *mut JSON_Value;
    if object.is_null() || name.is_null() || value.is_null()
        || !((*value).parent).is_null()
    {
        return JSONFailure as libc::c_int;
    }
    old_value = json_object_get_value(object, name);
    if !old_value.is_null() {
        json_value_free(old_value);
        i = 0 as libc::c_int as size_t;
        while i < json_object_get_count(object) {
            if strcmp(*((*object).names).offset(i as isize), name) == 0 as libc::c_int {
                (*value).parent = json_object_get_wrapping_value(object);
                let ref mut fresh7 = *((*object).values).offset(i as isize);
                *fresh7 = value;
                return JSONSuccess as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return json_object_add(object, name, value);
}
pub unsafe extern "C" fn json_object_set_string(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut string: *const libc::c_char,
) -> JSON_Status {
    return json_object_set_value(object, name, json_value_init_string(string));
}
pub unsafe extern "C" fn json_object_set_number(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut number: libc::c_double,
) -> JSON_Status {
    return json_object_set_value(object, name, json_value_init_number(number));
}
pub unsafe extern "C" fn json_object_set_boolean(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut boolean: libc::c_int,
) -> JSON_Status {
    return json_object_set_value(object, name, json_value_init_boolean(boolean));
}
pub unsafe extern "C" fn json_object_set_null(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
) -> JSON_Status {
    return json_object_set_value(object, name, json_value_init_null());
}
pub unsafe extern "C" fn json_object_dotset_value(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut value: *mut JSON_Value,
) -> JSON_Status {
    let mut dot_pos: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_obj: *mut JSON_Object = 0 as *mut JSON_Object;
    let mut new_value: *mut JSON_Value = 0 as *mut JSON_Value;
    if object.is_null() || name.is_null() || value.is_null() {
        return JSONFailure as libc::c_int;
    }
    dot_pos = strchr(name, '.' as i32);
    if dot_pos.is_null() {
        return json_object_set_value(object, name, value)
    } else {
        current_name = parson_strndup(
            name,
            dot_pos.offset_from(name) as libc::c_long as size_t,
        );
        temp_obj = json_object_get_object(object, current_name);
        if temp_obj.is_null() {
            new_value = json_value_init_object();
            if new_value.is_null() {
                parson_free.unwrap()(current_name as *mut libc::c_void);
                return JSONFailure as libc::c_int;
            }
            if json_object_add(object, current_name, new_value)
                == JSONFailure as libc::c_int
            {
                json_value_free(new_value);
                parson_free.unwrap()(current_name as *mut libc::c_void);
                return JSONFailure as libc::c_int;
            }
            temp_obj = json_object_get_object(object, current_name);
        }
        parson_free.unwrap()(current_name as *mut libc::c_void);
        return json_object_dotset_value(
            temp_obj,
            dot_pos.offset(1 as libc::c_int as isize),
            value,
        );
    };
}
pub unsafe extern "C" fn json_object_dotset_string(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut string: *const libc::c_char,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_string(string);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_object_dotset_value(object, name, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_object_dotset_number(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut number: libc::c_double,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_number(number);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_object_dotset_value(object, name, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_object_dotset_boolean(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
    mut boolean: libc::c_int,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_boolean(boolean);
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_object_dotset_value(object, name, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_object_dotset_null(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
) -> JSON_Status {
    let mut value: *mut JSON_Value = json_value_init_null();
    if value.is_null() {
        return JSONFailure as libc::c_int;
    }
    if json_object_dotset_value(object, name, value) == JSONFailure as libc::c_int {
        json_value_free(value);
        return JSONFailure as libc::c_int;
    }
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_object_remove(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
) -> JSON_Status {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut last_item_index: size_t = 0 as libc::c_int as size_t;
    if object.is_null() || (json_object_get_value(object, name)).is_null() {
        return JSONFailure as libc::c_int;
    }
    last_item_index = (json_object_get_count(object))
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int as size_t;
    while i < json_object_get_count(object) {
        if strcmp(*((*object).names).offset(i as isize), name) == 0 as libc::c_int {
            parson_free
                .unwrap()(*((*object).names).offset(i as isize) as *mut libc::c_void);
            json_value_free(*((*object).values).offset(i as isize));
            if i != last_item_index {
                let ref mut fresh8 = *((*object).names).offset(i as isize);
                *fresh8 = *((*object).names).offset(last_item_index as isize);
                let ref mut fresh9 = *((*object).values).offset(i as isize);
                *fresh9 = *((*object).values).offset(last_item_index as isize);
            }
            (*object)
                .count = ((*object).count as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
            return JSONSuccess as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return JSONFailure as libc::c_int;
}
pub unsafe extern "C" fn json_object_dotremove(
    mut object: *mut JSON_Object,
    mut name: *const libc::c_char,
) -> JSON_Status {
    let mut dot_pos: *const libc::c_char = strchr(name, '.' as i32);
    let mut current_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_obj: *mut JSON_Object = 0 as *mut JSON_Object;
    if dot_pos.is_null() {
        return json_object_remove(object, name)
    } else {
        current_name = parson_strndup(
            name,
            dot_pos.offset_from(name) as libc::c_long as size_t,
        );
        temp_obj = json_object_get_object(object, current_name);
        parson_free.unwrap()(current_name as *mut libc::c_void);
        if temp_obj.is_null() {
            return JSONFailure as libc::c_int;
        }
        return json_object_dotremove(
            temp_obj,
            dot_pos.offset(1 as libc::c_int as isize),
        );
    };
}
pub unsafe extern "C" fn json_object_clear(mut object: *mut JSON_Object) -> JSON_Status {
    let mut i: size_t = 0 as libc::c_int as size_t;
    if object.is_null() {
        return JSONFailure as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < json_object_get_count(object) {
        parson_free.unwrap()(*((*object).names).offset(i as isize) as *mut libc::c_void);
        json_value_free(*((*object).values).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    (*object).count = 0 as libc::c_int as size_t;
    return JSONSuccess as libc::c_int;
}
pub unsafe extern "C" fn json_validate(
    mut schema: *const JSON_Value,
    mut value: *const JSON_Value,
) -> JSON_Status {
    let mut temp_schema_value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut temp_value: *mut JSON_Value = 0 as *mut JSON_Value;
    let mut schema_array: *mut JSON_Array = 0 as *mut JSON_Array;
    let mut value_array: *mut JSON_Array = 0 as *mut JSON_Array;
    let mut schema_object: *mut JSON_Object = 0 as *mut JSON_Object;
    let mut value_object: *mut JSON_Object = 0 as *mut JSON_Object;
    let mut schema_type: JSON_Value_Type = JSONError as libc::c_int;
    let mut value_type: JSON_Value_Type = JSONError as libc::c_int;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut count: size_t = 0 as libc::c_int as size_t;
    if schema.is_null() || value.is_null() {
        return JSONFailure as libc::c_int;
    }
    schema_type = json_value_get_type(schema);
    value_type = json_value_get_type(value);
    if schema_type != value_type && schema_type != JSONNull as libc::c_int {
        return JSONFailure as libc::c_int;
    }
    match schema_type {
        5 => {
            schema_array = json_value_get_array(schema);
            value_array = json_value_get_array(value);
            count = json_array_get_count(schema_array);
            if count == 0 as libc::c_int as libc::c_ulong {
                return JSONSuccess as libc::c_int;
            }
            temp_schema_value = json_array_get_value(
                schema_array,
                0 as libc::c_int as size_t,
            );
            i = 0 as libc::c_int as size_t;
            while i < json_array_get_count(value_array) {
                temp_value = json_array_get_value(value_array, i);
                if json_validate(temp_schema_value, temp_value)
                    == JSONFailure as libc::c_int
                {
                    return JSONFailure as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
            return JSONSuccess as libc::c_int;
        }
        4 => {
            schema_object = json_value_get_object(schema);
            value_object = json_value_get_object(value);
            count = json_object_get_count(schema_object);
            if count == 0 as libc::c_int as libc::c_ulong {
                return JSONSuccess as libc::c_int
            } else if json_object_get_count(value_object) < count {
                return JSONFailure as libc::c_int
            }
            i = 0 as libc::c_int as size_t;
            while i < count {
                key = json_object_get_name(schema_object, i);
                temp_schema_value = json_object_get_value(schema_object, key);
                temp_value = json_object_get_value(value_object, key);
                if temp_value.is_null() {
                    return JSONFailure as libc::c_int;
                }
                if json_validate(temp_schema_value, temp_value)
                    == JSONFailure as libc::c_int
                {
                    return JSONFailure as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
            return JSONSuccess as libc::c_int;
        }
        2 | 3 | 6 | 1 => return JSONSuccess as libc::c_int,
        -1 | _ => return JSONFailure as libc::c_int,
    };
}
pub unsafe extern "C" fn json_value_equals(
    mut a: *const JSON_Value,
    mut b: *const JSON_Value,
) -> libc::c_int {
    let mut a_object: *mut JSON_Object = 0 as *mut JSON_Object;
    let mut b_object: *mut JSON_Object = 0 as *mut JSON_Object;
    let mut a_array: *mut JSON_Array = 0 as *mut JSON_Array;
    let mut b_array: *mut JSON_Array = 0 as *mut JSON_Array;
    let mut a_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut b_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut a_count: size_t = 0 as libc::c_int as size_t;
    let mut b_count: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut a_type: JSON_Value_Type = 0;
    let mut b_type: JSON_Value_Type = 0;
    a_type = json_value_get_type(a);
    b_type = json_value_get_type(b);
    if a_type != b_type {
        return 0 as libc::c_int;
    }
    match a_type {
        5 => {
            a_array = json_value_get_array(a);
            b_array = json_value_get_array(b);
            a_count = json_array_get_count(a_array);
            b_count = json_array_get_count(b_array);
            if a_count != b_count {
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int as size_t;
            while i < a_count {
                if json_value_equals(
                    json_array_get_value(a_array, i),
                    json_array_get_value(b_array, i),
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
            return 1 as libc::c_int;
        }
        4 => {
            a_object = json_value_get_object(a);
            b_object = json_value_get_object(b);
            a_count = json_object_get_count(a_object);
            b_count = json_object_get_count(b_object);
            if a_count != b_count {
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int as size_t;
            while i < a_count {
                key = json_object_get_name(a_object, i);
                if json_value_equals(
                    json_object_get_value(a_object, key),
                    json_object_get_value(b_object, key),
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
            return 1 as libc::c_int;
        }
        2 => {
            a_string = json_value_get_string(a);
            b_string = json_value_get_string(b);
            if a_string.is_null() || b_string.is_null() {
                return 0 as libc::c_int;
            }
            return (strcmp(a_string, b_string) == 0 as libc::c_int) as libc::c_int;
        }
        6 => {
            return (json_value_get_boolean(a) == json_value_get_boolean(b))
                as libc::c_int;
        }
        3 => {
            return (fabs(json_value_get_number(a) - json_value_get_number(b))
                < 0.000001f64) as libc::c_int;
        }
        -1 => return 1 as libc::c_int,
        1 => return 1 as libc::c_int,
        _ => return 1 as libc::c_int,
    };
}
pub unsafe extern "C" fn json_type(mut value: *const JSON_Value) -> JSON_Value_Type {
    return json_value_get_type(value);
}
pub unsafe extern "C" fn json_object(mut value: *const JSON_Value) -> *mut JSON_Object {
    return json_value_get_object(value);
}
pub unsafe extern "C" fn json_array(mut value: *const JSON_Value) -> *mut JSON_Array {
    return json_value_get_array(value);
}
pub unsafe extern "C" fn json_string(
    mut value: *const JSON_Value,
) -> *const libc::c_char {
    return json_value_get_string(value);
}
pub unsafe extern "C" fn json_number(mut value: *const JSON_Value) -> libc::c_double {
    return json_value_get_number(value);
}
pub unsafe extern "C" fn json_boolean(mut value: *const JSON_Value) -> libc::c_int {
    return json_value_get_boolean(value);
}
pub unsafe extern "C" fn json_set_allocation_functions(
    mut malloc_fun: JSON_Malloc_Function,
    mut free_fun: JSON_Free_Function,
) {
    parson_malloc = malloc_fun;
    parson_free = free_fun;
}
