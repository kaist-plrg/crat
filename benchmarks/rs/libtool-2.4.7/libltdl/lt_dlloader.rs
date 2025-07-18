use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut lt__alloc_die: Option::<unsafe extern "C" fn() -> ()>;
    fn lt__slist_concat(head: *mut SList, tail: *mut SList) -> *mut SList;
    fn lt__slist_cons(item: *mut SList, slist: *mut SList) -> *mut SList;
    fn lt__slist_remove(
        phead: *mut *mut SList,
        find: Option::<SListCallback>,
        matchdata: *mut libc::c_void,
    ) -> *mut SList;
    fn lt__slist_find(
        slist: *mut SList,
        find: Option::<SListCallback>,
        matchdata: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn lt__slist_box(userdata: *const libc::c_void) -> *mut SList;
    fn lt__slist_unbox(item: *mut SList) -> *mut libc::c_void;
    fn lt__error_string(errorcode: libc::c_int) -> *const libc::c_char;
    fn lt__set_last_error(errormsg: *const libc::c_char) -> *const libc::c_char;
    fn lt_dlinterface_free(key: lt_dlinterface_id);
    fn lt_dlisresident(handle: lt_dlhandle) -> libc::c_int;
    fn lt_dlhandle_iterate(iface: lt_dlinterface_id, place: lt_dlhandle) -> lt_dlhandle;
    fn lt_dlinterface_register(
        id_string: *const libc::c_char,
        iface: Option::<lt_dlhandle_interface>,
    ) -> lt_dlinterface_id;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slist {
    pub next: *mut slist,
    pub userdata: *const libc::c_void,
}
pub type SList = slist;
pub type SListCallback = unsafe extern "C" fn(
    *mut SList,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type C2RustUnnamed = libc::c_uint;
pub const LT_ERROR_MAX: C2RustUnnamed = 20;
pub const LT_ERROR_CONFLICTING_FLAGS: C2RustUnnamed = 19;
pub const LT_ERROR_INVALID_POSITION: C2RustUnnamed = 18;
pub const LT_ERROR_INVALID_MUTEX_ARGS: C2RustUnnamed = 17;
pub const LT_ERROR_CLOSE_RESIDENT_MODULE: C2RustUnnamed = 16;
pub const LT_ERROR_SHUTDOWN: C2RustUnnamed = 15;
pub const LT_ERROR_INVALID_ERRORCODE: C2RustUnnamed = 14;
pub const LT_ERROR_BUFFER_OVERFLOW: C2RustUnnamed = 13;
pub const LT_ERROR_INVALID_HANDLE: C2RustUnnamed = 12;
pub const LT_ERROR_NO_MEMORY: C2RustUnnamed = 11;
pub const LT_ERROR_SYMBOL_NOT_FOUND: C2RustUnnamed = 10;
pub const LT_ERROR_CANNOT_CLOSE: C2RustUnnamed = 9;
pub const LT_ERROR_CANNOT_OPEN: C2RustUnnamed = 8;
pub const LT_ERROR_NO_SYMBOLS: C2RustUnnamed = 7;
pub const LT_ERROR_DEPLIB_NOT_FOUND: C2RustUnnamed = 6;
pub const LT_ERROR_FILE_NOT_FOUND: C2RustUnnamed = 5;
pub const LT_ERROR_REMOVE_LOADER: C2RustUnnamed = 4;
pub const LT_ERROR_INIT_LOADER: C2RustUnnamed = 3;
pub const LT_ERROR_INVALID_LOADER: C2RustUnnamed = 2;
pub const LT_ERROR_DLOPEN_NOT_SUPPORTED: C2RustUnnamed = 1;
pub const LT_ERROR_UNKNOWN: C2RustUnnamed = 0;
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
pub type lt_dlinterface_id = *mut libc::c_void;
pub type lt_dlhandle = *mut lt__handle;
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
pub type lt_dlhandle_interface = unsafe extern "C" fn(
    lt_dlhandle,
    *const libc::c_char,
) -> libc::c_int;
static mut loaders: *mut SList = 0 as *const SList as *mut SList;
unsafe extern "C" fn loader_callback(
    mut item: *mut SList,
    mut userdata: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut vtable: *const lt_dlvtable = (*item).userdata as *const lt_dlvtable;
    let mut name: *const libc::c_char = userdata as *const libc::c_char;
    if !vtable.is_null() {} else {
        __assert_fail(
            b"vtable\0" as *const u8 as *const libc::c_char,
            b"libltdl/lt_dlloader.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void *loader_callback(SList *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_3737: {
        if !vtable.is_null() {} else {
            __assert_fail(
                b"vtable\0" as *const u8 as *const libc::c_char,
                b"libltdl/lt_dlloader.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void *loader_callback(SList *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    return if strcmp((*vtable).name, name) == 0 as libc::c_int {
        item as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    };
}
pub unsafe extern "C" fn lt_dlloader_add(mut vtable: *const lt_dlvtable) -> libc::c_int {
    let mut item: *mut SList = 0 as *mut SList;
    if vtable.is_null() || ((*vtable).module_open).is_none()
        || ((*vtable).module_close).is_none() || ((*vtable).find_sym).is_none()
        || (*vtable).priority as libc::c_uint
            != LT_DLLOADER_PREPEND as libc::c_int as libc::c_uint
            && (*vtable).priority as libc::c_uint
                != LT_DLLOADER_APPEND as libc::c_int as libc::c_uint
    {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_LOADER as libc::c_int));
        return 1 as libc::c_int;
    }
    item = lt__slist_box(vtable as *const libc::c_void);
    if item.is_null() {
        (Some(lt__alloc_die.unwrap())).unwrap()();
        return 1 as libc::c_int;
    }
    if (*vtable).priority as libc::c_uint
        == LT_DLLOADER_PREPEND as libc::c_int as libc::c_uint
    {
        loaders = lt__slist_cons(item, loaders);
    } else {
        if (*vtable).priority as libc::c_uint
            == LT_DLLOADER_APPEND as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"vtable->priority == LT_DLLOADER_APPEND\0" as *const u8
                    as *const libc::c_char,
                b"libltdl/lt_dlloader.c\0" as *const u8 as *const libc::c_char,
                94 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"int lt_dlloader_add(const lt_dlvtable *)\0"))
                    .as_ptr(),
            );
        }
        'c_3478: {
            if (*vtable).priority as libc::c_uint
                == LT_DLLOADER_APPEND as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"vtable->priority == LT_DLLOADER_APPEND\0" as *const u8
                        as *const libc::c_char,
                    b"libltdl/lt_dlloader.c\0" as *const u8 as *const libc::c_char,
                    94 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"int lt_dlloader_add(const lt_dlvtable *)\0"))
                        .as_ptr(),
                );
            }
        };
        loaders = lt__slist_concat(loaders, item);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dlloader_next(mut loader: lt_dlloader) -> lt_dlloader {
    let mut item: *mut SList = loader as *mut SList;
    return (if !item.is_null() { (*item).next } else { loaders }) as lt_dlloader;
}
pub unsafe extern "C" fn lt_dlloader_get(mut loader: lt_dlloader) -> *const lt_dlvtable {
    return (if !loader.is_null() {
        (*(loader as *mut SList)).userdata
    } else {
        0 as *const libc::c_void
    }) as *const lt_dlvtable;
}
pub unsafe extern "C" fn lt_dlloader_remove(
    mut name: *const libc::c_char,
) -> *mut lt_dlvtable {
    let mut vtable: *const lt_dlvtable = lt_dlloader_find(name);
    static mut id_string: [libc::c_char; 19] = unsafe {
        *::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"lt_dlloader_remove\0")
    };
    let mut iface: lt_dlinterface_id = 0 as *mut libc::c_void;
    let mut handle: lt_dlhandle = 0 as lt_dlhandle;
    let mut in_use: libc::c_int = 0 as libc::c_int;
    let mut in_use_by_resident: libc::c_int = 0 as libc::c_int;
    if vtable.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_INVALID_LOADER as libc::c_int));
        return 0 as *mut lt_dlvtable;
    }
    iface = lt_dlinterface_register(id_string.as_ptr(), None);
    if iface.is_null() {
        return 0 as *mut lt_dlvtable;
    }
    loop {
        handle = lt_dlhandle_iterate(iface, handle);
        if handle.is_null() {
            break;
        }
        let mut cur: lt_dlhandle = handle;
        if (*cur).vtable == vtable {
            in_use = 1 as libc::c_int;
            if lt_dlisresident(handle) != 0 {
                in_use_by_resident = 1 as libc::c_int;
            }
        }
    }
    lt_dlinterface_free(iface);
    if in_use != 0 {
        if in_use_by_resident == 0 {
            lt__set_last_error(lt__error_string(LT_ERROR_REMOVE_LOADER as libc::c_int));
        }
        return 0 as *mut lt_dlvtable;
    }
    if !vtable.is_null() && ((*vtable).dlloader_exit).is_some() {
        if (Some(((*vtable).dlloader_exit).unwrap())).unwrap()((*vtable).dlloader_data)
            != 0 as libc::c_int
        {
            return 0 as *mut lt_dlvtable;
        }
    }
    return lt__slist_unbox(
        lt__slist_remove(
            &mut loaders,
            Some(
                loader_callback
                    as unsafe extern "C" fn(
                        *mut SList,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            name as *mut libc::c_void,
        ),
    ) as *mut lt_dlvtable;
}
pub unsafe extern "C" fn lt_dlloader_find(
    mut name: *const libc::c_char,
) -> *const lt_dlvtable {
    return lt_dlloader_get(
        lt__slist_find(
            loaders,
            Some(
                loader_callback
                    as unsafe extern "C" fn(
                        *mut SList,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            name as *mut libc::c_void,
        ),
    );
}
