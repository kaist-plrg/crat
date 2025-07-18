use ::libc;
extern "C" {
    fn snoopy_genericregistry_getCount(regArray: *mut *mut libc::c_char) -> libc::c_int;
    fn snoopy_genericregistry_doesIdExist(
        regArray: *mut *mut libc::c_char,
        itemId: libc::c_int,
    ) -> libc::c_int;
    fn snoopy_genericregistry_doesNameExist(
        regArray: *mut *mut libc::c_char,
        itemName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_genericregistry_getIdFromName(
        regArray: *mut *mut libc::c_char,
        itemName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_genericregistry_getName(
        regArray: *mut *mut libc::c_char,
        itemId: libc::c_int,
    ) -> *mut libc::c_char;
    fn snoopy_filter_exclude_spawns_of(arg: *const libc::c_char) -> libc::c_int;
    fn snoopy_filter_exclude_uid(arg: *const libc::c_char) -> libc::c_int;
    fn snoopy_filter_only_root(arg: *const libc::c_char) -> libc::c_int;
    fn snoopy_filter_only_tty(arg: *const libc::c_char) -> libc::c_int;
    fn snoopy_filter_only_uid(arg: *const libc::c_char) -> libc::c_int;
    fn snoopy_filter_noop(arg: *const libc::c_char) -> libc::c_int;
}
pub static mut snoopy_filterregistry_names: [*mut libc::c_char; 7] = [
    b"exclude_spawns_of\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"exclude_uid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"only_root\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"only_tty\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"only_uid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"noop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut snoopy_filterregistry_ptrs: [Option::<
    unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
>; 6] = unsafe {
    [
        Some(
            snoopy_filter_exclude_spawns_of
                as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
        Some(
            snoopy_filter_exclude_uid
                as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
        Some(
            snoopy_filter_only_root
                as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
        Some(
            snoopy_filter_only_tty
                as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
        Some(
            snoopy_filter_only_uid
                as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
        Some(
            snoopy_filter_noop
                as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
        ),
    ]
};
pub unsafe extern "C" fn snoopy_filterregistry_getCount() -> libc::c_int {
    return snoopy_genericregistry_getCount(snoopy_filterregistry_names.as_mut_ptr());
}
pub unsafe extern "C" fn snoopy_filterregistry_doesIdExist(
    mut filterId: libc::c_int,
) -> libc::c_int {
    return snoopy_genericregistry_doesIdExist(
        snoopy_filterregistry_names.as_mut_ptr(),
        filterId,
    );
}
pub unsafe extern "C" fn snoopy_filterregistry_doesNameExist(
    filterName: *const libc::c_char,
) -> libc::c_int {
    return snoopy_genericregistry_doesNameExist(
        snoopy_filterregistry_names.as_mut_ptr(),
        filterName,
    );
}
pub unsafe extern "C" fn snoopy_filterregistry_getIdFromName(
    filterName: *const libc::c_char,
) -> libc::c_int {
    return snoopy_genericregistry_getIdFromName(
        snoopy_filterregistry_names.as_mut_ptr(),
        filterName,
    );
}
pub unsafe extern "C" fn snoopy_filterregistry_getName(
    mut filterId: libc::c_int,
) -> *mut libc::c_char {
    return snoopy_genericregistry_getName(
        snoopy_filterregistry_names.as_mut_ptr(),
        filterId,
    );
}
pub unsafe extern "C" fn snoopy_filterregistry_callById(
    mut filterId: libc::c_int,
    filterArg: *const libc::c_char,
) -> libc::c_int {
    if 0 as libc::c_int == snoopy_filterregistry_doesIdExist(filterId) {
        return -(1 as libc::c_int);
    }
    return (snoopy_filterregistry_ptrs[filterId as usize]).unwrap()(filterArg);
}
pub unsafe extern "C" fn snoopy_filterregistry_callByName(
    filterName: *const libc::c_char,
    filterArg: *const libc::c_char,
) -> libc::c_int {
    let mut filterId: libc::c_int = 0;
    filterId = snoopy_filterregistry_getIdFromName(filterName);
    if filterId == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return (snoopy_filterregistry_ptrs[filterId as usize]).unwrap()(filterArg);
}
