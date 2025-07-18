use ::libc;
extern "C" {
    pub type Compiler;
    
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pkByteBufferInit(self_0: *mut pkByteBuffer);
    fn pkByteBufferClear(self_0: *mut pkByteBuffer, vm: *mut PKVM);
    fn pkByteBufferReserve(self_0: *mut pkByteBuffer, vm: *mut PKVM, size: size_t);
    fn pkByteBufferFill(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        data: uint8_t,
        count: libc::c_int,
    );
    fn pkByteBufferWrite(self_0: *mut pkByteBuffer, vm: *mut PKVM, data: uint8_t);
    fn pkByteBufferAddString(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        str: *const libc::c_char,
        length: uint32_t,
    );
    fn moduleGetStringAt(module: *mut Module, index: libc::c_int) -> *mut String_0;
    fn toRepr(vm: *mut PKVM, value: Var) -> *mut String_0;
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub const OP_LOOP: Opcode = 51;
pub const OP_AND: Opcode = 55;
pub const OP_OR: Opcode = 54;
pub const OP_JUMP_IF_NOT: Opcode = 53;
pub const OP_JUMP_IF: Opcode = 52;
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
pub const OP_STORE_LOCAL_0: Opcode = 22;
pub type Opcode = libc::c_uint;
pub const OP_STORE_LOCAL_N: Opcode = 31;
pub const OP_STORE_LOCAL_8: Opcode = 30;
pub const OP_STORE_LOCAL_7: Opcode = 29;
pub const OP_STORE_LOCAL_6: Opcode = 28;
pub const OP_STORE_LOCAL_5: Opcode = 27;
pub const OP_STORE_LOCAL_4: Opcode = 26;
pub const OP_STORE_LOCAL_3: Opcode = 25;
pub const OP_STORE_LOCAL_2: Opcode = 24;
pub const OP_STORE_LOCAL_1: Opcode = 23;
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
unsafe extern "C" fn _printRed(mut vm: *mut PKVM, mut msg: *const libc::c_char) {
    if (*vm).config.use_ansi_escape {
        ((*vm).config.stderr_write)
            .unwrap()(
            vm,
            b"\x1B[38;2;220;100;100m\0" as *const u8 as *const libc::c_char,
        );
        ((*vm).config.stderr_write).unwrap()(vm, msg);
        ((*vm).config.stderr_write)
            .unwrap()(vm, b"\x1B[0m\0" as *const u8 as *const libc::c_char);
    } else {
        ((*vm).config.stderr_write).unwrap()(vm, msg);
    };
}
pub unsafe extern "C" fn reportCompileTimeError(
    mut vm: *mut PKVM,
    mut path: *const libc::c_char,
    mut line: libc::c_int,
    mut source: *const libc::c_char,
    mut at: *const libc::c_char,
    mut length: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut writefn: pkWriteFn = (*vm).config.stderr_write;
    if writefn.is_none() {
        return;
    }
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    pkByteBufferReserve(&mut buff, vm, 512 as libc::c_int as size_t);
    buff.count = 0 as libc::c_int as uint32_t;
    writefn.unwrap()(vm, path);
    writefn.unwrap()(vm, b":\0" as *const u8 as *const libc::c_char);
    snprintf(
        buff.data as *mut libc::c_char,
        buff.capacity as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        line,
    );
    writefn.unwrap()(vm, buff.data as *mut libc::c_char);
    _printRed(vm, b" error: \0" as *const u8 as *const libc::c_char);
    buff.count = 0 as libc::c_int as uint32_t;
    let mut args_copy: ::std::ffi::VaListImpl;
    args_copy = args.clone();
    let mut size: libc::c_int = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        fmt,
        args_copy.as_va_list(),
    ) + 1 as libc::c_int;
    pkByteBufferReserve(&mut buff, vm, size as size_t);
    vsnprintf(
        buff.data as *mut libc::c_char,
        size as libc::c_ulong,
        fmt,
        args.as_va_list(),
    );
    writefn.unwrap()(vm, buff.data as *mut libc::c_char);
    writefn.unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
    let mut start: libc::c_int = line - 2 as libc::c_int;
    if start < 1 as libc::c_int {
        start = 1 as libc::c_int;
    }
    let mut end: libc::c_int = start + 5 as libc::c_int;
    let mut line_number_width: libc::c_int = 5 as libc::c_int;
    let mut curr_line: libc::c_int = line;
    let mut c: *const libc::c_char = at;
    if c != source {
        loop {
            c = c.offset(-1);
            c;
            if *c as libc::c_int == '\n' as i32 {
                curr_line -= 1;
                curr_line;
            }
            if c == source {
                break;
            }
            if !(curr_line >= start) {
                break;
            }
        }
    }
    curr_line = start;
    if c != source {
        c = c.offset(1);
        c;
    }
    while curr_line < end {
        buff.count = 0 as libc::c_int as uint32_t;
        snprintf(
            buff.data as *mut libc::c_char,
            buff.capacity as libc::c_ulong,
            b"%*d\0" as *const u8 as *const libc::c_char,
            line_number_width,
            curr_line,
        );
        writefn.unwrap()(vm, buff.data as *mut libc::c_char);
        writefn.unwrap()(vm, b" | \0" as *const u8 as *const libc::c_char);
        if curr_line != line {
            let mut line_start: *const libc::c_char = c;
            while *c as libc::c_int != '\0' as i32 && *c as libc::c_int != '\n' as i32 {
                c = c.offset(1);
                c;
            }
            buff.count = 0 as libc::c_int as uint32_t;
            pkByteBufferAddString(
                &mut buff,
                vm,
                line_start,
                c.offset_from(line_start) as libc::c_long as uint32_t,
            );
            pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
            writefn.unwrap()(vm, buff.data as *mut libc::c_char);
            writefn.unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            let mut line_start_0: *const libc::c_char = c;
            buff.count = 0 as libc::c_int as uint32_t;
            pkByteBufferAddString(
                &mut buff,
                vm,
                line_start_0,
                at.offset_from(line_start_0) as libc::c_long as uint32_t,
            );
            pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
            writefn.unwrap()(vm, buff.data as *mut libc::c_char);
            if *at as libc::c_int != '\n' as i32 {
                buff.count = 0 as libc::c_int as uint32_t;
                pkByteBufferAddString(&mut buff, vm, at, length as uint32_t);
                pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
                _printRed(vm, buff.data as *mut libc::c_char);
                let mut tail_start: *const libc::c_char = at;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < length {
                    if *tail_start as libc::c_int == '\0' as i32 {
                        break;
                    }
                    tail_start = tail_start.offset(1);
                    tail_start;
                    i += 1;
                    i;
                }
                c = tail_start;
                while *c as libc::c_int != '\0' as i32
                    && *c as libc::c_int != '\n' as i32
                {
                    c = c.offset(1);
                    c;
                }
                if c != tail_start {
                    buff.count = 0 as libc::c_int as uint32_t;
                    pkByteBufferAddString(
                        &mut buff,
                        vm,
                        tail_start,
                        c.offset_from(tail_start) as libc::c_long as uint32_t,
                    );
                    pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
                    writefn.unwrap()(vm, buff.data as *mut libc::c_char);
                }
            } else {
                c = at;
            }
            writefn.unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            buff.count = 0 as libc::c_int as uint32_t;
            pkByteBufferFill(&mut buff, vm, ' ' as i32 as uint8_t, line_number_width);
            pkByteBufferAddString(
                &mut buff,
                vm,
                b" | \0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as uint32_t,
            );
            let mut c2: *const libc::c_char = line_start_0;
            while c2 < at {
                let mut white_space: libc::c_char = (if *c2 as libc::c_int == '\t' as i32
                {
                    '\t' as i32
                } else {
                    ' ' as i32
                }) as libc::c_char;
                pkByteBufferWrite(&mut buff, vm, white_space as uint8_t);
                c2 = c2.offset(1);
                c2;
            }
            pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
            writefn.unwrap()(vm, buff.data as *mut libc::c_char);
            buff.count = 0 as libc::c_int as uint32_t;
            pkByteBufferFill(
                &mut buff,
                vm,
                '~' as i32 as uint8_t,
                (if length != 0 { length } else { 1 as libc::c_int }) as uint32_t
                    as libc::c_int,
            );
            pkByteBufferWrite(&mut buff, vm, '\0' as i32 as uint8_t);
            _printRed(vm, buff.data as *mut libc::c_char);
            writefn.unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
        }
        if *c as libc::c_int == '\0' as i32 {
            break;
        }
        curr_line += 1;
        curr_line;
        c = c.offset(1);
        c;
    }
    pkByteBufferClear(&mut buff, vm);
}
unsafe extern "C" fn _reportStackFrame(mut vm: *mut PKVM, mut frame: *mut CallFrame) {
    let mut writefn: pkWriteFn = (*vm).config.stderr_write;
    let mut fn_0: *const Function = (*(*frame).closure).fn_0;
    let mut instruction_index: libc::c_int = ((*frame).ip)
        .offset_from((*(*fn_0).c2rust_unnamed.fn_0).opcodes.data) as libc::c_long
        as libc::c_int - 1 as libc::c_int;
    if instruction_index == -(1 as libc::c_int) {
        instruction_index += 1;
        instruction_index;
    }
    let mut line: libc::c_int = *((*(*fn_0).c2rust_unnamed.fn_0).oplines.data)
        .offset(instruction_index as isize) as libc::c_int;
    if ((*(*fn_0).owner).path).is_null() {
        writefn.unwrap()(vm, b"  [at:\0" as *const u8 as *const libc::c_char);
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(buff.as_mut_ptr(), b"%2d\0" as *const u8 as *const libc::c_char, line);
        writefn.unwrap()(vm, buff.as_mut_ptr());
        writefn.unwrap()(vm, b"] \0" as *const u8 as *const libc::c_char);
        writefn.unwrap()(vm, (*fn_0).name);
        writefn.unwrap()(vm, b"()\n\0" as *const u8 as *const libc::c_char);
    } else {
        writefn.unwrap()(vm, b"  \0" as *const u8 as *const libc::c_char);
        writefn.unwrap()(vm, (*fn_0).name);
        writefn.unwrap()(vm, b"() [\0" as *const u8 as *const libc::c_char);
        writefn.unwrap()(vm, ((*(*(*fn_0).owner).path).data).as_mut_ptr());
        writefn.unwrap()(vm, b":\0" as *const u8 as *const libc::c_char);
        let mut buff_0: [libc::c_char; 12] = [0; 12];
        sprintf(buff_0.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, line);
        writefn.unwrap()(vm, buff_0.as_mut_ptr());
        writefn.unwrap()(vm, b"]\n\0" as *const u8 as *const libc::c_char);
    };
}
pub unsafe extern "C" fn reportRuntimeError(mut vm: *mut PKVM, mut fiber: *mut Fiber) {
    let mut writefn: pkWriteFn = (*vm).config.stderr_write;
    if writefn.is_none() {
        return;
    }
    _printRed(vm, b"Error: \0" as *const u8 as *const libc::c_char);
    writefn.unwrap()(vm, ((*(*fiber).error).data).as_mut_ptr());
    writefn.unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
    let mut max_dump_frames: libc::c_int = 10 as libc::c_int;
    if (*fiber).frame_count > 2 as libc::c_int * max_dump_frames {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < max_dump_frames {
            let mut frame: *mut CallFrame = &mut *((*fiber).frames)
                .offset(((*fiber).frame_count - 1 as libc::c_int - i) as isize)
                as *mut CallFrame;
            _reportStackFrame(vm, frame);
            i += 1;
            i;
        }
        let mut skipped_count: libc::c_int = (*fiber).frame_count
            - max_dump_frames * 2 as libc::c_int;
        writefn.unwrap()(vm, b"  ...  skipping \0" as *const u8 as *const libc::c_char);
        let mut buff: [libc::c_char; 12] = [0; 12];
        sprintf(
            buff.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            skipped_count,
        );
        writefn.unwrap()(vm, buff.as_mut_ptr());
        writefn.unwrap()(vm, b" stack frames\n\0" as *const u8 as *const libc::c_char);
        let mut i_0: libc::c_int = max_dump_frames;
        while i_0 >= 0 as libc::c_int {
            let mut frame_0: *mut CallFrame = &mut *((*fiber).frames)
                .offset(i_0 as isize) as *mut CallFrame;
            _reportStackFrame(vm, frame_0);
            i_0 -= 1;
            i_0;
        }
    } else {
        let mut i_1: libc::c_int = (*fiber).frame_count - 1 as libc::c_int;
        while i_1 >= 0 as libc::c_int {
            let mut frame_1: *mut CallFrame = &mut *((*fiber).frames)
                .offset(i_1 as isize) as *mut CallFrame;
            _reportStackFrame(vm, frame_1);
            i_1 -= 1;
            i_1;
        }
    };
}
static mut op_names: [*const libc::c_char; 89] = [
    b"PUSH_CONSTANT\0" as *const u8 as *const libc::c_char,
    b"PUSH_NULL\0" as *const u8 as *const libc::c_char,
    b"PUSH_0\0" as *const u8 as *const libc::c_char,
    b"PUSH_TRUE\0" as *const u8 as *const libc::c_char,
    b"PUSH_FALSE\0" as *const u8 as *const libc::c_char,
    b"SWAP\0" as *const u8 as *const libc::c_char,
    b"DUP\0" as *const u8 as *const libc::c_char,
    b"PUSH_LIST\0" as *const u8 as *const libc::c_char,
    b"PUSH_MAP\0" as *const u8 as *const libc::c_char,
    b"PUSH_SELF\0" as *const u8 as *const libc::c_char,
    b"LIST_APPEND\0" as *const u8 as *const libc::c_char,
    b"MAP_INSERT\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_0\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_1\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_2\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_3\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_4\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_5\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_6\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_7\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_8\0" as *const u8 as *const libc::c_char,
    b"PUSH_LOCAL_N\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_0\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_1\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_2\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_3\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_4\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_5\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_6\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_7\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_8\0" as *const u8 as *const libc::c_char,
    b"STORE_LOCAL_N\0" as *const u8 as *const libc::c_char,
    b"PUSH_GLOBAL\0" as *const u8 as *const libc::c_char,
    b"STORE_GLOBAL\0" as *const u8 as *const libc::c_char,
    b"PUSH_BUILTIN_FN\0" as *const u8 as *const libc::c_char,
    b"PUSH_BUILTIN_TY\0" as *const u8 as *const libc::c_char,
    b"PUSH_UPVALUE\0" as *const u8 as *const libc::c_char,
    b"STORE_UPVALUE\0" as *const u8 as *const libc::c_char,
    b"PUSH_CLOSURE\0" as *const u8 as *const libc::c_char,
    b"CREATE_CLASS\0" as *const u8 as *const libc::c_char,
    b"BIND_METHOD\0" as *const u8 as *const libc::c_char,
    b"CLOSE_UPVALUE\0" as *const u8 as *const libc::c_char,
    b"POP\0" as *const u8 as *const libc::c_char,
    b"IMPORT\0" as *const u8 as *const libc::c_char,
    b"SUPER_CALL\0" as *const u8 as *const libc::c_char,
    b"METHOD_CALL\0" as *const u8 as *const libc::c_char,
    b"CALL\0" as *const u8 as *const libc::c_char,
    b"TAIL_CALL\0" as *const u8 as *const libc::c_char,
    b"ITER_TEST\0" as *const u8 as *const libc::c_char,
    b"ITER\0" as *const u8 as *const libc::c_char,
    b"JUMP\0" as *const u8 as *const libc::c_char,
    b"LOOP\0" as *const u8 as *const libc::c_char,
    b"JUMP_IF\0" as *const u8 as *const libc::c_char,
    b"JUMP_IF_NOT\0" as *const u8 as *const libc::c_char,
    b"OR\0" as *const u8 as *const libc::c_char,
    b"AND\0" as *const u8 as *const libc::c_char,
    b"RETURN\0" as *const u8 as *const libc::c_char,
    b"GET_ATTRIB\0" as *const u8 as *const libc::c_char,
    b"GET_ATTRIB_KEEP\0" as *const u8 as *const libc::c_char,
    b"SET_ATTRIB\0" as *const u8 as *const libc::c_char,
    b"GET_SUBSCRIPT\0" as *const u8 as *const libc::c_char,
    b"GET_SUBSCRIPT_KEEP\0" as *const u8 as *const libc::c_char,
    b"SET_SUBSCRIPT\0" as *const u8 as *const libc::c_char,
    b"POSITIVE\0" as *const u8 as *const libc::c_char,
    b"NEGATIVE\0" as *const u8 as *const libc::c_char,
    b"NOT\0" as *const u8 as *const libc::c_char,
    b"BIT_NOT\0" as *const u8 as *const libc::c_char,
    b"ADD\0" as *const u8 as *const libc::c_char,
    b"SUBTRACT\0" as *const u8 as *const libc::c_char,
    b"MULTIPLY\0" as *const u8 as *const libc::c_char,
    b"DIVIDE\0" as *const u8 as *const libc::c_char,
    b"EXPONENT\0" as *const u8 as *const libc::c_char,
    b"MOD\0" as *const u8 as *const libc::c_char,
    b"BIT_AND\0" as *const u8 as *const libc::c_char,
    b"BIT_OR\0" as *const u8 as *const libc::c_char,
    b"BIT_XOR\0" as *const u8 as *const libc::c_char,
    b"BIT_LSHIFT\0" as *const u8 as *const libc::c_char,
    b"BIT_RSHIFT\0" as *const u8 as *const libc::c_char,
    b"EQEQ\0" as *const u8 as *const libc::c_char,
    b"NOTEQ\0" as *const u8 as *const libc::c_char,
    b"LT\0" as *const u8 as *const libc::c_char,
    b"LTEQ\0" as *const u8 as *const libc::c_char,
    b"GT\0" as *const u8 as *const libc::c_char,
    b"GTEQ\0" as *const u8 as *const libc::c_char,
    b"RANGE\0" as *const u8 as *const libc::c_char,
    b"IN\0" as *const u8 as *const libc::c_char,
    b"IS\0" as *const u8 as *const libc::c_char,
    b"REPL_PRINT\0" as *const u8 as *const libc::c_char,
    b"END\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn dumpValue(mut vm: *mut PKVM, mut value: Var) {
    if ((*vm).config.stdout_write).is_none() {
        return;
    }
    let mut repr: *mut String_0 = toRepr(vm, value);
    ((*vm).config.stdout_write).unwrap()(vm, ((*repr).data).as_mut_ptr());
}
pub unsafe extern "C" fn dumpFunctionCode(mut vm: *mut PKVM, mut func: *mut Function) {
    if ((*vm).config.stdout_write).is_none() {
        return;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut opcodes: *mut uint8_t = (*(*func).c2rust_unnamed.fn_0).opcodes.data;
    let mut lines: *mut uint32_t = (*(*func).c2rust_unnamed.fn_0).oplines.data;
    let mut line: uint32_t = 1 as libc::c_int as uint32_t;
    let mut last_line: uint32_t = 0 as libc::c_int as uint32_t;
    let mut path: *const libc::c_char = if !((*(*func).owner).path).is_null() {
        ((*(*(*func).owner).path).data).as_mut_ptr()
    } else {
        ((*(*(*func).owner).name).data).as_mut_ptr()
    };
    ((*vm).config.stdout_write)
        .unwrap()(
        vm,
        b"Instruction Dump of function \0" as *const u8 as *const libc::c_char,
    );
    ((*vm).config.stdout_write).unwrap()(vm, (*func).name);
    ((*vm).config.stdout_write).unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
    ((*vm).config.stdout_write).unwrap()(vm, path);
    ((*vm).config.stdout_write)
        .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
    while i < (*(*func).c2rust_unnamed.fn_0).opcodes.count {
        line = *lines.offset(i as isize);
        if line != last_line {
            last_line = line;
            ((*vm).config.stdout_write)
                .unwrap()(vm, b"  \0" as *const u8 as *const libc::c_char);
            let mut sbuff: [libc::c_char; 12] = [0; 12];
            let mut length: libc::c_int = 0;
            if 5 as libc::c_int - 1 as libc::c_int > 0 as libc::c_int {
                length = sprintf(
                    sbuff.as_mut_ptr(),
                    b"%*d\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int - 1 as libc::c_int,
                    line,
                );
            } else {
                length = sprintf(
                    sbuff.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    line,
                );
            }
            sbuff[length as usize] = '\0' as i32 as libc::c_char;
            ((*vm).config.stdout_write).unwrap()(vm, sbuff.as_mut_ptr());
            ((*vm).config.stdout_write)
                .unwrap()(vm, b":\0" as *const u8 as *const libc::c_char);
        } else {
            ((*vm).config.stdout_write)
                .unwrap()(vm, b"       \0" as *const u8 as *const libc::c_char);
        }
        ((*vm).config.stdout_write)
            .unwrap()(vm, b"  \0" as *const u8 as *const libc::c_char);
        let mut sbuff_0: [libc::c_char; 12] = [0; 12];
        let mut length_0: libc::c_int = 0;
        if 5 as libc::c_int - 1 as libc::c_int > 0 as libc::c_int {
            length_0 = sprintf(
                sbuff_0.as_mut_ptr(),
                b"%*d\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int - 1 as libc::c_int,
                i,
            );
        } else {
            length_0 = sprintf(
                sbuff_0.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                i,
            );
        }
        sbuff_0[length_0 as usize] = '\0' as i32 as libc::c_char;
        ((*vm).config.stdout_write).unwrap()(vm, sbuff_0.as_mut_ptr());
        ((*vm).config.stdout_write)
            .unwrap()(vm, b"  \0" as *const u8 as *const libc::c_char);
        let mut op_name: *const libc::c_char = op_names[*opcodes.offset(i as isize)
            as usize];
        let mut op_length: uint32_t = strlen(op_name) as uint32_t;
        ((*vm).config.stdout_write).unwrap()(vm, op_name);
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while j < (16 as libc::c_int as libc::c_uint).wrapping_sub(op_length) {
            ((*vm).config.stdout_write)
                .unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
            j = j.wrapping_add(1);
            j;
        }
        let fresh0 = i;
        i = i.wrapping_add(1);
        let mut op: Opcode = *((*(*func).c2rust_unnamed.fn_0).opcodes.data)
            .offset(fresh0 as isize) as Opcode;
        match op as libc::c_uint {
            0 => {
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut index: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut value: Var = *((*(*func).owner).constants.data)
                    .offset(index as isize);
                let mut sbuff_1: [libc::c_char; 12] = [0; 12];
                let mut length_1: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_1 = sprintf(
                        sbuff_1.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index,
                    );
                } else {
                    length_1 = sprintf(
                        sbuff_1.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index,
                    );
                }
                sbuff_1[length_1 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_1.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
                dumpValue(vm, value);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            1 | 2 | 3 | 4 | 5 | 6 => {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            7 => {
                let mut sbuff_2: [libc::c_char; 12] = [0; 12];
                let mut length_2: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    i = (i as libc::c_uint)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    length_2 = sprintf(
                        sbuff_2.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        (*opcodes
                            .offset(
                                i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int) << 8 as libc::c_int
                            | *opcodes
                                .offset(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int,
                    );
                } else {
                    i = (i as libc::c_uint)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as uint32_t
                        as uint32_t;
                    length_2 = sprintf(
                        sbuff_2.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        (*opcodes
                            .offset(
                                i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int) << 8 as libc::c_int
                            | *opcodes
                                .offset(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int,
                    );
                }
                sbuff_2[length_2 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_2.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            8 | 9 | 10 | 11 => {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => {
                let mut arg: libc::c_int = 0;
                if op as libc::c_uint == OP_PUSH_LOCAL_N as libc::c_int as libc::c_uint {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    arg = *opcodes.offset(fresh1 as isize) as libc::c_int;
                    let mut sbuff_3: [libc::c_char; 12] = [0; 12];
                    let mut length_3: libc::c_int = 0;
                    if 5 as libc::c_int > 0 as libc::c_int {
                        length_3 = sprintf(
                            sbuff_3.as_mut_ptr(),
                            b"%*d\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                            arg,
                        );
                    } else {
                        length_3 = sprintf(
                            sbuff_3.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            arg,
                        );
                    }
                    sbuff_3[length_3 as usize] = '\0' as i32 as libc::c_char;
                    ((*vm).config.stdout_write).unwrap()(vm, sbuff_3.as_mut_ptr());
                } else {
                    arg = (op as libc::c_uint)
                        .wrapping_sub(OP_PUSH_LOCAL_0 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    let mut j_0: libc::c_int = 0 as libc::c_int;
                    while j_0 < 5 as libc::c_int {
                        ((*vm).config.stdout_write)
                            .unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
                        j_0 += 1;
                        j_0;
                    }
                }
                if arg < (*func).arity {
                    ((*vm).config.stdout_write)
                        .unwrap()(vm, b" (param:\0" as *const u8 as *const libc::c_char);
                    let mut sbuff_4: [libc::c_char; 12] = [0; 12];
                    let mut length_4: libc::c_int = 0;
                    if 1 as libc::c_int > 0 as libc::c_int {
                        length_4 = sprintf(
                            sbuff_4.as_mut_ptr(),
                            b"%*d\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                            arg,
                        );
                    } else {
                        length_4 = sprintf(
                            sbuff_4.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            arg,
                        );
                    }
                    sbuff_4[length_4 as usize] = '\0' as i32 as libc::c_char;
                    ((*vm).config.stdout_write).unwrap()(vm, sbuff_4.as_mut_ptr());
                    ((*vm).config.stdout_write)
                        .unwrap()(vm, b")\n\0" as *const u8 as *const libc::c_char);
                } else {
                    ((*vm).config.stdout_write)
                        .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
                }
            }
            22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => {
                let mut arg_0: libc::c_int = 0;
                if op as libc::c_uint == OP_STORE_LOCAL_N as libc::c_int as libc::c_uint
                {
                    let fresh2 = i;
                    i = i.wrapping_add(1);
                    arg_0 = *opcodes.offset(fresh2 as isize) as libc::c_int;
                    let mut sbuff_5: [libc::c_char; 12] = [0; 12];
                    let mut length_5: libc::c_int = 0;
                    if 5 as libc::c_int > 0 as libc::c_int {
                        length_5 = sprintf(
                            sbuff_5.as_mut_ptr(),
                            b"%*d\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                            arg_0,
                        );
                    } else {
                        length_5 = sprintf(
                            sbuff_5.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            arg_0,
                        );
                    }
                    sbuff_5[length_5 as usize] = '\0' as i32 as libc::c_char;
                    ((*vm).config.stdout_write).unwrap()(vm, sbuff_5.as_mut_ptr());
                } else {
                    arg_0 = (op as libc::c_uint)
                        .wrapping_sub(OP_STORE_LOCAL_0 as libc::c_int as libc::c_uint)
                        as libc::c_int;
                    let mut j_1: libc::c_int = 0 as libc::c_int;
                    while j_1 < 5 as libc::c_int {
                        ((*vm).config.stdout_write)
                            .unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
                        j_1 += 1;
                        j_1;
                    }
                }
                if arg_0 < (*func).arity {
                    ((*vm).config.stdout_write)
                        .unwrap()(vm, b" (param:\0" as *const u8 as *const libc::c_char);
                    let mut sbuff_6: [libc::c_char; 12] = [0; 12];
                    let mut length_6: libc::c_int = 0;
                    if 1 as libc::c_int > 0 as libc::c_int {
                        length_6 = sprintf(
                            sbuff_6.as_mut_ptr(),
                            b"%*d\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                            arg_0,
                        );
                    } else {
                        length_6 = sprintf(
                            sbuff_6.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            arg_0,
                        );
                    }
                    sbuff_6[length_6 as usize] = '\0' as i32 as libc::c_char;
                    ((*vm).config.stdout_write).unwrap()(vm, sbuff_6.as_mut_ptr());
                    ((*vm).config.stdout_write)
                        .unwrap()(vm, b")\n\0" as *const u8 as *const libc::c_char);
                } else {
                    ((*vm).config.stdout_write)
                        .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
                }
            }
            32 | 33 => {
                let fresh3 = i;
                i = i.wrapping_add(1);
                let mut index_0: libc::c_int = *opcodes.offset(fresh3 as isize)
                    as libc::c_int;
                let mut name_index: libc::c_int = *((*(*func).owner).global_names.data)
                    .offset(index_0 as isize) as libc::c_int;
                let mut name: Var = *((*(*func).owner).constants.data)
                    .offset(name_index as isize);
                let mut sbuff_7: [libc::c_char; 12] = [0; 12];
                let mut length_7: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_7 = sprintf(
                        sbuff_7.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_0,
                    );
                } else {
                    length_7 = sprintf(
                        sbuff_7.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_0,
                    );
                }
                sbuff_7[length_7 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_7.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" '\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write)
                    .unwrap()(
                    vm,
                    ((*((name & 0xffffffffffff as libc::c_long as uint64_t)
                        as *mut Object as *mut String_0))
                        .data)
                        .as_mut_ptr(),
                );
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"'\n\0" as *const u8 as *const libc::c_char);
            }
            34 => {
                let fresh4 = i;
                i = i.wrapping_add(1);
                let mut index_1: libc::c_int = *opcodes.offset(fresh4 as isize)
                    as libc::c_int;
                let mut name_0: *const libc::c_char = (*(*(*vm)
                    .builtins_funcs[index_1 as usize])
                    .fn_0)
                    .name;
                let mut sbuff_8: [libc::c_char; 12] = [0; 12];
                let mut length_8: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_8 = sprintf(
                        sbuff_8.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_1,
                    );
                } else {
                    length_8 = sprintf(
                        sbuff_8.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_1,
                    );
                }
                sbuff_8[length_8 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_8.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" [Fn:\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write).unwrap()(vm, name_0);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"]\n\0" as *const u8 as *const libc::c_char);
            }
            35 => {
                let fresh5 = i;
                i = i.wrapping_add(1);
                let mut index_2: libc::c_int = *opcodes.offset(fresh5 as isize)
                    as libc::c_int;
                let mut name_1: *const libc::c_char = ((*(*(*vm)
                    .builtin_classes[index_2 as usize])
                    .name)
                    .data)
                    .as_mut_ptr();
                let mut sbuff_9: [libc::c_char; 12] = [0; 12];
                let mut length_9: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_9 = sprintf(
                        sbuff_9.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_2,
                    );
                } else {
                    length_9 = sprintf(
                        sbuff_9.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_2,
                    );
                }
                sbuff_9[length_9 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_9.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" [Class:\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write).unwrap()(vm, name_1);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"]\n\0" as *const u8 as *const libc::c_char);
            }
            36 | 37 => {
                let fresh6 = i;
                i = i.wrapping_add(1);
                let mut index_3: libc::c_int = *opcodes.offset(fresh6 as isize)
                    as libc::c_int;
                let mut sbuff_10: [libc::c_char; 12] = [0; 12];
                let mut length_10: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_10 = sprintf(
                        sbuff_10.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_3,
                    );
                } else {
                    length_10 = sprintf(
                        sbuff_10.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_3,
                    );
                }
                sbuff_10[length_10 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_10.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            38 => {
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut index_4: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut value_0: Var = *((*(*func).owner).constants.data)
                    .offset(index_4 as isize);
                let mut sbuff_11: [libc::c_char; 12] = [0; 12];
                let mut length_11: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_11 = sprintf(
                        sbuff_11.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_4,
                    );
                } else {
                    length_11 = sprintf(
                        sbuff_11.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_4,
                    );
                }
                sbuff_11[length_11 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_11.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
                dumpValue(vm, value_0);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            39 => {
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut index_5: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut value_1: Var = *((*(*func).owner).constants.data)
                    .offset(index_5 as isize);
                let mut sbuff_12: [libc::c_char; 12] = [0; 12];
                let mut length_12: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_12 = sprintf(
                        sbuff_12.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_5,
                    );
                } else {
                    length_12 = sprintf(
                        sbuff_12.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_5,
                    );
                }
                sbuff_12[length_12 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_12.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" \0" as *const u8 as *const libc::c_char);
                dumpValue(vm, value_1);
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            40 | 41 | 42 => {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            43 => {
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut index_6: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut name_2: *mut String_0 = moduleGetStringAt(
                    (*func).owner,
                    index_6,
                );
                let mut sbuff_13: [libc::c_char; 12] = [0; 12];
                let mut length_13: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_13 = sprintf(
                        sbuff_13.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_6,
                    );
                } else {
                    length_13 = sprintf(
                        sbuff_13.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_6,
                    );
                }
                sbuff_13[length_13 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_13.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" '\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write).unwrap()(vm, ((*name_2).data).as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"'\n\0" as *const u8 as *const libc::c_char);
            }
            44 | 45 => {
                let fresh7 = i;
                i = i.wrapping_add(1);
                let mut argc: libc::c_int = *opcodes.offset(fresh7 as isize)
                    as libc::c_int;
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut index_7: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut name_3: *mut String_0 = moduleGetStringAt(
                    (*func).owner,
                    index_7,
                );
                let mut sbuff_14: [libc::c_char; 12] = [0; 12];
                let mut length_14: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_14 = sprintf(
                        sbuff_14.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        argc,
                    );
                } else {
                    length_14 = sprintf(
                        sbuff_14.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        argc,
                    );
                }
                sbuff_14[length_14 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_14.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" (argc) \0" as *const u8 as *const libc::c_char);
                let mut sbuff_15: [libc::c_char; 12] = [0; 12];
                let mut length_15: libc::c_int = 0;
                if 0 as libc::c_int > 0 as libc::c_int {
                    length_15 = sprintf(
                        sbuff_15.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        index_7,
                    );
                } else {
                    length_15 = sprintf(
                        sbuff_15.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_7,
                    );
                }
                sbuff_15[length_15 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_15.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" '\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write).unwrap()(vm, ((*name_3).data).as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"'\n\0" as *const u8 as *const libc::c_char);
            }
            46 => {
                let mut sbuff_16: [libc::c_char; 12] = [0; 12];
                let mut length_16: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    let fresh8 = i;
                    i = i.wrapping_add(1);
                    length_16 = sprintf(
                        sbuff_16.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        *opcodes.offset(fresh8 as isize) as libc::c_int,
                    );
                } else {
                    let fresh9 = i;
                    i = i.wrapping_add(1);
                    length_16 = sprintf(
                        sbuff_16.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        *opcodes.offset(fresh9 as isize) as libc::c_int,
                    );
                }
                sbuff_16[length_16 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_16.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" (argc)\n\0" as *const u8 as *const libc::c_char);
            }
            47 => {
                let mut sbuff_17: [libc::c_char; 12] = [0; 12];
                let mut length_17: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    let fresh10 = i;
                    i = i.wrapping_add(1);
                    length_17 = sprintf(
                        sbuff_17.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        *opcodes.offset(fresh10 as isize) as libc::c_int,
                    );
                } else {
                    let fresh11 = i;
                    i = i.wrapping_add(1);
                    length_17 = sprintf(
                        sbuff_17.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        *opcodes.offset(fresh11 as isize) as libc::c_int,
                    );
                }
                sbuff_17[length_17 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_17.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" (argc)\n\0" as *const u8 as *const libc::c_char);
            }
            48 => {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            49 | 50 | 52 | 53 | 54 | 55 => {
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut offset: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut sbuff_18: [libc::c_char; 12] = [0; 12];
                let mut length_18: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_18 = sprintf(
                        sbuff_18.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        offset,
                    );
                } else {
                    length_18 = sprintf(
                        sbuff_18.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        offset,
                    );
                }
                sbuff_18[length_18 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_18.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" (ip:\0" as *const u8 as *const libc::c_char);
                let mut sbuff_19: [libc::c_char; 12] = [0; 12];
                let mut length_19: libc::c_int = 0;
                if 0 as libc::c_int > 0 as libc::c_int {
                    length_19 = sprintf(
                        sbuff_19.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        i.wrapping_add(offset as libc::c_uint),
                    );
                } else {
                    length_19 = sprintf(
                        sbuff_19.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        i.wrapping_add(offset as libc::c_uint),
                    );
                }
                sbuff_19[length_19 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_19.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b")\n\0" as *const u8 as *const libc::c_char);
            }
            51 => {
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut offset_0: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut sbuff_20: [libc::c_char; 12] = [0; 12];
                let mut length_20: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_20 = sprintf(
                        sbuff_20.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        -offset_0,
                    );
                } else {
                    length_20 = sprintf(
                        sbuff_20.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        -offset_0,
                    );
                }
                sbuff_20[length_20 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_20.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" (ip:\0" as *const u8 as *const libc::c_char);
                let mut sbuff_21: [libc::c_char; 12] = [0; 12];
                let mut length_21: libc::c_int = 0;
                if 0 as libc::c_int > 0 as libc::c_int {
                    length_21 = sprintf(
                        sbuff_21.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        i.wrapping_sub(offset_0 as libc::c_uint),
                    );
                } else {
                    length_21 = sprintf(
                        sbuff_21.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        i.wrapping_sub(offset_0 as libc::c_uint),
                    );
                }
                sbuff_21[length_21 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_21.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b")\n\0" as *const u8 as *const libc::c_char);
            }
            56 => {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            57 | 58 | 59 => {
                i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                let mut index_8: libc::c_int = (*opcodes
                    .offset(i.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int) << 8 as libc::c_int
                    | *opcodes
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int;
                let mut name_4: *mut String_0 = moduleGetStringAt(
                    (*func).owner,
                    index_8,
                );
                let mut sbuff_22: [libc::c_char; 12] = [0; 12];
                let mut length_22: libc::c_int = 0;
                if 5 as libc::c_int > 0 as libc::c_int {
                    length_22 = sprintf(
                        sbuff_22.as_mut_ptr(),
                        b"%*d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                        index_8,
                    );
                } else {
                    length_22 = sprintf(
                        sbuff_22.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        index_8,
                    );
                }
                sbuff_22[length_22 as usize] = '\0' as i32 as libc::c_char;
                ((*vm).config.stdout_write).unwrap()(vm, sbuff_22.as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b" '\0" as *const u8 as *const libc::c_char);
                ((*vm).config.stdout_write).unwrap()(vm, ((*name_4).data).as_mut_ptr());
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"'\n\0" as *const u8 as *const libc::c_char);
            }
            60 | 61 | 62 => {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 => {
                let fresh12 = i;
                i = i.wrapping_add(1);
                let mut inplace: uint8_t = *opcodes.offset(fresh12 as isize);
                if inplace as libc::c_int == 1 as libc::c_int {
                    ((*vm).config.stdout_write)
                        .unwrap()(
                        vm,
                        b"(inplace)\n\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    ((*vm).config.stdout_write)
                        .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
                }
            }
            78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 => {
                ((*vm).config.stdout_write)
                    .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                unreachable!();
            }
        }
    }
    ((*vm).config.stdout_write)
        .unwrap()(vm, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn dumpGlobalValues(mut vm: *mut PKVM) {
    let mut fiber: *mut Fiber = (*vm).fiber;
    let mut frame_ind: libc::c_int = (*fiber).frame_count - 1 as libc::c_int;
    let mut frame: *mut CallFrame = &mut *((*fiber).frames).offset(frame_ind as isize)
        as *mut CallFrame;
    let mut module: *mut Module = (*(*(*frame).closure).fn_0).owner;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*module).global_names.count {
        let mut name: *mut String_0 = moduleGetStringAt(
            module,
            *((*module).global_names.data).offset(i as isize) as libc::c_int,
        );
        let mut value: Var = *((*module).globals.data).offset(i as isize);
        printf(
            b"%10s = \0" as *const u8 as *const libc::c_char,
            ((*name).data).as_mut_ptr(),
        );
        dumpValue(vm, value);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn dumpStackFrame(mut vm: *mut PKVM) {
    let mut fiber: *mut Fiber = (*vm).fiber;
    let mut frame_ind: libc::c_int = (*fiber).frame_count - 1 as libc::c_int;
    let mut frame: *mut CallFrame = &mut *((*fiber).frames).offset(frame_ind as isize)
        as *mut CallFrame;
    let mut sp: *mut Var = ((*fiber).sp).offset(-(1 as libc::c_int as isize));
    printf(b"Frame[%d]\n\0" as *const u8 as *const libc::c_char, frame_ind);
    while sp >= (*frame).rbp {
        printf(b"       \0" as *const u8 as *const libc::c_char);
        dumpValue(vm, *sp);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        sp = sp.offset(-1);
        sp;
    }
}
