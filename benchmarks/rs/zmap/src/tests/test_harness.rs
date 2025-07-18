use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fs_free(fs: *mut fieldset_t);
    fn fs_new_fieldset(_: *mut fielddefset_t) -> *mut fieldset_t;
    fn fs_new_repeated_string(free_: libc::c_int) -> *mut fieldset_t;
    fn fs_add_string(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        value: *mut libc::c_char,
        free_: libc::c_int,
    );
    fn fs_add_fieldset(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        child: *mut fieldset_t,
    );
    fn fs_add_repeated(
        fs: *mut fieldset_t,
        name: *const libc::c_char,
        child: *mut fieldset_t,
    );
    fn print_json_fieldset(fs: *mut fieldset_t) -> libc::c_int;
    fn cmdline_parser_ext(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        args_info: *mut gengetopt_args_info,
        params: *mut cmdline_parser_params,
    ) -> libc::c_int;
    fn cmdline_parser_print_help();
    fn cmdline_parser_print_version();
    fn cmdline_parser_params_create() -> *mut cmdline_parser_params;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_t = field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset {
    pub len: libc::c_int,
    pub fields: [field_t; 128],
    pub fds: *mut fielddefset_t,
    pub inner_type: libc::c_int,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
}
pub type fieldset_t = fieldset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gengetopt_args_info {
    pub help_help: *const libc::c_char,
    pub version_help: *const libc::c_char,
    pub help_given: libc::c_uint,
    pub version_given: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdline_parser_params {
    pub override_0: libc::c_int,
    pub initialize: libc::c_int,
    pub check_required: libc::c_int,
    pub check_ambiguity: libc::c_int,
    pub print_errors: libc::c_int,
}
pub unsafe extern "C" fn test_recursive_fieldsets() -> libc::c_int {
    let mut outer: *mut fieldset_t = fs_new_fieldset(0 as *mut fielddefset_t);
    let mut inner: *mut fieldset_t = fs_new_fieldset(0 as *mut fielddefset_t);
    let mut repeated: *mut fieldset_t = fs_new_repeated_string(0 as libc::c_int);
    if (*repeated).type_0 == 6 as libc::c_int {} else {
        __assert_fail(
            b"repeated->type == FS_REPEATED\0" as *const u8 as *const libc::c_char,
            b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int test_recursive_fieldsets(void)\0"))
                .as_ptr(),
        );
    }
    'c_8367: {
        if (*repeated).type_0 == 6 as libc::c_int {} else {
            __assert_fail(
                b"repeated->type == FS_REPEATED\0" as *const u8 as *const libc::c_char,
                b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int test_recursive_fieldsets(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*repeated).len == 0 as libc::c_int {} else {
        __assert_fail(
            b"repeated->len == 0\0" as *const u8 as *const libc::c_char,
            b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int test_recursive_fieldsets(void)\0"))
                .as_ptr(),
        );
    }
    'c_8326: {
        if (*repeated).len == 0 as libc::c_int {} else {
            __assert_fail(
                b"repeated->len == 0\0" as *const u8 as *const libc::c_char,
                b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int test_recursive_fieldsets(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*repeated).inner_type == 1 as libc::c_int {} else {
        __assert_fail(
            b"repeated->inner_type == FS_STRING\0" as *const u8 as *const libc::c_char,
            b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int test_recursive_fieldsets(void)\0"))
                .as_ptr(),
        );
    }
    'c_8281: {
        if (*repeated).inner_type == 1 as libc::c_int {} else {
            __assert_fail(
                b"repeated->inner_type == FS_STRING\0" as *const u8
                    as *const libc::c_char,
                b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int test_recursive_fieldsets(void)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        fs_add_string(
            repeated,
            0 as *const libc::c_char,
            b"hello world!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        i += 1;
        i;
    }
    fs_add_repeated(
        outer,
        b"repeatedstuff\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        repeated,
    );
    fs_add_string(
        outer,
        b"name\0" as *const u8 as *const libc::c_char,
        strdup(b"value\0" as *const u8 as *const libc::c_char),
        0 as libc::c_int,
    );
    fs_add_string(
        inner,
        b"name2\0" as *const u8 as *const libc::c_char,
        strdup(b"value2\0" as *const u8 as *const libc::c_char),
        0 as libc::c_int,
    );
    fs_add_fieldset(outer, b"inner\0" as *const u8 as *const libc::c_char, inner);
    print_json_fieldset(outer);
    fs_free(outer);
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut args: gengetopt_args_info = gengetopt_args_info {
        help_help: 0 as *const libc::c_char,
        version_help: 0 as *const libc::c_char,
        help_given: 0,
        version_given: 0,
    };
    let mut params: *mut cmdline_parser_params = 0 as *mut cmdline_parser_params;
    params = cmdline_parser_params_create();
    if !params.is_null() {} else {
        __assert_fail(
            b"params\0" as *const u8 as *const libc::c_char,
            b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_8505: {
        if !params.is_null() {} else {
            __assert_fail(
                b"params\0" as *const u8 as *const libc::c_char,
                b"tests/test_harness.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    (*params).initialize = 1 as libc::c_int;
    (*params).override_0 = 0 as libc::c_int;
    (*params).check_required = 0 as libc::c_int;
    if cmdline_parser_ext(argc, argv, &mut args, params) != 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    if args.help_given != 0 {
        cmdline_parser_print_help();
        exit(0 as libc::c_int);
    }
    if args.version_given != 0 {
        cmdline_parser_print_version();
        exit(0 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 100000000 as libc::c_int {
        test_recursive_fieldsets();
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
