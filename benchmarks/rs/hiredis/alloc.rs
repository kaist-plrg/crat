use ::libc;
extern "C" {
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hiredisAllocFuncs {
    pub mallocFn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub callocFn: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub reallocFn: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub strdupFn: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    >,
    pub freeFn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub static mut hiredisAllocFns: hiredisAllocFuncs = unsafe {
    {
        let mut init = hiredisAllocFuncs {
            mallocFn: Some(
                malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
            ),
            callocFn: Some(
                calloc
                    as unsafe extern "C" fn(
                        libc::c_ulong,
                        libc::c_ulong,
                    ) -> *mut libc::c_void,
            ),
            reallocFn: Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_ulong,
                    ) -> *mut libc::c_void,
            ),
            strdupFn: Some(
                strdup as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            ),
            freeFn: Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    }
};
pub unsafe extern "C" fn hiredisSetAllocators(
    mut override_0: *mut hiredisAllocFuncs,
) -> hiredisAllocFuncs {
    let mut orig: hiredisAllocFuncs = hiredisAllocFns;
    hiredisAllocFns = *override_0;
    return orig;
}
pub unsafe extern "C" fn hiredisResetAllocators() {
    hiredisAllocFns = {
        let mut init = hiredisAllocFuncs {
            mallocFn: Some(
                malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
            ),
            callocFn: Some(
                calloc
                    as unsafe extern "C" fn(
                        libc::c_ulong,
                        libc::c_ulong,
                    ) -> *mut libc::c_void,
            ),
            reallocFn: Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_ulong,
                    ) -> *mut libc::c_void,
            ),
            strdupFn: Some(
                strdup as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            ),
            freeFn: Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        };
        init
    };
}
