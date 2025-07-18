use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn cescape(zbuf: *mut libc::c_char) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scmd {
    pub bcmd: libc::c_char,
    pub bgrade: libc::c_char,
    pub pseq: pointer,
    pub zfrom: *const libc::c_char,
    pub zto: *const libc::c_char,
    pub zuser: *const libc::c_char,
    pub zoptions: *const libc::c_char,
    pub ztemp: *const libc::c_char,
    pub imode: libc::c_uint,
    pub znotify: *const libc::c_char,
    pub cbytes: libc::c_long,
    pub zcmd: *const libc::c_char,
    pub ipos: libc::c_long,
}
pub static mut parse_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: parse.c,v 1.11 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn fparse_cmd(
    mut zcmd: *mut libc::c_char,
    mut qcmd: *mut scmd,
) -> boolean {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
    z = strtok(zcmd, b" \t\n\0" as *const u8 as *const libc::c_char);
    if z.is_null() {
        return 0 as libc::c_int;
    }
    (*qcmd).bcmd = *z;
    if (*qcmd).bcmd as libc::c_int != 'S' as i32
        && (*qcmd).bcmd as libc::c_int != 'R' as i32
        && (*qcmd).bcmd as libc::c_int != 'X' as i32
        && (*qcmd).bcmd as libc::c_int != 'E' as i32
        && (*qcmd).bcmd as libc::c_int != 'H' as i32
        && (*qcmd).bcmd as libc::c_int != 'P' as i32
    {
        return 0 as libc::c_int;
    }
    (*qcmd).bgrade = '\0' as i32 as libc::c_char;
    (*qcmd).pseq = 0 as *mut libc::c_void;
    (*qcmd).zfrom = 0 as *const libc::c_char;
    (*qcmd).zto = 0 as *const libc::c_char;
    (*qcmd).zuser = 0 as *const libc::c_char;
    (*qcmd).zoptions = 0 as *const libc::c_char;
    (*qcmd).ztemp = 0 as *const libc::c_char;
    (*qcmd).imode = 0o666 as libc::c_int as libc::c_uint;
    (*qcmd).znotify = 0 as *const libc::c_char;
    (*qcmd).cbytes = -(1 as libc::c_int) as libc::c_long;
    (*qcmd).zcmd = 0 as *const libc::c_char;
    (*qcmd).ipos = 0 as libc::c_int as libc::c_long;
    if (*qcmd).bcmd as libc::c_int == 'H' as i32 {
        if *z.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
            if *z.offset(1 as libc::c_int as isize) as libc::c_int == 'Y' as i32 {
                (*qcmd).bcmd = 'Y' as i32 as libc::c_char;
            } else if *z.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
                (*qcmd).bcmd = 'N' as i32 as libc::c_char;
            } else {
                return 0 as libc::c_int
            }
        }
        return 1 as libc::c_int;
    }
    if (*qcmd).bcmd as libc::c_int == 'P' as i32 {
        return 1 as libc::c_int;
    }
    if *z.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        return 0 as libc::c_int;
    }
    z = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\n\0" as *const u8 as *const libc::c_char,
    );
    if z.is_null() {
        return 0 as libc::c_int;
    }
    (*qcmd).zfrom = z;
    z = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\n\0" as *const u8 as *const libc::c_char,
    );
    if z.is_null() {
        return 0 as libc::c_int;
    }
    (*qcmd).zto = z;
    z = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\n\0" as *const u8 as *const libc::c_char,
    );
    if z.is_null() {
        return 0 as libc::c_int;
    }
    (*qcmd).zuser = z;
    z = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\n\0" as *const u8 as *const libc::c_char,
    );
    if z.is_null() || *z as libc::c_int != '-' as i32 {
        return 0 as libc::c_int;
    }
    (*qcmd).zoptions = z.offset(1 as libc::c_int as isize);
    if (*qcmd).bcmd as libc::c_int == 'X' as i32 {
        ulunquote_cmd(qcmd);
        return 1 as libc::c_int;
    }
    if (*qcmd).bcmd as libc::c_int == 'R' as i32 {
        z = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\n\0" as *const u8 as *const libc::c_char,
        );
        if !z.is_null() {
            if strcmp(z, b"dummy\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                (*qcmd).cbytes = strtol(z, &mut zend, 0 as libc::c_int);
                if *zend as libc::c_int != '\0' as i32 {
                    (*qcmd).cbytes = -(1 as libc::c_int) as libc::c_long;
                }
            } else if !(strtok(
                0 as *mut libc::c_void as *mut libc::c_char,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
                && !(strtok(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    b" \t\n\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                && !(strtok(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    b" \t\n\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
            {
                z = strtok(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    b" \t\n\0" as *const u8 as *const libc::c_char,
                );
                if !z.is_null() {
                    (*qcmd).ipos = strtol(z, &mut zend, 0 as libc::c_int);
                    if *zend as libc::c_int != '\0' as i32 {
                        (*qcmd).ipos = 0 as libc::c_int as libc::c_long;
                    }
                }
            }
        }
        ulunquote_cmd(qcmd);
        return 1 as libc::c_int;
    }
    z = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\n\0" as *const u8 as *const libc::c_char,
    );
    if z.is_null() {
        return 0 as libc::c_int;
    }
    (*qcmd).ztemp = z;
    z = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\n\0" as *const u8 as *const libc::c_char,
    );
    if z.is_null() {
        return 0 as libc::c_int;
    }
    (*qcmd)
        .imode = strtol(z, &mut zend, 0 as libc::c_int) as libc::c_int as libc::c_uint;
    if *zend as libc::c_int != '\0' as i32 {
        return 0 as libc::c_int;
    }
    if (*qcmd).imode == 666 as libc::c_int as libc::c_uint {
        (*qcmd).imode = 0o666 as libc::c_int as libc::c_uint;
    } else if (*qcmd).imode == 777 as libc::c_int as libc::c_uint {
        (*qcmd).imode = 0o777 as libc::c_int as libc::c_uint;
    }
    z = strtok(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \t\n\0" as *const u8 as *const libc::c_char,
    );
    if (*qcmd).bcmd as libc::c_int == 'E' as i32 && z.is_null() {
        return 0 as libc::c_int;
    }
    (*qcmd).znotify = z;
    if !z.is_null()
        && strcmp(z, b"dummy\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        z = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if !z.is_null() {
        z = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            b" \t\n\0" as *const u8 as *const libc::c_char,
        );
        if !z.is_null() {
            (*qcmd).cbytes = strtol(z, &mut zend, 0 as libc::c_int);
            if *zend as libc::c_int != '\0' as i32 {
                (*qcmd).cbytes = -(1 as libc::c_int) as libc::c_long;
            }
        } else if (*qcmd).bcmd as libc::c_int == 'E' as i32 {
            return 0 as libc::c_int
        }
        if !z.is_null() {
            z = strtok(
                0 as *mut libc::c_void as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if !z.is_null() {
                *z
                    .offset(
                        strcspn(z, b"\n\0" as *const u8 as *const libc::c_char) as isize,
                    ) = '\0' as i32 as libc::c_char;
            }
            if (*qcmd).bcmd as libc::c_int == 'E' as i32 && z.is_null() {
                return 0 as libc::c_int;
            }
            (*qcmd).zcmd = z;
        }
    }
    ulunquote_cmd(qcmd);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ulunquote_cmd(mut qcmd: *mut scmd) {
    if ((*qcmd).zoptions).is_null() || (strchr((*qcmd).zoptions, 'q' as i32)).is_null() {
        return;
    }
    if !((*qcmd).zfrom).is_null() {
        cescape((*qcmd).zfrom as *mut libc::c_char);
    }
    if !((*qcmd).zto).is_null() {
        cescape((*qcmd).zto as *mut libc::c_char);
    }
    if !((*qcmd).zuser).is_null() {
        cescape((*qcmd).zuser as *mut libc::c_char);
    }
    if !((*qcmd).znotify).is_null() {
        cescape((*qcmd).znotify as *mut libc::c_char);
    }
    if !((*qcmd).zcmd).is_null() {
        cescape((*qcmd).zcmd as *mut libc::c_char);
    }
}
