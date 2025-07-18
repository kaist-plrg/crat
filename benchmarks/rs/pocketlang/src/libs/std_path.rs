use ::libc;
extern "C" {
    pub type PKVM;
    pub type PkHandle;
    pub type __dirstream;
    fn pkAddSearchPath(vm: *mut PKVM, path: *const libc::c_char);
    fn pkRealloc(
        vm: *mut PKVM,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn pkReleaseHandle(vm: *mut PKVM, handle: *mut PkHandle);
    fn pkNewModule(vm: *mut PKVM, name: *const libc::c_char) -> *mut PkHandle;
    fn pkRegisterModule(vm: *mut PKVM, module: *mut PkHandle);
    fn pkModuleAddFunction(
        vm: *mut PKVM,
        module: *mut PkHandle,
        name: *const libc::c_char,
        fptr: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
    fn pkSetRuntimeError(vm: *mut PKVM, message: *const libc::c_char);
    fn pkSetRuntimeErrorFmt(vm: *mut PKVM, fmt: *const libc::c_char, _: ...);
    fn pkGetArgc(vm: *const PKVM) -> libc::c_int;
    fn pkCheckArgcRange(
        vm: *mut PKVM,
        argc: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    ) -> bool;
    fn pkValidateSlotString(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut *const libc::c_char,
        length: *mut uint32_t,
    ) -> bool;
    fn pkSetSlotBool(vm: *mut PKVM, index: libc::c_int, value: bool);
    fn pkSetSlotString(vm: *mut PKVM, index: libc::c_int, value: *const libc::c_char);
    fn pkSetSlotStringLength(
        vm: *mut PKVM,
        index: libc::c_int,
        value: *const libc::c_char,
        length: uint32_t,
    );
    fn pkNewList(vm: *mut PKVM, index: libc::c_int);
    fn pkListInsert(
        vm: *mut PKVM,
        list: libc::c_int,
        index: int32_t,
        value: libc::c_int,
    ) -> bool;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn osGetExeFilePath(buff: *mut libc::c_char, size: libc::c_int) -> bool;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pkNativeFn = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cwk_segment_joined {
    pub segment: cwk_segment,
    pub paths: *mut *const libc::c_char,
    pub path_index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cwk_segment {
    pub path: *const libc::c_char,
    pub segments: *const libc::c_char,
    pub begin: *const libc::c_char,
    pub end: *const libc::c_char,
    pub size: size_t,
}
pub type cwk_path_style = libc::c_uint;
pub const CWK_STYLE_UNIX: cwk_path_style = 1;
pub const CWK_STYLE_WINDOWS: cwk_path_style = 0;
pub const CWK_BACK: cwk_segment_type = 2;
pub type cwk_segment_type = libc::c_uint;
pub const CWK_CURRENT: cwk_segment_type = 1;
pub const CWK_NORMAL: cwk_segment_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
static mut path_style: cwk_path_style = CWK_STYLE_UNIX;
static mut separators: [*const libc::c_char; 2] = [
    b"\\/\0" as *const u8 as *const libc::c_char,
    b"/\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn cwk_path_output_sized(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut position: size_t,
    mut str: *const libc::c_char,
    mut length: size_t,
) -> size_t {
    let mut amount_written: size_t = 0;
    if buffer_size > position.wrapping_add(length) {
        amount_written = length;
    } else if buffer_size > position {
        amount_written = buffer_size.wrapping_sub(position);
    } else {
        amount_written = 0 as libc::c_int as size_t;
    }
    if amount_written > 0 as libc::c_int as libc::c_ulong {
        memmove(
            &mut *buffer.offset(position as isize) as *mut libc::c_char
                as *mut libc::c_void,
            str as *const libc::c_void,
            amount_written,
        );
    }
    return length;
}
unsafe extern "C" fn cwk_path_output_current(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut position: size_t,
) -> size_t {
    return cwk_path_output_sized(
        buffer,
        buffer_size,
        position,
        b".\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn cwk_path_output_back(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut position: size_t,
) -> size_t {
    return cwk_path_output_sized(
        buffer,
        buffer_size,
        position,
        b"..\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn cwk_path_output_separator(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut position: size_t,
) -> size_t {
    return cwk_path_output_sized(
        buffer,
        buffer_size,
        position,
        separators[path_style as usize],
        1 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn cwk_path_output_dot(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut position: size_t,
) -> size_t {
    return cwk_path_output_sized(
        buffer,
        buffer_size,
        position,
        b".\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn cwk_path_output(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut position: size_t,
    mut str: *const libc::c_char,
) -> size_t {
    let mut length: size_t = 0;
    length = strlen(str);
    return cwk_path_output_sized(buffer, buffer_size, position, str, length);
}
unsafe extern "C" fn cwk_path_terminate_output(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut pos: size_t,
) {
    if buffer_size > 0 as libc::c_int as libc::c_ulong {
        if pos >= buffer_size {
            *buffer
                .offset(
                    buffer_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
        } else {
            *buffer.offset(pos as isize) = '\0' as i32 as libc::c_char;
        }
    }
}
unsafe extern "C" fn cwk_path_is_string_equal(
    mut first: *const libc::c_char,
    mut second: *const libc::c_char,
    mut first_size: size_t,
    mut second_size: size_t,
) -> bool {
    let mut are_both_separators: bool = false;
    if first_size != second_size {
        return 0 as libc::c_int != 0;
    }
    if path_style as libc::c_uint == CWK_STYLE_UNIX as libc::c_int as libc::c_uint {
        return strncmp(first, second, first_size) == 0 as libc::c_int;
    }
    while *first as libc::c_int != 0 && *second as libc::c_int != 0
        && first_size > 0 as libc::c_int as libc::c_ulong
    {
        are_both_separators = !(strchr(
            separators[path_style as usize],
            *first as libc::c_int,
        ))
            .is_null()
            && !(strchr(separators[path_style as usize], *second as libc::c_int))
                .is_null();
        if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *first as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*first as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(*first as libc::c_int as isize);
            }
            __res
        })
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *second as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(*second as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*second as libc::c_int as isize);
                }
                __res
            }) && !are_both_separators
        {
            return 0 as libc::c_int != 0;
        }
        first = first.offset(1);
        first;
        second = second.offset(1);
        second;
        first_size = first_size.wrapping_sub(1);
        first_size;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cwk_path_find_next_stop(
    mut c: *const libc::c_char,
) -> *const libc::c_char {
    while *c as libc::c_int != '\0' as i32 && !cwk_path_is_separator(c) {
        c = c.offset(1);
        c;
    }
    return c;
}
unsafe extern "C" fn cwk_path_find_previous_stop(
    mut begin: *const libc::c_char,
    mut c: *const libc::c_char,
) -> *const libc::c_char {
    while c > begin && !cwk_path_is_separator(c) {
        c = c.offset(-1);
        c;
    }
    if cwk_path_is_separator(c) {
        return c.offset(1 as libc::c_int as isize)
    } else {
        return c
    };
}
unsafe extern "C" fn cwk_path_get_first_segment_without_root(
    mut path: *const libc::c_char,
    mut segments: *const libc::c_char,
    mut segment: *mut cwk_segment,
) -> bool {
    (*segment).path = path;
    (*segment).segments = segments;
    (*segment).begin = segments;
    (*segment).end = segments;
    (*segment).size = 0 as libc::c_int as size_t;
    if *segments as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    while cwk_path_is_separator(segments) {
        segments = segments.offset(1);
        segments;
        if *segments as libc::c_int == '\0' as i32 {
            return 0 as libc::c_int != 0;
        }
    }
    (*segment).begin = segments;
    segments = cwk_path_find_next_stop(segments);
    (*segment).size = segments.offset_from((*segment).begin) as libc::c_long as size_t;
    (*segment).end = segments;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cwk_path_get_last_segment_without_root(
    mut path: *const libc::c_char,
    mut segment: *mut cwk_segment,
) -> bool {
    if !cwk_path_get_first_segment_without_root(path, path, segment) {
        return 0 as libc::c_int != 0;
    }
    while cwk_path_get_next_segment(segment) {}
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cwk_path_get_first_segment_joined(
    mut paths: *mut *const libc::c_char,
    mut sj: *mut cwk_segment_joined,
) -> bool {
    let mut result: bool = false;
    (*sj).path_index = 0 as libc::c_int as size_t;
    (*sj).paths = paths;
    result = 0 as libc::c_int != 0;
    while !(*paths.offset((*sj).path_index as isize)).is_null()
        && {
            result = cwk_path_get_first_segment(
                *paths.offset((*sj).path_index as isize),
                &mut (*sj).segment,
            );
            result as libc::c_int == 0 as libc::c_int
        }
    {
        (*sj).path_index = ((*sj).path_index).wrapping_add(1);
        (*sj).path_index;
    }
    return result;
}
unsafe extern "C" fn cwk_path_get_next_segment_joined(
    mut sj: *mut cwk_segment_joined,
) -> bool {
    let mut result: bool = false;
    if (*((*sj).paths).offset((*sj).path_index as isize)).is_null() {
        return 0 as libc::c_int != 0
    } else if cwk_path_get_next_segment(&mut (*sj).segment) {
        return 1 as libc::c_int != 0
    }
    result = 0 as libc::c_int != 0;
    loop {
        (*sj).path_index = ((*sj).path_index).wrapping_add(1);
        (*sj).path_index;
        if (*((*sj).paths).offset((*sj).path_index as isize)).is_null() {
            break;
        }
        result = cwk_path_get_first_segment_without_root(
            *((*sj).paths).offset((*sj).path_index as isize),
            *((*sj).paths).offset((*sj).path_index as isize),
            &mut (*sj).segment,
        );
        if result {
            break;
        }
    }
    return result;
}
unsafe extern "C" fn cwk_path_get_previous_segment_joined(
    mut sj: *mut cwk_segment_joined,
) -> bool {
    let mut result: bool = false;
    if (*(*sj).paths).is_null() {
        return 0 as libc::c_int != 0
    } else if cwk_path_get_previous_segment(&mut (*sj).segment) {
        return 1 as libc::c_int != 0
    }
    result = 0 as libc::c_int != 0;
    while !((*sj).path_index == 0 as libc::c_int as libc::c_ulong) {
        (*sj).path_index = ((*sj).path_index).wrapping_sub(1);
        (*sj).path_index;
        if (*sj).path_index == 0 as libc::c_int as libc::c_ulong {
            result = cwk_path_get_last_segment(
                *((*sj).paths).offset((*sj).path_index as isize),
                &mut (*sj).segment,
            );
        } else {
            result = cwk_path_get_last_segment_without_root(
                *((*sj).paths).offset((*sj).path_index as isize),
                &mut (*sj).segment,
            );
        }
        if result {
            break;
        }
    }
    return result;
}
unsafe extern "C" fn cwk_path_segment_back_will_be_removed(
    mut sj: *mut cwk_segment_joined,
) -> bool {
    let mut type_0: cwk_segment_type = CWK_NORMAL;
    let mut counter: libc::c_int = 0;
    counter = 0 as libc::c_int;
    while cwk_path_get_previous_segment_joined(sj) {
        type_0 = cwk_path_get_segment_type(&mut (*sj).segment);
        if type_0 as libc::c_uint == CWK_NORMAL as libc::c_int as libc::c_uint {
            counter += 1;
            counter;
            if counter > 0 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
        } else if type_0 as libc::c_uint == CWK_BACK as libc::c_int as libc::c_uint {
            counter -= 1;
            counter;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn cwk_path_segment_normal_will_be_removed(
    mut sj: *mut cwk_segment_joined,
) -> bool {
    let mut type_0: cwk_segment_type = CWK_NORMAL;
    let mut counter: libc::c_int = 0;
    counter = 0 as libc::c_int;
    while cwk_path_get_next_segment_joined(sj) {
        type_0 = cwk_path_get_segment_type(&mut (*sj).segment);
        if type_0 as libc::c_uint == CWK_NORMAL as libc::c_int as libc::c_uint {
            counter += 1;
            counter;
        } else if type_0 as libc::c_uint == CWK_BACK as libc::c_int as libc::c_uint {
            counter -= 1;
            counter;
            if counter < 0 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn cwk_path_segment_will_be_removed(
    mut sj: *const cwk_segment_joined,
    mut absolute: bool,
) -> bool {
    let mut type_0: cwk_segment_type = CWK_NORMAL;
    let mut sjc: cwk_segment_joined = cwk_segment_joined {
        segment: cwk_segment {
            path: 0 as *const libc::c_char,
            segments: 0 as *const libc::c_char,
            begin: 0 as *const libc::c_char,
            end: 0 as *const libc::c_char,
            size: 0,
        },
        paths: 0 as *mut *const libc::c_char,
        path_index: 0,
    };
    sjc = *sj;
    type_0 = cwk_path_get_segment_type(&(*sj).segment);
    if type_0 as libc::c_uint == CWK_CURRENT as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == CWK_BACK as libc::c_int as libc::c_uint
            && absolute as libc::c_int != 0
    {
        return 1 as libc::c_int != 0
    } else if type_0 as libc::c_uint == CWK_BACK as libc::c_int as libc::c_uint {
        return cwk_path_segment_back_will_be_removed(&mut sjc)
    } else {
        return cwk_path_segment_normal_will_be_removed(&mut sjc)
    };
}
unsafe extern "C" fn cwk_path_segment_joined_skip_invisible(
    mut sj: *mut cwk_segment_joined,
    mut absolute: bool,
) -> bool {
    while cwk_path_segment_will_be_removed(sj, absolute) {
        if !cwk_path_get_next_segment_joined(sj) {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cwk_path_get_root_windows(
    mut path: *const libc::c_char,
    mut length: *mut size_t,
) {
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut is_device_path: bool = false;
    c = path;
    *length = 0 as libc::c_int as size_t;
    if *c == 0 {
        return;
    }
    if cwk_path_is_separator(c) {
        c = c.offset(1);
        c;
        if !cwk_path_is_separator(c) {
            *length = (*length).wrapping_add(1);
            *length;
            return;
        }
        c = c.offset(1);
        c;
        is_device_path = (*c as libc::c_int == '?' as i32
            || *c as libc::c_int == '.' as i32)
            && {
                c = c.offset(1);
                cwk_path_is_separator(c) as libc::c_int != 0
            };
        if is_device_path {
            *length = 4 as libc::c_int as size_t;
            return;
        }
        c = cwk_path_find_next_stop(c);
        while cwk_path_is_separator(c) {
            c = c.offset(1);
            c;
        }
        c = cwk_path_find_next_stop(c);
        if cwk_path_is_separator(c) {
            c = c.offset(1);
            c;
        }
        *length = c.offset_from(path) as libc::c_long as size_t;
        return;
    }
    c = c.offset(1);
    if *c as libc::c_int == ':' as i32 {
        *length = 2 as libc::c_int as size_t;
        c = c.offset(1);
        if cwk_path_is_separator(c) {
            *length = 3 as libc::c_int as size_t;
        }
    }
}
unsafe extern "C" fn cwk_path_get_root_unix(
    mut path: *const libc::c_char,
    mut length: *mut size_t,
) {
    if cwk_path_is_separator(path) {
        *length = 1 as libc::c_int as size_t;
    } else {
        *length = 0 as libc::c_int as size_t;
    };
}
unsafe extern "C" fn cwk_path_is_root_absolute(
    mut path: *const libc::c_char,
    mut length: size_t,
) -> bool {
    if length == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    return cwk_path_is_separator(
        &*path.offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
    );
}
unsafe extern "C" fn cwk_path_fix_root(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut length: size_t,
) {
    let mut i: size_t = 0;
    if path_style as libc::c_uint != CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint {
        return;
    }
    if length > buffer_size {
        length = buffer_size;
    }
    i = 0 as libc::c_int as size_t;
    while i < length {
        if cwk_path_is_separator(&mut *buffer.offset(i as isize)) {
            *buffer
                .offset(
                    i as isize,
                ) = *separators[CWK_STYLE_WINDOWS as libc::c_int as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn cwk_path_join_and_normalize_multiple(
    mut paths: *mut *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut pos: size_t = 0;
    let mut absolute: bool = false;
    let mut has_segment_output: bool = false;
    let mut sj: cwk_segment_joined = cwk_segment_joined {
        segment: cwk_segment {
            path: 0 as *const libc::c_char,
            segments: 0 as *const libc::c_char,
            begin: 0 as *const libc::c_char,
            end: 0 as *const libc::c_char,
            size: 0,
        },
        paths: 0 as *mut *const libc::c_char,
        path_index: 0,
    };
    cwk_path_get_root(*paths.offset(0 as libc::c_int as isize), &mut pos);
    absolute = cwk_path_is_root_absolute(*paths.offset(0 as libc::c_int as isize), pos);
    cwk_path_output_sized(
        buffer,
        buffer_size,
        0 as libc::c_int as size_t,
        *paths.offset(0 as libc::c_int as isize),
        pos,
    );
    cwk_path_fix_root(buffer, buffer_size, pos);
    if cwk_path_get_first_segment_joined(paths, &mut sj) {
        has_segment_output = 0 as libc::c_int != 0;
        loop {
            if !cwk_path_segment_will_be_removed(&mut sj, absolute) {
                if has_segment_output {
                    pos = (pos as libc::c_ulong)
                        .wrapping_add(
                            cwk_path_output_separator(buffer, buffer_size, pos),
                        ) as size_t as size_t;
                }
                has_segment_output = 1 as libc::c_int != 0;
                pos = (pos as libc::c_ulong)
                    .wrapping_add(
                        cwk_path_output_sized(
                            buffer,
                            buffer_size,
                            pos,
                            sj.segment.begin,
                            sj.segment.size,
                        ),
                    ) as size_t as size_t;
            }
            if !cwk_path_get_next_segment_joined(&mut sj) {
                break;
            }
        }
        if !has_segment_output && pos == 0 as libc::c_int as libc::c_ulong {
            if absolute as libc::c_int == 0 as libc::c_int {} else {
                __assert_fail(
                    b"absolute == false\0" as *const u8 as *const libc::c_char,
                    b"src/libs/thirdparty/cwalk/cwalk.c\0" as *const u8
                        as *const libc::c_char,
                    671 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"size_t cwk_path_join_and_normalize_multiple(const char **, char *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_3144: {
                if absolute as libc::c_int == 0 as libc::c_int {} else {
                    __assert_fail(
                        b"absolute == false\0" as *const u8 as *const libc::c_char,
                        b"src/libs/thirdparty/cwalk/cwalk.c\0" as *const u8
                            as *const libc::c_char,
                        671 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 75],
                            &[libc::c_char; 75],
                        >(
                            b"size_t cwk_path_join_and_normalize_multiple(const char **, char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            pos = (pos as libc::c_ulong)
                .wrapping_add(cwk_path_output_current(buffer, buffer_size, pos))
                as size_t as size_t;
        }
    }
    cwk_path_terminate_output(buffer, buffer_size, pos);
    return pos;
}
pub unsafe extern "C" fn cwk_path_get_absolute(
    mut base: *const libc::c_char,
    mut path: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut paths: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    if cwk_path_is_absolute(base) {
        i = 0 as libc::c_int as size_t;
    } else if path_style as libc::c_uint
        == CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint
    {
        paths[0 as libc::c_int as usize] = b"\\\0" as *const u8 as *const libc::c_char;
        i = 1 as libc::c_int as size_t;
    } else {
        paths[0 as libc::c_int as usize] = b"/\0" as *const u8 as *const libc::c_char;
        i = 1 as libc::c_int as size_t;
    }
    if cwk_path_is_absolute(path) {
        let fresh0 = i;
        i = i.wrapping_add(1);
        paths[fresh0 as usize] = path;
        paths[i as usize] = 0 as *const libc::c_char;
    } else {
        let fresh1 = i;
        i = i.wrapping_add(1);
        paths[fresh1 as usize] = base;
        let fresh2 = i;
        i = i.wrapping_add(1);
        paths[fresh2 as usize] = path;
        paths[i as usize] = 0 as *const libc::c_char;
    }
    return cwk_path_join_and_normalize_multiple(paths.as_mut_ptr(), buffer, buffer_size);
}
unsafe extern "C" fn cwk_path_skip_segments_until_diverge(
    mut bsj: *mut cwk_segment_joined,
    mut osj: *mut cwk_segment_joined,
    mut absolute: bool,
    mut base_available: *mut bool,
    mut other_available: *mut bool,
) {
    loop {
        *base_available = cwk_path_segment_joined_skip_invisible(bsj, absolute);
        *other_available = cwk_path_segment_joined_skip_invisible(osj, absolute);
        if !*base_available || !*other_available {
            break;
        }
        if !cwk_path_is_string_equal(
            (*bsj).segment.begin,
            (*osj).segment.begin,
            (*bsj).segment.size,
            (*osj).segment.size,
        ) {
            break;
        }
        *base_available = cwk_path_get_next_segment_joined(bsj);
        *other_available = cwk_path_get_next_segment_joined(osj);
        if !(*base_available as libc::c_int != 0 && *other_available as libc::c_int != 0)
        {
            break;
        }
    };
}
pub unsafe extern "C" fn cwk_path_get_relative(
    mut base_directory: *const libc::c_char,
    mut path: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut pos: size_t = 0;
    let mut base_root_length: size_t = 0;
    let mut path_root_length: size_t = 0;
    let mut absolute: bool = false;
    let mut base_available: bool = false;
    let mut other_available: bool = false;
    let mut has_output: bool = false;
    let mut base_paths: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut other_paths: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut bsj: cwk_segment_joined = cwk_segment_joined {
        segment: cwk_segment {
            path: 0 as *const libc::c_char,
            segments: 0 as *const libc::c_char,
            begin: 0 as *const libc::c_char,
            end: 0 as *const libc::c_char,
            size: 0,
        },
        paths: 0 as *mut *const libc::c_char,
        path_index: 0,
    };
    let mut osj: cwk_segment_joined = cwk_segment_joined {
        segment: cwk_segment {
            path: 0 as *const libc::c_char,
            segments: 0 as *const libc::c_char,
            begin: 0 as *const libc::c_char,
            end: 0 as *const libc::c_char,
            size: 0,
        },
        paths: 0 as *mut *const libc::c_char,
        path_index: 0,
    };
    pos = 0 as libc::c_int as size_t;
    cwk_path_get_root(base_directory, &mut base_root_length);
    cwk_path_get_root(path, &mut path_root_length);
    if base_root_length != path_root_length
        || !cwk_path_is_string_equal(
            base_directory,
            path,
            base_root_length,
            path_root_length,
        )
    {
        cwk_path_terminate_output(buffer, buffer_size, pos);
        return pos;
    }
    absolute = cwk_path_is_root_absolute(base_directory, base_root_length);
    base_paths[0 as libc::c_int as usize] = base_directory;
    base_paths[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    other_paths[0 as libc::c_int as usize] = path;
    other_paths[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    cwk_path_get_first_segment_joined(base_paths.as_mut_ptr(), &mut bsj);
    cwk_path_get_first_segment_joined(other_paths.as_mut_ptr(), &mut osj);
    cwk_path_skip_segments_until_diverge(
        &mut bsj,
        &mut osj,
        absolute,
        &mut base_available,
        &mut other_available,
    );
    has_output = 0 as libc::c_int != 0;
    if base_available {
        while cwk_path_segment_joined_skip_invisible(&mut bsj, absolute) {
            has_output = 1 as libc::c_int != 0;
            pos = (pos as libc::c_ulong)
                .wrapping_add(cwk_path_output_back(buffer, buffer_size, pos)) as size_t
                as size_t;
            pos = (pos as libc::c_ulong)
                .wrapping_add(cwk_path_output_separator(buffer, buffer_size, pos))
                as size_t as size_t;
            if !cwk_path_get_next_segment_joined(&mut bsj) {
                break;
            }
        }
    }
    if other_available {
        while cwk_path_segment_joined_skip_invisible(&mut osj, absolute) {
            has_output = 1 as libc::c_int != 0;
            pos = (pos as libc::c_ulong)
                .wrapping_add(
                    cwk_path_output_sized(
                        buffer,
                        buffer_size,
                        pos,
                        osj.segment.begin,
                        osj.segment.size,
                    ),
                ) as size_t as size_t;
            pos = (pos as libc::c_ulong)
                .wrapping_add(cwk_path_output_separator(buffer, buffer_size, pos))
                as size_t as size_t;
            if !cwk_path_get_next_segment_joined(&mut osj) {
                break;
            }
        }
    }
    if has_output {
        pos = pos.wrapping_sub(1);
        pos;
    } else {
        pos = (pos as libc::c_ulong)
            .wrapping_add(cwk_path_output_current(buffer, buffer_size, pos)) as size_t
            as size_t;
    }
    cwk_path_terminate_output(buffer, buffer_size, pos);
    return pos;
}
pub unsafe extern "C" fn cwk_path_join(
    mut path_a: *const libc::c_char,
    mut path_b: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut paths: [*const libc::c_char; 3] = [0 as *const libc::c_char; 3];
    paths[0 as libc::c_int as usize] = path_a;
    paths[1 as libc::c_int as usize] = path_b;
    paths[2 as libc::c_int as usize] = 0 as *const libc::c_char;
    return cwk_path_join_and_normalize_multiple(paths.as_mut_ptr(), buffer, buffer_size);
}
pub unsafe extern "C" fn cwk_path_join_multiple(
    mut paths: *mut *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    return cwk_path_join_and_normalize_multiple(paths, buffer, buffer_size);
}
pub unsafe extern "C" fn cwk_path_get_root(
    mut path: *const libc::c_char,
    mut length: *mut size_t,
) {
    if path_style as libc::c_uint == CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint {
        cwk_path_get_root_windows(path, length);
    } else {
        cwk_path_get_root_unix(path, length);
    };
}
pub unsafe extern "C" fn cwk_path_change_root(
    mut path: *const libc::c_char,
    mut new_root: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut tail: *const libc::c_char = 0 as *const libc::c_char;
    let mut root_length: size_t = 0;
    let mut path_length: size_t = 0;
    let mut tail_length: size_t = 0;
    let mut new_root_length: size_t = 0;
    let mut new_path_size: size_t = 0;
    cwk_path_get_root(path, &mut root_length);
    new_root_length = strlen(new_root);
    path_length = strlen(path);
    tail = path.offset(root_length as isize);
    tail_length = path_length.wrapping_sub(root_length);
    cwk_path_output_sized(buffer, buffer_size, new_root_length, tail, tail_length);
    cwk_path_output_sized(
        buffer,
        buffer_size,
        0 as libc::c_int as size_t,
        new_root,
        new_root_length,
    );
    new_path_size = tail_length.wrapping_add(new_root_length);
    cwk_path_terminate_output(buffer, buffer_size, new_path_size);
    return new_path_size;
}
pub unsafe extern "C" fn cwk_path_is_absolute(mut path: *const libc::c_char) -> bool {
    let mut length: size_t = 0;
    cwk_path_get_root(path, &mut length);
    return cwk_path_is_root_absolute(path, length);
}
pub unsafe extern "C" fn cwk_path_is_relative(mut path: *const libc::c_char) -> bool {
    return !cwk_path_is_absolute(path);
}
pub unsafe extern "C" fn cwk_path_get_basename(
    mut path: *const libc::c_char,
    mut basename: *mut *const libc::c_char,
    mut length: *mut size_t,
) {
    let mut segment: cwk_segment = cwk_segment {
        path: 0 as *const libc::c_char,
        segments: 0 as *const libc::c_char,
        begin: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
        size: 0,
    };
    if !cwk_path_get_last_segment(path, &mut segment) {
        *basename = 0 as *const libc::c_char;
        if !length.is_null() {
            *length = 0 as libc::c_int as size_t;
        }
        return;
    }
    *basename = segment.begin;
    if !length.is_null() {
        *length = segment.size;
    }
}
pub unsafe extern "C" fn cwk_path_change_basename(
    mut path: *const libc::c_char,
    mut new_basename: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut segment: cwk_segment = cwk_segment {
        path: 0 as *const libc::c_char,
        segments: 0 as *const libc::c_char,
        begin: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
        size: 0,
    };
    let mut pos: size_t = 0;
    let mut root_size: size_t = 0;
    let mut new_basename_size: size_t = 0;
    if !cwk_path_get_last_segment(path, &mut segment) {
        cwk_path_get_root(path, &mut root_size);
        pos = cwk_path_output_sized(
            buffer,
            buffer_size,
            0 as libc::c_int as size_t,
            path,
            root_size,
        );
        while cwk_path_is_separator(new_basename) {
            new_basename = new_basename.offset(1);
            new_basename;
        }
        new_basename_size = 0 as libc::c_int as size_t;
        while *new_basename.offset(new_basename_size as isize) != 0 {
            new_basename_size = new_basename_size.wrapping_add(1);
            new_basename_size;
        }
        while new_basename_size > 0 as libc::c_int as libc::c_ulong
            && cwk_path_is_separator(
                &*new_basename
                    .offset(
                        new_basename_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ),
            ) as libc::c_int != 0
        {
            new_basename_size = new_basename_size.wrapping_sub(1);
            new_basename_size;
        }
        pos = (pos as libc::c_ulong)
            .wrapping_add(
                cwk_path_output_sized(
                    buffer,
                    buffer_size,
                    pos,
                    new_basename,
                    new_basename_size,
                ),
            ) as size_t as size_t;
        cwk_path_terminate_output(buffer, buffer_size, pos);
        return pos;
    }
    return cwk_path_change_segment(&mut segment, new_basename, buffer, buffer_size);
}
pub unsafe extern "C" fn cwk_path_get_dirname(
    mut path: *const libc::c_char,
    mut length: *mut size_t,
) {
    let mut segment: cwk_segment = cwk_segment {
        path: 0 as *const libc::c_char,
        segments: 0 as *const libc::c_char,
        begin: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
        size: 0,
    };
    if !cwk_path_get_last_segment(path, &mut segment) {
        *length = 0 as libc::c_int as size_t;
        return;
    }
    *length = (segment.begin).offset_from(path) as libc::c_long as size_t;
}
pub unsafe extern "C" fn cwk_path_get_extension(
    mut path: *const libc::c_char,
    mut extension: *mut *const libc::c_char,
    mut length: *mut size_t,
) -> bool {
    let mut segment: cwk_segment = cwk_segment {
        path: 0 as *const libc::c_char,
        segments: 0 as *const libc::c_char,
        begin: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
        size: 0,
    };
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    if !cwk_path_get_last_segment(path, &mut segment) {
        return 0 as libc::c_int != 0;
    }
    c = segment.end;
    while c >= segment.begin {
        if *c as libc::c_int == '.' as i32 {
            *extension = c;
            *length = (segment.end).offset_from(c) as libc::c_long as size_t;
            return 1 as libc::c_int != 0;
        }
        c = c.offset(-1);
        c;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn cwk_path_has_extension(mut path: *const libc::c_char) -> bool {
    let mut extension: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: size_t = 0;
    return cwk_path_get_extension(path, &mut extension, &mut length);
}
pub unsafe extern "C" fn cwk_path_change_extension(
    mut path: *const libc::c_char,
    mut new_extension: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut segment: cwk_segment = cwk_segment {
        path: 0 as *const libc::c_char,
        segments: 0 as *const libc::c_char,
        begin: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
        size: 0,
    };
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut old_extension: *const libc::c_char = 0 as *const libc::c_char;
    let mut pos: size_t = 0;
    let mut root_size: size_t = 0;
    let mut trail_size: size_t = 0;
    let mut new_extension_size: size_t = 0;
    if !cwk_path_get_last_segment(path, &mut segment) {
        cwk_path_get_root(path, &mut root_size);
        pos = cwk_path_output_sized(
            buffer,
            buffer_size,
            0 as libc::c_int as size_t,
            path,
            root_size,
        );
        if *new_extension as libc::c_int != '.' as i32 {
            pos = (pos as libc::c_ulong)
                .wrapping_add(cwk_path_output_dot(buffer, buffer_size, pos)) as size_t
                as size_t;
        }
        pos = (pos as libc::c_ulong)
            .wrapping_add(cwk_path_output(buffer, buffer_size, pos, new_extension))
            as size_t as size_t;
        cwk_path_terminate_output(buffer, buffer_size, pos);
        return pos;
    }
    old_extension = segment.end;
    c = segment.begin;
    while c < segment.end {
        if *c as libc::c_int == '.' as i32 {
            old_extension = c;
        }
        c = c.offset(1);
        c;
    }
    pos = cwk_path_output_sized(
        buffer,
        buffer_size,
        0 as libc::c_int as size_t,
        segment.path,
        old_extension.offset_from(segment.path) as libc::c_long as size_t,
    );
    if *new_extension as libc::c_int == '.' as i32 {
        new_extension = new_extension.offset(1);
        new_extension;
    }
    new_extension_size = (strlen(new_extension))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    trail_size = cwk_path_output(
        buffer,
        buffer_size,
        pos.wrapping_add(new_extension_size),
        segment.end,
    );
    pos = (pos as libc::c_ulong)
        .wrapping_add(cwk_path_output_dot(buffer, buffer_size, pos)) as size_t as size_t;
    pos = (pos as libc::c_ulong)
        .wrapping_add(cwk_path_output(buffer, buffer_size, pos, new_extension)) as size_t
        as size_t;
    pos = (pos as libc::c_ulong).wrapping_add(trail_size) as size_t as size_t;
    cwk_path_terminate_output(buffer, buffer_size, pos);
    return pos;
}
pub unsafe extern "C" fn cwk_path_normalize(
    mut path: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut paths: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    paths[0 as libc::c_int as usize] = path;
    paths[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    return cwk_path_join_and_normalize_multiple(paths.as_mut_ptr(), buffer, buffer_size);
}
pub unsafe extern "C" fn cwk_path_get_intersection(
    mut path_base: *const libc::c_char,
    mut path_other: *const libc::c_char,
) -> size_t {
    let mut absolute: bool = false;
    let mut base_root_length: size_t = 0;
    let mut other_root_length: size_t = 0;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut paths_base: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut paths_other: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut base: cwk_segment_joined = cwk_segment_joined {
        segment: cwk_segment {
            path: 0 as *const libc::c_char,
            segments: 0 as *const libc::c_char,
            begin: 0 as *const libc::c_char,
            end: 0 as *const libc::c_char,
            size: 0,
        },
        paths: 0 as *mut *const libc::c_char,
        path_index: 0,
    };
    let mut other: cwk_segment_joined = cwk_segment_joined {
        segment: cwk_segment {
            path: 0 as *const libc::c_char,
            segments: 0 as *const libc::c_char,
            begin: 0 as *const libc::c_char,
            end: 0 as *const libc::c_char,
            size: 0,
        },
        paths: 0 as *mut *const libc::c_char,
        path_index: 0,
    };
    cwk_path_get_root(path_base, &mut base_root_length);
    cwk_path_get_root(path_other, &mut other_root_length);
    if !cwk_path_is_string_equal(
        path_base,
        path_other,
        base_root_length,
        other_root_length,
    ) {
        return 0 as libc::c_int as size_t;
    }
    paths_base[0 as libc::c_int as usize] = path_base;
    paths_base[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    paths_other[0 as libc::c_int as usize] = path_other;
    paths_other[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    if !cwk_path_get_first_segment_joined(paths_base.as_mut_ptr(), &mut base)
        || !cwk_path_get_first_segment_joined(paths_other.as_mut_ptr(), &mut other)
    {
        return base_root_length;
    }
    absolute = cwk_path_is_root_absolute(path_base, base_root_length);
    end = path_base.offset(base_root_length as isize);
    while !(!cwk_path_segment_joined_skip_invisible(&mut base, absolute)
        || !cwk_path_segment_joined_skip_invisible(&mut other, absolute))
    {
        if !cwk_path_is_string_equal(
            base.segment.begin,
            other.segment.begin,
            base.segment.size,
            other.segment.size,
        ) {
            return end.offset_from(path_base) as libc::c_long as size_t;
        }
        end = base.segment.end;
        if !(cwk_path_get_next_segment_joined(&mut base) as libc::c_int != 0
            && cwk_path_get_next_segment_joined(&mut other) as libc::c_int != 0)
        {
            break;
        }
    }
    return end.offset_from(path_base) as libc::c_long as size_t;
}
pub unsafe extern "C" fn cwk_path_get_first_segment(
    mut path: *const libc::c_char,
    mut segment: *mut cwk_segment,
) -> bool {
    let mut length: size_t = 0;
    let mut segments: *const libc::c_char = 0 as *const libc::c_char;
    cwk_path_get_root(path, &mut length);
    segments = path.offset(length as isize);
    return cwk_path_get_first_segment_without_root(path, segments, segment);
}
pub unsafe extern "C" fn cwk_path_get_last_segment(
    mut path: *const libc::c_char,
    mut segment: *mut cwk_segment,
) -> bool {
    if !cwk_path_get_first_segment(path, segment) {
        return 0 as libc::c_int != 0;
    }
    while cwk_path_get_next_segment(segment) {}
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn cwk_path_get_next_segment(
    mut segment: *mut cwk_segment,
) -> bool {
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    c = ((*segment).begin).offset((*segment).size as isize);
    if *c as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    if cwk_path_is_separator(c) {} else {
        __assert_fail(
            b"cwk_path_is_separator(c)\0" as *const u8 as *const libc::c_char,
            b"src/libs/thirdparty/cwalk/cwalk.c\0" as *const u8 as *const libc::c_char,
            1278 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"_Bool cwk_path_get_next_segment(struct cwk_segment *)\0"))
                .as_ptr(),
        );
    }
    'c_3587: {
        if cwk_path_is_separator(c) {} else {
            __assert_fail(
                b"cwk_path_is_separator(c)\0" as *const u8 as *const libc::c_char,
                b"src/libs/thirdparty/cwalk/cwalk.c\0" as *const u8
                    as *const libc::c_char,
                1278 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"_Bool cwk_path_get_next_segment(struct cwk_segment *)\0"))
                    .as_ptr(),
            );
        }
    };
    loop {
        c = c.offset(1);
        c;
        if !cwk_path_is_separator(c) {
            break;
        }
    }
    if *c as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int != 0;
    }
    (*segment).begin = c;
    c = cwk_path_find_next_stop(c);
    (*segment).end = c;
    (*segment).size = c.offset_from((*segment).begin) as libc::c_long as size_t;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn cwk_path_get_previous_segment(
    mut segment: *mut cwk_segment,
) -> bool {
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    c = (*segment).begin;
    if c <= (*segment).segments {
        return 0 as libc::c_int != 0;
    }
    loop {
        c = c.offset(-1);
        c;
        if c < (*segment).segments {
            return 0 as libc::c_int != 0;
        }
        if !cwk_path_is_separator(c) {
            break;
        }
    }
    (*segment).end = c.offset(1 as libc::c_int as isize);
    (*segment).begin = cwk_path_find_previous_stop((*segment).segments, c);
    (*segment)
        .size = ((*segment).end).offset_from((*segment).begin) as libc::c_long as size_t;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn cwk_path_get_segment_type(
    mut segment: *const cwk_segment,
) -> cwk_segment_type {
    if strncmp(
        (*segment).begin,
        b".\0" as *const u8 as *const libc::c_char,
        (*segment).size,
    ) == 0 as libc::c_int
    {
        return CWK_CURRENT
    } else if strncmp(
        (*segment).begin,
        b"..\0" as *const u8 as *const libc::c_char,
        (*segment).size,
    ) == 0 as libc::c_int
    {
        return CWK_BACK
    }
    return CWK_NORMAL;
}
pub unsafe extern "C" fn cwk_path_set_style(mut style: cwk_path_style) {
    if style as libc::c_uint == CWK_STYLE_UNIX as libc::c_int as libc::c_uint
        || style as libc::c_uint == CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"style == CWK_STYLE_UNIX || style == CWK_STYLE_WINDOWS\0" as *const u8
                as *const libc::c_char,
            b"src/libs/thirdparty/cwalk/cwalk.c\0" as *const u8 as *const libc::c_char,
            1475 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void cwk_path_set_style(enum cwk_path_style)\0"))
                .as_ptr(),
        );
    }
    'c_7254: {
        if style as libc::c_uint == CWK_STYLE_UNIX as libc::c_int as libc::c_uint
            || style as libc::c_uint == CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"style == CWK_STYLE_UNIX || style == CWK_STYLE_WINDOWS\0" as *const u8
                    as *const libc::c_char,
                b"src/libs/thirdparty/cwalk/cwalk.c\0" as *const u8
                    as *const libc::c_char,
                1475 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void cwk_path_set_style(enum cwk_path_style)\0"))
                    .as_ptr(),
            );
        }
    };
    path_style = style;
}
pub unsafe extern "C" fn cwk_path_get_style() -> cwk_path_style {
    return path_style;
}
pub unsafe extern "C" fn cwk_path_guess_style(
    mut path: *const libc::c_char,
) -> cwk_path_style {
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut root_length: size_t = 0;
    let mut segment: cwk_segment = cwk_segment {
        path: 0 as *const libc::c_char,
        segments: 0 as *const libc::c_char,
        begin: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
        size: 0,
    };
    cwk_path_get_root_windows(path, &mut root_length);
    if root_length > 1 as libc::c_int as libc::c_ulong {
        return CWK_STYLE_WINDOWS;
    }
    c = path;
    while *c != 0 {
        if *c as libc::c_int
            == *separators[CWK_STYLE_UNIX as libc::c_int as usize] as libc::c_int
        {
            return CWK_STYLE_UNIX
        } else if *c as libc::c_int
            == *separators[CWK_STYLE_WINDOWS as libc::c_int as usize] as libc::c_int
        {
            return CWK_STYLE_WINDOWS
        }
        c = c.offset(1);
        c;
    }
    if !cwk_path_get_last_segment(path, &mut segment) {
        return CWK_STYLE_UNIX;
    }
    if *segment.begin as libc::c_int == '.' as i32 {
        return CWK_STYLE_UNIX;
    }
    c = segment.begin;
    while *c != 0 {
        if *c as libc::c_int == '.' as i32 {
            return CWK_STYLE_WINDOWS;
        }
        c = c.offset(1);
        c;
    }
    return CWK_STYLE_UNIX;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn cwk_path_change_segment(
    mut segment: *mut cwk_segment,
    mut value: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> size_t {
    let mut pos: size_t = 0;
    let mut value_size: size_t = 0;
    let mut tail_size: size_t = 0;
    pos = cwk_path_output_sized(
        buffer,
        buffer_size,
        0 as libc::c_int as size_t,
        (*segment).path,
        ((*segment).begin).offset_from((*segment).path) as libc::c_long as size_t,
    );
    while cwk_path_is_separator(value) {
        value = value.offset(1);
        value;
    }
    value_size = 0 as libc::c_int as size_t;
    while *value.offset(value_size as isize) != 0 {
        value_size = value_size.wrapping_add(1);
        value_size;
    }
    while value_size > 0 as libc::c_int as libc::c_ulong
        && cwk_path_is_separator(
            &*value
                .offset(
                    value_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ),
        ) as libc::c_int != 0
    {
        value_size = value_size.wrapping_sub(1);
        value_size;
    }
    tail_size = strlen((*segment).end);
    cwk_path_output_sized(
        buffer,
        buffer_size,
        pos.wrapping_add(value_size),
        (*segment).end,
        tail_size,
    );
    pos = (pos as libc::c_ulong)
        .wrapping_add(cwk_path_output_sized(buffer, buffer_size, pos, value, value_size))
        as size_t as size_t;
    pos = (pos as libc::c_ulong).wrapping_add(tail_size) as size_t as size_t;
    cwk_path_terminate_output(buffer, buffer_size, pos);
    return pos;
}
pub unsafe extern "C" fn cwk_path_is_separator(mut str: *const libc::c_char) -> bool {
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    c = separators[path_style as usize];
    while *c != 0 {
        if *c as libc::c_int == *str as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        c = c.offset(1);
        c;
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn checkImportExists(
    mut path: *mut libc::c_char,
    mut ext: *const libc::c_char,
    mut buff: *mut libc::c_char,
) -> size_t {
    let mut path_size: size_t = strlen(path);
    let mut ext_size: size_t = strlen(ext);
    if path_size.wrapping_add(ext_size).wrapping_add(1 as libc::c_int as libc::c_ulong)
        >= 4096 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    strcpy(buff, path);
    strcpy(buff.offset(path_size as isize), ext);
    if !pathIsFile(buff) {
        return 0 as libc::c_int as size_t;
    }
    return path_size.wrapping_add(ext_size);
}
unsafe extern "C" fn tryImportPaths(
    mut vm: *mut PKVM,
    mut path: *mut libc::c_char,
    mut buff: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut path_size: size_t = 0 as libc::c_int as size_t;
    static mut EXT: [*const libc::c_char; 6] = [
        b"\0" as *const u8 as *const libc::c_char,
        b".pk\0" as *const u8 as *const libc::c_char,
        b"/_init.pk\0" as *const u8 as *const libc::c_char,
        b".so\0" as *const u8 as *const libc::c_char,
        b"/_init.so\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut ext: *mut *const libc::c_char = EXT.as_mut_ptr();
    while !(*ext).is_null() {
        path_size = checkImportExists(path, *ext, buff);
        if path_size != 0 as libc::c_int as libc::c_ulong {
            break;
        }
        ext = ext.offset(1);
        ext;
    }
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if path_size != 0 as libc::c_int as libc::c_ulong {
        ret = pkRealloc(
            vm,
            0 as *mut libc::c_void,
            path_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            ret as *mut libc::c_void,
            buff as *const libc::c_void,
            path_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    return ret;
}
pub unsafe extern "C" fn pathResolveImport(
    mut vm: *mut PKVM,
    mut from: *const libc::c_char,
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buff1: [libc::c_char; 4096] = [0; 4096];
    let mut buff2: [libc::c_char; 4096] = [0; 4096];
    if cwk_path_is_absolute(path) {
        cwk_path_normalize(
            path,
            buff1.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        );
        return tryImportPaths(vm, buff1.as_mut_ptr(), buff2.as_mut_ptr());
    }
    if from.is_null() {
        pathAbs(
            path,
            buff1.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        );
        cwk_path_normalize(
            buff1.as_mut_ptr(),
            buff2.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        );
        return tryImportPaths(vm, buff2.as_mut_ptr(), buff1.as_mut_ptr());
    }
    let mut last: libc::c_char = *from
        .offset((strlen(from)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    pathAbs(
        from,
        buff1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    if last as libc::c_int != '/' as i32 && last as libc::c_int != '\\' as i32 {
        let mut from_dir_length: size_t = 0 as libc::c_int as size_t;
        cwk_path_get_dirname(buff1.as_mut_ptr(), &mut from_dir_length);
        if from_dir_length == 0 as libc::c_int as libc::c_ulong {
            return 0 as *mut libc::c_char;
        }
        buff1[from_dir_length as usize] = '\0' as i32 as libc::c_char;
    }
    cwk_path_join(
        buff1.as_mut_ptr(),
        path,
        buff2.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    cwk_path_normalize(
        buff2.as_mut_ptr(),
        buff1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    return tryImportPaths(vm, buff1.as_mut_ptr(), buff2.as_mut_ptr());
}
#[inline]
unsafe extern "C" fn pathIsFile(mut path: *const libc::c_char) -> bool {
    let mut path_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(path, &mut path_stat) != 0 {
        return 0 as libc::c_int != 0;
    }
    return path_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn pathIsDir(mut path: *const libc::c_char) -> bool {
    let mut path_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(path, &mut path_stat) != 0 {
        return 0 as libc::c_int != 0;
    }
    return path_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn pathIsExists(mut path: *const libc::c_char) -> bool {
    return access(path, 0 as libc::c_int) == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn pathAbs(
    mut path: *const libc::c_char,
    mut buff: *mut libc::c_char,
    mut buffsz: size_t,
) -> size_t {
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    (getcwd(
        cwd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null();
    return cwk_path_get_absolute(cwd.as_mut_ptr(), path, buff, buffsz);
}
unsafe extern "C" fn _pathGetCWD(mut vm: *mut PKVM) {
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    (getcwd(
        cwd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null();
    pkSetSlotString(vm, 0 as libc::c_int, cwd.as_mut_ptr());
}
static mut _pk_doc__pathGetCWD: *const libc::c_char = b"path.getcwd() -> String\n\nReturns the current working directory.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__pathAbspath: *const libc::c_char = b"path.abspath(path:String) -> String\n\nReturns the absolute path of the [path].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathAbspath(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    let mut abspath: [libc::c_char; 4096] = [0; 4096];
    let mut len: uint32_t = pathAbs(
        path,
        abspath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ) as uint32_t;
    pkSetSlotStringLength(vm, 0 as libc::c_int, abspath.as_mut_ptr(), len);
}
static mut _pk_doc__pathRelpath: *const libc::c_char = b"path.relpath(path:String, from:String) -> String\n\nReturns the relative path of the [path] argument from the [from] directory.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathRelpath(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut from: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    if !pkValidateSlotString(vm, 2 as libc::c_int, &mut from, 0 as *mut uint32_t) {
        return;
    }
    let mut abs_path: [libc::c_char; 4096] = [0; 4096];
    pathAbs(
        path,
        abs_path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    let mut abs_from: [libc::c_char; 4096] = [0; 4096];
    pathAbs(
        from,
        abs_from.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    let mut result: [libc::c_char; 4096] = [0; 4096];
    let mut len: uint32_t = cwk_path_get_relative(
        abs_from.as_mut_ptr(),
        abs_path.as_mut_ptr(),
        result.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ) as uint32_t;
    pkSetSlotStringLength(vm, 0 as libc::c_int, result.as_mut_ptr(), len);
}
static mut _pk_doc__pathJoin: *const libc::c_char = b"path.join(...) -> String\n\nJoins path with path seperator and return it. The maximum count of paths which can be joined for a call is MAX_JOIN_PATHS.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathJoin(mut vm: *mut PKVM) {
    let mut paths: [*const libc::c_char; 9] = [0 as *const libc::c_char; 9];
    let mut argc: libc::c_int = pkGetArgc(vm);
    if argc > 8 as libc::c_int {
        pkSetRuntimeError(
            vm,
            b"Cannot join more than 8paths.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        pkValidateSlotString(
            vm,
            i + 1 as libc::c_int,
            &mut *paths.as_mut_ptr().offset(i as isize),
            0 as *mut uint32_t,
        );
        i += 1;
        i;
    }
    paths[argc as usize] = 0 as *const libc::c_char;
    let mut result: [libc::c_char; 4096] = [0; 4096];
    let mut len: uint32_t = cwk_path_join_multiple(
        paths.as_mut_ptr(),
        result.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ) as uint32_t;
    pkSetSlotStringLength(vm, 0 as libc::c_int, result.as_mut_ptr(), len);
}
static mut _pk_doc__pathNormpath: *const libc::c_char = b"path.normpath(path:String) -> String\n\nReturns the normalized path of the [path].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathNormpath(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    let mut result: [libc::c_char; 4096] = [0; 4096];
    let mut len: uint32_t = cwk_path_normalize(
        path,
        result.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ) as uint32_t;
    pkSetSlotStringLength(vm, 0 as libc::c_int, result.as_mut_ptr(), len);
}
static mut _pk_doc__pathBaseName: *const libc::c_char = b"path.basename(path:String) -> String\n\nReturns the final component for the path\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathBaseName(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    let mut base_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: size_t = 0;
    cwk_path_get_basename(path, &mut base_name, &mut length);
    pkSetSlotStringLength(vm, 0 as libc::c_int, base_name, length as uint32_t);
}
static mut _pk_doc__pathDirName: *const libc::c_char = b"path.dirname(path:String) -> String\n\nReturns the directory of the path.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathDirName(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    let mut length: size_t = 0;
    cwk_path_get_dirname(path, &mut length);
    pkSetSlotStringLength(vm, 0 as libc::c_int, path, length as uint32_t);
}
static mut _pk_doc__pathIsPathAbs: *const libc::c_char = b"path.isabspath(path:String) -> Bool\n\nReturns true if the path is absolute otherwise false.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathIsPathAbs(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    pkSetSlotBool(vm, 0 as libc::c_int, cwk_path_is_absolute(path));
}
static mut _pk_doc__pathGetExtension: *const libc::c_char = b"path.getext(path:String) -> String\n\nReturns the file extension of the path.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathGetExtension(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: size_t = 0;
    if cwk_path_get_extension(path, &mut ext, &mut length) {
        pkSetSlotStringLength(vm, 0 as libc::c_int, ext, length as uint32_t);
    } else {
        pkSetSlotStringLength(
            vm,
            0 as libc::c_int,
            0 as *const libc::c_char,
            0 as libc::c_int as uint32_t,
        );
    };
}
static mut _pk_doc__pathExists: *const libc::c_char = b"path.exists(path:String) -> String\n\nReturns true if the file exists.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathExists(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    pkSetSlotBool(vm, 0 as libc::c_int, pathIsExists(path));
}
static mut _pk_doc__pathIsFile: *const libc::c_char = b"path.isfile(path:String) -> Bool\n\nReturns true if the path is a file.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathIsFile(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    pkSetSlotBool(vm, 0 as libc::c_int, pathIsFile(path));
}
static mut _pk_doc__pathIsDir: *const libc::c_char = b"path.isdir(path:String) -> Bool\n\nReturns true if the path is a directory.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathIsDir(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    pkSetSlotBool(vm, 0 as libc::c_int, pathIsDir(path));
}
static mut _pk_doc__pathListDir: *const libc::c_char = b"path.listdir(path:String='.') -> List\n\nReturns all the entries in the directory at the [path].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _pathListDir(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = pkGetArgc(vm);
    if !pkCheckArgcRange(vm, argc, 0 as libc::c_int, 1 as libc::c_int) {
        return;
    }
    let mut path: *const libc::c_char = b".\0" as *const u8 as *const libc::c_char;
    if argc == 1 as libc::c_int {
        if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
            return;
        }
    }
    if !pathIsExists(path) {
        pkSetRuntimeErrorFmt(
            vm,
            b"Path '%s' does not exists.\0" as *const u8 as *const libc::c_char,
            path,
        );
        return;
    }
    pkNewList(vm, 0 as libc::c_int);
    let mut dirstream: *mut DIR = opendir(path);
    if !dirstream.is_null() {
        let mut dir: *mut dirent = 0 as *mut dirent;
        loop {
            dir = readdir(dirstream);
            if dir.is_null() {
                break;
            }
            if strcmp(
                ((*dir).d_name).as_mut_ptr(),
                b".\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                continue;
            }
            if strcmp(
                ((*dir).d_name).as_mut_ptr(),
                b"..\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                continue;
            }
            pkSetSlotString(vm, 1 as libc::c_int, ((*dir).d_name).as_mut_ptr());
            if !pkListInsert(
                vm,
                0 as libc::c_int,
                -(1 as libc::c_int),
                1 as libc::c_int,
            ) {
                return;
            }
        }
        closedir(dirstream);
    }
}
pub unsafe extern "C" fn _registerSearchPaths(mut vm: *mut PKVM) {
    let mut ps: cwk_path_style = cwk_path_get_style();
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    if !(getcwd(
        cwd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        let mut len: size_t = strlen(cwd.as_mut_ptr());
        if len < (4096 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
            cwd[len
                as usize] = (if ps as libc::c_uint
                == CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint
            {
                '\\' as i32
            } else {
                '/' as i32
            }) as libc::c_char;
            len = len.wrapping_add(1);
            cwd[len as usize] = '\0' as i32 as libc::c_char;
            pkAddSearchPath(vm, cwd.as_mut_ptr());
        }
    }
    let mut buff: [libc::c_char; 4096] = [0; 4096];
    if !osGetExeFilePath(buff.as_mut_ptr(), 4096 as libc::c_int) {
        return;
    }
    let mut length: size_t = 0;
    cwk_path_get_dirname(buff.as_mut_ptr(), &mut length);
    if length == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut last: libc::c_char = buff[length
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize];
    if last as libc::c_int != '/' as i32 && last as libc::c_int != '\\' as i32 {
        last = (if ps as libc::c_uint == CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint
        {
            '\\' as i32
        } else {
            '/' as i32
        }) as libc::c_char;
        let fresh3 = length;
        length = length.wrapping_add(1);
        buff[fresh3 as usize] = last;
    }
    strcpy(
        buff.as_mut_ptr().offset(length as isize),
        if ps as libc::c_uint == CWK_STYLE_WINDOWS as libc::c_int as libc::c_uint {
            b"libs\\\0" as *const u8 as *const libc::c_char
        } else {
            b"libs/\0" as *const u8 as *const libc::c_char
        },
    );
    pkAddSearchPath(vm, buff.as_mut_ptr());
}
pub unsafe extern "C" fn registerModulePath(mut vm: *mut PKVM) {
    _registerSearchPaths(vm);
    let mut path: *mut PkHandle = pkNewModule(
        vm,
        b"path\0" as *const u8 as *const libc::c_char,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"getcwd\0" as *const u8 as *const libc::c_char,
        Some(_pathGetCWD as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__pathGetCWD,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"abspath\0" as *const u8 as *const libc::c_char,
        Some(_pathAbspath as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathAbspath,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"relpath\0" as *const u8 as *const libc::c_char,
        Some(_pathRelpath as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__pathRelpath,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"join\0" as *const u8 as *const libc::c_char,
        Some(_pathJoin as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__pathJoin,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"normpath\0" as *const u8 as *const libc::c_char,
        Some(_pathNormpath as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathNormpath,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"basename\0" as *const u8 as *const libc::c_char,
        Some(_pathBaseName as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathBaseName,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"dirname\0" as *const u8 as *const libc::c_char,
        Some(_pathDirName as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathDirName,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"isabspath\0" as *const u8 as *const libc::c_char,
        Some(_pathIsPathAbs as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathIsPathAbs,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"getext\0" as *const u8 as *const libc::c_char,
        Some(_pathGetExtension as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathGetExtension,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"exists\0" as *const u8 as *const libc::c_char,
        Some(_pathExists as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathExists,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"isfile\0" as *const u8 as *const libc::c_char,
        Some(_pathIsFile as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathIsFile,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"isdir\0" as *const u8 as *const libc::c_char,
        Some(_pathIsDir as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__pathIsDir,
    );
    pkModuleAddFunction(
        vm,
        path,
        b"listdir\0" as *const u8 as *const libc::c_char,
        Some(_pathListDir as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__pathListDir,
    );
    pkRegisterModule(vm, path);
    pkReleaseHandle(vm, path);
}
