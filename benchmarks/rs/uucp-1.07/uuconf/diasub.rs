use ::libc;
pub type UUCONF_POINTER = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_chat {
    pub uuconf_pzchat: *mut *mut libc::c_char,
    pub uuconf_pzprogram: *mut *mut libc::c_char,
    pub uuconf_ctimeout: libc::c_int,
    pub uuconf_pzfail: *mut *mut libc::c_char,
    pub uuconf_fstrip: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_proto_param {
    pub uuconf_bproto: libc::c_int,
    pub uuconf_qentries: *mut uuconf_proto_param_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_proto_param_entry {
    pub uuconf_cargs: libc::c_int,
    pub uuconf_pzargs: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uuconf_dialer {
    pub uuconf_zname: *mut libc::c_char,
    pub uuconf_schat: uuconf_chat,
    pub uuconf_zdialtone: *mut libc::c_char,
    pub uuconf_zpause: *mut libc::c_char,
    pub uuconf_fcarrier: libc::c_int,
    pub uuconf_ccarrier_wait: libc::c_int,
    pub uuconf_fdtr_toggle: libc::c_int,
    pub uuconf_fdtr_toggle_wait: libc::c_int,
    pub uuconf_scomplete: uuconf_chat,
    pub uuconf_sabort: uuconf_chat,
    pub uuconf_qproto_params: *mut uuconf_proto_param,
    pub uuconf_ireliable: libc::c_int,
    pub uuconf_palloc: UUCONF_POINTER,
}
pub static mut _uuconf_diasub_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: diasub.c,v 1.7 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_uclear_dialer(mut qdialer: *mut uuconf_dialer) {
    (*qdialer).uuconf_zname = 0 as *mut libc::c_char;
    (*qdialer).uuconf_schat.uuconf_pzchat = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_schat.uuconf_pzprogram = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_schat.uuconf_ctimeout = 60 as libc::c_int;
    (*qdialer).uuconf_schat.uuconf_pzfail = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_schat.uuconf_fstrip = 1 as libc::c_int;
    (*qdialer)
        .uuconf_zdialtone = b",\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*qdialer)
        .uuconf_zpause = b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*qdialer).uuconf_fcarrier = 1 as libc::c_int;
    (*qdialer).uuconf_ccarrier_wait = 60 as libc::c_int;
    (*qdialer).uuconf_fdtr_toggle = 0 as libc::c_int;
    (*qdialer).uuconf_fdtr_toggle_wait = 0 as libc::c_int;
    (*qdialer).uuconf_scomplete.uuconf_pzchat = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_scomplete.uuconf_pzprogram = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_scomplete.uuconf_ctimeout = 60 as libc::c_int;
    (*qdialer).uuconf_scomplete.uuconf_pzfail = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_scomplete.uuconf_fstrip = 1 as libc::c_int;
    (*qdialer).uuconf_sabort.uuconf_pzchat = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_sabort.uuconf_pzprogram = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_sabort.uuconf_ctimeout = 60 as libc::c_int;
    (*qdialer).uuconf_sabort.uuconf_pzfail = 0 as *mut *mut libc::c_char;
    (*qdialer).uuconf_sabort.uuconf_fstrip = 1 as libc::c_int;
    (*qdialer).uuconf_qproto_params = 0 as *mut uuconf_proto_param;
    (*qdialer)
        .uuconf_ireliable = 0o4 as libc::c_int | 0o2 as libc::c_int
        | 0o20 as libc::c_int;
    (*qdialer).uuconf_palloc = 0 as *mut libc::c_void;
}
