use ::libc;
extern "C" {
    fn backupfile_internal(
        _: libc::c_int,
        _: *const libc::c_char,
        _: backup_type,
        _: bool,
    ) -> *mut libc::c_char;
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
    ) -> ptrdiff_t;
    fn xalloc_die();
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
pub type ptrdiff_t = libc::c_long;
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub unsafe extern "C" fn find_backup_file_name(
    mut dir_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut backup_type: backup_type,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = backupfile_internal(
        dir_fd,
        file,
        backup_type,
        0 as libc::c_int != 0,
    );
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
static mut backup_args: [*const libc::c_char; 9] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"off\0" as *const u8 as *const libc::c_char,
    b"simple\0" as *const u8 as *const libc::c_char,
    b"never\0" as *const u8 as *const libc::c_char,
    b"existing\0" as *const u8 as *const libc::c_char,
    b"nil\0" as *const u8 as *const libc::c_char,
    b"numbered\0" as *const u8 as *const libc::c_char,
    b"t\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut backup_types: [backup_type; 8] = [
    no_backups,
    no_backups,
    simple_backups,
    simple_backups,
    numbered_existing_backups,
    numbered_existing_backups,
    numbered_backups,
    numbered_backups,
];
pub unsafe extern "C" fn get_version(
    mut context: *const libc::c_char,
    mut version: *const libc::c_char,
) -> backup_type {
    if version.is_null() || *version as libc::c_int == 0 as libc::c_int {
        return numbered_existing_backups
    } else {
        return backup_types[__xargmatch_internal(
            context,
            version,
            backup_args.as_ptr(),
            backup_types.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<backup_type>() as libc::c_ulong,
            argmatch_die,
        ) as usize]
    };
}
pub unsafe extern "C" fn xget_version(
    mut context: *const libc::c_char,
    mut version: *const libc::c_char,
) -> backup_type {
    if !version.is_null() && *version as libc::c_int != 0 {
        return get_version(context, version)
    } else {
        return get_version(
            b"$VERSION_CONTROL\0" as *const u8 as *const libc::c_char,
            getenv(b"VERSION_CONTROL\0" as *const u8 as *const libc::c_char),
        )
    };
}
