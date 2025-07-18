use ::libc;
extern "C" {
    pub type Compiler;
    
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn pkGetSlotType(vm: *mut PKVM, index: libc::c_int) -> PkVarType;
    fn pkGetSlotBool(vm: *mut PKVM, index: libc::c_int) -> bool;
    fn pkGetSlotString(
        vm: *mut PKVM,
        index: libc::c_int,
        length: *mut uint32_t,
    ) -> *const libc::c_char;
    fn pkSetSlotBool(vm: *mut PKVM, index: libc::c_int, value: bool);
    fn pkSetSlotNumber(vm: *mut PKVM, index: libc::c_int, value: libc::c_double);
    fn pkSetSlotStringLength(
        vm: *mut PKVM,
        index: libc::c_int,
        value: *const libc::c_char,
        length: uint32_t,
    );
    fn pkSetSlotStringFmt(
        vm: *mut PKVM,
        index: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pkByteBufferWrite(self_0: *mut pkByteBuffer, vm: *mut PKVM, data: uint8_t);
    fn pkByteBufferInit(self_0: *mut pkByteBuffer);
    fn pkByteBufferReserve(self_0: *mut pkByteBuffer, vm: *mut PKVM, size: size_t);
    fn pkByteBufferFill(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        data: uint8_t,
        count: libc::c_int,
    );
    fn pkByteBufferAddString(
        self_0: *mut pkByteBuffer,
        vm: *mut PKVM,
        str: *const libc::c_char,
        length: uint32_t,
    );
    fn getPkVarTypeName(type_0: PkVarType) -> *const libc::c_char;
    fn varTypeName(v: Var) -> *const libc::c_char;
    fn varHashValue(v: Var) -> uint32_t;
    fn isObjectHashable(type_0: ObjectType) -> bool;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
unsafe extern "C" fn _typesHashable(mut vm: *mut PKVM) {
    let mut value: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    if !(value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
    {
        pkSetSlotBool(vm, 0 as libc::c_int, 1 as libc::c_int != 0);
    } else {
        pkSetSlotBool(
            vm,
            0 as libc::c_int,
            isObjectHashable(
                (*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                    .type_0,
            ),
        );
    };
}
static mut _pk_doc__typesHashable: *const libc::c_char = b"types.hashable(value:Var) -> Bool\n\nReturns true if the [value] is hashable.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__typesHash: *const libc::c_char = b"types.hash(value:Var) -> Number\n\nReturns the hash of the [value]\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _typesHash(mut vm: *mut PKVM) {
    let mut value: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    if value
        & (0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong)
        == 0x7ffc000000000000 as libc::c_long as uint64_t
            | 0x8000000000000000 as libc::c_ulong
        && !isObjectHashable(
            (*((value & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object))
                .type_0,
        )
    {
        pkSetRuntimeErrorFmt(
            vm,
            b"Type '%s' is not hashable.\0" as *const u8 as *const libc::c_char,
            varTypeName(value),
        );
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, varHashValue(value) as libc::c_double);
}
unsafe extern "C" fn _bytebuffNew(mut vm: *mut PKVM) -> *mut libc::c_void {
    let mut self_0: *mut pkByteBuffer = pkRealloc(
        vm,
        0 as *mut libc::c_void,
        ::std::mem::size_of::<pkByteBuffer>() as libc::c_ulong,
    ) as *mut pkByteBuffer;
    pkByteBufferInit(self_0);
    return self_0 as *mut libc::c_void;
}
unsafe extern "C" fn _bytebuffDelete(mut vm: *mut PKVM, mut buff: *mut libc::c_void) {
    pkRealloc(vm, buff, 0 as libc::c_int as size_t);
}
static mut _pk_doc__bytebuffReserve: *const libc::c_char = b"types.ByteBuffer.reserve(count:Number) -> Null\n\nReserve [count] number of bytes internally. This is use full if the final size of the buffer is known beforehand to avoid reduce the number of re-allocations.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffReserve(mut vm: *mut PKVM) {
    let mut size: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut size) {
        return;
    }
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    pkByteBufferReserve(self_0, vm, size as size_t);
}
static mut _pk_doc__bytebuffFill: *const libc::c_char = b"types.ByteBuffer.fill(value:Number) -> Null\n\nFill the buffer with the given byte value. Note that the value must be in between 0 and 0xff inclusive.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffFill(mut vm: *mut PKVM) {
    let mut n: uint32_t = 0;
    if !pkValidateSlotInteger(
        vm,
        1 as libc::c_int,
        &mut n as *mut uint32_t as *mut int32_t,
    ) {
        return;
    }
    if n < 0 as libc::c_int as libc::c_uint || n > 0xff as libc::c_int as libc::c_uint {
        pkSetRuntimeErrorFmt(
            vm,
            b"Expected integer in range 0x00 to 0xff, got %i.\0" as *const u8
                as *const libc::c_char,
            n,
        );
        return;
    }
    let mut count: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut count) {
        return;
    }
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    pkByteBufferFill(self_0, vm, n as uint8_t, count as libc::c_int);
}
static mut _pk_doc__bytebuffClear: *const libc::c_char = b"types.ByteBuffer.clear() -> Null\n\nClear the buffer values.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffClear(mut vm: *mut PKVM) {
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    (*self_0).count = 0 as libc::c_int as uint32_t;
}
static mut _pk_doc__bytebuffWrite: *const libc::c_char = b"types.ByteBuffer.write(data:Number|String) -> Null\n\nWrites the data to the buffer. If the [data] is a number that should be in between 0 and 0xff inclusively. If the [data] is a string all the bytes of the string will be written to the buffer.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffWrite(mut vm: *mut PKVM) {
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    let mut type_0: PkVarType = pkGetSlotType(vm, 1 as libc::c_int);
    match type_0 as libc::c_uint {
        2 => {
            pkByteBufferWrite(
                self_0,
                vm,
                (if pkGetSlotBool(vm, 1 as libc::c_int) as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint8_t,
            );
            pkSetSlotNumber(vm, 0 as libc::c_int, 1 as libc::c_int as libc::c_double);
            return;
        }
        3 => {
            let mut i: uint32_t = 0;
            if !pkValidateSlotInteger(
                vm,
                1 as libc::c_int,
                &mut i as *mut uint32_t as *mut int32_t,
            ) {
                return;
            }
            if i < 0 as libc::c_int as libc::c_uint
                || i > 0xff as libc::c_int as libc::c_uint
            {
                pkSetRuntimeErrorFmt(
                    vm,
                    b"Expected integer in range 0x00 to 0xff, got %i.\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
                return;
            }
            pkByteBufferWrite(self_0, vm, i as uint8_t);
            pkSetSlotNumber(vm, 0 as libc::c_int, 1 as libc::c_int as libc::c_double);
            return;
        }
        4 => {
            let mut length: uint32_t = 0;
            let mut str: *const libc::c_char = pkGetSlotString(
                vm,
                1 as libc::c_int,
                &mut length,
            );
            pkByteBufferAddString(self_0, vm, str, length);
            pkSetSlotNumber(vm, 0 as libc::c_int, length as libc::c_double);
            return;
        }
        5 | _ => {}
    }
    let mut name: *const libc::c_char = getPkVarTypeName(type_0);
    pkSetRuntimeErrorFmt(
        vm,
        b"Object %s cannot be written to ByteBuffer.\0" as *const u8
            as *const libc::c_char,
        name,
    );
}
static mut _pk_doc__bytebuffSubscriptGet: *const libc::c_char = b"types.ByteBuffer.[](index:Number)\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffSubscriptGet(mut vm: *mut PKVM) {
    let mut index: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut index) {
        return;
    }
    if floor(index) != index {
        pkSetRuntimeError(
            vm,
            b"Expected an integer but got number.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    if index < 0 as libc::c_int as libc::c_double
        || index >= (*self_0).count as libc::c_double
    {
        pkSetRuntimeError(
            vm,
            b"Index out of bound\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    pkSetSlotNumber(
        vm,
        0 as libc::c_int,
        *((*self_0).data).offset(index as uint32_t as isize) as libc::c_double,
    );
}
static mut _pk_doc__bytebuffSubscriptSet: *const libc::c_char = b"types.ByteBuffer.[]=(index:Number, value:Number)\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffSubscriptSet(mut vm: *mut PKVM) {
    let mut index: libc::c_double = 0.;
    let mut value: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut index) {
        return;
    }
    if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut value) {
        return;
    }
    if floor(index) != index {
        pkSetRuntimeError(
            vm,
            b"Expected an integer but got float.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if floor(value) != value {
        pkSetRuntimeError(
            vm,
            b"Expected an integer but got float.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    if index < 0 as libc::c_int as libc::c_double
        || index >= (*self_0).count as libc::c_double
    {
        pkSetRuntimeError(
            vm,
            b"Index out of bound\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if value < 0 as libc::c_int as libc::c_double
        || value > 0xff as libc::c_int as libc::c_double
    {
        pkSetRuntimeError(
            vm,
            b"Value should be in range 0x00 to 0xff.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    *((*self_0).data).offset(index as uint32_t as isize) = value as uint8_t;
}
static mut _pk_doc__bytebuffString: *const libc::c_char = b"types.ByteBuffer.string() -> String\n\nReturns the buffered values as String.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffString(mut vm: *mut PKVM) {
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    pkSetSlotStringLength(
        vm,
        0 as libc::c_int,
        (*self_0).data as *const libc::c_char,
        (*self_0).count,
    );
}
static mut _pk_doc__bytebuffCount: *const libc::c_char = b"types.ByteBuffer.count() -> Number\n\nReturns the number of bytes that have written to the buffer.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _bytebuffCount(mut vm: *mut PKVM) {
    let mut self_0: *mut pkByteBuffer = pkGetSelf(vm) as *mut pkByteBuffer;
    pkSetSlotNumber(vm, 0 as libc::c_int, (*self_0).count as libc::c_double);
}
unsafe extern "C" fn _vectorNew(mut vm: *mut PKVM) -> *mut libc::c_void {
    let mut vec: *mut Vector = pkRealloc(
        vm,
        0 as *mut libc::c_void,
        ::std::mem::size_of::<Vector>() as libc::c_ulong,
    ) as *mut Vector;
    memset(
        vec as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Vector>() as libc::c_ulong,
    );
    return vec as *mut libc::c_void;
}
unsafe extern "C" fn _vectorDelete(mut vm: *mut PKVM, mut vec: *mut libc::c_void) {
    pkRealloc(vm, vec, 0 as libc::c_int as size_t);
}
static mut _pk_doc__vectorInit: *const libc::c_char = b"types.Vector._init()\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _vectorInit(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = pkGetArgc(vm);
    if !pkCheckArgcRange(vm, argc, 0 as libc::c_int, 3 as libc::c_int) {
        return;
    }
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut vec: *mut Vector = pkGetSelf(vm) as *mut Vector;
    if argc == 0 as libc::c_int {
        return;
    }
    if argc >= 1 as libc::c_int {
        if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut x) {
            return;
        }
        (*vec).x = x;
    }
    if argc >= 2 as libc::c_int {
        if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut y) {
            return;
        }
        (*vec).y = y;
    }
    if argc == 3 as libc::c_int {
        if !pkValidateSlotNumber(vm, 3 as libc::c_int, &mut z) {
            return;
        }
        (*vec).z = z;
    }
}
static mut _pk_doc__vectorGetter: *const libc::c_char = b"types.Vector.@getter()\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _vectorGetter(mut vm: *mut PKVM) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: uint32_t = 0;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut name, &mut length) {
        return;
    }
    let mut vec: *mut Vector = pkGetSelf(vm) as *mut Vector;
    if length == 1 as libc::c_int as libc::c_uint {
        if *name as libc::c_int == 'x' as i32 {
            pkSetSlotNumber(vm, 0 as libc::c_int, (*vec).x);
            return;
        } else if *name as libc::c_int == 'y' as i32 {
            pkSetSlotNumber(vm, 0 as libc::c_int, (*vec).y);
            return;
        } else if *name as libc::c_int == 'z' as i32 {
            pkSetSlotNumber(vm, 0 as libc::c_int, (*vec).z);
            return;
        }
    }
}
static mut _pk_doc__vectorSetter: *const libc::c_char = b"types.Vector.@setter()\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _vectorSetter(mut vm: *mut PKVM) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: uint32_t = 0;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut name, &mut length) {
        return;
    }
    let mut vec: *mut Vector = pkGetSelf(vm) as *mut Vector;
    if length == 1 as libc::c_int as libc::c_uint {
        if *name as libc::c_int == 'x' as i32 {
            let mut val: libc::c_double = 0.;
            if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut val) {
                return;
            }
            (*vec).x = val;
            return;
        } else if *name as libc::c_int == 'y' as i32 {
            let mut val_0: libc::c_double = 0.;
            if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut val_0) {
                return;
            }
            (*vec).y = val_0;
            return;
        } else if *name as libc::c_int == 'z' as i32 {
            let mut val_1: libc::c_double = 0.;
            if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut val_1) {
                return;
            }
            (*vec).z = val_1;
            return;
        }
    }
}
static mut _pk_doc__vectorRepr: *const libc::c_char = b"types.Vector._repr()\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _vectorRepr(mut vm: *mut PKVM) {
    let mut vec: *mut Vector = pkGetSelf(vm) as *mut Vector;
    pkSetSlotStringFmt(
        vm,
        0 as libc::c_int,
        b"[%g, %g, %g]\0" as *const u8 as *const libc::c_char,
        (*vec).x,
        (*vec).y,
        (*vec).z,
    );
}
pub unsafe extern "C" fn registerModuleTypes(mut vm: *mut PKVM) {
    let mut types: *mut PkHandle = pkNewModule(
        vm,
        b"types\0" as *const u8 as *const libc::c_char,
    );
    pkModuleAddFunction(
        vm,
        types,
        b"hashable\0" as *const u8 as *const libc::c_char,
        Some(_typesHashable as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__typesHashable,
    );
    pkModuleAddFunction(
        vm,
        types,
        b"hash\0" as *const u8 as *const libc::c_char,
        Some(_typesHash as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__typesHash,
    );
    let mut cls_byte_buffer: *mut PkHandle = pkNewClass(
        vm,
        b"ByteBuffer\0" as *const u8 as *const libc::c_char,
        0 as *mut PkHandle,
        types,
        Some(_bytebuffNew as unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void),
        Some(
            _bytebuffDelete as unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
        ),
        b"A simple dynamically allocated byte buffer type. This can be used for constructing larger strings without allocating and adding smaller intermeidate strings.\0"
            as *const u8 as *const libc::c_char,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"[]\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffSubscriptGet as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__bytebuffSubscriptGet,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"[]=\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffSubscriptSet as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__bytebuffSubscriptSet,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"reserve\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffReserve as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__bytebuffReserve,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"fill\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffFill as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__bytebuffFill,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"clear\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffClear as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__bytebuffClear,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"write\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffWrite as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__bytebuffWrite,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"string\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffString as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__bytebuffString,
    );
    pkClassAddMethod(
        vm,
        cls_byte_buffer,
        b"count\0" as *const u8 as *const libc::c_char,
        Some(_bytebuffCount as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__bytebuffCount,
    );
    pkReleaseHandle(vm, cls_byte_buffer);
    let mut cls_vector: *mut PkHandle = pkNewClass(
        vm,
        b"Vector\0" as *const u8 as *const libc::c_char,
        0 as *mut PkHandle,
        types,
        Some(_vectorNew as unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void),
        Some(_vectorDelete as unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> ()),
        b"A simple vector type contains x, y, and z components.\0" as *const u8
            as *const libc::c_char,
    );
    pkClassAddMethod(
        vm,
        cls_vector,
        b"_init\0" as *const u8 as *const libc::c_char,
        Some(_vectorInit as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__vectorInit,
    );
    pkClassAddMethod(
        vm,
        cls_vector,
        b"@getter\0" as *const u8 as *const libc::c_char,
        Some(_vectorGetter as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__vectorGetter,
    );
    pkClassAddMethod(
        vm,
        cls_vector,
        b"@setter\0" as *const u8 as *const libc::c_char,
        Some(_vectorSetter as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__vectorSetter,
    );
    pkClassAddMethod(
        vm,
        cls_vector,
        b"_repr\0" as *const u8 as *const libc::c_char,
        Some(_vectorRepr as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__vectorRepr,
    );
    pkReleaseHandle(vm, cls_vector);
    pkRegisterModule(vm, types);
    pkReleaseHandle(vm, types);
}
