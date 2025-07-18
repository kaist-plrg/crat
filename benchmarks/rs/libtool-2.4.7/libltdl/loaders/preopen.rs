use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn lt__set_last_error(errormsg: *const libc::c_char) -> *const libc::c_char;
    fn lt__error_string(errorcode: libc::c_int) -> *const libc::c_char;
    fn lt_dlopen(filename: *const libc::c_char) -> lt_dlhandle;
    fn lt__zalloc(n: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lt_dlsymlist {
    pub name: *const libc::c_char,
    pub address: *mut libc::c_void,
}
pub type lt_dlpreload_callback_func = unsafe extern "C" fn(lt_dlhandle) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symlist_chain {
    pub next: *mut symlist_chain,
    pub symlist: *const lt_dlsymlist,
}
static mut vtable: *mut lt_dlvtable = 0 as *const lt_dlvtable as *mut lt_dlvtable;
pub unsafe extern "C" fn preopen_LTX_get_vtable(
    mut loader_data: lt_user_data,
) -> *const lt_dlvtable {
    if vtable.is_null() {
        vtable = lt__zalloc(::std::mem::size_of::<lt_dlvtable>() as libc::c_ulong)
            as *mut lt_dlvtable;
    }
    if !vtable.is_null() && ((*vtable).name).is_null() {
        (*vtable).name = b"lt_preopen\0" as *const u8 as *const libc::c_char;
        (*vtable).sym_prefix = 0 as *const libc::c_char;
        (*vtable)
            .module_open = Some(
            vm_open
                as unsafe extern "C" fn(
                    lt_user_data,
                    *const libc::c_char,
                    lt_dladvise,
                ) -> lt_module,
        );
        (*vtable)
            .module_close = Some(
            vm_close as unsafe extern "C" fn(lt_user_data, lt_module) -> libc::c_int,
        );
        (*vtable)
            .find_sym = Some(
            vm_sym
                as unsafe extern "C" fn(
                    lt_user_data,
                    lt_module,
                    *const libc::c_char,
                ) -> *mut libc::c_void,
        );
        (*vtable)
            .dlloader_init = Some(
            vl_init as unsafe extern "C" fn(lt_user_data) -> libc::c_int,
        );
        (*vtable)
            .dlloader_exit = Some(
            vl_exit as unsafe extern "C" fn(lt_user_data) -> libc::c_int,
        );
        (*vtable).dlloader_data = loader_data;
        (*vtable).priority = LT_DLLOADER_PREPEND;
    }
    if !vtable.is_null() && (*vtable).dlloader_data != loader_data {
        lt__set_last_error(lt__error_string(LT_ERROR_INIT_LOADER as libc::c_int));
        return 0 as *const lt_dlvtable;
    }
    return vtable;
}
static mut preloaded_symlists: *mut symlist_chain = 0 as *const symlist_chain
    as *mut symlist_chain;
static mut default_preloaded_symbols: *const lt_dlsymlist = 0 as *const lt_dlsymlist;
unsafe extern "C" fn vl_init(mut loader_data: lt_user_data) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    preloaded_symlists = 0 as *mut symlist_chain;
    if !default_preloaded_symbols.is_null() {
        errors = lt_dlpreload(default_preloaded_symbols);
    }
    return errors;
}
unsafe extern "C" fn vl_exit(mut loader_data: lt_user_data) -> libc::c_int {
    vtable = 0 as *mut lt_dlvtable;
    free_symlists();
    return 0 as libc::c_int;
}
unsafe extern "C" fn vm_open(
    mut loader_data: lt_user_data,
    mut filename: *const libc::c_char,
    mut advise: lt_dladvise,
) -> lt_module {
    let mut current_block: u64;
    let mut lists: *mut symlist_chain = 0 as *mut symlist_chain;
    let mut module: lt_module = 0 as lt_module;
    if preloaded_symlists.is_null() {
        lt__set_last_error(lt__error_string(LT_ERROR_NO_SYMBOLS as libc::c_int));
    } else {
        if filename.is_null() {
            filename = b"@PROGRAM@\0" as *const u8 as *const libc::c_char;
        }
        lists = preloaded_symlists;
        's_28: loop {
            if lists.is_null() {
                current_block = 12349973810996921269;
                break;
            }
            let mut symbol: *const lt_dlsymlist = 0 as *const lt_dlsymlist;
            symbol = (*lists).symlist;
            while !((*symbol).name).is_null() {
                if ((*symbol).address).is_null()
                    && strcmp((*symbol).name, filename) == 0 as libc::c_int
                {
                    let mut next_symbol: *const lt_dlsymlist = symbol
                        .offset(1 as libc::c_int as isize);
                    if !((*next_symbol).address).is_null()
                        && !((*next_symbol).name).is_null()
                    {
                        module = (*lists).symlist as lt_module;
                        current_block = 9437972375160255536;
                        break 's_28;
                    }
                }
                symbol = symbol.offset(1);
                symbol;
            }
            lists = (*lists).next;
        }
        match current_block {
            9437972375160255536 => {}
            _ => {
                lt__set_last_error(
                    lt__error_string(LT_ERROR_FILE_NOT_FOUND as libc::c_int),
                );
            }
        }
    }
    return module;
}
unsafe extern "C" fn vm_close(
    mut loader_data: lt_user_data,
    mut module: lt_module,
) -> libc::c_int {
    module = 0 as lt_module;
    return 0 as libc::c_int;
}
unsafe extern "C" fn vm_sym(
    mut loader_data: lt_user_data,
    mut module: lt_module,
    mut name: *const libc::c_char,
) -> *mut libc::c_void {
    let mut symbol: *mut lt_dlsymlist = module as *mut lt_dlsymlist;
    if !((*symbol.offset(1 as libc::c_int as isize)).name).is_null()
        && strcmp(
            (*symbol.offset(1 as libc::c_int as isize)).name,
            b"@INIT@\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        symbol = symbol.offset(1);
        symbol;
    }
    symbol = symbol.offset(2 as libc::c_int as isize);
    while !((*symbol).name).is_null() {
        if strcmp((*symbol).name, name) == 0 as libc::c_int {
            return (*symbol).address;
        }
        symbol = symbol.offset(1);
        symbol;
    }
    lt__set_last_error(lt__error_string(LT_ERROR_SYMBOL_NOT_FOUND as libc::c_int));
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn free_symlists() -> libc::c_int {
    let mut lists: *mut symlist_chain = 0 as *mut symlist_chain;
    lists = preloaded_symlists;
    while !lists.is_null() {
        let mut next: *mut symlist_chain = (*lists).next;
        ({
            free(lists as *mut libc::c_void);
            lists = 0 as *mut symlist_chain;
            lists
        });
        lists = next;
    }
    preloaded_symlists = 0 as *mut symlist_chain;
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_symlist(mut symlist: *const lt_dlsymlist) -> libc::c_int {
    let mut lists: *mut symlist_chain = 0 as *mut symlist_chain;
    let mut errors: libc::c_int = 0 as libc::c_int;
    lists = preloaded_symlists;
    while !lists.is_null() && (*lists).symlist != symlist {
        lists = (*lists).next;
    }
    if lists.is_null() {
        let mut tmp: *mut symlist_chain = lt__zalloc(
            ::std::mem::size_of::<symlist_chain>() as libc::c_ulong,
        ) as *mut symlist_chain;
        if !tmp.is_null() {
            (*tmp).symlist = symlist;
            (*tmp).next = preloaded_symlists;
            preloaded_symlists = tmp;
            if !((*symlist.offset(1 as libc::c_int as isize)).name).is_null()
                && strcmp(
                    (*symlist.offset(1 as libc::c_int as isize)).name,
                    b"@INIT@\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                let mut init_symlist: Option::<unsafe extern "C" fn() -> ()> = None;
                let ref mut fresh0 = *(&mut init_symlist
                    as *mut Option::<unsafe extern "C" fn() -> ()>
                    as *mut *mut libc::c_void);
                *fresh0 = (*symlist.offset(1 as libc::c_int as isize)).address;
                (Some(init_symlist.unwrap())).unwrap()();
            }
        } else {
            errors += 1;
            errors;
        }
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlpreload_default(
    mut preloaded: *const lt_dlsymlist,
) -> libc::c_int {
    default_preloaded_symbols = preloaded;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn lt_dlpreload(
    mut preloaded: *const lt_dlsymlist,
) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    if !preloaded.is_null() {
        errors = add_symlist(preloaded);
    } else {
        free_symlists();
        if !default_preloaded_symbols.is_null() {
            errors = lt_dlpreload(default_preloaded_symbols);
        }
    }
    return errors;
}
pub unsafe extern "C" fn lt_dlpreload_open(
    mut originator: *const libc::c_char,
    mut func: Option::<lt_dlpreload_callback_func>,
) -> libc::c_int {
    let mut list: *mut symlist_chain = 0 as *mut symlist_chain;
    let mut errors: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    list = preloaded_symlists;
    while !list.is_null() {
        if !originator.is_null()
            && strcmp((*(*list).symlist).name, originator) == 0 as libc::c_int
            || originator.is_null()
                && strcmp(
                    (*(*list).symlist).name,
                    b"@PROGRAM@\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
        {
            let mut symbol: *const lt_dlsymlist = 0 as *const lt_dlsymlist;
            let mut idx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            found += 1;
            found;
            loop {
                idx = idx.wrapping_add(1);
                symbol = &*((*list).symlist).offset(idx as isize) as *const lt_dlsymlist;
                if ((*symbol).name).is_null() {
                    break;
                }
                if ((*symbol).address).is_null()
                    && strcmp(
                        (*symbol).name,
                        b"@PROGRAM@\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int
                {
                    let mut handle: lt_dlhandle = lt_dlopen((*symbol).name);
                    if handle.is_null() {
                        errors += 1;
                        errors;
                    } else {
                        errors += (Some(func.unwrap())).unwrap()(handle);
                    }
                }
            }
        }
        list = (*list).next;
    }
    if found == 0 {
        lt__set_last_error(lt__error_string(LT_ERROR_CANNOT_OPEN as libc::c_int));
        errors += 1;
        errors;
    }
    return errors;
}
