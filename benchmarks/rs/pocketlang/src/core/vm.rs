use ::libc;
extern "C" {
    pub type Compiler;
    
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn pkRealloc(
        vm: *mut PKVM,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn pkReleaseHandle(vm: *mut PKVM, handle: *mut PkHandle);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn varsetSubscript(vm: *mut PKVM, on: Var, key: Var, value: Var);
    fn varGetSubscript(vm: *mut PKVM, on: Var, key: Var) -> Var;
    fn varSetAttrib(vm: *mut PKVM, on: Var, name: *mut String_0, value: Var);
    fn varGetAttrib(vm: *mut PKVM, on: Var, attrib: *mut String_0) -> Var;
    fn varIsType(vm: *mut PKVM, inst: Var, type_0: Var) -> bool;
    fn varContains(vm: *mut PKVM, elem: Var, container: Var) -> bool;
    fn varOpRange(vm: *mut PKVM, v1: Var, v2: Var) -> Var;
    fn varLesser(vm: *mut PKVM, v1: Var, v2: Var) -> Var;
    fn varGreater(vm: *mut PKVM, v1: Var, v2: Var) -> Var;
    fn varEqals(vm: *mut PKVM, v1: Var, v2: Var) -> Var;
    fn varBitRshift(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varBitLshift(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varBitXor(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varBitOr(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varBitAnd(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varModulo(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varExponent(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varDivide(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varMultiply(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varSubtract(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varAdd(vm: *mut PKVM, v1: Var, v2: Var, inplace: bool) -> Var;
    fn varBitNot(vm: *mut PKVM, v: Var) -> Var;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pkVarBufferWrite(self_0: *mut pkVarBuffer, vm: *mut PKVM, data: Var);
    fn pkClosureBufferWrite(
        self_0: *mut pkClosureBuffer,
        vm: *mut PKVM,
        data: *mut Closure,
    );
    fn newStringLength(
        vm: *mut PKVM,
        text: *const libc::c_char,
        length: uint32_t,
    ) -> *mut String_0;
    fn newList(vm: *mut PKVM, size: uint32_t) -> *mut List;
    fn newMap(vm: *mut PKVM) -> *mut Map;
    fn newModule(vm: *mut PKVM) -> *mut Module;
    fn newClosure(vm: *mut PKVM, fn_0: *mut Function) -> *mut Closure;
    fn newUpvalue(vm: *mut PKVM, value: *mut Var) -> *mut Upvalue;
    fn newFiber(vm: *mut PKVM, closure: *mut Closure) -> *mut Fiber;
    fn markObject(vm: *mut PKVM, self_0: *mut Object);
    fn markValue(vm: *mut PKVM, self_0: Var);
    fn popMarkedObjects(vm: *mut PKVM);
    fn stringFormat(vm: *mut PKVM, fmt: *const libc::c_char, _: ...) -> *mut String_0;
    fn mapGet(self_0: *mut Map, key: Var) -> Var;
    fn mapSet(vm: *mut PKVM, self_0: *mut Map, key: Var, value: Var);
    fn moduleGetStringAt(module: *mut Module, index: libc::c_int) -> *mut String_0;
    fn freeObject(vm: *mut PKVM, self_0: *mut Object);
    fn doubleToVar(value: libc::c_double) -> Var;
    fn varToDouble(value: Var) -> libc::c_double;
    fn getPkVarTypeName(type_0: PkVarType) -> *const libc::c_char;
    fn varTypeName(v: Var) -> *const libc::c_char;
    fn isObjectHashable(type_0: ObjectType) -> bool;
    fn toBool(v: Var) -> bool;
    fn compile(
        vm: *mut PKVM,
        module: *mut Module,
        source: *const libc::c_char,
        options: *const CompileOptions,
    ) -> PkResult;
    fn compilerMarkObjects(vm: *mut PKVM, compiler: *mut Compiler);
    fn initializeModule(vm: *mut PKVM, module: *mut Module, is_main: bool);
    fn preConstructSelf(vm: *mut PKVM, cls: *mut Class) -> Var;
    fn getMethod(
        vm: *mut PKVM,
        self_0: Var,
        name: *mut String_0,
        is_method: *mut bool,
    ) -> Var;
    fn getSuperMethod(vm: *mut PKVM, self_0: Var, name: *mut String_0) -> *mut Closure;
    fn varToString(vm: *mut PKVM, self_0: Var, repr: bool) -> *mut String_0;
    fn varPositive(vm: *mut PKVM, v: Var) -> Var;
    fn varNegative(vm: *mut PKVM, v: Var) -> Var;
    fn varNot(vm: *mut PKVM, v: Var) -> Var;
    fn utilPowerOf2Ceil(n: libc::c_int) -> libc::c_int;
    fn utilHashString(string: *const libc::c_char) -> uint32_t;
    fn reportRuntimeError(vm: *mut PKVM, fiber: *mut Fiber);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type Opcode = libc::c_uint;
pub const OP_END: Opcode = 88;
pub const OP_REPL_PRINT: Opcode = 87;
pub const OP_IS: Opcode = 86;
pub const OP_IN: Opcode = 85;
pub const OP_RANGE: Opcode = 84;
pub const OP_GTEQ: Opcode = 83;
pub const OP_GT: Opcode = 82;
pub const OP_LTEQ: Opcode = 81;
pub const OP_LT: Opcode = 80;
pub const OP_NOTEQ: Opcode = 79;
pub const OP_EQEQ: Opcode = 78;
pub const OP_BIT_RSHIFT: Opcode = 77;
pub const OP_BIT_LSHIFT: Opcode = 76;
pub const OP_BIT_XOR: Opcode = 75;
pub const OP_BIT_OR: Opcode = 74;
pub const OP_BIT_AND: Opcode = 73;
pub const OP_MOD: Opcode = 72;
pub const OP_EXPONENT: Opcode = 71;
pub const OP_DIVIDE: Opcode = 70;
pub const OP_MULTIPLY: Opcode = 69;
pub const OP_SUBTRACT: Opcode = 68;
pub const OP_ADD: Opcode = 67;
pub const OP_BIT_NOT: Opcode = 66;
pub const OP_NOT: Opcode = 65;
pub const OP_NEGATIVE: Opcode = 64;
pub const OP_POSITIVE: Opcode = 63;
pub const OP_SET_SUBSCRIPT: Opcode = 62;
pub const OP_GET_SUBSCRIPT_KEEP: Opcode = 61;
pub const OP_GET_SUBSCRIPT: Opcode = 60;
pub const OP_SET_ATTRIB: Opcode = 59;
pub const OP_GET_ATTRIB_KEEP: Opcode = 58;
pub const OP_GET_ATTRIB: Opcode = 57;
pub const OP_RETURN: Opcode = 56;
pub const OP_AND: Opcode = 55;
pub const OP_OR: Opcode = 54;
pub const OP_JUMP_IF_NOT: Opcode = 53;
pub const OP_JUMP_IF: Opcode = 52;
pub const OP_LOOP: Opcode = 51;
pub const OP_JUMP: Opcode = 50;
pub const OP_ITER: Opcode = 49;
pub const OP_ITER_TEST: Opcode = 48;
pub const OP_TAIL_CALL: Opcode = 47;
pub const OP_CALL: Opcode = 46;
pub const OP_METHOD_CALL: Opcode = 45;
pub const OP_SUPER_CALL: Opcode = 44;
pub const OP_IMPORT: Opcode = 43;
pub const OP_POP: Opcode = 42;
pub const OP_CLOSE_UPVALUE: Opcode = 41;
pub const OP_BIND_METHOD: Opcode = 40;
pub const OP_CREATE_CLASS: Opcode = 39;
pub const OP_PUSH_CLOSURE: Opcode = 38;
pub const OP_STORE_UPVALUE: Opcode = 37;
pub const OP_PUSH_UPVALUE: Opcode = 36;
pub const OP_PUSH_BUILTIN_TY: Opcode = 35;
pub const OP_PUSH_BUILTIN_FN: Opcode = 34;
pub const OP_STORE_GLOBAL: Opcode = 33;
pub const OP_PUSH_GLOBAL: Opcode = 32;
pub const OP_STORE_LOCAL_N: Opcode = 31;
pub const OP_STORE_LOCAL_8: Opcode = 30;
pub const OP_STORE_LOCAL_7: Opcode = 29;
pub const OP_STORE_LOCAL_6: Opcode = 28;
pub const OP_STORE_LOCAL_5: Opcode = 27;
pub const OP_STORE_LOCAL_4: Opcode = 26;
pub const OP_STORE_LOCAL_3: Opcode = 25;
pub const OP_STORE_LOCAL_2: Opcode = 24;
pub const OP_STORE_LOCAL_1: Opcode = 23;
pub const OP_STORE_LOCAL_0: Opcode = 22;
pub const OP_PUSH_LOCAL_N: Opcode = 21;
pub const OP_PUSH_LOCAL_8: Opcode = 20;
pub const OP_PUSH_LOCAL_7: Opcode = 19;
pub const OP_PUSH_LOCAL_6: Opcode = 18;
pub const OP_PUSH_LOCAL_5: Opcode = 17;
pub const OP_PUSH_LOCAL_4: Opcode = 16;
pub const OP_PUSH_LOCAL_3: Opcode = 15;
pub const OP_PUSH_LOCAL_2: Opcode = 14;
pub const OP_PUSH_LOCAL_1: Opcode = 13;
pub const OP_PUSH_LOCAL_0: Opcode = 12;
pub const OP_MAP_INSERT: Opcode = 11;
pub const OP_LIST_APPEND: Opcode = 10;
pub const OP_PUSH_SELF: Opcode = 9;
pub const OP_PUSH_MAP: Opcode = 8;
pub const OP_PUSH_LIST: Opcode = 7;
pub const OP_DUP: Opcode = 6;
pub const OP_SWAP: Opcode = 5;
pub const OP_PUSH_FALSE: Opcode = 4;
pub const OP_PUSH_TRUE: Opcode = 3;
pub const OP_PUSH_0: Opcode = 2;
pub const OP_PUSH_NULL: Opcode = 1;
pub const OP_PUSH_CONSTANT: Opcode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompileOptions {
    pub debug: bool,
    pub repl_mode: bool,
}
pub unsafe extern "C" fn vmNewHandle(
    mut vm: *mut PKVM,
    mut value: Var,
) -> *mut PkHandle {
    let mut handle: *mut PkHandle = vmRealloc(
        vm,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
        ::std::mem::size_of::<PkHandle>() as libc::c_ulong,
    ) as *mut PkHandle;
    (*handle).value = value;
    (*handle).prev = 0 as *mut PkHandle;
    (*handle).next = (*vm).handles;
    if !((*handle).next).is_null() {
        (*(*handle).next).prev = handle;
    }
    (*vm).handles = handle;
    return handle;
}
pub unsafe extern "C" fn vmRealloc(
    mut vm: *mut PKVM,
    mut memory: *mut libc::c_void,
    mut old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    if !(*vm).collecting_garbage {
        (*vm)
            .bytes_allocated = ((*vm).bytes_allocated as libc::c_ulong)
            .wrapping_add(new_size.wrapping_sub(old_size)) as size_t as size_t;
    }
    if new_size > 0 as libc::c_int as libc::c_ulong
        && (*vm).bytes_allocated > (*vm).next_gc
    {
        (*vm).collecting_garbage = 1 as libc::c_int != 0;
        vmCollectGarbage(vm);
        (*vm).collecting_garbage = 0 as libc::c_int != 0;
    }
    return ((*vm).config.realloc_fn).unwrap()(memory, new_size, (*vm).config.user_data);
}
pub unsafe extern "C" fn vmPushTempRef(mut vm: *mut PKVM, mut obj: *mut Object) {
    let fresh0 = (*vm).temp_reference_count;
    (*vm).temp_reference_count = (*vm).temp_reference_count + 1;
    (*vm).temp_reference[fresh0 as usize] = obj;
}
pub unsafe extern "C" fn vmPopTempRef(mut vm: *mut PKVM) {
    (*vm).temp_reference_count -= 1;
    (*vm).temp_reference_count;
}
pub unsafe extern "C" fn vmRegisterModule(
    mut vm: *mut PKVM,
    mut module: *mut Module,
    mut key: *mut String_0,
) {
    mapSet(
        vm,
        (*vm).modules,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*key)._super as *mut Object as uintptr_t,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*module)._super as *mut Object as uintptr_t,
    );
}
pub unsafe extern "C" fn vmGetModule(
    mut vm: *mut PKVM,
    mut key: *mut String_0,
) -> *mut Module {
    let mut module: Var = mapGet(
        (*vm).modules,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*key)._super as *mut Object as uintptr_t,
    );
    if module
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t
    {
        return 0 as *mut Module;
    }
    return (module & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
        as *mut Module;
}
pub unsafe extern "C" fn vmCollectGarbage(mut vm: *mut PKVM) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*vm).builtins_count {
        markObject(
            vm,
            &mut (**((*vm).builtins_funcs).as_mut_ptr().offset(i as isize))._super,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < PK_INSTANCE as libc::c_int {
        if !((*vm).builtin_classes[i_0 as usize]).is_null() {
            markObject(
                vm,
                &mut (**((*vm).builtin_classes).as_mut_ptr().offset(i_0 as isize))._super,
            );
        }
        i_0 += 1;
        i_0;
    }
    markObject(vm, &mut (*(*vm).modules)._super);
    markObject(vm, &mut (*(*vm).search_paths)._super);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < (*vm).temp_reference_count {
        markObject(vm, (*vm).temp_reference[i_1 as usize]);
        i_1 += 1;
        i_1;
    }
    let mut h: *mut PkHandle = (*vm).handles;
    while !h.is_null() {
        markValue(vm, (*h).value);
        h = (*h).next;
    }
    if !((*vm).compiler).is_null() {
        compilerMarkObjects(vm, (*vm).compiler);
    }
    if !((*vm).fiber).is_null() {
        markObject(vm, &mut (*(*vm).fiber)._super);
    }
    (*vm).bytes_allocated = 0 as libc::c_int as size_t;
    popMarkedObjects(vm);
    let mut ptr: *mut *mut Object = &mut (*vm).first;
    while !(*ptr).is_null() {
        if !(**ptr).is_marked {
            let mut garbage: *mut Object = *ptr;
            *ptr = (*garbage).next;
            freeObject(vm, garbage);
        } else {
            (**ptr).is_marked = 0 as libc::c_int != 0;
            ptr = &mut (**ptr).next;
        }
    }
    (*vm)
        .next_gc = ((*vm).bytes_allocated)
        .wrapping_add(
            ((*vm).bytes_allocated)
                .wrapping_mul((*vm).heap_fill_percent as libc::c_ulong)
                .wrapping_div(100 as libc::c_int as libc::c_ulong),
        );
    if (*vm).next_gc < (*vm).min_heap_size {
        (*vm).next_gc = (*vm).min_heap_size;
    }
}
pub unsafe extern "C" fn vmPrepareFiber(
    mut vm: *mut PKVM,
    mut fiber: *mut Fiber,
    mut argc: libc::c_int,
    mut argv: *mut Var,
) -> bool {
    if (*(*(*fiber).closure).fn_0).arity != -(1 as libc::c_int)
        && argc != (*(*(*fiber).closure).fn_0).arity
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(
            buff.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*(*(*fiber).closure).fn_0).arity,
        );
        if !((*vm).fiber).is_null() {
            (*(*vm).fiber)
                .error = stringFormat(
                vm,
                b"Expected exactly $ argument(s) for function $.\0" as *const u8
                    as *const libc::c_char,
                buff.as_mut_ptr(),
                (*(*(*fiber).closure).fn_0).name,
            );
        }
        return 0 as libc::c_int != 0;
    }
    if (*fiber).state as libc::c_uint != FIBER_NEW as libc::c_int as libc::c_uint {
        match (*fiber).state as libc::c_uint {
            0 => {
                unreachable!();
            }
            1 => {
                if !((*vm).fiber).is_null() {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"The fiber has already been running.\0" as *const u8
                            as *const libc::c_char,
                        if (b"The fiber has already been running.\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"The fiber has already been running.\0" as *const u8
                                    as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                }
                return 0 as libc::c_int != 0;
            }
            2 => {
                if !((*vm).fiber).is_null() {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"Cannot run a fiber which is yielded, use fiber_resume() instead.\0"
                            as *const u8 as *const libc::c_char,
                        if (b"Cannot run a fiber which is yielded, use fiber_resume() instead.\0"
                            as *const u8 as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"Cannot run a fiber which is yielded, use fiber_resume() instead.\0"
                                    as *const u8 as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                }
                return 0 as libc::c_int != 0;
            }
            3 => {
                if !((*vm).fiber).is_null() {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"The fiber has done running.\0" as *const u8
                            as *const libc::c_char,
                        if (b"The fiber has done running.\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"The fiber has done running.\0" as *const u8
                                    as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                }
                return 0 as libc::c_int != 0;
            }
            _ => {}
        }
        unreachable!();
    }
    vmEnsureStackSize(
        vm,
        fiber,
        ((*fiber).sp).offset_from((*fiber).stack) as libc::c_long as libc::c_int + argc,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        *((*fiber).ret)
            .offset((1 as libc::c_int + i) as isize) = *argv.offset(i as isize);
        i += 1;
        i;
    }
    (*fiber).sp = ((*fiber).sp).offset(argc as isize);
    if (*(*(*fiber).closure).fn_0).is_native {
        return 1 as libc::c_int != 0;
    }
    (*((*fiber).frames).offset(0 as libc::c_int as isize)).self_0 = (*fiber).self_0;
    (*fiber)
        .self_0 = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn vmSwitchFiber(
    mut vm: *mut PKVM,
    mut fiber: *mut Fiber,
    mut value: *mut Var,
) -> bool {
    if (*fiber).state as libc::c_uint != FIBER_YIELDED as libc::c_int as libc::c_uint {
        match (*fiber).state as libc::c_uint {
            0 => {
                if !((*vm).fiber).is_null() {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"The fiber hasn't started. call fiber_run() to start.\0"
                            as *const u8 as *const libc::c_char,
                        if (b"The fiber hasn't started. call fiber_run() to start.\0"
                            as *const u8 as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"The fiber hasn't started. call fiber_run() to start.\0"
                                    as *const u8 as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                }
                return 0 as libc::c_int != 0;
            }
            1 => {
                if !((*vm).fiber).is_null() {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"The fiber has already been running.\0" as *const u8
                            as *const libc::c_char,
                        if (b"The fiber has already been running.\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"The fiber has already been running.\0" as *const u8
                                    as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                }
                return 0 as libc::c_int != 0;
            }
            2 => {
                unreachable!();
            }
            3 => {
                if !((*vm).fiber).is_null() {
                    (*(*vm).fiber)
                        .error = newStringLength(
                        vm,
                        b"The fiber has done running.\0" as *const u8
                            as *const libc::c_char,
                        if (b"The fiber has done running.\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                        {
                            0 as libc::c_int as libc::c_uint
                        } else {
                            strlen(
                                b"The fiber has done running.\0" as *const u8
                                    as *const libc::c_char,
                            ) as uint32_t
                        },
                    );
                }
                return 0 as libc::c_int != 0;
            }
            _ => {}
        }
        unreachable!();
    }
    if value.is_null() {
        *(*fiber)
            .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    } else {
        *(*fiber).ret = *value;
    }
    (*fiber).caller = (*vm).fiber;
    (*vm).fiber = fiber;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn vmYieldFiber(mut vm: *mut PKVM, mut value: *mut Var) {
    let mut caller: *mut Fiber = (*(*vm).fiber).caller;
    if !caller.is_null() {
        if value.is_null() {
            *(*caller)
                .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        } else {
            *(*caller).ret = *value;
        }
    }
    (*(*vm).fiber).caller = 0 as *mut Fiber;
    (*(*vm).fiber).state = FIBER_YIELDED;
    (*vm).fiber = caller;
}
pub unsafe extern "C" fn vmCallMethod(
    mut vm: *mut PKVM,
    mut self_0: Var,
    mut fn_0: *mut Closure,
    mut argc: libc::c_int,
    mut argv: *mut Var,
    mut ret: *mut Var,
) -> PkResult {
    let mut fiber: *mut Fiber = newFiber(vm, fn_0);
    (*fiber).self_0 = self_0;
    (*fiber).native = (*vm).fiber;
    vmPushTempRef(vm, &mut (*fiber)._super);
    let mut success: bool = vmPrepareFiber(vm, fiber, argc, argv);
    if !success {
        vmPopTempRef(vm);
        return PK_RESULT_RUNTIME_ERROR;
    }
    let mut result: PkResult = PK_RESULT_SUCCESS;
    let mut last: *mut Fiber = (*vm).fiber;
    if !last.is_null() {
        vmPushTempRef(vm, &mut (*last)._super);
    }
    if (*(*(*fiber).closure).fn_0).is_native {
        (*vm).fiber = fiber;
        ((*(*(*fiber).closure).fn_0).c2rust_unnamed.native).unwrap()(vm);
        if !((*(*vm).fiber).error).is_null() {
            if !last.is_null() {
                (*last).error = (*(*vm).fiber).error;
            }
            result = PK_RESULT_RUNTIME_ERROR;
        } else {
            result = PK_RESULT_SUCCESS;
        }
    } else {
        result = vmRunFiber(vm, fiber);
    }
    if !last.is_null() {
        vmPopTempRef(vm);
    }
    vmPopTempRef(vm);
    (*vm).fiber = last;
    if !ret.is_null() {
        *ret = *(*fiber).ret;
    }
    return result;
}
pub unsafe extern "C" fn vmCallFunction(
    mut vm: *mut PKVM,
    mut fn_0: *mut Closure,
    mut argc: libc::c_int,
    mut argv: *mut Var,
    mut ret: *mut Var,
) -> PkResult {
    return vmCallMethod(
        vm,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t,
        fn_0,
        argc,
        argv,
        ret,
    );
}
unsafe extern "C" fn _isPathDL(mut path: *mut String_0) -> bool {
    let mut dlext: [*const libc::c_char; 3] = [
        b".so\0" as *const u8 as *const libc::c_char,
        b".dll\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut ext: *mut *const libc::c_char = dlext.as_mut_ptr();
    while !(*ext).is_null() {
        let mut start: *const libc::c_char = ((*path).data)
            .as_mut_ptr()
            .offset(
                ((*path).length as libc::c_ulong).wrapping_sub(strlen(*ext)) as isize,
            );
        if strncmp(start, *ext, strlen(*ext)) == 0 {
            return 1 as libc::c_int != 0;
        }
        ext = ext.offset(1);
        ext;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn _importDL(
    mut vm: *mut PKVM,
    mut resolved: *mut String_0,
    mut name: *mut String_0,
) -> *mut Module {
    if ((*vm).config.import_dl_fn).is_none() {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Dynamic library importer not provided.\0" as *const u8
                as *const libc::c_char,
            if (b"Dynamic library importer not provided.\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Dynamic library importer not provided.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        return 0 as *mut Module;
    }
    let mut handle: *mut libc::c_void = ((*vm).config.load_dl_fn)
        .unwrap()(vm, ((*resolved).data).as_mut_ptr());
    if handle.is_null() {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Error loading module at \"@\"\0" as *const u8 as *const libc::c_char,
            resolved,
        );
        return 0 as *mut Module;
    }
    let mut ret_offset: uintptr_t = ((*(*vm).fiber).ret)
        .offset_from((*(*vm).fiber).stack) as libc::c_long as uintptr_t;
    (*(*vm).fiber).ret = (*(*vm).fiber).sp;
    let mut pkhandle: *mut PkHandle = ((*vm).config.import_dl_fn).unwrap()(vm, handle);
    (*(*vm).fiber).ret = ((*(*vm).fiber).stack).offset(ret_offset as isize);
    if pkhandle.is_null() {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Error loading module at \"@\"\0" as *const u8 as *const libc::c_char,
            resolved,
        );
        return 0 as *mut Module;
    }
    if !((*pkhandle).value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*(((*pkhandle).value & 0xffffffffffff as libc::c_long as uint64_t)
            as *mut Object))
            .type_0 as libc::c_uint == OBJ_MODULE as libc::c_int as libc::c_uint)
    {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Returned handle wasn't a module at \"@\"\0" as *const u8
                as *const libc::c_char,
            resolved,
        );
        return 0 as *mut Module;
    }
    let mut module: *mut Module = ((*pkhandle).value
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Module;
    (*module).name = name;
    (*module).path = resolved;
    (*module).handle = handle;
    vmRegisterModule(vm, module, resolved);
    pkReleaseHandle(vm, pkhandle);
    return module;
}
pub unsafe extern "C" fn vmUnloadDlHandle(
    mut vm: *mut PKVM,
    mut handle: *mut libc::c_void,
) {
    if !handle.is_null() && ((*vm).config.unload_dl_fn).is_some() {
        ((*vm).config.unload_dl_fn).unwrap()(vm, handle);
    }
}
unsafe extern "C" fn _importScript(
    mut vm: *mut PKVM,
    mut resolved: *mut String_0,
    mut name: *mut String_0,
) -> *mut Module {
    let mut source: *mut libc::c_char = ((*vm).config.load_script_fn)
        .unwrap()(vm, ((*resolved).data).as_mut_ptr());
    if source.is_null() {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Error loading module at \"@\"\0" as *const u8 as *const libc::c_char,
            resolved,
        );
        return 0 as *mut Module;
    }
    let mut module: *mut Module = newModule(vm);
    (*module).path = resolved;
    (*module).name = name;
    vmPushTempRef(vm, &mut (*module)._super);
    initializeModule(vm, module, 0 as libc::c_int != 0);
    let mut result: PkResult = compile(vm, module, source, 0 as *const CompileOptions);
    pkRealloc(vm, source as *mut libc::c_void, 0 as libc::c_int as size_t);
    if result as libc::c_uint == PK_RESULT_SUCCESS as libc::c_int as libc::c_uint {
        vmRegisterModule(vm, module, resolved);
    } else {
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Error compiling module at \"@\"\0" as *const u8 as *const libc::c_char,
            resolved,
        );
        module = 0 as *mut Module;
    }
    vmPopTempRef(vm);
    return module;
}
pub unsafe extern "C" fn vmImportModule(
    mut vm: *mut PKVM,
    mut from: *mut String_0,
    mut path: *mut String_0,
) -> Var {
    let mut is_relative: bool = *((*path).data)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32;
    if !is_relative {
        let mut entry: Var = mapGet(
            (*vm).modules,
            0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*path)._super as *mut Object as uintptr_t,
        );
        if !(entry
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000000 as libc::c_long as uint64_t)
        {
            return entry;
        }
    }
    let mut _resolved: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from_path: *const libc::c_char = if !from.is_null() {
        ((*from).data).as_mut_ptr()
    } else {
        0 as *mut libc::c_char
    };
    let mut search_path_idx: uint32_t = 0 as libc::c_int as uint32_t;
    loop {
        _resolved = ((*vm).config.resolve_path_fn)
            .unwrap()(vm, from_path, ((*path).data).as_mut_ptr());
        if !_resolved.is_null() {
            break;
        }
        if search_path_idx >= (*(*vm).search_paths).elements.count {
            break;
        }
        let fresh1 = search_path_idx;
        search_path_idx = search_path_idx.wrapping_add(1);
        let mut sp: Var = *((*(*vm).search_paths).elements.data).offset(fresh1 as isize);
        from_path = ((*((sp & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0))
            .data)
            .as_mut_ptr();
    }
    if _resolved.is_null() {
        pkRealloc(vm, _resolved as *mut libc::c_void, 0 as libc::c_int as size_t);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Cannot import module '@'\0" as *const u8 as *const libc::c_char,
            path,
        );
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    let mut resolved: *mut String_0 = newStringLength(
        vm,
        _resolved,
        if _resolved.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(_resolved) as uint32_t
        },
    );
    pkRealloc(vm, _resolved as *mut libc::c_void, 0 as libc::c_int as size_t);
    let mut entry_0: Var = mapGet(
        (*vm).modules,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*resolved)._super as *mut Object as uintptr_t,
    );
    if !(entry_0
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000000 as libc::c_long as uint64_t)
    {
        return entry_0;
    }
    let mut isdl: bool = _isPathDL(resolved);
    if isdl as libc::c_int != 0 && ((*vm).config.load_dl_fn).is_none()
        || ((*vm).config.load_script_fn).is_none()
    {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Cannot import. The hosting application haven't registered the module loading API\0"
                as *const u8 as *const libc::c_char,
            if (b"Cannot import. The hosting application haven't registered the module loading API\0"
                as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Cannot import. The hosting application haven't registered the module loading API\0"
                        as *const u8 as *const libc::c_char,
                ) as uint32_t
            },
        );
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    let mut module: *mut Module = 0 as *mut Module;
    vmPushTempRef(vm, &mut (*resolved)._super);
    let mut _name: *mut String_0 = newStringLength(
        vm,
        ((*path).data).as_mut_ptr(),
        (*path).length,
    );
    let mut c: *mut libc::c_char = ((*_name).data).as_mut_ptr();
    while c < ((*_name).data).as_mut_ptr().offset((*_name).length as isize) {
        if *c as libc::c_int == '/' as i32 {
            *c = '.' as i32 as libc::c_char;
        }
        c = c.offset(1);
        c;
    }
    (*_name).hash = utilHashString(((*_name).data).as_mut_ptr());
    vmPushTempRef(vm, &mut (*_name)._super);
    if isdl {
        module = _importDL(vm, resolved, _name);
    } else {
        module = _importScript(vm, resolved, _name);
    }
    vmPopTempRef(vm);
    vmPopTempRef(vm);
    if module.is_null() {
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    return 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*module)._super as *mut Object as uintptr_t;
}
pub unsafe extern "C" fn vmEnsureStackSize(
    mut vm: *mut PKVM,
    mut fiber: *mut Fiber,
    mut size: libc::c_int,
) {
    if size as libc::c_ulong
        >= ((1024 as libc::c_int * 800 as libc::c_int) as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Var>() as libc::c_ulong)
    {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Maximum stack limit reached.\0" as *const u8 as *const libc::c_char,
            if (b"Maximum stack limit reached.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Maximum stack limit reached.\0" as *const u8 as *const libc::c_char,
                ) as uint32_t
            },
        );
        return;
    }
    if (*fiber).stack_size >= size {
        return;
    }
    let mut new_size: libc::c_int = utilPowerOf2Ceil(size);
    let mut old_rbp: *mut Var = (*fiber).stack;
    (*fiber)
        .stack = vmRealloc(
        vm,
        (*fiber).stack as *mut libc::c_void,
        (::std::mem::size_of::<Var>() as libc::c_ulong)
            .wrapping_mul((*fiber).stack_size as libc::c_ulong),
        (::std::mem::size_of::<Var>() as libc::c_ulong)
            .wrapping_mul(new_size as libc::c_ulong),
    ) as *mut Var;
    (*fiber).stack_size = new_size;
    if old_rbp == (*fiber).stack {
        return;
    }
    (*fiber)
        .sp = ((*fiber).stack)
        .offset(((*fiber).sp).offset_from(old_rbp) as libc::c_long as isize);
    (*fiber)
        .ret = ((*fiber).stack)
        .offset(((*fiber).ret).offset_from(old_rbp) as libc::c_long as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*fiber).frame_count {
        let mut frame: *mut CallFrame = ((*fiber).frames).offset(i as isize);
        (*frame)
            .rbp = ((*fiber).stack)
            .offset(((*frame).rbp).offset_from(old_rbp) as libc::c_long as isize);
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn pushCallFrame(mut vm: *mut PKVM, mut closure: *const Closure) {
    if (*(*vm).fiber).frame_count + 1 as libc::c_int > (*(*vm).fiber).frame_capacity {
        let mut new_capacity: libc::c_int = (*(*vm).fiber).frame_capacity
            << 1 as libc::c_int;
        if new_capacity == 0 as libc::c_int {
            new_capacity = 1 as libc::c_int;
        }
        (*(*vm).fiber)
            .frames = vmRealloc(
            vm,
            (*(*vm).fiber).frames as *mut libc::c_void,
            (::std::mem::size_of::<CallFrame>() as libc::c_ulong)
                .wrapping_mul((*(*vm).fiber).frame_capacity as libc::c_ulong),
            (::std::mem::size_of::<CallFrame>() as libc::c_ulong)
                .wrapping_mul(new_capacity as libc::c_ulong),
        ) as *mut CallFrame;
        (*(*vm).fiber).frame_capacity = new_capacity;
    }
    let mut current_stack_slots: libc::c_int = ((*(*vm).fiber).sp)
        .offset_from((*(*vm).fiber).stack) as libc::c_long as libc::c_int
        + 1 as libc::c_int;
    let mut needed: libc::c_int = (*(*(*closure).fn_0).c2rust_unnamed.fn_0).stack_size
        + current_stack_slots;
    vmEnsureStackSize(vm, (*vm).fiber, needed);
    let fresh2 = (*(*vm).fiber).frame_count;
    (*(*vm).fiber).frame_count = (*(*vm).fiber).frame_count + 1;
    let mut frame: *mut CallFrame = ((*(*vm).fiber).frames).offset(fresh2 as isize);
    (*frame).rbp = (*(*vm).fiber).ret;
    (*frame).closure = closure;
    (*frame).ip = (*(*(*closure).fn_0).c2rust_unnamed.fn_0).opcodes.data;
    (*frame).self_0 = (*(*vm).fiber).self_0;
    (*(*vm).fiber)
        .self_0 = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
}
#[inline]
unsafe extern "C" fn reuseCallFrame(mut vm: *mut PKVM, mut closure: *const Closure) {
    let mut fb: *mut Fiber = (*vm).fiber;
    let mut frame: *mut CallFrame = ((*fb).frames)
        .offset((*fb).frame_count as isize)
        .offset(-(1 as libc::c_int as isize));
    (*frame).closure = closure;
    (*frame).ip = (*(*(*closure).fn_0).c2rust_unnamed.fn_0).opcodes.data;
    (*frame).self_0 = (*(*vm).fiber).self_0;
    (*(*vm).fiber)
        .self_0 = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
    let mut arg: *mut Var = ((*fb).sp).offset(-((*(*closure).fn_0).arity as isize));
    let mut target: *mut Var = ((*frame).rbp).offset(1 as libc::c_int as isize);
    while arg < (*fb).sp {
        *target = *arg;
        arg = arg.offset(1);
        arg;
        target = target.offset(1);
        target;
    }
    (*fb).sp = target;
    let mut needed: libc::c_int = (*(*(*closure).fn_0).c2rust_unnamed.fn_0).stack_size
        + ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).stack) as libc::c_long
            as libc::c_int;
    vmEnsureStackSize(vm, (*vm).fiber, needed);
}
unsafe extern "C" fn captureUpvalue(
    mut vm: *mut PKVM,
    mut fiber: *mut Fiber,
    mut local: *mut Var,
) -> *mut Upvalue {
    if ((*fiber).open_upvalues).is_null() {
        let mut upvalue: *mut Upvalue = newUpvalue(vm, local);
        (*fiber).open_upvalues = upvalue;
        return upvalue;
    }
    if (*(*fiber).open_upvalues).ptr < local {
        let mut head: *mut Upvalue = newUpvalue(vm, local);
        (*head).next = (*fiber).open_upvalues;
        (*fiber).open_upvalues = head;
        return head;
    }
    let mut last: *mut Upvalue = 0 as *mut Upvalue;
    let mut current: *mut Upvalue = (*fiber).open_upvalues;
    while (*current).ptr > local {
        last = current;
        current = (*current).next;
        if current.is_null() {
            (*last).next = newUpvalue(vm, local);
            return (*last).next;
        }
    }
    if (*current).ptr == local {
        return current;
    }
    let mut upvalue_0: *mut Upvalue = newUpvalue(vm, local);
    (*last).next = upvalue_0;
    (*upvalue_0).next = current;
    return upvalue_0;
}
unsafe extern "C" fn closeUpvalues(mut fiber: *mut Fiber, mut top: *mut Var) {
    while !((*fiber).open_upvalues).is_null() && (*(*fiber).open_upvalues).ptr >= top {
        let mut upvalue: *mut Upvalue = (*fiber).open_upvalues;
        (*upvalue).closed = *(*upvalue).ptr;
        (*upvalue).ptr = &mut (*upvalue).closed;
        (*fiber).open_upvalues = (*upvalue).next;
    }
}
unsafe extern "C" fn vmReportError(mut vm: *mut PKVM) {
    if ((*vm).config.stderr_write).is_none() {
        return;
    }
    reportRuntimeError(vm, (*vm).fiber);
}
pub unsafe extern "C" fn vmRunFiber(
    mut vm: *mut PKVM,
    mut fiber_: *mut Fiber,
) -> PkResult {
    let mut argc: uint8_t = 0;
    let mut callable: Var = 0;
    let mut closure_1: *const Closure = 0 as *const Closure;
    let mut index_13: uint16_t = 0;
    let mut name_0: *mut String_0 = 0 as *mut String_0;
    let mut super_method: *mut Closure = 0 as *mut Closure;
    let mut instruction: Opcode = OP_PUSH_CONSTANT;
    let mut value_0: *mut Var = 0 as *mut Var;
    let mut iterator: *mut Var = 0 as *mut Var;
    let mut seq_0: Var = 0;
    let mut jump_offset: uint16_t = 0;
    let mut it: libc::c_double = 0.;
    let mut obj: *mut Object = 0 as *mut Object;
    let mut current_block: u64;
    (*vm).fiber = fiber_;
    (*fiber_).state = FIBER_RUNNING;
    let mut ip: *const uint8_t = 0 as *const uint8_t;
    let mut rbp: *mut Var = 0 as *mut Var;
    let mut self_0: *mut Var = 0 as *mut Var;
    let mut frame: *mut CallFrame = 0 as *mut CallFrame;
    let mut module: *mut Module = 0 as *mut Module;
    let mut fiber: *mut Fiber = fiber_;
    frame = &mut *((*fiber).frames)
        .offset(((*fiber).frame_count - 1 as libc::c_int) as isize) as *mut CallFrame;
    ip = (*frame).ip;
    rbp = (*frame).rbp;
    self_0 = &mut (*frame).self_0;
    module = (*(*(*frame).closure).fn_0).owner;
    '_L_vm_main_loop: loop {
        loop {
            instruction = OP_PUSH_CONSTANT;
            let fresh3 = ip;
            ip = ip.offset(1);
            instruction = *fresh3 as Opcode;
            match instruction as libc::c_uint {
                0 => {
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut index: uint16_t = ((*ip.offset(-(2 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    let fresh4 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh4 = *((*module).constants.data).offset(index as isize);
                    continue '_L_vm_main_loop;
                }
                1 => {
                    let fresh5 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh5 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0 as libc::c_int as uint64_t;
                    continue '_L_vm_main_loop;
                }
                2 => {
                    let fresh6 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh6 = doubleToVar(0 as libc::c_int as libc::c_double);
                    continue '_L_vm_main_loop;
                }
                3 => {
                    let fresh7 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh7 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x1000000000003 as libc::c_long as uint64_t;
                    continue '_L_vm_main_loop;
                }
                4 => {
                    let fresh8 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh8 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x1000000000002 as libc::c_long as uint64_t;
                    continue '_L_vm_main_loop;
                }
                5 => {
                    let mut tmp: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int as isize));
                    *((*fiber).sp)
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = *((*fiber).sp).offset(-(2 as libc::c_int as isize));
                    *((*fiber).sp).offset(-(2 as libc::c_int as isize)) = tmp;
                    continue '_L_vm_main_loop;
                }
                6 => {
                    let fresh9 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh9 = *((*fiber).sp).offset(-(1 as libc::c_int as isize));
                    continue '_L_vm_main_loop;
                }
                7 => {
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut list: *mut List = newList(
                        vm,
                        ((*ip.offset(-(2 as libc::c_int) as isize) as libc::c_int)
                            << 8 as libc::c_int
                            | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                            as uint16_t as uint32_t,
                    );
                    let fresh10 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh10 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*list)._super as *mut Object as uintptr_t;
                    continue '_L_vm_main_loop;
                }
                8 => {
                    let mut map: *mut Map = newMap(vm);
                    let fresh11 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh11 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*map)._super as *mut Object as uintptr_t;
                    continue '_L_vm_main_loop;
                }
                9 => {
                    let fresh12 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh12 = *self_0;
                    continue '_L_vm_main_loop;
                }
                10 => {
                    let mut elem: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut list_0: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    pkVarBufferWrite(
                        &mut (*((list_0 & 0xffffffffffff as libc::c_long as uint64_t)
                            as *mut Object as *mut List))
                            .elements,
                        vm,
                        elem,
                    );
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    continue '_L_vm_main_loop;
                }
                11 => {
                    let mut value: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut key: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut on: Var = *((*fiber).sp)
                        .offset(-(3 as libc::c_int) as isize);
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
                            b"$ type is not hashable.\0" as *const u8
                                as *const libc::c_char,
                            varTypeName(key),
                        );
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    mapSet(
                        vm,
                        (on & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                            as *mut Map,
                        key,
                        value,
                    );
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    continue '_L_vm_main_loop;
                }
                12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
                    let mut index_0: libc::c_int = (instruction as libc::c_uint)
                        .wrapping_sub(OP_PUSH_LOCAL_0 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    let fresh13 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh13 = *rbp.offset((index_0 + 1 as libc::c_int) as isize);
                    continue '_L_vm_main_loop;
                }
                21 => {
                    let fresh14 = ip;
                    ip = ip.offset(1);
                    let mut index_1: uint8_t = *fresh14;
                    let fresh15 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh15 = *rbp
                        .offset((index_1 as libc::c_int + 1 as libc::c_int) as isize);
                    continue '_L_vm_main_loop;
                }
                22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => {
                    let mut index_2: libc::c_int = (instruction as libc::c_uint)
                        .wrapping_sub(OP_STORE_LOCAL_0 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    *rbp
                        .offset(
                            (index_2 + 1 as libc::c_int) as isize,
                        ) = *((*fiber).sp).offset(-(1 as libc::c_int) as isize);
                    continue '_L_vm_main_loop;
                }
                31 => {
                    let fresh16 = ip;
                    ip = ip.offset(1);
                    let mut index_3: uint8_t = *fresh16;
                    *rbp
                        .offset(
                            (index_3 as libc::c_int + 1 as libc::c_int) as isize,
                        ) = *((*fiber).sp).offset(-(1 as libc::c_int) as isize);
                    continue '_L_vm_main_loop;
                }
                32 => {
                    let fresh17 = ip;
                    ip = ip.offset(1);
                    let mut index_4: uint8_t = *fresh17;
                    let fresh18 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh18 = *((*module).globals.data).offset(index_4 as isize);
                    continue '_L_vm_main_loop;
                }
                33 => {
                    let fresh19 = ip;
                    ip = ip.offset(1);
                    let mut index_5: uint8_t = *fresh19;
                    *((*module).globals.data)
                        .offset(
                            index_5 as isize,
                        ) = *((*fiber).sp).offset(-(1 as libc::c_int) as isize);
                    continue '_L_vm_main_loop;
                }
                34 => {
                    let fresh20 = ip;
                    ip = ip.offset(1);
                    let mut index_6: uint8_t = *fresh20;
                    let mut closure: *mut Closure = (*vm)
                        .builtins_funcs[index_6 as usize];
                    let fresh21 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh21 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*closure)._super as *mut Object as uintptr_t;
                    continue '_L_vm_main_loop;
                }
                35 => {
                    let fresh22 = ip;
                    ip = ip.offset(1);
                    let mut index_7: uint8_t = *fresh22;
                    let mut cls: *mut Class = (*vm).builtin_classes[index_7 as usize];
                    let fresh23 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh23 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*cls)._super as *mut Object as uintptr_t;
                    continue '_L_vm_main_loop;
                }
                36 => {
                    let fresh24 = ip;
                    ip = ip.offset(1);
                    let mut index_8: uint8_t = *fresh24;
                    let fresh25 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh25 = *(**((*(*frame).closure).upvalues)
                        .as_ptr()
                        .offset(index_8 as isize))
                        .ptr;
                    continue '_L_vm_main_loop;
                }
                37 => {
                    let fresh26 = ip;
                    ip = ip.offset(1);
                    let mut index_9: uint8_t = *fresh26;
                    *(**((*(*frame).closure).upvalues).as_ptr().offset(index_9 as isize))
                        .ptr = *((*fiber).sp).offset(-(1 as libc::c_int) as isize);
                    continue '_L_vm_main_loop;
                }
                38 => {
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut index_10: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    let mut fn_0: *mut Function = (*((*module).constants.data)
                        .offset(index_10 as isize)
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Function;
                    let mut closure_0: *mut Closure = newClosure(vm, fn_0);
                    vmPushTempRef(vm, &mut (*closure_0)._super);
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i < (*fn_0).upvalue_count {
                        let fresh27 = ip;
                        ip = ip.offset(1);
                        let mut is_immediate: uint8_t = *fresh27;
                        let fresh28 = ip;
                        ip = ip.offset(1);
                        let mut idx: uint8_t = *fresh28;
                        if is_immediate != 0 {
                            let ref mut fresh29 = *((*closure_0).upvalues)
                                .as_mut_ptr()
                                .offset(i as isize);
                            *fresh29 = captureUpvalue(
                                vm,
                                fiber,
                                rbp
                                    .offset(1 as libc::c_int as isize)
                                    .offset(idx as libc::c_int as isize),
                            );
                        } else {
                            let ref mut fresh30 = *((*closure_0).upvalues)
                                .as_mut_ptr()
                                .offset(i as isize);
                            *fresh30 = *((*(*frame).closure).upvalues)
                                .as_ptr()
                                .offset(idx as isize);
                        }
                        i += 1;
                        i;
                    }
                    let fresh31 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh31 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*closure_0)._super as *mut Object as uintptr_t;
                    vmPopTempRef(vm);
                    continue '_L_vm_main_loop;
                }
                39 => {
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    let mut cls_0: Var = *(*fiber).sp;
                    if !(cls_0
                        & (0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong)
                        == 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                        && (*((cls_0 & 0xffffffffffff as libc::c_long as uint64_t)
                            as *mut Object))
                            .type_0 as libc::c_uint
                            == OBJ_CLASS as libc::c_int as libc::c_uint)
                    {
                        (*(*vm).fiber)
                            .error = newStringLength(
                            vm,
                            b"Cannot inherit a non class object.\0" as *const u8
                                as *const libc::c_char,
                            if (b"Cannot inherit a non class object.\0" as *const u8
                                as *const libc::c_char)
                                .is_null()
                            {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                strlen(
                                    b"Cannot inherit a non class object.\0" as *const u8
                                        as *const libc::c_char,
                                ) as uint32_t
                            },
                        );
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_0: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_0;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    let mut base: *mut Class = (cls_0
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Class;
                    if (*base).class_of as libc::c_uint
                        != PK_INSTANCE as libc::c_int as libc::c_uint
                        && (*base).class_of as libc::c_uint
                            != PK_OBJECT as libc::c_int as libc::c_uint
                    {
                        (*(*vm).fiber)
                            .error = stringFormat(
                            vm,
                            b"$ type cannot be inherited.\0" as *const u8
                                as *const libc::c_char,
                            getPkVarTypeName((*base).class_of),
                        );
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_1: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_1;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut index_11: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    let mut drived: *mut Class = (*((*module).constants.data)
                        .offset(index_11 as isize)
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Class;
                    (*drived).super_class = base;
                    let fresh32 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh32 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*drived)._super as *mut Object as uintptr_t;
                    continue '_L_vm_main_loop;
                }
                40 => {
                    let mut method: *mut Closure = (*((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize)
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Closure;
                    let mut cls_1: *mut Class = (*((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize)
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Class;
                    if strcmp(
                        (*(*method).fn_0).name,
                        b"_init\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        (*cls_1).ctor = method;
                    }
                    pkClosureBufferWrite(&mut (*cls_1).methods, vm, method);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    continue '_L_vm_main_loop;
                }
                41 => {
                    closeUpvalues(
                        fiber,
                        ((*fiber).sp).offset(-(1 as libc::c_int as isize)),
                    );
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    continue '_L_vm_main_loop;
                }
                42 => {
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    continue '_L_vm_main_loop;
                }
                43 => {
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut index_12: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    let mut name: *mut String_0 = moduleGetStringAt(
                        module,
                        index_12 as libc::c_int,
                    );
                    let mut _imported: Var = vmImportModule(vm, (*module).path, name);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_2: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_2;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    let fresh33 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh33 = _imported;
                    let mut imported: *mut Module = (_imported
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Module;
                    if !(*imported).initialized {
                        (*imported).initialized = 1 as libc::c_int != 0;
                        (*frame).ip = ip;
                        (*fiber)
                            .ret = ((*fiber).sp).offset(-(1 as libc::c_int as isize));
                        pushCallFrame(vm, (*imported).body);
                        frame = &mut *((*fiber).frames)
                            .offset(((*fiber).frame_count - 1 as libc::c_int) as isize)
                            as *mut CallFrame;
                        ip = (*frame).ip;
                        rbp = (*frame).rbp;
                        self_0 = &mut (*frame).self_0;
                        module = (*(*(*frame).closure).fn_0).owner;
                        if !((*(*vm).fiber).error).is_null() {
                            (*frame).ip = ip;
                            vmReportError(vm);
                            let mut caller_3: *mut Fiber = (*fiber).caller;
                            (*fiber).state = FIBER_DONE;
                            (*fiber).caller = 0 as *mut Fiber;
                            fiber = caller_3;
                            (*vm).fiber = fiber;
                            return PK_RESULT_RUNTIME_ERROR;
                        }
                    }
                    continue '_L_vm_main_loop;
                }
                44 => {
                    let fresh34 = ip;
                    ip = ip.offset(1);
                    argc = *fresh34;
                    (*fiber)
                        .ret = ((*fiber).sp)
                        .offset(-(argc as libc::c_int as isize))
                        .offset(-(1 as libc::c_int as isize));
                    (*fiber).self_0 = *(*fiber).ret;
                    ip = ip.offset(2 as libc::c_int as isize);
                    index_13 = ((*ip.offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    name_0 = moduleGetStringAt(module, index_13 as libc::c_int);
                    super_method = getSuperMethod(vm, (*fiber).self_0, name_0);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_4: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_4;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    callable = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*super_method)._super as *mut Object as uintptr_t;
                    current_block = 3334301343165654039;
                    break;
                }
                45 => {
                    let fresh35 = ip;
                    ip = ip.offset(1);
                    argc = *fresh35;
                    (*fiber)
                        .ret = ((*fiber).sp)
                        .offset(-(argc as libc::c_int as isize))
                        .offset(-(1 as libc::c_int as isize));
                    (*fiber).self_0 = *(*fiber).ret;
                    ip = ip.offset(2 as libc::c_int as isize);
                    index_13 = ((*ip.offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    name_0 = moduleGetStringAt(module, index_13 as libc::c_int);
                    callable = getMethod(vm, (*fiber).self_0, name_0, 0 as *mut bool);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_5: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_5;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    current_block = 3334301343165654039;
                    break;
                }
                46 | 47 => {
                    let fresh36 = ip;
                    ip = ip.offset(1);
                    argc = *fresh36;
                    (*fiber)
                        .ret = ((*fiber).sp)
                        .offset(-(argc as libc::c_int as isize))
                        .offset(-(1 as libc::c_int as isize));
                    callable = *(*fiber).ret;
                    current_block = 3334301343165654039;
                    break;
                }
                48 => {
                    let mut seq: Var = *((*fiber).sp)
                        .offset(-(3 as libc::c_int) as isize);
                    if !(seq
                        & (0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong)
                        == 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong)
                    {
                        if seq
                            == 0x7ffc000000000000 as libc::c_long as uint64_t
                                | 0 as libc::c_int as uint64_t
                        {
                            (*(*vm).fiber)
                                .error = newStringLength(
                                vm,
                                b"Null is not iterable.\0" as *const u8
                                    as *const libc::c_char,
                                if (b"Null is not iterable.\0" as *const u8
                                    as *const libc::c_char)
                                    .is_null()
                                {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    strlen(
                                        b"Null is not iterable.\0" as *const u8
                                            as *const libc::c_char,
                                    ) as uint32_t
                                },
                            );
                            (*frame).ip = ip;
                            vmReportError(vm);
                            let mut caller_15: *mut Fiber = (*fiber).caller;
                            (*fiber).state = FIBER_DONE;
                            (*fiber).caller = 0 as *mut Fiber;
                            fiber = caller_15;
                            (*vm).fiber = fiber;
                            return PK_RESULT_RUNTIME_ERROR;
                        } else if seq
                            == 0x7ffc000000000000 as libc::c_long as uint64_t
                                | 0x1000000000003 as libc::c_long as uint64_t
                            || seq
                                == 0x7ffc000000000000 as libc::c_long as uint64_t
                                    | 0x1000000000002 as libc::c_long as uint64_t
                        {
                            (*(*vm).fiber)
                                .error = newStringLength(
                                vm,
                                b"Boolenan is not iterable.\0" as *const u8
                                    as *const libc::c_char,
                                if (b"Boolenan is not iterable.\0" as *const u8
                                    as *const libc::c_char)
                                    .is_null()
                                {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    strlen(
                                        b"Boolenan is not iterable.\0" as *const u8
                                            as *const libc::c_char,
                                    ) as uint32_t
                                },
                            );
                            (*frame).ip = ip;
                            vmReportError(vm);
                            let mut caller_16: *mut Fiber = (*fiber).caller;
                            (*fiber).state = FIBER_DONE;
                            (*fiber).caller = 0 as *mut Fiber;
                            fiber = caller_16;
                            (*vm).fiber = fiber;
                            return PK_RESULT_RUNTIME_ERROR;
                        } else if seq & 0x7ffc000000000000 as libc::c_long as uint64_t
                            != 0x7ffc000000000000 as libc::c_long as uint64_t
                        {
                            (*(*vm).fiber)
                                .error = newStringLength(
                                vm,
                                b"Number is not iterable.\0" as *const u8
                                    as *const libc::c_char,
                                if (b"Number is not iterable.\0" as *const u8
                                    as *const libc::c_char)
                                    .is_null()
                                {
                                    0 as libc::c_int as libc::c_uint
                                } else {
                                    strlen(
                                        b"Number is not iterable.\0" as *const u8
                                            as *const libc::c_char,
                                    ) as uint32_t
                                },
                            );
                            (*frame).ip = ip;
                            vmReportError(vm);
                            let mut caller_17: *mut Fiber = (*fiber).caller;
                            (*fiber).state = FIBER_DONE;
                            (*fiber).caller = 0 as *mut Fiber;
                            fiber = caller_17;
                            (*vm).fiber = fiber;
                            return PK_RESULT_RUNTIME_ERROR;
                        } else {
                            unreachable!();
                        }
                    }
                    continue '_L_vm_main_loop;
                }
                49 => {
                    value_0 = ((*fiber).sp).offset(-(1 as libc::c_int as isize));
                    iterator = ((*fiber).sp).offset(-(2 as libc::c_int as isize));
                    seq_0 = *((*fiber).sp).offset(-(3 as libc::c_int) as isize);
                    ip = ip.offset(2 as libc::c_int as isize);
                    jump_offset = ((*ip.offset(-(2 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    it = varToDouble(*iterator);
                    obj = (seq_0 & 0xffffffffffff as libc::c_long as uint64_t)
                        as *mut Object;
                    match (*obj).type_0 as libc::c_uint {
                        0 => {
                            current_block = 11133866692293981515;
                            break;
                        }
                        1 => {
                            current_block = 4236685547385934669;
                            break;
                        }
                        2 => {
                            current_block = 2476309054306085207;
                            break;
                        }
                        3 => {
                            current_block = 393118887314444155;
                            break;
                        }
                        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
                            current_block = 5671151596416324312;
                            break;
                        }
                        _ => {
                            current_block = 992178513905314417;
                            break;
                        }
                    }
                }
                50 => {
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut offset: uint16_t = ((*ip.offset(-(2 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    ip = ip.offset(offset as libc::c_int as isize);
                    continue '_L_vm_main_loop;
                }
                51 => {
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut offset_0: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    ip = ip.offset(-(offset_0 as libc::c_int as isize));
                    continue '_L_vm_main_loop;
                }
                52 => {
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    let mut cond: Var = *(*fiber).sp;
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut offset_1: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    if toBool(cond) {
                        ip = ip.offset(offset_1 as libc::c_int as isize);
                    }
                    continue '_L_vm_main_loop;
                }
                53 => {
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    let mut cond_0: Var = *(*fiber).sp;
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut offset_2: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    if !toBool(cond_0) {
                        ip = ip.offset(offset_2 as libc::c_int as isize);
                    }
                    continue '_L_vm_main_loop;
                }
                54 => {
                    let mut cond_1: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut offset_3: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    if toBool(cond_1) {
                        ip = ip.offset(offset_3 as libc::c_int as isize);
                    } else {
                        (*fiber).sp = ((*fiber).sp).offset(-1);
                        (*fiber).sp;
                    }
                    continue '_L_vm_main_loop;
                }
                55 => {
                    let mut cond_2: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut offset_4: uint16_t = ((*ip
                        .offset(-(2 as libc::c_int) as isize) as libc::c_int)
                        << 8 as libc::c_int
                        | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                        as uint16_t;
                    if !toBool(cond_2) {
                        ip = ip.offset(offset_4 as libc::c_int as isize);
                    } else {
                        (*fiber).sp = ((*fiber).sp).offset(-1);
                        (*fiber).sp;
                    }
                    continue '_L_vm_main_loop;
                }
                56 => {
                    closeUpvalues(fiber, rbp.offset(1 as libc::c_int as isize));
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    let mut ret_value: Var = *(*fiber).sp;
                    (*fiber).frame_count -= 1;
                    if (*fiber).frame_count == 0 as libc::c_int {
                        if ((*fiber).caller).is_null() {
                            *(*fiber).ret = ret_value;
                            return PK_RESULT_SUCCESS;
                        } else {
                            let mut caller_18: *mut Fiber = (*fiber).caller;
                            (*fiber).state = FIBER_DONE;
                            (*fiber).caller = 0 as *mut Fiber;
                            fiber = caller_18;
                            (*vm).fiber = fiber;
                            *(*fiber).ret = ret_value;
                        }
                    } else {
                        *rbp = ret_value;
                        (*fiber).sp = rbp.offset(1 as libc::c_int as isize);
                    }
                    frame = &mut *((*fiber).frames)
                        .offset(((*fiber).frame_count - 1 as libc::c_int) as isize)
                        as *mut CallFrame;
                    ip = (*frame).ip;
                    rbp = (*frame).rbp;
                    self_0 = &mut (*frame).self_0;
                    module = (*(*(*frame).closure).fn_0).owner;
                    continue '_L_vm_main_loop;
                }
                57 => {
                    let mut on_0: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut name_1: *mut String_0 = moduleGetStringAt(
                        module,
                        ((*ip.offset(-(2 as libc::c_int) as isize) as libc::c_int)
                            << 8 as libc::c_int
                            | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                            as uint16_t as libc::c_int,
                    );
                    let mut value_1: Var = varGetAttrib(vm, on_0, name_1);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh37 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh37 = value_1;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_19: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_19;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                58 => {
                    let mut on_1: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut name_2: *mut String_0 = moduleGetStringAt(
                        module,
                        ((*ip.offset(-(2 as libc::c_int) as isize) as libc::c_int)
                            << 8 as libc::c_int
                            | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                            as uint16_t as libc::c_int,
                    );
                    let fresh38 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh38 = varGetAttrib(vm, on_1, name_2);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_20: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_20;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                59 => {
                    let mut value_2: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut on_2: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    ip = ip.offset(2 as libc::c_int as isize);
                    let mut name_3: *mut String_0 = moduleGetStringAt(
                        module,
                        ((*ip.offset(-(2 as libc::c_int) as isize) as libc::c_int)
                            << 8 as libc::c_int
                            | *ip.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                            as uint16_t as libc::c_int,
                    );
                    varSetAttrib(vm, on_2, name_3, value_2);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh39 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh39 = value_2;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_21: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_21;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                60 => {
                    let mut key_0: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut on_3: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut value_3: Var = varGetSubscript(vm, on_3, key_0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh40 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh40 = value_3;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_22: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_22;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                61 => {
                    let mut key_1: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut on_4: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh41 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh41 = varGetSubscript(vm, on_4, key_1);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_23: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_23;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                62 => {
                    let mut value_4: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut key_2: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut on_5: Var = *((*fiber).sp)
                        .offset(-(3 as libc::c_int) as isize);
                    varsetSubscript(vm, on_5, key_2, value_4);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh42 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh42 = value_4;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_24: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_24;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                63 => {
                    let mut self_: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut result: Var = varPositive(vm, self_);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh43 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh43 = result;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_25: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_25;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                64 => {
                    let mut v: Var = *((*fiber).sp).offset(-(1 as libc::c_int) as isize);
                    let mut result_0: Var = varNegative(vm, v);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh44 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh44 = result_0;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_26: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_26;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                65 => {
                    let mut v_0: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut result_1: Var = varNot(vm, v_0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh45 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh45 = result_1;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_27: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_27;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                66 => {
                    let mut v_1: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut result_2: Var = varBitNot(vm, v_1);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh46 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh46 = result_2;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_28: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_28;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                67 => {
                    let mut r: Var = *((*fiber).sp).offset(-(1 as libc::c_int) as isize);
                    let mut l: Var = *((*fiber).sp).offset(-(2 as libc::c_int) as isize);
                    let fresh47 = ip;
                    ip = ip.offset(1);
                    let mut inplace: uint8_t = *fresh47;
                    let mut result_3: Var = varAdd(vm, l, r, inplace != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh48 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh48 = result_3;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_29: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_29;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                68 => {
                    let mut r_0: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_0: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh49 = ip;
                    ip = ip.offset(1);
                    let mut inplace_0: uint8_t = *fresh49;
                    let mut result_4: Var = varSubtract(vm, l_0, r_0, inplace_0 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh50 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh50 = result_4;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_30: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_30;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                69 => {
                    let mut r_1: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_1: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh51 = ip;
                    ip = ip.offset(1);
                    let mut inplace_1: uint8_t = *fresh51;
                    let mut result_5: Var = varMultiply(vm, l_1, r_1, inplace_1 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh52 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh52 = result_5;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_31: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_31;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                70 => {
                    let mut r_2: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_2: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh53 = ip;
                    ip = ip.offset(1);
                    let mut inplace_2: uint8_t = *fresh53;
                    let mut result_6: Var = varDivide(vm, l_2, r_2, inplace_2 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh54 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh54 = result_6;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_32: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_32;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                71 => {
                    let mut r_3: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_3: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh55 = ip;
                    ip = ip.offset(1);
                    let mut inplace_3: uint8_t = *fresh55;
                    let mut result_7: Var = varExponent(vm, l_3, r_3, inplace_3 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh56 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh56 = result_7;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_33: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_33;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                72 => {
                    let mut r_4: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_4: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh57 = ip;
                    ip = ip.offset(1);
                    let mut inplace_4: uint8_t = *fresh57;
                    let mut result_8: Var = varModulo(vm, l_4, r_4, inplace_4 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh58 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh58 = result_8;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_34: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_34;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                73 => {
                    let mut r_5: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_5: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh59 = ip;
                    ip = ip.offset(1);
                    let mut inplace_5: uint8_t = *fresh59;
                    let mut result_9: Var = varBitAnd(vm, l_5, r_5, inplace_5 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh60 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh60 = result_9;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_35: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_35;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                74 => {
                    let mut r_6: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_6: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh61 = ip;
                    ip = ip.offset(1);
                    let mut inplace_6: uint8_t = *fresh61;
                    let mut result_10: Var = varBitOr(vm, l_6, r_6, inplace_6 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh62 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh62 = result_10;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_36: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_36;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                75 => {
                    let mut r_7: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_7: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh63 = ip;
                    ip = ip.offset(1);
                    let mut inplace_7: uint8_t = *fresh63;
                    let mut result_11: Var = varBitXor(vm, l_7, r_7, inplace_7 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh64 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh64 = result_11;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_37: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_37;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                76 => {
                    let mut r_8: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_8: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh65 = ip;
                    ip = ip.offset(1);
                    let mut inplace_8: uint8_t = *fresh65;
                    let mut result_12: Var = varBitLshift(vm, l_8, r_8, inplace_8 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh66 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh66 = result_12;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_38: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_38;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                77 => {
                    let mut r_9: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_9: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let fresh67 = ip;
                    ip = ip.offset(1);
                    let mut inplace_9: uint8_t = *fresh67;
                    let mut result_13: Var = varBitRshift(vm, l_9, r_9, inplace_9 != 0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh68 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh68 = result_13;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_39: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_39;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                78 => {
                    let mut r_10: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_10: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut result_14: Var = varEqals(vm, l_10, r_10);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh69 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh69 = result_14;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_40: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_40;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                79 => {
                    let mut r_11: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_11: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut result_15: Var = varEqals(vm, l_11, r_11);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh70 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh70 = if !toBool(result_15) {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000003 as libc::c_long as uint64_t
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000002 as libc::c_long as uint64_t
                    };
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_41: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_41;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                80 => {
                    let mut r_12: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_12: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut result_16: Var = varLesser(vm, l_12, r_12);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh71 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh71 = result_16;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_42: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_42;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                81 => {
                    let mut r_13: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_13: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut result_17: Var = varLesser(vm, l_13, r_13);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_43: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_43;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    let mut lteq: bool = toBool(result_17);
                    if !lteq {
                        result_17 = varEqals(vm, l_13, r_13);
                    }
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_44: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_44;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh72 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh72 = result_17;
                    continue '_L_vm_main_loop;
                }
                82 => {
                    let mut r_14: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_14: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut result_18: Var = varGreater(vm, l_14, r_14);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh73 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh73 = result_18;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_45: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_45;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                83 => {
                    let mut r_15: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_15: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut result_19: Var = varGreater(vm, l_15, r_15);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_46: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_46;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    let mut gteq: bool = toBool(result_19);
                    if !gteq {
                        result_19 = varEqals(vm, l_15, r_15);
                    }
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_47: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_47;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh74 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh74 = result_19;
                    continue '_L_vm_main_loop;
                }
                84 => {
                    let mut r_16: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut l_16: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut result_20: Var = varOpRange(vm, l_16, r_16);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh75 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh75 = result_20;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_48: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_48;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                85 => {
                    let mut container: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut elem_0: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut contains: bool = varContains(vm, elem_0, container);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh76 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh76 = if contains as libc::c_int != 0 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000003 as libc::c_long as uint64_t
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000002 as libc::c_long as uint64_t
                    };
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_49: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_49;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                86 => {
                    let mut type_0: Var = *((*fiber).sp)
                        .offset(-(1 as libc::c_int) as isize);
                    let mut inst: Var = *((*fiber).sp)
                        .offset(-(2 as libc::c_int) as isize);
                    let mut is: bool = varIsType(vm, inst, type_0);
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(-1);
                    (*fiber).sp;
                    let fresh77 = (*fiber).sp;
                    (*fiber).sp = ((*fiber).sp).offset(1);
                    *fresh77 = if is as libc::c_int != 0 {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000003 as libc::c_long as uint64_t
                    } else {
                        0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000002 as libc::c_long as uint64_t
                    };
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_50: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_50;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    continue '_L_vm_main_loop;
                }
                87 => {
                    if ((*vm).config.stdout_write).is_some() {
                        let mut tmp_0: Var = *((*fiber).sp)
                            .offset(-(1 as libc::c_int) as isize);
                        if !(tmp_0
                            == 0x7ffc000000000000 as libc::c_long as uint64_t
                                | 0 as libc::c_int as uint64_t)
                        {
                            ((*vm).config.stdout_write)
                                .unwrap()(
                                vm,
                                ((*varToString(vm, tmp_0, 1 as libc::c_int != 0)).data)
                                    .as_mut_ptr(),
                            );
                            ((*vm).config.stdout_write)
                                .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
                        }
                    }
                }
                88 => {
                    unreachable!();
                }
                _ => {
                    unreachable!();
                }
            }
        }
        match current_block {
            5671151596416324312 => {
                if 0 as libc::c_int == 0 {
                    fprintf(
                        stderr,
                        b"Assertion failed: %s\n\tat %s() (%s:%i)\n\tcondition: %s\0"
                            as *const u8 as *const libc::c_char,
                        b"TODO: It hasn't implemented yet.\0" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<
                            &[u8; 11],
                            &[libc::c_char; 11],
                        >(b"vmRunFiber\0"))
                            .as_ptr(),
                        b"src/core/vm.c\0" as *const u8 as *const libc::c_char,
                        1446 as libc::c_int,
                        b"false\0" as *const u8 as *const libc::c_char,
                    );
                    abort();
                }
            }
            2476309054306085207 => {
                let mut iter_1: uint32_t = trunc(it) as int32_t as uint32_t;
                let mut map_0: *mut Map = obj as *mut Map;
                if ((*map_0).entries).is_null() {
                    ip = ip.offset(jump_offset as libc::c_int as isize);
                } else {
                    let mut e: *mut MapEntry = ((*map_0).entries)
                        .offset(iter_1 as isize);
                    while iter_1 < (*map_0).capacity {
                        if !((*e).key
                            == 0x7ffc000000000000 as libc::c_long as uint64_t
                                | 0x1000000000000 as libc::c_long as uint64_t)
                        {
                            break;
                        }
                        iter_1 = iter_1.wrapping_add(1);
                        iter_1;
                        e = e.offset(1);
                        e;
                    }
                    if iter_1 >= (*map_0).capacity {
                        ip = ip.offset(jump_offset as libc::c_int as isize);
                    } else {
                        *value_0 = (*((*map_0).entries).offset(iter_1 as isize)).key;
                        *iterator = doubleToVar(
                            iter_1 as libc::c_double + 1 as libc::c_int as libc::c_double,
                        );
                    }
                }
            }
            4236685547385934669 => {
                let mut iter_0: uint32_t = trunc(it) as int32_t as uint32_t;
                let mut elems: *mut pkVarBuffer = &mut (*(obj as *mut List)).elements;
                if iter_0 >= (*elems).count {
                    ip = ip.offset(jump_offset as libc::c_int as isize);
                } else {
                    *value_0 = *((*elems).data).offset(iter_0 as isize);
                    *iterator = doubleToVar(
                        iter_0 as libc::c_double + 1 as libc::c_int as libc::c_double,
                    );
                }
            }
            11133866692293981515 => {
                let mut iter: uint32_t = trunc(it) as int32_t as uint32_t;
                let mut str: *mut String_0 = obj as *mut String_0;
                if iter >= (*str).length {
                    ip = ip.offset(jump_offset as libc::c_int as isize);
                } else {
                    *value_0 = 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*(newStringLength
                            as unsafe extern "C" fn(
                                *mut PKVM,
                                *const libc::c_char,
                                uint32_t,
                            ) -> *mut String_0)(
                            vm,
                            ((*str).data).as_mut_ptr().offset(iter as isize),
                            1 as libc::c_int as uint32_t,
                        ))
                            ._super as *mut Object as uintptr_t;
                    *iterator = doubleToVar(
                        iter as libc::c_double + 1 as libc::c_int as libc::c_double,
                    );
                }
            }
            3334301343165654039 => {
                *(*fiber)
                    .ret = 0x7ffc000000000000 as libc::c_long as uint64_t
                    | 0 as libc::c_int as uint64_t;
                if callable
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                    && (*((callable & 0xffffffffffff as libc::c_long as uint64_t)
                        as *mut Object))
                        .type_0 as libc::c_uint
                        == OBJ_CLOSURE as libc::c_int as libc::c_uint
                {
                    closure_1 = (callable & 0xffffffffffff as libc::c_long as uint64_t)
                        as *mut Object as *const Closure;
                } else if callable
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                    && (*((callable & 0xffffffffffff as libc::c_long as uint64_t)
                        as *mut Object))
                        .type_0 as libc::c_uint
                        == OBJ_METHOD_BIND as libc::c_int as libc::c_uint
                {
                    let mut mb: *const MethodBind = (callable
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *const MethodBind;
                    if (*mb).instance
                        == 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000000 as libc::c_long as uint64_t
                    {
                        (*(*vm).fiber)
                            .error = newStringLength(
                            vm,
                            b"Cannot call an unbound method.\0" as *const u8
                                as *const libc::c_char,
                            if (b"Cannot call an unbound method.\0" as *const u8
                                as *const libc::c_char)
                                .is_null()
                            {
                                0 as libc::c_int as libc::c_uint
                            } else {
                                strlen(
                                    b"Cannot call an unbound method.\0" as *const u8
                                        as *const libc::c_char,
                                ) as uint32_t
                            },
                        );
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_6: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_6;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    (*fiber).self_0 = (*mb).instance;
                    closure_1 = (*mb).method;
                } else if callable
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                    && (*((callable & 0xffffffffffff as libc::c_long as uint64_t)
                        as *mut Object))
                        .type_0 as libc::c_uint
                        == OBJ_CLASS as libc::c_int as libc::c_uint
                {
                    let mut cls_2: *mut Class = (callable
                        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                        as *mut Class;
                    (*fiber).self_0 = preConstructSelf(vm, cls_2);
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_8: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_8;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    *(*fiber).ret = (*fiber).self_0;
                    closure_1 = (*cls_2).ctor as *const Closure;
                    while closure_1.is_null() {
                        cls_2 = (*cls_2).super_class;
                        if cls_2.is_null() {
                            break;
                        }
                        closure_1 = (*cls_2).ctor;
                    }
                    if closure_1.is_null() {
                        if argc as libc::c_int != 0 as libc::c_int {
                            let mut msg: *mut String_0 = stringFormat(
                                vm,
                                b"Expected exactly 0 argument(s) for constructor $.\0"
                                    as *const u8 as *const libc::c_char,
                                ((*(*cls_2).name).data).as_mut_ptr(),
                            );
                            (*(*vm).fiber).error = msg;
                            (*frame).ip = ip;
                            vmReportError(vm);
                            let mut caller_9: *mut Fiber = (*fiber).caller;
                            (*fiber).state = FIBER_DONE;
                            (*fiber).caller = 0 as *mut Fiber;
                            fiber = caller_9;
                            (*vm).fiber = fiber;
                            return PK_RESULT_RUNTIME_ERROR;
                        }
                        (*fiber)
                            .self_0 = 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x1000000000000 as libc::c_long as uint64_t;
                        continue;
                    }
                } else {
                    (*(*vm).fiber)
                        .error = stringFormat(
                        vm,
                        b"$ '$'.\0" as *const u8 as *const libc::c_char,
                        b"Expected a callable to call, instead got\0" as *const u8
                            as *const libc::c_char,
                        varTypeName(callable),
                    );
                    (*frame).ip = ip;
                    vmReportError(vm);
                    let mut caller_10: *mut Fiber = (*fiber).caller;
                    (*fiber).state = FIBER_DONE;
                    (*fiber).caller = 0 as *mut Fiber;
                    fiber = caller_10;
                    (*vm).fiber = fiber;
                    return PK_RESULT_RUNTIME_ERROR;
                }
                if (*(*closure_1).fn_0).arity != -(1 as libc::c_int)
                    && (*(*closure_1).fn_0).arity != argc as libc::c_int
                {
                    let mut buff: [libc::c_char; 12] = [0; 12];
                    sprintf(
                        buff.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        (*(*closure_1).fn_0).arity,
                    );
                    let mut msg_0: *mut String_0 = stringFormat(
                        vm,
                        b"Expected exactly $ argument(s) for function $\0" as *const u8
                            as *const libc::c_char,
                        buff.as_mut_ptr(),
                        (*(*closure_1).fn_0).name,
                    );
                    (*(*vm).fiber).error = msg_0;
                    (*frame).ip = ip;
                    vmReportError(vm);
                    let mut caller_11: *mut Fiber = (*fiber).caller;
                    (*fiber).state = FIBER_DONE;
                    (*fiber).caller = 0 as *mut Fiber;
                    fiber = caller_11;
                    (*vm).fiber = fiber;
                    return PK_RESULT_RUNTIME_ERROR;
                }
                if (*(*closure_1).fn_0).is_native {
                    if ((*(*closure_1).fn_0).c2rust_unnamed.native).is_none() {
                        (*(*vm).fiber)
                            .error = stringFormat(
                            vm,
                            b"Native function pointer of $ was NULL.\0" as *const u8
                                as *const libc::c_char,
                            (*(*closure_1).fn_0).name,
                        );
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_12: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_12;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                    (*frame).ip = ip;
                    ((*(*closure_1).fn_0).c2rust_unnamed.native).unwrap()(vm);
                    if ((*vm).fiber).is_null() {
                        return PK_RESULT_SUCCESS;
                    }
                    (*fiber).sp = ((*fiber).ret).offset(1 as libc::c_int as isize);
                    if (*vm).fiber != fiber {
                        fiber = (*vm).fiber;
                        frame = &mut *((*fiber).frames)
                            .offset(((*fiber).frame_count - 1 as libc::c_int) as isize)
                            as *mut CallFrame;
                        ip = (*frame).ip;
                        rbp = (*frame).rbp;
                        self_0 = &mut (*frame).self_0;
                        module = (*(*(*frame).closure).fn_0).owner;
                    }
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_13: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_13;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                } else if instruction as libc::c_uint
                    == OP_TAIL_CALL as libc::c_int as libc::c_uint
                {
                    reuseCallFrame(vm, closure_1);
                    frame = &mut *((*fiber).frames)
                        .offset(((*fiber).frame_count - 1 as libc::c_int) as isize)
                        as *mut CallFrame;
                    ip = (*frame).ip;
                    rbp = (*frame).rbp;
                    self_0 = &mut (*frame).self_0;
                    module = (*(*(*frame).closure).fn_0).owner;
                } else {
                    (*frame).ip = ip;
                    pushCallFrame(vm, closure_1);
                    frame = &mut *((*fiber).frames)
                        .offset(((*fiber).frame_count - 1 as libc::c_int) as isize)
                        as *mut CallFrame;
                    ip = (*frame).ip;
                    rbp = (*frame).rbp;
                    self_0 = &mut (*frame).self_0;
                    module = (*(*(*frame).closure).fn_0).owner;
                    if !((*(*vm).fiber).error).is_null() {
                        (*frame).ip = ip;
                        vmReportError(vm);
                        let mut caller_14: *mut Fiber = (*fiber).caller;
                        (*fiber).state = FIBER_DONE;
                        (*fiber).caller = 0 as *mut Fiber;
                        fiber = caller_14;
                        (*vm).fiber = fiber;
                        return PK_RESULT_RUNTIME_ERROR;
                    }
                }
            }
            992178513905314417 => {
                unreachable!();
            }
            _ => {
                let mut from: libc::c_double = (*(obj as *mut Range)).from;
                let mut to: libc::c_double = (*(obj as *mut Range)).to;
                if from == to {
                    ip = ip.offset(jump_offset as libc::c_int as isize);
                } else {
                    let mut current: libc::c_double = 0.;
                    if from <= to {
                        current = from + it;
                    } else {
                        current = from - it;
                    }
                    if current == to {
                        ip = ip.offset(jump_offset as libc::c_int as isize);
                    } else {
                        *value_0 = doubleToVar(current);
                        *iterator = doubleToVar(it + 1 as libc::c_int as libc::c_double);
                    }
                }
            }
        }
    };
}
