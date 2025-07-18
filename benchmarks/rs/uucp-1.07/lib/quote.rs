use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn zquote_cmd_string(
        zorig: *const libc::c_char,
        fbackslashonly: boolean,
    ) -> *mut libc::c_char;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
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
pub static mut quote_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: quote.c,v 1.2 2002/03/05 19:10:42 ian Rel $\0")
};
#[inline]
unsafe extern "C" fn fneeds_quotes(mut z: *const libc::c_char) -> boolean {
    return (!z.is_null()
        && *z.offset(strcspn(z, b" \t\n\0" as *const u8 as *const libc::c_char) as isize)
            as libc::c_int != '\0' as i32) as libc::c_int;
}
pub unsafe extern "C" fn fcmd_needs_quotes(mut qcmd: *const scmd) -> boolean {
    if fneeds_quotes((*qcmd).zfrom) != 0 || fneeds_quotes((*qcmd).zto) != 0
        || fneeds_quotes((*qcmd).zuser) != 0 || fneeds_quotes((*qcmd).znotify) != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn uquote_cmd(mut qorig: *const scmd, mut qnew: *mut scmd) {
    (*qnew).bcmd = (*qorig).bcmd;
    (*qnew).bgrade = (*qorig).bgrade;
    (*qnew).pseq = (*qorig).pseq;
    (*qnew).zfrom = zquote_cmd_string((*qorig).zfrom, 0 as libc::c_int);
    (*qnew).zto = zquote_cmd_string((*qorig).zto, 0 as libc::c_int);
    (*qnew).zuser = zquote_cmd_string((*qorig).zuser, 0 as libc::c_int);
    if !(strchr((*qorig).zoptions, 'q' as i32)).is_null() {
        (*qnew).zoptions = zbufcpy((*qorig).zoptions);
    } else {
        let mut clen: size_t = 0;
        let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
        clen = strlen((*qorig).zoptions);
        z = zbufalc(clen.wrapping_add(2 as libc::c_int as libc::c_ulong));
        memcpy(z as *mut libc::c_void, (*qorig).zoptions as *const libc::c_void, clen);
        *z.offset(clen as isize) = 'q' as i32 as libc::c_char;
        *z
            .offset(
                clen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        (*qnew).zoptions = z;
    }
    (*qnew).ztemp = zbufcpy((*qorig).ztemp);
    (*qnew).imode = (*qorig).imode;
    (*qnew).znotify = zquote_cmd_string((*qorig).znotify, 0 as libc::c_int);
    (*qnew).cbytes = (*qorig).cbytes;
    (*qnew).zcmd = zbufcpy((*qorig).zcmd);
    (*qnew).ipos = (*qorig).ipos;
}
pub unsafe extern "C" fn ufree_quoted_cmd(mut qcmd: *mut scmd) {
    ubuffree((*qcmd).zfrom as *mut libc::c_char);
    ubuffree((*qcmd).zto as *mut libc::c_char);
    ubuffree((*qcmd).zuser as *mut libc::c_char);
    ubuffree((*qcmd).ztemp as *mut libc::c_char);
    ubuffree((*qcmd).znotify as *mut libc::c_char);
    ubuffree((*qcmd).zcmd as *mut libc::c_char);
    ubuffree((*qcmd).zoptions as *mut libc::c_char);
}
