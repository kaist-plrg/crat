use ::libc;
extern "C" {
    pub type htab;
    pub type upstream;
    pub type reversepath;
    fn htab_create(_: size_t) -> *mut htab;
    fn htab_find(_: *mut htab, key: *const libc::c_char) -> *mut htab_value;
    fn htab_insert(_: *mut htab, _: *mut libc::c_char, _: htab_value) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub basicauth_list: *mut sblist,
    pub logf_name: *mut libc::c_char,
    pub syslog: libc::c_uint,
    pub port: libc::c_uint,
    pub stathost: *mut libc::c_char,
    pub quit: libc::c_uint,
    pub maxclients: libc::c_uint,
    pub user: *mut libc::c_char,
    pub group: *mut libc::c_char,
    pub listen_addrs: *mut sblist,
    pub filter: *mut libc::c_char,
    pub filter_opts: libc::c_uint,
    pub add_xtinyproxy: libc::c_uint,
    pub reversepath_list: *mut reversepath,
    pub reverseonly: libc::c_uint,
    pub reversemagic: libc::c_uint,
    pub reversebaseurl: *mut libc::c_char,
    pub upstream_list: *mut upstream,
    pub pidpath: *mut libc::c_char,
    pub idletimeout: libc::c_uint,
    pub bind_addrs: *mut sblist,
    pub bindsame: libc::c_uint,
    pub via_proxy_name: *mut libc::c_char,
    pub disable_viaheader: libc::c_uint,
    pub errorpages: *mut htab,
    pub errorpage_undef: *mut libc::c_char,
    pub statpage: *mut libc::c_char,
    pub access_list: acl_list_t,
    pub connect_ports: *mut sblist,
    pub anonymous_map: *mut htab,
    pub add_headers: *mut sblist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
pub type acl_list_t = *mut sblist;
#[derive(Copy, Clone)]
#[repr(C)]
pub union htab_value {
    pub p: *mut libc::c_void,
    pub n: size_t,
}
pub unsafe extern "C" fn is_anonymous_enabled(mut conf: *mut config_s) -> libc::c_short {
    return (if !((*conf).anonymous_map).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_short;
}
pub unsafe extern "C" fn anonymous_search(
    mut conf: *mut config_s,
    mut s: *const libc::c_char,
) -> libc::c_int {
    return !(htab_find((*conf).anonymous_map, s)).is_null() as libc::c_int;
}
pub unsafe extern "C" fn anonymous_insert(
    mut conf: *mut config_s,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    if ((*conf).anonymous_map).is_null() {
        (*conf).anonymous_map = htab_create(32 as libc::c_int as size_t);
        if ((*conf).anonymous_map).is_null() {
            return -(1 as libc::c_int);
        }
    }
    if !(htab_find((*conf).anonymous_map, s)).is_null() {
        return 0 as libc::c_int;
    }
    return if htab_insert(
        (*conf).anonymous_map,
        s,
        htab_value {
            n: 1 as libc::c_int as size_t,
        },
    ) != 0
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
