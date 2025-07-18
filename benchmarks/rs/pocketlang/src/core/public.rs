use ::libc;
extern "C" {
    pub type Compiler;
    
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn moduleAddFunctionInternal(
        vm: *mut PKVM,
        module: *mut Module,
        name: *const libc::c_char,
        fptr: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
    fn newClass(
        vm: *mut PKVM,
        name: *const libc::c_char,
        length: libc::c_int,
        super_0: *mut Class,
        module: *mut Module,
        docstring: *const libc::c_char,
        cls_index: *mut libc::c_int,
    ) -> *mut Class;
    fn pkClosureBufferWrite(
        self_0: *mut pkClosureBuffer,
        vm: *mut PKVM,
        data: *mut Closure,
    );
    fn newModule(vm: *mut PKVM) -> *mut Module;
    fn newFiber(vm: *mut PKVM, closure: *mut Closure) -> *mut Fiber;
    fn initializeModule(vm: *mut PKVM, module: *mut Module, is_main: bool);
    fn pkByteBufferAddString(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        str: *const libc::c_char,
        length: uint32_t,
    );
    fn newStringVaArgs(
        vm: *mut PKVM,
        fmt: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> *mut String_0;
    fn stringFormat(vm: *mut PKVM, fmt: *const libc::c_char, _: ...) -> *mut String_0;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn varIsType(vm: *mut PKVM, inst: Var, type_0: Var) -> bool;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn clearerr(__stream: *mut FILE);
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn getClass(vm: *mut PKVM, instance: Var) -> *mut Class;
    fn preConstructSelf(vm: *mut PKVM, cls: *mut Class) -> Var;
    fn newRange(vm: *mut PKVM, from: libc::c_double, to: libc::c_double) -> *mut Range;
    fn listInsert(vm: *mut PKVM, self_0: *mut List, index: uint32_t, value: Var);
    fn listRemoveAt(vm: *mut PKVM, self_0: *mut List, index: uint32_t) -> Var;
    fn getMethod(
        vm: *mut PKVM,
        self_0: Var,
        name: *mut String_0,
        is_method: *mut bool,
    ) -> Var;
    fn varGetAttrib(vm: *mut PKVM, on: Var, attrib: *mut String_0) -> Var;
    fn varSetAttrib(vm: *mut PKVM, on: Var, name: *mut String_0, value: Var);
    fn newModuleInternal(vm: *mut PKVM, name: *const libc::c_char) -> *mut Module;
    fn pkVarBufferWrite(self_0: *mut pkVarBuffer, vm: *mut PKVM, data: Var);
    fn newStringLength(
        vm: *mut PKVM,
        text: *const libc::c_char,
        length: uint32_t,
    ) -> *mut String_0;
    fn newClosure(vm: *mut PKVM, fn_0: *mut Function) -> *mut Closure;
    fn newFunction(
        vm: *mut PKVM,
        name: *const libc::c_char,
        length: libc::c_int,
        owner: *mut Module,
        is_native: bool,
        docstring: *const libc::c_char,
        fn_index: *mut libc::c_int,
    ) -> *mut Function;
    fn newMap(vm: *mut PKVM) -> *mut Map;
    fn newList(vm: *mut PKVM, size: uint32_t) -> *mut List;
    fn initializeCore(vm: *mut PKVM);
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn pkByteBufferInit(self_0: *mut pkByteBuffer);
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    static mut stdin: *mut FILE;
    fn pkByteBufferWrite(self_0: *mut pkByteBuffer, vm: *mut PKVM, data: uint8_t);
    fn pkByteBufferClear(self_0: *mut pkByteBuffer, vm: *mut PKVM);
    fn moduleGetGlobalIndex(
        module: *mut Module,
        name: *const libc::c_char,
        length: uint32_t,
    ) -> libc::c_int;
    fn varToDouble(value: Var) -> libc::c_double;
    fn getPkVarTypeName(type_0: PkVarType) -> *const libc::c_char;
    fn getVarType(v: Var) -> PkVarType;
    fn doubleToVar(value: libc::c_double) -> Var;
    fn freeObject(vm: *mut PKVM, self_0: *mut Object);
    fn varHashValue(v: Var) -> uint32_t;
    fn toBool(v: Var) -> bool;
    fn newCompilerOptions() -> CompileOptions;
    fn compile(
        vm: *mut PKVM,
        module: *mut Module,
        source: *const libc::c_char,
        options: *const CompileOptions,
    ) -> PkResult;
    fn utilIsSpace(c: libc::c_char) -> bool;
    fn vmRealloc(
        vm: *mut PKVM,
        memory: *mut libc::c_void,
        old_size: size_t,
        new_size: size_t,
    ) -> *mut libc::c_void;
    fn vmNewHandle(vm: *mut PKVM, value: Var) -> *mut PkHandle;
    fn vmEnsureStackSize(vm: *mut PKVM, fiber: *mut Fiber, size: libc::c_int);
    fn vmPushTempRef(vm: *mut PKVM, obj: *mut Object);
    fn vmPopTempRef(vm: *mut PKVM);
    fn vmRegisterModule(vm: *mut PKVM, module: *mut Module, key: *mut String_0);
    fn vmPrepareFiber(
        vm: *mut PKVM,
        fiber: *mut Fiber,
        argc: libc::c_int,
        argv: *mut Var,
    ) -> bool;
    fn vmRunFiber(vm: *mut PKVM, fiber: *mut Fiber) -> PkResult;
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
    fn vmImportModule(vm: *mut PKVM, from: *mut String_0, path: *mut String_0) -> Var;
    fn registerLibs(vm: *mut PKVM);
    fn cleanupLibs(vm: *mut PKVM);
    fn pathResolveImport(
        vm: *mut PKVM,
        from: *const libc::c_char,
        path: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn osLoadDL(vm: *mut PKVM, path: *const libc::c_char) -> *mut libc::c_void;
    fn osImportDL(vm: *mut PKVM, handle: *mut libc::c_void) -> *mut PkHandle;
    fn osUnloadDL(vm: *mut PKVM, handle: *mut libc::c_void);
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
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
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
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompileOptions {
    pub debug: bool,
    pub repl_mode: bool,
}
pub type va_list = __builtin_va_list;
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
pub struct Range {
    pub _super: Object,
    pub from: libc::c_double,
    pub to: libc::c_double,
}
pub unsafe extern "C" fn pkRealloc(
    mut vm: *mut PKVM,
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return ((*vm).config.realloc_fn).unwrap()(ptr, size, (*vm).config.user_data);
}
pub unsafe extern "C" fn pkNewConfiguration() -> PkConfiguration {
    let mut config: PkConfiguration = PkConfiguration {
        realloc_fn: None,
        stderr_write: None,
        stdout_write: None,
        stdin_read: None,
        resolve_path_fn: None,
        load_script_fn: None,
        load_dl_fn: None,
        import_dl_fn: None,
        unload_dl_fn: None,
        use_ansi_escape: false,
        user_data: 0 as *mut libc::c_void,
    };
    memset(
        &mut config as *mut PkConfiguration as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PkConfiguration>() as libc::c_ulong,
    );
    config
        .realloc_fn = Some(
        defaultRealloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                size_t,
                *mut libc::c_void,
            ) -> *mut libc::c_void,
    );
    config
        .stdout_write = Some(
        stdoutWrite as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
    );
    config
        .stderr_write = Some(
        stderrWrite as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> (),
    );
    config
        .stdin_read = Some(
        stdinRead as unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_char,
    );
    config
        .resolve_path_fn = Some(
        pathResolveImport
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                *const libc::c_char,
            ) -> *mut libc::c_char,
    );
    config
        .load_dl_fn = Some(
        osLoadDL
            as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut libc::c_void,
    );
    config
        .import_dl_fn = Some(
        osImportDL as unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> *mut PkHandle,
    );
    config
        .unload_dl_fn = Some(
        osUnloadDL as unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
    );
    config
        .load_script_fn = Some(
        loadScript
            as unsafe extern "C" fn(*mut PKVM, *const libc::c_char) -> *mut libc::c_char,
    );
    return config;
}
pub unsafe extern "C" fn pkNewVM(mut config: *mut PkConfiguration) -> *mut PKVM {
    let mut default_config: PkConfiguration = pkNewConfiguration();
    if config.is_null() {
        config = &mut default_config;
    }
    let mut vm: *mut PKVM = ((*config).realloc_fn)
        .unwrap()(
        0 as *mut libc::c_void,
        ::std::mem::size_of::<PKVM>() as libc::c_ulong,
        (*config).user_data,
    ) as *mut PKVM;
    memset(
        vm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PKVM>() as libc::c_ulong,
    );
    (*vm).config = *config;
    (*vm).working_set_count = 0 as libc::c_int;
    (*vm).working_set_capacity = 8 as libc::c_int;
    (*vm)
        .working_set = ((*vm).config.realloc_fn)
        .unwrap()(
        0 as *mut libc::c_void,
        (::std::mem::size_of::<*mut Object>() as libc::c_ulong)
            .wrapping_mul((*vm).working_set_capacity as libc::c_ulong),
        0 as *mut libc::c_void,
    ) as *mut *mut Object;
    (*vm)
        .next_gc = (1024 as libc::c_int * 1024 as libc::c_int * 10 as libc::c_int)
        as size_t;
    (*vm).collecting_garbage = 0 as libc::c_int != 0;
    (*vm).min_heap_size = (1024 as libc::c_int * 1024 as libc::c_int) as size_t;
    (*vm).heap_fill_percent = 75 as libc::c_int;
    (*vm).modules = newMap(vm);
    (*vm).search_paths = newList(vm, 8 as libc::c_int as uint32_t);
    (*vm).builtins_count = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < PK_INSTANCE as libc::c_int {
        (*vm).builtin_classes[i as usize] = 0 as *mut Class;
        i += 1;
        i;
    }
    initializeCore(vm);
    registerLibs(vm);
    return vm;
}
pub unsafe extern "C" fn pkFreeVM(mut vm: *mut PKVM) {
    cleanupLibs(vm);
    let mut obj: *mut Object = (*vm).first;
    while !obj.is_null() {
        let mut next: *mut Object = (*obj).next;
        freeObject(vm, obj);
        obj = next;
    }
    (*vm)
        .working_set = ((*vm).config.realloc_fn)
        .unwrap()(
        (*vm).working_set as *mut libc::c_void,
        0 as libc::c_int as size_t,
        (*vm).config.user_data,
    ) as *mut *mut Object;
    vmRealloc(
        vm,
        vm as *mut libc::c_void,
        ::std::mem::size_of::<PKVM>() as libc::c_ulong,
        0 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn pkGetUserData(mut vm: *const PKVM) -> *mut libc::c_void {
    return (*vm).config.user_data;
}
pub unsafe extern "C" fn pkSetUserData(
    mut vm: *mut PKVM,
    mut user_data: *mut libc::c_void,
) {
    (*vm).config.user_data = user_data;
}
pub unsafe extern "C" fn pkRegisterBuiltinFn(
    mut vm: *mut PKVM,
    mut name: *const libc::c_char,
    mut fn_0: pkNativeFn,
    mut arity: libc::c_int,
    mut docstring: *const libc::c_char,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*vm).builtins_count {
        let mut bfn: *mut Closure = (*vm).builtins_funcs[i as usize];
        i += 1;
        i;
    }
    let mut fptr: *mut Function = newFunction(
        vm,
        name,
        strlen(name) as libc::c_int,
        0 as *mut Module,
        1 as libc::c_int != 0,
        docstring,
        0 as *mut libc::c_int,
    );
    vmPushTempRef(vm, &mut (*fptr)._super);
    (*fptr).c2rust_unnamed.native = fn_0;
    (*fptr).arity = arity;
    let fresh0 = (*vm).builtins_count;
    (*vm).builtins_count = (*vm).builtins_count + 1;
    (*vm).builtins_funcs[fresh0 as usize] = newClosure(vm, fptr);
    vmPopTempRef(vm);
}
pub unsafe extern "C" fn pkAddSearchPath(
    mut vm: *mut PKVM,
    mut path: *const libc::c_char,
) {
    let mut length: size_t = strlen(path);
    let mut last: libc::c_char = *path
        .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    let mut spath: *mut String_0 = newStringLength(vm, path, length as uint32_t);
    vmPushTempRef(vm, &mut (*spath)._super);
    pkVarBufferWrite(
        &mut (*(*vm).search_paths).elements,
        vm,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*spath)._super as *mut Object as uintptr_t,
    );
    vmPopTempRef(vm);
}
pub unsafe extern "C" fn pkNewModule(
    mut vm: *mut PKVM,
    mut name: *const libc::c_char,
) -> *mut PkHandle {
    let mut module: *mut Module = newModuleInternal(vm, name);
    vmPushTempRef(vm, &mut (*module)._super);
    let mut handle: *mut PkHandle = vmNewHandle(
        vm,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*module)._super as *mut Object as uintptr_t,
    );
    vmPopTempRef(vm);
    return handle;
}
pub unsafe extern "C" fn pkRegisterModule(mut vm: *mut PKVM, mut module: *mut PkHandle) {
    let mut module_: *mut Module = ((*module).value
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Module;
    vmRegisterModule(vm, module_, (*module_).name);
}
pub unsafe extern "C" fn pkModuleAddFunction(
    mut vm: *mut PKVM,
    mut module: *mut PkHandle,
    mut name: *const libc::c_char,
    mut fptr: pkNativeFn,
    mut arity: libc::c_int,
    mut docstring: *const libc::c_char,
) {
    moduleAddFunctionInternal(
        vm,
        ((*module).value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut Module,
        name,
        fptr,
        arity,
        docstring,
    );
}
pub unsafe extern "C" fn pkNewClass(
    mut vm: *mut PKVM,
    mut name: *const libc::c_char,
    mut base_class: *mut PkHandle,
    mut module: *mut PkHandle,
    mut new_fn: pkNewInstanceFn,
    mut delete_fn: pkDeleteInstanceFn,
    mut docstring: *const libc::c_char,
) -> *mut PkHandle {
    let mut super_0: *mut Class = (*vm)
        .builtin_classes[PK_OBJECT as libc::c_int as usize];
    if !base_class.is_null() {
        super_0 = ((*base_class).value & 0xffffffffffff as libc::c_long as uint64_t)
            as *mut Object as *mut Class;
    }
    let mut class_: *mut Class = newClass(
        vm,
        name,
        strlen(name) as libc::c_int,
        super_0,
        ((*module).value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut Module,
        docstring,
        0 as *mut libc::c_int,
    );
    (*class_).new_fn = new_fn;
    (*class_).delete_fn = delete_fn;
    vmPushTempRef(vm, &mut (*class_)._super);
    let mut handle: *mut PkHandle = vmNewHandle(
        vm,
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
            | &mut (*class_)._super as *mut Object as uintptr_t,
    );
    vmPopTempRef(vm);
    return handle;
}
pub unsafe extern "C" fn pkClassAddMethod(
    mut vm: *mut PKVM,
    mut cls: *mut PkHandle,
    mut name: *const libc::c_char,
    mut fptr: pkNativeFn,
    mut arity: libc::c_int,
    mut docstring: *const libc::c_char,
) {
    let mut class_: *mut Class = ((*cls).value
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Class;
    let mut fn_0: *mut Function = newFunction(
        vm,
        name,
        strlen(name) as libc::c_int,
        (*class_).owner,
        1 as libc::c_int != 0,
        docstring,
        0 as *mut libc::c_int,
    );
    vmPushTempRef(vm, &mut (*fn_0)._super);
    (*fn_0).arity = arity;
    (*fn_0).is_method = 1 as libc::c_int != 0;
    (*fn_0).c2rust_unnamed.native = fptr;
    let mut method: *mut Closure = newClosure(vm, fn_0);
    vmPopTempRef(vm);
    vmPushTempRef(vm, &mut (*method)._super);
    pkClosureBufferWrite(&mut (*class_).methods, vm, method);
    if strcmp(name, b"_init\0" as *const u8 as *const libc::c_char) == 0 {
        (*class_).ctor = method;
    }
    vmPopTempRef(vm);
}
pub unsafe extern "C" fn pkModuleAddSource(
    mut vm: *mut PKVM,
    mut module: *mut PkHandle,
    mut source: *const libc::c_char,
) {
    compile(
        vm,
        ((*module).value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut Module,
        source,
        0 as *const CompileOptions,
    );
}
pub unsafe extern "C" fn pkReleaseHandle(mut vm: *mut PKVM, mut handle: *mut PkHandle) {
    if handle == (*vm).handles {
        (*vm).handles = (*handle).next;
    }
    if !((*handle).next).is_null() {
        (*(*handle).next).prev = (*handle).prev;
    }
    if !((*handle).prev).is_null() {
        (*(*handle).prev).next = (*handle).next;
    }
    vmRealloc(
        vm,
        handle as *mut libc::c_void,
        ::std::mem::size_of::<PkHandle>() as libc::c_ulong,
        0 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn pkRunString(
    mut vm: *mut PKVM,
    mut source: *const libc::c_char,
) -> PkResult {
    let mut result: PkResult = PK_RESULT_SUCCESS;
    let mut module: *mut Module = newModule(vm);
    vmPushTempRef(vm, &mut (*module)._super);
    (*module)
        .path = newStringLength(
        vm,
        b"@(String)\0" as *const u8 as *const libc::c_char,
        if (b"@(String)\0" as *const u8 as *const libc::c_char).is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(b"@(String)\0" as *const u8 as *const libc::c_char) as uint32_t
        },
    );
    result = compile(vm, module, source, 0 as *const CompileOptions);
    if result as libc::c_uint != PK_RESULT_SUCCESS as libc::c_int as libc::c_uint {
        return result;
    }
    (*module).initialized = 1 as libc::c_int != 0;
    let mut fiber: *mut Fiber = newFiber(vm, (*module).body);
    vmPushTempRef(vm, &mut (*fiber)._super);
    vmPrepareFiber(vm, fiber, 0 as libc::c_int, 0 as *mut Var);
    vmPopTempRef(vm);
    result = vmRunFiber(vm, fiber);
    vmPopTempRef(vm);
    return result;
}
pub unsafe extern "C" fn pkRunFile(
    mut vm: *mut PKVM,
    mut path: *const libc::c_char,
) -> PkResult {
    let mut result: PkResult = PK_RESULT_SUCCESS;
    let mut module: *mut Module = 0 as *mut Module;
    let mut resolved_: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*vm).config.resolve_path_fn).is_some() {
        resolved_ = ((*vm).config.resolve_path_fn)
            .unwrap()(vm, 0 as *const libc::c_char, path);
    }
    if resolved_.is_null() {
        if ((*vm).config.stderr_write).is_some() {
            ((*vm).config.stderr_write)
                .unwrap()(
                vm,
                b"Error finding script at \"\0" as *const u8 as *const libc::c_char,
            );
            ((*vm).config.stderr_write).unwrap()(vm, path);
            ((*vm).config.stderr_write)
                .unwrap()(vm, b"\"\n\0" as *const u8 as *const libc::c_char);
        }
        return PK_RESULT_COMPILE_ERROR;
    }
    module = newModule(vm);
    vmPushTempRef(vm, &mut (*module)._super);
    let mut script_path: *mut String_0 = newStringLength(
        vm,
        resolved_,
        if resolved_.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(resolved_) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*script_path)._super);
    pkRealloc(vm, resolved_ as *mut libc::c_void, 0 as libc::c_int as size_t);
    (*module).path = script_path;
    vmPopTempRef(vm);
    initializeModule(vm, module, 1 as libc::c_int != 0);
    let mut _path: *const libc::c_char = ((*(*module).path).data).as_mut_ptr();
    let mut source: *mut libc::c_char = ((*vm).config.load_script_fn)
        .unwrap()(vm, _path);
    if source.is_null() {
        result = PK_RESULT_COMPILE_ERROR;
        if ((*vm).config.stderr_write).is_some() {
            ((*vm).config.stderr_write)
                .unwrap()(
                vm,
                b"Error loading script at \"\0" as *const u8 as *const libc::c_char,
            );
            ((*vm).config.stderr_write).unwrap()(vm, _path);
            ((*vm).config.stderr_write)
                .unwrap()(vm, b"\"\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        result = compile(vm, module, source, 0 as *const CompileOptions);
        pkRealloc(vm, source as *mut libc::c_void, 0 as libc::c_int as size_t);
    }
    if result as libc::c_uint == PK_RESULT_SUCCESS as libc::c_int as libc::c_uint {
        vmRegisterModule(vm, module, (*module).path);
    }
    vmPopTempRef(vm);
    if result as libc::c_uint != PK_RESULT_SUCCESS as libc::c_int as libc::c_uint {
        return result;
    }
    (*module).initialized = 1 as libc::c_int != 0;
    let mut fiber: *mut Fiber = newFiber(vm, (*module).body);
    vmPushTempRef(vm, &mut (*fiber)._super);
    vmPrepareFiber(vm, fiber, 0 as libc::c_int, 0 as *mut Var);
    vmPopTempRef(vm);
    return vmRunFiber(vm, fiber);
}
#[inline]
unsafe extern "C" fn isStringEmpty(mut line: *const libc::c_char) -> bool {
    let mut c: *const libc::c_char = line;
    while *c as libc::c_int != '\0' as i32 {
        if !utilIsSpace(*c) {
            return 0 as libc::c_int != 0;
        }
        c = c.offset(1);
        c;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn moduleGetMainFunction(
    mut vm: *mut PKVM,
    mut module: *mut Module,
) -> *mut Closure {
    let mut main_index: libc::c_int = moduleGetGlobalIndex(
        module,
        b"@main\0" as *const u8 as *const libc::c_char,
        strlen(b"@main\0" as *const u8 as *const libc::c_char) as uint32_t,
    );
    if main_index == -(1 as libc::c_int) {
        return 0 as *mut Closure;
    }
    let mut main_fn: Var = *((*module).globals.data).offset(main_index as isize);
    return (main_fn & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
        as *mut Closure;
}
pub unsafe extern "C" fn pkRunREPL(mut vm: *mut PKVM) -> PkResult {
    let mut printfn: pkWriteFn = (*vm).config.stdout_write;
    let mut printerrfn: pkWriteFn = (*vm).config.stderr_write;
    let mut inputfn: pkReadFn = (*vm).config.stdin_read;
    let mut result: PkResult = PK_RESULT_SUCCESS;
    let mut options: CompileOptions = newCompilerOptions();
    options.repl_mode = 1 as libc::c_int != 0;
    if inputfn.is_none() {
        if printerrfn.is_some() {
            printerrfn
                .unwrap()(
                vm,
                b"REPL failed to input.\0" as *const u8 as *const libc::c_char,
            );
        }
        return PK_RESULT_RUNTIME_ERROR;
    }
    let mut module: *mut PkHandle = pkNewModule(
        vm,
        b"@(REPL)\0" as *const u8 as *const libc::c_char,
    );
    let mut _module: *mut Module = ((*module).value
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Module;
    initializeModule(vm, _module, 1 as libc::c_int != 0);
    let mut lines: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut lines);
    let mut need_more_lines: bool = 0 as libc::c_int != 0;
    let mut done: bool = 0 as libc::c_int != 0;
    loop {
        let mut listening: *const libc::c_char = if !need_more_lines {
            b">>> \0" as *const u8 as *const libc::c_char
        } else {
            b"... \0" as *const u8 as *const libc::c_char
        };
        printfn.unwrap()(vm, listening);
        let mut line: *mut libc::c_char = inputfn.unwrap()(vm);
        if line.is_null() {
            if printerrfn.is_some() {
                printerrfn
                    .unwrap()(
                    vm,
                    b"REPL failed to input.\0" as *const u8 as *const libc::c_char,
                );
            }
            result = PK_RESULT_RUNTIME_ERROR;
            break;
        } else {
            let mut line_length: size_t = strlen(line);
            if line_length >= 1 as libc::c_int as libc::c_ulong
                && *line
                    .offset(line_length as isize)
                    .offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == -(1 as libc::c_int)
            {
                printfn.unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
                result = PK_RESULT_SUCCESS;
                pkRealloc(vm, line as *mut libc::c_void, 0 as libc::c_int as size_t);
                break;
            } else {
                if isStringEmpty(line) {
                    need_more_lines;
                    pkRealloc(vm, line as *mut libc::c_void, 0 as libc::c_int as size_t);
                } else {
                    if lines.count != 0 as libc::c_int as libc::c_uint {
                        pkByteBufferWrite(&mut lines, vm, '\n' as i32 as uint8_t);
                    }
                    pkByteBufferAddString(&mut lines, vm, line, line_length as uint32_t);
                    pkRealloc(vm, line as *mut libc::c_void, 0 as libc::c_int as size_t);
                    pkByteBufferWrite(&mut lines, vm, '\0' as i32 as uint8_t);
                    result = compile(
                        vm,
                        _module,
                        lines.data as *const libc::c_char,
                        &mut options,
                    );
                    if result as libc::c_uint
                        == PK_RESULT_UNEXPECTED_EOF as libc::c_int as libc::c_uint
                    {
                        lines
                            .count = (lines.count as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint32_t
                            as uint32_t;
                        need_more_lines = 1 as libc::c_int != 0;
                    } else {
                        need_more_lines = 0 as libc::c_int != 0;
                        pkByteBufferClear(&mut lines, vm);
                        if !(result as libc::c_uint
                            != PK_RESULT_SUCCESS as libc::c_int as libc::c_uint)
                        {
                            let mut _main: *mut Closure = moduleGetMainFunction(
                                vm,
                                _module,
                            );
                            result = vmCallFunction(
                                vm,
                                _main,
                                0 as libc::c_int,
                                0 as *mut Var,
                                0 as *mut Var,
                            );
                        }
                    }
                }
                if done {
                    break;
                }
            }
        }
    }
    pkReleaseHandle(vm, module);
    return result;
}
pub unsafe extern "C" fn pkSetRuntimeError(
    mut vm: *mut PKVM,
    mut message: *const libc::c_char,
) {
    (*(*vm).fiber)
        .error = newStringLength(
        vm,
        message,
        if message.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(message) as uint32_t
        },
    );
}
pub unsafe extern "C" fn pkSetRuntimeErrorFmt(
    mut vm: *mut PKVM,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    (*(*vm).fiber).error = newStringVaArgs(vm, fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn pkGetSelf(mut vm: *const PKVM) -> *mut libc::c_void {
    let mut inst: *mut Instance = ((*(*vm).fiber).self_0
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Instance;
    return (*inst).native;
}
pub unsafe extern "C" fn pkGetArgc(mut vm: *const PKVM) -> libc::c_int {
    return ((*(*vm).fiber).sp).offset_from((*(*vm).fiber).ret) as libc::c_long
        as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn pkCheckArgcRange(
    mut vm: *mut PKVM,
    mut argc: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> bool {
    if argc < min {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, min);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected at least %s argument(s).\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    } else if argc > max {
        let mut buff_0: [libc::c_char; 12] = [0; 12];
        sprintf(buff_0.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, max);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected at most %s argument(s).\0" as *const u8 as *const libc::c_char,
            buff_0.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkValidateSlotBool(
    mut vm: *mut PKVM,
    mut slot: libc::c_int,
    mut value: *mut bool,
) -> bool {
    let mut val: Var = *((*(*vm).fiber).ret).offset(slot as isize);
    if !(val
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
        || val
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t)
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, slot);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a '$' at slot $.\0" as *const u8 as *const libc::c_char,
            b"Boolean\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    if !value.is_null() {
        *value = val
            == 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000003 as libc::c_long as uint64_t;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkValidateSlotNumber(
    mut vm: *mut PKVM,
    mut slot: libc::c_int,
    mut value: *mut libc::c_double,
) -> bool {
    let mut val: Var = *((*(*vm).fiber).ret).offset(slot as isize);
    if !(val & 0x7ffc000000000000 as libc::c_long as uint64_t
        != 0x7ffc000000000000 as libc::c_long as uint64_t)
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, slot);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a '$' at slot $.\0" as *const u8 as *const libc::c_char,
            b"Number\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    if !value.is_null() {
        *value = varToDouble(val);
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkValidateSlotInteger(
    mut vm: *mut PKVM,
    mut slot: libc::c_int,
    mut value: *mut int32_t,
) -> bool {
    let mut n: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, slot, &mut n) {
        return 0 as libc::c_int != 0;
    }
    if floor(n) != n {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Expected an integer got float.\0" as *const u8 as *const libc::c_char,
            if (b"Expected an integer got float.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(
                    b"Expected an integer got float.\0" as *const u8
                        as *const libc::c_char,
                ) as uint32_t
            },
        );
        return 0 as libc::c_int != 0;
    }
    if !value.is_null() {
        *value = n as int32_t;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkValidateSlotString(
    mut vm: *mut PKVM,
    mut slot: libc::c_int,
    mut value: *mut *const libc::c_char,
    mut length: *mut uint32_t,
) -> bool {
    let mut val: Var = *((*(*vm).fiber).ret).offset(slot as isize);
    if !(val
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((val & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object)).type_0
            as libc::c_uint == OBJ_STRING as libc::c_int as libc::c_uint)
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, slot);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a '$' at slot $.\0" as *const u8 as *const libc::c_char,
            b"String\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    let mut str: *mut String_0 = (val & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object as *mut String_0;
    if !value.is_null() {
        *value = ((*str).data).as_mut_ptr();
    }
    if !length.is_null() {
        *length = (*str).length;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkValidateSlotType(
    mut vm: *mut PKVM,
    mut slot: libc::c_int,
    mut type_0: PkVarType,
) -> bool {
    if getVarType(*((*(*vm).fiber).ret).offset(slot as isize)) as libc::c_uint
        != type_0 as libc::c_uint
    {
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, slot);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a '$' at slot $.\0" as *const u8 as *const libc::c_char,
            getPkVarTypeName(type_0),
            buff.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkValidateSlotInstanceOf(
    mut vm: *mut PKVM,
    mut slot: libc::c_int,
    mut cls: libc::c_int,
) -> bool {
    let mut instance: Var = *((*(*vm).fiber).ret).offset(slot as isize);
    let mut class_: Var = *((*(*vm).fiber).ret).offset(cls as isize);
    if !varIsType(vm, instance, class_) {
        if !((*(*vm).fiber).error).is_null() {
            return 0 as libc::c_int != 0;
        }
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, slot);
        (*(*vm).fiber)
            .error = stringFormat(
            vm,
            b"Expected a '$' at slot $.\0" as *const u8 as *const libc::c_char,
            ((*(*((class_ & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Class))
                .name)
                .data)
                .as_mut_ptr(),
            buff.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkIsSlotInstanceOf(
    mut vm: *mut PKVM,
    mut inst: libc::c_int,
    mut cls: libc::c_int,
    mut val: *mut bool,
) -> bool {
    *val = varIsType(vm, inst as Var, cls as Var);
    return ((*(*vm).fiber).error).is_null();
}
pub unsafe extern "C" fn pkReserveSlots(mut vm: *mut PKVM, mut count: libc::c_int) {
    if ((*vm).fiber).is_null() {
        (*vm).fiber = newFiber(vm, 0 as *mut Closure);
    }
    let mut needed: libc::c_int = ((*(*vm).fiber).ret).offset_from((*(*vm).fiber).stack)
        as libc::c_long as libc::c_int + count;
    vmEnsureStackSize(vm, (*vm).fiber, needed);
}
pub unsafe extern "C" fn pkGetSlotsCount(mut vm: *mut PKVM) -> libc::c_int {
    return (*(*vm).fiber).stack_size
        - ((*(*vm).fiber).ret).offset_from((*(*vm).fiber).stack) as libc::c_long
            as libc::c_int;
}
pub unsafe extern "C" fn pkGetSlotType(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
) -> PkVarType {
    return getVarType(*((*(*vm).fiber).ret).offset(index as isize));
}
pub unsafe extern "C" fn pkGetSlotBool(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
) -> bool {
    let mut value: Var = *((*(*vm).fiber).ret).offset(index as isize);
    return toBool(value);
}
pub unsafe extern "C" fn pkGetSlotNumber(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
) -> libc::c_double {
    let mut value: Var = *((*(*vm).fiber).ret).offset(index as isize);
    return varToDouble(value);
}
pub unsafe extern "C" fn pkGetSlotString(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut length: *mut uint32_t,
) -> *const libc::c_char {
    let mut value: Var = *((*(*vm).fiber).ret).offset(index as isize);
    if !length.is_null() {
        *length = (*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0))
            .length;
    }
    return ((*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
        as *mut String_0))
        .data)
        .as_mut_ptr();
}
pub unsafe extern "C" fn pkGetSlotHandle(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
) -> *mut PkHandle {
    return vmNewHandle(vm, *((*(*vm).fiber).ret).offset(index as isize));
}
pub unsafe extern "C" fn pkGetSlotNativeInstance(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    let mut value: Var = *((*(*vm).fiber).ret).offset(index as isize);
    let mut inst: *mut Instance = (value & 0xffffffffffff as libc::c_long as uint64_t)
        as *mut Object as *mut Instance;
    return (*inst).native;
}
pub unsafe extern "C" fn pkSetSlotNull(mut vm: *mut PKVM, mut index: libc::c_int) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn pkSetSlotBool(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut value: bool,
) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = if value as libc::c_int != 0 {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000003 as libc::c_long as uint64_t
    } else {
        0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x1000000000002 as libc::c_long as uint64_t
    };
}
pub unsafe extern "C" fn pkSetSlotNumber(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut value: libc::c_double,
) {
    *((*(*vm).fiber).ret).offset(index as isize) = doubleToVar(value);
}
pub unsafe extern "C" fn pkSetSlotString(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut value: *const libc::c_char,
) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                uint32_t,
            ) -> *mut String_0)(
            vm,
            value,
            (if value.is_null() {
                0 as libc::c_int as libc::c_uint
            } else {
                (strlen
                    as unsafe extern "C" fn(*const libc::c_char) -> libc::c_ulong)(value)
                    as uint32_t
            }),
        ))
            ._super as *mut Object as uintptr_t;
}
pub unsafe extern "C" fn pkSetSlotStringLength(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut value: *const libc::c_char,
    mut length: uint32_t,
) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                uint32_t,
            ) -> *mut String_0)(vm, value, length))
            ._super as *mut Object as uintptr_t;
}
pub unsafe extern "C" fn pkSetSlotStringFmt(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringVaArgs
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                ::std::ffi::VaList,
            ) -> *mut String_0)(vm, fmt, args_0.as_va_list()))
            ._super as *mut Object as uintptr_t;
}
pub unsafe extern "C" fn pkSetSlotHandle(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut handle: *mut PkHandle,
) {
    *((*(*vm).fiber).ret).offset(index as isize) = (*handle).value;
}
pub unsafe extern "C" fn pkGetSlotHash(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
) -> uint32_t {
    let mut value: Var = *((*(*vm).fiber).ret).offset(index as isize);
    return varHashValue(value);
}
pub unsafe extern "C" fn pkSetAttribute(
    mut vm: *mut PKVM,
    mut instance: libc::c_int,
    mut name: *const libc::c_char,
    mut value: libc::c_int,
) -> bool {
    let mut sname: *mut String_0 = newStringLength(
        vm,
        name,
        if name.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(name) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*sname)._super);
    varSetAttrib(
        vm,
        *((*(*vm).fiber).ret).offset(instance as isize),
        sname,
        *((*(*vm).fiber).ret).offset(value as isize),
    );
    vmPopTempRef(vm);
    return ((*(*vm).fiber).error).is_null();
}
pub unsafe extern "C" fn pkGetAttribute(
    mut vm: *mut PKVM,
    mut instance: libc::c_int,
    mut name: *const libc::c_char,
    mut index: libc::c_int,
) -> bool {
    let mut sname: *mut String_0 = newStringLength(
        vm,
        name,
        if name.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(name) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*sname)._super);
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = varGetAttrib(vm, *((*(*vm).fiber).ret).offset(instance as isize), sname);
    vmPopTempRef(vm);
    return ((*(*vm).fiber).error).is_null();
}
unsafe extern "C" fn _newInstance(
    mut vm: *mut PKVM,
    mut cls: *mut Class,
    mut argc: libc::c_int,
    mut argv: *mut Var,
) -> Var {
    let mut instance: Var = preConstructSelf(vm, cls);
    if !((*(*vm).fiber).error).is_null() {
        return 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0 as libc::c_int as uint64_t;
    }
    if instance
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPushTempRef(
            vm,
            (instance & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object,
        );
    }
    let mut ctor: *mut Closure = (*cls).ctor;
    while ctor.is_null() {
        cls = (*cls).super_class;
        if cls.is_null() {
            break;
        }
        ctor = (*cls).ctor;
    }
    if !ctor.is_null() {
        vmCallMethod(vm, instance, ctor, argc, argv, 0 as *mut Var);
    }
    if instance
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
    {
        vmPopTempRef(vm);
    }
    return instance;
}
pub unsafe extern "C" fn pkNewInstance(
    mut vm: *mut PKVM,
    mut cls: libc::c_int,
    mut index: libc::c_int,
    mut argc: libc::c_int,
    mut argv: libc::c_int,
) -> bool {
    argc != 0 as libc::c_int;
    let mut class_: *mut Class = (*((*(*vm).fiber).ret).offset(cls as isize)
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Class;
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = _newInstance(vm, class_, argc, ((*(*vm).fiber).ret).offset(argv as isize));
    return ((*(*vm).fiber).error).is_null();
}
pub unsafe extern "C" fn pkNewRange(
    mut vm: *mut PKVM,
    mut index: libc::c_int,
    mut first: libc::c_double,
    mut last: libc::c_double,
) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newRange
            as unsafe extern "C" fn(
                *mut PKVM,
                libc::c_double,
                libc::c_double,
            ) -> *mut Range)(vm, first, last))
            ._super as *mut Object as uintptr_t;
}
pub unsafe extern "C" fn pkNewList(mut vm: *mut PKVM, mut index: libc::c_int) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newList
            as unsafe extern "C" fn(
                *mut PKVM,
                uint32_t,
            ) -> *mut List)(vm, 0 as libc::c_int as uint32_t))
            ._super as *mut Object as uintptr_t;
}
pub unsafe extern "C" fn pkNewMap(mut vm: *mut PKVM, mut index: libc::c_int) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newMap as unsafe extern "C" fn(*mut PKVM) -> *mut Map)(vm))._super
            as *mut Object as uintptr_t;
}
pub unsafe extern "C" fn pkListInsert(
    mut vm: *mut PKVM,
    mut list: libc::c_int,
    mut index: int32_t,
    mut value: libc::c_int,
) -> bool {
    let mut l: *mut List = (*((*(*vm).fiber).ret).offset(list as isize)
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
    if index < 0 as libc::c_int {
        index = ((*l).elements.count)
            .wrapping_add(index as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as int32_t;
    }
    if index < 0 as libc::c_int || index as uint32_t > (*l).elements.count {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Index out of bounds.\0" as *const u8 as *const libc::c_char,
            if (b"Index out of bounds.\0" as *const u8 as *const libc::c_char).is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Index out of bounds.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        return 0 as libc::c_int != 0;
    }
    listInsert(vm, l, index as uint32_t, *((*(*vm).fiber).ret).offset(value as isize));
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkListPop(
    mut vm: *mut PKVM,
    mut list: libc::c_int,
    mut index: int32_t,
    mut popped: libc::c_int,
) -> bool {
    popped >= 0 as libc::c_int;
    let mut l: *mut List = (*((*(*vm).fiber).ret).offset(list as isize)
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
    if index < 0 as libc::c_int {
        index = (index as libc::c_uint).wrapping_add((*l).elements.count) as int32_t
            as int32_t;
    }
    if index < 0 as libc::c_int || index as uint32_t >= (*l).elements.count {
        (*(*vm).fiber)
            .error = newStringLength(
            vm,
            b"Index out of bounds.\0" as *const u8 as *const libc::c_char,
            if (b"Index out of bounds.\0" as *const u8 as *const libc::c_char).is_null()
            {
                0 as libc::c_int as libc::c_uint
            } else {
                strlen(b"Index out of bounds.\0" as *const u8 as *const libc::c_char)
                    as uint32_t
            },
        );
        return 0 as libc::c_int != 0;
    }
    let mut p: Var = listRemoveAt(vm, l, index as uint32_t);
    if popped >= 0 as libc::c_int {
        *((*(*vm).fiber).ret).offset(popped as isize) = p;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkListLength(
    mut vm: *mut PKVM,
    mut list: libc::c_int,
) -> uint32_t {
    let mut l: *mut List = (*((*(*vm).fiber).ret).offset(list as isize)
        & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut List;
    return (*l).elements.count;
}
pub unsafe extern "C" fn pkCallFunction(
    mut vm: *mut PKVM,
    mut fn_0: libc::c_int,
    mut argc: libc::c_int,
    mut argv: libc::c_int,
    mut ret: libc::c_int,
) -> bool {
    argc != 0 as libc::c_int;
    ret >= 0 as libc::c_int;
    if *((*(*vm).fiber).ret).offset(fn_0 as isize)
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((*((*(*vm).fiber).ret).offset(fn_0 as isize)
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_CLASS as libc::c_int as libc::c_uint
    {
        let mut inst: Var = _newInstance(
            vm,
            (*((*(*vm).fiber).ret).offset(fn_0 as isize)
                & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Class,
            argc,
            ((*(*vm).fiber).ret).offset(argv as isize),
        );
        if ret >= 0 as libc::c_int {
            *((*(*vm).fiber).ret).offset(ret as isize) = inst;
        }
        return ((*(*vm).fiber).error).is_null();
    }
    if *((*(*vm).fiber).ret).offset(fn_0 as isize)
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((*((*(*vm).fiber).ret).offset(fn_0 as isize)
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_CLOSURE as libc::c_int as libc::c_uint
    {
        let mut func: *mut Closure = (*((*(*vm).fiber).ret).offset(fn_0 as isize)
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object as *mut Closure;
        let mut retval: Var = 0;
        vmCallFunction(
            vm,
            func,
            argc,
            ((*(*vm).fiber).ret).offset(argv as isize),
            &mut retval,
        );
        if ret >= 0 as libc::c_int {
            *((*(*vm).fiber).ret).offset(ret as isize) = retval;
        }
        return ((*(*vm).fiber).error).is_null();
    }
    (*(*vm).fiber)
        .error = newStringLength(
        vm,
        b"Expected a Callable.\0" as *const u8 as *const libc::c_char,
        if (b"Expected a Callable.\0" as *const u8 as *const libc::c_char).is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(b"Expected a Callable.\0" as *const u8 as *const libc::c_char)
                as uint32_t
        },
    );
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkCallMethod(
    mut vm: *mut PKVM,
    mut instance: libc::c_int,
    mut method: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: libc::c_int,
    mut ret: libc::c_int,
) -> bool {
    argc != 0 as libc::c_int;
    ret >= 0 as libc::c_int;
    let mut is_method: bool = 0 as libc::c_int != 0;
    let mut smethod: *mut String_0 = newStringLength(
        vm,
        method,
        if method.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(method) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*smethod)._super);
    let mut callable: Var = getMethod(
        vm,
        *((*(*vm).fiber).ret).offset(instance as isize),
        smethod,
        &mut is_method,
    );
    vmPopTempRef(vm);
    if !((*(*vm).fiber).error).is_null() {
        return 0 as libc::c_int != 0;
    }
    if callable
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((callable & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_CLASS as libc::c_int as libc::c_uint
    {
        let mut inst: Var = _newInstance(
            vm,
            (callable & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Class,
            argc,
            ((*(*vm).fiber).ret).offset(argv as isize),
        );
        if ret >= 0 as libc::c_int {
            *((*(*vm).fiber).ret).offset(ret as isize) = inst;
        }
        return ((*(*vm).fiber).error).is_null();
    }
    if callable
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && (*((callable & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
            .type_0 as libc::c_uint == OBJ_CLOSURE as libc::c_int as libc::c_uint
    {
        let mut retval: Var = 0;
        vmCallMethod(
            vm,
            *((*(*vm).fiber).ret).offset(instance as isize),
            (callable & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                as *mut Closure,
            argc,
            ((*(*vm).fiber).ret).offset(argv as isize),
            &mut retval,
        );
        if ret >= 0 as libc::c_int {
            *((*(*vm).fiber).ret).offset(ret as isize) = retval;
        }
        return ((*(*vm).fiber).error).is_null();
    }
    (*(*vm).fiber)
        .error = stringFormat(
        vm,
        b"Instance has no method named '$'.\0" as *const u8 as *const libc::c_char,
        method,
    );
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pkPlaceSelf(mut vm: *mut PKVM, mut index: libc::c_int) {
    *((*(*vm).fiber).ret).offset(index as isize) = (*(*vm).fiber).self_0;
}
pub unsafe extern "C" fn pkImportModule(
    mut vm: *mut PKVM,
    mut path: *const libc::c_char,
    mut index: libc::c_int,
) -> bool {
    let mut path_: *mut String_0 = newStringLength(
        vm,
        path,
        if path.is_null() {
            0 as libc::c_int as libc::c_uint
        } else {
            strlen(path) as uint32_t
        },
    );
    vmPushTempRef(vm, &mut (*path_)._super);
    let mut module: Var = vmImportModule(vm, 0 as *mut String_0, path_);
    vmPopTempRef(vm);
    *((*(*vm).fiber).ret).offset(index as isize) = module;
    return ((*(*vm).fiber).error).is_null();
}
pub unsafe extern "C" fn pkGetClass(
    mut vm: *mut PKVM,
    mut instance: libc::c_int,
    mut index: libc::c_int,
) {
    *((*(*vm).fiber).ret)
        .offset(
            index as isize,
        ) = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(getClass
            as unsafe extern "C" fn(
                *mut PKVM,
                Var,
            ) -> *mut Class)(vm, *((*(*vm).fiber).ret).offset(instance as isize)))
            ._super as *mut Object as uintptr_t;
}
unsafe extern "C" fn defaultRealloc(
    mut memory: *mut libc::c_void,
    mut new_size: size_t,
    mut _v: *mut libc::c_void,
) -> *mut libc::c_void {
    if new_size == 0 as libc::c_int as libc::c_ulong {
        free(memory);
        return 0 as *mut libc::c_void;
    }
    return realloc(memory, new_size);
}
unsafe extern "C" fn stderrWrite(mut vm: *mut PKVM, mut text: *const libc::c_char) {
    fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, text);
}
unsafe extern "C" fn stdoutWrite(mut vm: *mut PKVM, mut text: *const libc::c_char) {
    fprintf(stdout, b"%s\0" as *const u8 as *const libc::c_char, text);
}
unsafe extern "C" fn stdinRead(mut vm: *mut PKVM) -> *mut libc::c_char {
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    let mut c: libc::c_char = 0;
    loop {
        c = fgetc(stdin) as libc::c_char;
        if c as libc::c_int == '\n' as i32 {
            break;
        }
        pkByteBufferWrite(&mut buff, vm, c as uint8_t);
        if !(c as libc::c_int != -(1 as libc::c_int)) {
            break;
        }
    }
    pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
    let mut str: *mut libc::c_char = pkRealloc(
        vm,
        0 as *mut libc::c_void,
        buff.count as size_t,
    ) as *mut libc::c_char;
    memcpy(
        str as *mut libc::c_void,
        buff.data as *const libc::c_void,
        buff.count as libc::c_ulong,
    );
    pkByteBufferClear(&mut buff, vm);
    return str;
}
unsafe extern "C" fn loadScript(
    mut vm: *mut PKVM,
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut file: *mut FILE = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return 0 as *mut libc::c_char;
    }
    fseek(file, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    let mut file_size: size_t = ftell(file) as size_t;
    fseek(file, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    let mut buff: *mut libc::c_char = pkRealloc(
        vm,
        0 as *mut libc::c_void,
        file_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    clearerr(file);
    let mut read: size_t = fread(
        buff as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        file_size,
        file,
    );
    *buff.offset(read as isize) = '\0' as i32 as libc::c_char;
    fclose(file);
    return buff;
}
