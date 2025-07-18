use ::libc;
extern "C" {
    pub type PKVM;
    pub type PkHandle;
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
    fn pkGetSelf(vm: *const PKVM) -> *mut libc::c_void;
    fn pkSetSlotNumber(vm: *mut PKVM, index: libc::c_int, value: libc::c_double);
    fn pkValidateSlotNumber(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut libc::c_double,
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
    fn pkReserveSlots(vm: *mut PKVM, count: libc::c_int);
    fn pkGetSlotString(
        vm: *mut PKVM,
        index: libc::c_int,
        length: *mut uint32_t,
    ) -> *const libc::c_char;
    fn pkGetSlotNativeInstance(vm: *mut PKVM, index: libc::c_int) -> *mut libc::c_void;
    fn pkSetSlotBool(vm: *mut PKVM, index: libc::c_int, value: bool);
    fn pkSetSlotString(vm: *mut PKVM, index: libc::c_int, value: *const libc::c_char);
    fn pkSetSlotStringFmt(
        vm: *mut PKVM,
        index: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn pkPlaceSelf(vm: *mut PKVM, index: libc::c_int);
    fn pkGetClass(vm: *mut PKVM, instance: libc::c_int, index: libc::c_int);
    fn pkNewInstance(
        vm: *mut PKVM,
        cls: libc::c_int,
        index: libc::c_int,
        argc: libc::c_int,
        argv: libc::c_int,
    ) -> bool;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
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
pub type pkNativeFn = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
pub type pkNewInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void,
>;
pub type pkDeleteInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dummy {
    pub val: libc::c_double,
}
pub unsafe extern "C" fn _newDummy(mut vm: *mut PKVM) -> *mut libc::c_void {
    let mut dummy: *mut Dummy = pkRealloc(
        vm,
        0 as *mut libc::c_void,
        ::std::mem::size_of::<Dummy>() as libc::c_ulong,
    ) as *mut Dummy;
    (*dummy).val = 0 as libc::c_int as libc::c_double;
    return dummy as *mut libc::c_void;
}
pub unsafe extern "C" fn _deleteDummy(mut vm: *mut PKVM, mut ptr: *mut libc::c_void) {
    pkRealloc(vm, ptr, 0 as libc::c_int as size_t);
}
static mut _pk_doc__dummyInit: *const libc::c_char = b"dummy.Dummy._init(n:Number)\n\nInitialize a dummy instance with [n].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyInit(mut vm: *mut PKVM) {
    let mut val: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut val) {
        return;
    }
    let mut self_0: *mut Dummy = pkGetSelf(vm) as *mut Dummy;
    (*self_0).val = val;
}
static mut _pk_doc__dummyGetter: *const libc::c_char = b"dummy.Dummy.@getter()\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyGetter(mut vm: *mut PKVM) {
    let mut name: *const libc::c_char = pkGetSlotString(
        vm,
        1 as libc::c_int,
        0 as *mut uint32_t,
    );
    let mut self_0: *mut Dummy = pkGetSelf(vm) as *mut Dummy;
    if strcmp(b"val\0" as *const u8 as *const libc::c_char, name) == 0 as libc::c_int {
        pkSetSlotNumber(vm, 0 as libc::c_int, (*self_0).val);
        return;
    }
}
unsafe extern "C" fn _dummySetter(mut vm: *mut PKVM) {
    let mut name: *const libc::c_char = pkGetSlotString(
        vm,
        1 as libc::c_int,
        0 as *mut uint32_t,
    );
    let mut self_0: *mut Dummy = pkGetSelf(vm) as *mut Dummy;
    if strcmp(b"val\0" as *const u8 as *const libc::c_char, name) == 0 as libc::c_int {
        let mut val: libc::c_double = 0.;
        if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut val) {
            return;
        }
        (*self_0).val = val;
        return;
    }
}
static mut _pk_doc__dummySetter: *const libc::c_char = b"dummy.Dummy.@setter()\n\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyAdd(mut vm: *mut PKVM) {
    let mut self_0: *mut Dummy = pkGetSelf(vm) as *mut Dummy;
    pkReserveSlots(vm, 4 as libc::c_int);
    pkPlaceSelf(vm, 2 as libc::c_int);
    pkGetClass(vm, 2 as libc::c_int, 2 as libc::c_int);
    if !pkValidateSlotInstanceOf(vm, 1 as libc::c_int, 2 as libc::c_int) {
        return;
    }
    let mut other: *mut Dummy = pkGetSlotNativeInstance(vm, 1 as libc::c_int)
        as *mut Dummy;
    pkSetSlotNumber(vm, 3 as libc::c_int, (*self_0).val + (*other).val);
    if !pkNewInstance(
        vm,
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ) {
        return;
    }
}
static mut _pk_doc__dummyAdd: *const libc::c_char = b"dummy.Dummy.+(other:dummy.Dummy) -> dummy.Dummy\n\nAdds two dummy instances.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__dummyEq: *const libc::c_char = b"dummy.Dummy.==(other:dummy.Dummy) -> Bool\n\nCheck if two dummy instances are the equal.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyEq(mut vm: *mut PKVM) {
    let mut value: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut value) {
        return;
    }
    let mut self_0: *mut Dummy = pkGetSelf(vm) as *mut Dummy;
    pkSetSlotBool(vm, 0 as libc::c_int, value == (*self_0).val);
}
static mut _pk_doc__dummyGt: *const libc::c_char = b"dummy.Dummy.>(other:dummy.Dummy) -> Bool\n\nCheck if the dummy instance is greater than [other].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyGt(mut vm: *mut PKVM) {
    let mut value: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut value) {
        return;
    }
    let mut self_0: *mut Dummy = pkGetSelf(vm) as *mut Dummy;
    pkSetSlotBool(vm, 0 as libc::c_int, (*self_0).val > value);
}
static mut _pk_doc__dummyMethod: *const libc::c_char = b"Dummy.a_method(n1:Number, n2:Number) -> Number\n\nA dummy method to check dummy method calls. Will take 2 number arguments and return the multiplication.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyMethod(mut vm: *mut PKVM) {
    let mut n1: libc::c_double = 0.;
    let mut n2: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut n1) {
        return;
    }
    if !pkValidateSlotNumber(vm, 2 as libc::c_int, &mut n2) {
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, n1 * n2);
}
static mut _pk_doc__dummyFunction: *const libc::c_char = b"dummy.afunc(s1:String, s2:String) -> String\n\nA dummy function the'll return s2 + s1.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyFunction(mut vm: *mut PKVM) {
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut s1, 0 as *mut uint32_t) {
        return;
    }
    if !pkValidateSlotString(vm, 2 as libc::c_int, &mut s2, 0 as *mut uint32_t) {
        return;
    }
    pkSetSlotStringFmt(
        vm,
        0 as libc::c_int,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        s2,
        s1,
    );
}
static mut _pk_doc__dummyCallNative: *const libc::c_char = b"dummy.call_native(fn:Closure) -> Null\n\nCalls the function 'fn' with arguments 'foo', 42, false.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyCallNative(mut vm: *mut PKVM) {
    if !pkValidateSlotType(vm, 1 as libc::c_int, PK_CLOSURE) {
        return;
    }
    pkReserveSlots(vm, 5 as libc::c_int);
    pkSetSlotString(vm, 2 as libc::c_int, b"foo\0" as *const u8 as *const libc::c_char);
    pkSetSlotNumber(vm, 3 as libc::c_int, 42 as libc::c_int as libc::c_double);
    pkSetSlotBool(vm, 4 as libc::c_int, 0 as libc::c_int != 0);
    if !pkCallFunction(
        vm,
        1 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
    ) {
        return;
    }
}
static mut _pk_doc__dummyCallMethod: *const libc::c_char = b"dummy.call_method(o:Var, method:String, a1:Var, a2:Var) -> Var\n\nCalls the method int the object [o] with two arguments [a1] and [a2].\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _dummyCallMethod(mut vm: *mut PKVM) {
    let mut method: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 2 as libc::c_int, &mut method, 0 as *mut uint32_t) {
        return;
    }
    if !pkCallMethod(
        vm,
        1 as libc::c_int,
        method,
        2 as libc::c_int,
        3 as libc::c_int,
        0 as libc::c_int,
    ) {
        return;
    }
}
pub unsafe extern "C" fn registerModuleDummy(mut vm: *mut PKVM) {
    let mut dummy: *mut PkHandle = pkNewModule(
        vm,
        b"dummy\0" as *const u8 as *const libc::c_char,
    );
    pkModuleAddFunction(
        vm,
        dummy,
        b"afunc\0" as *const u8 as *const libc::c_char,
        Some(_dummyFunction as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__dummyFunction,
    );
    pkModuleAddFunction(
        vm,
        dummy,
        b"call_native\0" as *const u8 as *const libc::c_char,
        Some(_dummyCallNative as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__dummyCallNative,
    );
    pkModuleAddFunction(
        vm,
        dummy,
        b"call_method\0" as *const u8 as *const libc::c_char,
        Some(_dummyCallMethod as unsafe extern "C" fn(*mut PKVM) -> ()),
        4 as libc::c_int,
        _pk_doc__dummyCallMethod,
    );
    let mut cls_dummy: *mut PkHandle = pkNewClass(
        vm,
        b"Dummy\0" as *const u8 as *const libc::c_char,
        0 as *mut PkHandle,
        dummy,
        Some(_newDummy as unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void),
        Some(_deleteDummy as unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> ()),
        0 as *const libc::c_char,
    );
    pkClassAddMethod(
        vm,
        cls_dummy,
        b"_init\0" as *const u8 as *const libc::c_char,
        Some(_dummyInit as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__dummyInit,
    );
    pkClassAddMethod(
        vm,
        cls_dummy,
        b"@getter\0" as *const u8 as *const libc::c_char,
        Some(_dummyGetter as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__dummyGetter,
    );
    pkClassAddMethod(
        vm,
        cls_dummy,
        b"@setter\0" as *const u8 as *const libc::c_char,
        Some(_dummySetter as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__dummySetter,
    );
    pkClassAddMethod(
        vm,
        cls_dummy,
        b"+\0" as *const u8 as *const libc::c_char,
        Some(_dummyAdd as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__dummyAdd,
    );
    pkClassAddMethod(
        vm,
        cls_dummy,
        b"==\0" as *const u8 as *const libc::c_char,
        Some(_dummyEq as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__dummyEq,
    );
    pkClassAddMethod(
        vm,
        cls_dummy,
        b">\0" as *const u8 as *const libc::c_char,
        Some(_dummyGt as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__dummyGt,
    );
    pkClassAddMethod(
        vm,
        cls_dummy,
        b"a_method\0" as *const u8 as *const libc::c_char,
        Some(_dummyMethod as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__dummyMethod,
    );
    pkReleaseHandle(vm, cls_dummy);
    pkRegisterModule(vm, dummy);
    pkReleaseHandle(vm, dummy);
}
