use ::libc;
extern "C" {
    
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn doubleToVar(value: libc::c_double) -> Var;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pkUintBufferWrite(self_0: *mut pkUintBuffer, vm: *mut PKVM, data: uint32_t);
    fn pkByteBufferInit(self_0: *mut pkByteBuffer);
    fn pkByteBufferClear(self_0: *mut pkByteBuffer, vm: *mut PKVM);
    fn pkByteBufferWrite(self_0: *mut pkByteBuffer, vm: *mut PKVM, data: uint8_t);
    fn pkByteBufferAddString(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        str: *const libc::c_char,
        length: uint32_t,
    );
    fn newStringLength(
        vm: *mut PKVM,
        text: *const libc::c_char,
        length: uint32_t,
    ) -> *mut String_0;
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
    fn markObject(vm: *mut PKVM, self_0: *mut Object);
    fn markValue(vm: *mut PKVM, self_0: Var);
    fn moduleAddConstant(vm: *mut PKVM, module: *mut Module, value: Var) -> uint32_t;
    fn moduleAddString(
        module: *mut Module,
        vm: *mut PKVM,
        name: *const libc::c_char,
        length: uint32_t,
        index: *mut libc::c_int,
    ) -> *mut String_0;
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
    fn moduleAddMain(vm: *mut PKVM, module: *mut Module);
    fn utilIsName(c: libc::c_char) -> bool;
    fn utilIsDigit(c: libc::c_char) -> bool;
    fn utilIsCharHex(c: libc::c_char) -> bool;
    fn utilCharHexVal(c: libc::c_char) -> uint8_t;
    fn vmPushTempRef(vm: *mut PKVM, obj: *mut Object);
    fn vmPopTempRef(vm: *mut PKVM);
    fn reportCompileTimeError(
        vm: *mut PKVM,
        path: *const libc::c_char,
        line: libc::c_int,
        source: *const libc::c_char,
        at: *const libc::c_char,
        length: libc::c_int,
        fmt: *const libc::c_char,
        args: ::std::ffi::VaList,
    );
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
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub struct Compiler {
    pub parser: Parser,
    pub next_compiler: *mut Compiler,
    pub options: *const CompileOptions,
    pub module: *mut Module,
    pub loop_0: *mut Loop,
    pub func: *mut Func,
    pub scope_depth: libc::c_int,
    pub new_local: bool,
    pub l_value: bool,
    pub can_define: bool,
    pub is_last_call: bool,
    pub bifn_list_join: libc::c_int,
}
pub type Func = sFunc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sFunc {
    pub type_0: FuncType,
    pub depth: libc::c_int,
    pub locals: [Local; 256],
    pub local_count: libc::c_int,
    pub upvalues: [UpvalueInfo; 256],
    pub stack_size: libc::c_int,
    pub ptr: *mut Function,
    pub outer_func: *mut sFunc,
}
pub type UpvalueInfo = sUpvalueInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sUpvalueInfo {
    pub is_immediate: bool,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Local {
    pub name: *const libc::c_char,
    pub length: uint32_t,
    pub depth: libc::c_int,
    pub is_upvalue: bool,
    pub line: libc::c_int,
}
pub type FuncType = libc::c_uint;
pub const FUNC_CONSTRUCTOR: FuncType = 4;
pub const FUNC_METHOD: FuncType = 3;
pub const FUNC_LITERAL: FuncType = 2;
pub const FUNC_TOPLEVEL: FuncType = 1;
pub const FUNC_MAIN: FuncType = 0;
pub type Loop = sLoop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sLoop {
    pub start: libc::c_int,
    pub exit_jump: libc::c_int,
    pub patches: [libc::c_int; 256],
    pub patch_count: libc::c_int,
    pub outer_loop: *mut sLoop,
    pub depth: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CompileOptions {
    pub debug: bool,
    pub repl_mode: bool,
}
pub type Parser = sParser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sParser {
    pub vm: *mut PKVM,
    pub source: *const libc::c_char,
    pub file_path: *const libc::c_char,
    pub token_start: *const libc::c_char,
    pub current_char: *const libc::c_char,
    pub current_line: libc::c_int,
    pub previous: Token,
    pub current: Token,
    pub next: Token,
    pub si_depth: libc::c_int,
    pub si_open_brace: [libc::c_int; 8],
    pub si_quote: [libc::c_char; 8],
    pub si_name_end: *const libc::c_char,
    pub si_name_quote: libc::c_char,
    pub forwards: [ForwardName; 256],
    pub forwards_count: libc::c_int,
    pub optional_call_paran: bool,
    pub repl_mode: bool,
    pub parsing_class: bool,
    pub need_more_lines: bool,
    pub has_syntax_error: bool,
    pub has_errors: bool,
}
pub type ForwardName = sForwardName;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sForwardName {
    pub instruction: libc::c_int,
    pub func: *mut Fn_0,
    pub tkname: Token,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub type_0: _TokenType,
    pub start: *const libc::c_char,
    pub length: libc::c_int,
    pub line: libc::c_int,
    pub value: Var,
}
pub type _TokenType = libc::c_uint;
pub const TK_STRING_INTERP: _TokenType = 78;
pub const TK_STRING: _TokenType = 77;
pub const TK_NUMBER: _TokenType = 76;
pub const TK_NAME: _TokenType = 75;
pub const TK_RETURN: _TokenType = 74;
pub const TK_CONTINUE: _TokenType = 73;
pub const TK_BREAK: _TokenType = 72;
pub const TK_ELSE: _TokenType = 71;
pub const TK_ELIF: _TokenType = 70;
pub const TK_IF: _TokenType = 69;
pub const TK_FOR: _TokenType = 68;
pub const TK_WHILE: _TokenType = 67;
pub const TK_THEN: _TokenType = 66;
pub const TK_DO: _TokenType = 65;
pub const TK_SUPER: _TokenType = 64;
pub const TK_SELF: _TokenType = 63;
pub const TK_FALSE: _TokenType = 62;
pub const TK_TRUE: _TokenType = 61;
pub const TK_NOT: _TokenType = 60;
pub const TK_OR: _TokenType = 59;
pub const TK_AND: _TokenType = 58;
pub const TK_IS: _TokenType = 57;
pub const TK_IN: _TokenType = 56;
pub const TK_NULL: _TokenType = 55;
pub const TK_END: _TokenType = 54;
pub const TK_FN: _TokenType = 53;
pub const TK_NATIVE: _TokenType = 52;
pub const TK_DEF: _TokenType = 51;
pub const TK_AS: _TokenType = 50;
pub const TK_IMPORT: _TokenType = 49;
pub const TK_FROM: _TokenType = 48;
pub const TK_CLASS: _TokenType = 47;
pub const TK_SLEFTEQ: _TokenType = 46;
pub const TK_SRIGHTEQ: _TokenType = 45;
pub const TK_SLEFT: _TokenType = 44;
pub const TK_SRIGHT: _TokenType = 43;
pub const TK_XOREQ: _TokenType = 42;
pub const TK_OREQ: _TokenType = 41;
pub const TK_ANDEQ: _TokenType = 40;
pub const TK_POWEQ: _TokenType = 39;
pub const TK_MODEQ: _TokenType = 38;
pub const TK_DIVEQ: _TokenType = 37;
pub const TK_STAREQ: _TokenType = 36;
pub const TK_MINUSEQ: _TokenType = 35;
pub const TK_PLUSEQ: _TokenType = 34;
pub const TK_LTEQ: _TokenType = 33;
pub const TK_GTEQ: _TokenType = 32;
pub const TK_NOTEQ: _TokenType = 31;
pub const TK_EQEQ: _TokenType = 30;
pub const TK_LT: _TokenType = 29;
pub const TK_GT: _TokenType = 28;
pub const TK_EQ: _TokenType = 27;
pub const TK_BSLASH: _TokenType = 26;
pub const TK_STARSTAR: _TokenType = 25;
pub const TK_FSLASH: _TokenType = 24;
pub const TK_STAR: _TokenType = 23;
pub const TK_MINUS: _TokenType = 22;
pub const TK_PLUS: _TokenType = 21;
pub const TK_ARROW: _TokenType = 20;
pub const TK_CARET: _TokenType = 19;
pub const TK_PIPE: _TokenType = 18;
pub const TK_AMP: _TokenType = 17;
pub const TK_TILD: _TokenType = 16;
pub const TK_PERCENT: _TokenType = 15;
pub const TK_RBRACE: _TokenType = 14;
pub const TK_LBRACE: _TokenType = 13;
pub const TK_RBRACKET: _TokenType = 12;
pub const TK_LBRACKET: _TokenType = 11;
pub const TK_RPARAN: _TokenType = 10;
pub const TK_LPARAN: _TokenType = 9;
pub const TK_HASH: _TokenType = 8;
pub const TK_SEMICOLLON: _TokenType = 7;
pub const TK_COLLON: _TokenType = 6;
pub const TK_COMMA: _TokenType = 5;
pub const TK_DOTDOT: _TokenType = 4;
pub const TK_DOT: _TokenType = 3;
pub const TK_LINE: _TokenType = 2;
pub const TK_EOF: _TokenType = 1;
pub const TK_ERROR: _TokenType = 0;
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
pub type va_list = __builtin_va_list;
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
pub struct OpInfo {
    pub params: libc::c_int,
    pub stack: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Keyword {
    pub identifier: *const libc::c_char,
    pub length: libc::c_int,
    pub tk_type: _TokenType,
}
pub type Precedence = libc::c_uint;
pub const PREC_PRIMARY: Precedence = 19;
pub const PREC_ATTRIB: Precedence = 18;
pub const PREC_SUBSCRIPT: Precedence = 17;
pub const PREC_CALL: Precedence = 16;
pub const PREC_EXPONENT: Precedence = 15;
pub const PREC_UNARY: Precedence = 14;
pub const PREC_FACTOR: Precedence = 13;
pub const PREC_TERM: Precedence = 12;
pub const PREC_RANGE: Precedence = 11;
pub const PREC_BITWISE_SHIFT: Precedence = 10;
pub const PREC_BITWISE_AND: Precedence = 9;
pub const PREC_BITWISE_XOR: Precedence = 8;
pub const PREC_BITWISE_OR: Precedence = 7;
pub const PREC_COMPARISION: Precedence = 6;
pub const PREC_TEST: Precedence = 5;
pub const PREC_EQUALITY: Precedence = 4;
pub const PREC_LOGICAL_AND: Precedence = 3;
pub const PREC_LOGICAL_OR: Precedence = 2;
pub const PREC_LOWEST: Precedence = 1;
pub const PREC_NONE: Precedence = 0;
pub type GrammarFn = Option::<unsafe extern "C" fn(*mut Compiler) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GrammarRule {
    pub prefix: GrammarFn,
    pub infix: GrammarFn,
    pub precedence: Precedence,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameSearchResult {
    pub type_0: NameDefnType,
    pub index: libc::c_int,
    pub line: libc::c_int,
}
pub type NameDefnType = libc::c_uint;
pub const NAME_BUILTIN_TY: NameDefnType = 5;
pub const NAME_BUILTIN_FN: NameDefnType = 4;
pub const NAME_GLOBAL_VAR: NameDefnType = 3;
pub const NAME_UPVALUE: NameDefnType = 2;
pub const NAME_LOCAL_VAR: NameDefnType = 1;
pub const NAME_NOT_DEFINED: NameDefnType = 0;
pub const DEPTH_GLOBAL: C2RustUnnamed_0 = -1;
pub type BlockType = libc::c_uint;
pub const BLOCK_ELSE: BlockType = 3;
pub const BLOCK_IF: BlockType = 2;
pub const BLOCK_LOOP: BlockType = 1;
pub const BLOCK_FUNC: BlockType = 0;
pub type C2RustUnnamed_0 = libc::c_int;
pub const DEPTH_LOCAL: C2RustUnnamed_0 = 0;
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut _keywords: [_Keyword; 29] = [
    {
        let mut init = _Keyword {
            identifier: b"class\0" as *const u8 as *const libc::c_char,
            length: 5 as libc::c_int,
            tk_type: TK_CLASS,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"from\0" as *const u8 as *const libc::c_char,
            length: 4 as libc::c_int,
            tk_type: TK_FROM,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"import\0" as *const u8 as *const libc::c_char,
            length: 6 as libc::c_int,
            tk_type: TK_IMPORT,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"as\0" as *const u8 as *const libc::c_char,
            length: 2 as libc::c_int,
            tk_type: TK_AS,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"def\0" as *const u8 as *const libc::c_char,
            length: 3 as libc::c_int,
            tk_type: TK_DEF,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"native\0" as *const u8 as *const libc::c_char,
            length: 6 as libc::c_int,
            tk_type: TK_NATIVE,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"fn\0" as *const u8 as *const libc::c_char,
            length: 2 as libc::c_int,
            tk_type: TK_FN,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"end\0" as *const u8 as *const libc::c_char,
            length: 3 as libc::c_int,
            tk_type: TK_END,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"null\0" as *const u8 as *const libc::c_char,
            length: 4 as libc::c_int,
            tk_type: TK_NULL,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"in\0" as *const u8 as *const libc::c_char,
            length: 2 as libc::c_int,
            tk_type: TK_IN,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"is\0" as *const u8 as *const libc::c_char,
            length: 2 as libc::c_int,
            tk_type: TK_IS,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"and\0" as *const u8 as *const libc::c_char,
            length: 3 as libc::c_int,
            tk_type: TK_AND,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"or\0" as *const u8 as *const libc::c_char,
            length: 2 as libc::c_int,
            tk_type: TK_OR,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"not\0" as *const u8 as *const libc::c_char,
            length: 3 as libc::c_int,
            tk_type: TK_NOT,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"true\0" as *const u8 as *const libc::c_char,
            length: 4 as libc::c_int,
            tk_type: TK_TRUE,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"false\0" as *const u8 as *const libc::c_char,
            length: 5 as libc::c_int,
            tk_type: TK_FALSE,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"self\0" as *const u8 as *const libc::c_char,
            length: 4 as libc::c_int,
            tk_type: TK_SELF,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"super\0" as *const u8 as *const libc::c_char,
            length: 5 as libc::c_int,
            tk_type: TK_SUPER,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"do\0" as *const u8 as *const libc::c_char,
            length: 2 as libc::c_int,
            tk_type: TK_DO,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"then\0" as *const u8 as *const libc::c_char,
            length: 4 as libc::c_int,
            tk_type: TK_THEN,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"while\0" as *const u8 as *const libc::c_char,
            length: 5 as libc::c_int,
            tk_type: TK_WHILE,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"for\0" as *const u8 as *const libc::c_char,
            length: 3 as libc::c_int,
            tk_type: TK_FOR,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"if\0" as *const u8 as *const libc::c_char,
            length: 2 as libc::c_int,
            tk_type: TK_IF,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"elif\0" as *const u8 as *const libc::c_char,
            length: 4 as libc::c_int,
            tk_type: TK_ELIF,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"else\0" as *const u8 as *const libc::c_char,
            length: 4 as libc::c_int,
            tk_type: TK_ELSE,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"break\0" as *const u8 as *const libc::c_char,
            length: 5 as libc::c_int,
            tk_type: TK_BREAK,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"continue\0" as *const u8 as *const libc::c_char,
            length: 8 as libc::c_int,
            tk_type: TK_CONTINUE,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: b"return\0" as *const u8 as *const libc::c_char,
            length: 6 as libc::c_int,
            tk_type: TK_RETURN,
        };
        init
    },
    {
        let mut init = _Keyword {
            identifier: 0 as *const libc::c_char,
            length: 0 as libc::c_int,
            tk_type: TK_ERROR,
        };
        init
    },
];
static mut opcode_info: [OpInfo; 89] = [
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 3 as libc::c_int,
            stack: -(0 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 3 as libc::c_int,
            stack: -(0 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(0 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(0 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 3 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 2 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 1 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = OpInfo {
            params: 0 as libc::c_int,
            stack: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn parserInit(
    mut parser: *mut Parser,
    mut vm: *mut PKVM,
    mut compiler: *mut Compiler,
    mut source: *const libc::c_char,
    mut path: *const libc::c_char,
) {
    (*parser).vm = vm;
    (*parser).source = source;
    (*parser).file_path = path;
    (*parser).token_start = (*parser).source;
    (*parser).current_char = (*parser).source;
    (*parser).current_line = 1 as libc::c_int;
    (*parser).previous.type_0 = TK_ERROR;
    (*parser).current.type_0 = TK_ERROR;
    (*parser).next.type_0 = TK_ERROR;
    (*parser).next.start = 0 as *const libc::c_char;
    (*parser).next.length = 0 as libc::c_int;
    (*parser).next.line = 1 as libc::c_int;
    (*parser)
        .next
        .value = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x1000000000000 as libc::c_long as uint64_t;
    (*parser).si_depth = 0 as libc::c_int;
    (*parser).si_name_end = 0 as *const libc::c_char;
    (*parser).si_name_quote = '\0' as i32 as libc::c_char;
    (*parser).forwards_count = 0 as libc::c_int;
    (*parser)
        .repl_mode = !((*compiler).options).is_null()
        && (*(*compiler).options).repl_mode as libc::c_int != 0;
    (*parser).optional_call_paran = 0 as libc::c_int != 0;
    (*parser).parsing_class = 0 as libc::c_int != 0;
    (*parser).has_errors = 0 as libc::c_int != 0;
    (*parser).has_syntax_error = 0 as libc::c_int != 0;
    (*parser).need_more_lines = 0 as libc::c_int != 0;
}
unsafe extern "C" fn compilerInit(
    mut compiler: *mut Compiler,
    mut vm: *mut PKVM,
    mut source: *const libc::c_char,
    mut module: *mut Module,
    mut options: *const CompileOptions,
) {
    memset(
        compiler as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Compiler>() as libc::c_ulong,
    );
    (*compiler).next_compiler = 0 as *mut Compiler;
    (*compiler).module = module;
    (*compiler).options = options;
    (*compiler).scope_depth = DEPTH_GLOBAL as libc::c_int;
    (*compiler).loop_0 = 0 as *mut Loop;
    (*compiler).func = 0 as *mut Func;
    (*compiler).can_define = 1 as libc::c_int != 0;
    (*compiler).new_local = 0 as libc::c_int != 0;
    (*compiler).is_last_call = 0 as libc::c_int != 0;
    let mut source_path: *const libc::c_char = b"@??\0" as *const u8
        as *const libc::c_char;
    if !((*module).path).is_null() {
        source_path = ((*(*module).path).data).as_mut_ptr();
    } else if !options.is_null() && (*options).repl_mode as libc::c_int != 0 {
        source_path = b"@REPL\0" as *const u8 as *const libc::c_char;
    }
    parserInit(&mut (*compiler).parser, vm, compiler, source, source_path);
    (*compiler)
        .bifn_list_join = findBuiltinFunction(
        vm,
        b"list_join\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as uint32_t,
    );
}
unsafe extern "C" fn reportError(
    mut parser: *mut Parser,
    mut tk: Token,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    (*parser).has_errors = 1 as libc::c_int != 0;
    let mut vm: *mut PKVM = (*parser).vm;
    if ((*vm).config.stderr_write).is_none() {
        return;
    }
    if (*parser).need_more_lines {
        return;
    }
    reportCompileTimeError(
        vm,
        (*parser).file_path,
        tk.line,
        (*parser).source,
        tk.start,
        tk.length,
        fmt,
        args.as_va_list(),
    );
}
unsafe extern "C" fn syntaxError(
    mut compiler: *mut Compiler,
    mut tk: Token,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut parser: *mut Parser = &mut (*compiler).parser;
    if (*parser).has_syntax_error {
        return;
    }
    (*parser).has_syntax_error = 1 as libc::c_int != 0;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    reportError(parser, tk, fmt, args_0.as_va_list());
}
unsafe extern "C" fn semanticError(
    mut compiler: *mut Compiler,
    mut tk: Token,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut parser: *mut Parser = &mut (*compiler).parser;
    if (*parser).has_syntax_error {
        return;
    }
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    reportError(parser, tk, fmt, args_0.as_va_list());
}
unsafe extern "C" fn resolveError(
    mut compiler: *mut Compiler,
    mut tk: Token,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut parser: *mut Parser = &mut (*compiler).parser;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    reportError(parser, tk, fmt, args_0.as_va_list());
}
unsafe extern "C" fn checkMaxConstantsReached(
    mut compiler: *mut Compiler,
    mut index: libc::c_int,
) {
    if index >= (1 as libc::c_int) << 16 as libc::c_int {
        semanticError(
            compiler,
            (*compiler).parser.previous,
            b"A module should contain at most %d unique constants.\0" as *const u8
                as *const libc::c_char,
            (1 as libc::c_int) << 16 as libc::c_int,
        );
    }
}
unsafe extern "C" fn eatString(mut compiler: *mut Compiler, mut single_quote: bool) {
    let mut parser: *mut Parser = &mut (*compiler).parser;
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    let mut quote: libc::c_char = (if single_quote as libc::c_int != 0 {
        '\'' as i32
    } else {
        '"' as i32
    }) as libc::c_char;
    let mut tk_type: _TokenType = TK_STRING;
    loop {
        let mut c: libc::c_char = eatChar(parser);
        if c as libc::c_int == quote as libc::c_int {
            break;
        }
        if c as libc::c_int == '\0' as i32 {
            syntaxError(
                compiler,
                makeErrToken(parser),
                b"Non terminated string.\0" as *const u8 as *const libc::c_char,
            );
            return;
        } else if c as libc::c_int == '$' as i32 {
            if (*parser).si_depth < 8 as libc::c_int {
                tk_type = TK_STRING_INTERP;
                let mut c2: libc::c_char = peekChar(parser);
                if c2 as libc::c_int == '{' as i32 {
                    eatChar(parser);
                    (*parser).si_depth += 1;
                    (*parser).si_depth;
                    (*parser)
                        .si_quote[((*parser).si_depth - 1 as libc::c_int)
                        as usize] = quote;
                    (*parser)
                        .si_open_brace[((*parser).si_depth - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int;
                } else if !utilIsName(c2) {
                    syntaxError(
                        compiler,
                        makeErrToken(parser),
                        b"Expected '{' or identifier after '$'.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                } else {
                    let mut ptr: *const libc::c_char = (*parser).current_char;
                    while utilIsName(*ptr) as libc::c_int != 0
                        || utilIsDigit(*ptr) as libc::c_int != 0
                    {
                        ptr = ptr.offset(1);
                        ptr;
                    }
                    (*parser).si_name_end = ptr;
                    (*parser).si_name_quote = quote;
                }
            } else {
                semanticError(
                    compiler,
                    makeErrToken(parser),
                    b"Maximum interpolation level reached (can only interpolate upto depth %d).\0"
                        as *const u8 as *const libc::c_char,
                    8 as libc::c_int,
                );
            }
            break;
        } else if c as libc::c_int == '\\' as i32 {
            let mut current_block_40: u64;
            match eatChar(parser) as libc::c_int {
                34 => {
                    pkByteBufferWrite(&mut buff, (*parser).vm, '"' as i32 as uint8_t);
                    current_block_40 = 3689906465960840878;
                }
                39 => {
                    pkByteBufferWrite(&mut buff, (*parser).vm, '\'' as i32 as uint8_t);
                    current_block_40 = 3689906465960840878;
                }
                92 => {
                    pkByteBufferWrite(&mut buff, (*parser).vm, '\\' as i32 as uint8_t);
                    current_block_40 = 3689906465960840878;
                }
                110 => {
                    pkByteBufferWrite(&mut buff, (*parser).vm, '\n' as i32 as uint8_t);
                    current_block_40 = 3689906465960840878;
                }
                114 => {
                    pkByteBufferWrite(&mut buff, (*parser).vm, '\r' as i32 as uint8_t);
                    current_block_40 = 3689906465960840878;
                }
                116 => {
                    pkByteBufferWrite(&mut buff, (*parser).vm, '\t' as i32 as uint8_t);
                    current_block_40 = 3689906465960840878;
                }
                10 => {
                    current_block_40 = 3689906465960840878;
                }
                36 => {
                    pkByteBufferWrite(&mut buff, (*parser).vm, '$' as i32 as uint8_t);
                    current_block_40 = 3689906465960840878;
                }
                120 => {
                    let mut val: uint8_t = 0 as libc::c_int as uint8_t;
                    c = eatChar(parser);
                    if !utilIsCharHex(c) {
                        semanticError(
                            compiler,
                            makeErrToken(parser),
                            b"Invalid hex escape.\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        val = utilCharHexVal(c);
                        c = eatChar(parser);
                        if !utilIsCharHex(c) {
                            semanticError(
                                compiler,
                                makeErrToken(parser),
                                b"Invalid hex escape.\0" as *const u8 as *const libc::c_char,
                            );
                        } else {
                            val = ((val as libc::c_int) << 4 as libc::c_int
                                | utilCharHexVal(c) as libc::c_int) as uint8_t;
                            pkByteBufferWrite(&mut buff, (*parser).vm, val);
                        }
                    }
                    current_block_40 = 3689906465960840878;
                }
                13 => {
                    if matchChar(parser, '\n' as i32 as libc::c_char) {
                        current_block_40 = 3689906465960840878;
                    } else {
                        current_block_40 = 13475155725225270493;
                    }
                }
                _ => {
                    current_block_40 = 13475155725225270493;
                }
            }
            match current_block_40 {
                13475155725225270493 => {
                    semanticError(
                        compiler,
                        makeErrToken(parser),
                        b"Invalid escape character.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                _ => {}
            }
        } else {
            pkByteBufferWrite(&mut buff, (*parser).vm, c as uint8_t);
        }
    }
    let mut string: Var = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0x8000000000000000 as libc::c_ulong
        | &mut (*(newStringLength
            as unsafe extern "C" fn(
                *mut PKVM,
                *const libc::c_char,
                uint32_t,
            ) -> *mut String_0)(
            (*parser).vm,
            buff.data as *const libc::c_char,
            buff.count,
        ))
            ._super as *mut Object as uintptr_t;
    pkByteBufferClear(&mut buff, (*parser).vm);
    setNextValueToken(parser, tk_type, string);
}
unsafe extern "C" fn peekChar(mut parser: *mut Parser) -> libc::c_char {
    return *(*parser).current_char;
}
unsafe extern "C" fn peekNextChar(mut parser: *mut Parser) -> libc::c_char {
    if peekChar(parser) as libc::c_int == '\0' as i32 {
        return '\0' as i32 as libc::c_char;
    }
    return *((*parser).current_char).offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn eatChar(mut parser: *mut Parser) -> libc::c_char {
    let mut c: libc::c_char = peekChar(parser);
    (*parser).current_char = ((*parser).current_char).offset(1);
    (*parser).current_char;
    if c as libc::c_int == '\n' as i32 {
        (*parser).current_line += 1;
        (*parser).current_line;
    }
    return c;
}
unsafe extern "C" fn eatName(mut parser: *mut Parser) {
    let mut c: libc::c_char = peekChar(parser);
    while utilIsName(c) as libc::c_int != 0 || utilIsDigit(c) as libc::c_int != 0 {
        eatChar(parser);
        c = peekChar(parser);
    }
    let mut name_start: *const libc::c_char = (*parser).token_start;
    let mut type_0: _TokenType = TK_NAME;
    let mut length: libc::c_int = ((*parser).current_char).offset_from(name_start)
        as libc::c_long as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(_keywords[i as usize].identifier).is_null() {
        if _keywords[i as usize].length == length
            && strncmp(
                name_start,
                _keywords[i as usize].identifier,
                length as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            type_0 = _keywords[i as usize].tk_type;
            break;
        } else {
            i += 1;
            i;
        }
    }
    setNextToken(parser, type_0);
}
unsafe extern "C" fn eatNumber(mut compiler: *mut Compiler) {
    let mut parser: *mut Parser = &mut (*compiler).parser;
    let mut value: Var = 0x7ffc000000000000 as libc::c_long as uint64_t
        | 0 as libc::c_int as uint64_t;
    let mut c: libc::c_char = *(*parser).token_start;
    if c as libc::c_int == '0' as i32
        && (peekChar(parser) as libc::c_int == 'b' as i32
            || peekChar(parser) as libc::c_int == 'B' as i32)
    {
        eatChar(parser);
        let mut bin: uint64_t = 0 as libc::c_int as uint64_t;
        c = peekChar(parser);
        if !(c as libc::c_int == '0' as i32 || c as libc::c_int == '1' as i32) {
            syntaxError(
                compiler,
                makeErrToken(parser),
                b"Invalid binary literal.\0" as *const u8 as *const libc::c_char,
            );
            return;
        } else {
            loop {
                c = peekChar(parser);
                if !(c as libc::c_int == '0' as i32 || c as libc::c_int == '1' as i32) {
                    break;
                }
                eatChar(parser);
                let mut length: libc::c_int = ((*parser).current_char)
                    .offset_from((*parser).token_start) as libc::c_long as libc::c_int;
                if length > 68 as libc::c_int - 2 as libc::c_int {
                    semanticError(
                        compiler,
                        makeErrToken(parser),
                        b"Binary literal is too long.\0" as *const u8
                            as *const libc::c_char,
                    );
                    break;
                } else {
                    bin = bin << 1 as libc::c_int
                        | (c as libc::c_int - '0' as i32) as libc::c_ulong;
                }
            }
        }
        value = doubleToVar(bin as libc::c_double);
    } else if c as libc::c_int == '0' as i32
        && (peekChar(parser) as libc::c_int == 'x' as i32
            || peekChar(parser) as libc::c_int == 'X' as i32)
    {
        eatChar(parser);
        let mut hex: uint64_t = 0 as libc::c_int as uint64_t;
        c = peekChar(parser);
        if !utilIsCharHex(c) {
            syntaxError(
                compiler,
                makeErrToken(parser),
                b"Invalid hex literal.\0" as *const u8 as *const libc::c_char,
            );
            return;
        } else {
            loop {
                c = peekChar(parser);
                if !utilIsCharHex(c) {
                    break;
                }
                eatChar(parser);
                let mut length_0: libc::c_int = ((*parser).current_char)
                    .offset_from((*parser).token_start) as libc::c_long as libc::c_int;
                if length_0 > 20 as libc::c_int - 2 as libc::c_int {
                    semanticError(
                        compiler,
                        makeErrToken(parser),
                        b"Hex literal is too long.\0" as *const u8 as *const libc::c_char,
                    );
                    break;
                } else {
                    hex = hex << 4 as libc::c_int | utilCharHexVal(c) as libc::c_ulong;
                }
            }
            value = doubleToVar(hex as libc::c_double);
        }
    } else {
        while utilIsDigit(peekChar(parser)) {
            eatChar(parser);
        }
        if c as libc::c_int != '.' as i32 {
            if peekChar(parser) as libc::c_int == '.' as i32
                && utilIsDigit(peekNextChar(parser)) as libc::c_int != 0
            {
                matchChar(parser, '.' as i32 as libc::c_char);
                while utilIsDigit(peekChar(parser)) {
                    eatChar(parser);
                }
            }
        }
        if matchChar(parser, 'e' as i32 as libc::c_char) as libc::c_int != 0
            || matchChar(parser, 'E' as i32 as libc::c_char) as libc::c_int != 0
        {
            if peekChar(parser) as libc::c_int == '+' as i32
                || peekChar(parser) as libc::c_int == '-' as i32
            {
                eatChar(parser);
            }
            if !utilIsDigit(peekChar(parser)) {
                syntaxError(
                    compiler,
                    makeErrToken(parser),
                    b"Invalid number literal.\0" as *const u8 as *const libc::c_char,
                );
                return;
            } else {
                while utilIsDigit(peekChar(parser)) {
                    eatChar(parser);
                }
            }
        }
        *__errno_location() = 0 as libc::c_int;
        value = doubleToVar(atof((*parser).token_start));
        if *__errno_location() == 34 as libc::c_int {
            let mut start: *const libc::c_char = (*parser).token_start;
            let mut len: libc::c_int = ((*parser).current_char).offset_from(start)
                as libc::c_long as libc::c_int;
            semanticError(
                compiler,
                makeErrToken(parser),
                b"Number literal is too large (%.*s).\0" as *const u8
                    as *const libc::c_char,
                len,
                start,
            );
            value = doubleToVar(0 as libc::c_int as libc::c_double);
        }
    }
    setNextValueToken(parser, TK_NUMBER, value);
}
unsafe extern "C" fn skipLineComment(mut parser: *mut Parser) {
    let mut c: libc::c_char = 0;
    loop {
        c = peekChar(parser);
        if !(c as libc::c_int != '\0' as i32) {
            break;
        }
        if c as libc::c_int == '\n' as i32 {
            return;
        }
        eatChar(parser);
    };
}
unsafe extern "C" fn matchChar(mut parser: *mut Parser, mut c: libc::c_char) -> bool {
    if peekChar(parser) as libc::c_int != c as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    eatChar(parser);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn setNextTwoCharToken(
    mut parser: *mut Parser,
    mut c: libc::c_char,
    mut one: _TokenType,
    mut two: _TokenType,
) {
    if matchChar(parser, c) {
        setNextToken(parser, two);
    } else {
        setNextToken(parser, one);
    };
}
unsafe extern "C" fn makeErrToken(mut parser: *mut Parser) -> Token {
    let mut tk: Token = Token {
        type_0: TK_ERROR,
        start: 0 as *const libc::c_char,
        length: 0,
        line: 0,
        value: 0,
    };
    tk.type_0 = TK_ERROR;
    tk.start = (*parser).token_start;
    tk
        .length = ((*parser).current_char).offset_from((*parser).token_start)
        as libc::c_long as libc::c_int;
    tk.line = (*parser).current_line;
    return tk;
}
unsafe extern "C" fn setNextToken(mut parser: *mut Parser, mut type_0: _TokenType) {
    let mut next: *mut Token = &mut (*parser).next;
    (*next).type_0 = type_0;
    (*next).start = (*parser).token_start;
    (*next)
        .length = ((*parser).current_char).offset_from((*parser).token_start)
        as libc::c_long as libc::c_int;
    (*next)
        .line = (*parser).current_line
        - (if type_0 as libc::c_uint == TK_LINE as libc::c_int as libc::c_uint {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        });
}
unsafe extern "C" fn setNextValueToken(
    mut parser: *mut Parser,
    mut type_0: _TokenType,
    mut value: Var,
) {
    setNextToken(parser, type_0);
    (*parser).next.value = value;
}
unsafe extern "C" fn lexToken(mut compiler: *mut Compiler) {
    let mut parser: *mut Parser = &mut (*compiler).parser;
    (*parser).previous = (*parser).current;
    (*parser).current = (*parser).next;
    if (*parser).current.type_0 as libc::c_uint == TK_EOF as libc::c_int as libc::c_uint
    {
        return;
    }
    while peekChar(parser) as libc::c_int != '\0' as i32 {
        (*parser).token_start = (*parser).current_char;
        if !((*parser).si_name_end).is_null() {
            if (*parser).current_char == (*parser).si_name_end {
                (*parser).si_name_end = 0 as *const libc::c_char;
                eatString(
                    compiler,
                    (*parser).si_name_quote as libc::c_int == '\'' as i32,
                );
                return;
            }
        }
        let mut c: libc::c_char = eatChar(parser);
        match c as libc::c_int {
            123 => {
                if (*parser).si_depth > 0 as libc::c_int {
                    (*parser)
                        .si_open_brace[((*parser).si_depth - 1 as libc::c_int) as usize]
                        += 1;
                    (*parser)
                        .si_open_brace[((*parser).si_depth - 1 as libc::c_int) as usize];
                }
                setNextToken(parser, TK_LBRACE);
                return;
            }
            125 => {
                if (*parser).si_depth > 0 as libc::c_int {
                    if (*parser)
                        .si_open_brace[((*parser).si_depth - 1 as libc::c_int) as usize]
                        == 0 as libc::c_int
                    {
                        let mut quote: libc::c_char = (*parser)
                            .si_quote[((*parser).si_depth - 1 as libc::c_int) as usize];
                        (*parser).si_depth -= 1;
                        (*parser).si_depth;
                        eatString(compiler, quote as libc::c_int == '\'' as i32);
                        return;
                    } else {
                        (*parser)
                            .si_open_brace[((*parser).si_depth - 1 as libc::c_int)
                            as usize] -= 1;
                        (*parser)
                            .si_open_brace[((*parser).si_depth - 1 as libc::c_int)
                            as usize];
                    }
                }
                setNextToken(parser, TK_RBRACE);
                return;
            }
            44 => {
                setNextToken(parser, TK_COMMA);
                return;
            }
            58 => {
                setNextToken(parser, TK_COLLON);
                return;
            }
            59 => {
                setNextToken(parser, TK_SEMICOLLON);
                return;
            }
            35 => {
                skipLineComment(parser);
            }
            40 => {
                setNextToken(parser, TK_LPARAN);
                return;
            }
            41 => {
                setNextToken(parser, TK_RPARAN);
                return;
            }
            91 => {
                setNextToken(parser, TK_LBRACKET);
                return;
            }
            93 => {
                setNextToken(parser, TK_RBRACKET);
                return;
            }
            37 => {
                setNextTwoCharToken(
                    parser,
                    '=' as i32 as libc::c_char,
                    TK_PERCENT,
                    TK_MODEQ,
                );
                return;
            }
            126 => {
                setNextToken(parser, TK_TILD);
                return;
            }
            38 => {
                setNextTwoCharToken(
                    parser,
                    '=' as i32 as libc::c_char,
                    TK_AMP,
                    TK_ANDEQ,
                );
                return;
            }
            124 => {
                setNextTwoCharToken(
                    parser,
                    '=' as i32 as libc::c_char,
                    TK_PIPE,
                    TK_OREQ,
                );
                return;
            }
            94 => {
                setNextTwoCharToken(
                    parser,
                    '=' as i32 as libc::c_char,
                    TK_CARET,
                    TK_XOREQ,
                );
                return;
            }
            10 => {
                setNextToken(parser, TK_LINE);
                return;
            }
            32 | 9 | 13 => {
                c = peekChar(parser);
                while c as libc::c_int == ' ' as i32 || c as libc::c_int == '\t' as i32
                    || c as libc::c_int == '\r' as i32
                {
                    eatChar(parser);
                    c = peekChar(parser);
                }
            }
            46 => {
                if matchChar(parser, '.' as i32 as libc::c_char) {
                    setNextToken(parser, TK_DOTDOT);
                } else if utilIsDigit(peekChar(parser)) {
                    eatChar(parser);
                    eatNumber(compiler);
                    if (*parser).has_syntax_error {
                        return;
                    }
                } else {
                    setNextToken(parser, TK_DOT);
                }
                return;
            }
            61 => {
                setNextTwoCharToken(parser, '=' as i32 as libc::c_char, TK_EQ, TK_EQEQ);
                return;
            }
            33 => {
                setNextTwoCharToken(
                    parser,
                    '=' as i32 as libc::c_char,
                    TK_NOT,
                    TK_NOTEQ,
                );
                return;
            }
            62 => {
                if matchChar(parser, '>' as i32 as libc::c_char) {
                    setNextTwoCharToken(
                        parser,
                        '=' as i32 as libc::c_char,
                        TK_SRIGHT,
                        TK_SRIGHTEQ,
                    );
                } else {
                    setNextTwoCharToken(
                        parser,
                        '=' as i32 as libc::c_char,
                        TK_GT,
                        TK_GTEQ,
                    );
                }
                return;
            }
            60 => {
                if matchChar(parser, '<' as i32 as libc::c_char) {
                    setNextTwoCharToken(
                        parser,
                        '=' as i32 as libc::c_char,
                        TK_SLEFT,
                        TK_SLEFTEQ,
                    );
                } else {
                    setNextTwoCharToken(
                        parser,
                        '=' as i32 as libc::c_char,
                        TK_LT,
                        TK_LTEQ,
                    );
                }
                return;
            }
            43 => {
                setNextTwoCharToken(
                    parser,
                    '=' as i32 as libc::c_char,
                    TK_PLUS,
                    TK_PLUSEQ,
                );
                return;
            }
            45 => {
                if matchChar(parser, '=' as i32 as libc::c_char) {
                    setNextToken(parser, TK_MINUSEQ);
                } else if matchChar(parser, '>' as i32 as libc::c_char) {
                    setNextToken(parser, TK_ARROW);
                } else {
                    setNextToken(parser, TK_MINUS);
                }
                return;
            }
            42 => {
                if matchChar(parser, '*' as i32 as libc::c_char) {
                    setNextTwoCharToken(
                        parser,
                        '=' as i32 as libc::c_char,
                        TK_STARSTAR,
                        TK_POWEQ,
                    );
                } else {
                    setNextTwoCharToken(
                        parser,
                        '=' as i32 as libc::c_char,
                        TK_STAR,
                        TK_STAREQ,
                    );
                }
                return;
            }
            47 => {
                setNextTwoCharToken(
                    parser,
                    '=' as i32 as libc::c_char,
                    TK_FSLASH,
                    TK_DIVEQ,
                );
                return;
            }
            34 => {
                eatString(compiler, 0 as libc::c_int != 0);
                return;
            }
            39 => {
                eatString(compiler, 1 as libc::c_int != 0);
                return;
            }
            _ => {
                if utilIsDigit(c) {
                    eatNumber(compiler);
                    if (*parser).has_syntax_error {
                        return;
                    }
                } else if utilIsName(c) {
                    eatName(parser);
                } else {
                    setNextToken(parser, TK_ERROR);
                    if c as libc::c_int >= 32 as libc::c_int
                        && c as libc::c_int <= 126 as libc::c_int
                    {
                        syntaxError(
                            compiler,
                            (*parser).next,
                            b"Invalid character '%c'\0" as *const u8
                                as *const libc::c_char,
                            c as libc::c_int,
                        );
                    } else {
                        syntaxError(
                            compiler,
                            (*parser).next,
                            b"Invalid byte 0x%x\0" as *const u8 as *const libc::c_char,
                            c as uint8_t as libc::c_int,
                        );
                    }
                }
                return;
            }
        }
    }
    (*parser).token_start = (*parser).current_char;
    setNextToken(parser, TK_EOF);
}
unsafe extern "C" fn peek(mut compiler: *mut Compiler) -> _TokenType {
    return (*compiler).parser.current.type_0;
}
unsafe extern "C" fn match_0(
    mut compiler: *mut Compiler,
    mut expected: _TokenType,
) -> bool {
    if peek(compiler) as libc::c_uint != expected as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    lexToken(compiler);
    if (*compiler).parser.has_syntax_error {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn consume(
    mut compiler: *mut Compiler,
    mut expected: _TokenType,
    mut err_msg: *const libc::c_char,
) {
    lexToken(compiler);
    if (*compiler).parser.has_syntax_error {
        return;
    }
    let mut prev: *mut Token = &mut (*compiler).parser.previous;
    if (*prev).type_0 as libc::c_uint != expected as libc::c_uint {
        syntaxError(
            compiler,
            *prev,
            b"%s\0" as *const u8 as *const libc::c_char,
            err_msg,
        );
        return;
    }
}
unsafe extern "C" fn matchLine(mut compiler: *mut Compiler) -> bool {
    let mut consumed: bool = 0 as libc::c_int != 0;
    if peek(compiler) as libc::c_uint == TK_LINE as libc::c_int as libc::c_uint {
        while peek(compiler) as libc::c_uint == TK_LINE as libc::c_int as libc::c_uint {
            lexToken(compiler);
            if (*compiler).parser.has_syntax_error {
                return 0 as libc::c_int != 0;
            }
        }
        consumed = 1 as libc::c_int != 0;
    }
    if (*compiler).parser.repl_mode as libc::c_int != 0 && !(*compiler).parser.has_errors
    {
        if peek(compiler) as libc::c_uint == TK_EOF as libc::c_int as libc::c_uint {
            (*compiler).parser.need_more_lines = 1 as libc::c_int != 0;
        }
    }
    return consumed;
}
unsafe extern "C" fn skipNewLines(mut compiler: *mut Compiler) {
    matchLine(compiler);
}
unsafe extern "C" fn matchEndStatement(mut compiler: *mut Compiler) -> bool {
    if match_0(compiler, TK_SEMICOLLON) {
        skipNewLines(compiler);
        return 1 as libc::c_int != 0;
    }
    if matchLine(compiler) as libc::c_int != 0
        || peek(compiler) as libc::c_uint == TK_EOF as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    if peek(compiler) as libc::c_uint == TK_END as libc::c_int as libc::c_uint
        || peek(compiler) as libc::c_uint == TK_ELSE as libc::c_int as libc::c_uint
        || peek(compiler) as libc::c_uint == TK_ELIF as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn consumeEndStatement(mut compiler: *mut Compiler) {
    if !matchEndStatement(compiler) {
        syntaxError(
            compiler,
            (*compiler).parser.current,
            b"Expected statement end with '\\n' or ';'.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
}
unsafe extern "C" fn consumeStartBlock(
    mut compiler: *mut Compiler,
    mut delimiter: _TokenType,
) {
    let mut consumed: bool = 0 as libc::c_int != 0;
    if delimiter as libc::c_uint == TK_DO as libc::c_int as libc::c_uint
        || delimiter as libc::c_uint == TK_THEN as libc::c_int as libc::c_uint
    {
        if match_0(compiler, delimiter) {
            consumed = 1 as libc::c_int != 0;
        }
    }
    if matchLine(compiler) {
        consumed = 1 as libc::c_int != 0;
    }
    if !consumed {
        let mut msg: *const libc::c_char = 0 as *const libc::c_char;
        if delimiter as libc::c_uint == TK_DO as libc::c_int as libc::c_uint {
            msg = b"Expected enter block with newline or 'do'.\0" as *const u8
                as *const libc::c_char;
        } else {
            msg = b"Expected enter block with newline or 'then'.\0" as *const u8
                as *const libc::c_char;
        }
        syntaxError(compiler, (*compiler).parser.previous, msg);
        return;
    }
}
unsafe extern "C" fn matchAssignment(mut compiler: *mut Compiler) -> bool {
    if match_0(compiler, TK_EQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_PLUSEQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_MINUSEQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_STAREQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_DIVEQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_MODEQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_POWEQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_ANDEQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_OREQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_XOREQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_SRIGHTEQ) {
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_SLEFTEQ) {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn findBuiltinFunction(
    mut vm: *const PKVM,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*vm).builtins_count {
        let mut bfn_length: uint32_t = strlen(
            (*(*(*vm).builtins_funcs[i as usize]).fn_0).name,
        ) as uint32_t;
        if !(bfn_length != length) {
            if strncmp(
                name,
                (*(*(*vm).builtins_funcs[i as usize]).fn_0).name,
                length as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                return i;
            }
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn findBuiltinClass(
    mut vm: *const PKVM,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < PK_INSTANCE as libc::c_int {
        if (*(*(*vm).builtin_classes[i as usize]).name).length == length
            && memcmp(
                ((*(*(*vm).builtin_classes[i as usize]).name).data).as_mut_ptr()
                    as *const libc::c_void,
                name as *const libc::c_void,
                length as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn findLocal(
    mut func: *mut Func,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*func).local_count {
        if !((*func).locals[i as usize].length != length) {
            if strncmp((*func).locals[i as usize].name, name, length as libc::c_ulong)
                == 0 as libc::c_int
            {
                return i;
            }
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn addUpvalue(
    mut compiler: *mut Compiler,
    mut func: *mut Func,
    mut index: libc::c_int,
    mut is_immediate: bool,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*func).ptr).upvalue_count {
        let mut info: UpvalueInfo = (*func).upvalues[i as usize];
        if info.index == index
            && info.is_immediate as libc::c_int == is_immediate as libc::c_int
        {
            return i;
        }
        i += 1;
        i;
    }
    if (*(*func).ptr).upvalue_count == 256 as libc::c_int {
        semanticError(
            compiler,
            (*compiler).parser.previous,
            b"A function cannot capture more thatn %d upvalues.\0" as *const u8
                as *const libc::c_char,
            256 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    (*func).upvalues[(*(*func).ptr).upvalue_count as usize].index = index;
    (*func).upvalues[(*(*func).ptr).upvalue_count as usize].is_immediate = is_immediate;
    let fresh0 = (*(*func).ptr).upvalue_count;
    (*(*func).ptr).upvalue_count = (*(*func).ptr).upvalue_count + 1;
    return fresh0;
}
unsafe extern "C" fn findUpvalue(
    mut compiler: *mut Compiler,
    mut func: *mut Func,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> libc::c_int {
    if (*func).depth <= DEPTH_GLOBAL as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut index: libc::c_int = findLocal((*func).outer_func, name, length);
    if index != -(1 as libc::c_int) {
        (*(*func).outer_func).locals[index as usize].is_upvalue = 1 as libc::c_int != 0;
        return addUpvalue(compiler, func, index, 1 as libc::c_int != 0);
    }
    index = findUpvalue(compiler, (*func).outer_func, name, length);
    if index != -(1 as libc::c_int) {
        return addUpvalue(compiler, func, index, 0 as libc::c_int != 0);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn compilerSearchName(
    mut compiler: *mut Compiler,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> NameSearchResult {
    let mut result: NameSearchResult = NameSearchResult {
        type_0: NAME_NOT_DEFINED,
        index: 0,
        line: 0,
    };
    result.type_0 = NAME_NOT_DEFINED;
    let mut index: libc::c_int = 0;
    index = findLocal((*compiler).func, name, length);
    if index != -(1 as libc::c_int) {
        result.type_0 = NAME_LOCAL_VAR;
        result.index = index;
        return result;
    }
    index = findUpvalue(compiler, (*compiler).func, name, length);
    if index != -(1 as libc::c_int) {
        result.type_0 = NAME_UPVALUE;
        result.index = index;
        return result;
    }
    index = moduleGetGlobalIndex((*compiler).module, name, length);
    if index != -(1 as libc::c_int) {
        result.type_0 = NAME_GLOBAL_VAR;
        result.index = index;
        return result;
    }
    index = findBuiltinFunction((*compiler).parser.vm, name, length);
    if index != -(1 as libc::c_int) {
        result.type_0 = NAME_BUILTIN_FN;
        result.index = index;
        return result;
    }
    index = findBuiltinClass((*compiler).parser.vm, name, length);
    if index != -(1 as libc::c_int) {
        result.type_0 = NAME_BUILTIN_TY;
        result.index = index;
        return result;
    }
    return result;
}
pub static mut rules: [GrammarRule; 79] = unsafe {
    [
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprAttrib as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_ATTRIB,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_RANGE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprGrouping as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: Some(exprCall as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_CALL,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprList as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: Some(exprSubscript as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_SUBSCRIPT,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprMap as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_FACTOR,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprUnaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_BITWISE_AND,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_BITWISE_OR,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_BITWISE_XOR,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprUnaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_TERM,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprUnaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_TERM,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_FACTOR,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_FACTOR,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_EXPONENT,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_COMPARISION,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_COMPARISION,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_EQUALITY,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_EQUALITY,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_COMPARISION,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_COMPARISION,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_BITWISE_SHIFT,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_BITWISE_SHIFT,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprFunction as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprValue as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_TEST,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprBinaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_TEST,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprAnd as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_LOGICAL_AND,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: Some(exprOr as unsafe extern "C" fn(*mut Compiler) -> ()),
                precedence: PREC_LOGICAL_OR,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprUnaryOp as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_UNARY,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprValue as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprValue as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprSelf as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprSuper as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: None,
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprName as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprLiteral as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(exprLiteral as unsafe extern "C" fn(*mut Compiler) -> ()),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
        {
            let mut init = GrammarRule {
                prefix: Some(
                    exprInterpolation as unsafe extern "C" fn(*mut Compiler) -> (),
                ),
                infix: None,
                precedence: PREC_NONE,
            };
            init
        },
    ]
};
unsafe extern "C" fn getRule(mut type_0: _TokenType) -> *mut GrammarRule {
    return &mut *rules.as_mut_ptr().offset(type_0 as libc::c_int as isize)
        as *mut GrammarRule;
}
unsafe extern "C" fn emitStoreGlobal(
    mut compiler: *mut Compiler,
    mut index: libc::c_int,
) {
    emitOpcode(compiler, OP_STORE_GLOBAL);
    emitByte(compiler, index);
}
unsafe extern "C" fn emitPushValue(
    mut compiler: *mut Compiler,
    mut type_0: NameDefnType,
    mut index: libc::c_int,
) {
    match type_0 as libc::c_uint {
        0 => {
            if (*compiler).parser.has_errors {
                return;
            }
            unreachable!();
        }
        1 => {
            if index < 9 as libc::c_int {
                emitOpcode(compiler, (OP_PUSH_LOCAL_0 as libc::c_int + index) as Opcode);
            } else {
                emitOpcode(compiler, OP_PUSH_LOCAL_N);
                emitByte(compiler, index);
            }
            return;
        }
        2 => {
            emitOpcode(compiler, OP_PUSH_UPVALUE);
            emitByte(compiler, index);
            return;
        }
        3 => {
            emitOpcode(compiler, OP_PUSH_GLOBAL);
            emitByte(compiler, index);
            return;
        }
        4 => {
            emitOpcode(compiler, OP_PUSH_BUILTIN_FN);
            emitByte(compiler, index);
            return;
        }
        5 => {
            emitOpcode(compiler, OP_PUSH_BUILTIN_TY);
            emitByte(compiler, index);
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn emitStoreValue(
    mut compiler: *mut Compiler,
    mut type_0: NameDefnType,
    mut index: libc::c_int,
) {
    match type_0 as libc::c_uint {
        0 | 4 | 5 => {
            if (*compiler).parser.has_errors {
                return;
            }
            unreachable!();
        }
        1 => {
            if index < 9 as libc::c_int {
                emitOpcode(
                    compiler,
                    (OP_STORE_LOCAL_0 as libc::c_int + index) as Opcode,
                );
            } else {
                emitOpcode(compiler, OP_STORE_LOCAL_N);
                emitByte(compiler, index);
            }
            return;
        }
        2 => {
            emitOpcode(compiler, OP_STORE_UPVALUE);
            emitByte(compiler, index);
            return;
        }
        3 => {
            emitStoreGlobal(compiler, index);
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn _compileCall(
    mut compiler: *mut Compiler,
    mut call_type: Opcode,
    mut method: libc::c_int,
) {
    let mut argc: libc::c_int = 0 as libc::c_int;
    if (*compiler).parser.optional_call_paran {
        (*compiler).parser.optional_call_paran = 0 as libc::c_int != 0;
        compileExpression(compiler);
        argc = 1 as libc::c_int;
    } else if !match_0(compiler, TK_RPARAN) {
        loop {
            skipNewLines(compiler);
            compileExpression(compiler);
            skipNewLines(compiler);
            argc += 1;
            argc;
            if !match_0(compiler, TK_COMMA) {
                break;
            }
        }
        consume(
            compiler,
            TK_RPARAN,
            b"Expected ')' after parameter list.\0" as *const u8 as *const libc::c_char,
        );
    }
    emitOpcode(compiler, call_type);
    emitByte(compiler, argc);
    if call_type as libc::c_uint == OP_METHOD_CALL as libc::c_int as libc::c_uint
        || call_type as libc::c_uint == OP_SUPER_CALL as libc::c_int as libc::c_uint
    {
        emitShort(compiler, method);
    }
    compilerChangeStack(compiler, -argc);
}
unsafe extern "C" fn _compileOptionalParanCall(
    mut compiler: *mut Compiler,
    mut method: libc::c_int,
) -> bool {
    static mut tk: [_TokenType; 2] = [TK_FN, TK_ERROR];
    let mut i: libc::c_int = 0 as libc::c_int;
    while tk[i as usize] as libc::c_uint != TK_ERROR as libc::c_int as libc::c_uint {
        if peek(compiler) as libc::c_uint == tk[i as usize] as libc::c_uint {
            (*compiler).parser.optional_call_paran = 1 as libc::c_int != 0;
            let mut call_type: Opcode = (if method >= 0 as libc::c_int {
                OP_METHOD_CALL as libc::c_int
            } else {
                OP_CALL as libc::c_int
            }) as Opcode;
            _compileCall(compiler, call_type, method);
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn exprLiteral(mut compiler: *mut Compiler) {
    let mut value: *mut Token = &mut (*compiler).parser.previous;
    let mut index: libc::c_int = compilerAddConstant(compiler, (*value).value);
    emitOpcode(compiler, OP_PUSH_CONSTANT);
    emitShort(compiler, index);
}
unsafe extern "C" fn exprInterpolation(mut compiler: *mut Compiler) {
    emitOpcode(compiler, OP_PUSH_BUILTIN_FN);
    emitByte(compiler, (*compiler).bifn_list_join);
    emitOpcode(compiler, OP_PUSH_LIST);
    let mut size_index: libc::c_int = emitShort(compiler, 0 as libc::c_int);
    let mut size: libc::c_int = 0 as libc::c_int;
    loop {
        exprLiteral(compiler);
        emitOpcode(compiler, OP_LIST_APPEND);
        size += 1;
        size;
        skipNewLines(compiler);
        compileExpression(compiler);
        emitOpcode(compiler, OP_LIST_APPEND);
        size += 1;
        size;
        skipNewLines(compiler);
        if !match_0(compiler, TK_STRING_INTERP) {
            break;
        }
    }
    consume(
        compiler,
        TK_STRING,
        b"Non terminated interpolated string.\0" as *const u8 as *const libc::c_char,
    );
    if (*compiler).parser.previous.type_0 as libc::c_uint
        == TK_STRING as libc::c_int as libc::c_uint
    {
        let mut str: *mut String_0 = ((*compiler).parser.previous.value
            & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
            as *mut String_0;
        if (*str).length != 0 as libc::c_int as libc::c_uint {
            exprLiteral(compiler);
            emitOpcode(compiler, OP_LIST_APPEND);
            size += 1;
            size;
        }
    }
    patchListSize(compiler, size_index, size);
    emitOpcode(compiler, OP_CALL);
    emitByte(compiler, 1 as libc::c_int);
    compilerChangeStack(compiler, -(1 as libc::c_int));
}
unsafe extern "C" fn exprFunction(mut compiler: *mut Compiler) {
    let mut can_define: bool = (*compiler).can_define;
    (*compiler).can_define = 1 as libc::c_int != 0;
    compileFunction(compiler, FUNC_LITERAL);
    (*compiler).can_define = can_define;
}
unsafe extern "C" fn exprName(mut compiler: *mut Compiler) {
    let mut tkname: Token = (*compiler).parser.previous;
    let mut start: *const libc::c_char = tkname.start;
    let mut length: libc::c_int = tkname.length;
    let mut line: libc::c_int = tkname.line;
    let mut result: NameSearchResult = compilerSearchName(
        compiler,
        start,
        length as uint32_t,
    );
    if (*compiler).l_value as libc::c_int != 0
        && matchAssignment(compiler) as libc::c_int != 0
    {
        let mut assignment: _TokenType = (*compiler).parser.previous.type_0;
        skipNewLines(compiler);
        let mut name_type: NameDefnType = result.type_0;
        let mut index: libc::c_int = result.index;
        let mut new_local: bool = 0 as libc::c_int != 0;
        if assignment as libc::c_uint == TK_EQ as libc::c_int as libc::c_uint {
            if result.type_0 as libc::c_uint
                == NAME_NOT_DEFINED as libc::c_int as libc::c_uint
                || result.type_0 as libc::c_uint
                    == NAME_BUILTIN_FN as libc::c_int as libc::c_uint
                || result.type_0 as libc::c_uint
                    == NAME_BUILTIN_TY as libc::c_int as libc::c_uint
            {
                name_type = (if (*compiler).scope_depth == DEPTH_GLOBAL as libc::c_int {
                    NAME_GLOBAL_VAR as libc::c_int
                } else {
                    NAME_LOCAL_VAR as libc::c_int
                }) as NameDefnType;
                index = compilerAddVariable(compiler, start, length as uint32_t, line);
                if name_type as libc::c_uint
                    == NAME_LOCAL_VAR as libc::c_int as libc::c_uint
                {
                    new_local = 1 as libc::c_int != 0;
                }
                if !(*compiler).can_define {
                    semanticError(
                        compiler,
                        tkname,
                        b"Variable definition isn't allowed here.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            let mut can_define: bool = (*compiler).can_define;
            (*compiler).can_define = 0 as libc::c_int != 0;
            compileExpression(compiler);
            (*compiler).can_define = can_define;
        } else {
            if result.type_0 as libc::c_uint
                == NAME_NOT_DEFINED as libc::c_int as libc::c_uint
            {
                semanticError(
                    compiler,
                    tkname,
                    b"Name '%.*s' is not defined.\0" as *const u8 as *const libc::c_char,
                    length,
                    start,
                );
            }
            emitPushValue(compiler, name_type, index);
            compileExpression(compiler);
            emitAssignedOp(compiler, assignment);
        }
        if new_local {
            (*compiler).new_local = 1 as libc::c_int != 0;
        } else {
            emitStoreValue(compiler, name_type, index);
        }
    } else {
        if result.type_0 as libc::c_uint
            == NAME_NOT_DEFINED as libc::c_int as libc::c_uint
        {
            if (*compiler).scope_depth == DEPTH_GLOBAL as libc::c_int {
                semanticError(
                    compiler,
                    tkname,
                    b"Name '%.*s' is not defined.\0" as *const u8 as *const libc::c_char,
                    length,
                    start,
                );
            } else {
                emitOpcode(compiler, OP_PUSH_GLOBAL);
                let mut index_0: libc::c_int = emitByte(compiler, 0xff as libc::c_int);
                compilerAddForward(
                    compiler,
                    index_0,
                    (*(*(*compiler).func).ptr).c2rust_unnamed.fn_0,
                    &mut tkname,
                );
            }
        } else {
            emitPushValue(compiler, result.type_0, result.index);
        }
        _compileOptionalParanCall(compiler, -(1 as libc::c_int));
    };
}
unsafe extern "C" fn exprOr(mut compiler: *mut Compiler) {
    emitOpcode(compiler, OP_OR);
    let mut orpatch: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
    skipNewLines(compiler);
    parsePrecedence(compiler, PREC_LOGICAL_OR);
    patchJump(compiler, orpatch);
}
unsafe extern "C" fn exprAnd(mut compiler: *mut Compiler) {
    emitOpcode(compiler, OP_AND);
    let mut andpatch: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
    skipNewLines(compiler);
    parsePrecedence(compiler, PREC_LOGICAL_AND);
    patchJump(compiler, andpatch);
}
unsafe extern "C" fn exprBinaryOp(mut compiler: *mut Compiler) {
    let mut op: _TokenType = (*compiler).parser.previous.type_0;
    skipNewLines(compiler);
    parsePrecedence(
        compiler,
        ((*getRule(op)).precedence as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as Precedence,
    );
    match op as libc::c_uint {
        4 => {
            emitOpcode(compiler, OP_RANGE);
        }
        15 => {
            emitOpcode(compiler, OP_MOD);
            emitByte(compiler, 0 as libc::c_int);
        }
        21 => {
            emitOpcode(compiler, OP_ADD);
            emitByte(compiler, 0 as libc::c_int);
        }
        22 => {
            emitOpcode(compiler, OP_SUBTRACT);
            emitByte(compiler, 0 as libc::c_int);
        }
        23 => {
            emitOpcode(compiler, OP_MULTIPLY);
            emitByte(compiler, 0 as libc::c_int);
        }
        24 => {
            emitOpcode(compiler, OP_DIVIDE);
            emitByte(compiler, 0 as libc::c_int);
        }
        25 => {
            emitOpcode(compiler, OP_EXPONENT);
            emitByte(compiler, 0 as libc::c_int);
        }
        17 => {
            emitOpcode(compiler, OP_BIT_AND);
            emitByte(compiler, 0 as libc::c_int);
        }
        18 => {
            emitOpcode(compiler, OP_BIT_OR);
            emitByte(compiler, 0 as libc::c_int);
        }
        19 => {
            emitOpcode(compiler, OP_BIT_XOR);
            emitByte(compiler, 0 as libc::c_int);
        }
        43 => {
            emitOpcode(compiler, OP_BIT_RSHIFT);
            emitByte(compiler, 0 as libc::c_int);
        }
        44 => {
            emitOpcode(compiler, OP_BIT_LSHIFT);
            emitByte(compiler, 0 as libc::c_int);
        }
        28 => {
            emitOpcode(compiler, OP_GT);
        }
        29 => {
            emitOpcode(compiler, OP_LT);
        }
        30 => {
            emitOpcode(compiler, OP_EQEQ);
        }
        31 => {
            emitOpcode(compiler, OP_NOTEQ);
        }
        32 => {
            emitOpcode(compiler, OP_GTEQ);
        }
        33 => {
            emitOpcode(compiler, OP_LTEQ);
        }
        56 => {
            emitOpcode(compiler, OP_IN);
        }
        57 => {
            emitOpcode(compiler, OP_IS);
        }
        _ => {
            unreachable!();
        }
    };
}
unsafe extern "C" fn exprUnaryOp(mut compiler: *mut Compiler) {
    let mut op: _TokenType = (*compiler).parser.previous.type_0;
    skipNewLines(compiler);
    parsePrecedence(
        compiler,
        (PREC_UNARY as libc::c_int + 1 as libc::c_int) as Precedence,
    );
    match op as libc::c_uint {
        16 => {
            emitOpcode(compiler, OP_BIT_NOT);
        }
        21 => {
            emitOpcode(compiler, OP_POSITIVE);
        }
        22 => {
            emitOpcode(compiler, OP_NEGATIVE);
        }
        60 => {
            emitOpcode(compiler, OP_NOT);
        }
        _ => {
            unreachable!();
        }
    };
}
unsafe extern "C" fn exprGrouping(mut compiler: *mut Compiler) {
    skipNewLines(compiler);
    compileExpression(compiler);
    skipNewLines(compiler);
    consume(
        compiler,
        TK_RPARAN,
        b"Expected ')' after expression.\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn exprList(mut compiler: *mut Compiler) {
    emitOpcode(compiler, OP_PUSH_LIST);
    let mut size_index: libc::c_int = emitShort(compiler, 0 as libc::c_int);
    let mut size: libc::c_int = 0 as libc::c_int;
    loop {
        skipNewLines(compiler);
        if peek(compiler) as libc::c_uint == TK_RBRACKET as libc::c_int as libc::c_uint {
            break;
        }
        compileExpression(compiler);
        emitOpcode(compiler, OP_LIST_APPEND);
        size += 1;
        size;
        skipNewLines(compiler);
        if !match_0(compiler, TK_COMMA) {
            break;
        }
    }
    skipNewLines(compiler);
    consume(
        compiler,
        TK_RBRACKET,
        b"Expected ']' after list elements.\0" as *const u8 as *const libc::c_char,
    );
    patchListSize(compiler, size_index, size);
}
unsafe extern "C" fn exprMap(mut compiler: *mut Compiler) {
    emitOpcode(compiler, OP_PUSH_MAP);
    loop {
        skipNewLines(compiler);
        if peek(compiler) as libc::c_uint == TK_RBRACE as libc::c_int as libc::c_uint {
            break;
        }
        compileExpression(compiler);
        consume(
            compiler,
            TK_COLLON,
            b"Expected ':' after map's key.\0" as *const u8 as *const libc::c_char,
        );
        compileExpression(compiler);
        emitOpcode(compiler, OP_MAP_INSERT);
        skipNewLines(compiler);
        if !match_0(compiler, TK_COMMA) {
            break;
        }
    }
    skipNewLines(compiler);
    consume(
        compiler,
        TK_RBRACE,
        b"Expected '}' after map elements.\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn exprCall(mut compiler: *mut Compiler) {
    _compileCall(compiler, OP_CALL, -(1 as libc::c_int));
}
unsafe extern "C" fn exprAttrib(mut compiler: *mut Compiler) {
    consume(
        compiler,
        TK_NAME,
        b"Expected an attribute name after '.'.\0" as *const u8 as *const libc::c_char,
    );
    let mut name: *const libc::c_char = (*compiler).parser.previous.start;
    let mut length: libc::c_int = (*compiler).parser.previous.length;
    let mut index: libc::c_int = 0 as libc::c_int;
    moduleAddString(
        (*compiler).module,
        (*compiler).parser.vm,
        name,
        length as uint32_t,
        &mut index,
    );
    if match_0(compiler, TK_LPARAN) {
        _compileCall(compiler, OP_METHOD_CALL, index);
        return;
    }
    if _compileOptionalParanCall(compiler, index) {
        return;
    }
    if (*compiler).l_value as libc::c_int != 0
        && matchAssignment(compiler) as libc::c_int != 0
    {
        let mut assignment: _TokenType = (*compiler).parser.previous.type_0;
        skipNewLines(compiler);
        if assignment as libc::c_uint != TK_EQ as libc::c_int as libc::c_uint {
            emitOpcode(compiler, OP_GET_ATTRIB_KEEP);
            emitShort(compiler, index);
            compileExpression(compiler);
            emitAssignedOp(compiler, assignment);
        } else {
            compileExpression(compiler);
        }
        emitOpcode(compiler, OP_SET_ATTRIB);
        emitShort(compiler, index);
    } else {
        emitOpcode(compiler, OP_GET_ATTRIB);
        emitShort(compiler, index);
    };
}
unsafe extern "C" fn exprSubscript(mut compiler: *mut Compiler) {
    compileExpression(compiler);
    consume(
        compiler,
        TK_RBRACKET,
        b"Expected ']' after subscription ends.\0" as *const u8 as *const libc::c_char,
    );
    if (*compiler).l_value as libc::c_int != 0
        && matchAssignment(compiler) as libc::c_int != 0
    {
        let mut assignment: _TokenType = (*compiler).parser.previous.type_0;
        skipNewLines(compiler);
        if assignment as libc::c_uint != TK_EQ as libc::c_int as libc::c_uint {
            emitOpcode(compiler, OP_GET_SUBSCRIPT_KEEP);
            compileExpression(compiler);
            emitAssignedOp(compiler, assignment);
        } else {
            compileExpression(compiler);
        }
        emitOpcode(compiler, OP_SET_SUBSCRIPT);
    } else {
        emitOpcode(compiler, OP_GET_SUBSCRIPT);
    };
}
unsafe extern "C" fn exprValue(mut compiler: *mut Compiler) {
    let mut op: _TokenType = (*compiler).parser.previous.type_0;
    match op as libc::c_uint {
        55 => {
            emitOpcode(compiler, OP_PUSH_NULL);
        }
        61 => {
            emitOpcode(compiler, OP_PUSH_TRUE);
        }
        62 => {
            emitOpcode(compiler, OP_PUSH_FALSE);
        }
        _ => {
            unreachable!();
        }
    };
}
unsafe extern "C" fn exprSelf(mut compiler: *mut Compiler) {
    if (*(*compiler).func).type_0 as libc::c_uint
        == FUNC_CONSTRUCTOR as libc::c_int as libc::c_uint
        || (*(*compiler).func).type_0 as libc::c_uint
            == FUNC_METHOD as libc::c_int as libc::c_uint
    {
        emitOpcode(compiler, OP_PUSH_SELF);
        return;
    }
    if !(*compiler).parser.parsing_class {
        semanticError(
            compiler,
            (*compiler).parser.previous,
            b"Invalid use of 'self'.\0" as *const u8 as *const libc::c_char,
        );
    } else {
        semanticError(
            compiler,
            (*compiler).parser.previous,
            b"TODO: Closures cannot capture 'self' for now.\0" as *const u8
                as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn exprSuper(mut compiler: *mut Compiler) {
    if (*(*compiler).func).type_0 as libc::c_uint
        != FUNC_CONSTRUCTOR as libc::c_int as libc::c_uint
        && (*(*compiler).func).type_0 as libc::c_uint
            != FUNC_METHOD as libc::c_int as libc::c_uint
    {
        semanticError(
            compiler,
            (*compiler).parser.previous,
            b"Invalid use of 'super'.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut name: *const libc::c_char = (*(*(*compiler).func).ptr).name;
    let mut name_length: libc::c_int = -(1 as libc::c_int);
    if !match_0(compiler, TK_LPARAN) {
        consume(
            compiler,
            TK_DOT,
            b"Invalid use of 'super'.\0" as *const u8 as *const libc::c_char,
        );
        consume(
            compiler,
            TK_NAME,
            b"Expected a method name after 'super'.\0" as *const u8
                as *const libc::c_char,
        );
        name = (*compiler).parser.previous.start;
        name_length = (*compiler).parser.previous.length;
        consume(
            compiler,
            TK_LPARAN,
            b"Expected symbol '('.\0" as *const u8 as *const libc::c_char,
        );
    } else {
        name_length = strlen(name) as libc::c_int;
    }
    if (*compiler).parser.has_syntax_error {
        return;
    }
    emitOpcode(compiler, OP_PUSH_SELF);
    moduleAddString(
        (*compiler).module,
        (*compiler).parser.vm,
        name,
        name_length as uint32_t,
        &mut index,
    );
    _compileCall(compiler, OP_SUPER_CALL, index);
}
unsafe extern "C" fn parsePrecedence(
    mut compiler: *mut Compiler,
    mut precedence: Precedence,
) {
    lexToken(compiler);
    if (*compiler).parser.has_syntax_error {
        return;
    }
    let mut prefix: GrammarFn = (*getRule((*compiler).parser.previous.type_0)).prefix;
    if prefix.is_none() {
        syntaxError(
            compiler,
            (*compiler).parser.previous,
            b"Expected an expression.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut l_value: bool = (*compiler).l_value;
    let mut can_define: bool = (*compiler).can_define;
    if prefix != Some(exprName as unsafe extern "C" fn(*mut Compiler) -> ()) {
        (*compiler).can_define = 0 as libc::c_int != 0;
    }
    (*compiler)
        .l_value = precedence as libc::c_uint
        <= PREC_LOWEST as libc::c_int as libc::c_uint;
    prefix.unwrap()(compiler);
    (*compiler).can_define = 0 as libc::c_int != 0;
    (*compiler).is_last_call = 0 as libc::c_int != 0;
    while (*getRule((*compiler).parser.current.type_0)).precedence as libc::c_uint
        >= precedence as libc::c_uint
    {
        lexToken(compiler);
        if (*compiler).parser.has_syntax_error {
            return;
        }
        let mut op: _TokenType = (*compiler).parser.previous.type_0;
        let mut infix: GrammarFn = (*getRule(op)).infix;
        infix.unwrap()(compiler);
        (*compiler)
            .is_last_call = op as libc::c_uint
            == TK_LPARAN as libc::c_int as libc::c_uint;
    }
    (*compiler).l_value = l_value;
    (*compiler).can_define = can_define;
}
unsafe extern "C" fn compilerAddVariable(
    mut compiler: *mut Compiler,
    mut name: *const libc::c_char,
    mut length: uint32_t,
    mut line: libc::c_int,
) -> libc::c_int {
    let mut max_vars_reached: bool = 0 as libc::c_int != 0;
    let mut var_type: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    if (*compiler).scope_depth == DEPTH_GLOBAL as libc::c_int {
        if (*(*compiler).module).globals.count >= 256 as libc::c_int as libc::c_uint {
            max_vars_reached = 1 as libc::c_int != 0;
            var_type = b"globals\0" as *const u8 as *const libc::c_char;
        }
    } else if (*(*compiler).func).local_count >= 256 as libc::c_int {
        max_vars_reached = 1 as libc::c_int != 0;
        var_type = b"locals\0" as *const u8 as *const libc::c_char;
    }
    if max_vars_reached {
        semanticError(
            compiler,
            (*compiler).parser.previous,
            b"A module should contain at most %d %s.\0" as *const u8
                as *const libc::c_char,
            256 as libc::c_int,
            var_type,
        );
        return -(1 as libc::c_int);
    }
    if (*compiler).scope_depth == DEPTH_GLOBAL as libc::c_int {
        return moduleSetGlobal(
            (*compiler).parser.vm,
            (*compiler).module,
            name,
            length,
            0x7ffc000000000000 as libc::c_long as uint64_t | 0 as libc::c_int as uint64_t,
        ) as libc::c_int
    } else {
        let mut local: *mut Local = &mut *((*(*compiler).func).locals)
            .as_mut_ptr()
            .offset((*(*compiler).func).local_count as isize) as *mut Local;
        (*local).name = name;
        (*local).length = length;
        (*local).depth = (*compiler).scope_depth;
        (*local).is_upvalue = 0 as libc::c_int != 0;
        (*local).line = line;
        let fresh1 = (*(*compiler).func).local_count;
        (*(*compiler).func).local_count = (*(*compiler).func).local_count + 1;
        return fresh1;
    };
}
unsafe extern "C" fn compilerAddForward(
    mut compiler: *mut Compiler,
    mut instruction: libc::c_int,
    mut fn_0: *mut Fn_0,
    mut tkname: *mut Token,
) {
    if (*compiler).parser.forwards_count == 256 as libc::c_int {
        semanticError(
            compiler,
            *tkname,
            b"A module should contain at most %d implicit forward function declarations.\0"
                as *const u8 as *const libc::c_char,
            256 as libc::c_int,
        );
        return;
    }
    let fresh2 = (*compiler).parser.forwards_count;
    (*compiler).parser.forwards_count = (*compiler).parser.forwards_count + 1;
    let mut forward: *mut ForwardName = &mut *((*compiler).parser.forwards)
        .as_mut_ptr()
        .offset(fresh2 as isize) as *mut ForwardName;
    (*forward).instruction = instruction;
    (*forward).func = fn_0;
    (*forward).tkname = *tkname;
}
unsafe extern "C" fn compilerAddConstant(
    mut compiler: *mut Compiler,
    mut value: Var,
) -> libc::c_int {
    let mut index: uint32_t = moduleAddConstant(
        (*compiler).parser.vm,
        (*compiler).module,
        value,
    );
    checkMaxConstantsReached(compiler, index as libc::c_int);
    return index as libc::c_int;
}
unsafe extern "C" fn compilerEnterBlock(mut compiler: *mut Compiler) {
    (*compiler).scope_depth += 1;
    (*compiler).scope_depth;
}
unsafe extern "C" fn compilerChangeStack(
    mut compiler: *mut Compiler,
    mut num: libc::c_int,
) {
    (*(*compiler).func).stack_size += num;
    if (*(*compiler).func).stack_size
        > (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).stack_size
    {
        (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0)
            .stack_size = (*(*compiler).func).stack_size;
    }
}
unsafe extern "C" fn compilerPopLocals(
    mut compiler: *mut Compiler,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut local: libc::c_int = (*(*compiler).func).local_count - 1 as libc::c_int;
    while local >= 0 as libc::c_int
        && (*(*compiler).func).locals[local as usize].depth >= depth
    {
        if (*(*compiler).func).locals[local as usize].is_upvalue {
            emitByte(compiler, OP_CLOSE_UPVALUE as libc::c_int);
        } else {
            emitByte(compiler, OP_POP as libc::c_int);
        }
        local -= 1;
        local;
    }
    return (*(*compiler).func).local_count - 1 as libc::c_int - local;
}
unsafe extern "C" fn compilerExitBlock(mut compiler: *mut Compiler) {
    let mut popped: libc::c_int = compilerPopLocals(compiler, (*compiler).scope_depth);
    (*(*compiler).func).local_count -= popped;
    (*(*compiler).func).stack_size -= popped;
    (*compiler).scope_depth -= 1;
    (*compiler).scope_depth;
}
unsafe extern "C" fn compilerPushFunc(
    mut compiler: *mut Compiler,
    mut fn_0: *mut Func,
    mut func: *mut Function,
    mut type_0: FuncType,
) {
    (*fn_0).type_0 = type_0;
    (*fn_0).outer_func = (*compiler).func;
    (*fn_0).local_count = 0 as libc::c_int;
    (*fn_0).stack_size = 0 as libc::c_int;
    (*fn_0).ptr = func;
    (*fn_0).depth = (*compiler).scope_depth;
    (*compiler).func = fn_0;
}
unsafe extern "C" fn compilerPopFunc(mut compiler: *mut Compiler) {
    (*compiler).func = (*(*compiler).func).outer_func;
}
unsafe extern "C" fn emitByte(
    mut compiler: *mut Compiler,
    mut byte: libc::c_int,
) -> libc::c_int {
    pkByteBufferWrite(
        &mut (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes,
        (*compiler).parser.vm,
        byte as uint8_t,
    );
    pkUintBufferWrite(
        &mut (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).oplines,
        (*compiler).parser.vm,
        (*compiler).parser.previous.line as uint32_t,
    );
    return (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.count as libc::c_int
        - 1 as libc::c_int;
}
unsafe extern "C" fn emitShort(
    mut compiler: *mut Compiler,
    mut arg: libc::c_int,
) -> libc::c_int {
    emitByte(compiler, arg >> 8 as libc::c_int & 0xff as libc::c_int);
    return emitByte(compiler, arg & 0xff as libc::c_int) - 1 as libc::c_int;
}
unsafe extern "C" fn emitOpcode(mut compiler: *mut Compiler, mut opcode: Opcode) {
    emitByte(compiler, opcode as libc::c_int);
    compilerChangeStack(compiler, opcode_info[opcode as usize].stack);
}
unsafe extern "C" fn emitLoopJump(mut compiler: *mut Compiler) {
    emitOpcode(compiler, OP_LOOP);
    let mut offset: libc::c_int = (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0)
        .opcodes
        .count as libc::c_int - (*(*compiler).loop_0).start + 2 as libc::c_int;
    emitShort(compiler, offset);
}
unsafe extern "C" fn emitAssignedOp(
    mut compiler: *mut Compiler,
    mut assignment: _TokenType,
) {
    match assignment as libc::c_uint {
        34 => {
            emitOpcode(compiler, OP_ADD);
            emitByte(compiler, 1 as libc::c_int);
        }
        35 => {
            emitOpcode(compiler, OP_SUBTRACT);
            emitByte(compiler, 1 as libc::c_int);
        }
        36 => {
            emitOpcode(compiler, OP_MULTIPLY);
            emitByte(compiler, 1 as libc::c_int);
        }
        37 => {
            emitOpcode(compiler, OP_DIVIDE);
            emitByte(compiler, 1 as libc::c_int);
        }
        38 => {
            emitOpcode(compiler, OP_MOD);
            emitByte(compiler, 1 as libc::c_int);
        }
        39 => {
            emitOpcode(compiler, OP_EXPONENT);
            emitByte(compiler, 1 as libc::c_int);
        }
        40 => {
            emitOpcode(compiler, OP_BIT_AND);
            emitByte(compiler, 1 as libc::c_int);
        }
        41 => {
            emitOpcode(compiler, OP_BIT_OR);
            emitByte(compiler, 1 as libc::c_int);
        }
        42 => {
            emitOpcode(compiler, OP_BIT_XOR);
            emitByte(compiler, 1 as libc::c_int);
        }
        45 => {
            emitOpcode(compiler, OP_BIT_RSHIFT);
            emitByte(compiler, 1 as libc::c_int);
        }
        46 => {
            emitOpcode(compiler, OP_BIT_LSHIFT);
            emitByte(compiler, 1 as libc::c_int);
        }
        _ => {
            unreachable!();
        }
    };
}
unsafe extern "C" fn emitFunctionEnd(mut compiler: *mut Compiler) {
    emitByte(compiler, OP_RETURN as libc::c_int);
    emitOpcode(compiler, OP_END);
}
unsafe extern "C" fn patchJump(
    mut compiler: *mut Compiler,
    mut addr_index: libc::c_int,
) {
    let mut offset: libc::c_int = (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0)
        .opcodes
        .count as libc::c_int - (addr_index + 2 as libc::c_int);
    *((*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.data)
        .offset(
            addr_index as isize,
        ) = (offset >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    *((*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.data)
        .offset(
            (addr_index + 1 as libc::c_int) as isize,
        ) = (offset & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn patchListSize(
    mut compiler: *mut Compiler,
    mut size_index: libc::c_int,
    mut size: libc::c_int,
) {
    *((*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.data)
        .offset(
            size_index as isize,
        ) = (size >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
    *((*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.data)
        .offset(
            (size_index + 1 as libc::c_int) as isize,
        ) = (size & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn patchForward(
    mut compiler: *mut Compiler,
    mut fn_0: *mut Fn_0,
    mut index: libc::c_int,
    mut name: libc::c_int,
) {
    *((*fn_0).opcodes.data)
        .offset(index as isize) = (name & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn compileClass(mut compiler: *mut Compiler) -> libc::c_int {
    consume(
        compiler,
        TK_NAME,
        b"Expected a class name.\0" as *const u8 as *const libc::c_char,
    );
    let mut name: *const libc::c_char = (*compiler).parser.previous.start;
    let mut name_len: libc::c_int = (*compiler).parser.previous.length;
    let mut name_line: libc::c_int = (*compiler).parser.previous.line;
    let mut cls_index: libc::c_int = 0;
    let mut _vm: *mut PKVM = (*compiler).parser.vm;
    let mut cls: *mut Class = newClass(
        _vm,
        name,
        name_len,
        (*_vm).builtin_classes[PK_OBJECT as libc::c_int as usize],
        (*compiler).module,
        0 as *const libc::c_char,
        &mut cls_index,
    );
    vmPushTempRef(_vm, &mut (*cls)._super);
    (*compiler).parser.parsing_class = 1 as libc::c_int != 0;
    checkMaxConstantsReached(compiler, cls_index);
    if match_0(compiler, TK_IS) {
        consume(
            compiler,
            TK_NAME,
            b"Expected a class name to inherit.\0" as *const u8 as *const libc::c_char,
        );
        if !(*compiler).parser.has_syntax_error {
            exprName(compiler);
        }
    } else {
        emitPushValue(compiler, NAME_BUILTIN_TY, PK_OBJECT as libc::c_int);
    }
    emitOpcode(compiler, OP_CREATE_CLASS);
    emitShort(compiler, cls_index);
    skipNewLines(compiler);
    if match_0(compiler, TK_STRING) {
        let mut str: *mut Token = &mut (*compiler).parser.previous;
        let mut index: libc::c_int = compilerAddConstant(compiler, (*str).value);
        let mut docstring: *mut String_0 = moduleGetStringAt((*compiler).module, index);
        (*cls).docstring = ((*docstring).data).as_mut_ptr();
    }
    skipNewLines(compiler);
    while !(*compiler).parser.has_syntax_error && !match_0(compiler, TK_END) {
        if match_0(compiler, TK_EOF) {
            syntaxError(
                compiler,
                (*compiler).parser.previous,
                b"Unexpected EOF while parsing class.\0" as *const u8
                    as *const libc::c_char,
            );
            break;
        } else {
            consume(
                compiler,
                TK_DEF,
                b"Expected method definition.\0" as *const u8 as *const libc::c_char,
            );
            if (*compiler).parser.has_syntax_error {
                break;
            }
            compileFunction(compiler, FUNC_METHOD);
            if (*compiler).parser.has_syntax_error {
                break;
            }
            skipNewLines(compiler);
        }
    }
    let mut global_index: libc::c_int = compilerAddVariable(
        compiler,
        name,
        name_len as uint32_t,
        name_line,
    );
    emitStoreValue(compiler, NAME_GLOBAL_VAR, global_index);
    emitOpcode(compiler, OP_POP);
    (*compiler).parser.parsing_class = 0 as libc::c_int != 0;
    vmPopTempRef(_vm);
    return cls_index;
}
unsafe extern "C" fn matchOperatorMethod(
    mut compiler: *mut Compiler,
    mut name: *mut *const libc::c_char,
    mut length: *mut libc::c_int,
    mut argc: *mut libc::c_int,
) -> bool {
    if match_0(compiler, TK_PLUS) {
        if match_0(compiler, TK_SELF) {
            *name = b"+self\0" as *const u8 as *const libc::c_char;
            *length = strlen(b"+self\0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            *argc = 0 as libc::c_int;
            return 1 as libc::c_int != 0;
        } else {
            *name = b"+\0" as *const u8 as *const libc::c_char;
            *length = strlen(b"+\0" as *const u8 as *const libc::c_char) as libc::c_int;
            *argc = 1 as libc::c_int;
            return 1 as libc::c_int != 0;
        }
    }
    if match_0(compiler, TK_MINUS) {
        if match_0(compiler, TK_SELF) {
            *name = b"-self\0" as *const u8 as *const libc::c_char;
            *length = strlen(b"-self\0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            *argc = 0 as libc::c_int;
            return 1 as libc::c_int != 0;
        } else {
            *name = b"-\0" as *const u8 as *const libc::c_char;
            *length = strlen(b"-\0" as *const u8 as *const libc::c_char) as libc::c_int;
            *argc = 1 as libc::c_int;
            return 1 as libc::c_int != 0;
        }
    }
    if match_0(compiler, TK_TILD) {
        if match_0(compiler, TK_SELF) {
            *name = b"~self\0" as *const u8 as *const libc::c_char;
            *length = strlen(b"~self\0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            *argc = 0 as libc::c_int;
            return 1 as libc::c_int != 0;
        }
        syntaxError(
            compiler,
            (*compiler).parser.previous,
            b"Expected keyword self for unary operator definition.\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if match_0(compiler, TK_NOT) {
        if match_0(compiler, TK_SELF) {
            *name = b"!self\0" as *const u8 as *const libc::c_char;
            *length = strlen(b"!self\0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            *argc = 0 as libc::c_int;
            return 1 as libc::c_int != 0;
        }
        syntaxError(
            compiler,
            (*compiler).parser.previous,
            b"Expected keyword self for unary operator definition.\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if match_0(compiler, TK_LBRACKET) {
        if match_0(compiler, TK_RBRACKET) {
            if match_0(compiler, TK_EQ) {
                *name = b"[]=\0" as *const u8 as *const libc::c_char;
                *length = strlen(b"[]=\0" as *const u8 as *const libc::c_char)
                    as libc::c_int;
                *argc = 2 as libc::c_int;
                return 1 as libc::c_int != 0;
            }
            *name = b"[]\0" as *const u8 as *const libc::c_char;
            *length = strlen(b"[]\0" as *const u8 as *const libc::c_char) as libc::c_int;
            *argc = 1 as libc::c_int;
            return 1 as libc::c_int != 0;
        }
        syntaxError(
            compiler,
            (*compiler).parser.previous,
            b"Invalid operator method symbol.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if match_0(compiler, TK_PLUSEQ) {
        *name = b"+=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"+=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_MINUSEQ) {
        *name = b"-=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"-=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_STAR) {
        *name = b"*\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"*\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_STAREQ) {
        *name = b"*=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"*=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_FSLASH) {
        *name = b"/\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"/\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_STARSTAR) {
        *name = b"**\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"**\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_DIVEQ) {
        *name = b"/=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"/=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_PERCENT) {
        *name = b"%\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"%\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_MODEQ) {
        *name = b"%=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"%=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_POWEQ) {
        *name = b"**=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"**=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_AMP) {
        *name = b"&\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"&\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_ANDEQ) {
        *name = b"&=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"&=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_PIPE) {
        *name = b"|\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"|\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_OREQ) {
        *name = b"|=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"|=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_CARET) {
        *name = b"^\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"^\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_XOREQ) {
        *name = b"^=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"^=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_SLEFT) {
        *name = b"<<\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"<<\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_SLEFTEQ) {
        *name = b"<<=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"<<=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_SRIGHT) {
        *name = b">>\0" as *const u8 as *const libc::c_char;
        *length = strlen(b">>\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_SRIGHTEQ) {
        *name = b">>=\0" as *const u8 as *const libc::c_char;
        *length = strlen(b">>=\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_EQEQ) {
        *name = b"==\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"==\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_GT) {
        *name = b">\0" as *const u8 as *const libc::c_char;
        *length = strlen(b">\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_LT) {
        *name = b"<\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"<\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_DOTDOT) {
        *name = b"..\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"..\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    if match_0(compiler, TK_IN) {
        *name = b"in\0" as *const u8 as *const libc::c_char;
        *length = strlen(b"in\0" as *const u8 as *const libc::c_char) as libc::c_int;
        *argc = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn compileFunction(
    mut compiler: *mut Compiler,
    mut fn_type: FuncType,
) {
    let mut name: *const libc::c_char = b"(?)\0" as *const u8 as *const libc::c_char;
    let mut name_length: libc::c_int = 3 as libc::c_int;
    let mut operator_argc: libc::c_int = -(2 as libc::c_int);
    if fn_type as libc::c_uint != FUNC_LITERAL as libc::c_int as libc::c_uint {
        if match_0(compiler, TK_NAME) {
            name = (*compiler).parser.previous.start;
            name_length = (*compiler).parser.previous.length;
        } else if !(fn_type as libc::c_uint == FUNC_METHOD as libc::c_int as libc::c_uint
            && matchOperatorMethod(
                compiler,
                &mut name,
                &mut name_length,
                &mut operator_argc,
            ) as libc::c_int != 0)
        {
            if !(*compiler).parser.has_syntax_error {
                syntaxError(
                    compiler,
                    (*compiler).parser.previous,
                    b"Expected a function name.\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    } else {
        name = b"@func\0" as *const u8 as *const libc::c_char;
        name_length = strlen(name) as libc::c_int;
    }
    if (*compiler).parser.has_syntax_error {
        return;
    }
    let mut fn_index: libc::c_int = 0;
    let mut func: *mut Function = newFunction(
        (*compiler).parser.vm,
        name,
        name_length,
        (*compiler).module,
        0 as libc::c_int != 0,
        0 as *const libc::c_char,
        &mut fn_index,
    );
    (*func)
        .is_method = fn_type as libc::c_uint
        == FUNC_METHOD as libc::c_int as libc::c_uint
        || fn_type as libc::c_uint == FUNC_CONSTRUCTOR as libc::c_int as libc::c_uint;
    checkMaxConstantsReached(compiler, fn_index);
    let mut global_index: libc::c_int = -(1 as libc::c_int);
    if fn_type as libc::c_uint == FUNC_TOPLEVEL as libc::c_int as libc::c_uint {
        let mut name_line: libc::c_int = (*compiler).parser.previous.line;
        global_index = compilerAddVariable(
            compiler,
            name,
            name_length as uint32_t,
            name_line,
        );
    }
    if fn_type as libc::c_uint == FUNC_METHOD as libc::c_int as libc::c_uint
        && strncmp(
            name,
            b"_init\0" as *const u8 as *const libc::c_char,
            name_length as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        fn_type = FUNC_CONSTRUCTOR;
    }
    let mut curr_fn: Func = Func {
        type_0: FUNC_MAIN,
        depth: 0,
        locals: [Local {
            name: 0 as *const libc::c_char,
            length: 0,
            depth: 0,
            is_upvalue: false,
            line: 0,
        }; 256],
        local_count: 0,
        upvalues: [UpvalueInfo {
            is_immediate: false,
            index: 0,
        }; 256],
        stack_size: 0,
        ptr: 0 as *mut Function,
        outer_func: 0 as *mut sFunc,
    };
    compilerPushFunc(compiler, &mut curr_fn, func, fn_type);
    let mut argc: libc::c_int = 0 as libc::c_int;
    compilerEnterBlock(compiler);
    if match_0(compiler, TK_LPARAN) as libc::c_int != 0 && !match_0(compiler, TK_RPARAN)
    {
        loop {
            skipNewLines(compiler);
            consume(
                compiler,
                TK_NAME,
                b"Expected a parameter name.\0" as *const u8 as *const libc::c_char,
            );
            argc += 1;
            argc;
            let mut param_name: *const libc::c_char = (*compiler).parser.previous.start;
            let mut param_len: uint32_t = (*compiler).parser.previous.length as uint32_t;
            let mut predefined: bool = 0 as libc::c_int != 0;
            let mut i: libc::c_int = (*(*compiler).func).local_count - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                let mut local: *mut Local = &mut *((*(*compiler).func).locals)
                    .as_mut_ptr()
                    .offset(i as isize) as *mut Local;
                if (*local).length == param_len
                    && strncmp((*local).name, param_name, param_len as libc::c_ulong)
                        == 0 as libc::c_int
                {
                    predefined = 1 as libc::c_int != 0;
                    break;
                } else {
                    i -= 1;
                    i;
                }
            }
            if predefined {
                semanticError(
                    compiler,
                    (*compiler).parser.previous,
                    b"Multiple definition of a parameter.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            compilerAddVariable(
                compiler,
                param_name,
                param_len,
                (*compiler).parser.previous.line,
            );
            if !match_0(compiler, TK_COMMA) {
                break;
            }
        }
        consume(
            compiler,
            TK_RPARAN,
            b"Expected ')' after parameter list.\0" as *const u8 as *const libc::c_char,
        );
    }
    if operator_argc >= 0 as libc::c_int && argc != operator_argc {
        semanticError(
            compiler,
            (*compiler).parser.previous,
            b"Expected exactly %d parameters.\0" as *const u8 as *const libc::c_char,
            operator_argc,
        );
    }
    (*func).arity = argc;
    compilerChangeStack(compiler, argc);
    skipNewLines(compiler);
    if match_0(compiler, TK_STRING) {
        let mut str: *mut Token = &mut (*compiler).parser.previous;
        let mut index: libc::c_int = compilerAddConstant(compiler, (*str).value);
        let mut docstring: *mut String_0 = moduleGetStringAt((*compiler).module, index);
        (*func).docstring = ((*docstring).data).as_mut_ptr();
    }
    compileBlockBody(compiler, BLOCK_FUNC);
    if fn_type as libc::c_uint == FUNC_CONSTRUCTOR as libc::c_int as libc::c_uint {
        emitOpcode(compiler, OP_PUSH_SELF);
        emitOpcode(compiler, OP_RETURN);
    }
    consume(
        compiler,
        TK_END,
        b"Expected 'end' after function definition end.\0" as *const u8
            as *const libc::c_char,
    );
    compilerExitBlock(compiler);
    emitFunctionEnd(compiler);
    compilerPopFunc(compiler);
    emitOpcode(compiler, OP_PUSH_CLOSURE);
    emitShort(compiler, fn_index);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*curr_fn.ptr).upvalue_count {
        emitByte(
            compiler,
            if curr_fn.upvalues[i_0 as usize].is_immediate as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
        emitByte(compiler, curr_fn.upvalues[i_0 as usize].index);
        i_0 += 1;
        i_0;
    }
    if fn_type as libc::c_uint == FUNC_TOPLEVEL as libc::c_int as libc::c_uint {
        emitStoreValue(compiler, NAME_GLOBAL_VAR, global_index);
        emitOpcode(compiler, OP_POP);
    } else if fn_type as libc::c_uint == FUNC_METHOD as libc::c_int as libc::c_uint
        || fn_type as libc::c_uint == FUNC_CONSTRUCTOR as libc::c_int as libc::c_uint
    {
        emitOpcode(compiler, OP_BIND_METHOD);
    }
}
unsafe extern "C" fn compileBlockBody(
    mut compiler: *mut Compiler,
    mut type_0: BlockType,
) {
    compilerEnterBlock(compiler);
    if type_0 as libc::c_uint == BLOCK_IF as libc::c_int as libc::c_uint {
        consumeStartBlock(compiler, TK_THEN);
        skipNewLines(compiler);
    } else if type_0 as libc::c_uint == BLOCK_ELSE as libc::c_int as libc::c_uint {
        skipNewLines(compiler);
    } else if type_0 as libc::c_uint == BLOCK_FUNC as libc::c_int as libc::c_uint {
        skipNewLines(compiler);
    } else {
        consumeStartBlock(compiler, TK_DO);
        skipNewLines(compiler);
    }
    let mut next: _TokenType = peek(compiler);
    while !(next as libc::c_uint == TK_END as libc::c_int as libc::c_uint
        || next as libc::c_uint == TK_EOF as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == BLOCK_IF as libc::c_int as libc::c_uint
            && (next as libc::c_uint == TK_ELSE as libc::c_int as libc::c_uint
                || next as libc::c_uint == TK_ELIF as libc::c_int as libc::c_uint))
    {
        compileStatement(compiler);
        skipNewLines(compiler);
        next = peek(compiler);
    }
    compilerExitBlock(compiler);
}
unsafe extern "C" fn compileImportPath(mut compiler: *mut Compiler) -> Token {
    let mut vm: *mut PKVM = (*compiler).parser.vm;
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    if match_0(compiler, TK_DOT) {
        pkByteBufferAddString(
            &mut buff,
            vm,
            b"./\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as uint32_t,
        );
    } else {
        while match_0(compiler, TK_CARET) {
            pkByteBufferAddString(
                &mut buff,
                vm,
                b"../\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as uint32_t,
            );
        }
    }
    let mut tkmodule: Token = makeErrToken(&mut (*compiler).parser);
    loop {
        consume(
            compiler,
            TK_NAME,
            b"Expected a module name\0" as *const u8 as *const libc::c_char,
        );
        if (*compiler).parser.has_syntax_error {
            break;
        }
        if tkmodule.type_0 as libc::c_uint != TK_ERROR as libc::c_int as libc::c_uint {
            pkByteBufferWrite(&mut buff, vm, '/' as i32 as uint8_t);
        }
        tkmodule = (*compiler).parser.previous;
        pkByteBufferAddString(
            &mut buff,
            vm,
            tkmodule.start,
            tkmodule.length as uint32_t,
        );
        if !match_0(compiler, TK_DOT) {
            break;
        }
    }
    pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
    if (*compiler).parser.has_syntax_error {
        pkByteBufferClear(&mut buff, vm);
        return makeErrToken(&mut (*compiler).parser);
    }
    let mut index: libc::c_int = 0 as libc::c_int;
    moduleAddString(
        (*compiler).module,
        (*compiler).parser.vm,
        buff.data as *const libc::c_char,
        (buff.count).wrapping_sub(1 as libc::c_int as libc::c_uint),
        &mut index,
    );
    pkByteBufferClear(&mut buff, vm);
    emitOpcode(compiler, OP_IMPORT);
    emitShort(compiler, index);
    return tkmodule;
}
pub unsafe extern "C" fn compileRegularImport(mut compiler: *mut Compiler) {
    loop {
        let mut tkmodule: Token = compileImportPath(compiler);
        if tkmodule.type_0 as libc::c_uint == TK_ERROR as libc::c_int as libc::c_uint {
            return;
        }
        if match_0(compiler, TK_AS) {
            consume(
                compiler,
                TK_NAME,
                b"Expected a name after 'as'.\0" as *const u8 as *const libc::c_char,
            );
            if (*compiler).parser.has_syntax_error {
                return;
            }
            tkmodule = (*compiler).parser.previous;
        }
        let mut global_index: libc::c_int = compilerAddVariable(
            compiler,
            tkmodule.start,
            tkmodule.length as uint32_t,
            tkmodule.line,
        );
        emitStoreGlobal(compiler, global_index);
        emitOpcode(compiler, OP_POP);
        if !(match_0(compiler, TK_COMMA) as libc::c_int != 0
            && {
                skipNewLines(compiler);
                1 as libc::c_int != 0
            })
        {
            break;
        }
    }
    consumeEndStatement(compiler);
}
unsafe extern "C" fn compileFromImport(mut compiler: *mut Compiler) {
    let mut tkmodule: Token = compileImportPath(compiler);
    if tkmodule.type_0 as libc::c_uint == TK_ERROR as libc::c_int as libc::c_uint {
        return;
    }
    consume(
        compiler,
        TK_IMPORT,
        b"Expected keyword 'import'.\0" as *const u8 as *const libc::c_char,
    );
    if (*compiler).parser.has_syntax_error {
        return;
    }
    loop {
        consume(
            compiler,
            TK_NAME,
            b"Expected symbol to import.\0" as *const u8 as *const libc::c_char,
        );
        if (*compiler).parser.has_syntax_error {
            return;
        }
        let mut tkname: Token = (*compiler).parser.previous;
        let mut name_index: libc::c_int = 0 as libc::c_int;
        moduleAddString(
            (*compiler).module,
            (*compiler).parser.vm,
            tkname.start,
            tkname.length as uint32_t,
            &mut name_index,
        );
        emitOpcode(compiler, OP_GET_ATTRIB_KEEP);
        emitShort(compiler, name_index);
        if match_0(compiler, TK_AS) {
            consume(
                compiler,
                TK_NAME,
                b"Expected a name after 'as'.\0" as *const u8 as *const libc::c_char,
            );
            tkname = (*compiler).parser.previous;
        }
        let mut global_index: libc::c_int = compilerAddVariable(
            compiler,
            tkname.start,
            tkname.length as uint32_t,
            tkname.line,
        );
        emitStoreGlobal(compiler, global_index);
        emitOpcode(compiler, OP_POP);
        if !(match_0(compiler, TK_COMMA) as libc::c_int != 0
            && {
                skipNewLines(compiler);
                1 as libc::c_int != 0
            })
        {
            break;
        }
    }
    emitOpcode(compiler, OP_POP);
    consumeEndStatement(compiler);
}
unsafe extern "C" fn compileExpression(mut compiler: *mut Compiler) {
    parsePrecedence(compiler, PREC_LOWEST);
}
unsafe extern "C" fn compileIfStatement(mut compiler: *mut Compiler, mut elif: bool) {
    skipNewLines(compiler);
    let mut can_define: bool = (*compiler).can_define;
    (*compiler).can_define = 0 as libc::c_int != 0;
    compileExpression(compiler);
    (*compiler).can_define = can_define;
    emitOpcode(compiler, OP_JUMP_IF_NOT);
    let mut ifpatch: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
    compileBlockBody(compiler, BLOCK_IF);
    if match_0(compiler, TK_ELIF) {
        emitOpcode(compiler, OP_JUMP);
        let mut exit_jump: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
        patchJump(compiler, ifpatch);
        compilerEnterBlock(compiler);
        compileIfStatement(compiler, 1 as libc::c_int != 0);
        compilerExitBlock(compiler);
        patchJump(compiler, exit_jump);
    } else if match_0(compiler, TK_ELSE) {
        emitOpcode(compiler, OP_JUMP);
        let mut exit_jump_0: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
        patchJump(compiler, ifpatch);
        compileBlockBody(compiler, BLOCK_ELSE);
        patchJump(compiler, exit_jump_0);
    } else {
        patchJump(compiler, ifpatch);
    }
    if !elif {
        skipNewLines(compiler);
        consume(
            compiler,
            TK_END,
            b"Expected 'end' after statement end.\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn compileWhileStatement(mut compiler: *mut Compiler) {
    let mut loop_0: Loop = Loop {
        start: 0,
        exit_jump: 0,
        patches: [0; 256],
        patch_count: 0,
        outer_loop: 0 as *mut sLoop,
        depth: 0,
    };
    loop_0
        .start = (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.count
        as libc::c_int;
    loop_0.patch_count = 0 as libc::c_int;
    loop_0.outer_loop = (*compiler).loop_0;
    loop_0.depth = (*compiler).scope_depth;
    (*compiler).loop_0 = &mut loop_0;
    let mut can_define: bool = (*compiler).can_define;
    (*compiler).can_define = 0 as libc::c_int != 0;
    compileExpression(compiler);
    (*compiler).can_define = can_define;
    emitOpcode(compiler, OP_JUMP_IF_NOT);
    let mut whilepatch: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
    compileBlockBody(compiler, BLOCK_LOOP);
    emitLoopJump(compiler);
    patchJump(compiler, whilepatch);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*compiler).loop_0).patch_count {
        patchJump(compiler, (*(*compiler).loop_0).patches[i as usize]);
        i += 1;
        i;
    }
    (*compiler).loop_0 = loop_0.outer_loop;
    skipNewLines(compiler);
    consume(
        compiler,
        TK_END,
        b"Expected 'end' after statement end.\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn compileForStatement(mut compiler: *mut Compiler) {
    compilerEnterBlock(compiler);
    consume(
        compiler,
        TK_NAME,
        b"Expected an iterator name.\0" as *const u8 as *const libc::c_char,
    );
    let mut iter_name: *const libc::c_char = (*compiler).parser.previous.start;
    let mut iter_len: libc::c_int = (*compiler).parser.previous.length;
    let mut iter_line: libc::c_int = (*compiler).parser.previous.line;
    consume(
        compiler,
        TK_IN,
        b"Expected 'in' after iterator name.\0" as *const u8 as *const libc::c_char,
    );
    compilerAddVariable(
        compiler,
        b"@Sequence\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as uint32_t,
        iter_line,
    );
    let mut can_define: bool = (*compiler).can_define;
    (*compiler).can_define = 0 as libc::c_int != 0;
    compileExpression(compiler);
    (*compiler).can_define = can_define;
    compilerAddVariable(
        compiler,
        b"@iterator\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as uint32_t,
        iter_line,
    );
    emitOpcode(compiler, OP_PUSH_0);
    compilerAddVariable(compiler, iter_name, iter_len as uint32_t, iter_line);
    emitOpcode(compiler, OP_PUSH_NULL);
    emitOpcode(compiler, OP_ITER_TEST);
    let mut loop_0: Loop = Loop {
        start: 0,
        exit_jump: 0,
        patches: [0; 256],
        patch_count: 0,
        outer_loop: 0 as *mut sLoop,
        depth: 0,
    };
    loop_0
        .start = (*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.count
        as libc::c_int;
    loop_0.patch_count = 0 as libc::c_int;
    loop_0.outer_loop = (*compiler).loop_0;
    loop_0.depth = (*compiler).scope_depth;
    (*compiler).loop_0 = &mut loop_0;
    emitOpcode(compiler, OP_ITER);
    let mut forpatch: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
    compileBlockBody(compiler, BLOCK_LOOP);
    emitLoopJump(compiler);
    patchJump(compiler, forpatch);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*compiler).loop_0).patch_count {
        patchJump(compiler, (*(*compiler).loop_0).patches[i as usize]);
        i += 1;
        i;
    }
    (*compiler).loop_0 = loop_0.outer_loop;
    skipNewLines(compiler);
    consume(
        compiler,
        TK_END,
        b"Expected 'end' after statement end.\0" as *const u8 as *const libc::c_char,
    );
    compilerExitBlock(compiler);
}
unsafe extern "C" fn compileStatement(mut compiler: *mut Compiler) {
    let mut is_temporary: bool = 0 as libc::c_int != 0;
    let mut is_expression: bool = 0 as libc::c_int != 0;
    if match_0(compiler, TK_BREAK) {
        if ((*compiler).loop_0).is_null() {
            syntaxError(
                compiler,
                (*compiler).parser.previous,
                b"Cannot use 'break' outside a loop.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        consumeEndStatement(compiler);
        compilerPopLocals(compiler, (*(*compiler).loop_0).depth + 1 as libc::c_int);
        emitOpcode(compiler, OP_JUMP);
        let mut patch: libc::c_int = emitShort(compiler, 0xffff as libc::c_int);
        let fresh3 = (*(*compiler).loop_0).patch_count;
        (*(*compiler).loop_0).patch_count = (*(*compiler).loop_0).patch_count + 1;
        (*(*compiler).loop_0).patches[fresh3 as usize] = patch;
    } else if match_0(compiler, TK_CONTINUE) {
        if ((*compiler).loop_0).is_null() {
            syntaxError(
                compiler,
                (*compiler).parser.previous,
                b"Cannot use 'continue' outside a loop.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        consumeEndStatement(compiler);
        compilerPopLocals(compiler, (*(*compiler).loop_0).depth + 1 as libc::c_int);
        emitLoopJump(compiler);
    } else if match_0(compiler, TK_RETURN) {
        if (*compiler).scope_depth == DEPTH_GLOBAL as libc::c_int {
            syntaxError(
                compiler,
                (*compiler).parser.previous,
                b"Invalid 'return' outside a function.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if matchEndStatement(compiler) {
            if (*(*compiler).func).type_0 as libc::c_uint
                == FUNC_CONSTRUCTOR as libc::c_int as libc::c_uint
            {
                emitOpcode(compiler, OP_PUSH_SELF);
            } else {
                emitOpcode(compiler, OP_PUSH_NULL);
            }
            emitOpcode(compiler, OP_RETURN);
        } else {
            if (*(*compiler).func).type_0 as libc::c_uint
                == FUNC_CONSTRUCTOR as libc::c_int as libc::c_uint
            {
                syntaxError(
                    compiler,
                    (*compiler).parser.previous,
                    b"Cannor 'return' a value from constructor.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            let mut can_define: bool = (*compiler).can_define;
            (*compiler).can_define = 0 as libc::c_int != 0;
            compileExpression(compiler);
            (*compiler).can_define = can_define;
            if (*compiler).is_last_call {
                if !((*compiler).options).is_null() && !(*(*compiler).options).debug {
                    *((*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0).opcodes.data)
                        .offset(
                            ((*(*(*(*compiler).func).ptr).c2rust_unnamed.fn_0)
                                .opcodes
                                .count)
                                .wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) = OP_TAIL_CALL as libc::c_int as uint8_t;
                }
            }
            consumeEndStatement(compiler);
            emitOpcode(compiler, OP_RETURN);
        }
    } else if match_0(compiler, TK_IF) {
        compileIfStatement(compiler, 0 as libc::c_int != 0);
    } else if match_0(compiler, TK_WHILE) {
        compileWhileStatement(compiler);
    } else if match_0(compiler, TK_FOR) {
        compileForStatement(compiler);
    } else {
        (*compiler).new_local = 0 as libc::c_int != 0;
        compileExpression(compiler);
        consumeEndStatement(compiler);
        is_expression = 1 as libc::c_int != 0;
        if !(*compiler).new_local {
            is_temporary = 1 as libc::c_int != 0;
        }
        (*compiler).new_local = 0 as libc::c_int != 0;
    }
    if !((*compiler).options).is_null()
        && (*(*compiler).options).repl_mode as libc::c_int != 0
        && (*(*compiler).func).ptr == (*(*(*compiler).module).body).fn_0
        && is_expression as libc::c_int != 0
    {
        emitOpcode(compiler, OP_REPL_PRINT);
    }
    if is_temporary {
        emitOpcode(compiler, OP_POP);
    }
}
unsafe extern "C" fn compileTopLevelStatement(mut compiler: *mut Compiler) {
    if match_0(compiler, TK_CLASS) {
        compileClass(compiler);
    } else if match_0(compiler, TK_DEF) {
        compileFunction(compiler, FUNC_TOPLEVEL);
    } else if match_0(compiler, TK_IMPORT) {
        compileRegularImport(compiler);
    } else if match_0(compiler, TK_FROM) {
        compileFromImport(compiler);
    } else {
        compileStatement(compiler);
    };
}
pub unsafe extern "C" fn newCompilerOptions() -> CompileOptions {
    let mut options: CompileOptions = CompileOptions {
        debug: false,
        repl_mode: false,
    };
    options.debug = 0 as libc::c_int != 0;
    options.repl_mode = 0 as libc::c_int != 0;
    return options;
}
pub unsafe extern "C" fn compile(
    mut vm: *mut PKVM,
    mut module: *mut Module,
    mut source: *const libc::c_char,
    mut options: *const CompileOptions,
) -> PkResult {
    if strncmp(
        source,
        b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        source = source.offset(3 as libc::c_int as isize);
    }
    let mut _compiler: Compiler = Compiler {
        parser: Parser {
            vm: 0 as *mut PKVM,
            source: 0 as *const libc::c_char,
            file_path: 0 as *const libc::c_char,
            token_start: 0 as *const libc::c_char,
            current_char: 0 as *const libc::c_char,
            current_line: 0,
            previous: Token {
                type_0: TK_ERROR,
                start: 0 as *const libc::c_char,
                length: 0,
                line: 0,
                value: 0,
            },
            current: Token {
                type_0: TK_ERROR,
                start: 0 as *const libc::c_char,
                length: 0,
                line: 0,
                value: 0,
            },
            next: Token {
                type_0: TK_ERROR,
                start: 0 as *const libc::c_char,
                length: 0,
                line: 0,
                value: 0,
            },
            si_depth: 0,
            si_open_brace: [0; 8],
            si_quote: [0; 8],
            si_name_end: 0 as *const libc::c_char,
            si_name_quote: 0,
            forwards: [ForwardName {
                instruction: 0,
                func: 0 as *mut Fn_0,
                tkname: Token {
                    type_0: TK_ERROR,
                    start: 0 as *const libc::c_char,
                    length: 0,
                    line: 0,
                    value: 0,
                },
            }; 256],
            forwards_count: 0,
            optional_call_paran: false,
            repl_mode: false,
            parsing_class: false,
            need_more_lines: false,
            has_syntax_error: false,
            has_errors: false,
        },
        next_compiler: 0 as *mut Compiler,
        options: 0 as *const CompileOptions,
        module: 0 as *mut Module,
        loop_0: 0 as *mut Loop,
        func: 0 as *mut Func,
        scope_depth: 0,
        new_local: false,
        l_value: false,
        can_define: false,
        is_last_call: false,
        bifn_list_join: 0,
    };
    let mut compiler: *mut Compiler = &mut _compiler;
    compilerInit(compiler, vm, source, module, options);
    (*compiler).next_compiler = (*vm).compiler;
    (*vm).compiler = compiler;
    if ((*module).body).is_null() {
        moduleAddMain(vm, module);
    }
    pkByteBufferClear(&mut (*(*(*(*module).body).fn_0).c2rust_unnamed.fn_0).opcodes, vm);
    let mut constants_count: uint32_t = (*module).constants.count;
    let mut globals_count: uint32_t = (*module).globals.count;
    let mut curr_fn: Func = Func {
        type_0: FUNC_MAIN,
        depth: 0,
        locals: [Local {
            name: 0 as *const libc::c_char,
            length: 0,
            depth: 0,
            is_upvalue: false,
            line: 0,
        }; 256],
        local_count: 0,
        upvalues: [UpvalueInfo {
            is_immediate: false,
            index: 0,
        }; 256],
        stack_size: 0,
        ptr: 0 as *mut Function,
        outer_func: 0 as *mut sFunc,
    };
    compilerPushFunc(compiler, &mut curr_fn, (*(*module).body).fn_0, FUNC_MAIN);
    lexToken(compiler);
    lexToken(compiler);
    skipNewLines(compiler);
    while !match_0(compiler, TK_EOF) && !(*compiler).parser.has_syntax_error {
        compileTopLevelStatement(compiler);
        skipNewLines(compiler);
    }
    emitFunctionEnd(compiler);
    if !(*compiler).parser.has_syntax_error {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*compiler).parser.forwards_count {
            let mut forward: *mut ForwardName = &mut *((*compiler).parser.forwards)
                .as_mut_ptr()
                .offset(i as isize) as *mut ForwardName;
            let mut name: *const libc::c_char = (*forward).tkname.start;
            let mut length: libc::c_int = (*forward).tkname.length;
            let mut index: libc::c_int = moduleGetGlobalIndex(
                (*compiler).module,
                name,
                length as uint32_t,
            );
            if index != -(1 as libc::c_int) {
                patchForward(compiler, (*forward).func, (*forward).instruction, index);
            } else {
                (*compiler).parser.need_more_lines = 0 as libc::c_int != 0;
                resolveError(
                    compiler,
                    (*forward).tkname,
                    b"Name '%.*s' is not defined.\0" as *const u8 as *const libc::c_char,
                    length,
                    name,
                );
            }
            i += 1;
            i;
        }
    }
    (*vm).compiler = (*compiler).next_compiler;
    if (*compiler).parser.has_errors {
        (*module).constants.count = constants_count;
        (*module).global_names.count = globals_count;
        (*module).globals.count = (*module).global_names.count;
    }
    if (*compiler).parser.has_errors {
        if (*compiler).parser.repl_mode as libc::c_int != 0
            && (*compiler).parser.need_more_lines as libc::c_int != 0
        {
            return PK_RESULT_UNEXPECTED_EOF;
        }
        return PK_RESULT_COMPILE_ERROR;
    }
    return PK_RESULT_SUCCESS;
}
pub unsafe extern "C" fn compilerMarkObjects(
    mut vm: *mut PKVM,
    mut compiler: *mut Compiler,
) {
    markObject(vm, &mut (*(*compiler).module)._super);
    markValue(vm, (*compiler).parser.current.value);
    markValue(vm, (*compiler).parser.previous.value);
    markValue(vm, (*compiler).parser.next.value);
    if !((*compiler).next_compiler).is_null() {
        compilerMarkObjects(vm, (*compiler).next_compiler);
    }
}
