use ::libc;
extern "C" {
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: libc::c_int,
    pub link: *mut libc::c_void,
    pub na: libc::c_int,
    pub arg: *mut *mut libc::c_char,
    pub nf: libc::c_int,
    pub name: *mut *mut libc::c_char,
    pub type_0: *mut libc::c_int,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut libc::c_char,
}
pub unsafe extern "C" fn _glp_db_iodbc_open(
    mut dca: *mut TABDCA,
    mut mode: libc::c_int,
) -> *mut libc::c_void {
    (dca == dca
        || {
            glp_assert_(
                b"dca == dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (mode == mode
        || {
            glp_assert_(
                b"mode == mode\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                303 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_printf(
        b"iODBC table driver not supported\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn _glp_db_iodbc_read(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> libc::c_int {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                309 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                310 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_db_iodbc_write(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> libc::c_int {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                315 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                316 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_db_iodbc_close(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> libc::c_int {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                321 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_db_mysql_open(
    mut dca: *mut TABDCA,
    mut mode: libc::c_int,
) -> *mut libc::c_void {
    (dca == dca
        || {
            glp_assert_(
                b"dca == dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1083 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (mode == mode
        || {
            glp_assert_(
                b"mode == mode\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1084 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_printf(
        b"MySQL table driver not supported\n\0" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn _glp_db_mysql_read(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> libc::c_int {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1090 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1091 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_db_mysql_write(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> libc::c_int {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1096 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1097 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_db_mysql_close(
    mut dca: *mut TABDCA,
    mut link: *mut libc::c_void,
) -> libc::c_int {
    (dca != dca
        || {
            glp_assert_(
                b"dca != dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1102 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (link != link
        || {
            glp_assert_(
                b"link != link\0" as *const u8 as *const libc::c_char,
                b"mpl/mplsql.c\0" as *const u8 as *const libc::c_char,
                1103 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
