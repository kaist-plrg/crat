use ::libc;
extern "C" {
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_free(l: *mut sblist);
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub unsafe extern "C" fn add_connect_port_allowed(
    mut port: libc::c_int,
    mut connect_ports: *mut *mut sblist,
) {
    if (*connect_ports).is_null() {
        *connect_ports = sblist_new(
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            16 as libc::c_int as size_t,
        );
        if (*connect_ports).is_null() {
            log_message(
                4 as libc::c_int,
                b"Could not create a list of allowed CONNECT ports\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    }
    log_message(
        6 as libc::c_int,
        b"Adding Port [%d] to the list allowed by CONNECT\0" as *const u8
            as *const libc::c_char,
        port,
    );
    sblist_add(*connect_ports, &mut port as *mut libc::c_int as *mut libc::c_void);
}
pub unsafe extern "C" fn check_allowed_connect_ports(
    mut port: libc::c_int,
    mut connect_ports: *mut sblist,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    if connect_ports.is_null() {
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*connect_ports).count {
        data = sblist_get(connect_ports, i) as *mut libc::c_int;
        if !data.is_null() && *data == port {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn free_connect_ports_list(mut connect_ports: *mut sblist) {
    sblist_free(connect_ports);
}
