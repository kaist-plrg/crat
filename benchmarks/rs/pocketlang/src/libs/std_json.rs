use ::libc;
extern "C" {
    pub type Compiler;
    
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn pkSetRuntimeError(vm: *mut PKVM, message: *const libc::c_char);
    fn pkSetRuntimeErrorFmt(vm: *mut PKVM, fmt: *const libc::c_char, _: ...);
    fn pkGetArgc(vm: *const PKVM) -> libc::c_int;
    fn pkCheckArgcRange(
        vm: *mut PKVM,
        argc: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    ) -> bool;
    fn pkValidateSlotBool(vm: *mut PKVM, slot: libc::c_int, value: *mut bool) -> bool;
    fn pkValidateSlotString(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut *const libc::c_char,
        length: *mut uint32_t,
    ) -> bool;
    fn pkSetSlotString(vm: *mut PKVM, index: libc::c_int, value: *const libc::c_char);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn pkVarBufferWrite(self_0: *mut pkVarBuffer, vm: *mut PKVM, data: Var);
    fn newStringLength(
        vm: *mut PKVM,
        text: *const libc::c_char,
        length: uint32_t,
    ) -> *mut String_0;
    fn newList(vm: *mut PKVM, size: uint32_t) -> *mut List;
    fn newMap(vm: *mut PKVM) -> *mut Map;
    fn doubleToVar(value: libc::c_double) -> Var;
    fn varToDouble(value: Var) -> libc::c_double;
    fn varTypeName(v: Var) -> *const libc::c_char;
    fn getVarType(v: Var) -> PkVarType;
    fn vmPushTempRef(vm: *mut PKVM, obj: *mut Object);
    fn vmPopTempRef(vm: *mut PKVM);
    fn mapSet(vm: *mut PKVM, self_0: *mut Map, key: Var, value: Var);
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
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
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub type_0: libc::c_int,
    pub valuestring: *mut libc::c_char,
    pub valueint: libc::c_int,
    pub valuedouble: libc::c_double,
    pub string: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub free_fn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type cJSON_bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_hooks {
    pub allocate: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub deallocate: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub reallocate: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error {
    pub json: *const libc::c_uchar,
    pub position: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse_buffer {
    pub content: *const libc::c_uchar,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub hooks: internal_hooks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuffer {
    pub buffer: *mut libc::c_uchar,
    pub length: size_t,
    pub offset: size_t,
    pub depth: size_t,
    pub noalloc: cJSON_bool,
    pub format: cJSON_bool,
    pub hooks: internal_hooks,
}
static mut global_error: error = {
    let mut init = error {
        json: 0 as *const libc::c_uchar,
        position: 0 as libc::c_int as size_t,
    };
    init
};
pub unsafe extern "C" fn cJSON_GetErrorPtr() -> *const libc::c_char {
    return (global_error.json).offset(global_error.position as isize)
        as *const libc::c_char;
}
pub unsafe extern "C" fn cJSON_GetStringValue(item: *const cJSON) -> *mut libc::c_char {
    if cJSON_IsString(item) == 0 {
        return 0 as *mut libc::c_char;
    }
    return (*item).valuestring;
}
pub unsafe extern "C" fn cJSON_GetNumberValue(item: *const cJSON) -> libc::c_double {
    if cJSON_IsNumber(item) == 0 {
        return ::std::f32::NAN as libc::c_double;
    }
    return (*item).valuedouble;
}
pub unsafe extern "C" fn cJSON_Version() -> *const libc::c_char {
    static mut version: [libc::c_char; 15] = [0; 15];
    sprintf(
        version.as_mut_ptr(),
        b"%i.%i.%i\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        7 as libc::c_int,
        15 as libc::c_int,
    );
    return version.as_mut_ptr();
}
unsafe extern "C" fn case_insensitive_strcmp(
    mut string1: *const libc::c_uchar,
    mut string2: *const libc::c_uchar,
) -> libc::c_int {
    if string1.is_null() || string2.is_null() {
        return 1 as libc::c_int;
    }
    if string1 == string2 {
        return 0 as libc::c_int;
    }
    while ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = *string1 as libc::c_int;
                __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                });
            } else {
                __res = tolower(*string1 as libc::c_int);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(*string1 as libc::c_int as isize);
        }
        __res
    })
        == ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *string2 as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*string2 as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*string2 as libc::c_int as isize);
            }
            __res
        })
    {
        if *string1 as libc::c_int == '\0' as i32 {
            return 0 as libc::c_int;
        }
        string1 = string1.offset(1);
        string1;
        string2 = string2.offset(1);
        string2;
    }
    return ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = *string1 as libc::c_int;
                __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                });
            } else {
                __res = tolower(*string1 as libc::c_int);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(*string1 as libc::c_int as isize);
        }
        __res
    })
        - ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *string2 as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*string2 as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*string2 as libc::c_int as isize);
            }
            __res
        });
}
static mut global_hooks: internal_hooks = unsafe {
    {
        let mut init = internal_hooks {
            allocate: Some(
                malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
            ),
            deallocate: Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            reallocate: Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_ulong,
                    ) -> *mut libc::c_void,
            ),
        };
        init
    }
};
unsafe extern "C" fn cJSON_strdup(
    mut string: *const libc::c_uchar,
    hooks: *const internal_hooks,
) -> *mut libc::c_uchar {
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut copy: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if string.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    length = (strlen(string as *const libc::c_char))
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong);
    copy = ((*hooks).allocate).unwrap()(length) as *mut libc::c_uchar;
    if copy.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    memcpy(copy as *mut libc::c_void, string as *const libc::c_void, length);
    return copy;
}
pub unsafe extern "C" fn cJSON_InitHooks(mut hooks: *mut cJSON_Hooks) {
    if hooks.is_null() {
        global_hooks
            .allocate = Some(
            malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
        );
        global_hooks
            .deallocate = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
        global_hooks
            .reallocate = Some(
            realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> *mut libc::c_void,
        );
        return;
    }
    global_hooks
        .allocate = Some(
        malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
    );
    if ((*hooks).malloc_fn).is_some() {
        global_hooks.allocate = (*hooks).malloc_fn;
    }
    global_hooks
        .deallocate = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    if ((*hooks).free_fn).is_some() {
        global_hooks.deallocate = (*hooks).free_fn;
    }
    global_hooks.reallocate = None;
    if global_hooks.allocate
        == Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void)
        && global_hooks.deallocate
            == Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())
    {
        global_hooks
            .reallocate = Some(
            realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_ulong,
                ) -> *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn cJSON_New_Item(hooks: *const internal_hooks) -> *mut cJSON {
    let mut node: *mut cJSON = ((*hooks).allocate)
        .unwrap()(::std::mem::size_of::<cJSON>() as libc::c_ulong) as *mut cJSON;
    if !node.is_null() {
        memset(
            node as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<cJSON>() as libc::c_ulong,
        );
    }
    return node;
}
pub unsafe extern "C" fn cJSON_Delete(mut item: *mut cJSON) {
    let mut next: *mut cJSON = 0 as *mut cJSON;
    while !item.is_null() {
        next = (*item).next;
        if (*item).type_0 & 256 as libc::c_int == 0 && !((*item).child).is_null() {
            cJSON_Delete((*item).child);
        }
        if (*item).type_0 & 256 as libc::c_int == 0 && !((*item).valuestring).is_null() {
            (global_hooks.deallocate).unwrap()((*item).valuestring as *mut libc::c_void);
        }
        if (*item).type_0 & 512 as libc::c_int == 0 && !((*item).string).is_null() {
            (global_hooks.deallocate).unwrap()((*item).string as *mut libc::c_void);
        }
        (global_hooks.deallocate).unwrap()(item as *mut libc::c_void);
        item = next;
    }
}
unsafe extern "C" fn get_decimal_point() -> libc::c_uchar {
    return '.' as i32 as libc::c_uchar;
}
unsafe extern "C" fn parse_number(
    item: *mut cJSON,
    input_buffer: *mut parse_buffer,
) -> cJSON_bool {
    let mut number: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut after_end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut number_c_string: [libc::c_uchar; 64] = [0; 64];
    let mut decimal_point: libc::c_uchar = get_decimal_point();
    let mut i: size_t = 0 as libc::c_int as size_t;
    if input_buffer.is_null() || ((*input_buffer).content).is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && (!input_buffer.is_null()
            && ((*input_buffer).offset).wrapping_add(i) < (*input_buffer).length)
    {
        match *((*input_buffer).content)
            .offset((*input_buffer).offset as isize)
            .offset(i as isize) as libc::c_int
        {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 43 | 45 | 101 | 69 => {
                number_c_string[i
                    as usize] = *((*input_buffer).content)
                    .offset((*input_buffer).offset as isize)
                    .offset(i as isize);
            }
            46 => {
                number_c_string[i as usize] = decimal_point;
            }
            _ => {
                break;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    number_c_string[i as usize] = '\0' as i32 as libc::c_uchar;
    number = strtod(
        number_c_string.as_mut_ptr() as *const libc::c_char,
        &mut after_end as *mut *mut libc::c_uchar as *mut *mut libc::c_char,
    );
    if number_c_string.as_mut_ptr() == after_end {
        return 0 as libc::c_int;
    }
    (*item).valuedouble = number;
    if number >= 2147483647 as libc::c_int as libc::c_double {
        (*item).valueint = 2147483647 as libc::c_int;
    } else if number
        <= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
    {
        (*item).valueint = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    } else {
        (*item).valueint = number as libc::c_int;
    }
    (*item).type_0 = (1 as libc::c_int) << 3 as libc::c_int;
    (*input_buffer)
        .offset = ((*input_buffer).offset as libc::c_ulong)
        .wrapping_add(
            after_end.offset_from(number_c_string.as_mut_ptr()) as libc::c_long as size_t,
        ) as size_t as size_t;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn cJSON_SetNumberHelper(
    mut object: *mut cJSON,
    mut number: libc::c_double,
) -> libc::c_double {
    if number >= 2147483647 as libc::c_int as libc::c_double {
        (*object).valueint = 2147483647 as libc::c_int;
    } else if number
        <= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
    {
        (*object).valueint = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    } else {
        (*object).valueint = number as libc::c_int;
    }
    (*object).valuedouble = number;
    return (*object).valuedouble;
}
pub unsafe extern "C" fn cJSON_SetValuestring(
    mut object: *mut cJSON,
    mut valuestring: *const libc::c_char,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*object).type_0 & (1 as libc::c_int) << 4 as libc::c_int == 0
        || (*object).type_0 & 256 as libc::c_int != 0
    {
        return 0 as *mut libc::c_char;
    }
    if strlen(valuestring) <= strlen((*object).valuestring) {
        strcpy((*object).valuestring, valuestring);
        return (*object).valuestring;
    }
    copy = cJSON_strdup(valuestring as *const libc::c_uchar, &mut global_hooks)
        as *mut libc::c_char;
    if copy.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !((*object).valuestring).is_null() {
        cJSON_free((*object).valuestring as *mut libc::c_void);
    }
    (*object).valuestring = copy;
    return copy;
}
unsafe extern "C" fn ensure(
    p: *mut printbuffer,
    mut needed: size_t,
) -> *mut libc::c_uchar {
    let mut newbuffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut newsize: size_t = 0 as libc::c_int as size_t;
    if p.is_null() || ((*p).buffer).is_null() {
        return 0 as *mut libc::c_uchar;
    }
    if (*p).length > 0 as libc::c_int as libc::c_ulong && (*p).offset >= (*p).length {
        return 0 as *mut libc::c_uchar;
    }
    if needed > 2147483647 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_uchar;
    }
    needed = (needed as libc::c_ulong)
        .wrapping_add(((*p).offset).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as size_t as size_t;
    if needed <= (*p).length {
        return ((*p).buffer).offset((*p).offset as isize);
    }
    if (*p).noalloc != 0 {
        return 0 as *mut libc::c_uchar;
    }
    if needed > (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_ulong {
        if needed <= 2147483647 as libc::c_int as libc::c_ulong {
            newsize = 2147483647 as libc::c_int as size_t;
        } else {
            return 0 as *mut libc::c_uchar
        }
    } else {
        newsize = needed.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    }
    if ((*p).hooks.reallocate).is_some() {
        newbuffer = ((*p).hooks.reallocate)
            .unwrap()((*p).buffer as *mut libc::c_void, newsize) as *mut libc::c_uchar;
        if newbuffer.is_null() {
            ((*p).hooks.deallocate).unwrap()((*p).buffer as *mut libc::c_void);
            (*p).length = 0 as libc::c_int as size_t;
            (*p).buffer = 0 as *mut libc::c_uchar;
            return 0 as *mut libc::c_uchar;
        }
    } else {
        newbuffer = ((*p).hooks.allocate).unwrap()(newsize) as *mut libc::c_uchar;
        if newbuffer.is_null() {
            ((*p).hooks.deallocate).unwrap()((*p).buffer as *mut libc::c_void);
            (*p).length = 0 as libc::c_int as size_t;
            (*p).buffer = 0 as *mut libc::c_uchar;
            return 0 as *mut libc::c_uchar;
        }
        memcpy(
            newbuffer as *mut libc::c_void,
            (*p).buffer as *const libc::c_void,
            ((*p).offset).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        ((*p).hooks.deallocate).unwrap()((*p).buffer as *mut libc::c_void);
    }
    (*p).length = newsize;
    (*p).buffer = newbuffer;
    return newbuffer.offset((*p).offset as isize);
}
unsafe extern "C" fn update_offset(buffer: *mut printbuffer) {
    let mut buffer_pointer: *const libc::c_uchar = 0 as *const libc::c_uchar;
    if buffer.is_null() || ((*buffer).buffer).is_null() {
        return;
    }
    buffer_pointer = ((*buffer).buffer).offset((*buffer).offset as isize);
    (*buffer)
        .offset = ((*buffer).offset as libc::c_ulong)
        .wrapping_add(strlen(buffer_pointer as *const libc::c_char)) as size_t as size_t;
}
unsafe extern "C" fn compare_double(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> cJSON_bool {
    let mut maxVal: libc::c_double = if fabs(a) > fabs(b) { fabs(a) } else { fabs(b) };
    return (fabs(a - b) <= maxVal * 2.2204460492503131e-16f64) as libc::c_int;
}
unsafe extern "C" fn print_number(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output_pointer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut d: libc::c_double = (*item).valuedouble;
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut number_buffer: [libc::c_uchar; 26] = [
        0 as libc::c_int as libc::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut decimal_point: libc::c_uchar = get_decimal_point();
    let mut test: libc::c_double = 0.0f64;
    if output_buffer.is_null() {
        return 0 as libc::c_int;
    }
    if d.is_nan() as i32 != 0
        || if d.is_infinite() { if d.is_sign_positive() { 1 } else { -1 } } else { 0 }
            != 0
    {
        length = sprintf(
            number_buffer.as_mut_ptr() as *mut libc::c_char,
            b"null\0" as *const u8 as *const libc::c_char,
        );
    } else if d == (*item).valueint as libc::c_double {
        length = sprintf(
            number_buffer.as_mut_ptr() as *mut libc::c_char,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*item).valueint,
        );
    } else {
        length = sprintf(
            number_buffer.as_mut_ptr() as *mut libc::c_char,
            b"%1.15g\0" as *const u8 as *const libc::c_char,
            d,
        );
        if sscanf(
            number_buffer.as_mut_ptr() as *mut libc::c_char,
            b"%lg\0" as *const u8 as *const libc::c_char,
            &mut test as *mut libc::c_double,
        ) != 1 as libc::c_int || compare_double(test, d) == 0
        {
            length = sprintf(
                number_buffer.as_mut_ptr() as *mut libc::c_char,
                b"%1.17g\0" as *const u8 as *const libc::c_char,
                d,
            );
        }
    }
    if length < 0 as libc::c_int
        || length
            > (::std::mem::size_of::<[libc::c_uchar; 26]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
    {
        return 0 as libc::c_int;
    }
    output_pointer = ensure(
        output_buffer,
        (length as size_t)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong),
    );
    if output_pointer.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < length as size_t {
        if number_buffer[i as usize] as libc::c_int == decimal_point as libc::c_int {
            *output_pointer.offset(i as isize) = '.' as i32 as libc::c_uchar;
        } else {
            *output_pointer.offset(i as isize) = number_buffer[i as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    *output_pointer.offset(i as isize) = '\0' as i32 as libc::c_uchar;
    (*output_buffer)
        .offset = ((*output_buffer).offset as libc::c_ulong)
        .wrapping_add(length as size_t) as size_t as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse_hex4(input: *const libc::c_uchar) -> libc::c_uint {
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as libc::c_ulong {
        if *input.offset(i as isize) as libc::c_int >= '0' as i32
            && *input.offset(i as isize) as libc::c_int <= '9' as i32
        {
            h = h
                .wrapping_add(
                    (*input.offset(i as isize) as libc::c_uint)
                        .wrapping_sub('0' as i32 as libc::c_uint),
                );
        } else if *input.offset(i as isize) as libc::c_int >= 'A' as i32
            && *input.offset(i as isize) as libc::c_int <= 'F' as i32
        {
            h = h
                .wrapping_add(
                    (10 as libc::c_int as libc::c_uint)
                        .wrapping_add(*input.offset(i as isize) as libc::c_uint)
                        .wrapping_sub('A' as i32 as libc::c_uint),
                );
        } else if *input.offset(i as isize) as libc::c_int >= 'a' as i32
            && *input.offset(i as isize) as libc::c_int <= 'f' as i32
        {
            h = h
                .wrapping_add(
                    (10 as libc::c_int as libc::c_uint)
                        .wrapping_add(*input.offset(i as isize) as libc::c_uint)
                        .wrapping_sub('a' as i32 as libc::c_uint),
                );
        } else {
            return 0 as libc::c_int as libc::c_uint
        }
        if i < 3 as libc::c_int as libc::c_ulong {
            h = h << 4 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return h;
}
unsafe extern "C" fn utf16_literal_to_utf8(
    input_pointer: *const libc::c_uchar,
    input_end: *const libc::c_uchar,
    mut output_pointer: *mut *mut libc::c_uchar,
) -> libc::c_uchar {
    let mut current_block: u64;
    let mut codepoint: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut first_code: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut first_sequence: *const libc::c_uchar = input_pointer;
    let mut utf8_length: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut utf8_position: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut sequence_length: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut first_byte_mark: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    if !((input_end.offset_from(first_sequence) as libc::c_long)
        < 6 as libc::c_int as libc::c_long)
    {
        first_code = parse_hex4(first_sequence.offset(2 as libc::c_int as isize));
        if !(first_code >= 0xdc00 as libc::c_int as libc::c_uint
            && first_code <= 0xdfff as libc::c_int as libc::c_uint)
        {
            if first_code >= 0xd800 as libc::c_int as libc::c_uint
                && first_code <= 0xdbff as libc::c_int as libc::c_uint
            {
                let mut second_sequence: *const libc::c_uchar = first_sequence
                    .offset(6 as libc::c_int as isize);
                let mut second_code: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                sequence_length = 12 as libc::c_int as libc::c_uchar;
                if (input_end.offset_from(second_sequence) as libc::c_long)
                    < 6 as libc::c_int as libc::c_long
                {
                    current_block = 11547883441250996043;
                } else if *second_sequence.offset(0 as libc::c_int as isize)
                    as libc::c_int != '\\' as i32
                    || *second_sequence.offset(1 as libc::c_int as isize) as libc::c_int
                        != 'u' as i32
                {
                    current_block = 11547883441250996043;
                } else {
                    second_code = parse_hex4(
                        second_sequence.offset(2 as libc::c_int as isize),
                    );
                    if second_code < 0xdc00 as libc::c_int as libc::c_uint
                        || second_code > 0xdfff as libc::c_int as libc::c_uint
                    {
                        current_block = 11547883441250996043;
                    } else {
                        codepoint = (0x10000 as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                (first_code & 0x3ff as libc::c_int as libc::c_uint)
                                    << 10 as libc::c_int
                                    | second_code & 0x3ff as libc::c_int as libc::c_uint,
                            ) as libc::c_ulong;
                        current_block = 12039483399334584727;
                    }
                }
            } else {
                sequence_length = 6 as libc::c_int as libc::c_uchar;
                codepoint = first_code as libc::c_ulong;
                current_block = 12039483399334584727;
            }
            match current_block {
                11547883441250996043 => {}
                _ => {
                    if codepoint < 0x80 as libc::c_int as libc::c_ulong {
                        utf8_length = 1 as libc::c_int as libc::c_uchar;
                        current_block = 3437258052017859086;
                    } else if codepoint < 0x800 as libc::c_int as libc::c_ulong {
                        utf8_length = 2 as libc::c_int as libc::c_uchar;
                        first_byte_mark = 0xc0 as libc::c_int as libc::c_uchar;
                        current_block = 3437258052017859086;
                    } else if codepoint < 0x10000 as libc::c_int as libc::c_ulong {
                        utf8_length = 3 as libc::c_int as libc::c_uchar;
                        first_byte_mark = 0xe0 as libc::c_int as libc::c_uchar;
                        current_block = 3437258052017859086;
                    } else if codepoint <= 0x10ffff as libc::c_int as libc::c_ulong {
                        utf8_length = 4 as libc::c_int as libc::c_uchar;
                        first_byte_mark = 0xf0 as libc::c_int as libc::c_uchar;
                        current_block = 3437258052017859086;
                    } else {
                        current_block = 11547883441250996043;
                    }
                    match current_block {
                        11547883441250996043 => {}
                        _ => {
                            utf8_position = (utf8_length as libc::c_int
                                - 1 as libc::c_int) as libc::c_uchar;
                            while utf8_position as libc::c_int > 0 as libc::c_int {
                                *(*output_pointer)
                                    .offset(
                                        utf8_position as isize,
                                    ) = ((codepoint | 0x80 as libc::c_int as libc::c_ulong)
                                    & 0xbf as libc::c_int as libc::c_ulong) as libc::c_uchar;
                                codepoint >>= 6 as libc::c_int;
                                utf8_position = utf8_position.wrapping_sub(1);
                                utf8_position;
                            }
                            if utf8_length as libc::c_int > 1 as libc::c_int {
                                *(*output_pointer)
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = ((codepoint | first_byte_mark as libc::c_ulong)
                                    & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
                            } else {
                                *(*output_pointer)
                                    .offset(
                                        0 as libc::c_int as isize,
                                    ) = (codepoint & 0x7f as libc::c_int as libc::c_ulong)
                                    as libc::c_uchar;
                            }
                            *output_pointer = (*output_pointer)
                                .offset(utf8_length as libc::c_int as isize);
                            return sequence_length;
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn parse_string(
    item: *mut cJSON,
    input_buffer: *mut parse_buffer,
) -> cJSON_bool {
    let mut current_block: u64;
    let mut input_pointer: *const libc::c_uchar = ((*input_buffer).content)
        .offset((*input_buffer).offset as isize)
        .offset(1 as libc::c_int as isize);
    let mut input_end: *const libc::c_uchar = ((*input_buffer).content)
        .offset((*input_buffer).offset as isize)
        .offset(1 as libc::c_int as isize);
    let mut output_pointer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut output: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !(*((*input_buffer).content)
        .offset((*input_buffer).offset as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int != '"' as i32)
    {
        let mut allocation_length: size_t = 0 as libc::c_int as size_t;
        let mut skipped_bytes: size_t = 0 as libc::c_int as size_t;
        loop {
            if !((input_end.offset_from((*input_buffer).content) as libc::c_long
                as size_t) < (*input_buffer).length
                && *input_end as libc::c_int != '"' as i32)
            {
                current_block = 11812396948646013369;
                break;
            }
            if *input_end.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            {
                if input_end
                    .offset(1 as libc::c_int as isize)
                    .offset_from((*input_buffer).content) as libc::c_long as size_t
                    >= (*input_buffer).length
                {
                    current_block = 174580492069034133;
                    break;
                }
                skipped_bytes = skipped_bytes.wrapping_add(1);
                skipped_bytes;
                input_end = input_end.offset(1);
                input_end;
            }
            input_end = input_end.offset(1);
            input_end;
        }
        match current_block {
            174580492069034133 => {}
            _ => {
                if !(input_end.offset_from((*input_buffer).content) as libc::c_long
                    as size_t >= (*input_buffer).length
                    || *input_end as libc::c_int != '"' as i32)
                {
                    allocation_length = (input_end
                        .offset_from(
                            ((*input_buffer).content)
                                .offset((*input_buffer).offset as isize),
                        ) as libc::c_long as size_t)
                        .wrapping_sub(skipped_bytes);
                    output = ((*input_buffer).hooks.allocate)
                        .unwrap()(
                        allocation_length
                            .wrapping_add(
                                ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_uchar;
                    if !output.is_null() {
                        output_pointer = output;
                        loop {
                            if !(input_pointer < input_end) {
                                current_block = 7828949454673616476;
                                break;
                            }
                            if *input_pointer as libc::c_int != '\\' as i32 {
                                let fresh0 = input_pointer;
                                input_pointer = input_pointer.offset(1);
                                let fresh1 = output_pointer;
                                output_pointer = output_pointer.offset(1);
                                *fresh1 = *fresh0;
                            } else {
                                let mut sequence_length: libc::c_uchar = 2 as libc::c_int
                                    as libc::c_uchar;
                                if (input_end.offset_from(input_pointer) as libc::c_long)
                                    < 1 as libc::c_int as libc::c_long
                                {
                                    current_block = 174580492069034133;
                                    break;
                                }
                                match *input_pointer.offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                {
                                    98 => {
                                        let fresh2 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *fresh2 = '\u{8}' as i32 as libc::c_uchar;
                                    }
                                    102 => {
                                        let fresh3 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *fresh3 = '\u{c}' as i32 as libc::c_uchar;
                                    }
                                    110 => {
                                        let fresh4 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *fresh4 = '\n' as i32 as libc::c_uchar;
                                    }
                                    114 => {
                                        let fresh5 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *fresh5 = '\r' as i32 as libc::c_uchar;
                                    }
                                    116 => {
                                        let fresh6 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *fresh6 = '\t' as i32 as libc::c_uchar;
                                    }
                                    34 | 92 | 47 => {
                                        let fresh7 = output_pointer;
                                        output_pointer = output_pointer.offset(1);
                                        *fresh7 = *input_pointer.offset(1 as libc::c_int as isize);
                                    }
                                    117 => {
                                        sequence_length = utf16_literal_to_utf8(
                                            input_pointer,
                                            input_end,
                                            &mut output_pointer,
                                        );
                                        if sequence_length as libc::c_int == 0 as libc::c_int {
                                            current_block = 174580492069034133;
                                            break;
                                        }
                                    }
                                    _ => {
                                        current_block = 174580492069034133;
                                        break;
                                    }
                                }
                                input_pointer = input_pointer
                                    .offset(sequence_length as libc::c_int as isize);
                            }
                        }
                        match current_block {
                            174580492069034133 => {}
                            _ => {
                                *output_pointer = '\0' as i32 as libc::c_uchar;
                                (*item).type_0 = (1 as libc::c_int) << 4 as libc::c_int;
                                (*item).valuestring = output as *mut libc::c_char;
                                (*input_buffer)
                                    .offset = input_end.offset_from((*input_buffer).content)
                                    as libc::c_long as size_t;
                                (*input_buffer)
                                    .offset = ((*input_buffer).offset).wrapping_add(1);
                                (*input_buffer).offset;
                                return 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    if !output.is_null() {
        ((*input_buffer).hooks.deallocate).unwrap()(output as *mut libc::c_void);
    }
    if !input_pointer.is_null() {
        (*input_buffer)
            .offset = input_pointer.offset_from((*input_buffer).content) as libc::c_long
            as size_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_string_ptr(
    input: *const libc::c_uchar,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut input_pointer: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut output: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut output_pointer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut output_length: size_t = 0 as libc::c_int as size_t;
    let mut escape_characters: size_t = 0 as libc::c_int as size_t;
    if output_buffer.is_null() {
        return 0 as libc::c_int;
    }
    if input.is_null() {
        output = ensure(
            output_buffer,
            ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
        );
        if output.is_null() {
            return 0 as libc::c_int;
        }
        strcpy(
            output as *mut libc::c_char,
            b"\"\"\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    input_pointer = input;
    while *input_pointer != 0 {
        match *input_pointer as libc::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                escape_characters = escape_characters.wrapping_add(1);
                escape_characters;
            }
            _ => {
                if (*input_pointer as libc::c_int) < 32 as libc::c_int {
                    escape_characters = (escape_characters as libc::c_ulong)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
            }
        }
        input_pointer = input_pointer.offset(1);
        input_pointer;
    }
    output_length = (input_pointer.offset_from(input) as libc::c_long as size_t)
        .wrapping_add(escape_characters);
    output = ensure(
        output_buffer,
        output_length
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong),
    );
    if output.is_null() {
        return 0 as libc::c_int;
    }
    if escape_characters == 0 as libc::c_int as libc::c_ulong {
        *output.offset(0 as libc::c_int as isize) = '"' as i32 as libc::c_uchar;
        memcpy(
            output.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            input as *const libc::c_void,
            output_length,
        );
        *output
            .offset(
                output_length.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '"' as i32 as libc::c_uchar;
        *output
            .offset(
                output_length.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_uchar;
        return 1 as libc::c_int;
    }
    *output.offset(0 as libc::c_int as isize) = '"' as i32 as libc::c_uchar;
    output_pointer = output.offset(1 as libc::c_int as isize);
    input_pointer = input;
    while *input_pointer as libc::c_int != '\0' as i32 {
        if *input_pointer as libc::c_int > 31 as libc::c_int
            && *input_pointer as libc::c_int != '"' as i32
            && *input_pointer as libc::c_int != '\\' as i32
        {
            *output_pointer = *input_pointer;
        } else {
            let fresh8 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *fresh8 = '\\' as i32 as libc::c_uchar;
            match *input_pointer as libc::c_int {
                92 => {
                    *output_pointer = '\\' as i32 as libc::c_uchar;
                }
                34 => {
                    *output_pointer = '"' as i32 as libc::c_uchar;
                }
                8 => {
                    *output_pointer = 'b' as i32 as libc::c_uchar;
                }
                12 => {
                    *output_pointer = 'f' as i32 as libc::c_uchar;
                }
                10 => {
                    *output_pointer = 'n' as i32 as libc::c_uchar;
                }
                13 => {
                    *output_pointer = 'r' as i32 as libc::c_uchar;
                }
                9 => {
                    *output_pointer = 't' as i32 as libc::c_uchar;
                }
                _ => {
                    sprintf(
                        output_pointer as *mut libc::c_char,
                        b"u%04x\0" as *const u8 as *const libc::c_char,
                        *input_pointer as libc::c_int,
                    );
                    output_pointer = output_pointer.offset(4 as libc::c_int as isize);
                }
            }
        }
        input_pointer = input_pointer.offset(1);
        input_pointer;
        output_pointer = output_pointer.offset(1);
        output_pointer;
    }
    *output
        .offset(
            output_length.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '"' as i32 as libc::c_uchar;
    *output
        .offset(
            output_length.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_uchar;
    return 1 as libc::c_int;
}
unsafe extern "C" fn print_string(
    item: *const cJSON,
    p: *mut printbuffer,
) -> cJSON_bool {
    return print_string_ptr((*item).valuestring as *mut libc::c_uchar, p);
}
unsafe extern "C" fn buffer_skip_whitespace(
    buffer: *mut parse_buffer,
) -> *mut parse_buffer {
    if buffer.is_null() || ((*buffer).content).is_null() {
        return 0 as *mut parse_buffer;
    }
    if !(!buffer.is_null()
        && ((*buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
            < (*buffer).length)
    {
        return buffer;
    }
    while !buffer.is_null()
        && ((*buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
            < (*buffer).length
        && *((*buffer).content)
            .offset((*buffer).offset as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int <= 32 as libc::c_int
    {
        (*buffer).offset = ((*buffer).offset).wrapping_add(1);
        (*buffer).offset;
    }
    if (*buffer).offset == (*buffer).length {
        (*buffer).offset = ((*buffer).offset).wrapping_sub(1);
        (*buffer).offset;
    }
    return buffer;
}
unsafe extern "C" fn skip_utf8_bom(buffer: *mut parse_buffer) -> *mut parse_buffer {
    if buffer.is_null() || ((*buffer).content).is_null()
        || (*buffer).offset != 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut parse_buffer;
    }
    if !buffer.is_null()
        && ((*buffer).offset).wrapping_add(4 as libc::c_int as libc::c_ulong)
            < (*buffer).length
        && strncmp(
            ((*buffer).content).offset((*buffer).offset as isize) as *const libc::c_char,
            b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (*buffer)
            .offset = ((*buffer).offset as libc::c_ulong)
            .wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    return buffer;
}
pub unsafe extern "C" fn cJSON_ParseWithOpts(
    mut value: *const libc::c_char,
    mut return_parse_end: *mut *const libc::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let mut buffer_length: size_t = 0;
    if value.is_null() {
        return 0 as *mut cJSON;
    }
    buffer_length = (strlen(value))
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong);
    return cJSON_ParseWithLengthOpts(
        value,
        buffer_length,
        return_parse_end,
        require_null_terminated,
    );
}
pub unsafe extern "C" fn cJSON_ParseWithLengthOpts(
    mut value: *const libc::c_char,
    mut buffer_length: size_t,
    mut return_parse_end: *mut *const libc::c_char,
    mut require_null_terminated: cJSON_bool,
) -> *mut cJSON {
    let mut current_block: u64;
    let mut buffer: parse_buffer = {
        let mut init = parse_buffer {
            content: 0 as *const libc::c_uchar,
            length: 0 as libc::c_int as size_t,
            offset: 0 as libc::c_int as size_t,
            depth: 0 as libc::c_int as size_t,
            hooks: {
                let mut init = internal_hooks {
                    allocate: None,
                    deallocate: None,
                    reallocate: None,
                };
                init
            },
        };
        init
    };
    let mut item: *mut cJSON = 0 as *mut cJSON;
    global_error.json = 0 as *const libc::c_uchar;
    global_error.position = 0 as libc::c_int as size_t;
    if !(value.is_null() || 0 as libc::c_int as libc::c_ulong == buffer_length) {
        buffer.content = value as *const libc::c_uchar;
        buffer.length = buffer_length;
        buffer.offset = 0 as libc::c_int as size_t;
        buffer.hooks = global_hooks;
        item = cJSON_New_Item(&mut global_hooks);
        if !item.is_null() {
            if !(parse_value(item, buffer_skip_whitespace(skip_utf8_bom(&mut buffer)))
                == 0)
            {
                if require_null_terminated != 0 {
                    buffer_skip_whitespace(&mut buffer);
                    if buffer.offset >= buffer.length
                        || *(buffer.content)
                            .offset(buffer.offset as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            != '\0' as i32
                    {
                        current_block = 4946790526227553966;
                    } else {
                        current_block = 1841672684692190573;
                    }
                } else {
                    current_block = 1841672684692190573;
                }
                match current_block {
                    4946790526227553966 => {}
                    _ => {
                        if !return_parse_end.is_null() {
                            *return_parse_end = (buffer.content)
                                .offset(buffer.offset as isize) as *const libc::c_char;
                        }
                        return item;
                    }
                }
            }
        }
    }
    if !item.is_null() {
        cJSON_Delete(item);
    }
    if !value.is_null() {
        let mut local_error: error = error {
            json: 0 as *const libc::c_uchar,
            position: 0,
        };
        local_error.json = value as *const libc::c_uchar;
        local_error.position = 0 as libc::c_int as size_t;
        if buffer.offset < buffer.length {
            local_error.position = buffer.offset;
        } else if buffer.length > 0 as libc::c_int as libc::c_ulong {
            local_error
                .position = (buffer.length)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        if !return_parse_end.is_null() {
            *return_parse_end = (local_error.json as *const libc::c_char)
                .offset(local_error.position as isize);
        }
        global_error = local_error;
    }
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_Parse(mut value: *const libc::c_char) -> *mut cJSON {
    return cJSON_ParseWithOpts(value, 0 as *mut *const libc::c_char, 0 as libc::c_int);
}
pub unsafe extern "C" fn cJSON_ParseWithLength(
    mut value: *const libc::c_char,
    mut buffer_length: size_t,
) -> *mut cJSON {
    return cJSON_ParseWithLengthOpts(
        value,
        buffer_length,
        0 as *mut *const libc::c_char,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn print(
    item: *const cJSON,
    mut format: cJSON_bool,
    hooks: *const internal_hooks,
) -> *mut libc::c_uchar {
    let mut current_block: u64;
    static mut default_buffer_size: size_t = 256 as libc::c_int as size_t;
    let mut buffer: [printbuffer; 1] = [printbuffer {
        buffer: 0 as *mut libc::c_uchar,
        length: 0,
        offset: 0,
        depth: 0,
        noalloc: 0,
        format: 0,
        hooks: internal_hooks {
            allocate: None,
            deallocate: None,
            reallocate: None,
        },
    }; 1];
    let mut printed: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    memset(
        buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[printbuffer; 1]>() as libc::c_ulong,
    );
    let ref mut fresh9 = (*buffer.as_mut_ptr()).buffer;
    *fresh9 = ((*hooks).allocate).unwrap()(default_buffer_size) as *mut libc::c_uchar;
    (*buffer.as_mut_ptr()).length = default_buffer_size;
    (*buffer.as_mut_ptr()).format = format;
    (*buffer.as_mut_ptr()).hooks = *hooks;
    if !((*buffer.as_mut_ptr()).buffer).is_null() {
        if !(print_value(item, buffer.as_mut_ptr()) == 0) {
            update_offset(buffer.as_mut_ptr());
            if ((*hooks).reallocate).is_some() {
                printed = ((*hooks).reallocate)
                    .unwrap()(
                    (*buffer.as_mut_ptr()).buffer as *mut libc::c_void,
                    ((*buffer.as_mut_ptr()).offset)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_uchar;
                if printed.is_null() {
                    current_block = 2253495734257343714;
                } else {
                    let ref mut fresh10 = (*buffer.as_mut_ptr()).buffer;
                    *fresh10 = 0 as *mut libc::c_uchar;
                    current_block = 1054647088692577877;
                }
            } else {
                printed = ((*hooks).allocate)
                    .unwrap()(
                    ((*buffer.as_mut_ptr()).offset)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_uchar;
                if printed.is_null() {
                    current_block = 2253495734257343714;
                } else {
                    memcpy(
                        printed as *mut libc::c_void,
                        (*buffer.as_mut_ptr()).buffer as *const libc::c_void,
                        if (*buffer.as_mut_ptr()).length
                            < ((*buffer.as_mut_ptr()).offset)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        {
                            (*buffer.as_mut_ptr()).length
                        } else {
                            ((*buffer.as_mut_ptr()).offset)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        },
                    );
                    *printed
                        .offset(
                            (*buffer.as_mut_ptr()).offset as isize,
                        ) = '\0' as i32 as libc::c_uchar;
                    ((*hooks).deallocate)
                        .unwrap()((*buffer.as_mut_ptr()).buffer as *mut libc::c_void);
                    current_block = 1054647088692577877;
                }
            }
            match current_block {
                2253495734257343714 => {}
                _ => return printed,
            }
        }
    }
    if !((*buffer.as_mut_ptr()).buffer).is_null() {
        ((*hooks).deallocate)
            .unwrap()((*buffer.as_mut_ptr()).buffer as *mut libc::c_void);
    }
    if !printed.is_null() {
        ((*hooks).deallocate).unwrap()(printed as *mut libc::c_void);
    }
    return 0 as *mut libc::c_uchar;
}
pub unsafe extern "C" fn cJSON_Print(mut item: *const cJSON) -> *mut libc::c_char {
    return print(item, 1 as libc::c_int, &mut global_hooks) as *mut libc::c_char;
}
pub unsafe extern "C" fn cJSON_PrintUnformatted(
    mut item: *const cJSON,
) -> *mut libc::c_char {
    return print(item, 0 as libc::c_int, &mut global_hooks) as *mut libc::c_char;
}
pub unsafe extern "C" fn cJSON_PrintBuffered(
    mut item: *const cJSON,
    mut prebuffer: libc::c_int,
    mut fmt: cJSON_bool,
) -> *mut libc::c_char {
    let mut p: printbuffer = {
        let mut init = printbuffer {
            buffer: 0 as *mut libc::c_uchar,
            length: 0 as libc::c_int as size_t,
            offset: 0 as libc::c_int as size_t,
            depth: 0 as libc::c_int as size_t,
            noalloc: 0 as libc::c_int,
            format: 0 as libc::c_int,
            hooks: {
                let mut init = internal_hooks {
                    allocate: None,
                    deallocate: None,
                    reallocate: None,
                };
                init
            },
        };
        init
    };
    if prebuffer < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    p
        .buffer = (global_hooks.allocate).unwrap()(prebuffer as size_t)
        as *mut libc::c_uchar;
    if (p.buffer).is_null() {
        return 0 as *mut libc::c_char;
    }
    p.length = prebuffer as size_t;
    p.offset = 0 as libc::c_int as size_t;
    p.noalloc = 0 as libc::c_int;
    p.format = fmt;
    p.hooks = global_hooks;
    if print_value(item, &mut p) == 0 {
        (global_hooks.deallocate).unwrap()(p.buffer as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    return p.buffer as *mut libc::c_char;
}
pub unsafe extern "C" fn cJSON_PrintPreallocated(
    mut item: *mut cJSON,
    mut buffer: *mut libc::c_char,
    length: libc::c_int,
    format: cJSON_bool,
) -> cJSON_bool {
    let mut p: printbuffer = {
        let mut init = printbuffer {
            buffer: 0 as *mut libc::c_uchar,
            length: 0 as libc::c_int as size_t,
            offset: 0 as libc::c_int as size_t,
            depth: 0 as libc::c_int as size_t,
            noalloc: 0 as libc::c_int,
            format: 0 as libc::c_int,
            hooks: {
                let mut init = internal_hooks {
                    allocate: None,
                    deallocate: None,
                    reallocate: None,
                };
                init
            },
        };
        init
    };
    if length < 0 as libc::c_int || buffer.is_null() {
        return 0 as libc::c_int;
    }
    p.buffer = buffer as *mut libc::c_uchar;
    p.length = length as size_t;
    p.offset = 0 as libc::c_int as size_t;
    p.noalloc = 1 as libc::c_int;
    p.format = format;
    p.hooks = global_hooks;
    return print_value(item, &mut p);
}
unsafe extern "C" fn parse_value(
    item: *mut cJSON,
    input_buffer: *mut parse_buffer,
) -> cJSON_bool {
    if input_buffer.is_null() || ((*input_buffer).content).is_null() {
        return 0 as libc::c_int;
    }
    if !input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(4 as libc::c_int as libc::c_ulong)
            <= (*input_buffer).length
        && strncmp(
            ((*input_buffer).content).offset((*input_buffer).offset as isize)
                as *const libc::c_char,
            b"null\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (*item).type_0 = (1 as libc::c_int) << 2 as libc::c_int;
        (*input_buffer)
            .offset = ((*input_buffer).offset as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        return 1 as libc::c_int;
    }
    if !input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(5 as libc::c_int as libc::c_ulong)
            <= (*input_buffer).length
        && strncmp(
            ((*input_buffer).content).offset((*input_buffer).offset as isize)
                as *const libc::c_char,
            b"false\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (*item).type_0 = (1 as libc::c_int) << 0 as libc::c_int;
        (*input_buffer)
            .offset = ((*input_buffer).offset as libc::c_ulong)
            .wrapping_add(5 as libc::c_int as libc::c_ulong) as size_t as size_t;
        return 1 as libc::c_int;
    }
    if !input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(4 as libc::c_int as libc::c_ulong)
            <= (*input_buffer).length
        && strncmp(
            ((*input_buffer).content).offset((*input_buffer).offset as isize)
                as *const libc::c_char,
            b"true\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (*item).type_0 = (1 as libc::c_int) << 1 as libc::c_int;
        (*item).valueint = 1 as libc::c_int;
        (*input_buffer)
            .offset = ((*input_buffer).offset as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        return 1 as libc::c_int;
    }
    if !input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
            < (*input_buffer).length
        && *((*input_buffer).content)
            .offset((*input_buffer).offset as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32
    {
        return parse_string(item, input_buffer);
    }
    if !input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
            < (*input_buffer).length
        && (*((*input_buffer).content)
            .offset((*input_buffer).offset as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            || *((*input_buffer).content)
                .offset((*input_buffer).offset as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *((*input_buffer).content)
                    .offset((*input_buffer).offset as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
    {
        return parse_number(item, input_buffer);
    }
    if !input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
            < (*input_buffer).length
        && *((*input_buffer).content)
            .offset((*input_buffer).offset as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32
    {
        return parse_array(item, input_buffer);
    }
    if !input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
            < (*input_buffer).length
        && *((*input_buffer).content)
            .offset((*input_buffer).offset as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
    {
        return parse_object(item, input_buffer);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_value(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if item.is_null() || output_buffer.is_null() {
        return 0 as libc::c_int;
    }
    match (*item).type_0 & 0xff as libc::c_int {
        4 => {
            output = ensure(output_buffer, 5 as libc::c_int as size_t);
            if output.is_null() {
                return 0 as libc::c_int;
            }
            strcpy(
                output as *mut libc::c_char,
                b"null\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        1 => {
            output = ensure(output_buffer, 6 as libc::c_int as size_t);
            if output.is_null() {
                return 0 as libc::c_int;
            }
            strcpy(
                output as *mut libc::c_char,
                b"false\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        2 => {
            output = ensure(output_buffer, 5 as libc::c_int as size_t);
            if output.is_null() {
                return 0 as libc::c_int;
            }
            strcpy(
                output as *mut libc::c_char,
                b"true\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        8 => return print_number(item, output_buffer),
        128 => {
            let mut raw_length: size_t = 0 as libc::c_int as size_t;
            if ((*item).valuestring).is_null() {
                return 0 as libc::c_int;
            }
            raw_length = (strlen((*item).valuestring))
                .wrapping_add(
                    ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                );
            output = ensure(output_buffer, raw_length);
            if output.is_null() {
                return 0 as libc::c_int;
            }
            memcpy(
                output as *mut libc::c_void,
                (*item).valuestring as *const libc::c_void,
                raw_length,
            );
            return 1 as libc::c_int;
        }
        16 => return print_string(item, output_buffer),
        32 => return print_array(item, output_buffer),
        64 => return print_object(item, output_buffer),
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn parse_array(
    item: *mut cJSON,
    input_buffer: *mut parse_buffer,
) -> cJSON_bool {
    let mut current_block: u64;
    let mut head: *mut cJSON = 0 as *mut cJSON;
    let mut current_item: *mut cJSON = 0 as *mut cJSON;
    if (*input_buffer).depth >= 1000 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*input_buffer).depth = ((*input_buffer).depth).wrapping_add(1);
    (*input_buffer).depth;
    if !(*((*input_buffer).content)
        .offset((*input_buffer).offset as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int != '[' as i32)
    {
        (*input_buffer).offset = ((*input_buffer).offset).wrapping_add(1);
        (*input_buffer).offset;
        buffer_skip_whitespace(input_buffer);
        if !input_buffer.is_null()
            && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
                < (*input_buffer).length
            && *((*input_buffer).content)
                .offset((*input_buffer).offset as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int == ']' as i32
        {
            current_block = 13720440212020310251;
        } else if !(!input_buffer.is_null()
            && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
                < (*input_buffer).length)
        {
            (*input_buffer).offset = ((*input_buffer).offset).wrapping_sub(1);
            (*input_buffer).offset;
            current_block = 18368835354285971245;
        } else {
            (*input_buffer).offset = ((*input_buffer).offset).wrapping_sub(1);
            (*input_buffer).offset;
            loop {
                let mut new_item: *mut cJSON = cJSON_New_Item(
                    &mut (*input_buffer).hooks,
                );
                if new_item.is_null() {
                    current_block = 18368835354285971245;
                    break;
                }
                if head.is_null() {
                    head = new_item;
                    current_item = head;
                } else {
                    (*current_item).next = new_item;
                    (*new_item).prev = current_item;
                    current_item = new_item;
                }
                (*input_buffer).offset = ((*input_buffer).offset).wrapping_add(1);
                (*input_buffer).offset;
                buffer_skip_whitespace(input_buffer);
                if parse_value(current_item, input_buffer) == 0 {
                    current_block = 18368835354285971245;
                    break;
                }
                buffer_skip_whitespace(input_buffer);
                if !(!input_buffer.is_null()
                    && ((*input_buffer).offset)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        < (*input_buffer).length
                    && *((*input_buffer).content)
                        .offset((*input_buffer).offset as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32)
                {
                    current_block = 15089075282327824602;
                    break;
                }
            }
            match current_block {
                18368835354285971245 => {}
                _ => {
                    if !(!input_buffer.is_null()
                        && ((*input_buffer).offset)
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            < (*input_buffer).length)
                        || *((*input_buffer).content)
                            .offset((*input_buffer).offset as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            != ']' as i32
                    {
                        current_block = 18368835354285971245;
                    } else {
                        current_block = 13720440212020310251;
                    }
                }
            }
        }
        match current_block {
            18368835354285971245 => {}
            _ => {
                (*input_buffer).depth = ((*input_buffer).depth).wrapping_sub(1);
                (*input_buffer).depth;
                if !head.is_null() {
                    (*head).prev = current_item;
                }
                (*item).type_0 = (1 as libc::c_int) << 5 as libc::c_int;
                (*item).child = head;
                (*input_buffer).offset = ((*input_buffer).offset).wrapping_add(1);
                (*input_buffer).offset;
                return 1 as libc::c_int;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete(head);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_array(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output_pointer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut current_element: *mut cJSON = (*item).child;
    if output_buffer.is_null() {
        return 0 as libc::c_int;
    }
    output_pointer = ensure(output_buffer, 1 as libc::c_int as size_t);
    if output_pointer.is_null() {
        return 0 as libc::c_int;
    }
    *output_pointer = '[' as i32 as libc::c_uchar;
    (*output_buffer).offset = ((*output_buffer).offset).wrapping_add(1);
    (*output_buffer).offset;
    (*output_buffer).depth = ((*output_buffer).depth).wrapping_add(1);
    (*output_buffer).depth;
    while !current_element.is_null() {
        if print_value(current_element, output_buffer) == 0 {
            return 0 as libc::c_int;
        }
        update_offset(output_buffer);
        if !((*current_element).next).is_null() {
            length = (if (*output_buffer).format != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            }) as size_t;
            output_pointer = ensure(
                output_buffer,
                length.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if output_pointer.is_null() {
                return 0 as libc::c_int;
            }
            let fresh11 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *fresh11 = ',' as i32 as libc::c_uchar;
            if (*output_buffer).format != 0 {
                let fresh12 = output_pointer;
                output_pointer = output_pointer.offset(1);
                *fresh12 = ' ' as i32 as libc::c_uchar;
            }
            *output_pointer = '\0' as i32 as libc::c_uchar;
            (*output_buffer)
                .offset = ((*output_buffer).offset as libc::c_ulong).wrapping_add(length)
                as size_t as size_t;
        }
        current_element = (*current_element).next;
    }
    output_pointer = ensure(output_buffer, 2 as libc::c_int as size_t);
    if output_pointer.is_null() {
        return 0 as libc::c_int;
    }
    let fresh13 = output_pointer;
    output_pointer = output_pointer.offset(1);
    *fresh13 = ']' as i32 as libc::c_uchar;
    *output_pointer = '\0' as i32 as libc::c_uchar;
    (*output_buffer).depth = ((*output_buffer).depth).wrapping_sub(1);
    (*output_buffer).depth;
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse_object(
    item: *mut cJSON,
    input_buffer: *mut parse_buffer,
) -> cJSON_bool {
    let mut current_block: u64;
    let mut head: *mut cJSON = 0 as *mut cJSON;
    let mut current_item: *mut cJSON = 0 as *mut cJSON;
    if (*input_buffer).depth >= 1000 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*input_buffer).depth = ((*input_buffer).depth).wrapping_add(1);
    (*input_buffer).depth;
    if !(!(!input_buffer.is_null()
        && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
            < (*input_buffer).length)
        || *((*input_buffer).content)
            .offset((*input_buffer).offset as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int != '{' as i32)
    {
        (*input_buffer).offset = ((*input_buffer).offset).wrapping_add(1);
        (*input_buffer).offset;
        buffer_skip_whitespace(input_buffer);
        if !input_buffer.is_null()
            && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
                < (*input_buffer).length
            && *((*input_buffer).content)
                .offset((*input_buffer).offset as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32
        {
            current_block = 9971034709416972114;
        } else if !(!input_buffer.is_null()
            && ((*input_buffer).offset).wrapping_add(0 as libc::c_int as libc::c_ulong)
                < (*input_buffer).length)
        {
            (*input_buffer).offset = ((*input_buffer).offset).wrapping_sub(1);
            (*input_buffer).offset;
            current_block = 6193331150973751653;
        } else {
            (*input_buffer).offset = ((*input_buffer).offset).wrapping_sub(1);
            (*input_buffer).offset;
            loop {
                let mut new_item: *mut cJSON = cJSON_New_Item(
                    &mut (*input_buffer).hooks,
                );
                if new_item.is_null() {
                    current_block = 6193331150973751653;
                    break;
                }
                if head.is_null() {
                    head = new_item;
                    current_item = head;
                } else {
                    (*current_item).next = new_item;
                    (*new_item).prev = current_item;
                    current_item = new_item;
                }
                (*input_buffer).offset = ((*input_buffer).offset).wrapping_add(1);
                (*input_buffer).offset;
                buffer_skip_whitespace(input_buffer);
                if parse_string(current_item, input_buffer) == 0 {
                    current_block = 6193331150973751653;
                    break;
                }
                buffer_skip_whitespace(input_buffer);
                (*current_item).string = (*current_item).valuestring;
                (*current_item).valuestring = 0 as *mut libc::c_char;
                if !(!input_buffer.is_null()
                    && ((*input_buffer).offset)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        < (*input_buffer).length)
                    || *((*input_buffer).content)
                        .offset((*input_buffer).offset as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int != ':' as i32
                {
                    current_block = 6193331150973751653;
                    break;
                }
                (*input_buffer).offset = ((*input_buffer).offset).wrapping_add(1);
                (*input_buffer).offset;
                buffer_skip_whitespace(input_buffer);
                if parse_value(current_item, input_buffer) == 0 {
                    current_block = 6193331150973751653;
                    break;
                }
                buffer_skip_whitespace(input_buffer);
                if !(!input_buffer.is_null()
                    && ((*input_buffer).offset)
                        .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        < (*input_buffer).length
                    && *((*input_buffer).content)
                        .offset((*input_buffer).offset as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int == ',' as i32)
                {
                    current_block = 14763689060501151050;
                    break;
                }
            }
            match current_block {
                6193331150973751653 => {}
                _ => {
                    if !(!input_buffer.is_null()
                        && ((*input_buffer).offset)
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            < (*input_buffer).length)
                        || *((*input_buffer).content)
                            .offset((*input_buffer).offset as isize)
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            != '}' as i32
                    {
                        current_block = 6193331150973751653;
                    } else {
                        current_block = 9971034709416972114;
                    }
                }
            }
        }
        match current_block {
            6193331150973751653 => {}
            _ => {
                (*input_buffer).depth = ((*input_buffer).depth).wrapping_sub(1);
                (*input_buffer).depth;
                if !head.is_null() {
                    (*head).prev = current_item;
                }
                (*item).type_0 = (1 as libc::c_int) << 6 as libc::c_int;
                (*item).child = head;
                (*input_buffer).offset = ((*input_buffer).offset).wrapping_add(1);
                (*input_buffer).offset;
                return 1 as libc::c_int;
            }
        }
    }
    if !head.is_null() {
        cJSON_Delete(head);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_object(
    item: *const cJSON,
    output_buffer: *mut printbuffer,
) -> cJSON_bool {
    let mut output_pointer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut current_item: *mut cJSON = (*item).child;
    if output_buffer.is_null() {
        return 0 as libc::c_int;
    }
    length = (if (*output_buffer).format != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    }) as size_t;
    output_pointer = ensure(
        output_buffer,
        length.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if output_pointer.is_null() {
        return 0 as libc::c_int;
    }
    let fresh14 = output_pointer;
    output_pointer = output_pointer.offset(1);
    *fresh14 = '{' as i32 as libc::c_uchar;
    (*output_buffer).depth = ((*output_buffer).depth).wrapping_add(1);
    (*output_buffer).depth;
    if (*output_buffer).format != 0 {
        let fresh15 = output_pointer;
        output_pointer = output_pointer.offset(1);
        *fresh15 = '\n' as i32 as libc::c_uchar;
    }
    (*output_buffer)
        .offset = ((*output_buffer).offset as libc::c_ulong).wrapping_add(length)
        as size_t as size_t;
    while !current_item.is_null() {
        if (*output_buffer).format != 0 {
            let mut i: size_t = 0;
            output_pointer = ensure(output_buffer, (*output_buffer).depth);
            if output_pointer.is_null() {
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int as size_t;
            while i < (*output_buffer).depth {
                let fresh16 = output_pointer;
                output_pointer = output_pointer.offset(1);
                *fresh16 = '\t' as i32 as libc::c_uchar;
                i = i.wrapping_add(1);
                i;
            }
            (*output_buffer)
                .offset = ((*output_buffer).offset as libc::c_ulong)
                .wrapping_add((*output_buffer).depth) as size_t as size_t;
        }
        if print_string_ptr((*current_item).string as *mut libc::c_uchar, output_buffer)
            == 0
        {
            return 0 as libc::c_int;
        }
        update_offset(output_buffer);
        length = (if (*output_buffer).format != 0 {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        }) as size_t;
        output_pointer = ensure(output_buffer, length);
        if output_pointer.is_null() {
            return 0 as libc::c_int;
        }
        let fresh17 = output_pointer;
        output_pointer = output_pointer.offset(1);
        *fresh17 = ':' as i32 as libc::c_uchar;
        if (*output_buffer).format != 0 {
            let fresh18 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *fresh18 = '\t' as i32 as libc::c_uchar;
        }
        (*output_buffer)
            .offset = ((*output_buffer).offset as libc::c_ulong).wrapping_add(length)
            as size_t as size_t;
        if print_value(current_item, output_buffer) == 0 {
            return 0 as libc::c_int;
        }
        update_offset(output_buffer);
        length = ((if (*output_buffer).format != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as size_t)
            .wrapping_add(
                (if !((*current_item).next).is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as size_t,
            );
        output_pointer = ensure(
            output_buffer,
            length.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if output_pointer.is_null() {
            return 0 as libc::c_int;
        }
        if !((*current_item).next).is_null() {
            let fresh19 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *fresh19 = ',' as i32 as libc::c_uchar;
        }
        if (*output_buffer).format != 0 {
            let fresh20 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *fresh20 = '\n' as i32 as libc::c_uchar;
        }
        *output_pointer = '\0' as i32 as libc::c_uchar;
        (*output_buffer)
            .offset = ((*output_buffer).offset as libc::c_ulong).wrapping_add(length)
            as size_t as size_t;
        current_item = (*current_item).next;
    }
    output_pointer = ensure(
        output_buffer,
        if (*output_buffer).format != 0 {
            ((*output_buffer).depth).wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            2 as libc::c_int as libc::c_ulong
        },
    );
    if output_pointer.is_null() {
        return 0 as libc::c_int;
    }
    if (*output_buffer).format != 0 {
        let mut i_0: size_t = 0;
        i_0 = 0 as libc::c_int as size_t;
        while i_0
            < ((*output_buffer).depth).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            let fresh21 = output_pointer;
            output_pointer = output_pointer.offset(1);
            *fresh21 = '\t' as i32 as libc::c_uchar;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    let fresh22 = output_pointer;
    output_pointer = output_pointer.offset(1);
    *fresh22 = '}' as i32 as libc::c_uchar;
    *output_pointer = '\0' as i32 as libc::c_uchar;
    (*output_buffer).depth = ((*output_buffer).depth).wrapping_sub(1);
    (*output_buffer).depth;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn cJSON_GetArraySize(mut array: *const cJSON) -> libc::c_int {
    let mut child: *mut cJSON = 0 as *mut cJSON;
    let mut size: size_t = 0 as libc::c_int as size_t;
    if array.is_null() {
        return 0 as libc::c_int;
    }
    child = (*array).child;
    while !child.is_null() {
        size = size.wrapping_add(1);
        size;
        child = (*child).next;
    }
    return size as libc::c_int;
}
unsafe extern "C" fn get_array_item(
    mut array: *const cJSON,
    mut index: size_t,
) -> *mut cJSON {
    let mut current_child: *mut cJSON = 0 as *mut cJSON;
    if array.is_null() {
        return 0 as *mut cJSON;
    }
    current_child = (*array).child;
    while !current_child.is_null() && index > 0 as libc::c_int as libc::c_ulong {
        index = index.wrapping_sub(1);
        index;
        current_child = (*current_child).next;
    }
    return current_child;
}
pub unsafe extern "C" fn cJSON_GetArrayItem(
    mut array: *const cJSON,
    mut index: libc::c_int,
) -> *mut cJSON {
    if index < 0 as libc::c_int {
        return 0 as *mut cJSON;
    }
    return get_array_item(array, index as size_t);
}
unsafe extern "C" fn get_object_item(
    object: *const cJSON,
    name: *const libc::c_char,
    case_sensitive: cJSON_bool,
) -> *mut cJSON {
    let mut current_element: *mut cJSON = 0 as *mut cJSON;
    if object.is_null() || name.is_null() {
        return 0 as *mut cJSON;
    }
    current_element = (*object).child;
    if case_sensitive != 0 {
        while !current_element.is_null() && !((*current_element).string).is_null()
            && strcmp(name, (*current_element).string) != 0 as libc::c_int
        {
            current_element = (*current_element).next;
        }
    } else {
        while !current_element.is_null()
            && case_insensitive_strcmp(
                name as *const libc::c_uchar,
                (*current_element).string as *const libc::c_uchar,
            ) != 0 as libc::c_int
        {
            current_element = (*current_element).next;
        }
    }
    if current_element.is_null() || ((*current_element).string).is_null() {
        return 0 as *mut cJSON;
    }
    return current_element;
}
pub unsafe extern "C" fn cJSON_GetObjectItem(
    object: *const cJSON,
    string: *const libc::c_char,
) -> *mut cJSON {
    return get_object_item(object, string, 0 as libc::c_int);
}
pub unsafe extern "C" fn cJSON_GetObjectItemCaseSensitive(
    object: *const cJSON,
    string: *const libc::c_char,
) -> *mut cJSON {
    return get_object_item(object, string, 1 as libc::c_int);
}
pub unsafe extern "C" fn cJSON_HasObjectItem(
    mut object: *const cJSON,
    mut string: *const libc::c_char,
) -> cJSON_bool {
    return if !(cJSON_GetObjectItem(object, string)).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn suffix_object(mut prev: *mut cJSON, mut item: *mut cJSON) {
    (*prev).next = item;
    (*item).prev = prev;
}
unsafe extern "C" fn create_reference(
    mut item: *const cJSON,
    hooks: *const internal_hooks,
) -> *mut cJSON {
    let mut reference: *mut cJSON = 0 as *mut cJSON;
    if item.is_null() {
        return 0 as *mut cJSON;
    }
    reference = cJSON_New_Item(hooks);
    if reference.is_null() {
        return 0 as *mut cJSON;
    }
    memcpy(
        reference as *mut libc::c_void,
        item as *const libc::c_void,
        ::std::mem::size_of::<cJSON>() as libc::c_ulong,
    );
    (*reference).string = 0 as *mut libc::c_char;
    (*reference).type_0 |= 256 as libc::c_int;
    (*reference).prev = 0 as *mut cJSON;
    (*reference).next = (*reference).prev;
    return reference;
}
unsafe extern "C" fn add_item_to_array(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    let mut child: *mut cJSON = 0 as *mut cJSON;
    if item.is_null() || array.is_null() || array == item {
        return 0 as libc::c_int;
    }
    child = (*array).child;
    if child.is_null() {
        (*array).child = item;
        (*item).prev = item;
        (*item).next = 0 as *mut cJSON;
    } else if !((*child).prev).is_null() {
        suffix_object((*child).prev, item);
        (*(*array).child).prev = item;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn cJSON_AddItemToArray(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    return add_item_to_array(array, item);
}
unsafe extern "C" fn cast_away_const(
    mut string: *const libc::c_void,
) -> *mut libc::c_void {
    return string as *mut libc::c_void;
}
unsafe extern "C" fn add_item_to_object(
    object: *mut cJSON,
    string: *const libc::c_char,
    item: *mut cJSON,
    hooks: *const internal_hooks,
    constant_key: cJSON_bool,
) -> cJSON_bool {
    let mut new_key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_type: libc::c_int = 0 as libc::c_int;
    if object.is_null() || string.is_null() || item.is_null() || object == item {
        return 0 as libc::c_int;
    }
    if constant_key != 0 {
        new_key = cast_away_const(string as *const libc::c_void) as *mut libc::c_char;
        new_type = (*item).type_0 | 512 as libc::c_int;
    } else {
        new_key = cJSON_strdup(string as *const libc::c_uchar, hooks)
            as *mut libc::c_char;
        if new_key.is_null() {
            return 0 as libc::c_int;
        }
        new_type = (*item).type_0 & !(512 as libc::c_int);
    }
    if (*item).type_0 & 512 as libc::c_int == 0 && !((*item).string).is_null() {
        ((*hooks).deallocate).unwrap()((*item).string as *mut libc::c_void);
    }
    (*item).string = new_key;
    (*item).type_0 = new_type;
    return add_item_to_array(object, item);
}
pub unsafe extern "C" fn cJSON_AddItemToObject(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    return add_item_to_object(object, string, item, &mut global_hooks, 0 as libc::c_int);
}
pub unsafe extern "C" fn cJSON_AddItemToObjectCS(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    return add_item_to_object(object, string, item, &mut global_hooks, 1 as libc::c_int);
}
pub unsafe extern "C" fn cJSON_AddItemReferenceToArray(
    mut array: *mut cJSON,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if array.is_null() {
        return 0 as libc::c_int;
    }
    return add_item_to_array(array, create_reference(item, &mut global_hooks));
}
pub unsafe extern "C" fn cJSON_AddItemReferenceToObject(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
    mut item: *mut cJSON,
) -> cJSON_bool {
    if object.is_null() || string.is_null() {
        return 0 as libc::c_int;
    }
    return add_item_to_object(
        object,
        string,
        create_reference(item, &mut global_hooks),
        &mut global_hooks,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn cJSON_AddNullToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
) -> *mut cJSON {
    let mut null: *mut cJSON = cJSON_CreateNull();
    if add_item_to_object(object, name, null, &mut global_hooks, 0 as libc::c_int) != 0 {
        return null;
    }
    cJSON_Delete(null);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddTrueToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
) -> *mut cJSON {
    let mut true_item: *mut cJSON = cJSON_CreateTrue();
    if add_item_to_object(object, name, true_item, &mut global_hooks, 0 as libc::c_int)
        != 0
    {
        return true_item;
    }
    cJSON_Delete(true_item);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddFalseToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
) -> *mut cJSON {
    let mut false_item: *mut cJSON = cJSON_CreateFalse();
    if add_item_to_object(object, name, false_item, &mut global_hooks, 0 as libc::c_int)
        != 0
    {
        return false_item;
    }
    cJSON_Delete(false_item);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddBoolToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
    boolean: cJSON_bool,
) -> *mut cJSON {
    let mut bool_item: *mut cJSON = cJSON_CreateBool(boolean);
    if add_item_to_object(object, name, bool_item, &mut global_hooks, 0 as libc::c_int)
        != 0
    {
        return bool_item;
    }
    cJSON_Delete(bool_item);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddNumberToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
    number: libc::c_double,
) -> *mut cJSON {
    let mut number_item: *mut cJSON = cJSON_CreateNumber(number);
    if add_item_to_object(object, name, number_item, &mut global_hooks, 0 as libc::c_int)
        != 0
    {
        return number_item;
    }
    cJSON_Delete(number_item);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddStringToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
    string: *const libc::c_char,
) -> *mut cJSON {
    let mut string_item: *mut cJSON = cJSON_CreateString(string);
    if add_item_to_object(object, name, string_item, &mut global_hooks, 0 as libc::c_int)
        != 0
    {
        return string_item;
    }
    cJSON_Delete(string_item);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddRawToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
    raw: *const libc::c_char,
) -> *mut cJSON {
    let mut raw_item: *mut cJSON = cJSON_CreateRaw(raw);
    if add_item_to_object(object, name, raw_item, &mut global_hooks, 0 as libc::c_int)
        != 0
    {
        return raw_item;
    }
    cJSON_Delete(raw_item);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddObjectToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
) -> *mut cJSON {
    let mut object_item: *mut cJSON = cJSON_CreateObject();
    if add_item_to_object(object, name, object_item, &mut global_hooks, 0 as libc::c_int)
        != 0
    {
        return object_item;
    }
    cJSON_Delete(object_item);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_AddArrayToObject(
    object: *mut cJSON,
    name: *const libc::c_char,
) -> *mut cJSON {
    let mut array: *mut cJSON = cJSON_CreateArray();
    if add_item_to_object(object, name, array, &mut global_hooks, 0 as libc::c_int) != 0
    {
        return array;
    }
    cJSON_Delete(array);
    return 0 as *mut cJSON;
}
pub unsafe extern "C" fn cJSON_DetachItemViaPointer(
    mut parent: *mut cJSON,
    item: *mut cJSON,
) -> *mut cJSON {
    if parent.is_null() || item.is_null() {
        return 0 as *mut cJSON;
    }
    if item != (*parent).child {
        (*(*item).prev).next = (*item).next;
    }
    if !((*item).next).is_null() {
        (*(*item).next).prev = (*item).prev;
    }
    if item == (*parent).child {
        (*parent).child = (*item).next;
    } else if ((*item).next).is_null() {
        (*(*parent).child).prev = (*item).prev;
    }
    (*item).prev = 0 as *mut cJSON;
    (*item).next = 0 as *mut cJSON;
    return item;
}
pub unsafe extern "C" fn cJSON_DetachItemFromArray(
    mut array: *mut cJSON,
    mut which: libc::c_int,
) -> *mut cJSON {
    if which < 0 as libc::c_int {
        return 0 as *mut cJSON;
    }
    return cJSON_DetachItemViaPointer(array, get_array_item(array, which as size_t));
}
pub unsafe extern "C" fn cJSON_DeleteItemFromArray(
    mut array: *mut cJSON,
    mut which: libc::c_int,
) {
    cJSON_Delete(cJSON_DetachItemFromArray(array, which));
}
pub unsafe extern "C" fn cJSON_DetachItemFromObject(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
) -> *mut cJSON {
    let mut to_detach: *mut cJSON = cJSON_GetObjectItem(object, string);
    return cJSON_DetachItemViaPointer(object, to_detach);
}
pub unsafe extern "C" fn cJSON_DetachItemFromObjectCaseSensitive(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
) -> *mut cJSON {
    let mut to_detach: *mut cJSON = cJSON_GetObjectItemCaseSensitive(object, string);
    return cJSON_DetachItemViaPointer(object, to_detach);
}
pub unsafe extern "C" fn cJSON_DeleteItemFromObject(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
) {
    cJSON_Delete(cJSON_DetachItemFromObject(object, string));
}
pub unsafe extern "C" fn cJSON_DeleteItemFromObjectCaseSensitive(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
) {
    cJSON_Delete(cJSON_DetachItemFromObjectCaseSensitive(object, string));
}
pub unsafe extern "C" fn cJSON_InsertItemInArray(
    mut array: *mut cJSON,
    mut which: libc::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    let mut after_inserted: *mut cJSON = 0 as *mut cJSON;
    if which < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    after_inserted = get_array_item(array, which as size_t);
    if after_inserted.is_null() {
        return add_item_to_array(array, newitem);
    }
    (*newitem).next = after_inserted;
    (*newitem).prev = (*after_inserted).prev;
    (*after_inserted).prev = newitem;
    if after_inserted == (*array).child {
        (*array).child = newitem;
    } else {
        (*(*newitem).prev).next = newitem;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn cJSON_ReplaceItemViaPointer(
    parent: *mut cJSON,
    item: *mut cJSON,
    mut replacement: *mut cJSON,
) -> cJSON_bool {
    if parent.is_null() || replacement.is_null() || item.is_null() {
        return 0 as libc::c_int;
    }
    if replacement == item {
        return 1 as libc::c_int;
    }
    (*replacement).next = (*item).next;
    (*replacement).prev = (*item).prev;
    if !((*replacement).next).is_null() {
        (*(*replacement).next).prev = replacement;
    }
    if (*parent).child == item {
        if (*(*parent).child).prev == (*parent).child {
            (*replacement).prev = replacement;
        }
        (*parent).child = replacement;
    } else {
        if !((*replacement).prev).is_null() {
            (*(*replacement).prev).next = replacement;
        }
        if ((*replacement).next).is_null() {
            (*(*parent).child).prev = replacement;
        }
    }
    (*item).next = 0 as *mut cJSON;
    (*item).prev = 0 as *mut cJSON;
    cJSON_Delete(item);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn cJSON_ReplaceItemInArray(
    mut array: *mut cJSON,
    mut which: libc::c_int,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    if which < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return cJSON_ReplaceItemViaPointer(
        array,
        get_array_item(array, which as size_t),
        newitem,
    );
}
unsafe extern "C" fn replace_item_in_object(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
    mut replacement: *mut cJSON,
    mut case_sensitive: cJSON_bool,
) -> cJSON_bool {
    if replacement.is_null() || string.is_null() {
        return 0 as libc::c_int;
    }
    if (*replacement).type_0 & 512 as libc::c_int == 0
        && !((*replacement).string).is_null()
    {
        cJSON_free((*replacement).string as *mut libc::c_void);
    }
    (*replacement)
        .string = cJSON_strdup(string as *const libc::c_uchar, &mut global_hooks)
        as *mut libc::c_char;
    if ((*replacement).string).is_null() {
        return 0 as libc::c_int;
    }
    (*replacement).type_0 &= !(512 as libc::c_int);
    return cJSON_ReplaceItemViaPointer(
        object,
        get_object_item(object, string, case_sensitive),
        replacement,
    );
}
pub unsafe extern "C" fn cJSON_ReplaceItemInObject(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    return replace_item_in_object(object, string, newitem, 0 as libc::c_int);
}
pub unsafe extern "C" fn cJSON_ReplaceItemInObjectCaseSensitive(
    mut object: *mut cJSON,
    mut string: *const libc::c_char,
    mut newitem: *mut cJSON,
) -> cJSON_bool {
    return replace_item_in_object(object, string, newitem, 1 as libc::c_int);
}
pub unsafe extern "C" fn cJSON_CreateNull() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 2 as libc::c_int;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateTrue() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 1 as libc::c_int;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateFalse() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 0 as libc::c_int;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateBool(mut boolean: cJSON_bool) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item)
            .type_0 = if boolean != 0 {
            (1 as libc::c_int) << 1 as libc::c_int
        } else {
            (1 as libc::c_int) << 0 as libc::c_int
        };
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateNumber(mut num: libc::c_double) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 3 as libc::c_int;
        (*item).valuedouble = num;
        if num >= 2147483647 as libc::c_int as libc::c_double {
            (*item).valueint = 2147483647 as libc::c_int;
        } else if num
            <= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        {
            (*item).valueint = -(2147483647 as libc::c_int) - 1 as libc::c_int;
        } else {
            (*item).valueint = num as libc::c_int;
        }
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateString(
    mut string: *const libc::c_char,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 4 as libc::c_int;
        (*item)
            .valuestring = cJSON_strdup(
            string as *const libc::c_uchar,
            &mut global_hooks,
        ) as *mut libc::c_char;
        if ((*item).valuestring).is_null() {
            cJSON_Delete(item);
            return 0 as *mut cJSON;
        }
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateStringReference(
    mut string: *const libc::c_char,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 4 as libc::c_int | 256 as libc::c_int;
        (*item)
            .valuestring = cast_away_const(string as *const libc::c_void)
            as *mut libc::c_char;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateObjectReference(
    mut child: *const cJSON,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 6 as libc::c_int | 256 as libc::c_int;
        (*item).child = cast_away_const(child as *const libc::c_void) as *mut cJSON;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateArrayReference(
    mut child: *const cJSON,
) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 5 as libc::c_int | 256 as libc::c_int;
        (*item).child = cast_away_const(child as *const libc::c_void) as *mut cJSON;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateRaw(mut raw: *const libc::c_char) -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 7 as libc::c_int;
        (*item)
            .valuestring = cJSON_strdup(raw as *const libc::c_uchar, &mut global_hooks)
            as *mut libc::c_char;
        if ((*item).valuestring).is_null() {
            cJSON_Delete(item);
            return 0 as *mut cJSON;
        }
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateArray() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 5 as libc::c_int;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateObject() -> *mut cJSON {
    let mut item: *mut cJSON = cJSON_New_Item(&mut global_hooks);
    if !item.is_null() {
        (*item).type_0 = (1 as libc::c_int) << 6 as libc::c_int;
    }
    return item;
}
pub unsafe extern "C" fn cJSON_CreateIntArray(
    mut numbers: *const libc::c_int,
    mut count: libc::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: *mut cJSON = 0 as *mut cJSON;
    let mut p: *mut cJSON = 0 as *mut cJSON;
    let mut a: *mut cJSON = 0 as *mut cJSON;
    if count < 0 as libc::c_int || numbers.is_null() {
        return 0 as *mut cJSON;
    }
    a = cJSON_CreateArray();
    i = 0 as libc::c_int as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateNumber(*numbers.offset(i as isize) as libc::c_double);
        if n.is_null() {
            cJSON_Delete(a);
            return 0 as *mut cJSON;
        }
        if i == 0 {
            (*a).child = n;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
        i;
    }
    if !a.is_null() && !((*a).child).is_null() {
        (*(*a).child).prev = n;
    }
    return a;
}
pub unsafe extern "C" fn cJSON_CreateFloatArray(
    mut numbers: *const libc::c_float,
    mut count: libc::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: *mut cJSON = 0 as *mut cJSON;
    let mut p: *mut cJSON = 0 as *mut cJSON;
    let mut a: *mut cJSON = 0 as *mut cJSON;
    if count < 0 as libc::c_int || numbers.is_null() {
        return 0 as *mut cJSON;
    }
    a = cJSON_CreateArray();
    i = 0 as libc::c_int as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateNumber(*numbers.offset(i as isize) as libc::c_double);
        if n.is_null() {
            cJSON_Delete(a);
            return 0 as *mut cJSON;
        }
        if i == 0 {
            (*a).child = n;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
        i;
    }
    if !a.is_null() && !((*a).child).is_null() {
        (*(*a).child).prev = n;
    }
    return a;
}
pub unsafe extern "C" fn cJSON_CreateDoubleArray(
    mut numbers: *const libc::c_double,
    mut count: libc::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: *mut cJSON = 0 as *mut cJSON;
    let mut p: *mut cJSON = 0 as *mut cJSON;
    let mut a: *mut cJSON = 0 as *mut cJSON;
    if count < 0 as libc::c_int || numbers.is_null() {
        return 0 as *mut cJSON;
    }
    a = cJSON_CreateArray();
    i = 0 as libc::c_int as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateNumber(*numbers.offset(i as isize));
        if n.is_null() {
            cJSON_Delete(a);
            return 0 as *mut cJSON;
        }
        if i == 0 {
            (*a).child = n;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
        i;
    }
    if !a.is_null() && !((*a).child).is_null() {
        (*(*a).child).prev = n;
    }
    return a;
}
pub unsafe extern "C" fn cJSON_CreateStringArray(
    mut strings: *const *const libc::c_char,
    mut count: libc::c_int,
) -> *mut cJSON {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: *mut cJSON = 0 as *mut cJSON;
    let mut p: *mut cJSON = 0 as *mut cJSON;
    let mut a: *mut cJSON = 0 as *mut cJSON;
    if count < 0 as libc::c_int || strings.is_null() {
        return 0 as *mut cJSON;
    }
    a = cJSON_CreateArray();
    i = 0 as libc::c_int as size_t;
    while !a.is_null() && i < count as size_t {
        n = cJSON_CreateString(*strings.offset(i as isize));
        if n.is_null() {
            cJSON_Delete(a);
            return 0 as *mut cJSON;
        }
        if i == 0 {
            (*a).child = n;
        } else {
            suffix_object(p, n);
        }
        p = n;
        i = i.wrapping_add(1);
        i;
    }
    if !a.is_null() && !((*a).child).is_null() {
        (*(*a).child).prev = n;
    }
    return a;
}
pub unsafe extern "C" fn cJSON_Duplicate(
    mut item: *const cJSON,
    mut recurse: cJSON_bool,
) -> *mut cJSON {
    let mut current_block: u64;
    let mut newitem: *mut cJSON = 0 as *mut cJSON;
    let mut child: *mut cJSON = 0 as *mut cJSON;
    let mut next: *mut cJSON = 0 as *mut cJSON;
    let mut newchild: *mut cJSON = 0 as *mut cJSON;
    if !item.is_null() {
        newitem = cJSON_New_Item(&mut global_hooks);
        if !newitem.is_null() {
            (*newitem).type_0 = (*item).type_0 & !(256 as libc::c_int);
            (*newitem).valueint = (*item).valueint;
            (*newitem).valuedouble = (*item).valuedouble;
            if !((*item).valuestring).is_null() {
                (*newitem)
                    .valuestring = cJSON_strdup(
                    (*item).valuestring as *mut libc::c_uchar,
                    &mut global_hooks,
                ) as *mut libc::c_char;
                if ((*newitem).valuestring).is_null() {
                    current_block = 16701096872028934175;
                } else {
                    current_block = 11812396948646013369;
                }
            } else {
                current_block = 11812396948646013369;
            }
            match current_block {
                16701096872028934175 => {}
                _ => {
                    if !((*item).string).is_null() {
                        (*newitem)
                            .string = if (*item).type_0 & 512 as libc::c_int != 0 {
                            (*item).string
                        } else {
                            cJSON_strdup(
                                (*item).string as *mut libc::c_uchar,
                                &mut global_hooks,
                            ) as *mut libc::c_char
                        };
                        if ((*newitem).string).is_null() {
                            current_block = 16701096872028934175;
                        } else {
                            current_block = 12800627514080957624;
                        }
                    } else {
                        current_block = 12800627514080957624;
                    }
                    match current_block {
                        16701096872028934175 => {}
                        _ => {
                            if recurse == 0 {
                                return newitem;
                            }
                            child = (*item).child;
                            loop {
                                if child.is_null() {
                                    current_block = 6669252993407410313;
                                    break;
                                }
                                newchild = cJSON_Duplicate(child, 1 as libc::c_int);
                                if newchild.is_null() {
                                    current_block = 16701096872028934175;
                                    break;
                                }
                                if !next.is_null() {
                                    (*next).next = newchild;
                                    (*newchild).prev = next;
                                    next = newchild;
                                } else {
                                    (*newitem).child = newchild;
                                    next = newchild;
                                }
                                child = (*child).next;
                            }
                            match current_block {
                                16701096872028934175 => {}
                                _ => {
                                    if !newitem.is_null() && !((*newitem).child).is_null() {
                                        (*(*newitem).child).prev = newchild;
                                    }
                                    return newitem;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !newitem.is_null() {
        cJSON_Delete(newitem);
    }
    return 0 as *mut cJSON;
}
unsafe extern "C" fn skip_oneline_comment(mut input: *mut *mut libc::c_char) {
    *input = (*input)
        .offset(
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(
                    ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                ) as isize,
        );
    while *(*input).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        if *(*input).offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
            *input = (*input)
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                        .wrapping_sub(
                            ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                        ) as isize,
                );
            return;
        }
        *input = (*input).offset(1);
        *input;
    }
}
unsafe extern "C" fn skip_multiline_comment(mut input: *mut *mut libc::c_char) {
    *input = (*input)
        .offset(
            (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                .wrapping_sub(
                    ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                ) as isize,
        );
    while *(*input).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        if *(*input).offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
            && *(*input).offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            *input = (*input)
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                        .wrapping_sub(
                            ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                        ) as isize,
                );
            return;
        }
        *input = (*input).offset(1);
        *input;
    }
}
unsafe extern "C" fn minify_string(
    mut input: *mut *mut libc::c_char,
    mut output: *mut *mut libc::c_char,
) {
    *(*output)
        .offset(0 as libc::c_int as isize) = *(*input).offset(0 as libc::c_int as isize);
    *input = (*input)
        .offset(
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                .wrapping_sub(
                    ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                ) as isize,
        );
    *output = (*output)
        .offset(
            (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                .wrapping_sub(
                    ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                ) as isize,
        );
    while *(*input).offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        *(*output)
            .offset(
                0 as libc::c_int as isize,
            ) = *(*input).offset(0 as libc::c_int as isize);
        if *(*input).offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
            *(*output).offset(0 as libc::c_int as isize) = '"' as i32 as libc::c_char;
            *input = (*input)
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                        .wrapping_sub(
                            ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                        ) as isize,
                );
            *output = (*output)
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                        .wrapping_sub(
                            ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                        ) as isize,
                );
            return;
        } else if *(*input).offset(0 as libc::c_int as isize) as libc::c_int
            == '\\' as i32
            && *(*input).offset(1 as libc::c_int as isize) as libc::c_int == '"' as i32
        {
            *(*output)
                .offset(
                    1 as libc::c_int as isize,
                ) = *(*input).offset(1 as libc::c_int as isize);
            *input = (*input)
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                        .wrapping_sub(
                            ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                        ) as isize,
                );
            *output = (*output)
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                        .wrapping_sub(
                            ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong,
                        ) as isize,
                );
        }
        *input = (*input).offset(1);
        *input;
        *output = (*output).offset(1);
        *output;
    }
}
pub unsafe extern "C" fn cJSON_Minify(mut json: *mut libc::c_char) {
    let mut into: *mut libc::c_char = json;
    if json.is_null() {
        return;
    }
    while *json.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        match *json.offset(0 as libc::c_int as isize) as libc::c_int {
            32 | 9 | 13 | 10 => {
                json = json.offset(1);
                json;
            }
            47 => {
                if *json.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    skip_oneline_comment(&mut json);
                } else if *json.offset(1 as libc::c_int as isize) as libc::c_int
                    == '*' as i32
                {
                    skip_multiline_comment(&mut json);
                } else {
                    json = json.offset(1);
                    json;
                }
            }
            34 => {
                minify_string(&mut json, &mut into as *mut *mut libc::c_char);
            }
            _ => {
                *into
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *json.offset(0 as libc::c_int as isize);
                json = json.offset(1);
                json;
                into = into.offset(1);
                into;
            }
        }
    }
    *into = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn cJSON_IsInvalid(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsFalse(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_malloc(mut size: size_t) -> *mut libc::c_void {
    return (global_hooks.allocate).unwrap()(size);
}
pub unsafe extern "C" fn cJSON_Compare(
    a: *const cJSON,
    b: *const cJSON,
    case_sensitive: cJSON_bool,
) -> cJSON_bool {
    if a.is_null() || b.is_null()
        || (*a).type_0 & 0xff as libc::c_int != (*b).type_0 & 0xff as libc::c_int
    {
        return 0 as libc::c_int;
    }
    match (*a).type_0 & 0xff as libc::c_int {
        1 | 2 | 4 | 8 | 16 | 128 | 32 | 64 => {}
        _ => return 0 as libc::c_int,
    }
    if a == b {
        return 1 as libc::c_int;
    }
    match (*a).type_0 & 0xff as libc::c_int {
        1 | 2 | 4 => return 1 as libc::c_int,
        8 => {
            if compare_double((*a).valuedouble, (*b).valuedouble) != 0 {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        16 | 128 => {
            if ((*a).valuestring).is_null() || ((*b).valuestring).is_null() {
                return 0 as libc::c_int;
            }
            if strcmp((*a).valuestring, (*b).valuestring) == 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        32 => {
            let mut a_element: *mut cJSON = (*a).child;
            let mut b_element: *mut cJSON = (*b).child;
            while !a_element.is_null() && !b_element.is_null() {
                if cJSON_Compare(a_element, b_element, case_sensitive) == 0 {
                    return 0 as libc::c_int;
                }
                a_element = (*a_element).next;
                b_element = (*b_element).next;
            }
            if a_element != b_element {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        64 => {
            let mut a_element_0: *mut cJSON = 0 as *mut cJSON;
            let mut b_element_0: *mut cJSON = 0 as *mut cJSON;
            a_element_0 = if !a.is_null() { (*a).child } else { 0 as *mut cJSON };
            while !a_element_0.is_null() {
                b_element_0 = get_object_item(b, (*a_element_0).string, case_sensitive);
                if b_element_0.is_null() {
                    return 0 as libc::c_int;
                }
                if cJSON_Compare(a_element_0, b_element_0, case_sensitive) == 0 {
                    return 0 as libc::c_int;
                }
                a_element_0 = (*a_element_0).next;
            }
            b_element_0 = if !b.is_null() { (*b).child } else { 0 as *mut cJSON };
            while !b_element_0.is_null() {
                a_element_0 = get_object_item(a, (*b_element_0).string, case_sensitive);
                if a_element_0.is_null() {
                    return 0 as libc::c_int;
                }
                if cJSON_Compare(b_element_0, a_element_0, case_sensitive) == 0 {
                    return 0 as libc::c_int;
                }
                b_element_0 = (*b_element_0).next;
            }
            return 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn cJSON_free(mut object: *mut libc::c_void) {
    (global_hooks.deallocate).unwrap()(object);
}
pub unsafe extern "C" fn cJSON_IsRaw(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 7 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 6 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 5 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 2 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0
        & ((1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int) != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 1 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 3 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn cJSON_IsString(item: *const cJSON) -> cJSON_bool {
    if item.is_null() {
        return 0 as libc::c_int;
    }
    return ((*item).type_0 & 0xff as libc::c_int
        == (1 as libc::c_int) << 4 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn _cJsonToPocket(mut vm: *mut PKVM, mut item: *mut cJSON) -> Var {
    match (*item).type_0 & 0xff as libc::c_int {
        4 => {
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0 as libc::c_int as uint64_t;
        }
        1 => {
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000002 as libc::c_long as uint64_t;
        }
        2 => {
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x1000000000003 as libc::c_long as uint64_t;
        }
        8 => return doubleToVar((*item).valuedouble),
        128 | 16 => {
            let mut s: *const libc::c_char = if !((*item).valuestring).is_null() {
                (*item).valuestring as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*(newStringLength
                    as unsafe extern "C" fn(
                        *mut PKVM,
                        *const libc::c_char,
                        uint32_t,
                    ) -> *mut String_0)(
                    vm,
                    s,
                    (if s.is_null() {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        (strlen
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> libc::c_ulong)(s) as uint32_t
                    }),
                ))
                    ._super as *mut Object as uintptr_t;
        }
        32 => {
            let mut list: *mut List = newList(vm, 8 as libc::c_int as uint32_t);
            vmPushTempRef(vm, &mut (*list)._super);
            let mut elem: *mut cJSON = (*item).child;
            while !elem.is_null() {
                let mut v: Var = _cJsonToPocket(vm, elem);
                if v
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                {
                    vmPushTempRef(
                        vm,
                        (v & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object,
                    );
                }
                pkVarBufferWrite(&mut (*list).elements, vm, v);
                if v
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                {
                    vmPopTempRef(vm);
                }
                elem = (*elem).next;
            }
            vmPopTempRef(vm);
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*list)._super as *mut Object as uintptr_t;
        }
        64 => {
            let mut map: *mut Map = newMap(vm);
            vmPushTempRef(vm, &mut (*map)._super);
            let mut elem_0: *mut cJSON = (*item).child;
            while !elem_0.is_null() {
                let mut key: *mut String_0 = newStringLength(
                    vm,
                    (*elem_0).string,
                    if ((*elem_0).string).is_null() {
                        0 as libc::c_int as libc::c_uint
                    } else {
                        strlen((*elem_0).string) as uint32_t
                    },
                );
                vmPushTempRef(vm, &mut (*key)._super);
                let mut value: Var = _cJsonToPocket(vm, elem_0);
                if value
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                {
                    vmPushTempRef(
                        vm,
                        (value & 0xffffffffffff as libc::c_long as uint64_t)
                            as *mut Object,
                    );
                }
                mapSet(
                    vm,
                    map,
                    0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                        | &mut (*key)._super as *mut Object as uintptr_t,
                    value,
                );
                if value
                    & (0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong)
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x8000000000000000 as libc::c_ulong
                {
                    vmPopTempRef(vm);
                }
                vmPopTempRef(vm);
                elem_0 = (*elem_0).next;
            }
            vmPopTempRef(vm);
            return 0x7ffc000000000000 as libc::c_long as uint64_t
                | 0x8000000000000000 as libc::c_ulong
                | &mut (*map)._super as *mut Object as uintptr_t;
        }
        _ => {
            unreachable!();
        }
    };
}
unsafe extern "C" fn _pocketToCJson(mut vm: *mut PKVM, mut item: Var) -> *mut cJSON {
    let mut vt: PkVarType = getVarType(item);
    match vt as libc::c_uint {
        1 => return cJSON_CreateNull(),
        2 => {
            return cJSON_CreateBool(
                (item
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x1000000000003 as libc::c_long as uint64_t) as libc::c_int,
            );
        }
        3 => return cJSON_CreateNumber(varToDouble(item)),
        4 => {
            return cJSON_CreateString(
                ((*((item & 0xffffffffffff as libc::c_long as uint64_t) as *mut Object
                    as *mut String_0))
                    .data)
                    .as_mut_ptr(),
            );
        }
        5 => {
            let mut list: *mut List = (item & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut List;
            let mut arr: *mut cJSON = cJSON_CreateArray();
            let mut err: bool = 0 as libc::c_int != 0;
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*list).elements.count {
                let mut elem: *mut cJSON = _pocketToCJson(
                    vm,
                    *((*list).elements.data).offset(i as isize),
                );
                if elem.is_null() {
                    err = 1 as libc::c_int != 0;
                    break;
                } else {
                    cJSON_AddItemToArray(arr, elem);
                    i = i.wrapping_add(1);
                    i;
                }
            }
            if err {
                cJSON_Delete(arr);
                return 0 as *mut cJSON;
            }
            return arr;
        }
        6 => {
            let mut map: *mut Map = (item & 0xffffffffffff as libc::c_long as uint64_t)
                as *mut Object as *mut Map;
            let mut obj: *mut cJSON = cJSON_CreateObject();
            let mut err_0: bool = 0 as libc::c_int != 0;
            let mut e: *mut MapEntry = (*map).entries;
            while e < ((*map).entries).offset((*map).capacity as isize) {
                if !((*e).key
                    == 0x7ffc000000000000 as libc::c_long as uint64_t
                        | 0x1000000000000 as libc::c_long as uint64_t)
                {
                    if !((*e).key
                        & (0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong)
                        == 0x7ffc000000000000 as libc::c_long as uint64_t
                            | 0x8000000000000000 as libc::c_ulong
                        && (*(((*e).key & 0xffffffffffff as libc::c_long as uint64_t)
                            as *mut Object))
                            .type_0 as libc::c_uint
                            == OBJ_STRING as libc::c_int as libc::c_uint)
                    {
                        pkSetRuntimeErrorFmt(
                            vm,
                            b"Expected string as json object key, instead got type '%s'.\0"
                                as *const u8 as *const libc::c_char,
                            varTypeName((*e).key),
                        );
                        err_0 = 1 as libc::c_int != 0;
                        break;
                    } else {
                        let mut value: *mut cJSON = _pocketToCJson(vm, (*e).value);
                        if value.is_null() {
                            err_0 = 1 as libc::c_int != 0;
                            break;
                        } else {
                            cJSON_AddItemToObject(
                                obj,
                                ((*(((*e).key & 0xffffffffffff as libc::c_long as uint64_t)
                                    as *mut Object as *mut String_0))
                                    .data)
                                    .as_mut_ptr(),
                                value,
                            );
                        }
                    }
                }
                e = e.offset(1);
                e;
            }
            if err_0 {
                cJSON_Delete(obj);
                return 0 as *mut cJSON;
            }
            return obj;
        }
        _ => {
            pkSetRuntimeErrorFmt(
                vm,
                b"Object of type '%s' cannot be serialized to json.\0" as *const u8
                    as *const libc::c_char,
                varTypeName(item),
            );
            return 0 as *mut cJSON;
        }
    };
}
static mut _pk_doc__jsonParse: *const libc::c_char = b"json.parse(json_str:String) -> Var\n\nParse a json string into pocket lang object.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _jsonParse(mut vm: *mut PKVM) {
    let mut string: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut string, 0 as *mut uint32_t) {
        return;
    }
    let mut tree: *mut cJSON = cJSON_Parse(string);
    if tree.is_null() {
        pkSetRuntimeError(
            vm,
            b"Invalid json string\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut obj: Var = _cJsonToPocket(vm, tree);
    cJSON_Delete(tree);
    *((*(*vm).fiber).ret).offset(0 as libc::c_int as isize) = obj;
}
static mut _pk_doc__jsonPrint: *const libc::c_char = b"json.print(value:Var, pretty:Bool=false)\n\nRender a pocketlang value into text. Takes an optional argument pretty, if true it'll pretty print the output.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _jsonPrint(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = pkGetArgc(vm);
    if !pkCheckArgcRange(vm, argc, 1 as libc::c_int, 2 as libc::c_int) {
        return;
    }
    let mut pretty: bool = 0 as libc::c_int != 0;
    if argc == 2 as libc::c_int {
        if !pkValidateSlotBool(vm, 2 as libc::c_int, &mut pretty) {
            return;
        }
    }
    let mut value: Var = *((*(*vm).fiber).ret).offset(1 as libc::c_int as isize);
    let mut json: *mut cJSON = _pocketToCJson(vm, value);
    if json.is_null() {
        return;
    }
    let mut string: *mut libc::c_char = if pretty as libc::c_int != 0 {
        cJSON_Print(json)
    } else {
        cJSON_PrintUnformatted(json)
    };
    cJSON_Delete(json);
    if string.is_null() {
        pkSetRuntimeError(
            vm,
            b"Failed to print json.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    pkSetSlotString(vm, 0 as libc::c_int, string);
    free(string as *mut libc::c_void);
}
pub unsafe extern "C" fn registerModuleJson(mut vm: *mut PKVM) {
    let mut json: *mut PkHandle = pkNewModule(
        vm,
        b"json\0" as *const u8 as *const libc::c_char,
    );
    pkModuleAddFunction(
        vm,
        json,
        b"parse\0" as *const u8 as *const libc::c_char,
        Some(_jsonParse as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__jsonParse,
    );
    pkModuleAddFunction(
        vm,
        json,
        b"print\0" as *const u8 as *const libc::c_char,
        Some(_jsonPrint as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__jsonPrint,
    );
    pkRegisterModule(vm, json);
    pkReleaseHandle(vm, json);
}
