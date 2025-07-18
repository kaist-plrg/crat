use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type cpBool = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArray {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub arr: *mut *mut libc::c_void,
}
pub unsafe extern "C" fn cpArrayNew(mut size: libc::c_int) -> *mut cpArray {
    let mut arr: *mut cpArray = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpArray>() as libc::c_ulong,
    ) as *mut cpArray;
    (*arr).num = 0 as libc::c_int;
    (*arr).max = if size != 0 { size } else { 4 as libc::c_int };
    (*arr)
        .arr = calloc(
        (*arr).max as libc::c_ulong,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    ) as *mut *mut libc::c_void;
    return arr;
}
pub unsafe extern "C" fn cpArrayFree(mut arr: *mut cpArray) {
    if !arr.is_null() {
        free((*arr).arr as *mut libc::c_void);
        (*arr).arr = 0 as *mut *mut libc::c_void;
        free(arr as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpArrayPush(
    mut arr: *mut cpArray,
    mut object: *mut libc::c_void,
) {
    if (*arr).num == (*arr).max {
        (*arr)
            .max = 3 as libc::c_int * ((*arr).max + 1 as libc::c_int) / 2 as libc::c_int;
        (*arr)
            .arr = realloc(
            (*arr).arr as *mut libc::c_void,
            ((*arr).max as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
    }
    let ref mut fresh0 = *((*arr).arr).offset((*arr).num as isize);
    *fresh0 = object;
    (*arr).num += 1;
    (*arr).num;
}
pub unsafe extern "C" fn cpArrayPop(mut arr: *mut cpArray) -> *mut libc::c_void {
    (*arr).num -= 1;
    (*arr).num;
    let mut value: *mut libc::c_void = *((*arr).arr).offset((*arr).num as isize);
    let ref mut fresh1 = *((*arr).arr).offset((*arr).num as isize);
    *fresh1 = 0 as *mut libc::c_void;
    return value;
}
pub unsafe extern "C" fn cpArrayDeleteObj(
    mut arr: *mut cpArray,
    mut obj: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arr).num {
        if *((*arr).arr).offset(i as isize) == obj {
            (*arr).num -= 1;
            (*arr).num;
            let ref mut fresh2 = *((*arr).arr).offset(i as isize);
            *fresh2 = *((*arr).arr).offset((*arr).num as isize);
            let ref mut fresh3 = *((*arr).arr).offset((*arr).num as isize);
            *fresh3 = 0 as *mut libc::c_void;
            return;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn cpArrayFreeEach(
    mut arr: *mut cpArray,
    mut freeFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arr).num {
        freeFunc.unwrap()(*((*arr).arr).offset(i as isize));
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn cpArrayContains(
    mut arr: *mut cpArray,
    mut ptr: *mut libc::c_void,
) -> cpBool {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*arr).num {
        if *((*arr).arr).offset(i as isize) == ptr {
            return 1 as libc::c_int as cpBool;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as cpBool;
}
