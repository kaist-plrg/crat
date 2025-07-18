use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn lt__set_last_error(errormsg: *const libc::c_char) -> *const libc::c_char;
    fn lt__error_string(errorcode: libc::c_int) -> *const libc::c_char;
    fn lt__zalloc(n: size_t) -> *mut libc::c_void;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
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
static mut vtable: *mut lt_dlvtable = 0 as *const lt_dlvtable as *mut lt_dlvtable;
pub unsafe extern "C" fn dlopen_LTX_get_vtable(
    mut loader_data: lt_user_data,
) -> *mut lt_dlvtable {
    if vtable.is_null() {
        vtable = lt__zalloc(::std::mem::size_of::<lt_dlvtable>() as libc::c_ulong)
            as *mut lt_dlvtable;
    }
    if !vtable.is_null() && ((*vtable).name).is_null() {
        (*vtable).name = b"lt_dlopen\0" as *const u8 as *const libc::c_char;
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
            .dlloader_exit = Some(
            vl_exit as unsafe extern "C" fn(lt_user_data) -> libc::c_int,
        );
        (*vtable).dlloader_data = loader_data;
        (*vtable).priority = LT_DLLOADER_PREPEND;
    }
    if !vtable.is_null() && (*vtable).dlloader_data != loader_data {
        lt__set_last_error(lt__error_string(LT_ERROR_INIT_LOADER as libc::c_int));
        return 0 as *mut lt_dlvtable;
    }
    return vtable;
}
unsafe extern "C" fn vl_exit(mut loader_data: lt_user_data) -> libc::c_int {
    vtable = 0 as *mut lt_dlvtable;
    return 0 as libc::c_int;
}
unsafe extern "C" fn vm_open(
    mut loader_data: lt_user_data,
    mut filename: *const libc::c_char,
    mut advise: lt_dladvise,
) -> lt_module {
    let mut module_flags: libc::c_int = 0x1 as libc::c_int;
    let mut module: lt_module = 0 as *mut libc::c_void;
    if !advise.is_null() {
        if (*advise).is_symglobal() != 0 {
            module_flags |= 0x100 as libc::c_int;
        }
        if (*advise).is_symlocal() != 0 {
            module_flags |= 0 as libc::c_int;
        }
    }
    module = dlopen(filename, module_flags);
    if module.is_null() {
        lt__set_last_error(dlerror());
    }
    return module;
}
unsafe extern "C" fn vm_close(
    mut loader_data: lt_user_data,
    mut module: lt_module,
) -> libc::c_int {
    let mut errors: libc::c_int = 0 as libc::c_int;
    if dlclose(module) != 0 as libc::c_int {
        lt__set_last_error(dlerror());
        errors += 1;
        errors;
    }
    return errors;
}
unsafe extern "C" fn vm_sym(
    mut loader_data: lt_user_data,
    mut module: lt_module,
    mut name: *const libc::c_char,
) -> *mut libc::c_void {
    let mut address: *mut libc::c_void = dlsym(module, name);
    if address.is_null() {
        lt__set_last_error(dlerror());
    }
    return address;
}
