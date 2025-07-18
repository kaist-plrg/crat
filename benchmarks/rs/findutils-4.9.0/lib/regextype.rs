use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagRegexTypeMap {
    pub name: *const libc::c_char,
    pub context: libc::c_int,
    pub option_val: libc::c_int,
}
pub const CONTEXT_GENERIC: C2RustUnnamed_0 = 2;
pub const CONTEXT_ALL: C2RustUnnamed_0 = 3;
pub const CONTEXT_FINDUTILS: C2RustUnnamed_0 = 1;
pub const N_REGEX_MAP_ENTRIES: C2RustUnnamed_1 = 13;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
static mut regex_map: [tagRegexTypeMap; 13] = [
    {
        let mut init = tagRegexTypeMap {
            name: b"findutils-default\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_FINDUTILS as libc::c_int,
            option_val: (0 as libc::c_int as libc::c_ulong
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"ed\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_GENERIC as libc::c_int,
            option_val: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"emacs\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"gnu-awk\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | 1 as libc::c_int as libc::c_ulong
                | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                & !((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int)) as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"grep\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int)
                & !(((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int))
                as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-awk\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | 1 as libc::c_int as libc::c_ulong
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"awk\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: (1 as libc::c_int as libc::c_ulong
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-basic\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-egrep\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int)
                & !((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int))
                as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"egrep\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int)
                & !((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                    | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                        << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int))
                as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-extended\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_ALL as libc::c_int,
            option_val: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int
                | (((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-minimal-basic\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_GENERIC as libc::c_int,
            option_val: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_int,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"sed\0" as *const u8 as *const libc::c_char,
            context: CONTEXT_GENERIC as libc::c_int,
            option_val: (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int
                | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int
                | ((((((((((((((((((((((((1 as libc::c_int as libc::c_ulong)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn get_regex_type(mut s: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut msglen: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    msglen = 0 as libc::c_uint as size_t;
    i = 0 as libc::c_uint;
    while i < N_REGEX_MAP_ENTRIES as libc::c_int as libc::c_uint {
        if 0 as libc::c_int == strcmp(regex_map[i as usize].name, s) {
            return regex_map[i as usize].option_val
        } else {
            msglen = (msglen as libc::c_ulong)
                .wrapping_add(
                    (strlen(quote(regex_map[i as usize].name)))
                        .wrapping_add(2 as libc::c_uint as libc::c_ulong),
                ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    buf = xmalloc((1 as libc::c_uint as libc::c_ulong).wrapping_add(msglen))
        as *mut libc::c_char;
    p = buf;
    i = 0 as libc::c_uint;
    while i < N_REGEX_MAP_ENTRIES as libc::c_int as libc::c_uint {
        if i > 0 as libc::c_uint {
            strcpy(p, b", \0" as *const u8 as *const libc::c_char);
            p = p.offset(2 as libc::c_int as isize);
        }
        p = p
            .offset(
                sprintf(
                    p,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quote(regex_map[i as usize].name),
                ) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    if ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong != 0 {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown regular expression type %s; valid types are %s.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(s),
            buf,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown regular expression type %s; valid types are %s.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(s),
            buf,
        );
        if 0 as libc::c_int != 0 {} else {
            unreachable!();
        };
    };
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn get_regex_type_name(
    mut ix: libc::c_uint,
) -> *const libc::c_char {
    if ix < N_REGEX_MAP_ENTRIES as libc::c_int as libc::c_uint {
        return regex_map[ix as usize].name
    } else {
        return 0 as *const libc::c_char
    };
}
pub unsafe extern "C" fn get_regex_type_flags(mut ix: libc::c_uint) -> libc::c_int {
    if ix < N_REGEX_MAP_ENTRIES as libc::c_int as libc::c_uint {
        return regex_map[ix as usize].option_val
    } else {
        return -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn get_regex_type_context(mut ix: libc::c_uint) -> libc::c_uint {
    if ix < N_REGEX_MAP_ENTRIES as libc::c_int as libc::c_uint {
        return regex_map[ix as usize].context as libc::c_uint
    } else {
        return 0 as libc::c_uint
    };
}
pub unsafe extern "C" fn get_regex_type_synonym(
    mut ix: libc::c_uint,
    mut context: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut flags: libc::c_int = 0;
    if ix >= N_REGEX_MAP_ENTRIES as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    flags = regex_map[ix as usize].option_val;
    i = 0 as libc::c_uint;
    while i < ix {
        if !(regex_map[i as usize].context as libc::c_uint & context
            == 0 as libc::c_int as libc::c_uint)
        {
            if flags == regex_map[i as usize].option_val {
                return i as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return -(1 as libc::c_int);
}
