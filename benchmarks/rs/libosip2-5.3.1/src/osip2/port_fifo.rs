use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type osip_sem;
    pub type osip_mutex;
    fn free(__ptr: *mut libc::c_void);
    static mut osip_free_func: Option::<osip_free_func_t>;
    fn osip_trace(
        fi: *const libc::c_char,
        li: libc::c_int,
        level: osip_trace_level_t,
        f: *mut FILE,
        chfr: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn osip_list_init(li: *mut osip_list_t) -> libc::c_int;
    fn osip_list_size(li: *const osip_list_t) -> libc::c_int;
    fn osip_list_add(
        li: *mut osip_list_t,
        element: *mut libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn osip_list_get(li: *const osip_list_t, pos: libc::c_int) -> *mut libc::c_void;
    fn osip_list_remove(li: *mut osip_list_t, pos: libc::c_int) -> libc::c_int;
    fn osip_sem_init(value: libc::c_uint) -> *mut osip_sem;
    fn osip_sem_destroy(sem: *mut osip_sem) -> libc::c_int;
    fn osip_sem_post(sem: *mut osip_sem) -> libc::c_int;
    fn osip_sem_wait(sem: *mut osip_sem) -> libc::c_int;
    fn osip_sem_trywait(sem: *mut osip_sem) -> libc::c_int;
    fn osip_mutex_init() -> *mut osip_mutex;
    fn osip_mutex_destroy(mut_0: *mut osip_mutex);
    fn osip_mutex_lock(mut_0: *mut osip_mutex) -> libc::c_int;
    fn osip_mutex_unlock(mut_0: *mut osip_mutex) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __node {
    pub next: *mut __node_t,
    pub element: *mut libc::c_void,
}
pub type __node_t = __node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_list {
    pub nb_elt: libc::c_int,
    pub node: *mut __node_t,
}
pub type osip_list_t = osip_list;
pub type osip_free_func_t = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type _trace_level = libc::c_uint;
pub const END_TRACE_LEVEL: _trace_level = 8;
pub const TRACE_LEVEL7: _trace_level = 7;
pub const TRACE_LEVEL6: _trace_level = 6;
pub const TRACE_LEVEL5: _trace_level = 5;
pub const TRACE_LEVEL4: _trace_level = 4;
pub const TRACE_LEVEL3: _trace_level = 3;
pub const TRACE_LEVEL2: _trace_level = 2;
pub const TRACE_LEVEL1: _trace_level = 1;
pub const TRACE_LEVEL0: _trace_level = 0;
pub type osip_trace_level_t = _trace_level;
pub type osip_fifo_state = libc::c_uint;
pub const osip_empty: osip_fifo_state = 1;
pub const osip_ok: osip_fifo_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_fifo {
    pub qislocked: *mut osip_mutex,
    pub qisempty: *mut osip_sem,
    pub queue: osip_list_t,
    pub nb_elt: libc::c_int,
    pub state: osip_fifo_state,
}
pub type osip_fifo_t = osip_fifo;
pub unsafe extern "C" fn osip_fifo_init(mut ff: *mut osip_fifo_t) {
    (*ff).qislocked = osip_mutex_init();
    (*ff).qisempty = osip_sem_init(0 as libc::c_int as libc::c_uint);
    osip_list_init(&mut (*ff).queue);
    (*ff).state = osip_empty;
}
pub unsafe extern "C" fn osip_fifo_add(
    mut ff: *mut osip_fifo_t,
    mut el: *mut libc::c_void,
) -> libc::c_int {
    osip_mutex_lock((*ff).qislocked);
    osip_list_add(&mut (*ff).queue, el, -(1 as libc::c_int));
    (*ff).state = osip_ok;
    osip_sem_post((*ff).qisempty);
    osip_mutex_unlock((*ff).qislocked);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_fifo_insert(
    mut ff: *mut osip_fifo_t,
    mut el: *mut libc::c_void,
) -> libc::c_int {
    osip_mutex_lock((*ff).qislocked);
    osip_list_add(&mut (*ff).queue, el, 0 as libc::c_int);
    (*ff).state = osip_ok;
    osip_sem_post((*ff).qisempty);
    osip_mutex_unlock((*ff).qislocked);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn osip_fifo_size(mut ff: *mut osip_fifo_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    osip_mutex_lock((*ff).qislocked);
    i = osip_list_size(&mut (*ff).queue);
    osip_mutex_unlock((*ff).qislocked);
    return i;
}
pub unsafe extern "C" fn osip_fifo_get(mut ff: *mut osip_fifo_t) -> *mut libc::c_void {
    let mut el: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = osip_sem_wait((*ff).qisempty);
    if i != 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    osip_mutex_lock((*ff).qislocked);
    if (*ff).state as libc::c_uint != osip_empty as libc::c_int as libc::c_uint {
        el = osip_list_get(&mut (*ff).queue, 0 as libc::c_int);
        osip_list_remove(&mut (*ff).queue, 0 as libc::c_int);
    } else {
        osip_trace(
            b"port_fifo.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            TRACE_LEVEL2,
            0 as *mut FILE,
            b"no element in fifo.\n\0" as *const u8 as *const libc::c_char,
        );
        osip_mutex_unlock((*ff).qislocked);
        return 0 as *mut libc::c_void;
    }
    if osip_list_size(&mut (*ff).queue) <= 0 as libc::c_int {
        (*ff).state = osip_empty;
    } else {
        (*ff).state = osip_ok;
    }
    osip_mutex_unlock((*ff).qislocked);
    return el;
}
pub unsafe extern "C" fn osip_fifo_tryget(
    mut ff: *mut osip_fifo_t,
) -> *mut libc::c_void {
    let mut el: *mut libc::c_void = 0 as *mut libc::c_void;
    if 0 as libc::c_int != osip_sem_trywait((*ff).qisempty) {
        return 0 as *mut libc::c_void;
    }
    osip_mutex_lock((*ff).qislocked);
    if (*ff).state as libc::c_uint != osip_empty as libc::c_int as libc::c_uint {
        el = osip_list_get(&mut (*ff).queue, 0 as libc::c_int);
        osip_list_remove(&mut (*ff).queue, 0 as libc::c_int);
    } else {
        osip_trace(
            b"port_fifo.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            TRACE_LEVEL7,
            0 as *mut FILE,
            b"no element in fifo.\n\0" as *const u8 as *const libc::c_char,
        );
        osip_mutex_unlock((*ff).qislocked);
        return 0 as *mut libc::c_void;
    }
    if osip_list_size(&mut (*ff).queue) <= 0 as libc::c_int {
        (*ff).state = osip_empty;
    } else {
        (*ff).state = osip_ok;
    }
    osip_mutex_unlock((*ff).qislocked);
    return el;
}
pub unsafe extern "C" fn osip_fifo_free(mut ff: *mut osip_fifo_t) {
    if ff.is_null() {
        return;
    }
    osip_mutex_destroy((*ff).qislocked);
    osip_sem_destroy((*ff).qisempty);
    if !ff.is_null() {
        if osip_free_func.is_some() {
            osip_free_func.unwrap()(ff as *mut libc::c_void);
        } else {
            free(ff as *mut libc::c_void);
        }
    }
}
