use ::libc;
extern "C" {
    pub type PKVM;
    pub type PkHandle;
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn pkValidateSlotNumber(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut libc::c_double,
    ) -> bool;
    fn pkSetSlotNumber(vm: *mut PKVM, index: libc::c_int, value: libc::c_double);
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
pub type pkNativeFn = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
pub type useconds_t = __useconds_t;
static mut _pk_doc__timeEpoch: *const libc::c_char = b"time() -> Number\n\nReturns the number of seconds since the Epoch, 1970-01-01 00:00:00 +0000 (UTC).\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _timeEpoch(mut vm: *mut PKVM) {
    pkSetSlotNumber(vm, 0 as libc::c_int, time(0 as *mut time_t) as libc::c_double);
}
static mut _pk_doc__timeClock: *const libc::c_char = b"clock() -> Number\n\nReturns the number of clocks passed divied by CLOCKS_PER_SEC.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _timeClock(mut vm: *mut PKVM) {
    pkSetSlotNumber(
        vm,
        0 as libc::c_int,
        clock() as libc::c_double / 1000000 as libc::c_int as __clock_t as libc::c_double,
    );
}
static mut _pk_doc__timeSleep: *const libc::c_char = b"sleep(t:num) -> Number\n\nSleep for [t] milliseconds.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _timeSleep(mut vm: *mut PKVM) {
    let mut t: libc::c_double = 0.;
    pkValidateSlotNumber(vm, 1 as libc::c_int, &mut t);
    usleep((t as useconds_t).wrapping_mul(1000 as libc::c_int as libc::c_uint));
}
pub unsafe extern "C" fn registerModuleTime(mut vm: *mut PKVM) {
    let mut time_0: *mut PkHandle = pkNewModule(
        vm,
        b"time\0" as *const u8 as *const libc::c_char,
    );
    pkModuleAddFunction(
        vm,
        time_0,
        b"epoch\0" as *const u8 as *const libc::c_char,
        Some(_timeEpoch as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__timeEpoch,
    );
    pkModuleAddFunction(
        vm,
        time_0,
        b"sleep\0" as *const u8 as *const libc::c_char,
        Some(_timeSleep as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__timeSleep,
    );
    pkModuleAddFunction(
        vm,
        time_0,
        b"clock\0" as *const u8 as *const libc::c_char,
        Some(_timeClock as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__timeClock,
    );
    pkRegisterModule(vm, time_0);
    pkReleaseHandle(vm, time_0);
}
