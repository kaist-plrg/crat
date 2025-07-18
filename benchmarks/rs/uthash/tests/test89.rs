use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type clock_time_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netq_t {
    pub next: *mut netq_t,
    pub t: clock_time_t,
}
pub unsafe extern "C" fn dump_queue(mut queue: *mut netq_t) {
    let mut p: *mut netq_t = 0 as *mut netq_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    if queue.is_null() {
        printf(b"(null)\n\0" as *const u8 as *const libc::c_char);
    } else {
        p = queue;
        while !p.is_null() {
            let fresh0 = i;
            i = i + 1;
            printf(
                b"node #%d, timeout: %d\n\0" as *const u8 as *const libc::c_char,
                fresh0,
                (*p).t,
            );
            p = (*p).next;
        }
    };
}
pub unsafe extern "C" fn netq_insert_node(
    mut queue: *mut *mut netq_t,
    mut node: *mut netq_t,
) -> libc::c_int {
    let mut p: *mut netq_t = *queue;
    while !p.is_null() && (*p).t <= (*node).t {
        p = (*p).next;
    }
    if !p.is_null() {
        if !p.is_null() {
            let mut _tmp: *mut netq_t = 0 as *mut netq_t;
            if !(*queue).is_null() {} else {
                __assert_fail(
                    b"(*queue) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test89.c\0" as *const u8 as *const libc::c_char,
                    39 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"int netq_insert_node(netq_t **, netq_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_2061: {
                if !(*queue).is_null() {} else {
                    __assert_fail(
                        b"(*queue) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test89.c\0" as *const u8 as *const libc::c_char,
                        39 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 42],
                            &[libc::c_char; 42],
                        >(b"int netq_insert_node(netq_t **, netq_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if !node.is_null() {} else {
                __assert_fail(
                    b"(node) != NULL\0" as *const u8 as *const libc::c_char,
                    b"test89.c\0" as *const u8 as *const libc::c_char,
                    39 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"int netq_insert_node(netq_t **, netq_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_2012: {
                if !node.is_null() {} else {
                    __assert_fail(
                        b"(node) != NULL\0" as *const u8 as *const libc::c_char,
                        b"test89.c\0" as *const u8 as *const libc::c_char,
                        39 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 42],
                            &[libc::c_char; 42],
                        >(b"int netq_insert_node(netq_t **, netq_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*node).next = p;
            if *queue == p {
                *queue = node;
            } else {
                _tmp = *queue;
                while !((*_tmp).next).is_null() && (*_tmp).next != p {
                    _tmp = (*_tmp).next;
                }
                if !((*_tmp).next).is_null() {
                    (*_tmp).next = node;
                }
            }
        } else {
            let mut _tmp_0: *mut netq_t = 0 as *mut netq_t;
            (*node).next = 0 as *mut netq_t;
            if !(*queue).is_null() {
                _tmp_0 = *queue;
                while !((*_tmp_0).next).is_null() {
                    _tmp_0 = (*_tmp_0).next;
                }
                (*_tmp_0).next = node;
            } else {
                *queue = node;
            }
        }
    } else {
        let mut _tmp_1: *mut netq_t = 0 as *mut netq_t;
        (*node).next = 0 as *mut netq_t;
        if !(*queue).is_null() {
            _tmp_1 = *queue;
            while !((*_tmp_1).next).is_null() {
                _tmp_1 = (*_tmp_1).next;
            }
            (*_tmp_1).next = node;
        } else {
            *queue = node;
        }
    }
    return 1 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut nq: *mut netq_t = 0 as *mut netq_t;
    let mut i: size_t = 0;
    let mut timestamps: [clock_time_t; 5] = [
        300 as libc::c_int,
        100 as libc::c_int,
        200 as libc::c_int,
        400 as libc::c_int,
        500 as libc::c_int,
    ];
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[clock_time_t; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<clock_time_t>() as libc::c_ulong)
    {
        let mut node: *mut netq_t = malloc(
            ::std::mem::size_of::<netq_t>() as libc::c_ulong,
        ) as *mut netq_t;
        memset(
            node as *mut libc::c_void,
            '\0' as i32,
            ::std::mem::size_of::<netq_t>() as libc::c_ulong,
        );
        (*node).t = timestamps[i as usize];
        if netq_insert_node(&mut nq, node) != 1 as libc::c_int {
            puts(b"ERROR\0" as *const u8 as *const libc::c_char);
        }
        i = i.wrapping_add(1);
        i;
    }
    dump_queue(nq);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
