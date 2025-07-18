use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn snoopy_genericregistry_getCount(
    mut regArray: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while strcmp(*regArray.offset(i as isize), b"\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        i += 1;
        i;
    }
    return i;
}
pub unsafe extern "C" fn snoopy_genericregistry_doesIdExist(
    mut regArray: *mut *mut libc::c_char,
    mut itemId: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int <= itemId && itemId < snoopy_genericregistry_getCount(regArray) {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn snoopy_genericregistry_doesNameExist(
    mut regArray: *mut *mut libc::c_char,
    itemName: *const libc::c_char,
) -> libc::c_int {
    if snoopy_genericregistry_getIdFromName(regArray, itemName) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn snoopy_genericregistry_getIdFromName(
    mut regArray: *mut *mut libc::c_char,
    itemName: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while 0 as libc::c_int
        != strcmp(
            *regArray.offset(i as isize),
            b"\0" as *const u8 as *const libc::c_char,
        )
    {
        if strcmp(*regArray.offset(i as isize), itemName) == 0 as libc::c_int {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn snoopy_genericregistry_getName(
    mut regArray: *mut *mut libc::c_char,
    mut itemId: libc::c_int,
) -> *mut libc::c_char {
    if snoopy_genericregistry_doesIdExist(regArray, itemId) != 0 {
        return *regArray.offset(itemId as isize);
    }
    return 0 as *mut libc::c_char;
}
