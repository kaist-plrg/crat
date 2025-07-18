use ::libc;
extern "C" {
    pub type htab;
    pub type upstream;
    pub type buffer_s;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut config: *mut config_s;
    fn orderedmap_find(
        o: *mut orderedmap,
        key: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn log_message(level: libc::c_int, fmt: *const libc::c_char, _: ...);
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
pub struct reversepath {
    pub next: *mut reversepath,
    pub path: *mut libc::c_char,
    pub url: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_s {
    pub client_fd: libc::c_int,
    pub server_fd: libc::c_int,
    pub cbuffer: *mut buffer_s,
    pub sbuffer: *mut buffer_s,
    pub request_line: *mut libc::c_char,
    pub connect_method: libc::c_uint,
    pub show_stats: libc::c_uint,
    pub error_variables: *mut htab,
    pub error_number: libc::c_int,
    pub error_string: *mut libc::c_char,
    pub content_length: C2RustUnnamed_0,
    pub server_ip_addr: *mut libc::c_char,
    pub client_ip_addr: *mut libc::c_char,
    pub protocol: C2RustUnnamed,
    pub reversepath: *mut libc::c_char,
    pub upstream_proxy: *mut upstream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub major: libc::c_uint,
    pub minor: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub server: libc::c_long,
    pub client: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orderedmap {
    pub values: *mut sblist,
    pub map: *mut htab,
}
pub type orderedmap_0 = *mut orderedmap;
pub unsafe extern "C" fn reversepath_add(
    mut path: *const libc::c_char,
    mut url: *const libc::c_char,
    mut reversepath_list: *mut *mut reversepath,
) {
    let mut reverse: *mut reversepath = 0 as *mut reversepath;
    let mut l: size_t = 0;
    if url.is_null() {
        log_message(
            4 as libc::c_int,
            b"Illegal reverse proxy rule: missing url\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (strstr(url, b"://\0" as *const u8 as *const libc::c_char)).is_null() {
        log_message(
            4 as libc::c_int,
            b"Skipping reverse proxy rule: '%s' is not a valid url\0" as *const u8
                as *const libc::c_char,
            url,
        );
        return;
    }
    if !path.is_null() && *path as libc::c_int != '/' as i32 {
        log_message(
            4 as libc::c_int,
            b"Skipping reverse proxy rule: path '%s' doesn't start with a /\0"
                as *const u8 as *const libc::c_char,
            path,
        );
        return;
    }
    reverse = malloc(::std::mem::size_of::<reversepath>() as libc::c_ulong)
        as *mut reversepath;
    if reverse.is_null() {
        log_message(
            3 as libc::c_int,
            b"Unable to allocate memory in reversepath_add()\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if path.is_null() {
        (*reverse).path = strdup(b"/\0" as *const u8 as *const libc::c_char);
    } else {
        l = strlen(path);
        if l != 0
            && *path.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '/' as i32
        {
            (*reverse).path = strdup(path);
        } else {
            (*reverse)
                .path = malloc(l.wrapping_add(2 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            memcpy((*reverse).path as *mut libc::c_void, path as *const libc::c_void, l);
            *((*reverse).path).offset(l as isize) = '/' as i32 as libc::c_char;
            *((*reverse).path)
                .offset(
                    l.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
    }
    (*reverse).url = strdup(url);
    (*reverse).next = *reversepath_list;
    *reversepath_list = reverse;
    log_message(
        6 as libc::c_int,
        b"Added reverse proxy rule: %s -> %s\0" as *const u8 as *const libc::c_char,
        (*reverse).path,
        (*reverse).url,
    );
}
pub unsafe extern "C" fn reversepath_get(
    mut url: *mut libc::c_char,
    mut reverse: *mut reversepath,
) -> *mut reversepath {
    let mut l: size_t = 0;
    let mut lu: size_t = 0;
    let mut lp: size_t = 0;
    while !reverse.is_null() {
        lu = strlen(url);
        lp = strlen((*reverse).path);
        l = lu;
        if (l == lp.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            || {
                l = lp;
                l <= lu
            })
            && memcmp(
                url as *const libc::c_void,
                (*reverse).path as *const libc::c_void,
                l,
            ) == 0
        {
            return reverse;
        }
        reverse = (*reverse).next;
    }
    return 0 as *mut reversepath;
}
pub unsafe extern "C" fn free_reversepath_list(mut reverse: *mut reversepath) {
    while !reverse.is_null() {
        let mut tmp: *mut reversepath = reverse;
        reverse = (*reverse).next;
        free((*tmp).url as *mut libc::c_void);
        (*tmp).url = 0 as *mut libc::c_char;
        free((*tmp).path as *mut libc::c_void);
        (*tmp).path = 0 as *mut libc::c_char;
        free(tmp as *mut libc::c_void);
        tmp = 0 as *mut reversepath;
    }
}
pub unsafe extern "C" fn reverse_rewrite_url(
    mut connptr: *mut conn_s,
    mut hashofheaders: orderedmap_0,
    mut url: *mut libc::c_char,
    mut status: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut rewrite_url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cookie: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cookieval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reverse: *mut reversepath = 0 as *mut reversepath;
    *status = 0 as libc::c_int;
    if *url as libc::c_int == '/' as i32 {
        reverse = reversepath_get(url, (*config).reversepath_list);
        if !reverse.is_null() {
            let mut lu: size_t = strlen(url);
            let mut lrp: size_t = strlen((*reverse).path);
            if lrp > lu {
                rewrite_url = strdup((*reverse).path);
                *status = 301 as libc::c_int;
            } else {
                rewrite_url = malloc(
                    (strlen((*reverse).url))
                        .wrapping_add(lu)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                sprintf(
                    rewrite_url,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    (*reverse).url,
                    url.offset(lrp as isize),
                );
            }
        } else if (*config).reversemagic != 0
            && {
                cookie = orderedmap_find(
                    hashofheaders,
                    b"cookie\0" as *const u8 as *const libc::c_char,
                );
                !cookie.is_null()
            }
        {
            cookieval = strstr(
                cookie,
                b"yummy_magical_cookie=\0" as *const u8 as *const libc::c_char,
            );
            if !cookieval.is_null()
                && {
                    reverse = reversepath_get(
                        cookieval
                            .offset(
                                strlen(
                                    b"yummy_magical_cookie\0" as *const u8
                                        as *const libc::c_char,
                                ) as isize,
                            )
                            .offset(1 as libc::c_int as isize),
                        (*config).reversepath_list,
                    );
                    !reverse.is_null()
                }
            {
                rewrite_url = malloc(
                    (strlen(url))
                        .wrapping_add(strlen((*reverse).url))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                strcpy(rewrite_url, (*reverse).url);
                strcat(rewrite_url, url.offset(1 as libc::c_int as isize));
                log_message(
                    6 as libc::c_int,
                    b"Magical tracking cookie says: %s\0" as *const u8
                        as *const libc::c_char,
                    (*reverse).path,
                );
            }
        }
    }
    if rewrite_url.is_null() {
        return 0 as *mut libc::c_char;
    }
    log_message(
        8 as libc::c_int,
        b"Rewriting URL: %s -> %s\0" as *const u8 as *const libc::c_char,
        url,
        rewrite_url,
    );
    if (*config).reversemagic != 0 && !reverse.is_null() {
        (*connptr).reversepath = strdup((*reverse).path);
    }
    return rewrite_url;
}
