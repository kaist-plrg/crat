use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nc_alloc(
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: libc::c_int,
    pub event: *mut epoll_event,
    pub nevent: libc::c_int,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: libc::c_int,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: libc::c_int,
    pub service_str: string,
    pub service: string,
    pub source_str: string,
    pub source: string,
    pub version_str: string,
    pub version: string,
    pub uptime_str: string,
    pub timestamp_str: string,
    pub ntotal_conn_str: string,
    pub ncurr_conn_str: string,
    pub aggregate: libc::c_int,
    pub updated: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mhdr {
    pub stqh_first: *mut mbuf,
    pub stqh_last: *mut *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf {
    pub magic: uint32_t,
    pub next: C2RustUnnamed,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub stqe_next: *mut mbuf,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct instance {
    pub ctx: *mut context,
    pub log_level: libc::c_int,
    pub log_filename: *const libc::c_char,
    pub conf_filename: *const libc::c_char,
    pub stats_port: uint16_t,
    pub stats_interval: libc::c_int,
    pub stats_addr: *const libc::c_char,
    pub hostname: [libc::c_char; 256],
    pub mbuf_chunk_size: size_t,
    pub pid: pid_t,
    pub pid_filename: *const libc::c_char,
    #[bitfield(name = "pidfile", ty = "libc::c_uint", bits = "0..=0")]
    pub pidfile: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type mbuf_copy_t = Option::<
    unsafe extern "C" fn(*mut mbuf, *mut libc::c_void) -> (),
>;
static mut nfree_mbufq: uint32_t = 0;
static mut free_mbufq: mhdr = mhdr {
    stqh_first: 0 as *const mbuf as *mut mbuf,
    stqh_last: 0 as *const *mut mbuf as *mut *mut mbuf,
};
static mut mbuf_chunk_size: size_t = 0;
static mut mbuf_offset: size_t = 0;
unsafe extern "C" fn _mbuf_get() -> *mut mbuf {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    if !(free_mbufq.stqh_first).is_null() {
        mbuf = free_mbufq.stqh_first;
        nfree_mbufq = nfree_mbufq.wrapping_sub(1);
        nfree_mbufq;
        let mut oldnext: *mut *mut libc::c_void = &mut (*free_mbufq.stqh_first)
            .next
            .stqe_next as *mut *mut mbuf as *mut libc::c_void as *mut *mut libc::c_void;
        free_mbufq.stqh_first = (*free_mbufq.stqh_first).next.stqe_next;
        if (free_mbufq.stqh_first).is_null() {
            free_mbufq.stqh_last = &mut free_mbufq.stqh_first;
        }
        *oldnext = 0 as *mut libc::c_void;
    } else {
        buf = _nc_alloc(
            mbuf_chunk_size,
            b"nc_mbuf.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
        ) as *mut uint8_t;
        if buf.is_null() {
            return 0 as *mut mbuf;
        }
        mbuf = buf.offset(mbuf_offset as isize) as *mut mbuf;
        (*mbuf).magic = 0xdeadbeef as libc::c_uint;
    }
    (*mbuf).next.stqe_next = 0 as *mut mbuf;
    return mbuf;
}
pub unsafe extern "C" fn mbuf_get() -> *mut mbuf {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    mbuf = _mbuf_get();
    if mbuf.is_null() {
        return 0 as *mut mbuf;
    }
    buf = (mbuf as *mut uint8_t).offset(-(mbuf_offset as isize));
    (*mbuf).start = buf;
    (*mbuf).end = buf.offset(mbuf_offset as isize);
    (*mbuf).pos = (*mbuf).start;
    (*mbuf).last = (*mbuf).start;
    return mbuf;
}
unsafe extern "C" fn mbuf_free(mut mbuf: *mut mbuf) {
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    buf = (mbuf as *mut uint8_t).offset(-(mbuf_offset as isize));
    _nc_free(
        buf as *mut libc::c_void,
        b"nc_mbuf.c\0" as *const u8 as *const libc::c_char,
        115 as libc::c_int,
    );
    buf = 0 as *mut uint8_t;
}
pub unsafe extern "C" fn mbuf_put(mut mbuf: *mut mbuf) {
    nfree_mbufq = nfree_mbufq.wrapping_add(1);
    nfree_mbufq;
    (*mbuf).next.stqe_next = free_mbufq.stqh_first;
    if ((*mbuf).next.stqe_next).is_null() {
        free_mbufq.stqh_last = &mut (*mbuf).next.stqe_next;
    }
    free_mbufq.stqh_first = mbuf;
}
pub unsafe extern "C" fn mbuf_rewind(mut mbuf: *mut mbuf) {
    (*mbuf).pos = (*mbuf).start;
    (*mbuf).last = (*mbuf).start;
}
pub unsafe extern "C" fn mbuf_length(mut mbuf: *const mbuf) -> uint32_t {
    return ((*mbuf).last).offset_from((*mbuf).pos) as libc::c_long as uint32_t;
}
pub unsafe extern "C" fn mbuf_size(mut mbuf: *const mbuf) -> uint32_t {
    return ((*mbuf).end).offset_from((*mbuf).last) as libc::c_long as uint32_t;
}
pub unsafe extern "C" fn mbuf_data_size() -> size_t {
    return mbuf_offset;
}
pub unsafe extern "C" fn mbuf_insert(mut mhdr: *mut mhdr, mut mbuf: *mut mbuf) {
    (*mbuf).next.stqe_next = 0 as *mut mbuf;
    *(*mhdr).stqh_last = mbuf;
    (*mhdr).stqh_last = &mut (*mbuf).next.stqe_next;
}
pub unsafe extern "C" fn mbuf_remove(mut mhdr: *mut mhdr, mut mbuf: *mut mbuf) {
    if (*mhdr).stqh_first == mbuf {
        let mut oldnext: *mut *mut libc::c_void = &mut (*(*mhdr).stqh_first)
            .next
            .stqe_next as *mut *mut mbuf as *mut libc::c_void as *mut *mut libc::c_void;
        (*mhdr).stqh_first = (*(*mhdr).stqh_first).next.stqe_next;
        if ((*mhdr).stqh_first).is_null() {
            (*mhdr).stqh_last = &mut (*mhdr).stqh_first;
        }
        *oldnext = 0 as *mut libc::c_void;
    } else {
        let mut curelm: *mut mbuf = (*mhdr).stqh_first;
        while (*curelm).next.stqe_next != mbuf {
            curelm = (*curelm).next.stqe_next;
        }
        let mut oldnext_0: *mut *mut libc::c_void = &mut (*(*curelm).next.stqe_next)
            .next
            .stqe_next as *mut *mut mbuf as *mut libc::c_void as *mut *mut libc::c_void;
        (*curelm).next.stqe_next = (*(*curelm).next.stqe_next).next.stqe_next;
        if ((*curelm).next.stqe_next).is_null() {
            (*mhdr).stqh_last = &mut (*curelm).next.stqe_next;
        }
        *oldnext_0 = 0 as *mut libc::c_void;
    }
    (*mbuf).next.stqe_next = 0 as *mut mbuf;
}
pub unsafe extern "C" fn mbuf_copy(
    mut mbuf: *mut mbuf,
    mut pos: *const uint8_t,
    mut n: size_t,
) {
    if n == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    memcpy((*mbuf).last as *mut libc::c_void, pos as *const libc::c_void, n);
    (*mbuf).last = ((*mbuf).last).offset(n as isize);
}
pub unsafe extern "C" fn mbuf_split(
    mut h: *mut mhdr,
    mut pos: *mut uint8_t,
    mut cb: mbuf_copy_t,
    mut cbarg: *mut libc::c_void,
) -> *mut mbuf {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut nbuf: *mut mbuf = 0 as *mut mbuf;
    let mut size: size_t = 0;
    mbuf = if ((*h).stqh_first).is_null() {
        0 as *mut mbuf
    } else {
        ((*h).stqh_last as *mut libc::c_char)
            .offset(
                -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                    as *mut C2RustUnnamed as size_t as isize),
            ) as *mut libc::c_void as *mut mbuf
    };
    nbuf = mbuf_get();
    if nbuf.is_null() {
        return 0 as *mut mbuf;
    }
    if cb.is_some() {
        cb.unwrap()(nbuf, cbarg);
    }
    size = ((*mbuf).last).offset_from(pos) as libc::c_long as size_t;
    mbuf_copy(nbuf, pos, size);
    (*mbuf).last = pos;
    return nbuf;
}
pub unsafe extern "C" fn mbuf_init(mut nci: *const instance) {
    nfree_mbufq = 0 as libc::c_int as uint32_t;
    free_mbufq.stqh_first = 0 as *mut mbuf;
    free_mbufq.stqh_last = &mut free_mbufq.stqh_first;
    mbuf_chunk_size = (*nci).mbuf_chunk_size;
    mbuf_offset = mbuf_chunk_size
        .wrapping_sub(::std::mem::size_of::<mbuf>() as libc::c_ulong);
}
pub unsafe extern "C" fn mbuf_deinit() {
    while !(free_mbufq.stqh_first).is_null() {
        let mut mbuf: *mut mbuf = free_mbufq.stqh_first;
        mbuf_remove(&mut free_mbufq, mbuf);
        mbuf_free(mbuf);
        nfree_mbufq = nfree_mbufq.wrapping_sub(1);
        nfree_mbufq;
    }
}
