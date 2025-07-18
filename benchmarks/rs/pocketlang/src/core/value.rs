use ::libc;
extern "C" {
    pub type Compiler;
    
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn utilPowerOf2Ceil(n: libc::c_int) -> libc::c_int;
    fn utilHexDigit(value: uint8_t, uppercase: bool) -> libc::c_char;
    fn utilDoubleToBits(value: libc::c_double) -> uint64_t;
    fn utilDoubleFromBits(value: uint64_t) -> libc::c_double;
    fn utilHashBits(hash: uint64_t) -> uint32_t;
    fn utilHashNumber(num: libc::c_double) -> uint32_t;
    fn utilHashString(string: *const libc::c_char) -> uint32_t;
    fn vmRealloc(
        vm: *mut PKVM,
        memory: *mut libc::c_void,
        old_size: size_t,
        new_size: size_t,
    ) -> *mut libc::c_void;
    fn vmPushTempRef(vm: *mut PKVM, obj: *mut Object);
    fn vmPopTempRef(vm: *mut PKVM);
    fn vmUnloadDlHandle(vm: *mut PKVM, handle: *mut libc::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PKVM {
    pub first: *mut Object,
    pub bytes_allocated: size_t,
    pub next_gc: size_t,
    pub collecting_garbage: bool,
    pub min_heap_size: size_t,
    pub heap_fill_percent: libc::c_int,
    pub working_set: *mut *mut Object,
    pub working_set_count: libc::c_int,
    pub working_set_capacity: libc::c_int,
    pub temp_reference: [*mut Object; 64],
    pub temp_reference_count: libc::c_int,
    pub handles: *mut PkHandle,
    pub config: PkConfiguration,
    pub compiler: *mut Compiler,
    pub modules: *mut Map,
    pub search_paths: *mut List,
    pub builtins_funcs: [*mut Closure; 50],
    pub builtins_count: libc::c_int,
    pub builtin_classes: [*mut Class; 13],
    pub fiber: *mut Fiber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fiber {
    pub _super: Object,
    pub state: FiberState,
    pub closure: *mut Closure,
    pub stack: *mut Var,
    pub stack_size: libc::c_int,
    pub sp: *mut Var,
    pub frames: *mut CallFrame,
    pub frame_capacity: libc::c_int,
    pub frame_count: libc::c_int,
    pub open_upvalues: *mut Upvalue,
    pub ret: *mut Var,
    pub self_0: Var,
    pub caller: *mut Fiber,
    pub native: *mut Fiber,
    pub error: *mut String_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct String_0 {
    pub _super: Object,
    pub hash: uint32_t,
    pub length: uint32_t,
    pub capacity: uint32_t,
    pub data: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Object {
    pub type_0: ObjectType,
    pub is_marked: bool,
    pub next: *mut Object,
}
pub type ObjectType = libc::c_uint;
pub const OBJ_INST: ObjectType = 11;
pub const OBJ_CLASS: ObjectType = 10;
pub const OBJ_FIBER: ObjectType = 9;
pub const OBJ_UPVALUE: ObjectType = 8;
pub const OBJ_METHOD_BIND: ObjectType = 7;
pub const OBJ_CLOSURE: ObjectType = 6;
pub const OBJ_FUNC: ObjectType = 5;
pub const OBJ_MODULE: ObjectType = 4;
pub const OBJ_RANGE: ObjectType = 3;
pub const OBJ_MAP: ObjectType = 2;
pub const OBJ_LIST: ObjectType = 1;
pub const OBJ_STRING: ObjectType = 0;
pub type Var = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvalue {
    pub _super: Object,
    pub ptr: *mut Var,
    pub closed: Var,
    pub next: *mut Upvalue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallFrame {
    pub ip: *const uint8_t,
    pub closure: *const Closure,
    pub rbp: *mut Var,
    pub self_0: Var,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Closure {
    pub _super: Object,
    pub fn_0: *mut Function,
    pub upvalues: [*mut Upvalue; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Function {
    pub _super: Object,
    pub owner: *mut Module,
    pub name: *const libc::c_char,
    pub arity: libc::c_int,
    pub is_method: bool,
    pub upvalue_count: libc::c_int,
    pub docstring: *const libc::c_char,
    pub is_native: bool,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub native: pkNativeFn,
    pub fn_0: *mut Fn_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fn_0 {
    pub opcodes: pkByteBuffer,
    pub oplines: pkUintBuffer,
    pub stack_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkUintBuffer {
    pub data: *mut uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkByteBuffer {
    pub data: *mut uint8_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub type pkNativeFn = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Module {
    pub _super: Object,
    pub name: *mut String_0,
    pub path: *mut String_0,
    pub constants: pkVarBuffer,
    pub globals: pkVarBuffer,
    pub global_names: pkUintBuffer,
    pub body: *mut Closure,
    pub initialized: bool,
    pub handle: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkVarBuffer {
    pub data: *mut Var,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub type FiberState = libc::c_uint;
pub const FIBER_DONE: FiberState = 3;
pub const FIBER_YIELDED: FiberState = 2;
pub const FIBER_RUNNING: FiberState = 1;
pub const FIBER_NEW: FiberState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class {
    pub _super: Object,
    pub super_class: *mut Class,
    pub owner: *mut Module,
    pub name: *mut String_0,
    pub docstring: *const libc::c_char,
    pub class_of: PkVarType,
    pub ctor: *mut Closure,
    pub methods: pkClosureBuffer,
    pub static_attribs: *mut Map,
    pub new_fn: pkNewInstanceFn,
    pub delete_fn: pkDeleteInstanceFn,
}
pub type pkDeleteInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
>;
pub type pkNewInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Map {
    pub _super: Object,
    pub capacity: uint32_t,
    pub count: uint32_t,
    pub entries: *mut MapEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapEntry {
    pub key: Var,
    pub value: Var,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkClosureBuffer {
    pub data: *mut *mut Closure,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub type PkVarType = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub _super: Object,
    pub elements: pkVarBuffer,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PkHandle {
    pub value: Var,
    pub prev: *mut PkHandle,
    pub next: *mut PkHandle,
}
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
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub _super: Object,
    pub from: libc::c_double,
    pub to: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MethodBind {
    pub _super: Object,
    pub method: *mut Closure,
    pub instance: Var,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Instance {
    pub _super: Object,
    pub cls: *mut Class,
    pub native: *mut libc::c_void,
    pub attribs: *mut Map,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkStringBuffer {
    pub data: *mut *mut String_0,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OuterSequence {
    pub outer: *mut OuterSequence,
    pub is_list: bool,
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub list: *const List,
    pub map: *const Map,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn pkUintBufferClear(
    mut self_0: *mut pkUintBuffer,
    mut vm: *mut PKVM,
) {
    vmRealloc(
        vm,
        (*self_0).data as *mut libc::c_void,
        ((*self_0).capacity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    (*self_0).data = 0 as *mut uint32_t;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkUintBufferReserve(
    mut self_0: *mut pkUintBuffer,
    mut vm: *mut PKVM,
    mut size: size_t,
) {
    if ((*self_0).capacity as libc::c_ulong) < size {
        let mut capacity: libc::c_int = utilPowerOf2Ceil(size as libc::c_int);
        if capacity < 8 as libc::c_int {
            capacity = 8 as libc::c_int;
        }
        (*self_0)
            .data = vmRealloc(
            vm,
            (*self_0).data as *mut libc::c_void,
            ((*self_0).capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            (capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
        ) as *mut uint32_t;
        (*self_0).capacity = capacity as uint32_t;
    }
}
pub unsafe extern "C" fn pkUintBufferFill(
    mut self_0: *mut pkUintBuffer,
    mut vm: *mut PKVM,
    mut data: uint32_t,
    mut count: libc::c_int,
) {
    pkUintBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add(count as libc::c_uint) as size_t,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let fresh0 = (*self_0).count;
        (*self_0).count = ((*self_0).count).wrapping_add(1);
        *((*self_0).data).offset(fresh0 as isize) = data;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn pkUintBufferWrite(
    mut self_0: *mut pkUintBuffer,
    mut vm: *mut PKVM,
    mut data: uint32_t,
) {
    pkUintBufferFill(self_0, vm, data, 1 as libc::c_int);
}
pub unsafe extern "C" fn pkUintBufferConcat(
    mut self_0: *mut pkUintBuffer,
    mut vm: *mut PKVM,
    mut other: *mut pkUintBuffer,
) {
    pkUintBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add((*other).count) as size_t,
    );
    memcpy(
        ((*self_0).data).offset((*self_0).count as isize) as *mut libc::c_void,
        (*other).data as *const libc::c_void,
        ((*other).count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
    (*self_0)
        .count = ((*self_0).count as libc::c_uint).wrapping_add((*other).count)
        as uint32_t as uint32_t;
}
pub unsafe extern "C" fn pkUintBufferInit(mut self_0: *mut pkUintBuffer) {
    (*self_0).data = 0 as *mut uint32_t;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkByteBufferInit(mut self_0: *mut pkByteBuffer) {
    (*self_0).data = 0 as *mut uint8_t;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkByteBufferClear(
    mut self_0: *mut pkByteBuffer,
    mut vm: *mut PKVM,
) {
    vmRealloc(
        vm,
        (*self_0).data as *mut libc::c_void,
        ((*self_0).capacity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    (*self_0).data = 0 as *mut uint8_t;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkByteBufferConcat(
    mut self_0: *mut pkByteBuffer,
    mut vm: *mut PKVM,
    mut other: *mut pkByteBuffer,
) {
    pkByteBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add((*other).count) as size_t,
    );
    memcpy(
        ((*self_0).data).offset((*self_0).count as isize) as *mut libc::c_void,
        (*other).data as *const libc::c_void,
        ((*other).count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
    );
    (*self_0)
        .count = ((*self_0).count as libc::c_uint).wrapping_add((*other).count)
        as uint32_t as uint32_t;
}
pub unsafe extern "C" fn pkByteBufferWrite(
    mut self_0: *mut pkByteBuffer,
    mut vm: *mut PKVM,
    mut data: uint8_t,
) {
    pkByteBufferFill(self_0, vm, data, 1 as libc::c_int);
}
pub unsafe extern "C" fn pkByteBufferFill(
    mut self_0: *mut pkByteBuffer,
    mut vm: *mut PKVM,
    mut data: uint8_t,
    mut count: libc::c_int,
) {
    pkByteBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add(count as libc::c_uint) as size_t,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let fresh1 = (*self_0).count;
        (*self_0).count = ((*self_0).count).wrapping_add(1);
        *((*self_0).data).offset(fresh1 as isize) = data;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn pkByteBufferReserve(
    mut self_0: *mut pkByteBuffer,
    mut vm: *mut PKVM,
    mut size: size_t,
) {
    if ((*self_0).capacity as libc::c_ulong) < size {
        let mut capacity: libc::c_int = utilPowerOf2Ceil(size as libc::c_int);
        if capacity < 8 as libc::c_int {
            capacity = 8 as libc::c_int;
        }
        (*self_0)
            .data = vmRealloc(
            vm,
            (*self_0).data as *mut libc::c_void,
            ((*self_0).capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
            (capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t;
        (*self_0).capacity = capacity as uint32_t;
    }
}
pub unsafe extern "C" fn pkVarBufferConcat(
    mut self_0: *mut pkVarBuffer,
    mut vm: *mut PKVM,
    mut other: *mut pkVarBuffer,
) {
    pkVarBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add((*other).count) as size_t,
    );
    memcpy(
        ((*self_0).data).offset((*self_0).count as isize) as *mut libc::c_void,
        (*other).data as *const libc::c_void,
        ((*other).count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Var>() as libc::c_ulong),
    );
    (*self_0)
        .count = ((*self_0).count as libc::c_uint).wrapping_add((*other).count)
        as uint32_t as uint32_t;
}
pub unsafe extern "C" fn pkVarBufferWrite(
    mut self_0: *mut pkVarBuffer,
    mut vm: *mut PKVM,
    mut data: Var,
) {
    pkVarBufferFill(self_0, vm, data, 1 as libc::c_int);
}
pub unsafe extern "C" fn pkVarBufferFill(
    mut self_0: *mut pkVarBuffer,
    mut vm: *mut PKVM,
    mut data: Var,
    mut count: libc::c_int,
) {
    pkVarBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add(count as libc::c_uint) as size_t,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let fresh2 = (*self_0).count;
        (*self_0).count = ((*self_0).count).wrapping_add(1);
        *((*self_0).data).offset(fresh2 as isize) = data;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn pkVarBufferReserve(
    mut self_0: *mut pkVarBuffer,
    mut vm: *mut PKVM,
    mut size: size_t,
) {
    if ((*self_0).capacity as libc::c_ulong) < size {
        let mut capacity: libc::c_int = utilPowerOf2Ceil(size as libc::c_int);
        if capacity < 8 as libc::c_int {
            capacity = 8 as libc::c_int;
        }
        (*self_0)
            .data = vmRealloc(
            vm,
            (*self_0).data as *mut libc::c_void,
            ((*self_0).capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<Var>() as libc::c_ulong),
            (capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<Var>() as libc::c_ulong),
        ) as *mut Var;
        (*self_0).capacity = capacity as uint32_t;
    }
}
pub unsafe extern "C" fn pkVarBufferClear(
    mut self_0: *mut pkVarBuffer,
    mut vm: *mut PKVM,
) {
    vmRealloc(
        vm,
        (*self_0).data as *mut libc::c_void,
        ((*self_0).capacity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Var>() as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    (*self_0).data = 0 as *mut Var;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkVarBufferInit(mut self_0: *mut pkVarBuffer) {
    (*self_0).data = 0 as *mut Var;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkStringBufferWrite(
    mut self_0: *mut pkStringBuffer,
    mut vm: *mut PKVM,
    mut data: *mut String_0,
) {
    pkStringBufferFill(self_0, vm, data, 1 as libc::c_int);
}
pub unsafe extern "C" fn pkStringBufferInit(mut self_0: *mut pkStringBuffer) {
    (*self_0).data = 0 as *mut *mut String_0;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkStringBufferClear(
    mut self_0: *mut pkStringBuffer,
    mut vm: *mut PKVM,
) {
    vmRealloc(
        vm,
        (*self_0).data as *mut libc::c_void,
        ((*self_0).capacity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut String_0>() as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    (*self_0).data = 0 as *mut *mut String_0;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkStringBufferReserve(
    mut self_0: *mut pkStringBuffer,
    mut vm: *mut PKVM,
    mut size: size_t,
) {
    if ((*self_0).capacity as libc::c_ulong) < size {
        let mut capacity: libc::c_int = utilPowerOf2Ceil(size as libc::c_int);
        if capacity < 8 as libc::c_int {
            capacity = 8 as libc::c_int;
        }
        (*self_0)
            .data = vmRealloc(
            vm,
            (*self_0).data as *mut libc::c_void,
            ((*self_0).capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut String_0>() as libc::c_ulong),
            (capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut String_0>() as libc::c_ulong),
        ) as *mut *mut String_0;
        (*self_0).capacity = capacity as uint32_t;
    }
}
pub unsafe extern "C" fn pkStringBufferFill(
    mut self_0: *mut pkStringBuffer,
    mut vm: *mut PKVM,
    mut data: *mut String_0,
    mut count: libc::c_int,
) {
    pkStringBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add(count as libc::c_uint) as size_t,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let fresh3 = (*self_0).count;
        (*self_0).count = ((*self_0).count).wrapping_add(1);
        let ref mut fresh4 = *((*self_0).data).offset(fresh3 as isize);
        *fresh4 = data;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn pkStringBufferConcat(
    mut self_0: *mut pkStringBuffer,
    mut vm: *mut PKVM,
    mut other: *mut pkStringBuffer,
) {
    pkStringBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add((*other).count) as size_t,
    );
    memcpy(
        ((*self_0).data).offset((*self_0).count as isize) as *mut libc::c_void,
        (*other).data as *const libc::c_void,
        ((*other).count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut String_0>() as libc::c_ulong),
    );
    (*self_0)
        .count = ((*self_0).count as libc::c_uint).wrapping_add((*other).count)
        as uint32_t as uint32_t;
}
pub unsafe extern "C" fn pkClosureBufferClear(
    mut self_0: *mut pkClosureBuffer,
    mut vm: *mut PKVM,
) {
    vmRealloc(
        vm,
        (*self_0).data as *mut libc::c_void,
        ((*self_0).capacity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut Closure>() as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    (*self_0).data = 0 as *mut *mut Closure;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkClosureBufferWrite(
    mut self_0: *mut pkClosureBuffer,
    mut vm: *mut PKVM,
    mut data: *mut Closure,
) {
    pkClosureBufferFill(self_0, vm, data, 1 as libc::c_int);
}
pub unsafe extern "C" fn pkClosureBufferFill(
    mut self_0: *mut pkClosureBuffer,
    mut vm: *mut PKVM,
    mut data: *mut Closure,
    mut count: libc::c_int,
) {
    pkClosureBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add(count as libc::c_uint) as size_t,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        let fresh5 = (*self_0).count;
        (*self_0).count = ((*self_0).count).wrapping_add(1);
        let ref mut fresh6 = *((*self_0).data).offset(fresh5 as isize);
        *fresh6 = data;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn pkClosureBufferReserve(
    mut self_0: *mut pkClosureBuffer,
    mut vm: *mut PKVM,
    mut size: size_t,
) {
    if ((*self_0).capacity as libc::c_ulong) < size {
        let mut capacity: libc::c_int = utilPowerOf2Ceil(size as libc::c_int);
        if capacity < 8 as libc::c_int {
            capacity = 8 as libc::c_int;
        }
        (*self_0)
            .data = vmRealloc(
            vm,
            (*self_0).data as *mut libc::c_void,
            ((*self_0).capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut Closure>() as libc::c_ulong),
            (capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut Closure>() as libc::c_ulong),
        ) as *mut *mut Closure;
        (*self_0).capacity = capacity as uint32_t;
    }
}
pub unsafe extern "C" fn pkClosureBufferConcat(
    mut self_0: *mut pkClosureBuffer,
    mut vm: *mut PKVM,
    mut other: *mut pkClosureBuffer,
) {
    pkClosureBufferReserve(
        self_0,
        vm,
        ((*self_0).count).wrapping_add((*other).count) as size_t,
    );
    memcpy(
        ((*self_0).data).offset((*self_0).count as isize) as *mut libc::c_void,
        (*other).data as *const libc::c_void,
        ((*other).count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut Closure>() as libc::c_ulong),
    );
    (*self_0)
        .count = ((*self_0).count as libc::c_uint).wrapping_add((*other).count)
        as uint32_t as uint32_t;
}
pub unsafe extern "C" fn pkClosureBufferInit(mut self_0: *mut pkClosureBuffer) {
    (*self_0).data = 0 as *mut *mut Closure;
    (*self_0).count = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn pkByteBufferAddString(
    mut self_0: *mut pkByteBuffer,
    mut vm: *mut PKVM,
    mut str: *const libc::c_char,
    mut length: uint32_t,
) {
    pkByteBufferReserve(
        self_0,
        vm,
        ((*self_0).count as size_t).wrapping_add(length as libc::c_ulong),
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < length {
        let fresh7 = str;
        str = str.offset(1);
        let fresh8 = (*self_0).count;
        (*self_0).count = ((*self_0).count).wrapping_add(1);
        *((*self_0).data).offset(fresh8 as isize) = *fresh7 as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn pkByteBufferAddStringFmt(
    mut self_0: *mut pkByteBuffer,
    mut vm: *mut PKVM,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    let mut copy: ::std::ffi::VaListImpl;
    copy = args_0.clone();
    let mut length: libc::c_int = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        copy.as_va_list(),
    );
    pkByteBufferReserve(
        self_0,
        vm,
        ((*self_0).count as libc::c_ulong)
            .wrapping_add(length as size_t)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    vsnprintf(
        ((*self_0).data).offset((*self_0).count as isize) as *mut libc::c_char,
        ((*self_0).capacity).wrapping_sub((*self_0).count) as libc::c_ulong,
        fmt,
        args_0.as_va_list(),
    );
    (*self_0)
        .count = ((*self_0).count as libc::c_uint).wrapping_add(length as libc::c_uint)
        as uint32_t as uint32_t;
}
pub unsafe extern "C" fn varInitObject(
    mut self_0: *mut Object,
    mut vm: *mut PKVM,
    mut type_0: ObjectType,
) {
    (*self_0).type_0 = type_0;
    (*self_0).is_marked = 0 as libc::c_int != 0;
    (*self_0).next = (*vm).first;
    (*vm).first = self_0;
}
pub unsafe extern "C" fn markObject(mut vm: *mut PKVM, mut self_0: *mut Object) {
    if self_0.is_null() || (*self_0).is_marked as libc::c_int != 0 {
        return;
    }
    (*self_0).is_marked = 1 as libc::c_int != 0;
    if (*vm).working_set_count >= (*vm).working_set_capacity {
        (*vm).working_set_capacity *= 2 as libc::c_int;
        (*vm)
            .working_set = ((*vm).config.realloc_fn)
            .unwrap()(
            (*vm).working_set as *mut libc::c_void,
            ((*vm).working_set_capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut Object>() as libc::c_ulong),
            (*vm).config.user_data,
        ) as *mut *mut Object;
    }
    let fresh9 = (*vm).working_set_count;
    (*vm).working_set_count = (*vm).working_set_count + 1;
    let ref mut fresh10 = *((*vm).working_set).offset(fresh9 as isize);
    *fresh10 = self_0;
}
pub unsafe extern "C" fn markValue(mut vm: *mut PKVM, mut self_0: Var) {
    if !(self_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
    {
        return;
    }
    markObject(vm, (self_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object);
}
pub unsafe extern "C" fn markVarBuffer(mut vm: *mut PKVM, mut self_0: *mut pkVarBuffer) {
    if self_0.is_null() {
        return;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).count {
        markValue(vm, *((*self_0).data).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn markStringBuffer(
    mut vm: *mut PKVM,
    mut self_0: *mut pkStringBuffer,
) {
    if self_0.is_null() {
        return;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).count {
        markObject(vm, &mut (**((*self_0).data).offset(i as isize))._super);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn markClosureBuffer(
    mut vm: *mut PKVM,
    mut self_0: *mut pkClosureBuffer,
) {
    if self_0.is_null() {
        return;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).count {
        markObject(vm, &mut (**((*self_0).data).offset(i as isize))._super);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn popMarkedObjectsInternal(mut obj: *mut Object, mut vm: *mut PKVM) {
    match (*obj).type_0 as libc::c_uint {
        0 => {
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<String_0>() as libc::c_ulong)
                as size_t as size_t;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add((*(obj as *mut String_0)).capacity as size_t) as size_t
                as size_t;
        }
        1 => {
            let mut list: *mut List = obj as *mut List;
            markVarBuffer(vm, &mut (*list).elements);
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<List>() as libc::c_ulong) as size_t
                as size_t;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Var>() as libc::c_ulong)
                        .wrapping_mul((*list).elements.capacity as libc::c_ulong),
                ) as size_t as size_t;
        }
        2 => {
            let mut map: *mut Map = obj as *mut Map;
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*map).capacity {
                if !((*((*map).entries).offset(i as isize)).key
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x1000000000000 as libc::c_long as uint64_t)
                {
                    markValue(vm, (*((*map).entries).offset(i as isize)).key);
                    markValue(vm, (*((*map).entries).offset(i as isize)).value);
                }
                i = i.wrapping_add(1);
                i;
            }
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Map>() as libc::c_ulong) as size_t
                as size_t;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<MapEntry>() as libc::c_ulong)
                        .wrapping_mul((*map).capacity as libc::c_ulong),
                ) as size_t as size_t;
        }
        3 => {
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Range>() as libc::c_ulong) as size_t
                as size_t;
        }
        4 => {
            let mut module: *mut Module = obj as *mut Module;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Module>() as libc::c_ulong) as size_t
                as size_t;
            markObject(vm, &mut (*(*module).path)._super);
            markObject(vm, &mut (*(*module).name)._super);
            markVarBuffer(vm, &mut (*module).globals);
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Var>() as libc::c_ulong)
                        .wrapping_mul((*module).globals.capacity as libc::c_ulong),
                ) as size_t as size_t;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul((*module).global_names.capacity as libc::c_ulong),
                ) as size_t as size_t;
            markVarBuffer(vm, &mut (*module).constants);
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Var>() as libc::c_ulong)
                        .wrapping_mul((*module).constants.capacity as libc::c_ulong),
                ) as size_t as size_t;
            markObject(vm, &mut (*(*module).body)._super);
        }
        5 => {
            let mut func: *mut Function = obj as *mut Function;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Function>() as libc::c_ulong)
                as size_t as size_t;
            markObject(vm, &mut (*(*func).owner)._super);
            if !(*func).is_native && !((*func).c2rust_unnamed.fn_0).is_null() {
                let mut fn_0: *mut Fn_0 = (*func).c2rust_unnamed.fn_0;
                (*vm)
                    .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<Fn_0>() as libc::c_ulong)
                    as size_t as size_t;
                (*vm)
                    .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                            .wrapping_mul((*fn_0).opcodes.capacity as libc::c_ulong),
                    ) as size_t as size_t;
                (*vm)
                    .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                            .wrapping_mul((*fn_0).oplines.capacity as libc::c_ulong),
                    ) as size_t as size_t;
            }
        }
        6 => {
            let mut closure: *mut Closure = obj as *mut Closure;
            markObject(vm, &mut (*(*closure).fn_0)._super);
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < (*(*closure).fn_0).upvalue_count {
                markObject(
                    vm,
                    &mut (**((*closure).upvalues).as_mut_ptr().offset(i_0 as isize))
                        ._super,
                );
                i_0 += 1;
                i_0;
            }
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Closure>() as libc::c_ulong)
                as size_t as size_t;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<*mut Upvalue>() as libc::c_ulong)
                        .wrapping_mul((*(*closure).fn_0).upvalue_count as libc::c_ulong),
                ) as size_t as size_t;
        }
        7 => {
            let mut mb: *mut MethodBind = obj as *mut MethodBind;
            markObject(vm, &mut (*(*mb).method)._super);
            markValue(vm, (*mb).instance);
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<MethodBind>() as libc::c_ulong)
                as size_t as size_t;
        }
        8 => {
            let mut upvalue: *mut Upvalue = obj as *mut Upvalue;
            markValue(vm, (*upvalue).closed);
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Upvalue>() as libc::c_ulong)
                as size_t as size_t;
        }
        9 => {
            let mut fiber: *mut Fiber = obj as *mut Fiber;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Fiber>() as libc::c_ulong) as size_t
                as size_t;
            markObject(vm, &mut (*(*fiber).closure)._super);
            let mut local: *mut Var = (*fiber).stack;
            while local < (*fiber).sp {
                markValue(vm, *local);
                local = local.offset(1);
                local;
            }
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Var>() as libc::c_ulong)
                        .wrapping_mul((*fiber).stack_size as libc::c_ulong),
                ) as size_t as size_t;
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < (*fiber).frame_count {
                markObject(
                    vm,
                    &(*(*((*fiber).frames).offset(i_1 as isize)).closure)._super
                        as *const Object as *mut Object,
                );
                markValue(vm, (*((*fiber).frames).offset(i_1 as isize)).self_0);
                i_1 += 1;
                i_1;
            }
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<CallFrame>() as libc::c_ulong)
                        .wrapping_mul((*fiber).frame_capacity as libc::c_ulong),
                ) as size_t as size_t;
            markObject(vm, &mut (*(*fiber).caller)._super);
            markObject(vm, &mut (*(*fiber).native)._super);
            markObject(vm, &mut (*(*fiber).error)._super);
            markValue(vm, (*fiber).self_0);
        }
        10 => {
            let mut cls: *mut Class = obj as *mut Class;
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Class>() as libc::c_ulong) as size_t
                as size_t;
            markObject(vm, &mut (*(*cls).owner)._super);
            markObject(vm, &mut (*(*cls).ctor)._super);
            markObject(vm, &mut (*(*cls).name)._super);
            markObject(vm, &mut (*(*cls).static_attribs)._super);
            markClosureBuffer(vm, &mut (*cls).methods);
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Closure>() as libc::c_ulong)
                        .wrapping_mul((*cls).methods.capacity as libc::c_ulong),
                ) as size_t as size_t;
        }
        11 => {
            let mut inst: *mut Instance = obj as *mut Instance;
            markObject(vm, &mut (*(*inst).attribs)._super);
            markObject(vm, &mut (*(*inst).cls)._super);
            (*vm)
                .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<Instance>() as libc::c_ulong)
                as size_t as size_t;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn popMarkedObjects(mut vm: *mut PKVM) {
    while (*vm).working_set_count > 0 as libc::c_int {
        (*vm).working_set_count -= 1;
        let mut marked_obj: *mut Object = *((*vm).working_set)
            .offset((*vm).working_set_count as isize);
        popMarkedObjectsInternal(marked_obj, vm);
    }
}
pub unsafe extern "C" fn doubleToVar(mut value: libc::c_double) -> Var {
    return utilDoubleToBits(value);
}
pub unsafe extern "C" fn varToDouble(mut value: Var) -> libc::c_double {
    return utilDoubleFromBits(value);
}
unsafe extern "C" fn _allocateString(
    mut vm: *mut PKVM,
    mut length: size_t,
) -> *mut String_0 {
    let mut string: *mut String_0 = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (::std::mem::size_of::<String_0>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(length.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            ),
    ) as *mut String_0;
    varInitObject(&mut (*string)._super, vm, OBJ_STRING);
    (*string).length = length as uint32_t;
    *((*string).data).as_mut_ptr().offset(length as isize) = '\0' as i32 as libc::c_char;
    (*string)
        .capacity = length.wrapping_add(1 as libc::c_int as libc::c_ulong) as uint32_t;
    return string;
}
pub unsafe extern "C" fn newStringLength(
    mut vm: *mut PKVM,
    mut text: *const libc::c_char,
    mut length: uint32_t,
) -> *mut String_0 {
    let mut string: *mut String_0 = _allocateString(vm, length as size_t);
    if length != 0 as libc::c_int as libc::c_uint && !text.is_null() {
        memcpy(
            ((*string).data).as_mut_ptr() as *mut libc::c_void,
            text as *const libc::c_void,
            length as libc::c_ulong,
        );
    }
    (*string).hash = utilHashString(((*string).data).as_mut_ptr());
    return string;
}
pub unsafe extern "C" fn newStringVaArgs(
    mut vm: *mut PKVM,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> *mut String_0 {
    let mut copy: ::std::ffi::VaListImpl;
    copy = args.clone();
    let mut length: libc::c_int = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        copy.as_va_list(),
    );
    let mut string: *mut String_0 = _allocateString(vm, length as size_t);
    vsnprintf(
        ((*string).data).as_mut_ptr(),
        (*string).capacity as libc::c_ulong,
        fmt,
        args.as_va_list(),
    );
    (*string).hash = utilHashString(((*string).data).as_mut_ptr());
    return string;
}
pub unsafe extern "C" fn newList(mut vm: *mut PKVM, mut size: uint32_t) -> *mut List {
    let mut list: *mut List = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<List>() as libc::c_ulong,
    ) as *mut List;
    vmPushTempRef(vm, &mut (*list)._super);
    varInitObject(&mut (*list)._super, vm, OBJ_LIST);
    pkVarBufferInit(&mut (*list).elements);
    if size > 0 as libc::c_int as libc::c_uint {
        pkVarBufferFill(
            &mut (*list).elements,
            vm,
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t,
            size as libc::c_int,
        );
        (*list).elements.count = 0 as libc::c_int as uint32_t;
    }
    vmPopTempRef(vm);
    return list;
}
pub unsafe extern "C" fn newMap(mut vm: *mut PKVM) -> *mut Map {
    let mut map: *mut Map = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Map>() as libc::c_ulong,
    ) as *mut Map;
    varInitObject(&mut (*map)._super, vm, OBJ_MAP);
    (*map).capacity = 0 as libc::c_int as uint32_t;
    (*map).count = 0 as libc::c_int as uint32_t;
    (*map).entries = 0 as *mut MapEntry;
    return map;
}
pub unsafe extern "C" fn newRange(
    mut vm: *mut PKVM,
    mut from: libc::c_double,
    mut to: libc::c_double,
) -> *mut Range {
    let mut range: *mut Range = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Range>() as libc::c_ulong,
    ) as *mut Range;
    varInitObject(&mut (*range)._super, vm, OBJ_RANGE);
    (*range).from = from;
    (*range).to = to;
    return range;
}
pub unsafe extern "C" fn newModule(mut vm: *mut PKVM) -> *mut Module {
    let mut module: *mut Module = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Module>() as libc::c_ulong,
    ) as *mut Module;
    memset(
        module as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Module>() as libc::c_ulong,
    );
    varInitObject(&mut (*module)._super, vm, OBJ_MODULE);
    pkVarBufferInit(&mut (*module).globals);
    pkUintBufferInit(&mut (*module).global_names);
    pkVarBufferInit(&mut (*module).constants);
    return module;
}
pub unsafe extern "C" fn newFunction(
    mut vm: *mut PKVM,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
    mut owner: *mut Module,
    mut is_native: bool,
    mut docstring: *const libc::c_char,
    mut fn_index: *mut libc::c_int,
) -> *mut Function {
    let mut func: *mut Function = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Function>() as libc::c_ulong,
    ) as *mut Function;
    memset(
        func as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Function>() as libc::c_ulong,
    );
    varInitObject(&mut (*func)._super, vm, OBJ_FUNC);
    vmPushTempRef(vm, &mut (*func)._super);
    (*func).owner = owner;
    (*func).is_native = is_native;
    (*func).upvalue_count = 0 as libc::c_int;
    (*func).arity = -(2 as libc::c_int);
    (*func).is_method = 0 as libc::c_int != 0;
    (*func).docstring = docstring;
    if is_native as libc::c_int != 0 && owner.is_null() {
        (*func).name = name;
        (*func).c2rust_unnamed.native = None;
    } else {
        let mut _fn_index: uint32_t = moduleAddConstant(
            vm,
            owner,
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*func)._super as *mut Object as uintptr_t,
        );
        if !fn_index.is_null() {
            *fn_index = _fn_index as libc::c_int;
        }
        (*func)
            .name = ((*moduleAddString(
            owner,
            vm,
            name,
            length as uint32_t,
            0 as *mut libc::c_int,
        ))
            .data)
            .as_mut_ptr();
        if is_native {
            (*func).c2rust_unnamed.native = None;
        } else {
            let mut fn_0: *mut Fn_0 = vmRealloc(
                vm,
                0 as *mut libc::c_void,
                0 as libc::c_int as size_t,
                ::std::mem::size_of::<Fn_0>() as libc::c_ulong,
            ) as *mut Fn_0;
            pkByteBufferInit(&mut (*fn_0).opcodes);
            pkUintBufferInit(&mut (*fn_0).oplines);
            (*fn_0).stack_size = 0 as libc::c_int;
            (*func).c2rust_unnamed.fn_0 = fn_0;
        }
    }
    vmPopTempRef(vm);
    return func;
}
pub unsafe extern "C" fn newClosure(
    mut vm: *mut PKVM,
    mut fn_0: *mut Function,
) -> *mut Closure {
    let mut closure: *mut Closure = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (::std::mem::size_of::<Closure>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<*mut Upvalue>() as libc::c_ulong)
                    .wrapping_mul((*fn_0).upvalue_count as libc::c_ulong),
            ),
    ) as *mut Closure;
    varInitObject(&mut (*closure)._super, vm, OBJ_CLOSURE);
    (*closure).fn_0 = fn_0;
    memset(
        ((*closure).upvalues).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<*mut Upvalue>() as libc::c_ulong)
            .wrapping_mul((*fn_0).upvalue_count as libc::c_ulong),
    );
    return closure;
}
pub unsafe extern "C" fn newMethodBind(
    mut vm: *mut PKVM,
    mut method: *mut Closure,
) -> *mut MethodBind {
    let mut mb: *mut MethodBind = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<MethodBind>() as libc::c_ulong,
    ) as *mut MethodBind;
    varInitObject(&mut (*mb)._super, vm, OBJ_METHOD_BIND);
    (*mb).method = method;
    (*mb)
        .instance = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
    return mb;
}
pub unsafe extern "C" fn newUpvalue(
    mut vm: *mut PKVM,
    mut value: *mut Var,
) -> *mut Upvalue {
    let mut upvalue: *mut Upvalue = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Upvalue>() as libc::c_ulong,
    ) as *mut Upvalue;
    varInitObject(&mut (*upvalue)._super, vm, OBJ_UPVALUE);
    (*upvalue).ptr = value;
    (*upvalue)
        .closed = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0 as libc::c_int as uint64_t;
    (*upvalue).next = 0 as *mut Upvalue;
    return upvalue;
}
pub unsafe extern "C" fn newFiber(
    mut vm: *mut PKVM,
    mut closure: *mut Closure,
) -> *mut Fiber {
    let mut fiber: *mut Fiber = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Fiber>() as libc::c_ulong,
    ) as *mut Fiber;
    memset(
        fiber as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Fiber>() as libc::c_ulong,
    );
    varInitObject(&mut (*fiber)._super, vm, OBJ_FIBER);
    vmPushTempRef(vm, &mut (*fiber)._super);
    (*fiber).state = FIBER_NEW;
    (*fiber).closure = closure;
    if closure.is_null() || (*(*closure).fn_0).is_native as libc::c_int != 0 {
        let mut stack_size: libc::c_int = if closure.is_null() {
            1 as libc::c_int
        } else {
            (*(*closure).fn_0).arity + 1 as libc::c_int
        };
        stack_size = utilPowerOf2Ceil(stack_size);
        if stack_size == 0 as libc::c_int {
            stack_size += 1;
            stack_size;
        }
        (*fiber)
            .stack = vmRealloc(
            vm,
            0 as *mut libc::c_void,
            0 as libc::c_int as size_t,
            (::std::mem::size_of::<Var>() as libc::c_ulong)
                .wrapping_mul(stack_size as libc::c_ulong),
        ) as *mut Var;
        (*fiber).stack_size = stack_size;
        (*fiber).ret = (*fiber).stack;
        (*fiber).sp = ((*fiber).stack).offset(1 as libc::c_int as isize);
    } else {
        let mut stack_size_0: libc::c_int = utilPowerOf2Ceil(
            (*(*(*closure).fn_0).c2rust_unnamed.fn_0).stack_size + 1 as libc::c_int,
        );
        if stack_size_0 < 128 as libc::c_int {
            stack_size_0 = 128 as libc::c_int;
        }
        (*fiber)
            .stack = vmRealloc(
            vm,
            0 as *mut libc::c_void,
            0 as libc::c_int as size_t,
            (::std::mem::size_of::<Var>() as libc::c_ulong)
                .wrapping_mul(stack_size_0 as libc::c_ulong),
        ) as *mut Var;
        (*fiber).stack_size = stack_size_0;
        (*fiber).ret = (*fiber).stack;
        (*fiber).sp = ((*fiber).stack).offset(1 as libc::c_int as isize);
        (*fiber).frame_capacity = 4 as libc::c_int;
        (*fiber)
            .frames = vmRealloc(
            vm,
            0 as *mut libc::c_void,
            0 as libc::c_int as size_t,
            (::std::mem::size_of::<CallFrame>() as libc::c_ulong)
                .wrapping_mul((*fiber).frame_capacity as libc::c_ulong),
        ) as *mut CallFrame;
        (*fiber).frame_count = 1 as libc::c_int;
        let ref mut fresh11 = (*((*fiber).frames).offset(0 as libc::c_int as isize))
            .closure;
        *fresh11 = closure;
        let ref mut fresh12 = (*((*fiber).frames).offset(0 as libc::c_int as isize)).ip;
        *fresh12 = (*(*(*closure).fn_0).c2rust_unnamed.fn_0).opcodes.data;
        let ref mut fresh13 = (*((*fiber).frames).offset(0 as libc::c_int as isize)).rbp;
        *fresh13 = (*fiber).ret;
    }
    (*fiber).open_upvalues = 0 as *mut Upvalue;
    (*fiber)
        .self_0 = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
    *(*fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0 as libc::c_int as uint64_t;
    vmPopTempRef(vm);
    return fiber;
}
pub unsafe extern "C" fn newClass(
    mut vm: *mut PKVM,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
    mut super_0: *mut Class,
    mut module: *mut Module,
    mut docstring: *const libc::c_char,
    mut cls_index: *mut libc::c_int,
) -> *mut Class {
    let mut cls: *mut Class = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Class>() as libc::c_ulong,
    ) as *mut Class;
    memset(
        cls as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Class>() as libc::c_ulong,
    );
    varInitObject(&mut (*cls)._super, vm, OBJ_CLASS);
    vmPushTempRef(vm, &mut (*cls)._super);
    pkClosureBufferInit(&mut (*cls).methods);
    (*cls).static_attribs = newMap(vm);
    (*cls).class_of = PK_INSTANCE;
    (*cls).super_class = super_0;
    (*cls).docstring = docstring;
    if !module.is_null() {
        (*cls)
            .name = moduleAddString(
            module,
            vm,
            name,
            length as uint32_t,
            0 as *mut libc::c_int,
        );
        let mut _cls_index: libc::c_int = moduleAddConstant(
            vm,
            module,
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*cls)._super as *mut Object as uintptr_t,
        ) as libc::c_int;
        if !cls_index.is_null() {
            *cls_index = _cls_index;
        }
        moduleSetGlobal(
            vm,
            module,
            name,
            length as uint32_t,
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*cls)._super as *mut Object as uintptr_t,
        );
    } else {
        (*cls).name = newStringLength(vm, name, length as uint32_t);
    }
    vmPopTempRef(vm);
    return cls;
}
pub unsafe extern "C" fn newInstance(
    mut vm: *mut PKVM,
    mut cls: *mut Class,
) -> *mut Instance {
    let mut inst: *mut Instance = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<Instance>() as libc::c_ulong,
    ) as *mut Instance;
    memset(
        inst as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Instance>() as libc::c_ulong,
    );
    varInitObject(&mut (*inst)._super, vm, OBJ_INST);
    vmPushTempRef(vm, &mut (*inst)._super);
    (*inst).cls = cls;
    if ((*cls).new_fn).is_some() {
        (*inst).native = ((*cls).new_fn).unwrap()(vm);
    } else {
        (*inst).native = 0 as *mut libc::c_void;
    }
    (*inst).attribs = newMap(vm);
    vmPopTempRef(vm);
    return inst;
}
pub unsafe extern "C" fn rangeAsList(
    mut vm: *mut PKVM,
    mut self_0: *mut Range,
) -> *mut List {
    if (*self_0).from < (*self_0).to {
        let mut list: *mut List = newList(
            vm,
            ((*self_0).to - (*self_0).from) as uint32_t,
        );
        vmPushTempRef(vm, &mut (*list)._super);
        let mut i: libc::c_double = (*self_0).from;
        while i < (*self_0).to {
            pkVarBufferWrite(&mut (*list).elements, vm, doubleToVar(i));
            i += 1.;
            i;
        }
        vmPopTempRef(vm);
        return list;
    }
    return newList(vm, 0 as libc::c_int as uint32_t);
}
pub unsafe extern "C" fn stringLower(
    mut vm: *mut PKVM,
    mut self_0: *mut String_0,
) -> *mut String_0 {
    let mut index: uint32_t = 0 as libc::c_int as uint32_t;
    let mut c: *const libc::c_char = ((*self_0).data).as_mut_ptr();
    while *c as libc::c_int != '\0' as i32 {
        if *(*__ctype_b_loc()).offset(*c as libc::c_int as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut lower: *mut String_0 = newStringLength(
                vm,
                ((*self_0).data).as_mut_ptr(),
                (*self_0).length,
            );
            let mut _c: *mut libc::c_char = ((*lower).data)
                .as_mut_ptr()
                .offset(
                    c.offset_from(((*self_0).data).as_mut_ptr()) as libc::c_long as isize,
                );
            while *_c as libc::c_int != '\0' as i32 {
                *_c = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *_c as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*_c as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(*_c as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
                _c = _c.offset(1);
                _c;
            }
            (*lower).hash = utilHashString(((*lower).data).as_mut_ptr());
            return lower;
        }
        c = c.offset(1);
        c;
        index = index.wrapping_add(1);
        index;
    }
    return self_0;
}
pub unsafe extern "C" fn stringUpper(
    mut vm: *mut PKVM,
    mut self_0: *mut String_0,
) -> *mut String_0 {
    let mut index: uint32_t = 0 as libc::c_int as uint32_t;
    let mut c: *const libc::c_char = ((*self_0).data).as_mut_ptr();
    while *c as libc::c_int != '\0' as i32 {
        if *(*__ctype_b_loc()).offset(*c as libc::c_int as isize) as libc::c_int
            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut upper: *mut String_0 = newStringLength(
                vm,
                ((*self_0).data).as_mut_ptr(),
                (*self_0).length,
            );
            let mut _c: *mut libc::c_char = ((*upper).data)
                .as_mut_ptr()
                .offset(
                    c.offset_from(((*self_0).data).as_mut_ptr()) as libc::c_long as isize,
                );
            while *_c as libc::c_int != '\0' as i32 {
                *_c = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *_c as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(*_c as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*_c as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
                _c = _c.offset(1);
                _c;
            }
            (*upper).hash = utilHashString(((*upper).data).as_mut_ptr());
            return upper;
        }
        c = c.offset(1);
        c;
        index = index.wrapping_add(1);
        index;
    }
    return self_0;
}
pub unsafe extern "C" fn stringStrip(
    mut vm: *mut PKVM,
    mut self_0: *mut String_0,
) -> *mut String_0 {
    let mut start: *const libc::c_char = ((*self_0).data).as_mut_ptr();
    while *start as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*start as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        start = start.offset(1);
        start;
    }
    if *start as libc::c_int == '\0' as i32 {
        return newStringLength(
            vm,
            0 as *const libc::c_char,
            0 as libc::c_int as uint32_t,
        );
    }
    let mut end: *const libc::c_char = ((*self_0).data)
        .as_mut_ptr()
        .offset((*self_0).length as isize)
        .offset(-(1 as libc::c_int as isize));
    while *(*__ctype_b_loc()).offset(*end as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        end = end.offset(-1);
        end;
    }
    if start == ((*self_0).data).as_mut_ptr() as *const libc::c_char
        && end
            == ((*self_0).data)
                .as_mut_ptr()
                .offset((*self_0).length as isize)
                .offset(-(1 as libc::c_int as isize)) as *const libc::c_char
    {
        return self_0;
    }
    return newStringLength(
        vm,
        start,
        (end.offset_from(start) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as uint32_t,
    );
}
pub unsafe extern "C" fn stringReplace(
    mut vm: *mut PKVM,
    mut self_0: *mut String_0,
    mut old: *mut String_0,
    mut new_: *mut String_0,
    mut count: int32_t,
) -> *mut String_0 {
    if (*self_0).length == 0 as libc::c_int as libc::c_uint
        || (*old).length == 0 as libc::c_int as libc::c_uint || count == 0 as libc::c_int
    {
        return self_0;
    }
    if (*old).hash == (*new_).hash && (*old).length == (*new_).length
        && memcmp(
            ((*old).data).as_mut_ptr() as *const libc::c_void,
            ((*new_).data).as_mut_ptr() as *const libc::c_void,
            (*old).length as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        return self_0;
    }
    let mut max_count: int32_t = ((*self_0).length).wrapping_div((*old).length)
        as int32_t;
    count = if count == -(1 as libc::c_int) {
        max_count
    } else if count < max_count {
        count
    } else {
        max_count
    };
    let mut length: uint32_t = if (*self_0).length
        > ((*self_0).length)
            .wrapping_add(
                ((*new_).length)
                    .wrapping_sub((*old).length)
                    .wrapping_mul(count as libc::c_uint),
            )
    {
        (*self_0).length
    } else {
        ((*self_0).length)
            .wrapping_add(
                ((*new_).length)
                    .wrapping_sub((*old).length)
                    .wrapping_mul(count as libc::c_uint),
            )
    };
    let mut replaced: *mut String_0 = self_0;
    let mut replacedc: int32_t = 0 as libc::c_int;
    let mut s: *const libc::c_char = ((*self_0).data).as_mut_ptr();
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    while !(replacedc == count) {
        let mut match_0: *const libc::c_char = strstr(s, ((*old).data).as_mut_ptr());
        if match_0.is_null() {
            break;
        }
        if replacedc == 0 as libc::c_int {
            replaced = newStringLength(
                vm,
                b"\0" as *const u8 as *const libc::c_char,
                length,
            );
            d = ((*replaced).data).as_mut_ptr();
        }
        memcpy(
            d as *mut libc::c_void,
            s as *const libc::c_void,
            match_0.offset_from(s) as libc::c_long as libc::c_ulong,
        );
        d = d.offset(match_0.offset_from(s) as libc::c_long as isize);
        s = match_0;
        memcpy(
            d as *mut libc::c_void,
            ((*new_).data).as_mut_ptr() as *const libc::c_void,
            (*new_).length as libc::c_ulong,
        );
        d = d.offset((*new_).length as isize);
        s = s.offset((*old).length as isize);
        replacedc += 1;
        replacedc;
    }
    if !d.is_null() {
        let mut tail_length: uint32_t = ((*self_0).length)
            .wrapping_sub(
                s.offset_from(((*self_0).data).as_mut_ptr()) as libc::c_long as int32_t
                    as libc::c_uint,
            );
        memcpy(
            d as *mut libc::c_void,
            s as *const libc::c_void,
            tail_length as libc::c_ulong,
        );
        d = d.offset(tail_length as isize);
        (*replaced)
            .length = d.offset_from(((*replaced).data).as_mut_ptr()) as libc::c_long
            as int32_t as uint32_t;
        *((*replaced).data)
            .as_mut_ptr()
            .offset((*replaced).length as isize) = '\0' as i32 as libc::c_char;
        (*replaced).hash = utilHashString(((*replaced).data).as_mut_ptr());
    }
    return replaced;
}
pub unsafe extern "C" fn stringSplit(
    mut vm: *mut PKVM,
    mut self_0: *mut String_0,
    mut sep: *mut String_0,
) -> *mut List {
    let mut s: *const libc::c_char = ((*self_0).data).as_mut_ptr();
    let mut list: *mut List = newList(vm, 0 as libc::c_int as uint32_t);
    vmPushTempRef(vm, &mut (*list)._super);
    loop {
        let mut match_0: *const libc::c_char = strstr(s, ((*sep).data).as_mut_ptr());
        if match_0.is_null() {
            if s == ((*self_0).data).as_mut_ptr() as *const libc::c_char {
                pkVarBufferWrite(
                    &mut (*list).elements,
                    vm,
                    0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*self_0)._super as *mut Object as uintptr_t,
                );
            } else {
                let mut tail: *mut String_0 = newStringLength(
                    vm,
                    s,
                    ((*self_0).length as libc::c_long
                        - s.offset_from(((*self_0).data).as_mut_ptr()) as libc::c_long)
                        as uint32_t,
                );
                vmPushTempRef(vm, &mut (*tail)._super);
                pkVarBufferWrite(
                    &mut (*list).elements,
                    vm,
                    0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*tail)._super as *mut Object as uintptr_t,
                );
                vmPopTempRef(vm);
            }
            break;
        } else {
            let mut split: *mut String_0 = newStringLength(
                vm,
                s,
                match_0.offset_from(s) as libc::c_long as uint32_t,
            );
            vmPushTempRef(vm, &mut (*split)._super);
            pkVarBufferWrite(
                &mut (*list).elements,
                vm,
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*split)._super as *mut Object as uintptr_t,
            );
            vmPopTempRef(vm);
            s = match_0.offset((*sep).length as isize);
        }
    }
    vmPopTempRef(vm);
    return list;
}
pub unsafe extern "C" fn stringFormat(
    mut vm: *mut PKVM,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut String_0 {
    let mut arg_list: ::std::ffi::VaListImpl;
    arg_list = args.clone();
    let mut total_length: size_t = 0 as libc::c_int as size_t;
    let mut c: *const libc::c_char = fmt;
    while *c as libc::c_int != '\0' as i32 {
        match *c as libc::c_int {
            36 => {
                total_length = (total_length as libc::c_ulong)
                    .wrapping_add(strlen(arg_list.arg::<*const libc::c_char>()))
                    as size_t as size_t;
            }
            64 => {
                total_length = (total_length as libc::c_ulong)
                    .wrapping_add(
                        (*arg_list.arg::<*mut String_0>()).length as libc::c_ulong,
                    ) as size_t as size_t;
            }
            _ => {
                total_length = total_length.wrapping_add(1);
                total_length;
            }
        }
        c = c.offset(1);
        c;
    }
    let mut result: *mut String_0 = _allocateString(vm, total_length);
    arg_list = args.clone();
    let mut buff: *mut libc::c_char = ((*result).data).as_mut_ptr();
    let mut c_0: *const libc::c_char = fmt;
    while *c_0 as libc::c_int != '\0' as i32 {
        match *c_0 as libc::c_int {
            36 => {
                let mut string: *const libc::c_char = arg_list
                    .arg::<*const libc::c_char>();
                let mut length: size_t = strlen(string);
                memcpy(buff as *mut libc::c_void, string as *const libc::c_void, length);
                buff = buff.offset(length as isize);
            }
            64 => {
                let mut string_0: *mut String_0 = arg_list.arg::<*mut String_0>();
                memcpy(
                    buff as *mut libc::c_void,
                    ((*string_0).data).as_mut_ptr() as *const libc::c_void,
                    (*string_0).length as libc::c_ulong,
                );
                buff = buff.offset((*string_0).length as isize);
            }
            _ => {
                let fresh14 = buff;
                buff = buff.offset(1);
                *fresh14 = *c_0;
            }
        }
        c_0 = c_0.offset(1);
        c_0;
    }
    (*result).hash = utilHashString(((*result).data).as_mut_ptr());
    return result;
}
pub unsafe extern "C" fn stringJoin(
    mut vm: *mut PKVM,
    mut str1: *mut String_0,
    mut str2: *mut String_0,
) -> *mut String_0 {
    if (*str1).length == 0 as libc::c_int as libc::c_uint {
        return str2;
    }
    if (*str2).length == 0 as libc::c_int as libc::c_uint {
        return str1;
    }
    let mut length: size_t = ((*str1).length as size_t)
        .wrapping_add((*str2).length as size_t);
    let mut string: *mut String_0 = _allocateString(vm, length);
    memcpy(
        ((*string).data).as_mut_ptr() as *mut libc::c_void,
        ((*str1).data).as_mut_ptr() as *const libc::c_void,
        (*str1).length as libc::c_ulong,
    );
    memcpy(
        ((*string).data).as_mut_ptr().offset((*str1).length as isize)
            as *mut libc::c_void,
        ((*str2).data).as_mut_ptr() as *const libc::c_void,
        (*str2).length as libc::c_ulong,
    );
    (*string).hash = utilHashString(((*string).data).as_mut_ptr());
    return string;
}
pub unsafe extern "C" fn listInsert(
    mut vm: *mut PKVM,
    mut self_0: *mut List,
    mut index: uint32_t,
    mut value: Var,
) {
    if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPushTempRef(
            vm,
            (value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object,
        );
    }
    pkVarBufferWrite(
        &mut (*self_0).elements,
        vm,
        0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t,
    );
    if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPopTempRef(vm);
    }
    let mut i: uint32_t = ((*self_0).elements.count)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i > index {
        *((*self_0).elements.data)
            .offset(
                i as isize,
            ) = *((*self_0).elements.data)
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        i = i.wrapping_sub(1);
        i;
    }
    *((*self_0).elements.data).offset(index as isize) = value;
}
pub unsafe extern "C" fn listRemoveAt(
    mut vm: *mut PKVM,
    mut self_0: *mut List,
    mut index: uint32_t,
) -> Var {
    let mut removed: Var = *((*self_0).elements.data).offset(index as isize);
    if removed
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPushTempRef(
            vm,
            (removed & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object,
        );
    }
    let mut i: uint32_t = index;
    while i < ((*self_0).elements.count).wrapping_sub(1 as libc::c_int as libc::c_uint) {
        *((*self_0).elements.data)
            .offset(
                i as isize,
            ) = *((*self_0).elements.data)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        i = i.wrapping_add(1);
        i;
    }
    if ((*self_0).elements.capacity).wrapping_div(2 as libc::c_int as libc::c_uint)
        >= (*self_0).elements.count
    {
        (*self_0)
            .elements
            .data = vmRealloc(
            vm,
            (*self_0).elements.data as *mut libc::c_void,
            (::std::mem::size_of::<Var>() as libc::c_ulong)
                .wrapping_mul((*self_0).elements.capacity as libc::c_ulong),
            (::std::mem::size_of::<Var>() as libc::c_ulong)
                .wrapping_mul((*self_0).elements.capacity as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong),
        ) as *mut Var;
        (*self_0)
            .elements
            .capacity = ((*self_0).elements.capacity as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    }
    if removed
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPopTempRef(vm);
    }
    (*self_0).elements.count = ((*self_0).elements.count).wrapping_sub(1);
    (*self_0).elements.count;
    return removed;
}
pub unsafe extern "C" fn listClear(mut vm: *mut PKVM, mut self_0: *mut List) {
    pkVarBufferClear(&mut (*self_0).elements, vm);
}
pub unsafe extern "C" fn listAdd(
    mut vm: *mut PKVM,
    mut l1: *mut List,
    mut l2: *mut List,
) -> *mut List {
    if (*l1).elements.count == 0 as libc::c_int as libc::c_uint {
        return l2;
    }
    if (*l2).elements.count == 0 as libc::c_int as libc::c_uint {
        return l1;
    }
    let mut size: uint32_t = ((*l1).elements.count).wrapping_add((*l2).elements.count);
    let mut list: *mut List = newList(vm, size);
    vmPushTempRef(vm, &mut (*list)._super);
    pkVarBufferConcat(&mut (*list).elements, vm, &mut (*l1).elements);
    pkVarBufferConcat(&mut (*list).elements, vm, &mut (*l2).elements);
    vmPopTempRef(vm);
    return list;
}
unsafe extern "C" fn _hashObject(mut obj: *mut Object) -> uint32_t {
    match (*obj).type_0 as libc::c_uint {
        0 => return (*(obj as *mut String_0)).hash,
        3 => {
            let mut range: *mut Range = obj as *mut Range;
            return utilHashNumber((*range).from) ^ utilHashNumber((*range).to);
        }
        10 => return utilHashBits(obj as int64_t as uint64_t),
        _ => {}
    }
    unreachable!();
}
pub unsafe extern "C" fn varHashValue(mut v: Var) -> uint32_t {
    if v
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        return _hashObject(
            (v & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object,
        );
    }
    return utilHashBits(v);
}
unsafe extern "C" fn _mapFindEntry(
    mut self_0: *mut Map,
    mut key: Var,
    mut result: *mut *mut MapEntry,
) -> bool {
    if (*self_0).capacity == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    let mut start_index: uint32_t = (varHashValue(key)).wrapping_rem((*self_0).capacity);
    let mut index: uint32_t = start_index;
    let mut tombstone: *mut MapEntry = 0 as *mut MapEntry;
    loop {
        let mut entry: *mut MapEntry = &mut *((*self_0).entries).offset(index as isize)
            as *mut MapEntry;
        if (*entry).key
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000000 as libc::c_long as uint64_t
        {
            if (*entry).value
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000003 as libc::c_long as uint64_t
            {
                if tombstone.is_null() {
                    tombstone = entry;
                }
            } else {
                *result = if !tombstone.is_null() { tombstone } else { entry };
                return 0 as libc::c_int != 0;
            }
        } else if isValuesEqual((*entry).key, key) {
            *result = entry;
            return 1 as libc::c_int != 0;
        }
        index = index
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem((*self_0).capacity);
        if !(index != start_index) {
            break;
        }
    }
    *result = tombstone;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn _mapInsertEntry(
    mut self_0: *mut Map,
    mut key: Var,
    mut value: Var,
) -> bool {
    let mut result: *mut MapEntry = 0 as *mut MapEntry;
    if _mapFindEntry(self_0, key, &mut result) {
        (*result).value = value;
        return 0 as libc::c_int != 0;
    } else {
        (*result).key = key;
        (*result).value = value;
        return 1 as libc::c_int != 0;
    };
}
unsafe extern "C" fn _mapResize(
    mut vm: *mut PKVM,
    mut self_0: *mut Map,
    mut capacity: uint32_t,
) {
    let mut old_entries: *mut MapEntry = (*self_0).entries;
    let mut old_capacity: uint32_t = (*self_0).capacity;
    (*self_0)
        .entries = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (::std::mem::size_of::<MapEntry>() as libc::c_ulong)
            .wrapping_mul(capacity as libc::c_ulong),
    ) as *mut MapEntry;
    (*self_0).capacity = capacity;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < capacity {
        (*((*self_0).entries).offset(i as isize))
            .key = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t;
        (*((*self_0).entries).offset(i as isize))
            .value = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < old_capacity {
        if !((*old_entries.offset(i_0 as isize)).key
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000000 as libc::c_long as uint64_t)
        {
            _mapInsertEntry(
                self_0,
                (*old_entries.offset(i_0 as isize)).key,
                (*old_entries.offset(i_0 as isize)).value,
            );
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    vmRealloc(
        vm,
        old_entries as *mut libc::c_void,
        (::std::mem::size_of::<MapEntry>() as libc::c_ulong)
            .wrapping_mul(old_capacity as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn mapGet(mut self_0: *mut Map, mut key: Var) -> Var {
    let mut entry: *mut MapEntry = 0 as *mut MapEntry;
    if _mapFindEntry(self_0, key, &mut entry) {
        return (*entry).value;
    }
    return 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
}
pub unsafe extern "C" fn mapSet(
    mut vm: *mut PKVM,
    mut self_0: *mut Map,
    mut key: Var,
    mut value: Var,
) {
    if ((*self_0).count).wrapping_add(1 as libc::c_int as libc::c_uint)
        > ((*self_0).capacity)
            .wrapping_mul(75 as libc::c_int as libc::c_uint)
            .wrapping_div(100 as libc::c_int as libc::c_uint)
    {
        let mut capacity: uint32_t = ((*self_0).capacity)
            .wrapping_mul(2 as libc::c_int as libc::c_uint);
        if capacity < 8 as libc::c_int as libc::c_uint {
            capacity = 8 as libc::c_int as uint32_t;
        }
        _mapResize(vm, self_0, capacity);
    }
    if _mapInsertEntry(self_0, key, value) {
        (*self_0).count = ((*self_0).count).wrapping_add(1);
        (*self_0).count;
    }
}
pub unsafe extern "C" fn mapClear(mut vm: *mut PKVM, mut self_0: *mut Map) {
    vmRealloc(
        vm,
        (*self_0).entries as *mut libc::c_void,
        (::std::mem::size_of::<MapEntry>() as libc::c_ulong)
            .wrapping_mul((*self_0).capacity as libc::c_ulong),
        0 as libc::c_int as size_t,
    );
    (*self_0).entries = 0 as *mut MapEntry;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
    (*self_0).count = 0 as libc::c_int as uint32_t;
}
pub unsafe extern "C" fn mapRemoveKey(
    mut vm: *mut PKVM,
    mut self_0: *mut Map,
    mut key: Var,
) -> Var {
    let mut entry: *mut MapEntry = 0 as *mut MapEntry;
    if !_mapFindEntry(self_0, key, &mut entry) {
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t;
    }
    let mut value: Var = (*entry).value;
    (*entry)
        .key = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
    (*entry)
        .value = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000003 as libc::c_long as uint64_t;
    (*self_0).count = ((*self_0).count).wrapping_sub(1);
    (*self_0).count;
    if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPushTempRef(
            vm,
            (value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object,
        );
    }
    if (*self_0).count == 0 as libc::c_int as libc::c_uint {
        mapClear(vm, self_0);
    } else if (*self_0).capacity > 8 as libc::c_int as libc::c_uint
        && ((*self_0).capacity)
            .wrapping_div((2 as libc::c_int * 2 as libc::c_int) as libc::c_uint)
            > ((*self_0).count)
                .wrapping_mul(100 as libc::c_int as libc::c_uint)
                .wrapping_div(75 as libc::c_int as libc::c_uint)
    {
        let mut capacity: uint32_t = ((*self_0).capacity)
            .wrapping_div((2 as libc::c_int * 2 as libc::c_int) as libc::c_uint);
        if capacity < 8 as libc::c_int as libc::c_uint {
            capacity = 8 as libc::c_int as uint32_t;
        }
        _mapResize(vm, self_0, capacity);
    }
    if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPopTempRef(vm);
    }
    return value;
}
pub unsafe extern "C" fn fiberHasError(mut fiber: *mut Fiber) -> bool {
    return !((*fiber).error).is_null();
}
pub unsafe extern "C" fn freeObject(mut vm: *mut PKVM, mut self_0: *mut Object) {
    match (*self_0).type_0 as libc::c_uint {
        0 => {
            let mut str: *mut String_0 = self_0 as *mut String_0;
            vmRealloc(
                vm,
                str as *mut libc::c_void,
                (::std::mem::size_of::<String_0>() as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul((*str).capacity as libc::c_ulong),
                    ),
                0 as libc::c_int as size_t,
            );
            return;
        }
        1 => {
            pkVarBufferClear(&mut (*(self_0 as *mut List)).elements, vm);
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                ::std::mem::size_of::<List>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        2 => {
            let mut map: *mut Map = self_0 as *mut Map;
            vmRealloc(
                vm,
                (*map).entries as *mut libc::c_void,
                (::std::mem::size_of::<MapEntry>() as libc::c_ulong)
                    .wrapping_mul((*map).capacity as libc::c_ulong),
                0 as libc::c_int as size_t,
            );
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                ::std::mem::size_of::<Map>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        3 => {
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                ::std::mem::size_of::<Range>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        4 => {
            let mut module: *mut Module = self_0 as *mut Module;
            pkVarBufferClear(&mut (*module).globals, vm);
            pkUintBufferClear(&mut (*module).global_names, vm);
            pkVarBufferClear(&mut (*module).constants, vm);
            if !((*module).handle).is_null() {
                vmUnloadDlHandle(vm, (*module).handle);
            }
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                ::std::mem::size_of::<Module>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        5 => {
            let mut func: *mut Function = self_0 as *mut Function;
            if !(*func).is_native {
                pkByteBufferClear(&mut (*(*func).c2rust_unnamed.fn_0).opcodes, vm);
                pkUintBufferClear(&mut (*(*func).c2rust_unnamed.fn_0).oplines, vm);
                vmRealloc(
                    vm,
                    (*func).c2rust_unnamed.fn_0 as *mut libc::c_void,
                    ::std::mem::size_of::<Fn_0>() as libc::c_ulong,
                    0 as libc::c_int as size_t,
                );
            }
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                ::std::mem::size_of::<Function>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        6 => {
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                (::std::mem::size_of::<Closure>() as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<*mut Upvalue>() as libc::c_ulong)
                            .wrapping_mul(
                                (*(*(self_0 as *mut Closure)).fn_0).upvalue_count
                                    as libc::c_ulong,
                            ),
                    ),
                0 as libc::c_int as size_t,
            );
            return;
        }
        7 => {
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                ::std::mem::size_of::<MethodBind>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        8 => {
            vmRealloc(
                vm,
                self_0 as *mut libc::c_void,
                ::std::mem::size_of::<Upvalue>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        9 => {
            let mut fiber: *mut Fiber = self_0 as *mut Fiber;
            vmRealloc(
                vm,
                (*fiber).stack as *mut libc::c_void,
                (::std::mem::size_of::<Var>() as libc::c_ulong)
                    .wrapping_mul((*fiber).stack_size as libc::c_ulong),
                0 as libc::c_int as size_t,
            );
            vmRealloc(
                vm,
                (*fiber).frames as *mut libc::c_void,
                (::std::mem::size_of::<CallFrame>() as libc::c_ulong)
                    .wrapping_mul((*fiber).frame_capacity as libc::c_ulong),
                0 as libc::c_int as size_t,
            );
            vmRealloc(
                vm,
                fiber as *mut libc::c_void,
                ::std::mem::size_of::<Fiber>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        10 => {
            let mut cls: *mut Class = self_0 as *mut Class;
            pkClosureBufferClear(&mut (*cls).methods, vm);
            vmRealloc(
                vm,
                cls as *mut libc::c_void,
                ::std::mem::size_of::<Class>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        11 => {
            let mut inst: *mut Instance = self_0 as *mut Instance;
            if ((*(*inst).cls).delete_fn).is_some() {
                ((*(*inst).cls).delete_fn).unwrap()(vm, (*inst).native);
            }
            vmRealloc(
                vm,
                inst as *mut libc::c_void,
                ::std::mem::size_of::<Instance>() as libc::c_ulong,
                0 as libc::c_int as size_t,
            );
            return;
        }
        _ => {}
    }
    unreachable!();
}
pub unsafe extern "C" fn moduleAddConstant(
    mut vm: *mut PKVM,
    mut module: *mut Module,
    mut value: Var,
) -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*module).constants.count {
        if isValuesSame(*((*module).constants.data).offset(i as isize), value) {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    pkVarBufferWrite(&mut (*module).constants, vm, value);
    return ((*module).constants.count as libc::c_int - 1 as libc::c_int) as uint32_t;
}
pub unsafe extern "C" fn moduleAddString(
    mut module: *mut Module,
    mut vm: *mut PKVM,
    mut name: *const libc::c_char,
    mut length: uint32_t,
    mut index: *mut libc::c_int,
) -> *mut String_0 {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*module).constants.count {
        if *((*module).constants.data).offset(i as isize)
            & (0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong)
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
            && (*((*((*module).constants.data).offset(i as isize)
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
        {
            let mut _name: *mut String_0 = (*((*module).constants.data)
                .offset(i as isize) & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut String_0;
            if (*_name).length == length
                && strncmp(((*_name).data).as_mut_ptr(), name, length as libc::c_ulong)
                    == 0 as libc::c_int
            {
                if !index.is_null() {
                    *index = i as libc::c_int;
                }
                return _name;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut new_name: *mut String_0 = newStringLength(vm, name, length);
    vmPushTempRef(vm, &mut (*new_name)._super);
    pkVarBufferWrite(
        &mut (*module).constants,
        vm,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*new_name)._super as *mut Object as uintptr_t,
    );
    vmPopTempRef(vm);
    if !index.is_null() {
        *index = ((*module).constants.count)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    }
    return new_name;
}
pub unsafe extern "C" fn moduleGetStringAt(
    mut module: *mut Module,
    mut index: libc::c_int,
) -> *mut String_0 {
    if index >= (*module).constants.count as libc::c_int {
        return 0 as *mut String_0;
    }
    let mut constant: Var = *((*module).constants.data).offset(index as isize);
    if constant
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((constant & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        return (constant & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0;
    }
    return 0 as *mut String_0;
}
pub unsafe extern "C" fn moduleSetGlobal(
    mut vm: *mut PKVM,
    mut module: *mut Module,
    mut name: *const libc::c_char,
    mut length: uint32_t,
    mut value: Var,
) -> uint32_t {
    let mut g_index: libc::c_int = moduleGetGlobalIndex(module, name, length);
    if g_index != -(1 as libc::c_int) {
        *((*module).globals.data).offset(g_index as isize) = value;
        return g_index as uint32_t;
    }
    let mut name_index: libc::c_int = 0 as libc::c_int;
    moduleAddString(module, vm, name, length, &mut name_index);
    pkUintBufferWrite(&mut (*module).global_names, vm, name_index as uint32_t);
    pkVarBufferWrite(&mut (*module).globals, vm, value);
    return ((*module).globals.count).wrapping_sub(1 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn moduleGetGlobalIndex(
    mut module: *mut Module,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*module).global_names.count {
        let mut name_index: uint32_t = *((*module).global_names.data).offset(i as isize);
        let mut g_name: *mut String_0 = moduleGetStringAt(
            module,
            name_index as libc::c_int,
        );
        if (*g_name).length == length
            && strncmp(((*g_name).data).as_mut_ptr(), name, length as libc::c_ulong)
                == 0 as libc::c_int
        {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn moduleAddMain(mut vm: *mut PKVM, mut module: *mut Module) {
    (*module).initialized = 0 as libc::c_int != 0;
    let mut fn_name: *const libc::c_char = b"@main\0" as *const u8
        as *const libc::c_char;
    let mut body_fn: *mut Function = newFunction(
        vm,
        fn_name,
        strlen(fn_name) as libc::c_int,
        module,
        0 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*body_fn).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*body_fn)._super);
    (*module).body = newClosure(vm, body_fn);
    vmPopTempRef(vm);
    moduleSetGlobal(
        vm,
        module,
        b"@main\0" as *const u8 as *const libc::c_char,
        strlen(b"@main\0" as *const u8 as *const libc::c_char) as uint32_t,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*(*module).body)._super as *mut Object as uintptr_t,
    );
}
pub unsafe extern "C" fn getObjPkVarType(mut type_0: ObjectType) -> PkVarType {
    match type_0 as libc::c_uint {
        0 => return PK_STRING,
        1 => return PK_LIST,
        2 => return PK_MAP,
        3 => return PK_RANGE,
        4 => return PK_MODULE,
        5 => {
            unreachable!();
        }
        6 => return PK_CLOSURE,
        7 => return PK_METHOD_BIND,
        8 => {
            unreachable!();
        }
        9 => return PK_FIBER,
        10 => return PK_CLASS,
        11 => return PK_INSTANCE,
        _ => {}
    }
    unreachable!();
}
pub unsafe extern "C" fn getPkVarObjType(mut type_0: PkVarType) -> ObjectType {
    match type_0 as libc::c_uint {
        0 | 1 | 2 | 3 => {
            unreachable!();
        }
        4 => return OBJ_STRING,
        5 => return OBJ_LIST,
        6 => return OBJ_MAP,
        7 => return OBJ_RANGE,
        8 => return OBJ_MODULE,
        9 => return OBJ_CLOSURE,
        10 => return OBJ_METHOD_BIND,
        11 => return OBJ_FIBER,
        12 => return OBJ_CLASS,
        13 => return OBJ_INST,
        _ => {}
    }
    unreachable!();
}
pub unsafe extern "C" fn getPkVarTypeName(mut type_0: PkVarType) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        0 => return b"Object\0" as *const u8 as *const libc::c_char,
        1 => return b"Null\0" as *const u8 as *const libc::c_char,
        2 => return b"Bool\0" as *const u8 as *const libc::c_char,
        3 => return b"Number\0" as *const u8 as *const libc::c_char,
        _ => return getObjectTypeName(getPkVarObjType(type_0)),
    };
}
pub unsafe extern "C" fn getObjectTypeName(
    mut type_0: ObjectType,
) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        0 => return b"String\0" as *const u8 as *const libc::c_char,
        1 => return b"List\0" as *const u8 as *const libc::c_char,
        2 => return b"Map\0" as *const u8 as *const libc::c_char,
        3 => return b"Range\0" as *const u8 as *const libc::c_char,
        4 => return b"Module\0" as *const u8 as *const libc::c_char,
        5 => return b"Func\0" as *const u8 as *const libc::c_char,
        6 => return b"Closure\0" as *const u8 as *const libc::c_char,
        7 => return b"MethodBind\0" as *const u8 as *const libc::c_char,
        8 => return b"Upvalue\0" as *const u8 as *const libc::c_char,
        9 => return b"Fiber\0" as *const u8 as *const libc::c_char,
        10 => return b"Class\0" as *const u8 as *const libc::c_char,
        11 => return b"Inst\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    unreachable!();
}
pub unsafe extern "C" fn varTypeName(mut v: Var) -> *const libc::c_char {
    if v == 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t
    {
        return b"Null\0" as *const u8 as *const libc::c_char;
    }
    if v
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
        || v
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t
    {
        return b"Bool\0" as *const u8 as *const libc::c_char;
    }
    if v & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        return b"Number\0" as *const u8 as *const libc::c_char;
    }
    let mut obj: *mut Object = (v & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    if (*obj).type_0 as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint {
        return ((*(*(*(obj as *mut Instance)).cls).name).data).as_mut_ptr();
    }
    return getObjectTypeName((*obj).type_0);
}
pub unsafe extern "C" fn getVarType(mut v: Var) -> PkVarType {
    if v == 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t
    {
        return PK_NULL;
    }
    if v
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
        || v
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t
    {
        return PK_BOOL;
    }
    if v & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        return PK_NUMBER;
    }
    let mut obj: *mut Object = (v & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    return getObjPkVarType((*obj).type_0);
}
pub unsafe extern "C" fn isValuesSame(mut v1: Var, mut v2: Var) -> bool {
    return v1 == v2;
}
pub unsafe extern "C" fn isValuesEqual(mut v1: Var, mut v2: Var) -> bool {
    if isValuesSame(v1, v2) {
        return 1 as libc::c_int != 0;
    }
    if v1 & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
        && v2 & 0x7ffc000000000000 as libc::c_long as uint64_t
            != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        return varToDouble(v1) == varToDouble(v2);
    }
    if !(v1
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        || !(v2
            & (0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong)
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong)
    {
        return 0 as libc::c_int != 0;
    }
    let mut o1: *mut Object = (v1 & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    let mut o2: *mut Object = (v2 & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    if (*o1).type_0 as libc::c_uint != (*o2).type_0 as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    match (*o1).type_0 as libc::c_uint {
        3 => {
            return (*(o1 as *mut Range)).from == (*(o2 as *mut Range)).from
                && (*(o1 as *mut Range)).to == (*(o2 as *mut Range)).to;
        }
        0 => {
            let mut s1: *mut String_0 = o1 as *mut String_0;
            let mut s2: *mut String_0 = o2 as *mut String_0;
            return (*s1).hash == (*s2).hash && (*s1).length == (*s2).length
                && memcmp(
                    ((*s1).data).as_mut_ptr() as *const libc::c_void,
                    ((*s2).data).as_mut_ptr() as *const libc::c_void,
                    (*s1).length as libc::c_ulong,
                ) == 0 as libc::c_int;
        }
        1 => {
            let mut l1: *mut List = o1 as *mut List;
            let mut l2: *mut List = o2 as *mut List;
            if (*l1).elements.count != (*l2).elements.count {
                return 0 as libc::c_int != 0;
            }
            let mut _v1: *mut Var = (*l1).elements.data;
            let mut _v2: *mut Var = (*l2).elements.data;
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*l1).elements.count {
                if !isValuesEqual(*_v1, *_v2) {
                    return 0 as libc::c_int != 0;
                }
                _v1 = _v1.offset(1);
                _v1;
                _v2 = _v2.offset(1);
                _v2;
                i = i.wrapping_add(1);
                i;
            }
            return 1 as libc::c_int != 0;
        }
        2 => {
            let mut m1: *mut Map = o1 as *mut Map;
            let mut m2: *mut Map = o2 as *mut Map;
            let mut e: *mut MapEntry = (*m1).entries;
            while e < ((*m1).entries).offset((*m1).capacity as isize) {
                if !((*e).key
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x1000000000000 as libc::c_long as uint64_t)
                {
                    let mut v: Var = mapGet(m2, (*e).key);
                    if v
                        == 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000000 as libc::c_long as uint64_t
                    {
                        return 0 as libc::c_int != 0;
                    }
                    if !isValuesEqual((*e).value, v) {
                        return 0 as libc::c_int != 0;
                    }
                }
                e = e.offset(1);
                e;
            }
            return 1 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    };
}
pub unsafe extern "C" fn isObjectHashable(mut type_0: ObjectType) -> bool {
    return type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == OBJ_RANGE as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == OBJ_CLASS as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn _toStringInternal(
    mut vm: *mut PKVM,
    v: Var,
    mut buff: *mut pkByteBuffer,
    mut outer: *mut OuterSequence,
    mut repr: bool,
) {
    if v == 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t
    {
        pkByteBufferAddString(
            buff,
            vm,
            b"null\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as uint32_t,
        );
        return;
    } else if v
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
        || v
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t
    {
        if v
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000003 as libc::c_long as uint64_t
        {
            pkByteBufferAddString(
                buff,
                vm,
                b"true\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as uint32_t,
            );
        } else {
            pkByteBufferAddString(
                buff,
                vm,
                b"false\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as uint32_t,
            );
        }
        return;
    } else if v & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        let mut value: libc::c_double = varToDouble(v);
        if value.is_nan() as i32 != 0 {
            pkByteBufferAddString(
                buff,
                vm,
                b"nan\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as uint32_t,
            );
        } else if if value.is_infinite() {
            if value.is_sign_positive() { 1 } else { -1 }
        } else {
            0
        } != 0
        {
            if value > 0.0f64 {
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"+inf\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as uint32_t,
                );
            } else {
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"-inf\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as uint32_t,
                );
            }
        } else {
            let mut num_buff: [libc::c_char; 24] = [0; 24];
            let mut length: libc::c_int = sprintf(
                num_buff.as_mut_ptr(),
                b"%.16g\0" as *const u8 as *const libc::c_char,
                varToDouble(v),
            );
            pkByteBufferAddString(buff, vm, num_buff.as_mut_ptr(), length as uint32_t);
        }
        return;
    } else if v
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        let mut obj: *const Object = (v & 0xffffffffffff as libc::c_long as uint64_t)
            as *mut Object;
        match (*obj).type_0 as libc::c_uint {
            0 => {
                let mut str: *const String_0 = obj as *const String_0;
                if outer.is_null() && !repr {
                    pkByteBufferAddString(
                        buff,
                        vm,
                        ((*str).data).as_ptr(),
                        (*str).length,
                    );
                    return;
                } else {
                    pkByteBufferWrite(buff, vm, '"' as i32 as uint8_t);
                    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
                    while i < (*str).length {
                        let mut c: libc::c_char = *((*str).data)
                            .as_ptr()
                            .offset(i as isize);
                        match c as libc::c_int {
                            34 => {
                                pkByteBufferAddString(
                                    buff,
                                    vm,
                                    b"\\\"\0" as *const u8 as *const libc::c_char,
                                    2 as libc::c_int as uint32_t,
                                );
                            }
                            92 => {
                                pkByteBufferAddString(
                                    buff,
                                    vm,
                                    b"\\\\\0" as *const u8 as *const libc::c_char,
                                    2 as libc::c_int as uint32_t,
                                );
                            }
                            10 => {
                                pkByteBufferAddString(
                                    buff,
                                    vm,
                                    b"\\n\0" as *const u8 as *const libc::c_char,
                                    2 as libc::c_int as uint32_t,
                                );
                            }
                            13 => {
                                pkByteBufferAddString(
                                    buff,
                                    vm,
                                    b"\\r\0" as *const u8 as *const libc::c_char,
                                    2 as libc::c_int as uint32_t,
                                );
                            }
                            9 => {
                                pkByteBufferAddString(
                                    buff,
                                    vm,
                                    b"\\t\0" as *const u8 as *const libc::c_char,
                                    2 as libc::c_int as uint32_t,
                                );
                            }
                            _ => {
                                if 0 as libc::c_int <= c as libc::c_int
                                    && c as libc::c_int <= 0xff as libc::c_int
                                    && *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                                        as libc::c_int
                                        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                                        != 0
                                {
                                    pkByteBufferWrite(buff, vm, c as uint8_t);
                                } else {
                                    pkByteBufferAddString(
                                        buff,
                                        vm,
                                        b"\\x\0" as *const u8 as *const libc::c_char,
                                        2 as libc::c_int as uint32_t,
                                    );
                                    let mut byte: uint8_t = c as uint8_t;
                                    pkByteBufferWrite(
                                        buff,
                                        vm,
                                        utilHexDigit(
                                            (byte as libc::c_int >> 4 as libc::c_int
                                                & 0xf as libc::c_int) as uint8_t,
                                            0 as libc::c_int != 0,
                                        ) as uint8_t,
                                    );
                                    pkByteBufferWrite(
                                        buff,
                                        vm,
                                        utilHexDigit(
                                            (byte as libc::c_int >> 0 as libc::c_int
                                                & 0xf as libc::c_int) as uint8_t,
                                            0 as libc::c_int != 0,
                                        ) as uint8_t,
                                    );
                                }
                            }
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                    pkByteBufferWrite(buff, vm, '"' as i32 as uint8_t);
                    return;
                }
            }
            1 => {
                let mut list: *const List = obj as *const List;
                if (*list).elements.count == 0 as libc::c_int as libc::c_uint {
                    pkByteBufferAddString(
                        buff,
                        vm,
                        b"[]\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as uint32_t,
                    );
                    return;
                }
                let mut seq: *mut OuterSequence = outer;
                while !seq.is_null() {
                    if (*seq).is_list as libc::c_int != 0
                        && (*seq).c2rust_unnamed.list == list
                    {
                        pkByteBufferAddString(
                            buff,
                            vm,
                            b"[...]\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int as uint32_t,
                        );
                        return;
                    }
                    seq = (*seq).outer;
                }
                let mut seq_list: OuterSequence = OuterSequence {
                    outer: 0 as *mut OuterSequence,
                    is_list: false,
                    c2rust_unnamed: C2RustUnnamed_1 {
                        list: 0 as *const List,
                    },
                };
                seq_list.outer = outer;
                seq_list.is_list = 1 as libc::c_int != 0;
                seq_list.c2rust_unnamed.list = list;
                pkByteBufferWrite(buff, vm, '[' as i32 as uint8_t);
                let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
                while i_0 < (*list).elements.count {
                    if i_0 != 0 as libc::c_int as libc::c_uint {
                        pkByteBufferAddString(
                            buff,
                            vm,
                            b", \0" as *const u8 as *const libc::c_char,
                            2 as libc::c_int as uint32_t,
                        );
                    }
                    _toStringInternal(
                        vm,
                        *((*list).elements.data).offset(i_0 as isize),
                        buff,
                        &mut seq_list,
                        1 as libc::c_int != 0,
                    );
                    i_0 = i_0.wrapping_add(1);
                    i_0;
                }
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            2 => {
                let mut map: *const Map = obj as *const Map;
                if ((*map).entries).is_null() {
                    pkByteBufferAddString(
                        buff,
                        vm,
                        b"{}\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as uint32_t,
                    );
                    return;
                }
                let mut seq_0: *mut OuterSequence = outer;
                while !seq_0.is_null() {
                    if !(*seq_0).is_list && (*seq_0).c2rust_unnamed.map == map {
                        pkByteBufferAddString(
                            buff,
                            vm,
                            b"{...}\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int as uint32_t,
                        );
                        return;
                    }
                    seq_0 = (*seq_0).outer;
                }
                let mut seq_map: OuterSequence = OuterSequence {
                    outer: 0 as *mut OuterSequence,
                    is_list: false,
                    c2rust_unnamed: C2RustUnnamed_1 {
                        list: 0 as *const List,
                    },
                };
                seq_map.outer = outer;
                seq_map.is_list = 0 as libc::c_int != 0;
                seq_map.c2rust_unnamed.map = map;
                pkByteBufferWrite(buff, vm, '{' as i32 as uint8_t);
                let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
                let mut _first: bool = 1 as libc::c_int != 0;
                loop {
                    let mut _done: bool = 0 as libc::c_int != 0;
                    while (*((*map).entries).offset(i_1 as isize)).key
                        == 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000000 as libc::c_long as uint64_t
                    {
                        i_1 = i_1.wrapping_add(1);
                        if !(i_1 >= (*map).capacity) {
                            continue;
                        }
                        _done = 1 as libc::c_int != 0;
                        break;
                    }
                    if _done {
                        break;
                    }
                    if !_first {
                        pkByteBufferAddString(
                            buff,
                            vm,
                            b", \0" as *const u8 as *const libc::c_char,
                            2 as libc::c_int as uint32_t,
                        );
                    }
                    _toStringInternal(
                        vm,
                        (*((*map).entries).offset(i_1 as isize)).key,
                        buff,
                        &mut seq_map,
                        1 as libc::c_int != 0,
                    );
                    pkByteBufferWrite(buff, vm, ':' as i32 as uint8_t);
                    _toStringInternal(
                        vm,
                        (*((*map).entries).offset(i_1 as isize)).value,
                        buff,
                        &mut seq_map,
                        1 as libc::c_int != 0,
                    );
                    i_1 = i_1.wrapping_add(1);
                    i_1;
                    _first = 0 as libc::c_int != 0;
                    if !(i_1 < (*map).capacity) {
                        break;
                    }
                }
                pkByteBufferWrite(buff, vm, '}' as i32 as uint8_t);
                return;
            }
            3 => {
                let mut range: *const Range = obj as *const Range;
                let mut buff_from: [libc::c_char; 24] = [0; 24];
                let len_from: libc::c_int = snprintf(
                    buff_from.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
                    b"%.16g\0" as *const u8 as *const libc::c_char,
                    (*range).from,
                );
                let mut buff_to: [libc::c_char; 24] = [0; 24];
                let len_to: libc::c_int = snprintf(
                    buff_to.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
                    b"%.16g\0" as *const u8 as *const libc::c_char,
                    (*range).to,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[Range:\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    buff_from.as_mut_ptr(),
                    len_from as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"..\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    buff_to.as_mut_ptr(),
                    len_to as uint32_t,
                );
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            4 => {
                let mut module: *const Module = obj as *const Module;
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[Module:\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as uint32_t,
                );
                if !((*module).name).is_null() {
                    pkByteBufferAddString(
                        buff,
                        vm,
                        ((*(*module).name).data).as_mut_ptr(),
                        (*(*module).name).length,
                    );
                } else {
                    pkByteBufferWrite(buff, vm, '"' as i32 as uint8_t);
                    pkByteBufferAddString(
                        buff,
                        vm,
                        ((*(*module).path).data).as_mut_ptr(),
                        (*(*module).path).length,
                    );
                    pkByteBufferWrite(buff, vm, '"' as i32 as uint8_t);
                }
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            5 => {
                let mut fn_0: *const Function = obj as *const Function;
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[Func:\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    (*fn_0).name,
                    strlen((*fn_0).name) as uint32_t,
                );
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            6 => {
                let mut closure: *const Closure = obj as *const Closure;
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[Closure:\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    (*(*closure).fn_0).name,
                    strlen((*(*closure).fn_0).name) as uint32_t,
                );
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            7 => {
                let mut mb: *const MethodBind = obj as *const MethodBind;
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[MethodBind:\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    (*(*(*mb).method).fn_0).name,
                    strlen((*(*(*mb).method).fn_0).name) as uint32_t,
                );
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            9 => {
                let mut fb: *const Fiber = obj as *const Fiber;
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[Fiber:\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    (*(*(*fb).closure).fn_0).name,
                    strlen((*(*(*fb).closure).fn_0).name) as uint32_t,
                );
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            8 => {
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[Upvalue]\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as uint32_t,
                );
                return;
            }
            10 => {
                let mut cls: *const Class = obj as *const Class;
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"[Class:\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int as uint32_t,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    ((*(*cls).name).data).as_mut_ptr(),
                    (*(*cls).name).length,
                );
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            11 => {
                let mut inst: *const Instance = obj as *const Instance;
                pkByteBufferWrite(buff, vm, '[' as i32 as uint8_t);
                pkByteBufferWrite(buff, vm, '\'' as i32 as uint8_t);
                pkByteBufferAddString(
                    buff,
                    vm,
                    ((*(*(*inst).cls).name).data).as_mut_ptr(),
                    (*(*(*inst).cls).name).length,
                );
                pkByteBufferAddString(
                    buff,
                    vm,
                    b"' instance at \0" as *const u8 as *const libc::c_char,
                    14 as libc::c_int as uint32_t,
                );
                let mut buff_addr: [libc::c_char; 20] = [0; 20];
                let mut ptr: *mut libc::c_char = buff_addr.as_mut_ptr();
                let fresh15 = ptr;
                ptr = ptr.offset(1);
                *fresh15 = '0' as i32 as libc::c_char;
                let fresh16 = ptr;
                ptr = ptr.offset(1);
                *fresh16 = 'x' as i32 as libc::c_char;
                let len: libc::c_int = snprintf(
                    ptr,
                    (::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong),
                    b"%08x\0" as *const u8 as *const libc::c_char,
                    inst as uintptr_t as libc::c_uint,
                );
                pkByteBufferAddString(buff, vm, buff_addr.as_mut_ptr(), len as uint32_t);
                pkByteBufferWrite(buff, vm, ']' as i32 as uint8_t);
                return;
            }
            _ => {}
        }
    }
    unreachable!();
}
pub unsafe extern "C" fn toString(mut vm: *mut PKVM, value: Var) -> *mut String_0 {
    if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        return (value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0;
    }
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    _toStringInternal(
        vm,
        value,
        &mut buff,
        0 as *mut OuterSequence,
        0 as libc::c_int != 0,
    );
    let mut ret: *mut String_0 = newStringLength(
        vm,
        buff.data as *const libc::c_char,
        buff.count,
    );
    pkByteBufferClear(&mut buff, vm);
    return ret;
}
pub unsafe extern "C" fn toRepr(mut vm: *mut PKVM, value: Var) -> *mut String_0 {
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    _toStringInternal(
        vm,
        value,
        &mut buff,
        0 as *mut OuterSequence,
        1 as libc::c_int != 0,
    );
    let mut ret: *mut String_0 = newStringLength(
        vm,
        buff.data as *const libc::c_char,
        buff.count,
    );
    pkByteBufferClear(&mut buff, vm);
    return ret;
}
pub unsafe extern "C" fn toBool(mut v: Var) -> bool {
    if v
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
        || v
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t
    {
        return v
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000003 as libc::c_long as uint64_t;
    }
    if v == 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t
    {
        return 0 as libc::c_int != 0;
    }
    if v & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        return varToDouble(v) != 0 as libc::c_int as libc::c_double;
    }
    let mut o: *mut Object = (v & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    match (*o).type_0 as libc::c_uint {
        0 => return (*(o as *mut String_0)).length != 0 as libc::c_int as libc::c_uint,
        1 => {
            return (*(o as *mut List)).elements.count != 0 as libc::c_int as libc::c_uint;
        }
        2 => return (*(o as *mut Map)).count != 0 as libc::c_int as libc::c_uint,
        3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => return 1 as libc::c_int != 0,
        _ => {}
    }
    unreachable!();
}
