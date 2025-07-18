use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_setcancelstate(
        __state: libc::c_int,
        __oldstate: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: size_t) -> size_t;
    fn tcp_read(
        tcp: *mut tcp_t,
        buffer: *mut libc::c_void,
        size: libc::c_int,
    ) -> ssize_t;
    fn conn_set(conn: *mut conn_t, set_url: *const libc::c_char) -> libc::c_int;
    fn conn_disconnect(conn: *mut conn_t);
    fn conn_init(conn: *mut conn_t) -> libc::c_int;
    fn conn_setup(conn: *mut conn_t) -> libc::c_int;
    fn conn_exec(conn: *mut conn_t) -> libc::c_int;
    fn conn_info(conn: *mut conn_t) -> libc::c_int;
    fn axel_gettime() -> libc::c_double;
    fn axel_sleep(delay: timespec) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint16_t = __uint16_t;
pub type sa_family_t = libc::c_ushort;
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_CANCEL_DISABLE: C2RustUnnamed = 1;
pub const PTHREAD_CANCEL_ENABLE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: C2RustUnnamed_0 = 1;
pub const PTHREAD_CANCEL_DEFERRED: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct message_t {
    pub next: *mut libc::c_void,
    pub text: [libc::c_char; 1024],
}
pub type if_t = message_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct abuf_t {
    pub p: *mut libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf_t {
    pub default_filename: [libc::c_char; 1024],
    pub http_proxy: [libc::c_char; 1024],
    pub no_proxy: [libc::c_char; 1024],
    pub num_connections: uint16_t,
    pub strip_cgi_parameters: libc::c_int,
    pub save_state_interval: libc::c_int,
    pub connection_timeout: libc::c_int,
    pub reconnect_delay: libc::c_int,
    pub max_redirect: libc::c_int,
    pub buffer_size: libc::c_int,
    pub max_speed: libc::c_ulonglong,
    pub verbose: libc::c_int,
    pub alternate_output: libc::c_int,
    pub insecure: libc::c_int,
    pub no_clobber: libc::c_int,
    pub percentage: libc::c_int,
    pub interfaces: *mut if_t,
    pub ai_family: sa_family_t,
    pub search_timeout: libc::c_int,
    pub search_threads: libc::c_int,
    pub search_amount: libc::c_int,
    pub search_top: libc::c_int,
    pub io_timeout: libc::c_uint,
    pub add_header_count: libc::c_int,
    pub add_header: [[libc::c_char; 1024]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcp_t {
    pub fd: libc::c_int,
    pub ai_family: sa_family_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftp_t {
    pub cwd: [libc::c_char; 1024],
    pub message: *mut libc::c_char,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub data_tcp: tcp_t,
    pub proto: libc::c_int,
    pub ftp_mode: libc::c_int,
    pub local_if: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_t {
    pub host: [libc::c_char; 1024],
    pub auth: [libc::c_char; 1024],
    pub request: [abuf_t; 1],
    pub headers: [abuf_t; 1],
    pub port: libc::c_int,
    pub proto: libc::c_int,
    pub proxy: libc::c_int,
    pub proxy_auth: [libc::c_char; 1024],
    pub firstbyte: off_t,
    pub lastbyte: off_t,
    pub status: libc::c_int,
    pub tcp: tcp_t,
    pub local_if: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_t {
    pub conf: *mut conf_t,
    pub proto: libc::c_int,
    pub port: libc::c_int,
    pub proxy: libc::c_int,
    pub host: [libc::c_char; 1024],
    pub dir: [libc::c_char; 1024],
    pub file: [libc::c_char; 1024],
    pub user: [libc::c_char; 1024],
    pub pass: [libc::c_char; 1024],
    pub output_filename: [libc::c_char; 1024],
    pub ftp: [ftp_t; 1],
    pub http: [http_t; 1],
    pub size: off_t,
    pub currentbyte: off_t,
    pub lastbyte: off_t,
    pub tcp: *mut tcp_t,
    pub enabled: bool,
    pub supported: bool,
    pub last_transfer: libc::c_int,
    pub message: *mut libc::c_char,
    pub local_if: *mut libc::c_char,
    pub state: bool,
    pub setup_thread: [pthread_t; 1],
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_t {
    pub url: [libc::c_char; 1024],
    pub speed_start_time: libc::c_double,
    pub speed: off_t,
    pub size: off_t,
    pub speed_thread: [pthread_t; 1],
    pub conf: *mut conf_t,
}
pub const SPEED_DONE: C2RustUnnamed_1 = -1;
pub const SPEED_FAILED: C2RustUnnamed_1 = -2;
pub const SPEED_ACTIVE: C2RustUnnamed_1 = -3;
pub const SPEED_PENDING: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_int;
pub unsafe extern "C" fn search_makelist(
    mut results: *mut search_t,
    mut orig_url: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut size: libc::c_int = 8192 as libc::c_int;
    let mut conn: [conn_t; 1] = [conn_t {
        conf: 0 as *mut conf_t,
        proto: 0,
        port: 0,
        proxy: 0,
        host: [0; 1024],
        dir: [0; 1024],
        file: [0; 1024],
        user: [0; 1024],
        pass: [0; 1024],
        output_filename: [0; 1024],
        ftp: [ftp_t {
            cwd: [0; 1024],
            message: 0 as *mut libc::c_char,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            data_tcp: tcp_t { fd: 0, ai_family: 0 },
            proto: 0,
            ftp_mode: 0,
            local_if: 0 as *mut libc::c_char,
        }; 1],
        http: [http_t {
            host: [0; 1024],
            auth: [0; 1024],
            request: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            headers: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            port: 0,
            proto: 0,
            proxy: 0,
            proxy_auth: [0; 1024],
            firstbyte: 0,
            lastbyte: 0,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            local_if: 0 as *mut libc::c_char,
        }; 1],
        size: 0,
        currentbyte: 0,
        lastbyte: 0,
        tcp: 0 as *mut tcp_t,
        enabled: false,
        supported: false,
        last_transfer: 0,
        message: 0 as *mut libc::c_char,
        local_if: 0 as *mut libc::c_char,
        state: false,
        setup_thread: [0; 1],
        lock: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
    }; 1];
    let mut t: libc::c_double = 0.;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    memset(
        conn.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conn_t>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*conn.as_mut_ptr()).conf;
    *fresh0 = (*results).conf;
    t = axel_gettime();
    if conn_set(conn.as_mut_ptr(), orig_url) == 0 || conn_init(conn.as_mut_ptr()) == 0
        || conn_info(conn.as_mut_ptr()) == 0
    {
        return -(1 as libc::c_int);
    }
    let mut orig_len: size_t = strlcpy(
        ((*results.offset(0 as libc::c_int as isize)).url).as_mut_ptr(),
        orig_url,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    (*results.offset(0 as libc::c_int as isize))
        .speed = (1 as libc::c_int as libc::c_double
        + 1000 as libc::c_int as libc::c_double * (axel_gettime() - t)) as off_t;
    (*results.offset(0 as libc::c_int as isize)).size = (*conn.as_mut_ptr()).size;
    let mut nresults: libc::c_int = 1 as libc::c_int;
    let mut s: *mut libc::c_char = malloc(size as libc::c_ulong) as *mut libc::c_char;
    if s.is_null() {
        return 1 as libc::c_int;
    }
    snprintf(
        s,
        size as libc::c_ulong,
        b"http://www.filesearching.com/cgi-bin/s?w=a&x=15&y=15&s=on&e=on&l=en&t=f&o=n&q=%s&m=%i&s1=%jd&s2=%jd\0"
            as *const u8 as *const libc::c_char,
        ((*conn.as_mut_ptr()).file).as_mut_ptr(),
        (*(*results).conf).search_amount,
        (*conn.as_mut_ptr()).size,
        (*conn.as_mut_ptr()).size,
    );
    conn_disconnect(conn.as_mut_ptr());
    memset(
        conn.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conn_t>() as libc::c_ulong,
    );
    let ref mut fresh1 = (*conn.as_mut_ptr()).conf;
    *fresh1 = (*results).conf;
    if !(conn_set(conn.as_mut_ptr(), s) == 0) {
        pthread_mutex_lock(&mut (*conn.as_mut_ptr()).lock);
        let mut tmp: libc::c_int = conn_setup(conn.as_mut_ptr());
        pthread_mutex_unlock(&mut (*conn.as_mut_ptr()).lock);
        if !(tmp == 0 || conn_exec(conn.as_mut_ptr()) == 0) {
            let mut j: libc::c_int = 0 as libc::c_int;
            let mut i: libc::c_int = 0;
            loop {
                i = tcp_read(
                    (*conn.as_mut_ptr()).tcp,
                    s.offset(j as isize) as *mut libc::c_void,
                    size - j,
                ) as libc::c_int;
                if !(i > 0 as libc::c_int) {
                    current_block = 5634871135123216486;
                    break;
                }
                j += i;
                if !(j + 10 as libc::c_int >= size) {
                    continue;
                }
                size *= 2 as libc::c_int;
                let mut tmp_0: *mut libc::c_char = realloc(
                    s as *mut libc::c_void,
                    size as libc::c_ulong,
                ) as *mut libc::c_char;
                if tmp_0.is_null() {
                    current_block = 12857748498702458421;
                    break;
                }
                s = tmp_0;
                memset(
                    s.offset((size / 2 as libc::c_int) as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    (size / 2 as libc::c_int) as libc::c_ulong,
                );
            }
            match current_block {
                12857748498702458421 => {}
                _ => {
                    *s.offset(j as isize) = '\0' as i32 as libc::c_char;
                    conn_disconnect(conn.as_mut_ptr());
                    start = strstr(
                        s,
                        b"<pre class=list\0" as *const u8 as *const libc::c_char,
                    );
                    if !start.is_null() {
                        end = strstr(
                            start,
                            b"</pre>\0" as *const u8 as *const libc::c_char,
                        );
                        if !end.is_null() {
                            let mut url: *const libc::c_char = 0 as *const libc::c_char;
                            let mut eol: *const libc::c_char = 0 as *const libc::c_char;
                            while start < end
                                && nresults < (*(*results).conf).search_amount
                            {
                                eol = strchr(start, '\n' as i32);
                                if eol > end || eol.is_null() {
                                    eol = end;
                                }
                                loop {
                                    url = start;
                                    start = (strstr(
                                        start,
                                        b"<a href=\0" as *const u8 as *const libc::c_char,
                                    ))
                                        .offset(8 as libc::c_int as isize);
                                    if !(start < eol) {
                                        break;
                                    }
                                }
                                if !(strncmp(url, orig_url, orig_len) == 0) {
                                    strlcpy(
                                        ((*results.offset(nresults as isize)).url).as_mut_ptr(),
                                        url,
                                        ::std::mem::size_of::<[libc::c_char; 1024]>()
                                            as libc::c_ulong,
                                    );
                                    (*results.offset(nresults as isize))
                                        .size = (*results.offset(0 as libc::c_int as isize)).size;
                                    let ref mut fresh2 = (*results.offset(nresults as isize))
                                        .conf;
                                    *fresh2 = (*results).conf;
                                    nresults += 1;
                                    nresults;
                                }
                                start = eol.offset(1 as libc::c_int as isize);
                            }
                        }
                    }
                }
            }
        }
    }
    free(s as *mut libc::c_void);
    return nresults;
}
pub unsafe extern "C" fn search_getspeeds(
    mut results: *mut search_t,
    mut count: libc::c_int,
) -> libc::c_int {
    let delay: timespec = {
        let mut init = timespec {
            tv_sec: 0,
            tv_nsec: 10000000 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    let mut left: libc::c_int = count;
    let mut correct: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        if (*results.offset(i as isize)).speed != 0 {
            (*results.offset(i as isize))
                .speed_start_time = 0 as libc::c_int as libc::c_double;
            left -= 1;
            left;
            if (*results.offset(i as isize)).speed > 0 as libc::c_int as libc::c_long {
                correct += 1;
                correct;
            }
        }
        i += 1;
        i;
    }
    let mut running: libc::c_int = 0 as libc::c_int;
    while left > 0 as libc::c_int {
        let mut current_block_21: u64;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < count {
            match (*results.offset(i_0 as isize)).speed {
                -3 => {
                    if axel_gettime()
                        < (*results.offset(i_0 as isize)).speed_start_time
                            + (*(*results).conf).search_timeout as libc::c_double
                    {
                        current_block_21 = 13109137661213826276;
                    } else {
                        pthread_cancel(
                            *((*results.offset(i_0 as isize)).speed_thread).as_mut_ptr(),
                        );
                        current_block_21 = 14401909646449704462;
                    }
                }
                -2 => {
                    current_block_21 = 14401909646449704462;
                }
                -1 => {
                    current_block_21 = 13109137661213826276;
                }
                0 => {
                    if running >= (*(*results).conf).search_threads {
                        current_block_21 = 13109137661213826276;
                    } else {
                        (*results.offset(i_0 as isize))
                            .speed = SPEED_ACTIVE as libc::c_int as off_t;
                        (*results.offset(i_0 as isize))
                            .speed_start_time = axel_gettime();
                        if pthread_create(
                            ((*results.offset(i_0 as isize)).speed_thread).as_mut_ptr(),
                            0 as *const pthread_attr_t,
                            Some(
                                search_speedtest
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                    ) -> *mut libc::c_void,
                            ),
                            &mut *results.offset(i_0 as isize) as *mut search_t
                                as *mut libc::c_void,
                        ) == 0 as libc::c_int
                        {
                            running += 1;
                            running;
                        } else {
                            (*results.offset(i_0 as isize))
                                .speed = SPEED_PENDING as libc::c_int as off_t;
                        }
                        current_block_21 = 13109137661213826276;
                    }
                }
                _ => {
                    if (*results.offset(i_0 as isize)).speed_start_time == 0. {
                        current_block_21 = 13109137661213826276;
                    } else {
                        current_block_21 = 14401909646449704462;
                    }
                }
            }
            match current_block_21 {
                14401909646449704462 => {
                    pthread_join(
                        *((*results.offset(i_0 as isize)).speed_thread).as_mut_ptr(),
                        0 as *mut *mut libc::c_void,
                    );
                    running -= 1;
                    running;
                    left -= 1;
                    left;
                    match (*results.offset(i_0 as isize)).speed {
                        -3 | -2 => {
                            (*results.offset(i_0 as isize))
                                .speed = SPEED_DONE as libc::c_int as off_t;
                        }
                        _ => {
                            (*results.offset(i_0 as isize))
                                .speed_start_time = 0 as libc::c_int as libc::c_double;
                            if (*results.offset(i_0 as isize)).speed
                                > 0 as libc::c_int as libc::c_long
                            {
                                correct += 1;
                                correct;
                            }
                        }
                    }
                }
                _ => {}
            }
            i_0 += 1;
            i_0;
        }
        axel_sleep(delay);
    }
    return correct;
}
unsafe extern "C" fn search_speedtest(mut r: *mut libc::c_void) -> *mut libc::c_void {
    let mut results: *mut search_t = r as *mut search_t;
    let mut conn: [conn_t; 1] = [conn_t {
        conf: 0 as *mut conf_t,
        proto: 0,
        port: 0,
        proxy: 0,
        host: [0; 1024],
        dir: [0; 1024],
        file: [0; 1024],
        user: [0; 1024],
        pass: [0; 1024],
        output_filename: [0; 1024],
        ftp: [ftp_t {
            cwd: [0; 1024],
            message: 0 as *mut libc::c_char,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            data_tcp: tcp_t { fd: 0, ai_family: 0 },
            proto: 0,
            ftp_mode: 0,
            local_if: 0 as *mut libc::c_char,
        }; 1],
        http: [http_t {
            host: [0; 1024],
            auth: [0; 1024],
            request: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            headers: [abuf_t {
                p: 0 as *mut libc::c_char,
                len: 0,
            }; 1],
            port: 0,
            proto: 0,
            proxy: 0,
            proxy_auth: [0; 1024],
            firstbyte: 0,
            lastbyte: 0,
            status: 0,
            tcp: tcp_t { fd: 0, ai_family: 0 },
            local_if: 0 as *mut libc::c_char,
        }; 1],
        size: 0,
        currentbyte: 0,
        lastbyte: 0,
        tcp: 0 as *mut tcp_t,
        enabled: false,
        supported: false,
        last_transfer: 0,
        message: 0 as *mut libc::c_char,
        local_if: 0 as *mut libc::c_char,
        state: false,
        setup_thread: [0; 1],
        lock: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_list_t {
                    __prev: 0 as *mut __pthread_internal_list,
                    __next: 0 as *mut __pthread_internal_list,
                },
            },
        },
    }; 1];
    let mut oldstate: libc::c_int = 0;
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE as libc::c_int, &mut oldstate);
    pthread_setcanceltype(PTHREAD_CANCEL_ASYNCHRONOUS as libc::c_int, &mut oldstate);
    memset(
        conn.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<conn_t>() as libc::c_ulong,
    );
    let ref mut fresh3 = (*conn.as_mut_ptr()).conf;
    *fresh3 = (*results).conf;
    if conn_set(conn.as_mut_ptr(), ((*results).url).as_mut_ptr()) != 0
        && conn_init(conn.as_mut_ptr()) != 0 && conn_info(conn.as_mut_ptr()) != 0
        && (*conn.as_mut_ptr()).size == (*results).size
    {
        (*results)
            .speed = (1 as libc::c_int as libc::c_double
            + 1000 as libc::c_int as libc::c_double
                * (axel_gettime() - (*results).speed_start_time)) as off_t;
    } else {
        (*results).speed = SPEED_FAILED as libc::c_int as off_t;
    }
    conn_disconnect(conn.as_mut_ptr());
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn search_sortlist(
    mut results: *mut search_t,
    mut count: libc::c_int,
) {
    qsort(
        results as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<search_t>() as libc::c_ulong,
        Some(
            search_sortlist_qsort
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn search_sortlist_qsort(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    if (*(a as *mut search_t)).speed < 0 as libc::c_int as libc::c_long
        && (*(b as *mut search_t)).speed > 0 as libc::c_int as libc::c_long
    {
        return 1 as libc::c_int;
    }
    if (*(a as *mut search_t)).speed > 0 as libc::c_int as libc::c_long
        && (*(b as *mut search_t)).speed < 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if (*(a as *mut search_t)).speed < (*(b as *mut search_t)).speed {
        return -(1 as libc::c_int)
    } else {
        return ((*(a as *mut search_t)).speed > (*(b as *mut search_t)).speed)
            as libc::c_int
    };
}
