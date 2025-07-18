use ::libc;
extern "C" {
    pub type PKVM;
    pub type PkHandle;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn pkNewConfiguration() -> PkConfiguration;
    fn pkNewVM(config: *mut PkConfiguration) -> *mut PKVM;
    fn pkFreeVM(vm: *mut PKVM);
    fn pkSetUserData(vm: *mut PKVM, user_data: *mut libc::c_void);
    fn pkGetUserData(vm: *const PKVM) -> *mut libc::c_void;
    fn pkRegisterBuiltinFn(
        vm: *mut PKVM,
        name: *const libc::c_char,
        fn_0: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
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
    fn pkNewClass(
        vm: *mut PKVM,
        name: *const libc::c_char,
        base_class: *mut PkHandle,
        module: *mut PkHandle,
        new_fn: pkNewInstanceFn,
        delete_fn: pkDeleteInstanceFn,
        docstring: *const libc::c_char,
    ) -> *mut PkHandle;
    fn pkClassAddMethod(
        vm: *mut PKVM,
        cls: *mut PkHandle,
        name: *const libc::c_char,
        fptr: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
    fn pkModuleAddSource(
        vm: *mut PKVM,
        module: *mut PkHandle,
        source: *const libc::c_char,
    );
    fn pkRunString(vm: *mut PKVM, source: *const libc::c_char) -> PkResult;
    fn pkRunFile(vm: *mut PKVM, path: *const libc::c_char) -> PkResult;
    fn pkRunREPL(vm: *mut PKVM) -> PkResult;
    fn pkSetRuntimeError(vm: *mut PKVM, message: *const libc::c_char);
    fn pkSetRuntimeErrorFmt(vm: *mut PKVM, fmt: *const libc::c_char, _: ...);
    fn pkGetSelf(vm: *const PKVM) -> *mut libc::c_void;
    fn pkGetArgc(vm: *const PKVM) -> libc::c_int;
    fn pkCheckArgcRange(
        vm: *mut PKVM,
        argc: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    ) -> bool;
    fn pkValidateSlotBool(vm: *mut PKVM, slot: libc::c_int, value: *mut bool) -> bool;
    fn pkValidateSlotNumber(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut libc::c_double,
    ) -> bool;
    fn pkValidateSlotInteger(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut int32_t,
    ) -> bool;
    fn pkValidateSlotString(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut *const libc::c_char,
        length: *mut uint32_t,
    ) -> bool;
    fn pkValidateSlotType(vm: *mut PKVM, slot: libc::c_int, type_0: PkVarType) -> bool;
    fn pkValidateSlotInstanceOf(
        vm: *mut PKVM,
        slot: libc::c_int,
        cls: libc::c_int,
    ) -> bool;
    fn pkIsSlotInstanceOf(
        vm: *mut PKVM,
        inst: libc::c_int,
        cls: libc::c_int,
        val: *mut bool,
    ) -> bool;
    fn pkReserveSlots(vm: *mut PKVM, count: libc::c_int);
    fn pkGetSlotsCount(vm: *mut PKVM) -> libc::c_int;
    fn pkGetSlotType(vm: *mut PKVM, index: libc::c_int) -> PkVarType;
    fn pkGetSlotBool(vm: *mut PKVM, index: libc::c_int) -> bool;
    fn pkGetSlotNumber(vm: *mut PKVM, index: libc::c_int) -> libc::c_double;
    fn pkGetSlotString(
        vm: *mut PKVM,
        index: libc::c_int,
        length: *mut uint32_t,
    ) -> *const libc::c_char;
    fn pkGetSlotHandle(vm: *mut PKVM, index: libc::c_int) -> *mut PkHandle;
    fn pkGetSlotNativeInstance(vm: *mut PKVM, index: libc::c_int) -> *mut libc::c_void;
    fn pkSetSlotNull(vm: *mut PKVM, index: libc::c_int);
    fn pkSetSlotBool(vm: *mut PKVM, index: libc::c_int, value: bool);
    fn pkSetSlotNumber(vm: *mut PKVM, index: libc::c_int, value: libc::c_double);
    fn pkSetSlotString(vm: *mut PKVM, index: libc::c_int, value: *const libc::c_char);
    fn pkSetSlotStringLength(
        vm: *mut PKVM,
        index: libc::c_int,
        value: *const libc::c_char,
        length: uint32_t,
    );
    fn pkSetSlotHandle(vm: *mut PKVM, index: libc::c_int, handle: *mut PkHandle);
    fn pkGetSlotHash(vm: *mut PKVM, index: libc::c_int) -> uint32_t;
    fn pkPlaceSelf(vm: *mut PKVM, index: libc::c_int);
    fn pkGetClass(vm: *mut PKVM, instance: libc::c_int, index: libc::c_int);
    fn pkNewInstance(
        vm: *mut PKVM,
        cls: libc::c_int,
        index: libc::c_int,
        argc: libc::c_int,
        argv: libc::c_int,
    ) -> bool;
    fn pkNewRange(
        vm: *mut PKVM,
        index: libc::c_int,
        first: libc::c_double,
        last: libc::c_double,
    );
    fn pkNewList(vm: *mut PKVM, index: libc::c_int);
    fn pkNewMap(vm: *mut PKVM, index: libc::c_int);
    fn pkListInsert(
        vm: *mut PKVM,
        list: libc::c_int,
        index: int32_t,
        value: libc::c_int,
    ) -> bool;
    fn pkListPop(
        vm: *mut PKVM,
        list: libc::c_int,
        index: int32_t,
        popped: libc::c_int,
    ) -> bool;
    fn pkListLength(vm: *mut PKVM, list: libc::c_int) -> uint32_t;
    fn pkCallFunction(
        vm: *mut PKVM,
        fn_0: libc::c_int,
        argc: libc::c_int,
        argv: libc::c_int,
        ret: libc::c_int,
    ) -> bool;
    fn pkCallMethod(
        vm: *mut PKVM,
        instance: libc::c_int,
        method: *const libc::c_char,
        argc: libc::c_int,
        argv: libc::c_int,
        ret: libc::c_int,
    ) -> bool;
    fn pkGetAttribute(
        vm: *mut PKVM,
        instance: libc::c_int,
        name: *const libc::c_char,
        index: libc::c_int,
    ) -> bool;
    fn pkSetAttribute(
        vm: *mut PKVM,
        instance: libc::c_int,
        name: *const libc::c_char,
        value: libc::c_int,
    ) -> bool;
    fn pkImportModule(
        vm: *mut PKVM,
        path: *const libc::c_char,
        index: libc::c_int,
    ) -> bool;
    fn __errno_location() -> *mut libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub const PK_INSTANCE: PkVarType = 13;
pub const PK_CLASS: PkVarType = 12;
pub const PK_FIBER: PkVarType = 11;
pub const PK_METHOD_BIND: PkVarType = 10;
pub const PK_CLOSURE: PkVarType = 9;
pub const PK_MODULE: PkVarType = 8;
pub const PK_RANGE: PkVarType = 7;
pub const PK_MAP: PkVarType = 6;
pub const PK_LIST: PkVarType = 5;
pub const PK_STRING: PkVarType = 4;
pub const PK_NUMBER: PkVarType = 3;
pub const PK_BOOL: PkVarType = 2;
pub const PK_NULL: PkVarType = 1;
pub const PK_OBJECT: PkVarType = 0;
pub type PkVarType = libc::c_uint;
pub const PK_RESULT_RUNTIME_ERROR: PkResult = 3;
pub const PK_RESULT_COMPILE_ERROR: PkResult = 2;
pub const PK_RESULT_UNEXPECTED_EOF: PkResult = 1;
pub const PK_RESULT_SUCCESS: PkResult = 0;
pub type PkResult = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PkConfiguration {
    pub realloc_fn: pkReallocFn,
    pub stderr_write: pkWriteFn,
    pub stdout_write: pkWriteFn,
    pub stdin_read: pkReadFn,
    pub resolve_path_fn: pkResolvePathFn,
    pub load_script_fn: pkLoadScriptFn,
    pub load_dl_fn: pkLoadDL,
    pub import_dl_fn: pkImportDL,
    pub unload_dl_fn: pkUnloadDL,
    pub use_ansi_escape: bool,
    pub user_data: *mut libc::c_void,
}
pub type pkUnloadDL = Option::<unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> ()>;
pub type pkImportDL = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> *mut PkHandle,
>;
pub type pkLoadDL = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut libc::c_void,
>;
pub type pkLoadScriptFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut libc::c_char,
>;
pub type pkResolvePathFn = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        *const libc::c_char,
        *const libc::c_char,
    ) -> *mut libc::c_char,
>;
pub type pkReadFn = Option::<unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_char>;
pub type pkWriteFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
>;
pub type pkReallocFn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        size_t,
        *mut libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type pkNativeFn = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
pub type pkNewInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void,
>;
pub type pkDeleteInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PkNativeApi {
    pub pkNewConfiguration_ptr: pkNewConfiguration_t,
    pub pkNewVM_ptr: pkNewVM_t,
    pub pkFreeVM_ptr: pkFreeVM_t,
    pub pkSetUserData_ptr: pkSetUserData_t,
    pub pkGetUserData_ptr: pkGetUserData_t,
    pub pkRegisterBuiltinFn_ptr: pkRegisterBuiltinFn_t,
    pub pkAddSearchPath_ptr: pkAddSearchPath_t,
    pub pkRealloc_ptr: pkRealloc_t,
    pub pkReleaseHandle_ptr: pkReleaseHandle_t,
    pub pkNewModule_ptr: pkNewModule_t,
    pub pkRegisterModule_ptr: pkRegisterModule_t,
    pub pkModuleAddFunction_ptr: pkModuleAddFunction_t,
    pub pkNewClass_ptr: pkNewClass_t,
    pub pkClassAddMethod_ptr: pkClassAddMethod_t,
    pub pkModuleAddSource_ptr: pkModuleAddSource_t,
    pub pkRunString_ptr: pkRunString_t,
    pub pkRunFile_ptr: pkRunFile_t,
    pub pkRunREPL_ptr: pkRunREPL_t,
    pub pkSetRuntimeError_ptr: pkSetRuntimeError_t,
    pub pkGetSelf_ptr: pkGetSelf_t,
    pub pkGetArgc_ptr: pkGetArgc_t,
    pub pkCheckArgcRange_ptr: pkCheckArgcRange_t,
    pub pkValidateSlotBool_ptr: pkValidateSlotBool_t,
    pub pkValidateSlotNumber_ptr: pkValidateSlotNumber_t,
    pub pkValidateSlotInteger_ptr: pkValidateSlotInteger_t,
    pub pkValidateSlotString_ptr: pkValidateSlotString_t,
    pub pkValidateSlotType_ptr: pkValidateSlotType_t,
    pub pkValidateSlotInstanceOf_ptr: pkValidateSlotInstanceOf_t,
    pub pkIsSlotInstanceOf_ptr: pkIsSlotInstanceOf_t,
    pub pkReserveSlots_ptr: pkReserveSlots_t,
    pub pkGetSlotsCount_ptr: pkGetSlotsCount_t,
    pub pkGetSlotType_ptr: pkGetSlotType_t,
    pub pkGetSlotBool_ptr: pkGetSlotBool_t,
    pub pkGetSlotNumber_ptr: pkGetSlotNumber_t,
    pub pkGetSlotString_ptr: pkGetSlotString_t,
    pub pkGetSlotHandle_ptr: pkGetSlotHandle_t,
    pub pkGetSlotNativeInstance_ptr: pkGetSlotNativeInstance_t,
    pub pkSetSlotNull_ptr: pkSetSlotNull_t,
    pub pkSetSlotBool_ptr: pkSetSlotBool_t,
    pub pkSetSlotNumber_ptr: pkSetSlotNumber_t,
    pub pkSetSlotString_ptr: pkSetSlotString_t,
    pub pkSetSlotStringLength_ptr: pkSetSlotStringLength_t,
    pub pkSetSlotHandle_ptr: pkSetSlotHandle_t,
    pub pkGetSlotHash_ptr: pkGetSlotHash_t,
    pub pkPlaceSelf_ptr: pkPlaceSelf_t,
    pub pkGetClass_ptr: pkGetClass_t,
    pub pkNewInstance_ptr: pkNewInstance_t,
    pub pkNewRange_ptr: pkNewRange_t,
    pub pkNewList_ptr: pkNewList_t,
    pub pkNewMap_ptr: pkNewMap_t,
    pub pkListInsert_ptr: pkListInsert_t,
    pub pkListPop_ptr: pkListPop_t,
    pub pkListLength_ptr: pkListLength_t,
    pub pkCallFunction_ptr: pkCallFunction_t,
    pub pkCallMethod_ptr: pkCallMethod_t,
    pub pkGetAttribute_ptr: pkGetAttribute_t,
    pub pkSetAttribute_ptr: pkSetAttribute_t,
    pub pkImportModule_ptr: pkImportModule_t,
}
pub type pkImportModule_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char, libc::c_int) -> bool,
>;
pub type pkSetAttribute_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
    ) -> bool,
>;
pub type pkGetAttribute_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
    ) -> bool,
>;
pub type pkCallMethod_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> bool,
>;
pub type pkCallFunction_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> bool,
>;
pub type pkListLength_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int) -> uint32_t,
>;
pub type pkListPop_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, int32_t, libc::c_int) -> bool,
>;
pub type pkListInsert_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, int32_t, libc::c_int) -> bool,
>;
pub type pkNewMap_t = Option::<unsafe extern "C" fn(*mut PKVM, libc::c_int) -> ()>;
pub type pkNewList_t = Option::<unsafe extern "C" fn(*mut PKVM, libc::c_int) -> ()>;
pub type pkNewRange_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_double, libc::c_double) -> (),
>;
pub type pkNewInstance_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> bool,
>;
pub type pkGetClass_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_int) -> (),
>;
pub type pkPlaceSelf_t = Option::<unsafe extern "C" fn(*mut PKVM, libc::c_int) -> ()>;
pub type pkGetSlotHash_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int) -> uint32_t,
>;
pub type pkSetSlotHandle_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut PkHandle) -> (),
>;
pub type pkSetSlotStringLength_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, *const libc::c_char, uint32_t) -> (),
>;
pub type pkSetSlotString_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, *const libc::c_char) -> (),
>;
pub type pkSetSlotNumber_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_double) -> (),
>;
pub type pkSetSlotBool_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, bool) -> (),
>;
pub type pkSetSlotNull_t = Option::<unsafe extern "C" fn(*mut PKVM, libc::c_int) -> ()>;
pub type pkGetSlotNativeInstance_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int) -> *mut libc::c_void,
>;
pub type pkGetSlotHandle_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int) -> *mut PkHandle,
>;
pub type pkGetSlotString_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut uint32_t) -> *const libc::c_char,
>;
pub type pkGetSlotNumber_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int) -> libc::c_double,
>;
pub type pkGetSlotBool_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int) -> bool,
>;
pub type pkGetSlotType_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int) -> PkVarType,
>;
pub type pkGetSlotsCount_t = Option::<unsafe extern "C" fn(*mut PKVM) -> libc::c_int>;
pub type pkReserveSlots_t = Option::<unsafe extern "C" fn(*mut PKVM, libc::c_int) -> ()>;
pub type pkIsSlotInstanceOf_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_int, *mut bool) -> bool,
>;
pub type pkValidateSlotInstanceOf_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_int) -> bool,
>;
pub type pkValidateSlotType_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, PkVarType) -> bool,
>;
pub type pkValidateSlotString_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        libc::c_int,
        *mut *const libc::c_char,
        *mut uint32_t,
    ) -> bool,
>;
pub type pkValidateSlotInteger_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut int32_t) -> bool,
>;
pub type pkValidateSlotNumber_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut libc::c_double) -> bool,
>;
pub type pkValidateSlotBool_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut bool) -> bool,
>;
pub type pkCheckArgcRange_t = Option::<
    unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_int, libc::c_int) -> bool,
>;
pub type pkGetArgc_t = Option::<unsafe extern "C" fn(*const PKVM) -> libc::c_int>;
pub type pkGetSelf_t = Option::<unsafe extern "C" fn(*const PKVM) -> *mut libc::c_void>;
pub type pkSetRuntimeError_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
>;
pub type pkRunREPL_t = Option::<unsafe extern "C" fn(*mut PKVM) -> PkResult>;
pub type pkRunFile_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> PkResult,
>;
pub type pkRunString_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> PkResult,
>;
pub type pkModuleAddSource_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut PkHandle, *const libc::c_char) -> (),
>;
pub type pkClassAddMethod_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        *mut PkHandle,
        *const libc::c_char,
        pkNativeFn,
        libc::c_int,
        *const libc::c_char,
    ) -> (),
>;
pub type pkNewClass_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        *const libc::c_char,
        *mut PkHandle,
        *mut PkHandle,
        pkNewInstanceFn,
        pkDeleteInstanceFn,
        *const libc::c_char,
    ) -> *mut PkHandle,
>;
pub type pkModuleAddFunction_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        *mut PkHandle,
        *const libc::c_char,
        pkNativeFn,
        libc::c_int,
        *const libc::c_char,
    ) -> (),
>;
pub type pkRegisterModule_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut PkHandle) -> (),
>;
pub type pkNewModule_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut PkHandle,
>;
pub type pkReleaseHandle_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut PkHandle) -> (),
>;
pub type pkRealloc_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type pkAddSearchPath_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
>;
pub type pkRegisterBuiltinFn_t = Option::<
    unsafe extern "C" fn(
        *mut PKVM,
        *const libc::c_char,
        pkNativeFn,
        libc::c_int,
        *const libc::c_char,
    ) -> (),
>;
pub type pkGetUserData_t = Option::<
    unsafe extern "C" fn(*const PKVM) -> *mut libc::c_void,
>;
pub type pkSetUserData_t = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
>;
pub type pkFreeVM_t = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
pub type pkNewVM_t = Option::<unsafe extern "C" fn(*mut PkConfiguration) -> *mut PKVM>;
pub type pkNewConfiguration_t = Option::<unsafe extern "C" fn() -> PkConfiguration>;
pub type pkInitApiFn = Option::<unsafe extern "C" fn(*mut PkNativeApi) -> ()>;
pub type pkExportModuleFn = Option::<unsafe extern "C" fn(*mut PKVM) -> *mut PkHandle>;
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
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub unsafe extern "C" fn pkMakeNativeAPI() -> PkNativeApi {
    let mut api: PkNativeApi = PkNativeApi {
        pkNewConfiguration_ptr: None,
        pkNewVM_ptr: None,
        pkFreeVM_ptr: None,
        pkSetUserData_ptr: None,
        pkGetUserData_ptr: None,
        pkRegisterBuiltinFn_ptr: None,
        pkAddSearchPath_ptr: None,
        pkRealloc_ptr: None,
        pkReleaseHandle_ptr: None,
        pkNewModule_ptr: None,
        pkRegisterModule_ptr: None,
        pkModuleAddFunction_ptr: None,
        pkNewClass_ptr: None,
        pkClassAddMethod_ptr: None,
        pkModuleAddSource_ptr: None,
        pkRunString_ptr: None,
        pkRunFile_ptr: None,
        pkRunREPL_ptr: None,
        pkSetRuntimeError_ptr: None,
        pkGetSelf_ptr: None,
        pkGetArgc_ptr: None,
        pkCheckArgcRange_ptr: None,
        pkValidateSlotBool_ptr: None,
        pkValidateSlotNumber_ptr: None,
        pkValidateSlotInteger_ptr: None,
        pkValidateSlotString_ptr: None,
        pkValidateSlotType_ptr: None,
        pkValidateSlotInstanceOf_ptr: None,
        pkIsSlotInstanceOf_ptr: None,
        pkReserveSlots_ptr: None,
        pkGetSlotsCount_ptr: None,
        pkGetSlotType_ptr: None,
        pkGetSlotBool_ptr: None,
        pkGetSlotNumber_ptr: None,
        pkGetSlotString_ptr: None,
        pkGetSlotHandle_ptr: None,
        pkGetSlotNativeInstance_ptr: None,
        pkSetSlotNull_ptr: None,
        pkSetSlotBool_ptr: None,
        pkSetSlotNumber_ptr: None,
        pkSetSlotString_ptr: None,
        pkSetSlotStringLength_ptr: None,
        pkSetSlotHandle_ptr: None,
        pkGetSlotHash_ptr: None,
        pkPlaceSelf_ptr: None,
        pkGetClass_ptr: None,
        pkNewInstance_ptr: None,
        pkNewRange_ptr: None,
        pkNewList_ptr: None,
        pkNewMap_ptr: None,
        pkListInsert_ptr: None,
        pkListPop_ptr: None,
        pkListLength_ptr: None,
        pkCallFunction_ptr: None,
        pkCallMethod_ptr: None,
        pkGetAttribute_ptr: None,
        pkSetAttribute_ptr: None,
        pkImportModule_ptr: None,
    };
    api
        .pkNewConfiguration_ptr = Some(
        ::std::mem::transmute::<
            unsafe extern "C" fn() -> PkConfiguration,
            unsafe extern "C" fn() -> PkConfiguration,
        >(pkNewConfiguration),
    );
    api
        .pkNewVM_ptr = Some(
        pkNewVM as unsafe extern "C" fn(*mut PkConfiguration) -> *mut PKVM,
    );
    api.pkFreeVM_ptr = Some(pkFreeVM as unsafe extern "C" fn(*mut PKVM) -> ());
    api
        .pkSetUserData_ptr = Some(
        pkSetUserData as unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
    );
    api
        .pkGetUserData_ptr = Some(
        pkGetUserData as unsafe extern "C" fn(*const PKVM) -> *mut libc::c_void,
    );
    api
        .pkRegisterBuiltinFn_ptr = Some(
        pkRegisterBuiltinFn
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                pkNativeFn,
                libc::c_int,
                *const libc::c_char,
            ) -> (),
    );
    api
        .pkAddSearchPath_ptr = Some(
        pkAddSearchPath as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
    );
    api
        .pkRealloc_ptr = Some(
        pkRealloc
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut libc::c_void,
                size_t,
            ) -> *mut libc::c_void,
    );
    api
        .pkReleaseHandle_ptr = Some(
        pkReleaseHandle as unsafe extern "C" fn(*mut PKVM, *mut PkHandle) -> (),
    );
    api
        .pkNewModule_ptr = Some(
        pkNewModule
            as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut PkHandle,
    );
    api
        .pkRegisterModule_ptr = Some(
        pkRegisterModule as unsafe extern "C" fn(*mut PKVM, *mut PkHandle) -> (),
    );
    api
        .pkModuleAddFunction_ptr = Some(
        pkModuleAddFunction
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut PkHandle,
                *const libc::c_char,
                pkNativeFn,
                libc::c_int,
                *const libc::c_char,
            ) -> (),
    );
    api
        .pkNewClass_ptr = Some(
        pkNewClass
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                *mut PkHandle,
                *mut PkHandle,
                pkNewInstanceFn,
                pkDeleteInstanceFn,
                *const libc::c_char,
            ) -> *mut PkHandle,
    );
    api
        .pkClassAddMethod_ptr = Some(
        pkClassAddMethod
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut PkHandle,
                *const libc::c_char,
                pkNativeFn,
                libc::c_int,
                *const libc::c_char,
            ) -> (),
    );
    api
        .pkModuleAddSource_ptr = Some(
        pkModuleAddSource
            as unsafe extern "C" fn(*mut PKVM, *mut PkHandle, *const libc::c_char) -> (),
    );
    api
        .pkRunString_ptr = Some(
        pkRunString as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> PkResult,
    );
    api
        .pkRunFile_ptr = Some(
        pkRunFile as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> PkResult,
    );
    api.pkRunREPL_ptr = Some(pkRunREPL as unsafe extern "C" fn(*mut PKVM) -> PkResult);
    api
        .pkSetRuntimeError_ptr = Some(
        pkSetRuntimeError as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
    );
    api
        .pkGetSelf_ptr = Some(
        pkGetSelf as unsafe extern "C" fn(*const PKVM) -> *mut libc::c_void,
    );
    api
        .pkGetArgc_ptr = Some(
        pkGetArgc as unsafe extern "C" fn(*const PKVM) -> libc::c_int,
    );
    api
        .pkCheckArgcRange_ptr = Some(
        pkCheckArgcRange
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> bool,
    );
    api
        .pkValidateSlotBool_ptr = Some(
        pkValidateSlotBool
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut bool) -> bool,
    );
    api
        .pkValidateSlotNumber_ptr = Some(
        pkValidateSlotNumber
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut libc::c_double) -> bool,
    );
    api
        .pkValidateSlotInteger_ptr = Some(
        pkValidateSlotInteger
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut int32_t) -> bool,
    );
    api
        .pkValidateSlotString_ptr = Some(
        pkValidateSlotString
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                *mut *const libc::c_char,
                *mut uint32_t,
            ) -> bool,
    );
    api
        .pkValidateSlotType_ptr = Some(
        pkValidateSlotType
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, PkVarType) -> bool,
    );
    api
        .pkValidateSlotInstanceOf_ptr = Some(
        pkValidateSlotInstanceOf
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_int) -> bool,
    );
    api
        .pkIsSlotInstanceOf_ptr = Some(
        pkIsSlotInstanceOf
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                libc::c_int,
                *mut bool,
            ) -> bool,
    );
    api
        .pkReserveSlots_ptr = Some(
        pkReserveSlots as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> (),
    );
    api
        .pkGetSlotsCount_ptr = Some(
        pkGetSlotsCount as unsafe extern "C" fn(*mut PKVM) -> libc::c_int,
    );
    api
        .pkGetSlotType_ptr = Some(
        pkGetSlotType as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> PkVarType,
    );
    api
        .pkGetSlotBool_ptr = Some(
        pkGetSlotBool as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> bool,
    );
    api
        .pkGetSlotNumber_ptr = Some(
        pkGetSlotNumber as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> libc::c_double,
    );
    api
        .pkGetSlotString_ptr = Some(
        pkGetSlotString
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                *mut uint32_t,
            ) -> *const libc::c_char,
    );
    api
        .pkGetSlotHandle_ptr = Some(
        pkGetSlotHandle as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> *mut PkHandle,
    );
    api
        .pkGetSlotNativeInstance_ptr = Some(
        pkGetSlotNativeInstance
            as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> *mut libc::c_void,
    );
    api
        .pkSetSlotNull_ptr = Some(
        pkSetSlotNull as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> (),
    );
    api
        .pkSetSlotBool_ptr = Some(
        pkSetSlotBool as unsafe extern "C" fn(*mut PKVM, libc::c_int, bool) -> (),
    );
    api
        .pkSetSlotNumber_ptr = Some(
        pkSetSlotNumber
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_double) -> (),
    );
    api
        .pkSetSlotString_ptr = Some(
        pkSetSlotString
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, *const libc::c_char) -> (),
    );
    api
        .pkSetSlotStringLength_ptr = Some(
        pkSetSlotStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                *const libc::c_char,
                uint32_t,
            ) -> (),
    );
    api
        .pkSetSlotHandle_ptr = Some(
        pkSetSlotHandle
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, *mut PkHandle) -> (),
    );
    api
        .pkGetSlotHash_ptr = Some(
        pkGetSlotHash as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> uint32_t,
    );
    api
        .pkPlaceSelf_ptr = Some(
        pkPlaceSelf as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> (),
    );
    api
        .pkGetClass_ptr = Some(
        pkGetClass as unsafe extern "C" fn(*mut PKVM, libc::c_int, libc::c_int) -> (),
    );
    api
        .pkNewInstance_ptr = Some(
        pkNewInstance
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> bool,
    );
    api
        .pkNewRange_ptr = Some(
        pkNewRange
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                libc::c_double,
                libc::c_double,
            ) -> (),
    );
    api
        .pkNewList_ptr = Some(
        pkNewList as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> (),
    );
    api
        .pkNewMap_ptr = Some(
        pkNewMap as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> (),
    );
    api
        .pkListInsert_ptr = Some(
        pkListInsert
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, int32_t, libc::c_int) -> bool,
    );
    api
        .pkListPop_ptr = Some(
        pkListPop
            as unsafe extern "C" fn(*mut PKVM, libc::c_int, int32_t, libc::c_int) -> bool,
    );
    api
        .pkListLength_ptr = Some(
        pkListLength as unsafe extern "C" fn(*mut PKVM, libc::c_int) -> uint32_t,
    );
    api
        .pkCallFunction_ptr = Some(
        pkCallFunction
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> bool,
    );
    api
        .pkCallMethod_ptr = Some(
        pkCallMethod
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                *const libc::c_char,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> bool,
    );
    api
        .pkGetAttribute_ptr = Some(
        pkGetAttribute
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                *const libc::c_char,
                libc::c_int,
            ) -> bool,
    );
    api
        .pkSetAttribute_ptr = Some(
        pkSetAttribute
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_int,
                *const libc::c_char,
                libc::c_int,
            ) -> bool,
    );
    api
        .pkImportModule_ptr = Some(
        pkImportModule
            as unsafe extern "C" fn(*mut PKVM, *const libc::c_char, libc::c_int) -> bool,
    );
    return api;
}
pub unsafe extern "C" fn osLoadDL(
    mut vm: *mut PKVM,
    mut path: *const libc::c_char,
) -> *mut libc::c_void {
    let mut handle: *mut libc::c_void = dlopen(path, 0x1 as libc::c_int);
    if handle.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut init_fn: pkInitApiFn = ::std::mem::transmute::<
        *mut libc::c_void,
        pkInitApiFn,
    >(dlsym(handle, b"pkInitApi\0" as *const u8 as *const libc::c_char));
    if init_fn.is_none() {
        dlclose(handle) != 0;
        dlerror();
        return 0 as *mut libc::c_void;
    }
    let mut api: PkNativeApi = pkMakeNativeAPI();
    init_fn.unwrap()(&mut api);
    return handle;
}
pub unsafe extern "C" fn osImportDL(
    mut vm: *mut PKVM,
    mut handle: *mut libc::c_void,
) -> *mut PkHandle {
    let mut export_fn: pkExportModuleFn = ::std::mem::transmute::<
        *mut libc::c_void,
        pkExportModuleFn,
    >(dlsym(handle, b"pkExportModule\0" as *const u8 as *const libc::c_char));
    if export_fn.is_none() {
        dlerror();
        return 0 as *mut PkHandle;
    }
    let mut module: *mut PkHandle = export_fn.unwrap()(vm);
    dlerror();
    return module;
}
pub unsafe extern "C" fn osUnloadDL(mut vm: *mut PKVM, mut handle: *mut libc::c_void) {
    dlclose(handle);
}
pub unsafe extern "C" fn osGetExeFilePath(
    mut buff: *mut libc::c_char,
    mut size: libc::c_int,
) -> bool {
    let mut tmp: [libc::c_char; 4096] = [0; 4096];
    sprintf(
        tmp.as_mut_ptr(),
        b"/proc/%d/exe\0" as *const u8 as *const libc::c_char,
        getpid(),
    );
    let mut len: libc::c_int = readlink(tmp.as_mut_ptr(), buff, size as size_t)
        as libc::c_int;
    *buff.offset(len as isize) = '\0' as i32 as libc::c_char;
    return 1 as libc::c_int != 0;
}
static mut _pk_doc__osGetCWD: *const libc::c_char = b"os.getcwd() -> String\n\nReturns the current working directory\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osGetCWD(mut vm: *mut PKVM) {
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    (getcwd(
        cwd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null();
    pkSetSlotString(vm, 0 as libc::c_int, cwd.as_mut_ptr());
}
static mut _pk_doc__osChdir: *const libc::c_char = b"os.chdir(path:String)\n\nChange the current working directory\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osChdir(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    if chdir(path) != 0 {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.chdir errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
    }
}
static mut _pk_doc__osMkdir: *const libc::c_char = b"os.mkdir(path:String)\n\nCreates a directory at the path. The path should be valid.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osMkdir(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    if mkdir(path, 0x1ff as libc::c_int as __mode_t) != 0 {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.mkdir errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn _osRmdir(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    if rmdir(path) != 0 {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.rmdir errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
    }
}
static mut _pk_doc__osRmdir: *const libc::c_char = b"os.rmdir(path:String)\n\nRemoves an empty directory at the path.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osUnlink(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    if unlink(path) != 0 {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.unlink errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
    }
}
static mut _pk_doc__osUnlink: *const libc::c_char = b"os.rmdir(path:String)\n\nRemoves a file at the path.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__osModitime: *const libc::c_char = b"os.moditime(path:String) -> Number\n\nReturns the modified timestamp of the file.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osModitime(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    let mut mtime: libc::c_double = 0 as libc::c_int as libc::c_double;
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
    if stat(path, &mut path_stat) == 0 as libc::c_int {
        mtime = path_stat.st_mtim.tv_sec as libc::c_double;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, mtime);
}
unsafe extern "C" fn _osFileSize(mut vm: *mut PKVM) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
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
    if stat(path, &mut path_stat) != 0
        || path_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            != 0o100000 as libc::c_int as libc::c_uint
    {
        pkSetRuntimeErrorFmt(
            vm,
            b"Path '%s' wasn't a file.\0" as *const u8 as *const libc::c_char,
            path,
        );
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, path_stat.st_size as libc::c_double);
}
static mut _pk_doc__osFileSize: *const libc::c_char = b"os.filesize(path:String) -> Number\n\nReturns the file size in bytes.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__osSystem: *const libc::c_char = b"os.system(cmd:String) -> Number\n\nExecute the command in a subprocess, Returns the exit code of the child process.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osSystem(mut vm: *mut PKVM) {
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut cmd, 0 as *mut uint32_t) {
        return;
    }
    *__errno_location() = 0 as libc::c_int;
    let mut code: libc::c_int = system(cmd);
    if *__errno_location() != 0 as libc::c_int {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.system errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, code as libc::c_double);
}
static mut _pk_doc__osGetenv: *const libc::c_char = b"os.getenv(name:String) -> String\n\nReturns the environment variable as String if it exists otherwise it'll return null.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osGetenv(mut vm: *mut PKVM) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut name, 0 as *mut uint32_t) {
        return;
    }
    let mut value: *const libc::c_char = getenv(name);
    if value.is_null() {
        pkSetSlotNull(vm, 0 as libc::c_int);
        return;
    }
    pkSetSlotString(vm, 0 as libc::c_int, value);
}
static mut _pk_doc__osExepath: *const libc::c_char = b"os.exepath() -> String\n\nReturns the path of the pocket interpreter executable.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _osExepath(mut vm: *mut PKVM) {
    let mut buff: [libc::c_char; 4096] = [0; 4096];
    if !osGetExeFilePath(buff.as_mut_ptr(), 4096 as libc::c_int) {
        pkSetRuntimeError(
            vm,
            b"Cannot obtain ececutable path.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    pkSetSlotString(vm, 0 as libc::c_int, buff.as_mut_ptr());
}
pub unsafe extern "C" fn registerModuleOS(mut vm: *mut PKVM) {
    let mut os: *mut PkHandle = pkNewModule(
        vm,
        b"os\0" as *const u8 as *const libc::c_char,
    );
    pkReserveSlots(vm, 2 as libc::c_int);
    pkSetSlotHandle(vm, 0 as libc::c_int, os);
    pkSetSlotString(
        vm,
        1 as libc::c_int,
        b"linux\0" as *const u8 as *const libc::c_char,
    );
    pkSetAttribute(
        vm,
        0 as libc::c_int,
        b"NAME\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"getcwd\0" as *const u8 as *const libc::c_char,
        Some(_osGetCWD as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__osGetCWD,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"chdir\0" as *const u8 as *const libc::c_char,
        Some(_osChdir as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osChdir,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"mkdir\0" as *const u8 as *const libc::c_char,
        Some(_osMkdir as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osMkdir,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"rmdir\0" as *const u8 as *const libc::c_char,
        Some(_osRmdir as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osRmdir,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"unlink\0" as *const u8 as *const libc::c_char,
        Some(_osUnlink as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osUnlink,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"moditime\0" as *const u8 as *const libc::c_char,
        Some(_osModitime as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osModitime,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"filesize\0" as *const u8 as *const libc::c_char,
        Some(_osFileSize as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osFileSize,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"system\0" as *const u8 as *const libc::c_char,
        Some(_osSystem as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osSystem,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"getenv\0" as *const u8 as *const libc::c_char,
        Some(_osGetenv as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__osGetenv,
    );
    pkModuleAddFunction(
        vm,
        os,
        b"exepath\0" as *const u8 as *const libc::c_char,
        Some(_osExepath as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__osExepath,
    );
    pkRegisterModule(vm, os);
    pkReleaseHandle(vm, os);
}
