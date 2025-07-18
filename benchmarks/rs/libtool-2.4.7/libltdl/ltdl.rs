use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut lt__alloc_die: Option::<unsafe extern "C" fn() -> ()>;
    fn lt__malloc(n: size_t) -> *mut libc::c_void;
    fn lt__zalloc(n: size_t) -> *mut libc::c_void;
    fn lt__realloc(mem: *mut libc::c_void, n: size_t) -> *mut libc::c_void;
    fn lt__strdup(string: *const libc::c_char) -> *mut libc::c_char;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn lt_strlcpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        dstsize: size_t,
    ) -> size_t;
    fn argz_create_sep(
        __string: *const libc::c_char,
        __sep: libc::c_int,
        __argz: *mut *mut libc::c_char,
        __len: *mut size_t,
    ) -> error_t;
    fn argz_stringify(__argz: *mut libc::c_char, __len: size_t, __sep: libc::c_int);
    fn argz_append(
        __argz: *mut *mut libc::c_char,
        __argz_len: *mut size_t,
        __buf: *const libc::c_char,
        __buf_len: size_t,
    ) -> error_t;
    fn argz_insert(
        __argz: *mut *mut libc::c_char,
        __argz_len: *mut size_t,
        __before: *mut libc::c_char,
        __entry: *const libc::c_char,
    ) -> error_t;
    fn lt_dlloader_add(vtable: *const lt_dlvtable) -> libc::c_int;
    fn lt_dlloader_next(loader: lt_dlloader) -> lt_dlloader;
    fn lt_dlloader_remove(name: *const libc::c_char) -> *mut lt_dlvtable;
    fn lt_dlloader_find(name: *const libc::c_char) -> *const lt_dlvtable;
    fn lt_dlloader_get(loader: lt_dlloader) -> *const lt_dlvtable;
    fn lt__set_last_error(errormsg: *const libc::c_char) -> *const libc::c_char;
    fn lt__get_last_error() -> *const libc::c_char;
    fn lt__error_string(errorcode: libc::c_int) -> *const libc::c_char;
    fn lt_dlpreload_open(
        originator: *const libc::c_char,
        func: Option::<lt_dlpreload_callback_func>,
    ) -> libc::c_int;
    fn lt_dlpreload(preloaded: *const lt_dlsymlist) -> libc::c_int;
    fn preopen_LTX_get_vtable(data: lt_user_data) -> *const lt_dlvtable;
    static lt_libltdl_LTX_preloaded_symbols: [lt_dlsymlist; 0];
}
pub type size_t = libc::c_ulong;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type error_t = libc::c_int;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LT_ERROR_MAX: C2RustUnnamed_0 = 20;
pub const LT_ERROR_CONFLICTING_FLAGS: C2RustUnnamed_0 = 19;
pub const LT_ERROR_INVALID_POSITION: C2RustUnnamed_0 = 18;
pub const LT_ERROR_INVALID_MUTEX_ARGS: C2RustUnnamed_0 = 17;
pub const LT_ERROR_CLOSE_RESIDENT_MODULE: C2RustUnnamed_0 = 16;
pub const LT_ERROR_SHUTDOWN: C2RustUnnamed_0 = 15;
pub const LT_ERROR_INVALID_ERRORCODE: C2RustUnnamed_0 = 14;
pub const LT_ERROR_BUFFER_OVERFLOW: C2RustUnnamed_0 = 13;
pub const LT_ERROR_INVALID_HANDLE: C2RustUnnamed_0 = 12;
pub const LT_ERROR_NO_MEMORY: C2RustUnnamed_0 = 11;
pub const LT_ERROR_SYMBOL_NOT_FOUND: C2RustUnnamed_0 = 10;
pub const LT_ERROR_CANNOT_CLOSE: C2RustUnnamed_0 = 9;
pub const LT_ERROR_CANNOT_OPEN: C2RustUnnamed_0 = 8;
pub const LT_ERROR_NO_SYMBOLS: C2RustUnnamed_0 = 7;
pub const LT_ERROR_DEPLIB_NOT_FOUND: C2RustUnnamed_0 = 6;
pub const LT_ERROR_FILE_NOT_FOUND: C2RustUnnamed_0 = 5;
pub const LT_ERROR_REMOVE_LOADER: C2RustUnnamed_0 = 4;
pub const LT_ERROR_INIT_LOADER: C2RustUnnamed_0 = 3;
pub const LT_ERROR_INVALID_LOADER: C2RustUnnamed_0 = 2;
pub const LT_ERROR_DLOPEN_NOT_SUPPORTED: C2RustUnnamed_0 = 1;
pub const LT_ERROR_UNKNOWN: C2RustUnnamed_0 = 0;
pub type lt_dlloader = *mut libc::c_void;
pub type lt_module = *mut libc::c_void;
pub type lt_user_data = *mut libc::c_void;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct lt__advise {
    #[bitfield(name = "try_ext", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_resident", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_symglobal", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "is_symlocal", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "try_preload_only", ty = "libc::c_uint", bits = "4..=4")]
    pub try_ext_is_resident_is_symglobal_is_symlocal_try_preload_only: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type lt_dladvise = *mut lt__advise;
pub type lt_module_open = unsafe extern "C" fn(
    lt_user_data,
    *const libc::c_char,
    lt_dladvise,
) -> lt_module;
pub type lt_module_close = unsafe extern "C" fn(lt_user_data, lt_module) -> libc::c_int;
pub type lt_find_sym = unsafe extern "C" fn(
    lt_user_data,
    lt_module,
    *const libc::c_char,
) -> *mut libc::c_void;
pub type lt_dlloader_init = unsafe extern "C" fn(lt_user_data) -> libc::c_int;
pub type lt_dlloader_exit = unsafe extern "C" fn(lt_user_data) -> libc::c_int;
pub type lt_dlloader_priority = libc::c_uint;
pub const LT_DLLOADER_APPEND: lt_dlloader_priority = 1;
pub const LT_DLLOADER_PREPEND: lt_dlloader_priority = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_dlvtable {
    pub name: *const libc::c_char,
    pub sym_prefix: *const libc::c_char,
    pub module_open: Option::<lt_module_open>,
    pub module_close: Option::<lt_module_close>,
    pub find_sym: Option::<lt_find_sym>,
    pub dlloader_init: Option::<lt_dlloader_init>,
    pub dlloader_exit: Option::<lt_dlloader_exit>,
    pub dlloader_data: lt_user_data,
    pub priority: lt_dlloader_priority,
}
pub type lt_get_vtable = unsafe extern "C" fn(lt_user_data) -> *const lt_dlvtable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt__handle {
    pub next: lt_dlhandle,
    pub vtable: *const lt_dlvtable,
    pub info: lt_dlinfo,
    pub depcount: libc::c_int,
    pub deplibs: *mut lt_dlhandle,
    pub module: lt_module,
    pub system: *mut libc::c_void,
    pub interface_data: *mut lt_interface_data,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_interface_data {
    pub key: lt_dlinterface_id,
    pub data: *mut libc::c_void,
}
pub type lt_dlinterface_id = *mut libc::c_void;
pub type lt_dlhandle = *mut lt__handle;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct lt_dlinfo {
    pub filename: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub ref_count: libc::c_int,
    #[bitfield(name = "is_resident", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_symglobal", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_symlocal", ty = "libc::c_uint", bits = "2..=2")]
    pub is_resident_is_symglobal_is_symlocal: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type lt_dlpreload_callback_func = unsafe extern "C" fn(lt_dlhandle) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_dlsymlist {
    pub name: *const libc::c_char,
    pub address: *mut libc::c_void,
}
pub type file_worker_func = unsafe extern "C" fn(
    *const libc::c_char,
    *mut libc::c_void,
) -> libc::c_int;
pub type foreach_callback_func = unsafe extern "C" fn(
    *mut libc::c_char,
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type lt_dlhandle_interface = unsafe extern "C" fn(
    lt_dlhandle,
    *const libc::c_char,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt__interface_id {
    pub id_string: *mut libc::c_char,
    pub iface: Option::<lt_dlhandle_interface>,
}
#[inline]
unsafe extern "C" fn __argz_next(
    mut __argz: *const libc::c_char,
    mut __argz_len: size_t,
    mut __entry: *const libc::c_char,
) -> *mut libc::c_char {
    if !__entry.is_null() {
        if __entry < __argz.offset(__argz_len as isize) {
            __entry = (strchr(__entry, '\0' as i32)).offset(1 as libc::c_int as isize);
        }
        return if __entry >= __argz.offset(__argz_len as isize) {
            0 as *mut libc::c_void as *mut libc::c_char
        } else {
            __entry as *mut libc::c_char
        };
    } else {
        return if __argz_len > 0 as libc::c_int as libc::c_ulong {
            __argz as *mut libc::c_char
        } else {
            0 as *mut libc::c_char
        }
    };
}
#[inline]
unsafe extern "C" fn argz_next(
    mut __argz: *const libc::c_char,
    mut __argz_len: size_t,
    mut __entry: *const libc::c_char,
) -> *mut libc::c_char {
    return __argz_next(__argz, __argz_len, __entry);
}
static mut objdir: [libc::c_char; 7] = unsafe {
    *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b".libs/\0")
};
static mut archive_ext: [libc::c_char; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b".la\0")
};
static mut libext: [libc::c_char; 2] = unsafe {
    *::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"a\0")
};
static mut libprefix: [libc::c_char; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"lib\0")
};
static mut shlib_ext: [libc::c_char; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b".so\0")
};
static mut sys_dlsearch_path: [libc::c_char; 165] = unsafe {
    *::std::mem::transmute::<
        &[u8; 165],
        &[libc::c_char; 165],
    >(
        b"/lib:/usr/lib:/usr/lib/x86_64-linux-gnu/libfakeroot:/usr/local/lib:/usr/local/lib/x86_64-linux-gnu:/lib/x86_64-linux-gnu:/usr/lib/x86_64-linux-gnu:/lib32:/usr/lib32\0",
    )
};
static mut user_search_path: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut handles: lt_dlhandle = 0 as *const lt__handle as lt_dlhandle;
static mut initialized: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn lt__alloc_die_callback() {
    lt__set_last_error(lt__error_string(LT_ERROR_NO_MEMORY as libc::c_int));
}
unsafe extern "C" fn loader_init_callback(mut handle: lt_dlhandle) -> libc::c_int {
    let mut vtable_func: Option::<lt_get_vtable> = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<lt_get_vtable>,
    >(lt_dlsym(handle, b"get_vtable\0" as *const u8 as *const libc::c_char));
    return loader_init(vtable_func, 0 as lt_user_data);
}
unsafe extern "C" fn loader_init(
    mut vtable_func: Option::<lt_get_vtable>,
    mut data: lt_user_data,
) -> libc::c_int {
    let mut vtable: *const lt_dlvtable = 0 as *const lt_dlvtable;
    let mut errors: libc::c_int = 0 as libc::c_int;
    if vtable_func.is_some() {
        vtable = (Some(vtable_func.unwrap())).unwrap()(data);
    }
    errors += lt_dlloader_add(vtable);
    if errors != 0 || !vtable.is_null() {} else {
        __assert_fail(
            b"errors || vtable\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int loader_init(lt_get_vtable *, lt_user_data)\0"))
                .as_ptr(),
        );
    }
    'c_4053: {
        if errors != 0 || !vtable.is_null() {} else {
            __assert_fail(
                b"errors || vtable\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int loader_init(lt_get_vtable *, lt_user_data)\0"))
                    .as_ptr(),
            );
        }
    };
    if errors == 0 && ((*vtable).dlloader_init).is_some() {
        if (Some(((*vtable).dlloader_init).unwrap())).unwrap()((*vtable).dlloader_data)
            != 0
        {
            lt__set_last_error(lt__error_string(LT_ERROR_INIT_LOADER as libc::c_int));
            errors += 1;
            errors;
        }
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlinit() -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    initialized += 1;
    if initialized == 1 as libc::c_int {
        lt__alloc_die = Some(lt__alloc_die_callback as unsafe extern "C" fn() -> ());
        handles = 0 as lt_dlhandle;
        user_search_path = 0 as *mut libc::c_char;
        errors
            += loader_init(
                Some(
                    preopen_LTX_get_vtable
                        as unsafe extern "C" fn(lt_user_data) -> *const lt_dlvtable,
                ),
                0 as lt_user_data,
            );
        if errors == 0 {
            errors += lt_dlpreload(lt_libltdl_LTX_preloaded_symbols.as_ptr());
        }
        if errors == 0 {
            errors
                += lt_dlpreload_open(
                    b"libltdl\0" as *const u8 as *const libc::c_char,
                    Some(
                        loader_init_callback
                            as unsafe extern "C" fn(lt_dlhandle) -> libc::c_int,
                    ),
                );
        }
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlexit() -> libc::c_int {
    let mut loader: *mut lt_dlloader = 0 as *mut lt_dlloader;
    let mut handle: lt_dlhandle = handles;
    let mut errors: libc::c_int = 0 as libc::c_int;
    if initialized == 0 {
        lt__set_last_error(lt__error_string(LT_ERROR_SHUTDOWN as libc::c_int));
        errors += 1;
        errors;
    } else {
        initialized -= 1;
        if initialized == 0 as libc::c_int {
            let mut level: libc::c_int = 0;
            while !handles.is_null()
                && ((*handles).info).is_resident() as libc::c_int != 0
            {
                handles = (*handles).next;
            }
            level = 1 as libc::c_int;
            while !handle.is_null() {
                let mut cur: lt_dlhandle = handles;
                let mut saw_nonresident: libc::c_int = 0 as libc::c_int;
                while !cur.is_null() {
                    let mut tmp: lt_dlhandle = cur;
                    cur = (*cur).next;
                    if ((*tmp).info).is_resident() == 0 {
                        saw_nonresident = 1 as libc::c_int;
                        if (*tmp).info.ref_count <= level {
                            if lt_dlclose(tmp) != 0 {
                                errors += 1;
                                errors;
                            }
                            if !cur.is_null() {
                                tmp = handles;
                                while !tmp.is_null() {
                                    if tmp == cur {
                                        break;
                                    }
                                    tmp = (*tmp).next;
                                }
                                if tmp.is_null() {
                                    cur = handles;
                                }
                            }
                        }
                    }
                }
                if saw_nonresident == 0 {
                    break;
                }
                level += 1;
                level;
            }
            if errors == 0 {
                lt__set_last_error(0 as *const libc::c_char);
            }
            loader = lt_dlloader_next(0 as *mut libc::c_void) as *mut lt_dlloader;
            while !loader.is_null() {
                let mut next: *mut lt_dlloader = lt_dlloader_next(loader as lt_dlloader)
                    as *mut lt_dlloader;
                let mut vtable: *mut lt_dlvtable = lt_dlloader_get(loader as lt_dlloader)
                    as *mut lt_dlvtable;
                vtable = lt_dlloader_remove((*vtable).name as *mut libc::c_char);
                if !vtable.is_null() {
                    ({
                        free(vtable as *mut libc::c_void);
                        vtable = 0 as *mut lt_dlvtable;
                        vtable
                    });
                } else {
                    let mut err: *const libc::c_char = 0 as *const libc::c_char;
                    err = lt__get_last_error();
                    if !err.is_null() {
                        errors += 1;
                        errors;
                    }
                }
                loader = next;
            }
            ({
                free(user_search_path as *mut libc::c_void);
                user_search_path = 0 as *mut libc::c_char;
                user_search_path
            });
        }
    }
    return errors;
}
unsafe extern "C" fn tryall_dlopen(
    mut phandle: *mut lt_dlhandle,
    mut filename: *const libc::c_char,
    mut advise: lt_dladvise,
    mut vtable: *const lt_dlvtable,
) -> libc::c_int {
    let mut current_block: u64;
    let mut handle: lt_dlhandle = handles;
    let mut saved_error: *const libc::c_char = 0 as *const libc::c_char;
    let mut errors: libc::c_int = 0 as libc::c_int;
    saved_error = lt__get_last_error();
    while !handle.is_null() {
        if (*handle).info.filename == filename as *mut libc::c_char
            || !((*handle).info.filename).is_null() && !filename.is_null()
                && strcmp((*handle).info.filename, filename) == 0 as libc::c_int
        {
            break;
        }
        handle = (*handle).next;
    }
    if !handle.is_null() {
        (*handle).info.ref_count += 1;
        (*handle).info.ref_count;
        *phandle = handle;
    } else {
        handle = *phandle;
        if !filename.is_null() {
            (*handle).info.filename = lt__strdup(filename);
            if ((*handle).info.filename).is_null() {
                errors += 1;
                errors;
                current_block = 17882793176858721997;
            } else {
                current_block = 12599329904712511516;
            }
        } else {
            (*handle).info.filename = 0 as *mut libc::c_char;
            current_block = 12599329904712511516;
        }
        match current_block {
            17882793176858721997 => {}
            _ => {
                let mut loader: lt_dlloader = lt_dlloader_next(0 as lt_dlloader);
                let mut loader_vtable: *const lt_dlvtable = 0 as *const lt_dlvtable;
                loop {
                    if !vtable.is_null() {
                        loader_vtable = vtable;
                    } else {
                        loader_vtable = lt_dlloader_get(loader);
                    }
                    (*handle)
                        .module = (Some(((*loader_vtable).module_open).unwrap()))
                        .unwrap()((*loader_vtable).dlloader_data, filename, advise);
                    if !((*handle).module).is_null() {
                        if !advise.is_null() {
                            ((*handle).info).set_is_resident((*advise).is_resident());
                            ((*handle).info).set_is_symglobal((*advise).is_symglobal());
                            ((*handle).info).set_is_symlocal((*advise).is_symlocal());
                        }
                        break;
                    } else if !(vtable.is_null()
                        && {
                            loader = lt_dlloader_next(loader);
                            !loader.is_null()
                        })
                    {
                        break;
                    }
                }
                if !vtable.is_null() && ((*handle).module).is_null()
                    || vtable.is_null() && loader.is_null()
                {
                    ({
                        free((*handle).info.filename as *mut libc::c_void);
                        (*handle).info.filename = 0 as *mut libc::c_char;
                        (*handle).info.filename
                    });
                    errors += 1;
                    errors;
                } else {
                    (*handle).vtable = loader_vtable;
                    lt__set_last_error(saved_error);
                }
            }
        }
    }
    return errors;
}
unsafe extern "C" fn tryall_dlopen_module(
    mut handle: *mut lt_dlhandle,
    mut prefix: *const libc::c_char,
    mut dirname: *const libc::c_char,
    mut dlname: *const libc::c_char,
    mut advise: lt_dladvise,
) -> libc::c_int {
    let mut error: libc::c_int = 0 as libc::c_int;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename_len: size_t = 0 as libc::c_int as size_t;
    let mut dirname_len: size_t = if !dirname.is_null()
        && *dirname.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        strlen(dirname)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if !handle.is_null() {} else {
        __assert_fail(
            b"handle\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            502 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"int tryall_dlopen_module(lt_dlhandle *, const char *, const char *, const char *, lt_dladvise)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9481: {
        if !handle.is_null() {} else {
            __assert_fail(
                b"handle\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                502 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"int tryall_dlopen_module(lt_dlhandle *, const char *, const char *, const char *, lt_dladvise)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !dirname.is_null() {} else {
        __assert_fail(
            b"dirname\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"int tryall_dlopen_module(lt_dlhandle *, const char *, const char *, const char *, lt_dladvise)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9449: {
        if !dirname.is_null() {} else {
            __assert_fail(
                b"dirname\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                503 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"int tryall_dlopen_module(lt_dlhandle *, const char *, const char *, const char *, lt_dladvise)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !dlname.is_null() {} else {
        __assert_fail(
            b"dlname\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            504 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"int tryall_dlopen_module(lt_dlhandle *, const char *, const char *, const char *, lt_dladvise)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9415: {
        if !dlname.is_null() {} else {
            __assert_fail(
                b"dlname\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                504 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"int tryall_dlopen_module(lt_dlhandle *, const char *, const char *, const char *, lt_dladvise)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dirname_len > 0 as libc::c_int as libc::c_ulong {
        if *dirname
            .offset(dirname_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32
        {
            dirname_len = dirname_len.wrapping_sub(1);
            dirname_len;
        }
    }
    filename_len = dirname_len
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (if !dlname.is_null()
                && *dlname.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                strlen(dlname)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        );
    filename = lt__malloc(
        filename_len
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if filename.is_null() {
        return 1 as libc::c_int;
    }
    sprintf(
        filename,
        b"%.*s/%s\0" as *const u8 as *const libc::c_char,
        dirname_len as libc::c_int,
        dirname,
        dlname,
    );
    if !prefix.is_null() {
        error
            += tryall_dlopen_module(
                handle,
                0 as *const libc::c_char,
                prefix,
                filename,
                advise,
            );
    } else if tryall_dlopen(handle, filename, advise, 0 as *const lt_dlvtable)
        != 0 as libc::c_int
    {
        error += 1;
        error;
    }
    ({
        free(filename as *mut libc::c_void);
        filename = 0 as *mut libc::c_char;
        filename
    });
    return error;
}
unsafe extern "C" fn find_module(
    mut handle: *mut lt_dlhandle,
    mut dir: *const libc::c_char,
    mut libdir: *const libc::c_char,
    mut dlname: *const libc::c_char,
    mut old_name: *const libc::c_char,
    mut installed: libc::c_int,
    mut advise: lt_dladvise,
) -> libc::c_int {
    if !old_name.is_null()
        && tryall_dlopen(
            handle,
            old_name,
            advise,
            lt_dlloader_find(b"lt_preopen\0" as *const u8 as *const libc::c_char),
        ) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !dlname.is_null() {
        if installed != 0 && !libdir.is_null() {
            if tryall_dlopen_module(
                handle,
                0 as *const libc::c_char,
                libdir,
                dlname,
                advise,
            ) == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        if installed == 0 {
            if tryall_dlopen_module(handle, dir, objdir.as_ptr(), dlname, advise)
                == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        if !dir.is_null()
            && tryall_dlopen_module(
                handle,
                0 as *const libc::c_char,
                dir,
                dlname,
                advise,
            ) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn canonicalize_path(
    mut path: *const libc::c_char,
    mut pcanonical: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut canonical: *mut libc::c_char = 0 as *mut libc::c_char;
    if !path.is_null() && *path as libc::c_int != 0 {} else {
        __assert_fail(
            b"path && *path\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            591 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"int canonicalize_path(const char *, char **)\0"))
                .as_ptr(),
        );
    }
    'c_5839: {
        if !path.is_null() && *path as libc::c_int != 0 {} else {
            __assert_fail(
                b"path && *path\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                591 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"int canonicalize_path(const char *, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pcanonical.is_null() {} else {
        __assert_fail(
            b"pcanonical\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            592 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"int canonicalize_path(const char *, char **)\0"))
                .as_ptr(),
        );
    }
    'c_5806: {
        if !pcanonical.is_null() {} else {
            __assert_fail(
                b"pcanonical\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                592 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"int canonicalize_path(const char *, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    canonical = lt__malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if !path.is_null()
                    && *path.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    strlen(path)
                } else {
                    0 as libc::c_int as libc::c_ulong
                }),
            )
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if canonical.is_null() {
        return 1 as libc::c_int;
    }
    let mut dest: size_t = 0 as libc::c_int as size_t;
    let mut src: size_t = 0;
    let mut current_block_11: u64;
    src = 0 as libc::c_int as size_t;
    while *path.offset(src as isize) as libc::c_int != '\0' as i32 {
        if *path.offset(src as isize) as libc::c_int == ':' as i32 {
            if dest == 0 as libc::c_int as libc::c_ulong
                || *path
                    .offset(
                        (1 as libc::c_int as libc::c_ulong).wrapping_add(src) as isize,
                    ) as libc::c_int == ':' as i32
                || *path
                    .offset(
                        (1 as libc::c_int as libc::c_ulong).wrapping_add(src) as isize,
                    ) as libc::c_int == '\0' as i32
            {
                current_block_11 = 7095457783677275021;
            } else {
                current_block_11 = 8515828400728868193;
            }
        } else {
            current_block_11 = 8515828400728868193;
        }
        match current_block_11 {
            8515828400728868193 => {
                if *path.offset(src as isize) as libc::c_int != '/' as i32 {
                    let fresh0 = dest;
                    dest = dest.wrapping_add(1);
                    *canonical.offset(fresh0 as isize) = *path.offset(src as isize);
                } else if *path
                    .offset(
                        (1 as libc::c_int as libc::c_ulong).wrapping_add(src) as isize,
                    ) as libc::c_int != ':' as i32
                    && *path
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add(src)
                                as isize,
                        ) as libc::c_int != '\0' as i32
                    && *path
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add(src)
                                as isize,
                        ) as libc::c_int != '/' as i32
                {
                    let fresh1 = dest;
                    dest = dest.wrapping_add(1);
                    *canonical.offset(fresh1 as isize) = '/' as i32 as libc::c_char;
                }
            }
            _ => {}
        }
        src = src.wrapping_add(1);
        src;
    }
    *canonical.offset(dest as isize) = '\0' as i32 as libc::c_char;
    *pcanonical = canonical;
    return 0 as libc::c_int;
}
unsafe extern "C" fn argzize_path(
    mut path: *const libc::c_char,
    mut pargz: *mut *mut libc::c_char,
    mut pargz_len: *mut size_t,
) -> libc::c_int {
    let mut error: error_t = 0;
    if !path.is_null() {} else {
        __assert_fail(
            b"path\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            652 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int argzize_path(const char *, char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5351: {
        if !path.is_null() {} else {
            __assert_fail(
                b"path\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                652 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int argzize_path(const char *, char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pargz.is_null() {} else {
        __assert_fail(
            b"pargz\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            653 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int argzize_path(const char *, char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5319: {
        if !pargz.is_null() {} else {
            __assert_fail(
                b"pargz\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                653 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int argzize_path(const char *, char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pargz_len.is_null() {} else {
        __assert_fail(
            b"pargz_len\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            654 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int argzize_path(const char *, char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5285: {
        if !pargz_len.is_null() {} else {
            __assert_fail(
                b"pargz_len\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                654 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"int argzize_path(const char *, char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    error = argz_create_sep(path, ':' as i32, pargz, pargz_len);
    if error != 0 {
        match error {
            12 => {
                lt__set_last_error(lt__error_string(LT_ERROR_NO_MEMORY as libc::c_int));
            }
            _ => {
                lt__set_last_error(lt__error_string(LT_ERROR_UNKNOWN as libc::c_int));
            }
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn foreach_dirinpath(
    mut search_path: *const libc::c_char,
    mut base_name: *const libc::c_char,
    mut func: Option::<foreach_callback_func>,
    mut data1: *mut libc::c_void,
    mut data2: *mut libc::c_void,
) -> libc::c_int {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut filenamesize: size_t = 0 as libc::c_int as size_t;
    let mut lenbase: size_t = if !base_name.is_null()
        && *base_name.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        strlen(base_name)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut argz_len: size_t = 0 as libc::c_int as size_t;
    let mut argz: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canonical: *mut libc::c_char = 0 as *mut libc::c_char;
    if search_path.is_null() || *search_path == 0 {
        lt__set_last_error(lt__error_string(LT_ERROR_FILE_NOT_FOUND as libc::c_int));
    } else if !(canonicalize_path(search_path, &mut canonical) != 0 as libc::c_int) {
        if !(argzize_path(canonical, &mut argz, &mut argz_len) != 0 as libc::c_int) {
            let mut dir_name: *mut libc::c_char = 0 as *mut libc::c_char;
            loop {
                dir_name = argz_next(argz, argz_len, dir_name);
                if dir_name.is_null() {
                    break;
                }
                let mut lendir: size_t = if !dir_name.is_null()
                    && *dir_name.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    strlen(dir_name)
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                if (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(lendir)
                    .wrapping_add(lenbase) >= filenamesize
                {
                    ({
                        free(filename as *mut libc::c_void);
                        filename = 0 as *mut libc::c_char;
                        filename
                    });
                    filenamesize = (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(lendir)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(lenbase);
                    filename = lt__malloc(
                        filenamesize
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_char;
                    if filename.is_null() {
                        break;
                    }
                }
                if filenamesize > lendir {} else {
                    __assert_fail(
                        b"filenamesize > lendir\0" as *const u8 as *const libc::c_char,
                        b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                        717 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 91],
                            &[libc::c_char; 91],
                        >(
                            b"int foreach_dirinpath(const char *, const char *, foreach_callback_func *, void *, void *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_7377: {
                    if filenamesize > lendir {} else {
                        __assert_fail(
                            b"filenamesize > lendir\0" as *const u8
                                as *const libc::c_char,
                            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                            717 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 91],
                                &[libc::c_char; 91],
                            >(
                                b"int foreach_dirinpath(const char *, const char *, foreach_callback_func *, void *, void *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                strcpy(filename, dir_name);
                if !base_name.is_null() && *base_name as libc::c_int != 0 {
                    if *filename
                        .offset(
                            lendir.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int != '/' as i32
                    {
                        let fresh2 = lendir;
                        lendir = lendir.wrapping_add(1);
                        *filename.offset(fresh2 as isize) = '/' as i32 as libc::c_char;
                    }
                    strcpy(filename.offset(lendir as isize), base_name);
                }
                result = (Some(func.unwrap())).unwrap()(filename, data1, data2);
                if result != 0 {
                    break;
                }
            }
        }
    }
    ({
        free(argz as *mut libc::c_void);
        argz = 0 as *mut libc::c_char;
        argz
    });
    ({
        free(canonical as *mut libc::c_void);
        canonical = 0 as *mut libc::c_char;
        canonical
    });
    ({
        free(filename as *mut libc::c_void);
        filename = 0 as *mut libc::c_char;
        filename
    });
    return result;
}
unsafe extern "C" fn find_file_callback(
    mut filename: *mut libc::c_char,
    mut data1: *mut libc::c_void,
    mut data2: *mut libc::c_void,
) -> libc::c_int {
    let mut pdir: *mut *mut libc::c_char = data1 as *mut *mut libc::c_char;
    let mut pfile: *mut *mut FILE = data2 as *mut *mut FILE;
    let mut is_done: libc::c_int = 0 as libc::c_int;
    if !filename.is_null() && *filename as libc::c_int != 0 {} else {
        __assert_fail(
            b"filename && *filename\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            752 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int find_file_callback(char *, void *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_10781: {
        if !filename.is_null() && *filename as libc::c_int != 0 {} else {
            __assert_fail(
                b"filename && *filename\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                752 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int find_file_callback(char *, void *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pdir.is_null() {} else {
        __assert_fail(
            b"pdir\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            753 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int find_file_callback(char *, void *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_10749: {
        if !pdir.is_null() {} else {
            __assert_fail(
                b"pdir\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                753 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int find_file_callback(char *, void *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pfile.is_null() {} else {
        __assert_fail(
            b"pfile\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            754 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int find_file_callback(char *, void *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_10717: {
        if !pfile.is_null() {} else {
            __assert_fail(
                b"pfile\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                754 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int find_file_callback(char *, void *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    *pfile = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if !(*pfile).is_null() {
        let mut dirend: *mut libc::c_char = strrchr(filename, '/' as i32);
        if dirend > filename {
            *dirend = '\0' as i32 as libc::c_char;
        }
        ({
            free(*pdir as *mut libc::c_void);
            *pdir = 0 as *mut libc::c_char;
            *pdir
        });
        *pdir = lt__strdup(filename);
        is_done = if (*pdir).is_null() { -(1 as libc::c_int) } else { 1 as libc::c_int };
    }
    return is_done;
}
unsafe extern "C" fn find_file(
    mut search_path: *const libc::c_char,
    mut base_name: *const libc::c_char,
    mut pdir: *mut *mut libc::c_char,
) -> *mut FILE {
    let mut file: *mut FILE = 0 as *mut FILE;
    foreach_dirinpath(
        search_path,
        base_name,
        Some(
            find_file_callback
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        pdir as *mut libc::c_void,
        &mut file as *mut *mut FILE as *mut libc::c_void,
    );
    return file;
}
unsafe extern "C" fn find_handle_callback(
    mut filename: *mut libc::c_char,
    mut data: *mut libc::c_void,
    mut data2: *mut libc::c_void,
) -> libc::c_int {
    let mut phandle: *mut lt_dlhandle = data as *mut lt_dlhandle;
    let mut notfound: libc::c_int = access(filename, 4 as libc::c_int);
    let mut advise: lt_dladvise = data2 as lt_dladvise;
    if notfound != 0 {
        return 0 as libc::c_int;
    }
    if tryall_dlopen(phandle, filename, advise, 0 as *const lt_dlvtable)
        != 0 as libc::c_int
    {
        *phandle = 0 as lt_dlhandle;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_handle(
    mut search_path: *const libc::c_char,
    mut base_name: *const libc::c_char,
    mut phandle: *mut lt_dlhandle,
    mut advise: lt_dladvise,
) -> *mut lt_dlhandle {
    if search_path.is_null() {
        return 0 as *mut lt_dlhandle;
    }
    if foreach_dirinpath(
        search_path,
        base_name,
        Some(
            find_handle_callback
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        phandle as *mut libc::c_void,
        advise as *mut libc::c_void,
    ) == 0
    {
        return 0 as *mut lt_dlhandle;
    }
    return phandle;
}
unsafe extern "C" fn load_deplibs(
    mut handle: lt_dlhandle,
    mut deplibs: *mut libc::c_char,
) -> libc::c_int {
    (*handle).depcount = 0 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn unload_deplibs(mut handle: lt_dlhandle) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut cur: lt_dlhandle = handle;
    if (*cur).depcount != 0 {
        i = 0 as libc::c_int;
        while i < (*cur).depcount {
            if ((**((*cur).deplibs).offset(i as isize)).info).is_resident() == 0 {
                errors += lt_dlclose(*((*cur).deplibs).offset(i as isize));
            }
            i += 1;
            i;
        }
        ({
            free((*cur).deplibs as *mut libc::c_void);
            (*cur).deplibs = 0 as *mut lt_dlhandle;
            (*cur).deplibs
        });
    }
    return errors;
}
unsafe extern "C" fn trim(
    mut dest: *mut *mut libc::c_char,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut end: *const libc::c_char = strrchr(str, '\'' as i32);
    let mut len: size_t = if !str.is_null()
        && *str.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        strlen(str)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    ({
        free(*dest as *mut libc::c_void);
        *dest = 0 as *mut libc::c_char;
        *dest
    });
    if end.is_null() || end == str {
        return 1 as libc::c_int;
    }
    if len > 3 as libc::c_int as libc::c_ulong
        && *str.offset(0 as libc::c_int as isize) as libc::c_int == '\'' as i32
    {
        tmp = lt__malloc(
            (end.offset_from(str) as libc::c_long as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        if tmp.is_null() {
            return 1 as libc::c_int;
        }
        memcpy(
            tmp as *mut libc::c_void,
            &*str.offset(1 as libc::c_int as isize) as *const libc::c_char
                as *const libc::c_void,
            (end.offset_from(str) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
        *tmp
            .offset(
                (end.offset_from(str) as libc::c_long - 1 as libc::c_int as libc::c_long)
                    as isize,
            ) = '\0' as i32 as libc::c_char;
        *dest = tmp;
    } else {
        *dest = 0 as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_dotla_file(
    mut file: *mut FILE,
    mut dlname: *mut *mut libc::c_char,
    mut libdir: *mut *mut libc::c_char,
    mut deplibs: *mut *mut libc::c_char,
    mut old_name: *mut *mut libc::c_char,
    mut installed: *mut libc::c_int,
) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut line_len: size_t = 2048 as libc::c_int as size_t;
    let mut line: *mut libc::c_char = lt__malloc(
        line_len.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if line.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_FILE_NOT_FOUND as libc::c_int));
        return 1 as libc::c_int;
    }
    's_20: while feof(file) == 0 {
        *line
            .offset(
                line_len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        if (fgets(line, line_len as libc::c_int, file)).is_null() {
            break;
        }
        while *line
            .offset(line_len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '\0' as i32
            && *line
                .offset(
                    line_len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int != '\n' as i32 && feof(file) == 0
        {
            line = lt__realloc(
                line as *mut libc::c_void,
                line_len
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            if line.is_null() {
                errors += 1;
                errors;
                break 's_20;
            } else {
                *line
                    .offset(
                        line_len
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\0' as i32 as libc::c_char;
                if (fgets(
                    &mut *line
                        .offset(
                            line_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ),
                    line_len as libc::c_int + 1 as libc::c_int,
                    file,
                ))
                    .is_null()
                {
                    break;
                }
                line_len = (line_len as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
        }
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || *line.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        {
            continue;
        }
        if strncmp(
            line,
            b"dlname=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            errors
                += trim(
                    dlname,
                    &mut *line
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
        } else if strncmp(
            line,
            b"old_library=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            errors
                += trim(
                    old_name,
                    &mut *line
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 13]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
        } else if strncmp(
            line,
            b"libdir=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            errors
                += trim(
                    libdir,
                    &mut *line
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
        } else if strncmp(
            line,
            b"dependency_libs=\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            errors
                += trim(
                    deplibs,
                    &mut *line
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 17]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
        } else if strcmp(line, b"installed=yes\n\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            *installed = 1 as libc::c_int;
        } else if strcmp(line, b"installed=no\n\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            *installed = 0 as libc::c_int;
        } else if (*dlname).is_null()
            && strncmp(
                line,
                b"library_names=\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            let mut last_libname: *mut libc::c_char = 0 as *mut libc::c_char;
            errors
                += trim(
                    dlname,
                    &mut *line
                        .offset(
                            (::std::mem::size_of::<[libc::c_char; 15]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
            if errors == 0 && !(*dlname).is_null()
                && {
                    last_libname = strrchr(*dlname, ' ' as i32);
                    !last_libname.is_null()
                }
            {
                last_libname = lt__strdup(
                    last_libname.offset(1 as libc::c_int as isize),
                );
                if last_libname.is_null() {
                    errors += 1;
                    errors;
                    break;
                } else if *dlname != last_libname {
                    free(*dlname as *mut libc::c_void);
                    *dlname = last_libname;
                    last_libname = 0 as *mut libc::c_char;
                }
            }
        }
        if errors != 0 {
            break;
        }
    }
    ({
        free(line as *mut libc::c_void);
        line = 0 as *mut libc::c_char;
        line
    });
    return errors;
}
unsafe extern "C" fn try_dlopen(
    mut phandle: *mut lt_dlhandle,
    mut filename: *const libc::c_char,
    mut ext: *const libc::c_char,
    mut advise: lt_dladvise,
) -> libc::c_int {
    let mut current_block: u64;
    let mut saved_error: *const libc::c_char = 0 as *const libc::c_char;
    let mut archive_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canonical: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut attempt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut newhandle: lt_dlhandle = 0 as *mut lt__handle;
    if !phandle.is_null() {} else {
        __assert_fail(
            b"phandle\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1172 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11721: {
        if !phandle.is_null() {} else {
            __assert_fail(
                b"phandle\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1172 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*phandle).is_null() {} else {
        __assert_fail(
            b"*phandle == 0\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11679: {
        if (*phandle).is_null() {} else {
            __assert_fail(
                b"*phandle == 0\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1173 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    saved_error = lt__get_last_error();
    if filename.is_null() {
        *phandle = lt__zalloc(::std::mem::size_of::<lt__handle>() as libc::c_ulong)
            as lt_dlhandle;
        if (*phandle).is_null() {
            return 1 as libc::c_int;
        }
        newhandle = *phandle;
        ((*newhandle).info).set_is_resident(1 as libc::c_int as libc::c_uint);
        if tryall_dlopen(
            &mut newhandle,
            0 as *const libc::c_char,
            advise,
            0 as *const lt_dlvtable,
        ) != 0 as libc::c_int
        {
            ({
                free(*phandle as *mut libc::c_void);
                *phandle = 0 as lt_dlhandle;
                *phandle
            });
            return 1 as libc::c_int;
        }
        current_block = 12000893622463060107;
    } else {
        if !filename.is_null() && *filename as libc::c_int != 0 {} else {
            __assert_fail(
                b"filename && *filename\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1204 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 71],
                    &[libc::c_char; 71],
                >(
                    b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_11550: {
            if !filename.is_null() && *filename as libc::c_int != 0 {} else {
                __assert_fail(
                    b"filename && *filename\0" as *const u8 as *const libc::c_char,
                    b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                    1204 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 71],
                        &[libc::c_char; 71],
                    >(
                        b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if !ext.is_null() {
            attempt = lt__malloc(
                (if !filename.is_null()
                    && *filename.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    strlen(filename)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_add(
                        (if !ext.is_null()
                            && *ext.offset(0 as libc::c_int as isize) as libc::c_int != 0
                        {
                            strlen(ext)
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        }),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            if attempt.is_null() {
                return 1 as libc::c_int;
            }
            sprintf(
                attempt,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                filename,
                ext,
            );
        } else {
            attempt = lt__strdup(filename);
            if attempt.is_null() {
                return 1 as libc::c_int;
            }
        }
        if canonicalize_path(attempt, &mut canonical) != 0 as libc::c_int {
            errors += 1;
            errors;
            current_block = 6153539693549283295;
        } else {
            base_name = strrchr(canonical, '/' as i32);
            if !base_name.is_null() {
                let mut dirlen: size_t = base_name
                    .offset(1 as libc::c_int as isize)
                    .offset_from(canonical) as libc::c_long as size_t;
                dir = lt__malloc(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(dirlen)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_char;
                if dir.is_null() {
                    errors += 1;
                    errors;
                    current_block = 6153539693549283295;
                } else {
                    lt_strlcpy(dir, canonical, dirlen);
                    *dir.offset(dirlen as isize) = '\0' as i32 as libc::c_char;
                    base_name = base_name.offset(1);
                    base_name;
                    current_block = 7205609094909031804;
                }
            } else {
                if base_name != canonical {
                    free(base_name as *mut libc::c_void);
                    base_name = canonical;
                    canonical = 0 as *mut libc::c_char;
                }
                current_block = 7205609094909031804;
            }
            match current_block {
                6153539693549283295 => {}
                _ => {
                    if !base_name.is_null() && *base_name as libc::c_int != 0 {} else {
                        __assert_fail(
                            b"base_name && *base_name\0" as *const u8
                                as *const libc::c_char,
                            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                            1251 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 71],
                                &[libc::c_char; 71],
                            >(
                                b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_11295: {
                        if !base_name.is_null() && *base_name as libc::c_int != 0
                        {} else {
                            __assert_fail(
                                b"base_name && *base_name\0" as *const u8
                                    as *const libc::c_char,
                                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                                1251 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 71],
                                    &[libc::c_char; 71],
                                >(
                                    b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    ext = strrchr(base_name, '.' as i32);
                    if ext.is_null() {
                        ext = base_name
                            .offset(
                                (if !base_name.is_null()
                                    && *base_name.offset(0 as libc::c_int as isize)
                                        as libc::c_int != 0
                                {
                                    strlen(base_name)
                                } else {
                                    0 as libc::c_int as libc::c_ulong
                                }) as isize,
                            );
                    }
                    name = lt__malloc(
                        ((ext.offset_from(base_name) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_char;
                    if name.is_null() {
                        errors += 1;
                        errors;
                        current_block = 6153539693549283295;
                    } else {
                        let mut i: libc::c_int = 0;
                        i = 0 as libc::c_int;
                        while (i as libc::c_long)
                            < ext.offset_from(base_name) as libc::c_long
                        {
                            if *(*__ctype_b_loc())
                                .offset(
                                    *base_name.offset(i as isize) as libc::c_uchar
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                *name.offset(i as isize) = *base_name.offset(i as isize);
                            } else {
                                *name.offset(i as isize) = '_' as i32 as libc::c_char;
                            }
                            i += 1;
                            i;
                        }
                        *name
                            .offset(
                                ext.offset_from(base_name) as libc::c_long as isize,
                            ) = '\0' as i32 as libc::c_char;
                        if dir.is_null() {
                            let mut vtable: *const lt_dlvtable = lt_dlloader_find(
                                b"lt_preopen\0" as *const u8 as *const libc::c_char,
                            );
                            if !vtable.is_null() {
                                archive_name = lt__malloc(
                                    (strlen(libprefix.as_ptr()))
                                        .wrapping_add(
                                            (if !name.is_null()
                                                && *name.offset(0 as libc::c_int as isize) as libc::c_int
                                                    != 0
                                            {
                                                strlen(name)
                                            } else {
                                                0 as libc::c_int as libc::c_ulong
                                            }),
                                        )
                                        .wrapping_add(strlen(libext.as_ptr()))
                                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ) as *mut libc::c_char;
                                *phandle = lt__zalloc(
                                    ::std::mem::size_of::<lt__handle>() as libc::c_ulong,
                                ) as lt_dlhandle;
                                if (*phandle).is_null() || archive_name.is_null() {
                                    errors += 1;
                                    errors;
                                    current_block = 6153539693549283295;
                                } else {
                                    newhandle = *phandle;
                                    if strncmp(
                                        name,
                                        b"lib\0" as *const u8 as *const libc::c_char,
                                        3 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int
                                    {
                                        sprintf(
                                            archive_name,
                                            b"%s%s.%s\0" as *const u8 as *const libc::c_char,
                                            libprefix.as_ptr(),
                                            name.offset(3 as libc::c_int as isize),
                                            libext.as_ptr(),
                                        );
                                    } else {
                                        sprintf(
                                            archive_name,
                                            b"%s.%s\0" as *const u8 as *const libc::c_char,
                                            name,
                                            libext.as_ptr(),
                                        );
                                    }
                                    if tryall_dlopen(
                                        &mut newhandle,
                                        archive_name,
                                        advise,
                                        vtable,
                                    ) == 0 as libc::c_int
                                    {
                                        current_block = 12000893622463060107;
                                    } else {
                                        ({
                                            free(*phandle as *mut libc::c_void);
                                            *phandle = 0 as lt_dlhandle;
                                            *phandle
                                        });
                                        newhandle = 0 as lt_dlhandle;
                                        current_block = 9859671972921157070;
                                    }
                                }
                            } else {
                                current_block = 9859671972921157070;
                            }
                        } else {
                            current_block = 9859671972921157070;
                        }
                        match current_block {
                            6153539693549283295 => {}
                            12000893622463060107 => {}
                            _ => {
                                if !advise.is_null()
                                    && (*advise).try_preload_only() as libc::c_int != 0
                                {
                                    current_block = 6153539693549283295;
                                } else if !ext.is_null()
                                    && strcmp(ext, archive_ext.as_ptr()) == 0 as libc::c_int
                                {
                                    let mut file: *mut FILE = 0 as *mut FILE;
                                    let mut dlname: *mut libc::c_char = 0 as *mut libc::c_char;
                                    let mut old_name: *mut libc::c_char = 0
                                        as *mut libc::c_char;
                                    let mut libdir: *mut libc::c_char = 0 as *mut libc::c_char;
                                    let mut deplibs: *mut libc::c_char = 0 as *mut libc::c_char;
                                    let mut installed: libc::c_int = 1 as libc::c_int;
                                    if dir.is_null() {
                                        let mut search_path: *const libc::c_char = user_search_path;
                                        if !search_path.is_null() {
                                            file = find_file(user_search_path, base_name, &mut dir);
                                        }
                                        if file.is_null() {
                                            search_path = getenv(
                                                b"LTDL_LIBRARY_PATH\0" as *const u8 as *const libc::c_char,
                                            );
                                            if !search_path.is_null() {
                                                file = find_file(search_path, base_name, &mut dir);
                                            }
                                        }
                                        if file.is_null() {
                                            search_path = getenv(
                                                b"LD_LIBRARY_PATH\0" as *const u8 as *const libc::c_char,
                                            );
                                            if !search_path.is_null() {
                                                file = find_file(search_path, base_name, &mut dir);
                                            }
                                        }
                                        if file.is_null()
                                            && *sys_dlsearch_path.as_ptr() as libc::c_int != 0
                                        {
                                            file = find_file(
                                                sys_dlsearch_path.as_ptr(),
                                                base_name,
                                                &mut dir,
                                            );
                                        }
                                    } else {
                                        file = fopen(
                                            attempt,
                                            b"r\0" as *const u8 as *const libc::c_char,
                                        );
                                    }
                                    if file.is_null() {
                                        lt__set_last_error(
                                            lt__error_string(LT_ERROR_FILE_NOT_FOUND as libc::c_int),
                                        );
                                        errors += 1;
                                        errors;
                                        current_block = 6153539693549283295;
                                    } else {
                                        if parse_dotla_file(
                                            file,
                                            &mut dlname,
                                            &mut libdir,
                                            &mut deplibs,
                                            &mut old_name,
                                            &mut installed,
                                        ) != 0 as libc::c_int
                                        {
                                            errors += 1;
                                            errors;
                                        }
                                        fclose(file);
                                        *phandle = lt__zalloc(
                                            ::std::mem::size_of::<lt__handle>() as libc::c_ulong,
                                        ) as lt_dlhandle;
                                        if (*phandle).is_null() {
                                            errors += 1;
                                            errors;
                                        }
                                        if errors != 0 {
                                            ({
                                                free(dlname as *mut libc::c_void);
                                                dlname = 0 as *mut libc::c_char;
                                                dlname
                                            });
                                            ({
                                                free(old_name as *mut libc::c_void);
                                                old_name = 0 as *mut libc::c_char;
                                                old_name
                                            });
                                            ({
                                                free(libdir as *mut libc::c_void);
                                                libdir = 0 as *mut libc::c_char;
                                                libdir
                                            });
                                            ({
                                                free(deplibs as *mut libc::c_void);
                                                deplibs = 0 as *mut libc::c_char;
                                                deplibs
                                            });
                                            ({
                                                free(*phandle as *mut libc::c_void);
                                                *phandle = 0 as lt_dlhandle;
                                                *phandle
                                            });
                                            current_block = 6153539693549283295;
                                        } else {
                                            if !(*phandle).is_null() {} else {
                                                __assert_fail(
                                                    b"*phandle\0" as *const u8 as *const libc::c_char,
                                                    b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                                                    1417 as libc::c_int as libc::c_uint,
                                                    (*::std::mem::transmute::<
                                                        &[u8; 71],
                                                        &[libc::c_char; 71],
                                                    >(
                                                        b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                                                    ))
                                                        .as_ptr(),
                                                );
                                            }
                                            'c_9617: {
                                                if !(*phandle).is_null() {} else {
                                                    __assert_fail(
                                                        b"*phandle\0" as *const u8 as *const libc::c_char,
                                                        b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                                                        1417 as libc::c_int as libc::c_uint,
                                                        (*::std::mem::transmute::<
                                                            &[u8; 71],
                                                            &[libc::c_char; 71],
                                                        >(
                                                            b"int try_dlopen(lt_dlhandle *, const char *, const char *, lt_dladvise)\0",
                                                        ))
                                                            .as_ptr(),
                                                    );
                                                }
                                            };
                                            if load_deplibs(*phandle, deplibs) == 0 as libc::c_int {
                                                newhandle = *phandle;
                                                if find_module(
                                                    &mut newhandle,
                                                    dir,
                                                    libdir,
                                                    dlname,
                                                    old_name,
                                                    installed,
                                                    advise,
                                                ) != 0
                                                {
                                                    unload_deplibs(*phandle);
                                                    errors += 1;
                                                    errors;
                                                }
                                            } else {
                                                errors += 1;
                                                errors;
                                            }
                                            ({
                                                free(dlname as *mut libc::c_void);
                                                dlname = 0 as *mut libc::c_char;
                                                dlname
                                            });
                                            ({
                                                free(old_name as *mut libc::c_void);
                                                old_name = 0 as *mut libc::c_char;
                                                old_name
                                            });
                                            ({
                                                free(libdir as *mut libc::c_void);
                                                libdir = 0 as *mut libc::c_char;
                                                libdir
                                            });
                                            ({
                                                free(deplibs as *mut libc::c_void);
                                                deplibs = 0 as *mut libc::c_char;
                                                deplibs
                                            });
                                            if errors != 0 {
                                                ({
                                                    free(*phandle as *mut libc::c_void);
                                                    *phandle = 0 as lt_dlhandle;
                                                    *phandle
                                                });
                                                current_block = 6153539693549283295;
                                            } else {
                                                if *phandle != newhandle {
                                                    unload_deplibs(*phandle);
                                                }
                                                current_block = 12000893622463060107;
                                            }
                                        }
                                    }
                                } else {
                                    *phandle = lt__zalloc(
                                        ::std::mem::size_of::<lt__handle>() as libc::c_ulong,
                                    ) as lt_dlhandle;
                                    if (*phandle).is_null() {
                                        errors += 1;
                                        errors;
                                        current_block = 6153539693549283295;
                                    } else {
                                        newhandle = *phandle;
                                        if !dir.is_null()
                                            || (find_handle(
                                                user_search_path,
                                                base_name,
                                                &mut newhandle,
                                                advise,
                                            ))
                                                .is_null()
                                                && (find_handle(
                                                    getenv(
                                                        b"LTDL_LIBRARY_PATH\0" as *const u8 as *const libc::c_char,
                                                    ),
                                                    base_name,
                                                    &mut newhandle,
                                                    advise,
                                                ))
                                                    .is_null()
                                                && (find_handle(
                                                    getenv(
                                                        b"LD_LIBRARY_PATH\0" as *const u8 as *const libc::c_char,
                                                    ),
                                                    base_name,
                                                    &mut newhandle,
                                                    advise,
                                                ))
                                                    .is_null()
                                                && (find_handle(
                                                    sys_dlsearch_path.as_ptr(),
                                                    base_name,
                                                    &mut newhandle,
                                                    advise,
                                                ))
                                                    .is_null()
                                        {
                                            if tryall_dlopen(
                                                &mut newhandle,
                                                attempt,
                                                advise,
                                                0 as *const lt_dlvtable,
                                            ) != 0 as libc::c_int
                                            {
                                                newhandle = 0 as lt_dlhandle;
                                            }
                                        }
                                        if newhandle.is_null() {
                                            ({
                                                free(*phandle as *mut libc::c_void);
                                                *phandle = 0 as lt_dlhandle;
                                                *phandle
                                            });
                                            errors += 1;
                                            errors;
                                            current_block = 6153539693549283295;
                                        } else {
                                            current_block = 12000893622463060107;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        12000893622463060107 => {
            if *phandle != newhandle {
                free(*phandle as *mut libc::c_void);
                *phandle = newhandle;
                newhandle = 0 as lt_dlhandle;
            }
            if (**phandle).info.ref_count == 0 as libc::c_int {
                (**phandle).info.ref_count = 1 as libc::c_int;
                if (**phandle).info.name != name {
                    free((**phandle).info.name as *mut libc::c_void);
                    (**phandle).info.name = name;
                    name = 0 as *mut libc::c_char;
                }
                (**phandle).next = handles;
                handles = *phandle;
            }
            lt__set_last_error(saved_error);
        }
        _ => {}
    }
    ({
        free(dir as *mut libc::c_void);
        dir = 0 as *mut libc::c_char;
        dir
    });
    ({
        free(attempt as *mut libc::c_void);
        attempt = 0 as *mut libc::c_char;
        attempt
    });
    ({
        free(name as *mut libc::c_void);
        name = 0 as *mut libc::c_char;
        name
    });
    if canonical.is_null() {
        ({
            free(base_name as *mut libc::c_void);
            base_name = 0 as *mut libc::c_char;
            base_name
        });
    }
    ({
        free(canonical as *mut libc::c_void);
        canonical = 0 as *mut libc::c_char;
        canonical
    });
    ({
        free(archive_name as *mut libc::c_void);
        archive_name = 0 as *mut libc::c_char;
        archive_name
    });
    return errors;
}
unsafe extern "C" fn file_not_found() -> libc::c_int {
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    error = lt__get_last_error();
    if error == lt__error_string(LT_ERROR_FILE_NOT_FOUND as libc::c_int) {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn has_library_ext(mut filename: *const libc::c_char) -> libc::c_int {
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    if !filename.is_null() {} else {
        __assert_fail(
            b"filename\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1544 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"int has_library_ext(const char *)\0"))
                .as_ptr(),
        );
    }
    'c_11882: {
        if !filename.is_null() {} else {
            __assert_fail(
                b"filename\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1544 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"int has_library_ext(const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    ext = strrchr(filename, '.' as i32);
    if !ext.is_null()
        && (strcmp(ext, archive_ext.as_ptr()) == 0 as libc::c_int
            || strcmp(ext, shlib_ext.as_ptr()) == 0 as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dladvise_init(mut padvise: *mut lt_dladvise) -> libc::c_int {
    let mut advise: lt_dladvise = lt__zalloc(
        ::std::mem::size_of::<lt__advise>() as libc::c_ulong,
    ) as lt_dladvise;
    *padvise = advise;
    return if !advise.is_null() { 0 as libc::c_int } else { 1 as libc::c_int };
}
pub unsafe extern "C" fn lt_dladvise_destroy(
    mut padvise: *mut lt_dladvise,
) -> libc::c_int {
    if !padvise.is_null() {
        ({
            free(*padvise as *mut libc::c_void);
            *padvise = 0 as lt_dladvise;
            *padvise
        });
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dladvise_ext(mut padvise: *mut lt_dladvise) -> libc::c_int {
    if !padvise.is_null() && !(*padvise).is_null() {} else {
        __assert_fail(
            b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1585 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int lt_dladvise_ext(lt_dladvise *)\0"))
                .as_ptr(),
        );
    }
    'c_7729: {
        if !padvise.is_null() && !(*padvise).is_null() {} else {
            __assert_fail(
                b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1585 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int lt_dladvise_ext(lt_dladvise *)\0"))
                    .as_ptr(),
            );
        }
    };
    (**padvise).set_try_ext(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dladvise_resident(
    mut padvise: *mut lt_dladvise,
) -> libc::c_int {
    if !padvise.is_null() && !(*padvise).is_null() {} else {
        __assert_fail(
            b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1593 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"int lt_dladvise_resident(lt_dladvise *)\0"))
                .as_ptr(),
        );
    }
    'c_7789: {
        if !padvise.is_null() && !(*padvise).is_null() {} else {
            __assert_fail(
                b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1593 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"int lt_dladvise_resident(lt_dladvise *)\0"))
                    .as_ptr(),
            );
        }
    };
    (**padvise).set_is_resident(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dladvise_local(
    mut padvise: *mut lt_dladvise,
) -> libc::c_int {
    if !padvise.is_null() && !(*padvise).is_null() {} else {
        __assert_fail(
            b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1601 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"int lt_dladvise_local(lt_dladvise *)\0"))
                .as_ptr(),
        );
    }
    'c_7848: {
        if !padvise.is_null() && !(*padvise).is_null() {} else {
            __assert_fail(
                b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1601 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"int lt_dladvise_local(lt_dladvise *)\0"))
                    .as_ptr(),
            );
        }
    };
    (**padvise).set_is_symlocal(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dladvise_global(
    mut padvise: *mut lt_dladvise,
) -> libc::c_int {
    if !padvise.is_null() && !(*padvise).is_null() {} else {
        __assert_fail(
            b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1609 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int lt_dladvise_global(lt_dladvise *)\0"))
                .as_ptr(),
        );
    }
    'c_7907: {
        if !padvise.is_null() && !(*padvise).is_null() {} else {
            __assert_fail(
                b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1609 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int lt_dladvise_global(lt_dladvise *)\0"))
                    .as_ptr(),
            );
        }
    };
    (**padvise).set_is_symglobal(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dladvise_preload(
    mut padvise: *mut lt_dladvise,
) -> libc::c_int {
    if !padvise.is_null() && !(*padvise).is_null() {} else {
        __assert_fail(
            b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1617 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"int lt_dladvise_preload(lt_dladvise *)\0"))
                .as_ptr(),
        );
    }
    'c_7966: {
        if !padvise.is_null() && !(*padvise).is_null() {} else {
            __assert_fail(
                b"padvise && *padvise\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1617 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"int lt_dladvise_preload(lt_dladvise *)\0"))
                    .as_ptr(),
            );
        }
    };
    (**padvise).set_try_preload_only(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dlopen(mut filename: *const libc::c_char) -> lt_dlhandle {
    return lt_dlopenadvise(filename, 0 as lt_dladvise);
}
pub unsafe extern "C" fn lt_dlopenext(mut filename: *const libc::c_char) -> lt_dlhandle {
    let mut handle: lt_dlhandle = 0 as lt_dlhandle;
    let mut advise: lt_dladvise = 0 as *mut lt__advise;
    if lt_dladvise_init(&mut advise) == 0 && lt_dladvise_ext(&mut advise) == 0 {
        handle = lt_dlopenadvise(filename, advise);
    }
    lt_dladvise_destroy(&mut advise);
    return handle;
}
pub unsafe extern "C" fn lt_dlopenadvise(
    mut filename: *const libc::c_char,
    mut advise: lt_dladvise,
) -> lt_dlhandle {
    let mut handle: lt_dlhandle = 0 as lt_dlhandle;
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut saved_error: *const libc::c_char = 0 as *const libc::c_char;
    saved_error = lt__get_last_error();
    if !advise.is_null() && (*advise).is_symlocal() as libc::c_int != 0
        && (*advise).is_symglobal() as libc::c_int != 0
    {
        lt__set_last_error(lt__error_string(LT_ERROR_CONFLICTING_FLAGS as libc::c_int));
        return 0 as lt_dlhandle;
    }
    if filename.is_null() || advise.is_null() || (*advise).try_ext() == 0
        || has_library_ext(filename) != 0
    {
        if try_dlopen(&mut handle, filename, 0 as *const libc::c_char, advise)
            != 0 as libc::c_int
        {
            return 0 as lt_dlhandle;
        }
        return handle;
    } else if !filename.is_null() && *filename as libc::c_int != 0 {
        errors += try_dlopen(&mut handle, filename, archive_ext.as_ptr(), advise);
        if !handle.is_null() || errors > 0 as libc::c_int && file_not_found() == 0 {
            return handle;
        }
        lt__set_last_error(saved_error);
        errors = try_dlopen(&mut handle, filename, shlib_ext.as_ptr(), advise);
        if !handle.is_null() || errors > 0 as libc::c_int && file_not_found() == 0 {
            return handle;
        }
    }
    lt__set_last_error(lt__error_string(LT_ERROR_FILE_NOT_FOUND as libc::c_int));
    return 0 as lt_dlhandle;
}
unsafe extern "C" fn lt_argz_insert(
    mut pargz: *mut *mut libc::c_char,
    mut pargz_len: *mut size_t,
    mut before: *mut libc::c_char,
    mut entry: *const libc::c_char,
) -> libc::c_int {
    let mut error: error_t = 0;
    if !before.is_null() {
        error = argz_insert(pargz, pargz_len, before, entry);
    } else {
        error = argz_append(
            pargz,
            pargz_len,
            entry,
            (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(entry)),
        );
    }
    if error != 0 {
        match error {
            12 => {
                lt__set_last_error(lt__error_string(LT_ERROR_NO_MEMORY as libc::c_int));
            }
            _ => {
                lt__set_last_error(lt__error_string(LT_ERROR_UNKNOWN as libc::c_int));
            }
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lt_argz_insertinorder(
    mut pargz: *mut *mut libc::c_char,
    mut pargz_len: *mut size_t,
    mut entry: *const libc::c_char,
) -> libc::c_int {
    let mut before: *mut libc::c_char = 0 as *mut libc::c_char;
    if !pargz.is_null() {} else {
        __assert_fail(
            b"pargz\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1755 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int lt_argz_insertinorder(char **, size_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6575: {
        if !pargz.is_null() {} else {
            __assert_fail(
                b"pargz\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1755 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int lt_argz_insertinorder(char **, size_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pargz_len.is_null() {} else {
        __assert_fail(
            b"pargz_len\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1756 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int lt_argz_insertinorder(char **, size_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6543: {
        if !pargz_len.is_null() {} else {
            __assert_fail(
                b"pargz_len\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1756 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int lt_argz_insertinorder(char **, size_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !entry.is_null() && *entry as libc::c_int != 0 {} else {
        __assert_fail(
            b"entry && *entry\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1757 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"int lt_argz_insertinorder(char **, size_t *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_6498: {
        if !entry.is_null() && *entry as libc::c_int != 0 {} else {
            __assert_fail(
                b"entry && *entry\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1757 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"int lt_argz_insertinorder(char **, size_t *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(*pargz).is_null() {
        loop {
            before = argz_next(*pargz, *pargz_len, before);
            if before.is_null() {
                break;
            }
            let mut cmp: libc::c_int = strcmp(entry, before);
            if cmp < 0 as libc::c_int {
                break;
            }
            if cmp == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
    }
    return lt_argz_insert(pargz, pargz_len, before, entry);
}
unsafe extern "C" fn lt_argz_insertdir(
    mut pargz: *mut *mut libc::c_char,
    mut pargz_len: *mut size_t,
    mut dirnam: *const libc::c_char,
    mut dp: *mut dirent,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_len: size_t = 0 as libc::c_int as size_t;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_offset: size_t = 0 as libc::c_int as size_t;
    let mut dir_len: size_t = 0 as libc::c_int as size_t;
    let mut errors: libc::c_int = 0 as libc::c_int;
    if !pargz.is_null() {} else {
        __assert_fail(
            b"pargz\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1782 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6917: {
        if !pargz.is_null() {} else {
            __assert_fail(
                b"pargz\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1782 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !pargz_len.is_null() {} else {
        __assert_fail(
            b"pargz_len\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1783 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6885: {
        if !pargz_len.is_null() {} else {
            __assert_fail(
                b"pargz_len\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1783 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !dp.is_null() {} else {
        __assert_fail(
            b"dp\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1784 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6852: {
        if !dp.is_null() {} else {
            __assert_fail(
                b"dp\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1784 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    dir_len = if !dirnam.is_null()
        && *dirnam.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        strlen(dirnam)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    end = ((*dp).d_name)
        .as_mut_ptr()
        .offset(strlen(((*dp).d_name).as_mut_ptr()) as isize);
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = end;
    while p.offset(-(1 as libc::c_int as isize)) > ((*dp).d_name).as_mut_ptr() {
        if (strchr(
            b".0123456789\0" as *const u8 as *const libc::c_char,
            *p.offset(-(1 as libc::c_int) as isize) as libc::c_int,
        ))
            .is_null()
        {
            break;
        }
        p = p.offset(-1);
        p;
    }
    if *p as libc::c_int == '.' as i32 {
        end = p;
    }
    let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
    p_0 = end.offset(-(1 as libc::c_int as isize));
    while p_0 > ((*dp).d_name).as_mut_ptr() {
        if *p_0 as libc::c_int == '.' as i32 {
            end = p_0;
            break;
        } else {
            p_0 = p_0.offset(-1);
            p_0;
        }
    }
    end_offset = end.offset_from(((*dp).d_name).as_mut_ptr()) as libc::c_long as size_t;
    buf_len = dir_len
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(end_offset);
    buf = lt__malloc(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(buf_len)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if buf.is_null() {
        errors += 1;
        return errors;
    }
    if !buf.is_null() {} else {
        __assert_fail(
            b"buf\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1818 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6649: {
        if !buf.is_null() {} else {
            __assert_fail(
                b"buf\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1818 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 72],
                    &[libc::c_char; 72],
                >(
                    b"int lt_argz_insertdir(char **, size_t *, const char *, struct dirent *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    strcpy(buf, dirnam);
    strcat(buf, b"/\0" as *const u8 as *const libc::c_char);
    strncat(buf, ((*dp).d_name).as_mut_ptr(), end_offset);
    *buf.offset(buf_len as isize) = '\0' as i32 as libc::c_char;
    if lt_argz_insertinorder(pargz, pargz_len, buf) != 0 as libc::c_int {
        errors += 1;
        errors;
    }
    ({
        free(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
        buf
    });
    return errors;
}
unsafe extern "C" fn list_files_by_dir(
    mut dirnam: *const libc::c_char,
    mut pargz: *mut *mut libc::c_char,
    mut pargz_len: *mut size_t,
) -> libc::c_int {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut errors: libc::c_int = 0 as libc::c_int;
    if !dirnam.is_null() && *dirnam as libc::c_int != 0 {} else {
        __assert_fail(
            b"dirnam && *dirnam\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1840 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7130: {
        if !dirnam.is_null() && *dirnam as libc::c_int != 0 {} else {
            __assert_fail(
                b"dirnam && *dirnam\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1840 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pargz.is_null() {} else {
        __assert_fail(
            b"pargz\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1841 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7098: {
        if !pargz.is_null() {} else {
            __assert_fail(
                b"pargz\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1841 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !pargz_len.is_null() {} else {
        __assert_fail(
            b"pargz_len\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1842 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7066: {
        if !pargz_len.is_null() {} else {
            __assert_fail(
                b"pargz_len\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1842 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if *dirnam
        .offset(
            (if !dirnam.is_null()
                && *dirnam.offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                strlen(dirnam)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int != '/' as i32
    {} else {
        __assert_fail(
            b"dirnam[LT_STRLEN(dirnam) -1] != '/'\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            1843 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6975: {
        if *dirnam
            .offset(
                (if !dirnam.is_null()
                    && *dirnam.offset(0 as libc::c_int as isize) as libc::c_int != 0
                {
                    strlen(dirnam)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int != '/' as i32
        {} else {
            __assert_fail(
                b"dirnam[LT_STRLEN(dirnam) -1] != '/'\0" as *const u8
                    as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                1843 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int list_files_by_dir(const char *, char **, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    dirp = opendir(dirnam);
    if !dirp.is_null() {
        let mut dp: *mut dirent = 0 as *mut dirent;
        loop {
            dp = readdir(dirp);
            if dp.is_null() {
                break;
            }
            if !((*dp).d_name[0 as libc::c_int as usize] as libc::c_int != '.' as i32) {
                continue;
            }
            if !(lt_argz_insertdir(pargz, pargz_len, dirnam, dp) != 0) {
                continue;
            }
            errors += 1;
            errors;
            break;
        }
        closedir(dirp);
    } else {
        errors += 1;
        errors;
    }
    return errors;
}
unsafe extern "C" fn foreachfile_callback(
    mut dirname: *mut libc::c_char,
    mut data1: *mut libc::c_void,
    mut data2: *mut libc::c_void,
) -> libc::c_int {
    let mut func: Option::<file_worker_func> = *(data1
        as *mut Option::<file_worker_func>);
    let mut is_done: libc::c_int = 0 as libc::c_int;
    let mut argz: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argz_len: size_t = 0 as libc::c_int as size_t;
    if !(list_files_by_dir(dirname, &mut argz, &mut argz_len) != 0 as libc::c_int) {
        if !argz.is_null() {
            let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
            loop {
                filename = argz_next(argz, argz_len, filename);
                if filename.is_null() {
                    break;
                }
                is_done = (Some(func.unwrap())).unwrap()(filename, data2);
                if is_done != 0 {
                    break;
                }
            }
        }
    }
    ({
        free(argz as *mut libc::c_void);
        argz = 0 as *mut libc::c_char;
        argz
    });
    return is_done;
}
pub unsafe extern "C" fn lt_dlforeachfile(
    mut search_path: *const libc::c_char,
    mut func: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut is_done: libc::c_int = 0 as libc::c_int;
    let mut fpptr: *mut Option::<file_worker_func> = &mut func;
    if !search_path.is_null() {
        is_done = foreach_dirinpath(
            search_path,
            0 as *const libc::c_char,
            Some(
                foreachfile_callback
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            fpptr as *mut libc::c_void,
            data,
        );
    } else {
        is_done = foreach_dirinpath(
            user_search_path,
            0 as *const libc::c_char,
            Some(
                foreachfile_callback
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            fpptr as *mut libc::c_void,
            data,
        );
        if is_done == 0 {
            is_done = foreach_dirinpath(
                getenv(b"LTDL_LIBRARY_PATH\0" as *const u8 as *const libc::c_char),
                0 as *const libc::c_char,
                Some(
                    foreachfile_callback
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                fpptr as *mut libc::c_void,
                data,
            );
        }
        if is_done == 0 {
            is_done = foreach_dirinpath(
                getenv(b"LD_LIBRARY_PATH\0" as *const u8 as *const libc::c_char),
                0 as *const libc::c_char,
                Some(
                    foreachfile_callback
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                fpptr as *mut libc::c_void,
                data,
            );
        }
        if is_done == 0 && *sys_dlsearch_path.as_ptr() as libc::c_int != 0 {
            is_done = foreach_dirinpath(
                sys_dlsearch_path.as_ptr(),
                0 as *const libc::c_char,
                Some(
                    foreachfile_callback
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                fpptr as *mut libc::c_void,
                data,
            );
        }
    }
    return is_done;
}
pub unsafe extern "C" fn lt_dlclose(mut handle: lt_dlhandle) -> libc::c_int {
    let mut cur: lt_dlhandle = 0 as *mut lt__handle;
    let mut last: lt_dlhandle = 0 as *mut lt__handle;
    let mut errors: libc::c_int = 0 as libc::c_int;
    cur = handles;
    last = cur;
    while !cur.is_null() && handle != cur {
        last = cur;
        cur = (*cur).next;
    }
    if cur.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_HANDLE as libc::c_int));
        errors += 1;
        errors;
    } else {
        cur = handle;
        (*cur).info.ref_count -= 1;
        (*cur).info.ref_count;
        if (*cur).info.ref_count <= 0 as libc::c_int && ((*cur).info).is_resident() == 0
        {
            let mut data: lt_user_data = (*(*cur).vtable).dlloader_data;
            if cur != handles {
                (*last).next = (*cur).next;
            } else {
                handles = (*cur).next;
            }
            errors += ((*(*cur).vtable).module_close).unwrap()(data, (*cur).module);
            errors += unload_deplibs(handle);
            ({
                free((*cur).interface_data as *mut libc::c_void);
                (*cur).interface_data = 0 as *mut lt_interface_data;
                (*cur).interface_data
            });
            ({
                free((*cur).info.filename as *mut libc::c_void);
                (*cur).info.filename = 0 as *mut libc::c_char;
                (*cur).info.filename
            });
            ({
                free((*cur).info.name as *mut libc::c_void);
                (*cur).info.name = 0 as *mut libc::c_char;
                (*cur).info.name
            });
            ({
                free(cur as *mut libc::c_void);
                cur = 0 as lt_dlhandle;
                cur
            });
        } else if ((*handle).info).is_resident() != 0 {
            lt__set_last_error(
                lt__error_string(LT_ERROR_CLOSE_RESIDENT_MODULE as libc::c_int),
            );
            errors += 1;
            errors;
        }
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlsym(
    mut place: lt_dlhandle,
    mut symbol: *const libc::c_char,
) -> *mut libc::c_void {
    let mut lensym: size_t = 0;
    let mut lsym: [libc::c_char; 128] = [0; 128];
    let mut sym: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut address: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut data: lt_user_data = 0 as *mut libc::c_void;
    let mut handle: lt_dlhandle = 0 as *mut lt__handle;
    if place.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_HANDLE as libc::c_int));
        return 0 as *mut libc::c_void;
    }
    handle = place;
    if symbol.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_SYMBOL_NOT_FOUND as libc::c_int));
        return 0 as *mut libc::c_void;
    }
    lensym = (if !symbol.is_null()
        && *symbol.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        strlen(symbol)
    } else {
        0 as libc::c_int as libc::c_ulong
    })
        .wrapping_add(
            (if !((*(*handle).vtable).sym_prefix).is_null()
                && *((*(*handle).vtable).sym_prefix).offset(0 as libc::c_int as isize)
                    as libc::c_int != 0
            {
                strlen((*(*handle).vtable).sym_prefix)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        )
        .wrapping_add(
            (if !((*handle).info.name).is_null()
                && *((*handle).info.name).offset(0 as libc::c_int as isize)
                    as libc::c_int != 0
            {
                strlen((*handle).info.name)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        );
    if lensym.wrapping_add(5 as libc::c_int as libc::c_ulong)
        < 128 as libc::c_int as libc::c_ulong
    {
        sym = lsym.as_mut_ptr();
    } else {
        sym = lt__malloc(
            lensym
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        if sym.is_null() {
            lt__set_last_error(
                lt__error_string(LT_ERROR_BUFFER_OVERFLOW as libc::c_int),
            );
            return 0 as *mut libc::c_void;
        }
    }
    data = (*(*handle).vtable).dlloader_data;
    if !((*handle).info.name).is_null() {
        let mut saved_error: *const libc::c_char = 0 as *const libc::c_char;
        saved_error = lt__get_last_error();
        if !((*(*handle).vtable).sym_prefix).is_null() {
            strcpy(sym, (*(*handle).vtable).sym_prefix);
            strcat(sym, (*handle).info.name);
        } else {
            strcpy(sym, (*handle).info.name);
        }
        strcat(sym, b"_LTX_\0" as *const u8 as *const libc::c_char);
        strcat(sym, symbol);
        address = ((*(*handle).vtable).find_sym).unwrap()(data, (*handle).module, sym);
        if !address.is_null() {
            if sym != lsym.as_mut_ptr() {
                ({
                    free(sym as *mut libc::c_void);
                    sym = 0 as *mut libc::c_char;
                    sym
                });
            }
            return address;
        }
        lt__set_last_error(saved_error);
    }
    if !((*(*handle).vtable).sym_prefix).is_null() {
        strcpy(sym, (*(*handle).vtable).sym_prefix);
        strcat(sym, symbol);
    } else {
        strcpy(sym, symbol);
    }
    address = ((*(*handle).vtable).find_sym).unwrap()(data, (*handle).module, sym);
    if sym != lsym.as_mut_ptr() {
        ({
            free(sym as *mut libc::c_void);
            sym = 0 as *mut libc::c_char;
            sym
        });
    }
    return address;
}
pub unsafe extern "C" fn lt_dlerror() -> *const libc::c_char {
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    error = lt__get_last_error();
    lt__set_last_error(0 as *const libc::c_char);
    return error;
}
unsafe extern "C" fn lt_dlpath_insertdir(
    mut ppath: *mut *mut libc::c_char,
    mut before: *mut libc::c_char,
    mut dir: *const libc::c_char,
) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut canonical: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argz: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argz_len: size_t = 0 as libc::c_int as size_t;
    if !ppath.is_null() {} else {
        __assert_fail(
            b"ppath\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            2126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_5929: {
        if !ppath.is_null() {} else {
            __assert_fail(
                b"ppath\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                2126 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !dir.is_null() && *dir as libc::c_int != 0 {} else {
        __assert_fail(
            b"dir && *dir\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            2127 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_5884: {
        if !dir.is_null() && *dir as libc::c_int != 0 {} else {
            __assert_fail(
                b"dir && *dir\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                2127 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if canonicalize_path(dir, &mut canonical) != 0 as libc::c_int {
        errors += 1;
        errors;
    } else {
        if !canonical.is_null() && *canonical as libc::c_int != 0 {} else {
            __assert_fail(
                b"canonical && *canonical\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                2135 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_5527: {
            if !canonical.is_null() && *canonical as libc::c_int != 0 {} else {
                __assert_fail(
                    b"canonical && *canonical\0" as *const u8 as *const libc::c_char,
                    b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                    2135 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ppath).is_null() {
            if before.is_null() {} else {
                __assert_fail(
                    b"!before\0" as *const u8 as *const libc::c_char,
                    b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                    2140 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_5487: {
                if before.is_null() {} else {
                    __assert_fail(
                        b"!before\0" as *const u8 as *const libc::c_char,
                        b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                        2140 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if !dir.is_null() {} else {
                __assert_fail(
                    b"dir\0" as *const u8 as *const libc::c_char,
                    b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                    2141 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_5454: {
                if !dir.is_null() {} else {
                    __assert_fail(
                        b"dir\0" as *const u8 as *const libc::c_char,
                        b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                        2141 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            *ppath = lt__strdup(dir);
            if (*ppath).is_null() {
                errors += 1;
                errors;
            }
        } else {
            if !ppath.is_null() && !(*ppath).is_null() {} else {
                __assert_fail(
                    b"ppath && *ppath\0" as *const u8 as *const libc::c_char,
                    b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                    2150 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_5384: {
                if !ppath.is_null() && !(*ppath).is_null() {} else {
                    __assert_fail(
                        b"ppath && *ppath\0" as *const u8 as *const libc::c_char,
                        b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                        2150 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"int lt_dlpath_insertdir(char **, char *, const char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if argzize_path(*ppath, &mut argz, &mut argz_len) != 0 as libc::c_int {
                errors += 1;
                errors;
            } else {
                if !before.is_null() {
                    if *ppath <= before {} else {
                        __assert_fail(
                            b"*ppath <= before\0" as *const u8 as *const libc::c_char,
                            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                            2165 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"int lt_dlpath_insertdir(char **, char *, const char *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5165: {
                        if *ppath <= before {} else {
                            __assert_fail(
                                b"*ppath <= before\0" as *const u8 as *const libc::c_char,
                                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                                2165 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 55],
                                    &[libc::c_char; 55],
                                >(
                                    b"int lt_dlpath_insertdir(char **, char *, const char *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if before.offset_from(*ppath) as libc::c_long as libc::c_int
                        <= strlen(*ppath) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"(int) (before - *ppath) <= (int) strlen (*ppath)\0"
                                as *const u8 as *const libc::c_char,
                            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                            2166 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 55],
                                &[libc::c_char; 55],
                            >(
                                b"int lt_dlpath_insertdir(char **, char *, const char *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5097: {
                        if before.offset_from(*ppath) as libc::c_long as libc::c_int
                            <= strlen(*ppath) as libc::c_int
                        {} else {
                            __assert_fail(
                                b"(int) (before - *ppath) <= (int) strlen (*ppath)\0"
                                    as *const u8 as *const libc::c_char,
                                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                                2166 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 55],
                                    &[libc::c_char; 55],
                                >(
                                    b"int lt_dlpath_insertdir(char **, char *, const char *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    before = argz
                        .offset(before.offset_from(*ppath) as libc::c_long as isize);
                }
                if lt_argz_insert(&mut argz, &mut argz_len, before, dir)
                    != 0 as libc::c_int
                {
                    errors += 1;
                    errors;
                } else {
                    argz_stringify(argz, argz_len, ':' as i32);
                    if *ppath != argz {
                        free(*ppath as *mut libc::c_void);
                        *ppath = argz;
                        argz = 0 as *mut libc::c_char;
                    }
                }
            }
        }
    }
    ({
        free(argz as *mut libc::c_void);
        argz = 0 as *mut libc::c_char;
        argz
    });
    ({
        free(canonical as *mut libc::c_void);
        canonical = 0 as *mut libc::c_char;
        canonical
    });
    return errors;
}
pub unsafe extern "C" fn lt_dladdsearchdir(
    mut search_dir: *const libc::c_char,
) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    if !search_dir.is_null() && *search_dir as libc::c_int != 0 {
        if lt_dlpath_insertdir(&mut user_search_path, 0 as *mut libc::c_char, search_dir)
            != 0 as libc::c_int
        {
            errors += 1;
            errors;
        }
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlinsertsearchdir(
    mut before: *const libc::c_char,
    mut search_dir: *const libc::c_char,
) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    if !before.is_null() {
        if before < user_search_path as *const libc::c_char
            || before
                >= user_search_path
                    .offset(
                        (if !user_search_path.is_null()
                            && *user_search_path.offset(0 as libc::c_int as isize)
                                as libc::c_int != 0
                        {
                            strlen(user_search_path)
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        }) as isize,
                    ) as *const libc::c_char
        {
            lt__set_last_error(
                lt__error_string(LT_ERROR_INVALID_POSITION as libc::c_int),
            );
            return 1 as libc::c_int;
        }
    }
    if !search_dir.is_null() && *search_dir as libc::c_int != 0 {
        if lt_dlpath_insertdir(
            &mut user_search_path,
            before as *mut libc::c_char,
            search_dir,
        ) != 0 as libc::c_int
        {
            errors += 1;
            errors;
        }
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlsetsearchpath(
    mut search_path: *const libc::c_char,
) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    ({
        free(user_search_path as *mut libc::c_void);
        user_search_path = 0 as *mut libc::c_char;
        user_search_path
    });
    if search_path.is_null()
        || (if !search_path.is_null()
            && *search_path.offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            strlen(search_path)
        } else {
            0 as libc::c_int as libc::c_ulong
        }) == 0
    {
        return errors;
    }
    if canonicalize_path(search_path, &mut user_search_path) != 0 as libc::c_int {
        errors += 1;
        errors;
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlgetsearchpath() -> *const libc::c_char {
    let mut saved_path: *const libc::c_char = 0 as *const libc::c_char;
    saved_path = user_search_path;
    return saved_path;
}
pub unsafe extern "C" fn lt_dlmakeresident(mut handle: lt_dlhandle) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    if handle.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_HANDLE as libc::c_int));
        errors += 1;
        errors;
    } else {
        ((*handle).info).set_is_resident(1 as libc::c_int as libc::c_uint);
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlisresident(mut handle: lt_dlhandle) -> libc::c_int {
    if handle.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_HANDLE as libc::c_int));
        return -(1 as libc::c_int);
    }
    return ((*handle).info).is_resident() as libc::c_int;
}
pub unsafe extern "C" fn lt_dlinterface_register(
    mut id_string: *const libc::c_char,
    mut iface: Option::<lt_dlhandle_interface>,
) -> lt_dlinterface_id {
    let mut interface_id: *mut lt__interface_id = lt__malloc(
        ::std::mem::size_of::<lt__interface_id>() as libc::c_ulong,
    ) as *mut lt__interface_id;
    if !interface_id.is_null() {
        (*interface_id).id_string = lt__strdup(id_string);
        if ((*interface_id).id_string).is_null() {
            ({
                free(interface_id as *mut libc::c_void);
                interface_id = 0 as *mut lt__interface_id;
                interface_id
            });
        } else {
            (*interface_id).iface = iface;
        }
    }
    return interface_id as lt_dlinterface_id;
}
pub unsafe extern "C" fn lt_dlinterface_free(mut key: lt_dlinterface_id) {
    let mut interface_id: *mut lt__interface_id = key as *mut lt__interface_id;
    ({
        free((*interface_id).id_string as *mut libc::c_void);
        (*interface_id).id_string = 0 as *mut libc::c_char;
        (*interface_id).id_string
    });
    ({
        free(interface_id as *mut libc::c_void);
        interface_id = 0 as *mut lt__interface_id;
        interface_id
    });
}
pub unsafe extern "C" fn lt_dlcaller_set_data(
    mut key: lt_dlinterface_id,
    mut handle: lt_dlhandle,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut n_elements: libc::c_int = 0 as libc::c_int;
    let mut stale: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cur: lt_dlhandle = handle;
    let mut i: libc::c_int = 0;
    if !((*cur).interface_data).is_null() {
        while !((*((*cur).interface_data).offset(n_elements as isize)).key).is_null() {
            n_elements += 1;
            n_elements;
        }
    }
    i = 0 as libc::c_int;
    while i < n_elements {
        if (*((*cur).interface_data).offset(i as isize)).key == key {
            stale = (*((*cur).interface_data).offset(i as isize)).data;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if i == n_elements {
        let mut temp: *mut lt_interface_data = lt__realloc(
            (*cur).interface_data as *mut libc::c_void,
            ((2 as libc::c_int + n_elements) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<lt_interface_data>() as libc::c_ulong,
                ),
        ) as *mut lt_interface_data;
        if temp.is_null() {
            stale = 0 as *mut libc::c_void;
            current_block = 7663761025762018705;
        } else {
            (*cur).interface_data = temp;
            let ref mut fresh3 = (*((*cur).interface_data).offset(i as isize)).key;
            *fresh3 = key;
            let ref mut fresh4 = (*((*cur).interface_data)
                .offset((1 as libc::c_int + i) as isize))
                .key;
            *fresh4 = 0 as lt_dlinterface_id;
            current_block = 10048703153582371463;
        }
    } else {
        current_block = 10048703153582371463;
    }
    match current_block {
        10048703153582371463 => {
            let ref mut fresh5 = (*((*cur).interface_data).offset(i as isize)).data;
            *fresh5 = data;
        }
        _ => {}
    }
    return stale;
}
pub unsafe extern "C" fn lt_dlcaller_get_data(
    mut key: lt_dlinterface_id,
    mut handle: lt_dlhandle,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cur: lt_dlhandle = handle;
    if !((*cur).interface_data).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !((*((*cur).interface_data).offset(i as isize)).key).is_null() {
            if (*((*cur).interface_data).offset(i as isize)).key == key {
                result = (*((*cur).interface_data).offset(i as isize)).data;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    return result;
}
pub unsafe extern "C" fn lt_dlgetinfo(mut handle: lt_dlhandle) -> *const lt_dlinfo {
    if handle.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_HANDLE as libc::c_int));
        return 0 as *const lt_dlinfo;
    }
    return &mut (*handle).info;
}
pub unsafe extern "C" fn lt_dlhandle_iterate(
    mut iface: lt_dlinterface_id,
    mut place: lt_dlhandle,
) -> lt_dlhandle {
    let mut handle: lt_dlhandle = place;
    let mut iterator: *mut lt__interface_id = iface as *mut lt__interface_id;
    if !iface.is_null() {} else {
        __assert_fail(
            b"iface\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            2410 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 64],
                &[libc::c_char; 64],
            >(b"lt_dlhandle lt_dlhandle_iterate(lt_dlinterface_id, lt_dlhandle)\0"))
                .as_ptr(),
        );
    }
    'c_12480: {
        if !iface.is_null() {} else {
            __assert_fail(
                b"iface\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                2410 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 64],
                    &[libc::c_char; 64],
                >(b"lt_dlhandle lt_dlhandle_iterate(lt_dlinterface_id, lt_dlhandle)\0"))
                    .as_ptr(),
            );
        }
    };
    if handle.is_null() {
        handle = handles;
    } else {
        handle = (*handle).next;
    }
    while !handle.is_null() && ((*iterator).iface).is_some()
        && (Some(((*iterator).iface).unwrap())).unwrap()(handle, (*iterator).id_string)
            != 0 as libc::c_int
    {
        handle = (*handle).next;
    }
    return handle;
}
pub unsafe extern "C" fn lt_dlhandle_fetch(
    mut iface: lt_dlinterface_id,
    mut module_name: *const libc::c_char,
) -> lt_dlhandle {
    let mut handle: lt_dlhandle = 0 as lt_dlhandle;
    if !iface.is_null() {} else {
        __assert_fail(
            b"iface\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            2433 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 63],
                &[libc::c_char; 63],
            >(b"lt_dlhandle lt_dlhandle_fetch(lt_dlinterface_id, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_12570: {
        if !iface.is_null() {} else {
            __assert_fail(
                b"iface\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                2433 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 63],
                    &[libc::c_char; 63],
                >(b"lt_dlhandle lt_dlhandle_fetch(lt_dlinterface_id, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    loop {
        handle = lt_dlhandle_iterate(iface, handle);
        if handle.is_null() {
            break;
        }
        let mut cur: lt_dlhandle = handle;
        if !cur.is_null() && !((*cur).info.name).is_null()
            && strcmp((*cur).info.name, module_name) == 0 as libc::c_int
        {
            break;
        }
    }
    return handle;
}
pub unsafe extern "C" fn lt_dlhandle_map(
    mut iface: lt_dlinterface_id,
    mut func: Option::<
        unsafe extern "C" fn(lt_dlhandle, *mut libc::c_void) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut iterator: *mut lt__interface_id = iface as *mut lt__interface_id;
    let mut cur: lt_dlhandle = handles;
    if !iface.is_null() {} else {
        __assert_fail(
            b"iface\0" as *const u8 as *const libc::c_char,
            b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
            2453 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"int lt_dlhandle_map(lt_dlinterface_id, int (*)(lt_dlhandle, void *), void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12685: {
        if !iface.is_null() {} else {
            __assert_fail(
                b"iface\0" as *const u8 as *const libc::c_char,
                b"libltdl/ltdl.c\0" as *const u8 as *const libc::c_char,
                2453 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"int lt_dlhandle_map(lt_dlinterface_id, int (*)(lt_dlhandle, void *), void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while !cur.is_null() {
        let mut errorcode: libc::c_int = 0 as libc::c_int;
        while !cur.is_null() && ((*iterator).iface).is_some()
            && (Some(((*iterator).iface).unwrap())).unwrap()(cur, (*iterator).id_string)
                != 0 as libc::c_int
        {
            cur = (*cur).next;
        }
        errorcode = (Some(func.unwrap())).unwrap()(cur, data);
        if errorcode != 0 as libc::c_int {
            return errorcode;
        }
    }
    return 0 as libc::c_int;
}
