use ::libc;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
    ) -> ptrdiff_t;
    static mut argmatch_die: argmatch_exit_fn;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
static mut warning_args: [*const libc::c_char; 26] = [
    b"all\0" as *const u8 as *const libc::c_char,
    b"alone-zero-block\0" as *const u8 as *const libc::c_char,
    b"bad-dumpdir\0" as *const u8 as *const libc::c_char,
    b"cachedir\0" as *const u8 as *const libc::c_char,
    b"contiguous-cast\0" as *const u8 as *const libc::c_char,
    b"file-changed\0" as *const u8 as *const libc::c_char,
    b"file-ignored\0" as *const u8 as *const libc::c_char,
    b"file-removed\0" as *const u8 as *const libc::c_char,
    b"file-shrank\0" as *const u8 as *const libc::c_char,
    b"file-unchanged\0" as *const u8 as *const libc::c_char,
    b"filename-with-nuls\0" as *const u8 as *const libc::c_char,
    b"ignore-archive\0" as *const u8 as *const libc::c_char,
    b"ignore-newer\0" as *const u8 as *const libc::c_char,
    b"new-directory\0" as *const u8 as *const libc::c_char,
    b"rename-directory\0" as *const u8 as *const libc::c_char,
    b"symlink-cast\0" as *const u8 as *const libc::c_char,
    b"timestamp\0" as *const u8 as *const libc::c_char,
    b"unknown-cast\0" as *const u8 as *const libc::c_char,
    b"unknown-keyword\0" as *const u8 as *const libc::c_char,
    b"xdev\0" as *const u8 as *const libc::c_char,
    b"decompress-program\0" as *const u8 as *const libc::c_char,
    b"existing-file\0" as *const u8 as *const libc::c_char,
    b"xattr-write\0" as *const u8 as *const libc::c_char,
    b"record-size\0" as *const u8 as *const libc::c_char,
    b"failed-read\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut warning_types: [libc::c_int; 25] = [
    !(0x2000 as libc::c_int | 0x1000 as libc::c_int | 0x80000 as libc::c_int
        | 0x100000 as libc::c_int | 0x400000 as libc::c_int),
    0x1 as libc::c_int,
    0x2 as libc::c_int,
    0x4 as libc::c_int,
    0x8 as libc::c_int,
    0x10 as libc::c_int,
    0x20 as libc::c_int,
    0x40 as libc::c_int,
    0x80 as libc::c_int,
    0x100 as libc::c_int,
    0x200 as libc::c_int,
    0x400 as libc::c_int,
    0x800 as libc::c_int,
    0x1000 as libc::c_int,
    0x2000 as libc::c_int,
    0x4000 as libc::c_int,
    0x8000 as libc::c_int,
    0x10000 as libc::c_int,
    0x20000 as libc::c_int,
    0x40000 as libc::c_int,
    0x80000 as libc::c_int,
    0x100000 as libc::c_int,
    0x200000 as libc::c_int,
    0x400000 as libc::c_int,
    0x800000 as libc::c_int,
];
pub static mut warning_option: libc::c_int = !(0x2000 as libc::c_int
    | 0x1000 as libc::c_int | 0x80000 as libc::c_int | 0x100000 as libc::c_int
    | 0x400000 as libc::c_int);
pub unsafe extern "C" fn set_warning_option(mut arg: *const libc::c_char) {
    let mut negate: libc::c_int = 0 as libc::c_int;
    let mut option: libc::c_int = 0;
    if strcmp(arg, b"none\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        warning_option = 0 as libc::c_int;
        return;
    }
    if strlen(arg) > 2 as libc::c_int as libc::c_ulong
        && memcmp(
            arg as *const libc::c_void,
            b"no-\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        negate = 1 as libc::c_int;
        arg = arg.offset(3 as libc::c_int as isize);
    }
    option = warning_types[__xargmatch_internal(
        b"--warning\0" as *const u8 as *const libc::c_char,
        arg,
        warning_args.as_ptr(),
        warning_types.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        argmatch_die,
    ) as usize];
    if negate != 0 {
        warning_option &= !option;
    } else {
        warning_option |= option;
    };
}
