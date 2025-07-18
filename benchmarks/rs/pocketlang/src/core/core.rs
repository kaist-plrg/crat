use ::libc;
extern "C" {
    pub type Compiler;
    
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn pkRealloc(
        vm: *mut PKVM,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn pkCheckArgcRange(
        vm: *mut PKVM,
        argc: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    ) -> bool;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
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
    fn pkByteBufferInit(self_0: *mut pkByteBuffer);
    fn pkByteBufferClear(self_0: *mut pkByteBuffer, vm: *mut PKVM);
    fn pkVarBufferWrite(self_0: *mut pkVarBuffer, vm: *mut PKVM, data: Var);
    fn pkVarBufferConcat(
        self_0: *mut pkVarBuffer,
        vm: *mut PKVM,
        other: *mut pkVarBuffer,
    );
    fn pkClosureBufferWrite(
        self_0: *mut pkClosureBuffer,
        vm: *mut PKVM,
        data: *mut Closure,
    );
    fn pkByteBufferAddString(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        str: *const libc::c_char,
        length: uint32_t,
    );
    fn pkByteBufferAddStringFmt(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        fmt: *const libc::c_char,
        _: ...
    );
    fn newStringLength(
        vm: *mut PKVM,
        text: *const libc::c_char,
        length: uint32_t,
    ) -> *mut String_0;
    fn newList(vm: *mut PKVM, size: uint32_t) -> *mut List;
    fn newMap(vm: *mut PKVM) -> *mut Map;
    fn newRange(vm: *mut PKVM, from: libc::c_double, to: libc::c_double) -> *mut Range;
    fn newModule(vm: *mut PKVM) -> *mut Module;
    fn newClosure(vm: *mut PKVM, fn_0: *mut Function) -> *mut Closure;
    fn newMethodBind(vm: *mut PKVM, method: *mut Closure) -> *mut MethodBind;
    fn newFiber(vm: *mut PKVM, closure: *mut Closure) -> *mut Fiber;
    fn newFunction(
        vm: *mut PKVM,
        name: *const libc::c_char,
        length: libc::c_int,
        owner: *mut Module,
        is_native: bool,
        docstring: *const libc::c_char,
        fn_index: *mut libc::c_int,
    ) -> *mut Function;
    fn newClass(
        vm: *mut PKVM,
        name: *const libc::c_char,
        length: libc::c_int,
        super_0: *mut Class,
        module: *mut Module,
        docstring: *const libc::c_char,
        cls_index: *mut libc::c_int,
    ) -> *mut Class;
    fn newInstance(vm: *mut PKVM, cls: *mut Class) -> *mut Instance;
    fn rangeAsList(vm: *mut PKVM, self_0: *mut Range) -> *mut List;
    fn stringLower(vm: *mut PKVM, self_0: *mut String_0) -> *mut String_0;
    fn stringUpper(vm: *mut PKVM, self_0: *mut String_0) -> *mut String_0;
    fn stringStrip(vm: *mut PKVM, self_0: *mut String_0) -> *mut String_0;
    fn stringReplace(
        vm: *mut PKVM,
        self_0: *mut String_0,
        old: *mut String_0,
        new_: *mut String_0,
        count: libc::c_int,
    ) -> *mut String_0;
    fn stringSplit(
        vm: *mut PKVM,
        self_0: *mut String_0,
        sep: *mut String_0,
    ) -> *mut List;
    fn stringFormat(vm: *mut PKVM, fmt: *const libc::c_char, _: ...) -> *mut String_0;
    fn stringJoin(
        vm: *mut PKVM,
        str1: *mut String_0,
        str2: *mut String_0,
    ) -> *mut String_0;
    fn listInsert(vm: *mut PKVM, self_0: *mut List, index: uint32_t, value: Var);
    fn listRemoveAt(vm: *mut PKVM, self_0: *mut List, index: uint32_t) -> Var;
    fn listClear(vm: *mut PKVM, self_0: *mut List);
    fn listAdd(vm: *mut PKVM, l1: *mut List, l2: *mut List) -> *mut List;
    fn mapGet(self_0: *mut Map, key: Var) -> Var;
    fn mapSet(vm: *mut PKVM, self_0: *mut Map, key: Var, value: Var);
    fn mapClear(vm: *mut PKVM, self_0: *mut Map);
    fn mapRemoveKey(vm: *mut PKVM, self_0: *mut Map, key: Var) -> Var;
    fn moduleGetStringAt(module: *mut Module, index: libc::c_int) -> *mut String_0;
    fn moduleSetGlobal(
        vm: *mut PKVM,
        module: *mut Module,
        name: *const libc::c_char,
        length: uint32_t,
        value: Var,
    ) -> uint32_t;
    fn moduleGetGlobalIndex(
        module: *mut Module,
        name: *const libc::c_char,
        length: uint32_t,
    ) -> libc::c_int;
    fn doubleToVar(value: libc::c_double) -> Var;
    fn varToDouble(value: Var) -> libc::c_double;
    fn getPkVarTypeName(type_0: PkVarType) -> *const libc::c_char;
    fn varTypeName(v: Var) -> *const libc::c_char;
    fn getVarType(v: Var) -> PkVarType;
    fn isValuesEqual(v1_0: Var, v2_0: Var) -> bool;
    fn isObjectHashable(type_0: ObjectType) -> bool;
    fn toString(vm: *mut PKVM, value: Var) -> *mut String_0;
    fn toRepr(vm: *mut PKVM, value: Var) -> *mut String_0;
    fn toBool(v: Var) -> bool;
    fn dumpFunctionCode(vm: *mut PKVM, func: *mut Function);
    fn utilHashString(string: *const libc::c_char) -> uint32_t;
    fn utilToNumber(
        str: *const libc::c_char,
        num: *mut libc::c_double,
    ) -> *const libc::c_char;
    fn vmPopTempRef(vm: *mut PKVM);
    fn vmCollectGarbage(vm: *mut PKVM);
    fn vmPushTempRef(vm: *mut PKVM, obj: *mut Object);
    fn vmRegisterModule(vm: *mut PKVM, module: *mut Module, key: *mut String_0);
    fn vmGetModule(vm: *mut PKVM, key: *mut String_0) -> *mut Module;
    fn vmPrepareFiber(
        vm: *mut PKVM,
        fiber: *mut Fiber,
        argc: libc::c_int,
        argv: *mut Var,
    ) -> bool;
    fn vmSwitchFiber(vm: *mut PKVM, fiber: *mut Fiber, value: *mut Var) -> bool;
    fn vmYieldFiber(vm: *mut PKVM, value: *mut Var);
    fn vmCallFunction(
        vm: *mut PKVM,
        fn_0: *mut Closure,
        argc: libc::c_int,
        argv: *mut Var,
        ret: *mut Var,
    ) -> PkResult;
    fn vmCallMethod(
        vm: *mut PKVM,
        self_0: Var,
        fn_0: *mut Closure,
        argc: libc::c_int,
        argv: *mut Var,
        ret: *mut Var,
    ) -> PkResult;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub const PK_RESULT_RUNTIME_ERROR: PkResult = 3;
pub const PK_RESULT_COMPILE_ERROR: PkResult = 2;
pub const PK_RESULT_UNEXPECTED_EOF: PkResult = 1;
pub const PK_RESULT_SUCCESS: PkResult = 0;
pub type PkResult = libc::c_uint;
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
#[inline]
unsafe extern "C" fn isNumeric(mut var: Var, mut value: *mut libc::c_double) -> bool {
    if var & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        *value = varToDouble(var);
        return 1 as libc::c_int != 0;
    }
    if var
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
        || var
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t
    {
        *value = (var
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000003 as libc::c_long as uint64_t) as libc::c_int
            as libc::c_double;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn isInteger(mut var: Var, mut value: *mut int64_t) -> bool {
    let mut number: libc::c_double = 0.;
    if isNumeric(var, &mut number) {
        if floor(number) == number {
            *value = number as int64_t;
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn validateNumeric(
    mut vm: *mut PKVM,
    mut var: Var,
    mut value: *mut libc::c_double,
    mut name: *const libc::c_char,
) -> bool {
    if isNumeric(var, value) {
        return 1 as libc::c_int != 0;
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"$ must be a numeric value.\0" as *const u8 as *const libc::c_char,
        name,
    );
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn validateInteger(
    mut vm: *mut PKVM,
    mut var: Var,
    mut value: *mut int64_t,
    mut name: *const libc::c_char,
) -> bool {
    if isInteger(var, value) {
        return 1 as libc::c_int != 0;
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"$ must be an Integer.\0" as *const u8 as *const libc::c_char,
        name,
    );
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn validateCond(
    mut vm: *mut PKVM,
    mut condition: bool,
    mut err: *const libc::c_char,
) -> bool {
    if !condition {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            err,
            if err.is_null() {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(err) as uint32_t
            },
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn validateArgString(
    mut vm: *mut PKVM,
    mut arg: libc::c_int,
    mut value: *mut *mut String_0,
) -> bool {
    let mut var: Var = *((*(*vm).fiber).ret).offset(arg as isize);
    if !(var
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        || (*((var & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint != OBJ_STRING as libc::c_int as libc::c_uint
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, arg);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a string at argument $.\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
            0 as libc::c_int,
        );
        return 0 as libc::c_int != 0;
    }
    *value = (var & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
        as *mut String_0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn validateArgList(
    mut vm: *mut PKVM,
    mut arg: libc::c_int,
    mut value: *mut *mut List,
) -> bool {
    let mut var: Var = *((*(*vm).fiber).ret).offset(arg as isize);
    if !(var
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        || (*((var & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint != OBJ_LIST as libc::c_int as libc::c_uint
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, arg);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a list at argument $.\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
            0 as libc::c_int,
        );
        return 0 as libc::c_int != 0;
    }
    *value = (var & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
        as *mut List;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn validateArgClosure(
    mut vm: *mut PKVM,
    mut arg: libc::c_int,
    mut value: *mut *mut Closure,
) -> bool {
    let mut var: Var = *((*(*vm).fiber).ret).offset(arg as isize);
    if !(var
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        || (*((var & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint != OBJ_CLOSURE as libc::c_int as libc::c_uint
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, arg);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a closure at argument $.\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
            0 as libc::c_int,
        );
        return 0 as libc::c_int != 0;
    }
    *value = (var & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
        as *mut Closure;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn initializeCore(mut vm: *mut PKVM) {
    initializeBuiltinFunctions(vm);
    initializeCoreModules(vm);
    initializePrimitiveClasses(vm);
}
pub unsafe extern "C" fn initializeModule(
    mut vm: *mut PKVM,
    mut module: *mut Module,
    mut is_main: bool,
) {
    let mut path: *mut String_0 = (*module).path;
    let mut name: *mut String_0 = 0 as *mut String_0;
    if is_main {
        name = newStringLength(
            vm,
            b"@main\0" as *const u8 as *const libc::c_char,
            if (b"@main\0" as *const u8 as *const libc::c_char).is_null() {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"@main\0" as *const u8 as *const libc::c_char) as uint32_t
            },
        );
        (*module).name = name;
        vmPushTempRef(vm, &mut (*name)._super);
    } else {
        name = (*module).name;
    }
    if !path.is_null() {
        moduleSetGlobal(
            vm,
            module,
            b"__file__\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as uint32_t,
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*path)._super as *mut Object as uintptr_t,
        );
    }
    moduleSetGlobal(
        vm,
        module,
        b"_name\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as uint32_t,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*name)._super as *mut Object as uintptr_t,
    );
    if is_main {
        vmPopTempRef(vm);
    }
}
pub unsafe extern "C" fn varToString(
    mut vm: *mut PKVM,
    mut self_0: Var,
    mut repr: bool,
) -> *mut String_0 {
    if self_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((self_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut closure: *mut Closure = 0 as *mut Closure;
        let mut has_method: bool = 0 as libc::c_int != 0;
        if !repr {
            let mut name: *mut String_0 = newStringLength(
                vm,
                b"_str\0" as *const u8 as *const libc::c_char,
                if (b"_str\0" as *const u8 as *const libc::c_char).is_null() {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(b"_str\0" as *const u8 as *const libc::c_char) as uint32_t
                },
            );
            vmPushTempRef(vm, &mut (*name)._super);
            has_method = hasMethod(vm, self_0, name, &mut closure);
            vmPopTempRef(vm);
        }
        if !has_method {
            let mut name_0: *mut String_0 = newStringLength(
                vm,
                b"_repr\0" as *const u8 as *const libc::c_char,
                if (b"_repr\0" as *const u8 as *const libc::c_char).is_null() {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(b"_repr\0" as *const u8 as *const libc::c_char) as uint32_t
                },
            );
            vmPushTempRef(vm, &mut (*name_0)._super);
            has_method = hasMethod(vm, self_0, name_0, &mut closure);
            vmPopTempRef(vm);
        }
        if has_method {
            let mut ret: Var = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            let mut result: PkResult = vmCallMethod(
                vm,
                self_0,
                closure,
                0 as libc::c_int,
                0 as *mut Var,
                &mut ret,
            );
            if result as libc::c_uint != PK_RESULT_SUCCESS as libc::c_int as libc::c_uint
            {
                return 0 as *mut String_0;
            }
            if !(ret
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && (*((ret & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                    .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint)
            {
                (*(*vm).fiber)
                    .error = newStringLength(
                    vm,
                    b"method _str returned non-string type.\0" as *const u8
                        as *const libc::c_char,
                    if (b"method _str returned non-string type.\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                    {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        strlen(
                            b"method _str returned non-string type.\0" as *const u8
                                as *const libc::c_char,
                        ) as uint32_t
                    },
                );
                return 0 as *mut String_0;
            }
            return (ret & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut String_0;
        }
    }
    if repr {
        return toRepr(vm, self_0);
    }
    return toString(vm, self_0);
}
#[inline]
unsafe extern "C" fn _callUnaryOpMethod(
    mut vm: *mut PKVM,
    mut self_0: Var,
    mut method_name: *const libc::c_char,
    mut ret: *mut Var,
) -> bool {
    let mut closure: *mut Closure = 0 as *mut Closure;
    let mut name: *mut String_0 = newStringLength(
        vm,
        method_name,
        if method_name.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(method_name) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*name)._super);
    let mut has_method: bool = hasMethod(vm, self_0, name, &mut closure);
    vmPopTempRef(vm);
    if !has_method {
        return 0 as libc::c_int != 0;
    }
    vmCallMethod(vm, self_0, closure, 0 as libc::c_int, 0 as *mut Var, ret);
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn _callBinaryOpMethod(
    mut vm: *mut PKVM,
    mut self_0: Var,
    mut other: Var,
    mut method_name: *const libc::c_char,
    mut ret: *mut Var,
) -> bool {
    let mut closure: *mut Closure = 0 as *mut Closure;
    let mut name: *mut String_0 = newStringLength(
        vm,
        method_name,
        if method_name.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(method_name) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*name)._super);
    let mut has_method: bool = hasMethod(vm, self_0, name, &mut closure);
    vmPopTempRef(vm);
    if !has_method {
        return 0 as libc::c_int != 0;
    }
    vmCallMethod(vm, self_0, closure, 1 as libc::c_int, &mut other, ret);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _collectMethods(
    mut vm: *mut PKVM,
    mut list: *mut List,
    mut cls: *mut Class,
) {
    if cls.is_null() {
        return;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*cls).methods.count {
        pkVarBufferWrite(
            &mut (*list).elements,
            vm,
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*(newStringLength
                    as unsafe extern "C" fn(
                        *mut PKVM,
                        *const libc::c_char,
                        uint32_t,
                    ) -> *mut String_0)(
                    vm,
                    (*(**((*cls).methods.data).offset(i as isize)).fn_0).name,
                    (if ((*(**((*cls).methods.data).offset(i as isize)).fn_0).name)
                        .is_null()
                    {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        (strlen
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> libc::c_ulong)(
                            (*(**((*cls).methods.data).offset(i as isize)).fn_0).name,
                        ) as uint32_t
                    }),
                ))
                    ._super as *mut Object as uintptr_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    _collectMethods(vm, list, (*cls).super_class);
}
unsafe extern "C" fn coreHelp(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret)
        as libc::c_long as libc::c_int - 1 as libc::c_int;
    if argc != 0 as libc::c_int && argc != 1 as libc::c_int {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Invalid argument count.\0" as *const u8 as *const libc::c_char,
            if (b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    if argc == 0 as libc::c_int {
        if ((*vm).config.stdout_write).is_none() {
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            return;
        }
        ((*vm).config.stdout_write)
            .unwrap()(
            vm,
            b"TODO: print help here\n\0" as *const u8 as *const libc::c_char,
        );
    } else if argc == 1 as libc::c_int {
        if ((*vm).config.stdout_write).is_none() {
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            return;
        }
        let mut value: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
        if value
            & (0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong)
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
            && (*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                .type_0 as libc::c_uint == OBJ_CLOSURE as libc::c_int as libc::c_uint
        {
            let mut closure: *mut Closure = (value
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Closure;
            if !((*(*closure).fn_0).docstring).is_null() {
                ((*vm).config.stdout_write).unwrap()(vm, (*(*closure).fn_0).docstring);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\n\0" as *const u8 as *const libc::c_char);
            } else {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"function '\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write).unwrap()(vm, (*(*closure).fn_0).name);
                ((*vm).config.stdout_write)
                    .unwrap()(
                    vm,
                    b"()' doesn't have a docstring.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if value
            & (0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong)
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
            && (*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                .type_0 as libc::c_uint == OBJ_METHOD_BIND as libc::c_int as libc::c_uint
        {
            let mut mb: *mut MethodBind = (value
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut MethodBind;
            if !((*(*(*mb).method).fn_0).docstring).is_null() {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, (*(*(*mb).method).fn_0).docstring);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\n\0" as *const u8 as *const libc::c_char);
            } else {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"method '\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write).unwrap()(vm, (*(*(*mb).method).fn_0).name);
                ((*vm).config.stdout_write)
                    .unwrap()(
                    vm,
                    b"()' doesn't have a docstring.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if value
            & (0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong)
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
            && (*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                .type_0 as libc::c_uint == OBJ_CLASS as libc::c_int as libc::c_uint
        {
            let mut cls: *mut Class = (value
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Class;
            if !((*cls).docstring).is_null() {
                ((*vm).config.stdout_write).unwrap()(vm, (*cls).docstring);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\n\0" as *const u8 as *const libc::c_char);
            } else {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"class '\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, ((*(*cls).name).data).as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(
                    vm,
                    b"' doesn't have a docstring.\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            (*(*vm).fiber)
                .error = newStringLength(
                vm,
                b"Expected a Closure, MethodBind or Class to get help.\0" as *const u8
                    as *const libc::c_char,
                if (b"Expected a Closure, MethodBind or Class to get help.\0"
                    as *const u8 as *const libc::c_char)
                    .is_null()
                {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(
                        b"Expected a Closure, MethodBind or Class to get help.\0"
                            as *const u8 as *const libc::c_char,
                    ) as uint32_t
                },
            );
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            return;
        }
    }
}
static mut _pk_doc_coreHelp: *const libc::c_char = b"help([value:Closure|MethodBind|Class]) -> Null\n\nIt'll print the docstring the object and return.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreDir(mut vm: *mut PKVM) {
    let mut v: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    match getVarType(v) as libc::c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 9 | 11 => {
            let mut list: *mut List = newList(vm, 8 as libc::c_int as uint32_t);
            vmPushTempRef(vm, &mut (*list)._super);
            _collectMethods(vm, list, getClass(vm, v));
            vmPopTempRef(vm);
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*list)._super as *mut Object as uintptr_t;
            return;
        }
        8 => {
            let mut m: *mut Module = (v & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut Module;
            let mut list_0: *mut List = newList(vm, 8 as libc::c_int as uint32_t);
            vmPushTempRef(vm, &mut (*list_0)._super);
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*m).globals.count {
                let mut name: Var = *((*m).constants.data)
                    .offset(*((*m).global_names.data).offset(i as isize) as isize);
                pkVarBufferWrite(&mut (*list_0).elements, vm, name);
                i = i.wrapping_add(1);
                i;
            }
            vmPopTempRef(vm);
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*list_0)._super as *mut Object as uintptr_t;
            return;
        }
        12 => {
            let mut cls: *mut Class = (v & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut Class;
            let mut list_1: *mut List = newList(vm, 8 as libc::c_int as uint32_t);
            vmPushTempRef(vm, &mut (*list_1)._super);
            _collectMethods(vm, list_1, cls);
            vmPopTempRef(vm);
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*list_1)._super as *mut Object as uintptr_t;
            return;
        }
        13 => {
            let mut inst: *mut Instance = (v
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Instance;
            let mut list_2: *mut List = newList(vm, 8 as libc::c_int as uint32_t);
            vmPushTempRef(vm, &mut (*list_2)._super);
            let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
            while i_0 < (*(*inst).attribs).capacity {
                let mut key: Var = (*((*(*inst).attribs).entries).offset(i_0 as isize))
                    .key;
                if !(key
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x1000000000000 as libc::c_long as uint64_t)
                {
                    pkVarBufferWrite(&mut (*list_2).elements, vm, key);
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            _collectMethods(vm, list_2, (*inst).cls);
            vmPopTempRef(vm);
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*list_2)._super as *mut Object as uintptr_t;
            return;
        }
        _ => {}
    }
    unreachable!();
}
static mut _pk_doc_coreDir: *const libc::c_char = b"dir(v:Var) -> List[String]\n\nIt'll return all the elements of the variable [v]. If [v] is a module it'll return the names of globals, functions, and classes. If it's an instance it'll return all the attributes and methods.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreAssert(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret)
        as libc::c_long as libc::c_int - 1 as libc::c_int;
    if argc != 1 as libc::c_int && argc != 2 as libc::c_int {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Invalid argument count.\0" as *const u8 as *const libc::c_char,
            if (b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    if !toBool(*((*(*vm).fiber).ret).offset(1 as libc::c_int as isize)) {
        let mut msg: *mut String_0 = 0 as *mut String_0;
        if argc == 2 as libc::c_int {
            if !(*((*(*vm).fiber).ret).offset(2 as libc::c_int as isize)
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && (*((*((*(*vm).fiber).ret).offset(2 as libc::c_int as isize)
                    & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                    .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint)
            {
                msg = varToString(
                    vm,
                    *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize),
                    0 as libc::c_int != 0,
                );
                if msg.is_null() {
                    return;
                }
            } else {
                msg = (*((*(*vm).fiber).ret).offset(2 as libc::c_int as isize)
                    & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                    as *mut String_0;
            }
            vmPushTempRef(vm, &mut (*msg)._super);
            (*(*vm).fiber)
                .error = stringFormat(
                vm,
                b"Assertion failed: '@'.\0" as *const u8 as *const libc::c_char,
                msg,
            );
            vmPopTempRef(vm);
        } else {
            (*(*vm).fiber)
                .error = newStringLength(
                vm,
                b"Assertion failed.\0" as *const u8 as *const libc::c_char,
                if (b"Assertion failed.\0" as *const u8 as *const libc::c_char).is_null()
                {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(b"Assertion failed.\0" as *const u8 as *const libc::c_char)
                        as uint32_t
                },
            );
        }
    }
}
static mut _pk_doc_coreAssert: *const libc::c_char = b"assert(condition:Bool [, msg:String]) -> Null\n\nIf the condition is false it'll terminate the current fiber with the optional error message\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreBin(mut vm: *mut PKVM) {
    let mut value: int64_t = 0;
    if !validateInteger(
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
        &mut value,
        b"Argument 1\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    let mut buff: [libc::c_char; 68] = [0; 68];
    let mut negative: bool = if value < 0 as libc::c_int as libc::c_long {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
    if negative {
        value = -value;
    }
    let mut ptr: *mut libc::c_char = buff
        .as_mut_ptr()
        .offset(68 as libc::c_int as isize)
        .offset(-(1 as libc::c_int as isize));
    let fresh0 = ptr;
    ptr = ptr.offset(-1);
    *fresh0 = '\0' as i32 as libc::c_char;
    if value != 0 as libc::c_int as libc::c_long {
        while value > 0 as libc::c_int as libc::c_long {
            let fresh1 = ptr;
            ptr = ptr.offset(-1);
            *fresh1 = ('0' as i32 as libc::c_long
                + (value & 1 as libc::c_int as libc::c_long)) as libc::c_char;
            value >>= 1 as libc::c_int;
        }
    } else {
        let fresh2 = ptr;
        ptr = ptr.offset(-1);
        *fresh2 = '0' as i32 as libc::c_char;
    }
    let fresh3 = ptr;
    ptr = ptr.offset(-1);
    *fresh3 = 'b' as i32 as libc::c_char;
    let fresh4 = ptr;
    ptr = ptr.offset(-1);
    *fresh4 = '0' as i32 as libc::c_char;
    if negative {
        let fresh5 = ptr;
        ptr = ptr.offset(-1);
        *fresh5 = '-' as i32 as libc::c_char;
    }
    let mut length: uint32_t = buff
        .as_mut_ptr()
        .offset(68 as libc::c_int as isize)
        .offset(-(1 as libc::c_int as isize))
        .offset_from(ptr.offset(1 as libc::c_int as isize)) as libc::c_long as uint32_t;
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                uint32_t,
            ) -> *mut String_0)(vm, ptr.offset(1 as libc::c_int as isize), length))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc_coreBin: *const libc::c_char = b"bin(value:Number) -> String\n\nReturns as a binary value string with '0b' prefix.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreHex(mut vm: *mut PKVM) {
    let mut value: int64_t = 0;
    if !validateInteger(
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
        &mut value,
        b"Argument 1\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    let mut buff: [libc::c_char; 20] = [0; 20];
    let mut ptr: *mut libc::c_char = buff.as_mut_ptr();
    if value < 0 as libc::c_int as libc::c_long {
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        *fresh6 = '-' as i32 as libc::c_char;
    }
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = '0' as i32 as libc::c_char;
    let fresh8 = ptr;
    ptr = ptr.offset(1);
    *fresh8 = 'x' as i32 as libc::c_char;
    if value > 4294967295 as libc::c_uint as libc::c_long
        || value < -(4294967295 as libc::c_uint as int64_t)
    {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Integer is too large.\0" as *const u8 as *const libc::c_char,
            if (b"Integer is too large.\0" as *const u8 as *const libc::c_char).is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Integer is too large.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    let mut _x: uint32_t = (if value < 0 as libc::c_int as libc::c_long {
        -value
    } else {
        value
    }) as uint32_t;
    let mut length: libc::c_int = sprintf(
        ptr,
        b"%x\0" as *const u8 as *const libc::c_char,
        _x,
    );
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                uint32_t,
            ) -> *mut String_0)(
            vm,
            buff.as_mut_ptr(),
            ptr.offset(length as isize).offset_from(buff.as_mut_ptr()) as libc::c_long
                as uint32_t,
        ))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc_coreHex: *const libc::c_char = b"hex(value:Number) -> String\n\nReturns as a hexadecimal value string with '0x' prefix.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreYield(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret)
        as libc::c_long as libc::c_int - 1 as libc::c_int;
    if argc > 1 as libc::c_int {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Invalid argument count.\0" as *const u8 as *const libc::c_char,
            if (b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    vmYieldFiber(
        vm,
        if argc == 1 as libc::c_int {
            &mut *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize)
        } else {
            0 as *mut Var
        },
    );
}
static mut _pk_doc_coreYield: *const libc::c_char = b"yield([value:Var]) -> Var\n\nReturn the current function with the yield [value] to current running fiber. If the fiber is resumed, it'll run from the next statement of the yield() call. If the fiber resumed with with a value, the return value of the yield() would be that value otherwise null.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc_coreToString: *const libc::c_char = b"str(valueVar) -> String\n\nReturns the string representation of the value.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreToString(mut vm: *mut PKVM) {
    let mut str: *mut String_0 = varToString(
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
        0 as libc::c_int != 0,
    );
    if str.is_null() {
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*str)._super as *mut Object as uintptr_t;
}
static mut _pk_doc_coreChr: *const libc::c_char = b"chr(value:Num) -> String\n\nReturns the ASCII string value of the integer argument.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreChr(mut vm: *mut PKVM) {
    let mut num: int64_t = 0;
    if !validateInteger(
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
        &mut num,
        b"Argument 1\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    if !(0 as libc::c_int as libc::c_long <= num
        && num <= 0xff as libc::c_int as libc::c_long)
    {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"The number should be in range 0x00 to 0xff.\0" as *const u8
                as *const libc::c_char,
            if (b"The number should be in range 0x00 to 0xff.\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"The number should be in range 0x00 to 0xff.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    let mut c: libc::c_char = num as libc::c_char;
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                uint32_t,
            ) -> *mut String_0)(vm, &mut c, 1 as libc::c_int as uint32_t))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc_coreOrd: *const libc::c_char = b"ord(value:String) -> Number\n\nReturns integer value of the given ASCII character.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreOrd(mut vm: *mut PKVM) {
    let mut c: *mut String_0 = 0 as *mut String_0;
    if !validateArgString(vm, 1 as libc::c_int, &mut c) {
        return;
    }
    if (*c).length != 1 as libc::c_int as libc::c_uint {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Expected a string of length 1.\0" as *const u8 as *const libc::c_char,
            if (b"Expected a string of length 1.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Expected a string of length 1.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    } else {
        *(*(*vm).fiber)
            .ret = doubleToVar(
            *((*c).data).as_mut_ptr().offset(0 as libc::c_int as isize) as libc::c_double,
        );
        return;
    };
}
static mut _pk_doc_coreMin: *const libc::c_char = b"min(a:Var, b:Var) -> Bool\n\nReturns minimum of [a] and [b].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreMin(mut vm: *mut PKVM) {
    let mut a: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    let mut b: Var = *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize);
    let mut islesser: Var = varLesser(vm, a, b);
    if !((*(*vm).fiber).error).is_null() {
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    if toBool(islesser) {
        *(*(*vm).fiber).ret = a;
        return;
    }
    *(*(*vm).fiber).ret = b;
}
unsafe extern "C" fn coreMax(mut vm: *mut PKVM) {
    let mut a: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    let mut b: Var = *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize);
    let mut islesser: Var = varLesser(vm, a, b);
    if !((*(*vm).fiber).error).is_null() {
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    if toBool(islesser) {
        *(*(*vm).fiber).ret = b;
        return;
    }
    *(*(*vm).fiber).ret = a;
}
static mut _pk_doc_coreMax: *const libc::c_char = b"max(a:var, b:var) -> Bool\n\nReturns maximum of [a] and [b].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn corePrint(mut vm: *mut PKVM) {
    if ((*vm).config.stdout_write).is_none() {
        return;
    }
    let mut i: libc::c_int = 1 as libc::c_int;
    while i
        <= ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int
    {
        if i != 1 as libc::c_int {
            ((*vm).config.stdout_write)
                .unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
        }
        let mut str: *mut String_0 = varToString(
            vm,
            *((*(*vm).fiber).ret).offset(i as isize),
            0 as libc::c_int != 0,
        );
        if str.is_null() {
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            return;
        }
        ((*vm).config.stdout_write).unwrap()(vm, ((*str).data).as_mut_ptr());
        i += 1;
        i;
    }
    ((*vm).config.stdout_write)
        .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut _pk_doc_corePrint: *const libc::c_char = b"print(...) -> Null\n\nWrite each argument as space seperated, to the stdout and ends with a newline.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreInput(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret)
        as libc::c_long as libc::c_int - 1 as libc::c_int;
    if argc > 1 as libc::c_int {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Invalid argument count.\0" as *const u8 as *const libc::c_char,
            if (b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    if ((*vm).config.stdin_read).is_none() {
        return;
    }
    if argc == 1 as libc::c_int {
        let mut str: *mut String_0 = varToString(
            vm,
            *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
            0 as libc::c_int != 0,
        );
        if str.is_null() {
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            return;
        }
        ((*vm).config.stdout_write).unwrap()(vm, ((*str).data).as_mut_ptr());
    }
    let mut str_0: *mut libc::c_char = ((*vm).config.stdin_read).unwrap()(vm);
    if str_0.is_null() {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Input function failed.\0" as *const u8 as *const libc::c_char,
            if (b"Input function failed.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Input function failed.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    let mut line: *mut String_0 = newStringLength(
        vm,
        str_0,
        if str_0.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(str_0) as uint32_t
        },
    );
    pkRealloc(vm, str_0 as *mut libc::c_void, 0 as libc::c_int as size_t);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*line)._super as *mut Object as uintptr_t;
}
static mut _pk_doc_coreInput: *const libc::c_char = b"input([msg:Var]) -> String\n\nRead a line from stdin and returns it without the line ending. Accepting an optional argument [msg] and prints it before reading.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreExit(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret)
        as libc::c_long as libc::c_int - 1 as libc::c_int;
    if argc > 1 as libc::c_int {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Invalid argument count.\0" as *const u8 as *const libc::c_char,
            if (b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Invalid argument count.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    let mut value: int64_t = 0 as libc::c_int as int64_t;
    if argc == 1 as libc::c_int {
        if !validateInteger(
            vm,
            *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
            &mut value,
            b"Argument 1\0" as *const u8 as *const libc::c_char,
        ) {
            return;
        }
    }
    exit(value as libc::c_int);
}
static mut _pk_doc_coreExit: *const libc::c_char = b"exit([value:Number]) -> Null\n\nExit the process with an optional exit code provided by the argument [value]. The default exit code is would be 0.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc_coreListAppend: *const libc::c_char = b"list_append(self:List, value:Var) -> List\n\nAppend the [value] to the list [self] and return the list.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreListAppend(mut vm: *mut PKVM) {
    let mut list: *mut List = 0 as *mut List;
    if !validateArgList(vm, 1 as libc::c_int, &mut list) {
        return;
    }
    let mut elem: Var = *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize);
    pkVarBufferWrite(&mut (*list).elements, vm, elem);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*list)._super as *mut Object as uintptr_t;
}
static mut _pk_doc_coreListJoin: *const libc::c_char = b"list_join(self:List) -> String\n\nConcatinate the elements of the list and return as a string.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn coreListJoin(mut vm: *mut PKVM) {
    let mut list: *mut List = 0 as *mut List;
    if !validateArgList(vm, 1 as libc::c_int, &mut list) {
        return;
    }
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*list).elements.count {
        let mut str: *mut String_0 = varToString(
            vm,
            *((*list).elements.data).offset(i as isize),
            0 as libc::c_int != 0,
        );
        if str.is_null() {
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            return;
        }
        vmPushTempRef(vm, &mut (*str)._super);
        pkByteBufferAddString(&mut buff, vm, ((*str).data).as_mut_ptr(), (*str).length);
        vmPopTempRef(vm);
        i = i.wrapping_add(1);
        i;
    }
    let mut str_0: *mut String_0 = newStringLength(
        vm,
        buff.data as *const libc::c_char,
        buff.count,
    );
    pkByteBufferClear(&mut buff, vm);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*str_0)._super as *mut Object as uintptr_t;
}
unsafe extern "C" fn initializeBuiltinFN(
    mut vm: *mut PKVM,
    mut bfn: *mut *mut Closure,
    mut name: *const libc::c_char,
    mut length: libc::c_int,
    mut arity: libc::c_int,
    mut ptr: pkNativeFn,
    mut docstring: *const libc::c_char,
) {
    let mut fn_0: *mut Function = newFunction(
        vm,
        name,
        length,
        0 as *mut Module,
        1 as libc::c_int != 0,
        docstring,
        0 as *mut libc::c_int,
    );
    (*fn_0).arity = arity;
    (*fn_0).c2rust_unnamed.native = ptr;
    vmPushTempRef(vm, &mut (*fn_0)._super);
    *bfn = newClosure(vm, fn_0);
    vmPopTempRef(vm);
}
unsafe extern "C" fn initializeBuiltinFunctions(mut vm: *mut PKVM) {
    let fresh9 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh9 as isize),
        b"help\0" as *const u8 as *const libc::c_char,
        strlen(b"help\0" as *const u8 as *const libc::c_char) as libc::c_int,
        -(1 as libc::c_int),
        Some(coreHelp as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreHelp,
    );
    let fresh10 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh10 as isize),
        b"dir\0" as *const u8 as *const libc::c_char,
        strlen(b"dir\0" as *const u8 as *const libc::c_char) as libc::c_int,
        1 as libc::c_int,
        Some(coreDir as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreDir,
    );
    let fresh11 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh11 as isize),
        b"assert\0" as *const u8 as *const libc::c_char,
        strlen(b"assert\0" as *const u8 as *const libc::c_char) as libc::c_int,
        -(1 as libc::c_int),
        Some(coreAssert as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreAssert,
    );
    let fresh12 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh12 as isize),
        b"bin\0" as *const u8 as *const libc::c_char,
        strlen(b"bin\0" as *const u8 as *const libc::c_char) as libc::c_int,
        1 as libc::c_int,
        Some(coreBin as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreBin,
    );
    let fresh13 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh13 as isize),
        b"hex\0" as *const u8 as *const libc::c_char,
        strlen(b"hex\0" as *const u8 as *const libc::c_char) as libc::c_int,
        1 as libc::c_int,
        Some(coreHex as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreHex,
    );
    let fresh14 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh14 as isize),
        b"yield\0" as *const u8 as *const libc::c_char,
        strlen(b"yield\0" as *const u8 as *const libc::c_char) as libc::c_int,
        -(1 as libc::c_int),
        Some(coreYield as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreYield,
    );
    let fresh15 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh15 as isize),
        b"str\0" as *const u8 as *const libc::c_char,
        strlen(b"str\0" as *const u8 as *const libc::c_char) as libc::c_int,
        1 as libc::c_int,
        Some(coreToString as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreToString,
    );
    let fresh16 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh16 as isize),
        b"chr\0" as *const u8 as *const libc::c_char,
        strlen(b"chr\0" as *const u8 as *const libc::c_char) as libc::c_int,
        1 as libc::c_int,
        Some(coreChr as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreChr,
    );
    let fresh17 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh17 as isize),
        b"ord\0" as *const u8 as *const libc::c_char,
        strlen(b"ord\0" as *const u8 as *const libc::c_char) as libc::c_int,
        1 as libc::c_int,
        Some(coreOrd as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreOrd,
    );
    let fresh18 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh18 as isize),
        b"min\0" as *const u8 as *const libc::c_char,
        strlen(b"min\0" as *const u8 as *const libc::c_char) as libc::c_int,
        2 as libc::c_int,
        Some(coreMin as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreMin,
    );
    let fresh19 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh19 as isize),
        b"max\0" as *const u8 as *const libc::c_char,
        strlen(b"max\0" as *const u8 as *const libc::c_char) as libc::c_int,
        2 as libc::c_int,
        Some(coreMax as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreMax,
    );
    let fresh20 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh20 as isize),
        b"print\0" as *const u8 as *const libc::c_char,
        strlen(b"print\0" as *const u8 as *const libc::c_char) as libc::c_int,
        -(1 as libc::c_int),
        Some(corePrint as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_corePrint,
    );
    let fresh21 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh21 as isize),
        b"input\0" as *const u8 as *const libc::c_char,
        strlen(b"input\0" as *const u8 as *const libc::c_char) as libc::c_int,
        -(1 as libc::c_int),
        Some(coreInput as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreInput,
    );
    let fresh22 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh22 as isize),
        b"exit\0" as *const u8 as *const libc::c_char,
        strlen(b"exit\0" as *const u8 as *const libc::c_char) as libc::c_int,
        -(1 as libc::c_int),
        Some(coreExit as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreExit,
    );
    let fresh23 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh23 as isize),
        b"list_append\0" as *const u8 as *const libc::c_char,
        strlen(b"list_append\0" as *const u8 as *const libc::c_char) as libc::c_int,
        2 as libc::c_int,
        Some(coreListAppend as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreListAppend,
    );
    let fresh24 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    initializeBuiltinFN(
        vm,
        &mut *((*vm).builtins_funcs).as_mut_ptr().offset(fresh24 as isize),
        b"list_join\0" as *const u8 as *const libc::c_char,
        strlen(b"list_join\0" as *const u8 as *const libc::c_char) as libc::c_int,
        1 as libc::c_int,
        Some(coreListJoin as unsafe extern "C" fn(*mut PKVM) -> ()),
        _pk_doc_coreListJoin,
    );
}
pub unsafe extern "C" fn newModuleInternal(
    mut vm: *mut PKVM,
    mut name: *const libc::c_char,
) -> *mut Module {
    let mut _name: *mut String_0 = newStringLength(
        vm,
        name,
        if name.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(name) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*_name)._super);
    !(vmGetModule(vm, _name)).is_null();
    let mut module: *mut Module = newModule(vm);
    (*module).name = _name;
    (*module).initialized = 1 as libc::c_int != 0;
    vmPopTempRef(vm);
    initializeModule(vm, module, 0 as libc::c_int != 0);
    return module;
}
pub unsafe extern "C" fn moduleAddFunctionInternal(
    mut vm: *mut PKVM,
    mut module: *mut Module,
    mut name: *const libc::c_char,
    mut fptr: pkNativeFn,
    mut arity: libc::c_int,
    mut docstring: *const libc::c_char,
) {
    let mut fn_0: *mut Function = newFunction(
        vm,
        name,
        strlen(name) as libc::c_int,
        module,
        1 as libc::c_int != 0,
        docstring,
        0 as *mut libc::c_int,
    );
    (*fn_0).c2rust_unnamed.native = fptr;
    (*fn_0).arity = arity;
    vmPushTempRef(vm, &mut (*fn_0)._super);
    let mut closure: *mut Closure = newClosure(vm, fn_0);
    moduleSetGlobal(
        vm,
        module,
        name,
        strlen(name) as uint32_t,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*closure)._super as *mut Object as uintptr_t,
    );
    vmPopTempRef(vm);
}
unsafe extern "C" fn stdLangGC(mut vm: *mut PKVM) {
    let mut bytes_before: size_t = (*vm).bytes_allocated;
    vmCollectGarbage(vm);
    let mut garbage: size_t = bytes_before.wrapping_sub((*vm).bytes_allocated);
    *(*(*vm).fiber).ret = doubleToVar(garbage as libc::c_double);
}
static mut _pk_doc_stdLangGC: *const libc::c_char = b"lang.gc() -> Number\n\nTrigger garbage collection and return the amount of bytes cleaned.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc_stdLangDisas: *const libc::c_char = b"lang.disas(fn:Closure) -> String\n\nReturns the disassembled opcode of the function [fn].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdLangDisas(mut vm: *mut PKVM) {
    let mut closure: *mut Closure = 0 as *mut Closure;
    if !validateArgClosure(vm, 1 as libc::c_int, &mut closure) {
        return;
    }
    if !validateCond(
        vm,
        !(*(*closure).fn_0).is_native,
        b"Cannot disassemble native functions.\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    dumpFunctionCode(vm, (*closure).fn_0);
}
static mut _pk_doc_stdLangBackTrace: *const libc::c_char = b"lang.backtrace() -> String\n\nReturns the backtrace as a string, each line is formated as '<function>;<file>;<line>\n'.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdLangBackTrace(mut vm: *mut PKVM) {
    let mut bb: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut bb);
    let mut fiber: *mut Fiber = (*vm).fiber;
    while !fiber.is_null() {
        let mut i: libc::c_int = (*fiber).frame_count - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let mut frame: *mut CallFrame = &mut *((*fiber).frames).offset(i as isize)
                as *mut CallFrame;
            let mut fn_0: *const Function = (*(*frame).closure).fn_0;
            let mut instruction_index: libc::c_int = ((*frame).ip)
                .offset_from((*(*fn_0).c2rust_unnamed.fn_0).opcodes.data) as libc::c_long
                as libc::c_int - 1 as libc::c_int;
            if instruction_index == -(1 as libc::c_int) {
                instruction_index = 0 as libc::c_int;
            }
            let mut line: libc::c_int = *((*(*fn_0).c2rust_unnamed.fn_0).oplines.data)
                .offset(instruction_index as isize) as libc::c_int;
            let mut path: *const libc::c_char = if !((*(*fn_0).owner).path).is_null() {
                ((*(*(*fn_0).owner).path).data).as_mut_ptr() as *const libc::c_char
            } else {
                b"<?>\0" as *const u8 as *const libc::c_char
            };
            let mut fn_name: *const libc::c_char = if !((*fn_0).name).is_null() {
                (*fn_0).name
            } else {
                b"<?>\0" as *const u8 as *const libc::c_char
            };
            pkByteBufferAddStringFmt(
                &mut bb as *mut pkByteBuffer,
                vm,
                b"%s;%s;%i\n\0" as *const u8 as *const libc::c_char,
                fn_name,
                path,
                line,
            );
            i -= 1;
            i;
        }
        if !((*fiber).caller).is_null() {
            fiber = (*fiber).caller;
        } else {
            fiber = (*fiber).native;
        }
    }
    let mut bt: *mut String_0 = newStringLength(
        vm,
        bb.data as *const libc::c_char,
        bb.count,
    );
    vmPushTempRef(vm, &mut (*bt)._super);
    pkByteBufferClear(&mut bb, vm);
    vmPopTempRef(vm);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*bt)._super as *mut Object as uintptr_t;
}
static mut _pk_doc_stdLangModules: *const libc::c_char = b"lang.modules() -> List\n\nReturns the list of all registered modules.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn stdLangModules(mut vm: *mut PKVM) {
    let mut list: *mut List = newList(vm, 8 as libc::c_int as uint32_t);
    vmPushTempRef(vm, &mut (*list)._super);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*vm).modules).capacity {
        if !((*((*(*vm).modules).entries).offset(i as isize)).key
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000000 as libc::c_long as uint64_t)
        {
            let mut entry: Var = (*((*(*vm).modules).entries).offset(i as isize)).value;
            let mut module: *mut Module = (entry
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Module;
            if !(*((*(*module).name).data).as_mut_ptr().offset(0 as libc::c_int as isize)
                as libc::c_int == '@' as i32)
            {
                pkVarBufferWrite(&mut (*list).elements, vm, entry);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    vmPopTempRef(vm);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*list)._super as *mut Object as uintptr_t;
}
unsafe extern "C" fn initializeCoreModules(mut vm: *mut PKVM) {
    let mut lang: *mut Module = newModuleInternal(
        vm,
        b"lang\0" as *const u8 as *const libc::c_char,
    );
    vmPushTempRef(vm, &mut (*lang)._super);
    vmRegisterModule(vm, lang, (*lang).name);
    vmPopTempRef(vm);
    moduleAddFunctionInternal(
        vm,
        lang,
        b"gc\0" as *const u8 as *const libc::c_char,
        Some(stdLangGC as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc_stdLangGC,
    );
    moduleAddFunctionInternal(
        vm,
        lang,
        b"disas\0" as *const u8 as *const libc::c_char,
        Some(stdLangDisas as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc_stdLangDisas,
    );
    moduleAddFunctionInternal(
        vm,
        lang,
        b"backtrace\0" as *const u8 as *const libc::c_char,
        Some(stdLangBackTrace as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc_stdLangBackTrace,
    );
    moduleAddFunctionInternal(
        vm,
        lang,
        b"modules\0" as *const u8 as *const libc::c_char,
        Some(stdLangModules as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc_stdLangModules,
    );
}
unsafe extern "C" fn _ctorNull(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0 as libc::c_int as uint64_t;
}
unsafe extern "C" fn _ctorBool(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = if toBool(*((*(*vm).fiber).ret).offset(1 as libc::c_int as isize))
        as libc::c_int != 0
    {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t
    };
}
unsafe extern "C" fn _ctorNumber(mut vm: *mut PKVM) {
    let mut value: libc::c_double = 0.;
    if isNumeric(*((*(*vm).fiber).ret).offset(1 as libc::c_int as isize), &mut value) {
        *(*(*vm).fiber).ret = doubleToVar(value);
        return;
    }
    if *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize)
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((*((*(*vm).fiber).ret).offset(1 as libc::c_int as isize)
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        let mut str: *mut String_0 = (*((*(*vm).fiber).ret)
            .offset(1 as libc::c_int as isize)
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0;
        let mut err: *const libc::c_char = utilToNumber(
            ((*str).data).as_mut_ptr(),
            &mut value,
        );
        if err.is_null() {
            *(*(*vm).fiber).ret = doubleToVar(value);
            return;
        }
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            err,
            if err.is_null() {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(err) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    (*(*vm).fiber)
        .error = newStringLength(
        vm,
        b"Argument must be numeric or string.\0" as *const u8 as *const libc::c_char,
        if (b"Argument must be numeric or string.\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(
                b"Argument must be numeric or string.\0" as *const u8
                    as *const libc::c_char,
            ) as uint32_t
        },
    );
}
unsafe extern "C" fn _ctorString(mut vm: *mut PKVM) {
    if !pkCheckArgcRange(
        vm,
        ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ) {
        return;
    }
    if ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long as libc::c_int
        - 1 as libc::c_int == 0 as libc::c_int
    {
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*(newStringLength
                as unsafe extern "C" fn(
                    *mut PKVM,
                    *const libc::c_char,
                    uint32_t,
                ) -> *mut String_0)(
                vm,
                0 as *const libc::c_char,
                0 as libc::c_int as uint32_t,
            ))
                ._super as *mut Object as uintptr_t;
        return;
    }
    let mut str: *mut String_0 = varToString(
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
        0 as libc::c_int != 0,
    );
    if str.is_null() {
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*str)._super as *mut Object as uintptr_t;
}
unsafe extern "C" fn _ctorList(mut vm: *mut PKVM) {
    let mut list: *mut List = newList(
        vm,
        (((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int) as uint32_t,
    );
    vmPushTempRef(vm, &mut (*list)._super);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i
        < ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int
    {
        pkVarBufferWrite(
            &mut (*list).elements,
            vm,
            *((*(*vm).fiber).ret).offset((i + 1 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
    vmPopTempRef(vm);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*list)._super as *mut Object as uintptr_t;
}
unsafe extern "C" fn _ctorMap(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newMap as unsafe extern "C" fn(*mut PKVM) -> *mut Map)(vm))._super
            as *mut Object as uintptr_t;
}
unsafe extern "C" fn _ctorRange(mut vm: *mut PKVM) {
    let mut from: libc::c_double = 0.;
    let mut to: libc::c_double = 0.;
    if !validateNumeric(
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
        &mut from,
        b"Argument 1\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    if !validateNumeric(
        vm,
        *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize),
        &mut to,
        b"Argument 2\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newRange
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_double,
                libc::c_double,
            ) -> *mut Range)(vm, from, to))
            ._super as *mut Object as uintptr_t;
}
unsafe extern "C" fn _ctorFiber(mut vm: *mut PKVM) {
    let mut closure: *mut Closure = 0 as *mut Closure;
    if !validateArgClosure(vm, 1 as libc::c_int, &mut closure) {
        return;
    }
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newFiber
            as unsafe extern "C" fn(*mut PKVM, *mut Closure) -> *mut Fiber)(vm, closure))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc__objTypeName: *const libc::c_char = b"Object.typename() -> String\n\nReturns the type name of the object.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _objTypeName(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                uint32_t,
            ) -> *mut String_0)(
            vm,
            (varTypeName
                as unsafe extern "C" fn(
                    Var,
                ) -> *const libc::c_char)((*(*vm).fiber).self_0),
            (if ((varTypeName
                as unsafe extern "C" fn(
                    Var,
                ) -> *const libc::c_char)((*(*vm).fiber).self_0))
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                (strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(
                    (varTypeName
                        as unsafe extern "C" fn(
                            Var,
                        ) -> *const libc::c_char)((*(*vm).fiber).self_0),
                ) as uint32_t
            }),
        ))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc__objRepr: *const libc::c_char = b"Object._repr() -> String\n\nReturns the repr string of the object.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _objRepr(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(toRepr
            as unsafe extern "C" fn(
                *mut PKVM,
                Var,
            ) -> *mut String_0)(vm, (*(*vm).fiber).self_0))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc__numberTimes: *const libc::c_char = b"Number.times(f:Closure)\n\nIterate the function [f] n times. Here n is the integral value of the number. If the number is not an integer the floor value will be taken.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _numberTimes(mut vm: *mut PKVM) {
    let mut n: libc::c_double = varToDouble((*(*vm).fiber).self_0);
    let mut closure: *mut Closure = 0 as *mut Closure;
    if !validateArgClosure(vm, 1 as libc::c_int, &mut closure) {
        return;
    }
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while (i as libc::c_double) < n {
        let mut _i: Var = doubleToVar(i as libc::c_double);
        let mut result: PkResult = vmCallFunction(
            vm,
            closure,
            1 as libc::c_int,
            &mut _i,
            0 as *mut Var,
        );
        if result as libc::c_uint != PK_RESULT_SUCCESS as libc::c_int as libc::c_uint {
            break;
        }
        i += 1;
        i;
    }
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0 as libc::c_int as uint64_t;
}
static mut _pk_doc__numberIsint: *const libc::c_char = b"Number.isint() -> Bool\n\nReturns true if the number is a whold number, otherwise false.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _numberIsint(mut vm: *mut PKVM) {
    let mut n: libc::c_double = varToDouble((*(*vm).fiber).self_0);
    *(*(*vm).fiber)
        .ret = if floor(n) == n {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t
    };
}
static mut _pk_doc__numberIsbyte: *const libc::c_char = b"Number.isbyte() -> bool\n\nReturns true if the number is an integer and is between 0x00 and 0xff.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _numberIsbyte(mut vm: *mut PKVM) {
    let mut n: libc::c_double = varToDouble((*(*vm).fiber).self_0);
    *(*(*vm).fiber)
        .ret = if floor(n) == n
        && (0 as libc::c_int as libc::c_double <= n
            && n <= 0xff as libc::c_int as libc::c_double)
    {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t
    };
}
static mut _pk_doc__stringFind: *const libc::c_char = b"String.find(sub:String[, start:Number=0]) -> Number\n\nReturns the first index of the substring [sub] found from the [start] index\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _stringFind(mut vm: *mut PKVM) {
    if !pkCheckArgcRange(
        vm,
        ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
    ) {
        return;
    }
    let mut sub: *mut String_0 = 0 as *mut String_0;
    if !validateArgString(vm, 1 as libc::c_int, &mut sub) {
        return;
    }
    let mut start: int64_t = 0 as libc::c_int as int64_t;
    if ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long as libc::c_int
        - 1 as libc::c_int == 2 as libc::c_int
    {
        if !validateInteger(
            vm,
            *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize),
            &mut start,
            b"Argument 1\0" as *const u8 as *const libc::c_char,
        ) {
            return;
        }
    }
    let mut self_0: *mut String_0 = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut String_0;
    if (*self_0).length as libc::c_long <= start {
        *(*(*vm).fiber).ret = doubleToVar(-(1 as libc::c_int) as libc::c_double);
        return;
    }
    let mut match_0: *const libc::c_char = strstr(
        ((*self_0).data).as_mut_ptr().offset(start as isize),
        ((*sub).data).as_mut_ptr(),
    );
    if match_0.is_null() {
        *(*(*vm).fiber).ret = doubleToVar(-(1 as libc::c_int) as libc::c_double);
        return;
    }
    *(*(*vm).fiber)
        .ret = doubleToVar(
        match_0.offset_from(((*self_0).data).as_mut_ptr()) as libc::c_long
            as libc::c_double,
    );
}
unsafe extern "C" fn _stringReplace(mut vm: *mut PKVM) {
    if !pkCheckArgcRange(
        vm,
        ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ) {
        return;
    }
    let mut old: *mut String_0 = 0 as *mut String_0;
    let mut new_: *mut String_0 = 0 as *mut String_0;
    if !validateArgString(vm, 1 as libc::c_int, &mut old) {
        return;
    }
    if !validateArgString(vm, 2 as libc::c_int, &mut new_) {
        return;
    }
    let mut self_0: *mut String_0 = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut String_0;
    let mut count: int64_t = -(1 as libc::c_int) as int64_t;
    if ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long as libc::c_int
        - 1 as libc::c_int == 3 as libc::c_int
    {
        if !validateInteger(
            vm,
            *((*(*vm).fiber).ret).offset(3 as libc::c_int as isize),
            &mut count,
            b"Argument 3\0" as *const u8 as *const libc::c_char,
        ) {
            return;
        }
        if count < 0 as libc::c_int as libc::c_long
            && count != -(1 as libc::c_int) as libc::c_long
        {
            (*(*vm).fiber)
                .error = newStringLength(
                vm,
                b"count should either be >= 0 or -1\0" as *const u8
                    as *const libc::c_char,
                if (b"count should either be >= 0 or -1\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(
                        b"count should either be >= 0 or -1\0" as *const u8
                            as *const libc::c_char,
                    ) as uint32_t
                },
            );
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            return;
        }
    }
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(stringReplace
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut String_0,
                *mut String_0,
                *mut String_0,
                libc::c_int,
            ) -> *mut String_0)(vm, self_0, old, new_, count as int32_t))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc__stringReplace: *const libc::c_char = b"String.replace(old:Sttring, new:String[, count:Number=-1]) -> String\n\nReturns a copy of the string where [count] occurrence of the substring [old] will be replaced with [new]. If [count] == -1 all the occurrence will be replaced.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _stringSplit(mut vm: *mut PKVM) {
    let mut sep: *mut String_0 = 0 as *mut String_0;
    if !validateArgString(vm, 1 as libc::c_int, &mut sep) {
        return;
    }
    if (*sep).length == 0 as libc::c_int as libc::c_uint {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Cannot use empty string as a seperator.\0" as *const u8
                as *const libc::c_char,
            if (b"Cannot use empty string as a seperator.\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Cannot use empty string as a seperator.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(stringSplit
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut String_0,
                *mut String_0,
            ) -> *mut List)(
            vm,
            ((*(*vm).fiber).self_0 & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut String_0,
            sep,
        ))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc__stringSplit: *const libc::c_char = b"String.split(sep:String) -> List\n\nSplit the string into a list of string seperated by [sep] delimeter.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _stringStrip(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(stringStrip
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut String_0,
            ) -> *mut String_0)(
            vm,
            ((*(*vm).fiber).self_0 & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut String_0,
        ))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc__stringStrip: *const libc::c_char = b"String.strip() -> String\n\nReturns a copy of the string where the leading and trailing whitespace removed.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__stringLower: *const libc::c_char = b"String.lower() -> String\n\nReturns a copy of the string where all the characters are converted to lower case letters.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _stringLower(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(stringLower
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut String_0,
            ) -> *mut String_0)(
            vm,
            ((*(*vm).fiber).self_0 & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut String_0,
        ))
            ._super as *mut Object as uintptr_t;
}
unsafe extern "C" fn _stringUpper(mut vm: *mut PKVM) {
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(stringUpper
            as unsafe extern "C" fn(
                *mut PKVM,
                *mut String_0,
            ) -> *mut String_0)(
            vm,
            ((*(*vm).fiber).self_0 & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut String_0,
        ))
            ._super as *mut Object as uintptr_t;
}
static mut _pk_doc__stringUpper: *const libc::c_char = b"String.lower() -> String\n\nReturns a copy of the string where all the characters are converted to upper case letters.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _stingStartswith(mut vm: *mut PKVM) {
    let mut prefix: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    let mut self_0: *mut String_0 = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut String_0;
    if prefix
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((prefix & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        let mut pre: *mut String_0 = (prefix
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0;
        if (*pre).length > (*self_0).length {
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t;
            return;
        }
        *(*(*vm).fiber)
            .ret = if strncmp(
            ((*self_0).data).as_mut_ptr(),
            ((*pre).data).as_mut_ptr(),
            (*pre).length as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000003 as libc::c_long as uint64_t
        } else {
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t
        };
        return;
    } else if prefix
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((prefix & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_LIST as libc::c_int as libc::c_uint
    {
        let mut prefixes: *mut List = (prefix
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*prefixes).elements.count {
            let mut pre_var: Var = *((*prefixes).elements.data).offset(i as isize);
            if !(pre_var
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && (*((pre_var & 0xffffffffffff as libc::c_long as uint64_t)
                    as *mut Object))
                    .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint)
            {
                (*(*vm).fiber)
                    .error = newStringLength(
                    vm,
                    b"Expected a String for prefix.\0" as *const u8
                        as *const libc::c_char,
                    if (b"Expected a String for prefix.\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                    {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        strlen(
                            b"Expected a String for prefix.\0" as *const u8
                                as *const libc::c_char,
                        ) as uint32_t
                    },
                );
                *(*(*vm).fiber)
                    .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0 as libc::c_int as uint64_t;
                return;
            }
            let mut pre_0: *mut String_0 = (pre_var
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut String_0;
            if (*pre_0).length > (*self_0).length {
                *(*(*vm).fiber)
                    .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000002 as libc::c_long as uint64_t;
                return;
            }
            if strncmp(
                ((*self_0).data).as_mut_ptr(),
                ((*pre_0).data).as_mut_ptr(),
                (*pre_0).length as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                *(*(*vm).fiber)
                    .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000003 as libc::c_long as uint64_t;
                return;
            }
            i = i.wrapping_add(1);
            i;
        }
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t;
        return;
    } else {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Expected a String or a List of prifiexes.\0" as *const u8
                as *const libc::c_char,
            if (b"Expected a String or a List of prifiexes.\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Expected a String or a List of prifiexes.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    };
}
static mut _pk_doc__stingStartswith: *const libc::c_char = b"String.startswith(prefix: String | List) -> Bool\n\nReturns true if the string starts the specified prefix.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__stingEndswith: *const libc::c_char = b"String.endswith(suffix: String | List) -> Bool\n\nReturns true if the string ends with the specified suffix.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _stingEndswith(mut vm: *mut PKVM) {
    let mut suffix: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    let mut self_0: *mut String_0 = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut String_0;
    if suffix
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((suffix & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        let mut suf: *mut String_0 = (suffix
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0;
        if (*suf).length > (*self_0).length {
            *(*(*vm).fiber)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t;
            return;
        }
        let mut start: *const libc::c_char = ((*self_0).data)
            .as_mut_ptr()
            .offset(((*self_0).length).wrapping_sub((*suf).length) as isize);
        *(*(*vm).fiber)
            .ret = if strncmp(
            start,
            ((*suf).data).as_mut_ptr(),
            (*suf).length as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000003 as libc::c_long as uint64_t
        } else {
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t
        };
        return;
    } else if suffix
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((suffix & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_LIST as libc::c_int as libc::c_uint
    {
        let mut suffixes: *mut List = (suffix
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*suffixes).elements.count {
            let mut suff_var: Var = *((*suffixes).elements.data).offset(i as isize);
            if !(suff_var
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && (*((suff_var & 0xffffffffffff as libc::c_long as uint64_t)
                    as *mut Object))
                    .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint)
            {
                (*(*vm).fiber)
                    .error = newStringLength(
                    vm,
                    b"Expected a String for suffix.\0" as *const u8
                        as *const libc::c_char,
                    if (b"Expected a String for suffix.\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                    {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        strlen(
                            b"Expected a String for suffix.\0" as *const u8
                                as *const libc::c_char,
                        ) as uint32_t
                    },
                );
                *(*(*vm).fiber)
                    .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0 as libc::c_int as uint64_t;
                return;
            }
            let mut suf_0: *mut String_0 = (suff_var
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut String_0;
            if (*suf_0).length > (*self_0).length {
                *(*(*vm).fiber)
                    .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000002 as libc::c_long as uint64_t;
                return;
            }
            let mut start_0: *const libc::c_char = ((*self_0).data)
                .as_mut_ptr()
                .offset(((*self_0).length).wrapping_sub((*suf_0).length) as isize);
            if strncmp(
                start_0,
                ((*suf_0).data).as_mut_ptr(),
                (*suf_0).length as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                *(*(*vm).fiber)
                    .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000003 as libc::c_long as uint64_t;
                return;
            }
            i = i.wrapping_add(1);
            i;
        }
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t;
        return;
    } else {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Expected a String or a List of suffixes.\0" as *const u8
                as *const libc::c_char,
            if (b"Expected a String or a List of suffixes.\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Expected a String or a List of suffixes.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    };
}
unsafe extern "C" fn _listAppend(mut vm: *mut PKVM) {
    pkVarBufferWrite(
        &mut (*(((*(*vm).fiber).self_0 & 0xffffffffffff as libc::c_long as uint64_t)
            as *mut Object as *mut List))
            .elements,
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
    );
    *(*(*vm).fiber).ret = (*(*vm).fiber).self_0;
}
static mut _pk_doc__listAppend: *const libc::c_char = b"List.append(value:Var) -> List\n\nAppend the [value] to the list and return the List.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _listInsert(mut vm: *mut PKVM) {
    let mut self_0: *mut List = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
    let mut index: int64_t = 0;
    if !validateInteger(
        vm,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
        &mut index,
        b"Argument 1\0" as *const u8 as *const libc::c_char,
    ) {
        return;
    }
    if index < 0 as libc::c_int as libc::c_long
        || index > (*self_0).elements.count as libc::c_long
    {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"List.insert index out of bounds.\0" as *const u8 as *const libc::c_char,
            if (b"List.insert index out of bounds.\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"List.insert index out of bounds.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    listInsert(
        vm,
        self_0,
        index as uint32_t,
        *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize),
    );
}
static mut _pk_doc__listInsert: *const libc::c_char = b"List.insert(index:Number, value:Var) -> Null\n\nInsert the element at the given index. The index should be 0 <= index <= list.length.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _listPop(mut vm: *mut PKVM) {
    let mut self_0: *mut List = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
    if !pkCheckArgcRange(
        vm,
        ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ) {
        return;
    }
    if (*self_0).elements.count == 0 as libc::c_int as libc::c_uint {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Cannot pop from an empty list.\0" as *const u8 as *const libc::c_char,
            if (b"Cannot pop from an empty list.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Cannot pop from an empty list.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    let mut index: int64_t = -(1 as libc::c_int) as int64_t;
    if ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long as libc::c_int
        - 1 as libc::c_int == 1 as libc::c_int
    {
        if !validateInteger(
            vm,
            *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
            &mut index,
            b"Argument 1\0" as *const u8 as *const libc::c_char,
        ) {
            return;
        }
    }
    if index < 0 as libc::c_int as libc::c_long {
        index = (*self_0).elements.count as libc::c_long + index;
    }
    if index < 0 as libc::c_int as libc::c_long
        || index >= (*self_0).elements.count as libc::c_long
    {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"List.pop index out of bounds.\0" as *const u8 as *const libc::c_char,
            if (b"List.pop index out of bounds.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"List.pop index out of bounds.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    *(*(*vm).fiber).ret = listRemoveAt(vm, self_0, index as uint32_t);
}
static mut _pk_doc__listPop: *const libc::c_char = b"List.pop(index:Number=-1) -> Var\n\nRemoves the last element of the list and return it.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__listFind: *const libc::c_char = b"List.find(value:Var) -> Number\n\nFind the value and return its index. If the vlaue not exists it'll return -1.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _listFind(mut vm: *mut PKVM) {
    let mut self_0: *mut List = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
    let mut it: *mut Var = (*self_0).elements.data;
    if it.is_null() {
        *(*(*vm).fiber).ret = doubleToVar(-(1 as libc::c_int) as libc::c_double);
        return;
    }
    while it < ((*self_0).elements.data).offset((*self_0).elements.count as isize) {
        if isValuesEqual(*it, *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize)) {
            *(*(*vm).fiber)
                .ret = doubleToVar(
                it.offset_from((*self_0).elements.data) as libc::c_long as libc::c_double,
            );
            return;
        }
        it = it.offset(1);
        it;
    }
    *(*(*vm).fiber).ret = doubleToVar(-(1 as libc::c_int) as libc::c_double);
}
static mut _pk_doc__listClear: *const libc::c_char = b"List.clear() -> Null\n\nRemoves all the entries in the list.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _listClear(mut vm: *mut PKVM) {
    listClear(
        vm,
        ((*(*vm).fiber).self_0 & 0xffffffffffff as libc::c_long as uint64_t)
            as *mut Object as *mut List,
    );
}
unsafe extern "C" fn _mapClear(mut vm: *mut PKVM) {
    let mut self_0: *mut Map = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Map;
    mapClear(vm, self_0);
}
static mut _pk_doc__mapClear: *const libc::c_char = b"Map.clear() -> Null\n\nRemoves all the entries in the map.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__mapGet: *const libc::c_char = b"Map.get(key:Var, default=Null) -> Var\n\nReturns the key if its in the map, otherwise the default value will be returned.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _mapGet(mut vm: *mut PKVM) {
    if !pkCheckArgcRange(
        vm,
        ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
    ) {
        return;
    }
    let mut default_: Var = if ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret)
        as libc::c_long as libc::c_int - 1 as libc::c_int == 1 as libc::c_int
    {
        0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t
    } else {
        *((*(*vm).fiber).ret).offset(2 as libc::c_int as isize)
    };
    let mut self_0: *mut Map = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Map;
    let mut value: Var = mapGet(
        self_0,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
    );
    if value
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t
    {
        *(*(*vm).fiber).ret = default_;
        return;
    }
    *(*(*vm).fiber).ret = value;
}
static mut _pk_doc__mapHas: *const libc::c_char = b"Map.has(key:Var) -> Bool\n\nReturns true if the key exists.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _mapHas(mut vm: *mut PKVM) {
    let mut self_0: *mut Map = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Map;
    let mut value: Var = mapGet(
        self_0,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
    );
    *(*(*vm).fiber)
        .ret = if !(value
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t)
    {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t
    };
}
static mut _pk_doc__mapPop: *const libc::c_char = b"Map.pop(key:Var) -> Var\n\nPops the value at the key and return it.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _mapPop(mut vm: *mut PKVM) {
    let mut self_0: *mut Map = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Map;
    let mut value: Var = mapRemoveKey(
        vm,
        self_0,
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
    );
    if value
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t
    {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Key '@' does not exists.\0" as *const u8 as *const libc::c_char,
            toRepr(vm, *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize)),
        );
        *(*(*vm).fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
        return;
    }
    *(*(*vm).fiber).ret = value;
}
static mut _pk_doc__methodBindBind: *const libc::c_char = b"MethodBind.bind(instance:Var) -> MethodBind\n\nBind the method to the instance and the method bind will be returned. The method should be a valid method of the instance. ie. the instance's interitance tree should contain the method.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _methodBindBind(mut vm: *mut PKVM) {
    let mut self_0: *mut MethodBind = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut MethodBind;
    let mut method_name: *mut String_0 = newStringLength(
        vm,
        (*(*(*self_0).method).fn_0).name,
        if ((*(*(*self_0).method).fn_0).name).is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen((*(*(*self_0).method).fn_0).name) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*method_name)._super);
    let mut instance: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    let mut method: *mut Closure = 0 as *mut Closure;
    if !hasMethod(vm, instance, method_name, &mut method) || method != (*self_0).method {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Cannot bind method, instance and method types miss-match.\0" as *const u8
                as *const libc::c_char,
            if (b"Cannot bind method, instance and method types miss-match.\0"
                as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Cannot bind method, instance and method types miss-match.\0"
                        as *const u8 as *const libc::c_char,
                ) as uint32_t
            },
        );
        return;
    }
    (*self_0).instance = instance;
    vmPopTempRef(vm);
    *(*(*vm).fiber).ret = (*(*vm).fiber).self_0;
}
static mut _pk_doc__classMethods: *const libc::c_char = b"Class.methods() -> List\n\nReturns a list of unbound MethodBind of the class.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _classMethods(mut vm: *mut PKVM) {
    let mut self_0: *mut Class = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Class;
    let mut list: *mut List = newList(vm, (*self_0).methods.count);
    vmPushTempRef(vm, &mut (*list)._super);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*self_0).methods.count as libc::c_int {
        let mut method: *mut Closure = *((*self_0).methods.data).offset(i as isize);
        if !(*((*(*method).fn_0).name).offset(0 as libc::c_int as isize) as libc::c_int
            == '@' as i32)
        {
            let mut mb: *mut MethodBind = newMethodBind(vm, method);
            vmPushTempRef(vm, &mut (*mb)._super);
            pkVarBufferWrite(
                &mut (*list).elements,
                vm,
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*mb)._super as *mut Object as uintptr_t,
            );
            vmPopTempRef(vm);
        }
        i += 1;
        i;
    }
    vmPopTempRef(vm);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*list)._super as *mut Object as uintptr_t;
}
static mut _pk_doc__moduleGlobals: *const libc::c_char = b"Module.globals() -> List\n\nReturns a list of all the globals in the module. Since classes and functinos are also globals to a module it'll contain them too.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _moduleGlobals(mut vm: *mut PKVM) {
    let mut self_0: *mut Module = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Module;
    let mut list: *mut List = newList(vm, (*self_0).globals.count);
    vmPushTempRef(vm, &mut (*list)._super);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*self_0).globals.count as libc::c_int {
        if !(*((*moduleGetStringAt(
            self_0,
            *((*self_0).global_names.data).offset(i as isize) as libc::c_int,
        ))
            .data)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as libc::c_int == '@' as i32)
        {
            pkVarBufferWrite(
                &mut (*list).elements,
                vm,
                *((*self_0).globals.data).offset(i as isize),
            );
        }
        i += 1;
        i;
    }
    vmPopTempRef(vm);
    *(*(*vm).fiber)
        .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*list)._super as *mut Object as uintptr_t;
}
static mut _pk_doc__fiberRun: *const libc::c_char = b"Fiber.run(...) -> Var\n\nRuns the fiber's function with the provided arguments and returns it's return value or the yielded value if it's yielded.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fiberRun(mut vm: *mut PKVM) {
    let mut self_0: *mut Fiber = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Fiber;
    if vmPrepareFiber(
        vm,
        self_0,
        ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int,
        &mut *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize),
    ) {
        (*self_0).caller = (*vm).fiber;
        (*vm).fiber = self_0;
        (*self_0).state = FIBER_RUNNING;
    }
}
static mut _pk_doc__fiberResume: *const libc::c_char = b"Fiber.resume() -> Var\n\nResumes a yielded function from a previous call of fiber_run() function. Return it's return value or the yielded value if it's yielded.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fiberResume(mut vm: *mut PKVM) {
    let mut self_0: *mut Fiber = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Fiber;
    if !pkCheckArgcRange(
        vm,
        ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
            as libc::c_int - 1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ) {
        return;
    }
    let mut value: Var = if ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret)
        as libc::c_long as libc::c_int - 1 as libc::c_int == 1 as libc::c_int
    {
        *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize)
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t
    };
    if vmSwitchFiber(vm, self_0, &mut value) {
        (*self_0).state = FIBER_RUNNING;
    }
}
unsafe extern "C" fn initializePrimitiveClasses(mut vm: *mut PKVM) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < PK_INSTANCE as libc::c_int {
        let mut super_0: *mut Class = 0 as *mut Class;
        if i != 0 as libc::c_int {
            super_0 = (*vm).builtin_classes[PK_OBJECT as libc::c_int as usize];
        }
        let mut name: *const libc::c_char = getPkVarTypeName(i as PkVarType);
        let mut cls: *mut Class = newClass(
            vm,
            name,
            strlen(name) as libc::c_int,
            super_0,
            0 as *mut Module,
            0 as *const libc::c_char,
            0 as *mut libc::c_int,
        );
        (*vm).builtin_classes[i as usize] = cls;
        (*cls).class_of = i as PkVarType;
        i += 1;
        i;
    }
    let mut fn_0: *mut Function = newFunction(
        vm,
        b"@ctorNull\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorNull\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_0)
        .c2rust_unnamed
        .native = Some(_ctorNull as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_0).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_0)._super);
    (*(*vm).builtin_classes[PK_NULL as libc::c_int as usize])
        .ctor = newClosure(vm, fn_0);
    vmPopTempRef(vm);
    let mut fn_1: *mut Function = newFunction(
        vm,
        b"@ctorBool\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorBool\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_1)
        .c2rust_unnamed
        .native = Some(_ctorBool as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_1).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_1)._super);
    (*(*vm).builtin_classes[PK_BOOL as libc::c_int as usize])
        .ctor = newClosure(vm, fn_1);
    vmPopTempRef(vm);
    let mut fn_2: *mut Function = newFunction(
        vm,
        b"@ctorNumber\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorNumber\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_2)
        .c2rust_unnamed
        .native = Some(_ctorNumber as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_2).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_2)._super);
    (*(*vm).builtin_classes[PK_NUMBER as libc::c_int as usize])
        .ctor = newClosure(vm, fn_2);
    vmPopTempRef(vm);
    let mut fn_3: *mut Function = newFunction(
        vm,
        b"@ctorString\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorString\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_3)
        .c2rust_unnamed
        .native = Some(_ctorString as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_3).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_3)._super);
    (*(*vm).builtin_classes[PK_STRING as libc::c_int as usize])
        .ctor = newClosure(vm, fn_3);
    vmPopTempRef(vm);
    let mut fn_4: *mut Function = newFunction(
        vm,
        b"@ctorRange\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorRange\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_4)
        .c2rust_unnamed
        .native = Some(_ctorRange as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_4).arity = 2 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_4)._super);
    (*(*vm).builtin_classes[PK_RANGE as libc::c_int as usize])
        .ctor = newClosure(vm, fn_4);
    vmPopTempRef(vm);
    let mut fn_5: *mut Function = newFunction(
        vm,
        b"@ctorList\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorList\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_5)
        .c2rust_unnamed
        .native = Some(_ctorList as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_5).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_5)._super);
    (*(*vm).builtin_classes[PK_LIST as libc::c_int as usize])
        .ctor = newClosure(vm, fn_5);
    vmPopTempRef(vm);
    let mut fn_6: *mut Function = newFunction(
        vm,
        b"@ctorMap\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorMap\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_6)
        .c2rust_unnamed
        .native = Some(_ctorMap as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_6).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_6)._super);
    (*(*vm).builtin_classes[PK_MAP as libc::c_int as usize]).ctor = newClosure(vm, fn_6);
    vmPopTempRef(vm);
    let mut fn_7: *mut Function = newFunction(
        vm,
        b"@ctorFiber\0" as *const u8 as *const libc::c_char,
        strlen(b"@ctorFiber\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    (*fn_7)
        .c2rust_unnamed
        .native = Some(_ctorFiber as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_7).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_7)._super);
    (*(*vm).builtin_classes[PK_FIBER as libc::c_int as usize])
        .ctor = newClosure(vm, fn_7);
    vmPopTempRef(vm);
    let mut fn_8: *mut Function = newFunction(
        vm,
        b"typename\0" as *const u8 as *const libc::c_char,
        strlen(b"typename\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__objTypeName,
        0 as *mut libc::c_int,
    );
    (*fn_8).is_method = 1 as libc::c_int != 0;
    (*fn_8)
        .c2rust_unnamed
        .native = Some(_objTypeName as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_8).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_8)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_OBJECT as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_8),
    );
    vmPopTempRef(vm);
    let mut fn_9: *mut Function = newFunction(
        vm,
        b"_repr\0" as *const u8 as *const libc::c_char,
        strlen(b"_repr\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__objRepr,
        0 as *mut libc::c_int,
    );
    (*fn_9).is_method = 1 as libc::c_int != 0;
    (*fn_9)
        .c2rust_unnamed
        .native = Some(_objRepr as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_9).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_9)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_OBJECT as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_9),
    );
    vmPopTempRef(vm);
    let mut fn_10: *mut Function = newFunction(
        vm,
        b"times\0" as *const u8 as *const libc::c_char,
        strlen(b"times\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__numberTimes,
        0 as *mut libc::c_int,
    );
    (*fn_10).is_method = 1 as libc::c_int != 0;
    (*fn_10)
        .c2rust_unnamed
        .native = Some(_numberTimes as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_10).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_10)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_NUMBER as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_10),
    );
    vmPopTempRef(vm);
    let mut fn_11: *mut Function = newFunction(
        vm,
        b"isint\0" as *const u8 as *const libc::c_char,
        strlen(b"isint\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__numberIsint,
        0 as *mut libc::c_int,
    );
    (*fn_11).is_method = 1 as libc::c_int != 0;
    (*fn_11)
        .c2rust_unnamed
        .native = Some(_numberIsint as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_11).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_11)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_NUMBER as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_11),
    );
    vmPopTempRef(vm);
    let mut fn_12: *mut Function = newFunction(
        vm,
        b"isbyte\0" as *const u8 as *const libc::c_char,
        strlen(b"isbyte\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__numberIsbyte,
        0 as *mut libc::c_int,
    );
    (*fn_12).is_method = 1 as libc::c_int != 0;
    (*fn_12)
        .c2rust_unnamed
        .native = Some(_numberIsbyte as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_12).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_12)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_NUMBER as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_12),
    );
    vmPopTempRef(vm);
    let mut fn_13: *mut Function = newFunction(
        vm,
        b"strip\0" as *const u8 as *const libc::c_char,
        strlen(b"strip\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stringStrip,
        0 as *mut libc::c_int,
    );
    (*fn_13).is_method = 1 as libc::c_int != 0;
    (*fn_13)
        .c2rust_unnamed
        .native = Some(_stringStrip as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_13).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_13)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_13),
    );
    vmPopTempRef(vm);
    let mut fn_14: *mut Function = newFunction(
        vm,
        b"lower\0" as *const u8 as *const libc::c_char,
        strlen(b"lower\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stringLower,
        0 as *mut libc::c_int,
    );
    (*fn_14).is_method = 1 as libc::c_int != 0;
    (*fn_14)
        .c2rust_unnamed
        .native = Some(_stringLower as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_14).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_14)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_14),
    );
    vmPopTempRef(vm);
    let mut fn_15: *mut Function = newFunction(
        vm,
        b"upper\0" as *const u8 as *const libc::c_char,
        strlen(b"upper\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stringUpper,
        0 as *mut libc::c_int,
    );
    (*fn_15).is_method = 1 as libc::c_int != 0;
    (*fn_15)
        .c2rust_unnamed
        .native = Some(_stringUpper as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_15).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_15)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_15),
    );
    vmPopTempRef(vm);
    let mut fn_16: *mut Function = newFunction(
        vm,
        b"find\0" as *const u8 as *const libc::c_char,
        strlen(b"find\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stringFind,
        0 as *mut libc::c_int,
    );
    (*fn_16).is_method = 1 as libc::c_int != 0;
    (*fn_16)
        .c2rust_unnamed
        .native = Some(_stringFind as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_16).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_16)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_16),
    );
    vmPopTempRef(vm);
    let mut fn_17: *mut Function = newFunction(
        vm,
        b"replace\0" as *const u8 as *const libc::c_char,
        strlen(b"replace\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stringReplace,
        0 as *mut libc::c_int,
    );
    (*fn_17).is_method = 1 as libc::c_int != 0;
    (*fn_17)
        .c2rust_unnamed
        .native = Some(_stringReplace as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_17).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_17)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_17),
    );
    vmPopTempRef(vm);
    let mut fn_18: *mut Function = newFunction(
        vm,
        b"split\0" as *const u8 as *const libc::c_char,
        strlen(b"split\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stringSplit,
        0 as *mut libc::c_int,
    );
    (*fn_18).is_method = 1 as libc::c_int != 0;
    (*fn_18)
        .c2rust_unnamed
        .native = Some(_stringSplit as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_18).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_18)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_18),
    );
    vmPopTempRef(vm);
    let mut fn_19: *mut Function = newFunction(
        vm,
        b"startswith\0" as *const u8 as *const libc::c_char,
        strlen(b"startswith\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stingStartswith,
        0 as *mut libc::c_int,
    );
    (*fn_19).is_method = 1 as libc::c_int != 0;
    (*fn_19)
        .c2rust_unnamed
        .native = Some(_stingStartswith as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_19).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_19)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_19),
    );
    vmPopTempRef(vm);
    let mut fn_20: *mut Function = newFunction(
        vm,
        b"endswith\0" as *const u8 as *const libc::c_char,
        strlen(b"endswith\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__stingEndswith,
        0 as *mut libc::c_int,
    );
    (*fn_20).is_method = 1 as libc::c_int != 0;
    (*fn_20)
        .c2rust_unnamed
        .native = Some(_stingEndswith as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_20).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_20)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_STRING as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_20),
    );
    vmPopTempRef(vm);
    let mut fn_21: *mut Function = newFunction(
        vm,
        b"clear\0" as *const u8 as *const libc::c_char,
        strlen(b"clear\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__listClear,
        0 as *mut libc::c_int,
    );
    (*fn_21).is_method = 1 as libc::c_int != 0;
    (*fn_21)
        .c2rust_unnamed
        .native = Some(_listClear as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_21).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_21)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_LIST as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_21),
    );
    vmPopTempRef(vm);
    let mut fn_22: *mut Function = newFunction(
        vm,
        b"find\0" as *const u8 as *const libc::c_char,
        strlen(b"find\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__listFind,
        0 as *mut libc::c_int,
    );
    (*fn_22).is_method = 1 as libc::c_int != 0;
    (*fn_22)
        .c2rust_unnamed
        .native = Some(_listFind as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_22).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_22)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_LIST as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_22),
    );
    vmPopTempRef(vm);
    let mut fn_23: *mut Function = newFunction(
        vm,
        b"append\0" as *const u8 as *const libc::c_char,
        strlen(b"append\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__listAppend,
        0 as *mut libc::c_int,
    );
    (*fn_23).is_method = 1 as libc::c_int != 0;
    (*fn_23)
        .c2rust_unnamed
        .native = Some(_listAppend as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_23).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_23)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_LIST as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_23),
    );
    vmPopTempRef(vm);
    let mut fn_24: *mut Function = newFunction(
        vm,
        b"pop\0" as *const u8 as *const libc::c_char,
        strlen(b"pop\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__listPop,
        0 as *mut libc::c_int,
    );
    (*fn_24).is_method = 1 as libc::c_int != 0;
    (*fn_24)
        .c2rust_unnamed
        .native = Some(_listPop as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_24).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_24)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_LIST as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_24),
    );
    vmPopTempRef(vm);
    let mut fn_25: *mut Function = newFunction(
        vm,
        b"insert\0" as *const u8 as *const libc::c_char,
        strlen(b"insert\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__listInsert,
        0 as *mut libc::c_int,
    );
    (*fn_25).is_method = 1 as libc::c_int != 0;
    (*fn_25)
        .c2rust_unnamed
        .native = Some(_listInsert as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_25).arity = 2 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_25)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_LIST as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_25),
    );
    vmPopTempRef(vm);
    let mut fn_26: *mut Function = newFunction(
        vm,
        b"clear\0" as *const u8 as *const libc::c_char,
        strlen(b"clear\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__mapClear,
        0 as *mut libc::c_int,
    );
    (*fn_26).is_method = 1 as libc::c_int != 0;
    (*fn_26)
        .c2rust_unnamed
        .native = Some(_mapClear as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_26).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_26)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_MAP as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_26),
    );
    vmPopTempRef(vm);
    let mut fn_27: *mut Function = newFunction(
        vm,
        b"get\0" as *const u8 as *const libc::c_char,
        strlen(b"get\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__mapGet,
        0 as *mut libc::c_int,
    );
    (*fn_27).is_method = 1 as libc::c_int != 0;
    (*fn_27)
        .c2rust_unnamed
        .native = Some(_mapGet as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_27).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_27)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_MAP as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_27),
    );
    vmPopTempRef(vm);
    let mut fn_28: *mut Function = newFunction(
        vm,
        b"has\0" as *const u8 as *const libc::c_char,
        strlen(b"has\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__mapHas,
        0 as *mut libc::c_int,
    );
    (*fn_28).is_method = 1 as libc::c_int != 0;
    (*fn_28)
        .c2rust_unnamed
        .native = Some(_mapHas as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_28).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_28)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_MAP as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_28),
    );
    vmPopTempRef(vm);
    let mut fn_29: *mut Function = newFunction(
        vm,
        b"pop\0" as *const u8 as *const libc::c_char,
        strlen(b"pop\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__mapPop,
        0 as *mut libc::c_int,
    );
    (*fn_29).is_method = 1 as libc::c_int != 0;
    (*fn_29)
        .c2rust_unnamed
        .native = Some(_mapPop as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_29).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_29)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_MAP as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_29),
    );
    vmPopTempRef(vm);
    let mut fn_30: *mut Function = newFunction(
        vm,
        b"bind\0" as *const u8 as *const libc::c_char,
        strlen(b"bind\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__methodBindBind,
        0 as *mut libc::c_int,
    );
    (*fn_30).is_method = 1 as libc::c_int != 0;
    (*fn_30)
        .c2rust_unnamed
        .native = Some(_methodBindBind as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_30).arity = 1 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_30)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_METHOD_BIND as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_30),
    );
    vmPopTempRef(vm);
    let mut fn_31: *mut Function = newFunction(
        vm,
        b"methods\0" as *const u8 as *const libc::c_char,
        strlen(b"methods\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__classMethods,
        0 as *mut libc::c_int,
    );
    (*fn_31).is_method = 1 as libc::c_int != 0;
    (*fn_31)
        .c2rust_unnamed
        .native = Some(_classMethods as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_31).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_31)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_CLASS as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_31),
    );
    vmPopTempRef(vm);
    let mut fn_32: *mut Function = newFunction(
        vm,
        b"globals\0" as *const u8 as *const libc::c_char,
        strlen(b"globals\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__moduleGlobals,
        0 as *mut libc::c_int,
    );
    (*fn_32).is_method = 1 as libc::c_int != 0;
    (*fn_32)
        .c2rust_unnamed
        .native = Some(_moduleGlobals as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_32).arity = 0 as libc::c_int;
    vmPushTempRef(vm, &mut (*fn_32)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_MODULE as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_32),
    );
    vmPopTempRef(vm);
    let mut fn_33: *mut Function = newFunction(
        vm,
        b"run\0" as *const u8 as *const libc::c_char,
        strlen(b"run\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__fiberRun,
        0 as *mut libc::c_int,
    );
    (*fn_33).is_method = 1 as libc::c_int != 0;
    (*fn_33)
        .c2rust_unnamed
        .native = Some(_fiberRun as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_33).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_33)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_FIBER as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_33),
    );
    vmPopTempRef(vm);
    let mut fn_34: *mut Function = newFunction(
        vm,
        b"resume\0" as *const u8 as *const libc::c_char,
        strlen(b"resume\0" as *const u8 as *const libc::c_char) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        _pk_doc__fiberResume,
        0 as *mut libc::c_int,
    );
    (*fn_34).is_method = 1 as libc::c_int != 0;
    (*fn_34)
        .c2rust_unnamed
        .native = Some(_fiberResume as unsafe extern "C" fn(*mut PKVM) -> ());
    (*fn_34).arity = -(1 as libc::c_int);
    vmPushTempRef(vm, &mut (*fn_34)._super);
    pkClosureBufferWrite(
        &mut (**((*vm).builtin_classes)
            .as_mut_ptr()
            .offset(PK_FIBER as libc::c_int as isize))
            .methods,
        vm,
        newClosure(vm, fn_34),
    );
    vmPopTempRef(vm);
}
pub unsafe extern "C" fn preConstructSelf(
    mut vm: *mut PKVM,
    mut cls: *mut Class,
) -> Var {
    match (*cls).class_of as libc::c_uint {
        0 => {
            (*(*vm).fiber)
                .error = newStringLength(
                vm,
                b"Class 'Object' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char,
                if (b"Class 'Object' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(
                        b"Class 'Object' cannot be instanciated.\0" as *const u8
                            as *const libc::c_char,
                    ) as uint32_t
                },
            );
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        8 => {
            (*(*vm).fiber)
                .error = newStringLength(
                vm,
                b"Class 'Module' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char,
                if (b"Class 'Module' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(
                        b"Class 'Module' cannot be instanciated.\0" as *const u8
                            as *const libc::c_char,
                    ) as uint32_t
                },
            );
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        9 => {
            (*(*vm).fiber)
                .error = newStringLength(
                vm,
                b"Class 'Closure' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char,
                if (b"Class 'Closure' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(
                        b"Class 'Closure' cannot be instanciated.\0" as *const u8
                            as *const libc::c_char,
                    ) as uint32_t
                },
            );
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        11 => {
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        12 => {
            (*(*vm).fiber)
                .error = newStringLength(
                vm,
                b"Class 'Class' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char,
                if (b"Class 'Class' cannot be instanciated.\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(
                        b"Class 'Class' cannot be instanciated.\0" as *const u8
                            as *const libc::c_char,
                    ) as uint32_t
                },
            );
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        13 => {
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*(newInstance
                    as unsafe extern "C" fn(
                        *mut PKVM,
                        *mut Class,
                    ) -> *mut Instance)(vm, cls))
                    ._super as *mut Object as uintptr_t;
        }
        _ => {}
    }
    unreachable!();
}
pub unsafe extern "C" fn getClass(mut vm: *mut PKVM, mut instance: Var) -> *mut Class {
    let mut type_0: PkVarType = getVarType(instance);
    if 0 as libc::c_int as libc::c_uint <= type_0 as libc::c_uint
        && (type_0 as libc::c_uint) < PK_INSTANCE as libc::c_int as libc::c_uint
    {
        return (*vm).builtin_classes[type_0 as usize];
    }
    let mut inst: *mut Instance = (instance & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object as *mut Instance;
    return (*inst).cls;
}
#[inline]
unsafe extern "C" fn clsGetMethod(
    mut cls: *mut Class,
    mut name: *mut String_0,
) -> *mut Closure {
    let mut cls_: *mut Class = cls;
    loop {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*cls_).methods.count as libc::c_int {
            let mut method_: *mut Closure = *((*cls_).methods.data).offset(i as isize);
            let mut method_name: *const libc::c_char = (*(*method_).fn_0).name;
            if (*name).length as libc::c_ulong == strlen(method_name)
                && memcmp(
                    ((*name).data).as_mut_ptr() as *const libc::c_void,
                    method_name as *const libc::c_void,
                    strlen(method_name),
                ) == 0 as libc::c_int
            {
                return method_;
            }
            i += 1;
            i;
        }
        cls_ = (*cls_).super_class;
        if cls_.is_null() {
            break;
        }
    }
    return 0 as *mut Closure;
}
pub unsafe extern "C" fn hasMethod(
    mut vm: *mut PKVM,
    mut self_0: Var,
    mut name: *mut String_0,
    mut _method: *mut *mut Closure,
) -> bool {
    let mut cls: *mut Class = getClass(vm, self_0);
    let mut method_: *mut Closure = clsGetMethod(cls, name);
    if !method_.is_null() {
        *_method = method_;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn getMethod(
    mut vm: *mut PKVM,
    mut self_0: Var,
    mut name: *mut String_0,
    mut is_method: *mut bool,
) -> Var {
    let mut method: *mut Closure = 0 as *mut Closure;
    if hasMethod(vm, self_0, name, &mut method) {
        if !is_method.is_null() {
            *is_method = 1 as libc::c_int != 0;
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*method)._super as *mut Object as uintptr_t;
    }
    if !is_method.is_null() {
        *is_method = 0 as libc::c_int != 0;
    }
    return varGetAttrib(vm, self_0, name);
}
pub unsafe extern "C" fn getSuperMethod(
    mut vm: *mut PKVM,
    mut self_0: Var,
    mut name: *mut String_0,
) -> *mut Closure {
    let mut super_0: *mut Class = (*getClass(vm, self_0)).super_class;
    if super_0.is_null() {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"'$' object has no parent class.\0" as *const u8 as *const libc::c_char,
            varTypeName(self_0),
        );
        return 0 as *mut Closure;
    }
    let mut method: *mut Closure = clsGetMethod(super_0, name);
    if method.is_null() {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"'@' class has no method named '@'.\0" as *const u8 as *const libc::c_char,
            (*super_0).name,
            name,
        );
    }
    return method;
}
pub unsafe extern "C" fn varPositive(mut vm: *mut PKVM, mut v: Var) -> Var {
    let mut n: libc::c_double = 0.;
    if isNumeric(v, &mut n) {
        return v;
    }
    if v
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if _callUnaryOpMethod(
            vm,
            v,
            b"+self\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand ($) for unary operator unary +.\0" as *const u8
            as *const libc::c_char,
        varTypeName(v),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varNegative(mut vm: *mut PKVM, mut v: Var) -> Var {
    let mut n: libc::c_double = 0.;
    if isNumeric(v, &mut n) {
        return doubleToVar(-varToDouble(v));
    }
    if v
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if _callUnaryOpMethod(
            vm,
            v,
            b"-self\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand ($) for unary operator unary -.\0" as *const u8
            as *const libc::c_char,
        varTypeName(v),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varNot(mut vm: *mut PKVM, mut v: Var) -> Var {
    if v
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if _callUnaryOpMethod(
            vm,
            v,
            b"!self\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    return if !toBool(v) {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t
    };
}
pub unsafe extern "C" fn varBitNot(mut vm: *mut PKVM, mut v: Var) -> Var {
    let mut i: int64_t = 0;
    if isInteger(v, &mut i) {
        return doubleToVar(!i as libc::c_double);
    }
    if v
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if _callUnaryOpMethod(
            vm,
            v,
            b"~self\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand ($) for unary operator unary ~.\0" as *const u8
            as *const libc::c_char,
        varTypeName(v),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varAdd(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar(n1 + n2);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        let mut o1: *mut Object = (v1_0 & 0xffffffffffff as libc::c_long as uint64_t)
            as *mut Object;
        match (*o1).type_0 as libc::c_uint {
            0 => {
                if v2_0
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                {
                    let mut o2: *mut Object = (v2_0
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object;
                    if (*o2).type_0 as libc::c_uint
                        == OBJ_STRING as libc::c_int as libc::c_uint
                    {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(stringJoin
                                as unsafe extern "C" fn(
                                    *mut PKVM,
                                    *mut String_0,
                                    *mut String_0,
                                ) -> *mut String_0)(
                                vm,
                                o1 as *mut String_0,
                                o2 as *mut String_0,
                            ))
                                ._super as *mut Object as uintptr_t;
                    }
                }
            }
            1 => {
                if v2_0
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                {
                    let mut o2_0: *mut Object = (v2_0
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object;
                    if (*o2_0).type_0 as libc::c_uint
                        == OBJ_LIST as libc::c_int as libc::c_uint
                    {
                        if inplace {
                            pkVarBufferConcat(
                                &mut (*(o1 as *mut List)).elements,
                                vm,
                                &mut (*(o2_0 as *mut List)).elements,
                            );
                            return v1_0;
                        } else {
                            return 0x7ffc000000000000 as libc::c_long as uint64_t
                                | 0x8000000000000000 as libc::c_ulong
                                | &mut (*(listAdd
                                    as unsafe extern "C" fn(
                                        *mut PKVM,
                                        *mut List,
                                        *mut List,
                                    ) -> *mut List)(vm, o1 as *mut List, o2_0 as *mut List))
                                    ._super as *mut Object as uintptr_t
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"+=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"+\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '+' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varModulo(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar(fmod(n1, n2));
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int == 0 {
            fprintf(
                stderr,
                b"Assertion failed: %s\n\tat %s() (%s:%i)\n\tcondition: %s\0"
                    as *const u8 as *const libc::c_char,
                b"TODO: It hasn't implemented yet.\0" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 10],
                    &[libc::c_char; 10],
                >(b"varModulo\0"))
                    .as_ptr(),
                b"src/core/core.c\0" as *const u8 as *const libc::c_char,
                1743 as libc::c_int,
                b"false\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"%=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"%\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '%' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varSubtract(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar(n1 - n2);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"-=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"-\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '-' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varMultiply(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar(n1 * n2);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"*=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"*\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        let mut left: *mut String_0 = (v1_0 & 0xffffffffffff as libc::c_long as uint64_t)
            as *mut Object as *mut String_0;
        let mut right: int64_t = 0;
        if isInteger(v2_0, &mut right) {
            if (*left).length == 0 as libc::c_int as libc::c_uint {
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*left)._super as *mut Object as uintptr_t;
            }
            if right == 0 as libc::c_int as libc::c_long {
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*(newStringLength
                        as unsafe extern "C" fn(
                            *mut PKVM,
                            *const libc::c_char,
                            uint32_t,
                        ) -> *mut String_0)(
                        vm,
                        b"\0" as *const u8 as *const libc::c_char,
                        (if (b"\0" as *const u8 as *const libc::c_char).is_null() {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            (strlen
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> libc::c_ulong)(
                                b"\0" as *const u8 as *const libc::c_char,
                            ) as uint32_t
                        }),
                    ))
                        ._super as *mut Object as uintptr_t;
            }
            if right < 0 as libc::c_int as libc::c_long {
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*(newStringLength
                        as unsafe extern "C" fn(
                            *mut PKVM,
                            *const libc::c_char,
                            uint32_t,
                        ) -> *mut String_0)(
                        vm,
                        b"\0" as *const u8 as *const libc::c_char,
                        (if (b"\0" as *const u8 as *const libc::c_char).is_null() {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            (strlen
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> libc::c_ulong)(
                                b"\0" as *const u8 as *const libc::c_char,
                            ) as uint32_t
                        }),
                    ))
                        ._super as *mut Object as uintptr_t;
            }
            let mut str: *mut String_0 = newStringLength(
                vm,
                b"\0" as *const u8 as *const libc::c_char,
                ((*left).length).wrapping_mul(right as uint32_t),
            );
            let mut buff: *mut libc::c_char = ((*str).data).as_mut_ptr();
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < right as libc::c_int {
                memcpy(
                    buff as *mut libc::c_void,
                    ((*left).data).as_mut_ptr() as *const libc::c_void,
                    (*left).length as libc::c_ulong,
                );
                buff = buff.offset((*left).length as isize);
                i += 1;
                i;
            }
            (*str).hash = utilHashString(((*str).data).as_mut_ptr());
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*str)._super as *mut Object as uintptr_t;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '*' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varDivide(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar(n1 / n2);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"/=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"/\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '/' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varExponent(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar(pow(n1, n2));
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"**=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"**\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '**' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varBitAnd(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut i1: int64_t = 0;
    let mut i2: int64_t = 0;
    if isInteger(v1_0, &mut i1) {
        if validateInteger(
            vm,
            v2_0,
            &mut i2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar((i1 & i2) as libc::c_double);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"&=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"&\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '&' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varBitOr(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut i1: int64_t = 0;
    let mut i2: int64_t = 0;
    if isInteger(v1_0, &mut i1) {
        if validateInteger(
            vm,
            v2_0,
            &mut i2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar((i1 | i2) as libc::c_double);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"|=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"|\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '|' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varBitXor(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut i1: int64_t = 0;
    let mut i2: int64_t = 0;
    if isInteger(v1_0, &mut i1) {
        if validateInteger(
            vm,
            v2_0,
            &mut i2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar((i1 ^ i2) as libc::c_double);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"^=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"^\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '^' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varBitLshift(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut i1: int64_t = 0;
    let mut i2: int64_t = 0;
    if isInteger(v1_0, &mut i1) {
        if validateInteger(
            vm,
            v2_0,
            &mut i2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar((i1 << i2) as libc::c_double);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"<<=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"<<\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '<<' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varBitRshift(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
    mut inplace: bool,
) -> Var {
    let mut i1: int64_t = 0;
    let mut i2: int64_t = 0;
    if isInteger(v1_0, &mut i1) {
        if validateInteger(
            vm,
            v2_0,
            &mut i2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return doubleToVar((i1 >> i2) as libc::c_double);
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b">>=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b">>\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '>>' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varEqals(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
) -> Var {
    let inplace: bool = 0 as libc::c_int != 0;
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"===\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"==\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    return if isValuesEqual(v1_0, v2_0) as libc::c_int != 0 {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t
    };
}
pub unsafe extern "C" fn varGreater(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return if n1 > n2 {
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000003 as libc::c_long as uint64_t
            } else {
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000002 as libc::c_long as uint64_t
            };
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    let inplace: bool = 0 as libc::c_int != 0;
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b">=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b">\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '>' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varLesser(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
) -> Var {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if isNumeric(v1_0, &mut n1) {
        if validateNumeric(
            vm,
            v2_0,
            &mut n2,
            b"Right operand\0" as *const u8 as *const libc::c_char,
        ) {
            return if n1 < n2 {
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000003 as libc::c_long as uint64_t
            } else {
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000002 as libc::c_long as uint64_t
            };
        }
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    let inplace: bool = 0 as libc::c_int != 0;
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"<=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"<\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '<' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varOpRange(
    mut vm: *mut PKVM,
    mut v1_0: Var,
    mut v2_0: Var,
) -> Var {
    if v1_0 & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t
        && v2_0 & 0x7ffc000000000000 as libc::c_long as uint64_t
            != 0x7ffc000000000000 as libc::c_long as uint64_t
    {
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*(newRange
                as unsafe extern "C" fn(
                    *mut PKVM,
                    libc::c_double,
                    libc::c_double,
                ) -> *mut Range)(
                vm,
                (varToDouble as unsafe extern "C" fn(Var) -> libc::c_double)(v1_0),
                (varToDouble as unsafe extern "C" fn(Var) -> libc::c_double)(v2_0),
            ))
                ._super as *mut Object as uintptr_t;
    }
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint
    {
        let mut str: *mut String_0 = varToString(vm, v2_0, 0 as libc::c_int != 0);
        if str.is_null() {
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        let mut concat: *mut String_0 = stringJoin(
            vm,
            (v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut String_0,
            str,
        );
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*concat)._super as *mut Object as uintptr_t;
    }
    let inplace: bool = 0 as libc::c_int != 0;
    if v1_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((v1_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                v1_0,
                v2_0,
                b"..=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result;
            }
        }
        if _callBinaryOpMethod(
            vm,
            v1_0,
            v2_0,
            b"..\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Unsupported operand types for operator '..' $ and $\0" as *const u8
            as *const libc::c_char,
        varTypeName(v1_0),
        varTypeName(v2_0),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varContains(
    mut vm: *mut PKVM,
    mut elem: Var,
    mut container: Var,
) -> bool {
    if !(container
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
    {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"'$' is not iterable.\0" as *const u8 as *const libc::c_char,
            varTypeName(container),
        );
    }
    let mut obj: *mut Object = (container & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    match (*obj).type_0 as libc::c_uint {
        0 => {
            if !(elem
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && (*((elem & 0xffffffffffff as libc::c_long as uint64_t)
                    as *mut Object))
                    .type_0 as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint)
            {
                (*(*vm).fiber)
                    .error = stringFormat(
                    vm,
                    b"Expected a string operand.\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
            let mut sub: *mut String_0 = (elem
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut String_0;
            let mut str: *mut String_0 = (container
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut String_0;
            if (*sub).length > (*str).length {
                return 0 as libc::c_int != 0;
            }
            let mut match_0: *const libc::c_char = strstr(
                ((*str).data).as_mut_ptr(),
                ((*sub).data).as_mut_ptr(),
            );
            return !match_0.is_null();
        }
        1 => {
            let mut list: *mut List = (container
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut List;
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*list).elements.count {
                if isValuesEqual(elem, *((*list).elements.data).offset(i as isize)) {
                    return 1 as libc::c_int != 0;
                }
                i = i.wrapping_add(1);
                i;
            }
            return 0 as libc::c_int != 0;
        }
        2 => {
            let mut map: *mut Map = (container
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Map;
            return !(mapGet(map, elem)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000000 as libc::c_long as uint64_t);
        }
        _ => {}
    }
    let inplace: bool = 0 as libc::c_int != 0;
    if container
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((container & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_INST as libc::c_int as libc::c_uint
    {
        let mut result: Var = 0;
        if inplace {
            if _callBinaryOpMethod(
                vm,
                container,
                elem,
                b"in=\0" as *const u8 as *const libc::c_char,
                &mut result,
            ) {
                return result != 0;
            }
        }
        if _callBinaryOpMethod(
            vm,
            container,
            elem,
            b"in\0" as *const u8 as *const libc::c_char,
            &mut result,
        ) {
            return result != 0;
        }
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Argument of type $ is not iterable.\0" as *const u8 as *const libc::c_char,
        varTypeName(container),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t
        != 0;
}
pub unsafe extern "C" fn varIsType(
    mut vm: *mut PKVM,
    mut inst: Var,
    mut type_0: Var,
) -> bool {
    if !(type_0
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((type_0 & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_CLASS as libc::c_int as libc::c_uint)
    {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Right operand must be a class.\0" as *const u8 as *const libc::c_char,
            if (b"Right operand must be a class.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Right operand must be a class.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        return 0 as libc::c_int != 0;
    }
    let mut cls: *mut Class = (type_0 & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object as *mut Class;
    let mut cls_inst: *mut Class = getClass(vm, inst);
    loop {
        if cls_inst == cls {
            return 1 as libc::c_int != 0;
        }
        cls_inst = (*cls_inst).super_class;
        if cls_inst.is_null() {
            break;
        }
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn varGetAttrib(
    mut vm: *mut PKVM,
    mut on: Var,
    mut attrib: *mut String_0,
) -> Var {
    if (*attrib).hash == 0xa2d93eae as libc::c_uint {
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*(getClass
                as unsafe extern "C" fn(*mut PKVM, Var) -> *mut Class)(vm, on))
                ._super as *mut Object as uintptr_t;
    }
    if !(on
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
    {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"'$' object has no attribute named '$'.\0" as *const u8
                as *const libc::c_char,
            varTypeName(on),
            ((*attrib).data).as_mut_ptr(),
        );
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    let mut obj: *mut Object = (on & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    match (*obj).type_0 as libc::c_uint {
        0 => {
            let mut str: *mut String_0 = obj as *mut String_0;
            match (*attrib).hash {
                2211460629 => return doubleToVar((*str).length as libc::c_double),
                _ => {}
            }
        }
        1 => {
            let mut list: *mut List = obj as *mut List;
            match (*attrib).hash {
                2211460629 => {
                    return doubleToVar((*list).elements.count as libc::c_double);
                }
                _ => {}
            }
        }
        3 => {
            let mut range: *mut Range = obj as *mut Range;
            match (*attrib).hash {
                22424610 => {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*(rangeAsList
                            as unsafe extern "C" fn(
                                *mut PKVM,
                                *mut Range,
                            ) -> *mut List)(vm, range))
                            ._super as *mut Object as uintptr_t;
                }
                1216469057 => return doubleToVar((*range).from),
                1675745305 => return doubleToVar((*range).to),
                _ => {}
            }
        }
        4 => {
            let mut module: *mut Module = obj as *mut Module;
            let mut index: libc::c_int = moduleGetGlobalIndex(
                module,
                ((*attrib).data).as_mut_ptr(),
                (*attrib).length,
            );
            if index != -(1 as libc::c_int) {
                return *((*module).globals.data).offset(index as isize);
            }
        }
        6 => {
            let mut closure: *mut Closure = obj as *mut Closure;
            match (*attrib).hash {
                2369371622 => {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*(newStringLength
                            as unsafe extern "C" fn(
                                *mut PKVM,
                                *const libc::c_char,
                                uint32_t,
                            ) -> *mut String_0)(
                            vm,
                            (*(*closure).fn_0).name,
                            (if ((*(*closure).fn_0).name).is_null() {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                (strlen
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                    ) -> libc::c_ulong)((*(*closure).fn_0).name) as uint32_t
                            }),
                        ))
                            ._super as *mut Object as uintptr_t;
                }
                2411017897 => {
                    if !((*(*closure).fn_0).docstring).is_null() {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(newStringLength
                                as unsafe extern "C" fn(
                                    *mut PKVM,
                                    *const libc::c_char,
                                    uint32_t,
                                ) -> *mut String_0)(
                                vm,
                                (*(*closure).fn_0).docstring,
                                (if ((*(*closure).fn_0).docstring).is_null() {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (strlen
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                        ) -> libc::c_ulong)((*(*closure).fn_0).docstring)
                                        as uint32_t
                                }),
                            ))
                                ._super as *mut Object as uintptr_t
                    } else {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(newStringLength
                                as unsafe extern "C" fn(
                                    *mut PKVM,
                                    *const libc::c_char,
                                    uint32_t,
                                ) -> *mut String_0)(
                                vm,
                                b"\0" as *const u8 as *const libc::c_char,
                                (if (b"\0" as *const u8 as *const libc::c_char).is_null() {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (strlen
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                        ) -> libc::c_ulong)(
                                        b"\0" as *const u8 as *const libc::c_char,
                                    ) as uint32_t
                                }),
                            ))
                                ._super as *mut Object as uintptr_t
                    }
                }
                1050066298 => {
                    return doubleToVar((*(*closure).fn_0).arity as libc::c_double);
                }
                _ => {}
            }
        }
        7 => {
            let mut mb: *mut MethodBind = obj as *mut MethodBind;
            match (*attrib).hash {
                2411017897 => {
                    if !((*(*(*mb).method).fn_0).docstring).is_null() {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(newStringLength
                                as unsafe extern "C" fn(
                                    *mut PKVM,
                                    *const libc::c_char,
                                    uint32_t,
                                ) -> *mut String_0)(
                                vm,
                                (*(*(*mb).method).fn_0).docstring,
                                (if ((*(*(*mb).method).fn_0).docstring).is_null() {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (strlen
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                        ) -> libc::c_ulong)((*(*(*mb).method).fn_0).docstring)
                                        as uint32_t
                                }),
                            ))
                                ._super as *mut Object as uintptr_t
                    } else {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(newStringLength
                                as unsafe extern "C" fn(
                                    *mut PKVM,
                                    *const libc::c_char,
                                    uint32_t,
                                ) -> *mut String_0)(
                                vm,
                                b"\0" as *const u8 as *const libc::c_char,
                                (if (b"\0" as *const u8 as *const libc::c_char).is_null() {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (strlen
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                        ) -> libc::c_ulong)(
                                        b"\0" as *const u8 as *const libc::c_char,
                                    ) as uint32_t
                                }),
                            ))
                                ._super as *mut Object as uintptr_t
                    }
                }
                2369371622 => {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*(newStringLength
                            as unsafe extern "C" fn(
                                *mut PKVM,
                                *const libc::c_char,
                                uint32_t,
                            ) -> *mut String_0)(
                            vm,
                            (*(*(*mb).method).fn_0).name,
                            (if ((*(*(*mb).method).fn_0).name).is_null() {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                (strlen
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                    ) -> libc::c_ulong)((*(*(*mb).method).fn_0).name)
                                    as uint32_t
                            }),
                        ))
                            ._super as *mut Object as uintptr_t;
                }
                193386898 => {
                    if (*mb).instance
                        == 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000000 as libc::c_long as uint64_t
                    {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0 as libc::c_int as uint64_t;
                    }
                    return (*mb).instance;
                }
                _ => {}
            }
        }
        8 => {
            unreachable!();
        }
        9 => {
            let mut fb: *mut Fiber = obj as *mut Fiber;
            match (*attrib).hash {
                2023499526 => {
                    return if (*fb).state as libc::c_uint
                        == FIBER_DONE as libc::c_int as libc::c_uint
                    {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000003 as libc::c_long as uint64_t
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000002 as libc::c_long as uint64_t
                    };
                }
                2664841801 => {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*(*fb).closure)._super as *mut Object as uintptr_t;
                }
                _ => {}
            }
        }
        10 => {
            let mut cls: *mut Class = obj as *mut Class;
            match (*attrib).hash {
                2411017897 => {
                    if !((*cls).docstring).is_null() {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(newStringLength
                                as unsafe extern "C" fn(
                                    *mut PKVM,
                                    *const libc::c_char,
                                    uint32_t,
                                ) -> *mut String_0)(
                                vm,
                                (*cls).docstring,
                                (if ((*cls).docstring).is_null() {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (strlen
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                        ) -> libc::c_ulong)((*cls).docstring) as uint32_t
                                }),
                            ))
                                ._super as *mut Object as uintptr_t
                    } else {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(newStringLength
                                as unsafe extern "C" fn(
                                    *mut PKVM,
                                    *const libc::c_char,
                                    uint32_t,
                                ) -> *mut String_0)(
                                vm,
                                b"\0" as *const u8 as *const libc::c_char,
                                (if (b"\0" as *const u8 as *const libc::c_char).is_null() {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    (strlen
                                        as unsafe extern "C" fn(
                                            *const libc::c_char,
                                        ) -> libc::c_ulong)(
                                        b"\0" as *const u8 as *const libc::c_char,
                                    ) as uint32_t
                                }),
                            ))
                                ._super as *mut Object as uintptr_t
                    }
                }
                2369371622 => {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*(newStringLength
                            as unsafe extern "C" fn(
                                *mut PKVM,
                                *const libc::c_char,
                                uint32_t,
                            ) -> *mut String_0)(
                            vm,
                            ((*(*cls).name).data).as_mut_ptr(),
                            (if ((*(*cls).name).data).as_mut_ptr().is_null() {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                (strlen
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                    ) -> libc::c_ulong)(((*(*cls).name).data).as_mut_ptr())
                                    as uint32_t
                            }),
                        ))
                            ._super as *mut Object as uintptr_t;
                }
                3939368189 => {
                    if !((*cls).super_class).is_null() {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*(*cls).super_class)._super as *mut Object
                                as uintptr_t
                    } else {
                        return 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0 as libc::c_int as uint64_t
                    }
                }
                _ => {}
            }
            let mut value: Var = mapGet(
                (*cls).static_attribs,
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*attrib)._super as *mut Object as uintptr_t,
            );
            if !(value
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000000 as libc::c_long as uint64_t)
            {
                return value;
            }
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < (*cls).methods.count as libc::c_int {
                let mut method_: *mut Closure = *((*cls).methods.data)
                    .offset(i as isize);
                let mut method_name: *const libc::c_char = (*(*method_).fn_0).name;
                if (*attrib).length as libc::c_ulong == strlen(method_name)
                    && memcmp(
                        ((*attrib).data).as_mut_ptr() as *const libc::c_void,
                        method_name as *const libc::c_void,
                        strlen(method_name),
                    ) == 0 as libc::c_int
                {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*(newMethodBind
                            as unsafe extern "C" fn(
                                *mut PKVM,
                                *mut Closure,
                            ) -> *mut MethodBind)(vm, method_))
                            ._super as *mut Object as uintptr_t;
                }
                i += 1;
                i;
            }
        }
        11 => {
            let mut inst: *mut Instance = obj as *mut Instance;
            let mut value_0: Var = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
            if !((*inst).native).is_null() {
                let mut getter: *mut Closure = 0 as *mut Closure;
                let mut getter_name: *mut String_0 = newStringLength(
                    vm,
                    b"@getter\0" as *const u8 as *const libc::c_char,
                    if (b"@getter\0" as *const u8 as *const libc::c_char).is_null() {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        strlen(b"@getter\0" as *const u8 as *const libc::c_char)
                            as uint32_t
                    },
                );
                vmPushTempRef(vm, &mut (*getter_name)._super);
                let mut has_getter: bool = hasMethod(vm, on, getter_name, &mut getter);
                vmPopTempRef(vm);
                if has_getter {
                    let mut attrib_name: Var = 0x7ffc000000000000 as libc::c_long
                        as uint64_t | 0x8000000000000000 as libc::c_ulong
                        | &mut (*attrib)._super as *mut Object as uintptr_t;
                    vmCallMethod(
                        vm,
                        on,
                        getter,
                        1 as libc::c_int,
                        &mut attrib_name,
                        &mut value_0,
                    );
                    return value_0;
                }
            }
            value_0 = mapGet(
                (*inst).attribs,
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*attrib)._super as *mut Object as uintptr_t,
            );
            if !(value_0
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000000 as libc::c_long as uint64_t)
            {
                return value_0;
            }
            let mut method: *mut Closure = 0 as *mut Closure;
            if hasMethod(vm, on, attrib, &mut method) {
                let mut mb_0: *mut MethodBind = newMethodBind(vm, method);
                (*mb_0).instance = on;
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*mb_0)._super as *mut Object as uintptr_t;
            }
        }
        2 | 5 | _ => {}
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"'$' object has no attribute named '$'.\0" as *const u8 as *const libc::c_char,
        varTypeName(on),
        ((*attrib).data).as_mut_ptr(),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varSetAttrib(
    mut vm: *mut PKVM,
    mut on: Var,
    mut attrib: *mut String_0,
    mut value: Var,
) {
    if !(on
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
    {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"'$' object has no mutable attribute named '$'\0" as *const u8
                as *const libc::c_char,
            varTypeName(on),
            ((*attrib).data).as_mut_ptr(),
        );
        return;
    }
    let mut obj: *mut Object = (on & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    match (*obj).type_0 as libc::c_uint {
        4 => {
            moduleSetGlobal(
                vm,
                obj as *mut Module,
                ((*attrib).data).as_mut_ptr(),
                (*attrib).length,
                value,
            );
            return;
        }
        5 | 8 => {
            unreachable!();
        }
        10 => {
            let mut cls: *mut Class = obj as *mut Class;
            mapSet(
                vm,
                (*cls).static_attribs,
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*attrib)._super as *mut Object as uintptr_t,
                value,
            );
            return;
        }
        11 => {
            let mut inst: *mut Instance = obj as *mut Instance;
            if !((*inst).native).is_null() {
                let mut setter: *mut Closure = 0 as *mut Closure;
                let mut setter_name: *mut String_0 = newStringLength(
                    vm,
                    b"@setter\0" as *const u8 as *const libc::c_char,
                    if (b"@setter\0" as *const u8 as *const libc::c_char).is_null() {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        strlen(b"@setter\0" as *const u8 as *const libc::c_char)
                            as uint32_t
                    },
                );
                vmPushTempRef(vm, &mut (*setter_name)._super);
                let mut has_setter: bool = hasMethod(
                    vm,
                    0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*inst)._super as *mut Object as uintptr_t,
                    setter_name,
                    &mut setter,
                );
                vmPopTempRef(vm);
                if has_setter {
                    let mut args: [Var; 2] = [
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*attrib)._super as *mut Object as uintptr_t,
                        value,
                    ];
                    vmCallMethod(
                        vm,
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                            | &mut (*inst)._super as *mut Object as uintptr_t,
                        setter,
                        2 as libc::c_int,
                        args.as_mut_ptr(),
                        0 as *mut Var,
                    );
                    return;
                }
            }
            mapSet(
                vm,
                (*inst).attribs,
                0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*attrib)._super as *mut Object as uintptr_t,
                value,
            );
            return;
        }
        _ => {}
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"'$' object has no mutable attribute named '$'\0" as *const u8
            as *const libc::c_char,
        varTypeName(on),
        ((*attrib).data).as_mut_ptr(),
    );
}
unsafe extern "C" fn _normalizeSliceRange(
    mut vm: *mut PKVM,
    mut range: *mut Range,
    mut count: uint32_t,
    mut start: *mut int32_t,
    mut length: *mut int32_t,
    mut reversed: *mut bool,
) -> bool {
    if floor((*range).from) != (*range).from || floor((*range).to) != (*range).to {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Expected a whole number.\0" as *const u8 as *const libc::c_char,
            if (b"Expected a whole number.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Expected a whole number.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        return 0 as libc::c_int != 0;
    }
    let mut from: int32_t = (*range).from as int32_t;
    let mut to: int32_t = (*range).to as int32_t;
    if from < 0 as libc::c_int {
        from = count.wrapping_add(from as libc::c_uint) as int32_t;
    }
    if to < 0 as libc::c_int {
        to = count.wrapping_add(to as libc::c_uint) as int32_t;
    }
    *reversed = 0 as libc::c_int != 0;
    if to < from {
        let mut tmp: int32_t = to;
        to = from;
        from = tmp;
        *reversed = 1 as libc::c_int != 0;
    }
    if from < 0 as libc::c_int || count <= to as uint32_t {
        if count == 0 as libc::c_int as libc::c_uint
            && (from == 0 as libc::c_int || from == -(1 as libc::c_int))
            && (to == 0 as libc::c_int || to == -(1 as libc::c_int))
        {
            *start = 0 as libc::c_int;
            *length = 0 as libc::c_int;
            *reversed = 0 as libc::c_int != 0;
            return 1 as libc::c_int != 0;
        }
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Index out of bound.\0" as *const u8 as *const libc::c_char,
            if (b"Index out of bound.\0" as *const u8 as *const libc::c_char).is_null() {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Index out of bound.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        return 0 as libc::c_int != 0;
    }
    *start = from;
    *length = to - from + 1 as libc::c_int;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn _sliceString(
    mut vm: *mut PKVM,
    mut str: *mut String_0,
    mut range: *mut Range,
) -> *mut String_0 {
    let mut start: int32_t = 0;
    let mut length: int32_t = 0;
    let mut reversed: bool = false;
    if !_normalizeSliceRange(
        vm,
        range,
        (*str).length,
        &mut start,
        &mut length,
        &mut reversed,
    ) {
        return 0 as *mut String_0;
    }
    if start == 0 as libc::c_int && length as libc::c_uint == (*str).length && !reversed
    {
        return str;
    }
    let mut slice: *mut String_0 = newStringLength(
        vm,
        ((*str).data).as_mut_ptr().offset(start as isize),
        length as uint32_t,
    );
    if !reversed {
        return slice;
    }
    let mut i: int32_t = 0 as libc::c_int;
    while i < length / 2 as libc::c_int {
        let mut tmp: libc::c_char = *((*slice).data).as_mut_ptr().offset(i as isize);
        *((*slice).data)
            .as_mut_ptr()
            .offset(
                i as isize,
            ) = *((*slice).data)
            .as_mut_ptr()
            .offset((length - i - 1 as libc::c_int) as isize);
        *((*slice).data)
            .as_mut_ptr()
            .offset((length - i - 1 as libc::c_int) as isize) = tmp;
        i += 1;
        i;
    }
    (*slice).hash = utilHashString(((*slice).data).as_mut_ptr());
    return slice;
}
unsafe extern "C" fn _sliceList(
    mut vm: *mut PKVM,
    mut list: *mut List,
    mut range: *mut Range,
) -> *mut List {
    let mut start: int32_t = 0;
    let mut length: int32_t = 0;
    let mut reversed: bool = false;
    if !_normalizeSliceRange(
        vm,
        range,
        (*list).elements.count,
        &mut start,
        &mut length,
        &mut reversed,
    ) {
        return 0 as *mut List;
    }
    let mut slice: *mut List = newList(vm, length as uint32_t);
    vmPushTempRef(vm, &mut (*slice)._super);
    let mut i: int32_t = 0 as libc::c_int;
    while i < length {
        let mut ind: int32_t = if reversed as libc::c_int != 0 {
            start + length - 1 as libc::c_int - i
        } else {
            start + i
        };
        pkVarBufferWrite(
            &mut (*slice).elements,
            vm,
            *((*list).elements.data).offset(ind as isize),
        );
        i += 1;
        i;
    }
    vmPopTempRef(vm);
    return slice;
}
pub unsafe extern "C" fn varGetSubscript(
    mut vm: *mut PKVM,
    mut on: Var,
    mut key: Var,
) -> Var {
    if !(on
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
    {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"$ type is not subscriptable.\0" as *const u8 as *const libc::c_char,
            varTypeName(on),
        );
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    let mut obj: *mut Object = (on & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    match (*obj).type_0 as libc::c_uint {
        0 => {
            let mut index: int64_t = 0;
            let mut str: *mut String_0 = obj as *mut String_0;
            if isInteger(key, &mut index) {
                if index < 0 as libc::c_int as libc::c_long {
                    index = (*str).length as libc::c_long + index;
                }
                if index >= (*str).length as libc::c_long
                    || index < 0 as libc::c_int as libc::c_long
                {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"String index out of bound.\0" as *const u8
                            as *const libc::c_char,
                        if (b"String index out of bound.\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"String index out of bound.\0" as *const u8
                                    as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0 as libc::c_int as uint64_t;
                }
                let mut c: *mut String_0 = newStringLength(
                    vm,
                    ((*str).data).as_mut_ptr().offset(index as isize),
                    1 as libc::c_int as uint32_t,
                );
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                    | &mut (*c)._super as *mut Object as uintptr_t;
            }
            if key
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && (*((key & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                    .type_0 as libc::c_uint == OBJ_RANGE as libc::c_int as libc::c_uint
            {
                let mut subs: *mut String_0 = _sliceString(
                    vm,
                    str,
                    (key & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Range,
                );
                if !subs.is_null() {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*subs)._super as *mut Object as uintptr_t;
                }
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0 as libc::c_int as uint64_t;
            }
        }
        1 => {
            let mut index_0: int64_t = 0;
            let mut elems: *mut pkVarBuffer = &mut (*(obj as *mut List)).elements;
            if isInteger(key, &mut index_0) {
                if index_0 < 0 as libc::c_int as libc::c_long {
                    index_0 = (*elems).count as libc::c_long + index_0;
                }
                if index_0 >= (*elems).count as libc::c_long
                    || index_0 < 0 as libc::c_int as libc::c_long
                {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"List index out of bound.\0" as *const u8
                            as *const libc::c_char,
                        if (b"List index out of bound.\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"List index out of bound.\0" as *const u8
                                    as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0 as libc::c_int as uint64_t;
                }
                return *((*elems).data).offset(index_0 as isize);
            }
            if key
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && (*((key & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                    .type_0 as libc::c_uint == OBJ_RANGE as libc::c_int as libc::c_uint
            {
                let mut sublist: *mut List = _sliceList(
                    vm,
                    obj as *mut List,
                    (key & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Range,
                );
                if !sublist.is_null() {
                    return 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*sublist)._super as *mut Object as uintptr_t;
                }
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0 as libc::c_int as uint64_t;
            }
        }
        2 => {
            let mut value: Var = mapGet(obj as *mut Map, key);
            if value
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x1000000000000 as libc::c_long as uint64_t
            {
                if key
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                    && !isObjectHashable(
                        (*((key & 0xffffffffffff as libc::c_long as uint64_t)
                            as *mut Object))
                            .type_0,
                    )
                {
                    (*(*vm).fiber)
                        .error = stringFormat(
                        vm,
                        b"Unhashable key '$'.\0" as *const u8 as *const libc::c_char,
                        varTypeName(key),
                    );
                } else {
                    let mut key_repr: *mut String_0 = varToString(
                        vm,
                        key,
                        1 as libc::c_int != 0,
                    );
                    vmPushTempRef(vm, &mut (*key_repr)._super);
                    (*(*vm).fiber)
                        .error = stringFormat(
                        vm,
                        b"Key '@' not exists\0" as *const u8 as *const libc::c_char,
                        key_repr,
                    );
                    vmPopTempRef(vm);
                }
                return 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0 as libc::c_int as uint64_t;
            }
            return value;
        }
        5 | 8 => {
            unreachable!();
        }
        11 => {
            let mut ret: Var = 0;
            if _callBinaryOpMethod(
                vm,
                on,
                key,
                b"[]\0" as *const u8 as *const libc::c_char,
                &mut ret,
            ) {
                return ret;
            }
        }
        _ => {}
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"$ type is not subscriptable.\0" as *const u8 as *const libc::c_char,
        varTypeName(on),
    );
    return 0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn varsetSubscript(
    mut vm: *mut PKVM,
    mut on: Var,
    mut key: Var,
    mut value: Var,
) {
    if !(on
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
    {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"$ type is not subscriptable.\0" as *const u8 as *const libc::c_char,
            varTypeName(on),
        );
        return;
    }
    let mut obj: *mut Object = (on & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object;
    match (*obj).type_0 as libc::c_uint {
        1 => {
            let mut index: int64_t = 0;
            let mut elems: *mut pkVarBuffer = &mut (*(obj as *mut List)).elements;
            if !validateInteger(
                vm,
                key,
                &mut index,
                b"List index\0" as *const u8 as *const libc::c_char,
            ) {
                return;
            }
            if index < 0 as libc::c_int as libc::c_long {
                index = (*elems).count as libc::c_long + index;
            }
            if index >= (*elems).count as libc::c_long
                || index < 0 as libc::c_int as libc::c_long
            {
                (*(*vm).fiber)
                    .error = newStringLength(
                    vm,
                    b"List index out of bound.\0" as *const u8 as *const libc::c_char,
                    if (b"List index out of bound.\0" as *const u8
                        as *const libc::c_char)
                        .is_null()
                    {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        strlen(
                            b"List index out of bound.\0" as *const u8
                                as *const libc::c_char,
                        ) as uint32_t
                    },
                );
                return;
            }
            *((*elems).data).offset(index as isize) = value;
            return;
        }
        2 => {
            if key
                & (0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong)
                == 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0x8000000000000000 as libc::c_ulong
                && !isObjectHashable(
                    (*((key & 0xffffffffffff as libc::c_long as uint64_t)
                        as *mut Object))
                        .type_0,
                )
            {
                (*(*vm).fiber)
                    .error = stringFormat(
                    vm,
                    b"$ type is not hashable.\0" as *const u8 as *const libc::c_char,
                    varTypeName(key),
                );
            } else {
                mapSet(vm, obj as *mut Map, key, value);
            }
            return;
        }
        5 | 8 => {
            unreachable!();
        }
        11 => {
            let mut closure: *mut Closure = 0 as *mut Closure;
            let mut name: *mut String_0 = newStringLength(
                vm,
                b"[]=\0" as *const u8 as *const libc::c_char,
                if (b"[]=\0" as *const u8 as *const libc::c_char).is_null() {
                    0 as libc::c_int as libc::c_uint
                } else {
                    strlen(b"[]=\0" as *const u8 as *const libc::c_char) as uint32_t
                },
            );
            vmPushTempRef(vm, &mut (*name)._super);
            let mut has_method: bool = hasMethod(vm, on, name, &mut closure);
            vmPopTempRef(vm);
            if has_method {
                let mut args: [Var; 2] = [key, value];
                vmCallMethod(
                    vm,
                    on,
                    closure,
                    2 as libc::c_int,
                    args.as_mut_ptr(),
                    0 as *mut Var,
                );
                return;
            }
        }
        _ => {}
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"$ type is not subscriptable.\0" as *const u8 as *const libc::c_char,
        varTypeName(on),
    );
}
