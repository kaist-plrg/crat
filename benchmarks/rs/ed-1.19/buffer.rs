use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    fn tmpfile() -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
    fn clear_active_list();
    fn unset_active_nodes(bp: *const line_t, ep: *const line_t);
    fn get_stdin_line(sizep: *mut libc::c_int) -> *const libc::c_char;
    fn reset_unterminated_line();
    fn unmark_unterminated_line(lp: *const line_t);
    fn show_strerror(filename: *const libc::c_char, errcode: libc::c_int);
    fn invalid_address();
    fn set_error_msg(msg: *const libc::c_char);
    fn unmark_line_node(lp: *const line_t);
    fn disable_interrupts();
    fn enable_interrupts();
    fn resize_buffer(
        buf: *mut *mut libc::c_char,
        size: *mut libc::c_int,
        min_size: libc::c_uint,
    ) -> bool_0;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _PC_2_SYMLINKS: C2RustUnnamed = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed = 12;
pub const _PC_PRIO_IO: C2RustUnnamed = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed = 10;
pub const _PC_SYNC_IO: C2RustUnnamed = 9;
pub const _PC_VDISABLE: C2RustUnnamed = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed = 6;
pub const _PC_PIPE_BUF: C2RustUnnamed = 5;
pub const _PC_PATH_MAX: C2RustUnnamed = 4;
pub const _PC_NAME_MAX: C2RustUnnamed = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed = 2;
pub const _PC_MAX_CANON: C2RustUnnamed = 1;
pub const _PC_LINK_MAX: C2RustUnnamed = 0;
pub type Bool = libc::c_uint;
pub const true_0: Bool = 1;
pub const false_0: Bool = 0;
pub type bool_0 = Bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub q_forw: *mut line,
    pub q_back: *mut line,
    pub pos: libc::c_long,
    pub len: libc::c_int,
}
pub type line_t = line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct undo_t {
    pub type_0: C2RustUnnamed_0,
    pub head: *mut line_t,
    pub tail: *mut line_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const VMOV: C2RustUnnamed_0 = 3;
pub const UMOV: C2RustUnnamed_0 = 2;
pub const UDEL: C2RustUnnamed_0 = 1;
pub const UADD: C2RustUnnamed_0 = 0;
static mut mem_msg: *const libc::c_char = b"Memory exhausted\0" as *const u8
    as *const libc::c_char;
static mut current_addr_: libc::c_int = 0 as libc::c_int;
static mut last_addr_: libc::c_int = 0 as libc::c_int;
static mut isbinary_: bool_0 = false_0;
static mut modified_: bool_0 = false_0;
static mut seek_write: bool_0 = false_0;
static mut sfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut sfpos: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut buffer_head: line_t = line_t {
    q_forw: 0 as *const line as *mut line,
    q_back: 0 as *const line as *mut line,
    pos: 0,
    len: 0,
};
static mut yank_buffer_head: line_t = line_t {
    q_forw: 0 as *const line as *mut line,
    q_back: 0 as *const line as *mut line,
    pos: 0,
    len: 0,
};
pub unsafe extern "C" fn current_addr() -> libc::c_int {
    return current_addr_;
}
pub unsafe extern "C" fn inc_current_addr() -> libc::c_int {
    current_addr_ += 1;
    if current_addr_ > last_addr_ {
        current_addr_ = last_addr_;
    }
    return current_addr_;
}
pub unsafe extern "C" fn set_current_addr(addr: libc::c_int) {
    current_addr_ = addr;
}
pub unsafe extern "C" fn last_addr() -> libc::c_int {
    return last_addr_;
}
pub unsafe extern "C" fn isbinary() -> bool_0 {
    return isbinary_;
}
pub unsafe extern "C" fn set_binary() {
    isbinary_ = true_0;
}
pub unsafe extern "C" fn modified() -> bool_0 {
    return modified_;
}
pub unsafe extern "C" fn set_modified(m: bool_0) {
    modified_ = m;
}
pub unsafe extern "C" fn inc_addr(mut addr: libc::c_int) -> libc::c_int {
    addr += 1;
    if addr > last_addr_ {
        addr = 0 as libc::c_int;
    }
    return addr;
}
pub unsafe extern "C" fn dec_addr(mut addr: libc::c_int) -> libc::c_int {
    addr -= 1;
    if addr < 0 as libc::c_int {
        addr = last_addr_;
    }
    return addr;
}
unsafe extern "C" fn link_nodes(prev: *mut line_t, next: *mut line_t) {
    (*prev).q_forw = next;
    (*next).q_back = prev;
}
unsafe extern "C" fn insert_node(lp: *mut line_t, prev: *mut line_t) {
    link_nodes(lp, (*prev).q_forw);
    link_nodes(prev, lp);
}
unsafe extern "C" fn too_many_lines() -> bool_0 {
    if last_addr_ < 2147483647 as libc::c_int - 2 as libc::c_int {
        return false_0;
    }
    set_error_msg(b"Too many lines in buffer\0" as *const u8 as *const libc::c_char);
    return true_0;
}
unsafe extern "C" fn add_line_node(lp: *mut line_t) {
    let prev: *mut line_t = search_line_node(current_addr_);
    insert_node(lp, prev);
    current_addr_ += 1;
    current_addr_;
    last_addr_ += 1;
    last_addr_;
}
unsafe extern "C" fn dup_line_node(lp: *mut line_t) -> *mut line_t {
    let p: *mut line_t = malloc(::std::mem::size_of::<line_t>() as libc::c_ulong)
        as *mut line_t;
    if p.is_null() {
        show_strerror(0 as *const libc::c_char, *__errno_location());
        set_error_msg(mem_msg);
        return 0 as *mut line_t;
    }
    if !lp.is_null() {
        (*p).pos = (*lp).pos;
        (*p).len = (*lp).len;
    }
    return p;
}
pub unsafe extern "C" fn append_lines(
    ibufpp: *mut *const libc::c_char,
    addr: libc::c_int,
    mut insert: bool_0,
    isglobal: bool_0,
) -> bool_0 {
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut up: *mut undo_t = 0 as *mut undo_t;
    current_addr_ = addr;
    while true_0 as libc::c_int != 0 {
        if isglobal as u64 == 0 {
            *ibufpp = get_stdin_line(&mut size);
            if (*ibufpp).is_null() {
                return false_0;
            }
            if size <= 0 as libc::c_int {
                return true_0;
            }
        } else {
            if **ibufpp == 0 {
                return true_0;
            }
            size = 0 as libc::c_int;
            loop {
                let fresh0 = size;
                size = size + 1;
                if !(*(*ibufpp).offset(fresh0 as isize) as libc::c_int != '\n' as i32) {
                    break;
                }
            }
        }
        if size == 2 as libc::c_int && **ibufpp as libc::c_int == '.' as i32 {
            *ibufpp = (*ibufpp).offset(size as isize);
            return true_0;
        }
        disable_interrupts();
        if insert as u64 != 0 {
            insert = false_0;
            if current_addr_ > 0 as libc::c_int {
                current_addr_ -= 1;
                current_addr_;
            }
        }
        if (put_sbuf_line(*ibufpp, size)).is_null() {
            enable_interrupts();
            return false_0;
        }
        if !up.is_null() {
            (*up).tail = search_line_node(current_addr_);
        } else {
            up = push_undo_atom(UADD as libc::c_int, current_addr_, current_addr_);
            if up.is_null() {
                enable_interrupts();
                return false_0;
            }
        }
        *ibufpp = (*ibufpp).offset(size as isize);
        modified_ = true_0;
        enable_interrupts();
    }
    panic!();
}
unsafe extern "C" fn clear_yank_buffer() {
    let mut lp: *mut line_t = yank_buffer_head.q_forw;
    disable_interrupts();
    while lp != &mut yank_buffer_head as *mut line_t {
        let p: *mut line_t = (*lp).q_forw;
        link_nodes((*lp).q_back, (*lp).q_forw);
        free(lp as *mut libc::c_void);
        lp = p;
    }
    enable_interrupts();
}
pub unsafe extern "C" fn close_sbuf() -> bool_0 {
    clear_yank_buffer();
    clear_undo_stack();
    if !sfp.is_null() {
        if fclose(sfp) != 0 as libc::c_int {
            show_strerror(0 as *const libc::c_char, *__errno_location());
            set_error_msg(
                b"Cannot close temp file\0" as *const u8 as *const libc::c_char,
            );
            return false_0;
        }
        sfp = 0 as *mut FILE;
    }
    sfpos = 0 as libc::c_int as libc::c_long;
    seek_write = false_0;
    return true_0;
}
pub unsafe extern "C" fn copy_lines(
    first_addr: libc::c_int,
    second_addr: libc::c_int,
    addr: libc::c_int,
) -> bool_0 {
    let mut lp: *mut line_t = 0 as *mut line_t;
    let mut np: *mut line_t = search_line_node(first_addr);
    let mut up: *mut undo_t = 0 as *mut undo_t;
    let mut n: libc::c_int = second_addr - first_addr + 1 as libc::c_int;
    let mut m: libc::c_int = 0 as libc::c_int;
    current_addr_ = addr;
    if addr >= first_addr && addr < second_addr {
        n = addr - first_addr + 1 as libc::c_int;
        m = second_addr - addr;
    }
    while n > 0 as libc::c_int {
        loop {
            let fresh1 = n;
            n = n - 1;
            if !(fresh1 > 0 as libc::c_int) {
                break;
            }
            if too_many_lines() as u64 != 0 {
                return false_0;
            }
            disable_interrupts();
            lp = dup_line_node(np);
            if lp.is_null() {
                enable_interrupts();
                return false_0;
            }
            add_line_node(lp);
            if !up.is_null() {
                (*up).tail = lp;
            } else {
                up = push_undo_atom(UADD as libc::c_int, current_addr_, current_addr_);
                if up.is_null() {
                    enable_interrupts();
                    return false_0;
                }
            }
            modified_ = true_0;
            enable_interrupts();
            np = (*np).q_forw;
        }
        n = m;
        m = 0 as libc::c_int;
        np = search_line_node(current_addr_ + 1 as libc::c_int);
    }
    return true_0;
}
pub unsafe extern "C" fn delete_lines(
    from: libc::c_int,
    to: libc::c_int,
    isglobal: bool_0,
) -> bool_0 {
    let mut n: *mut line_t = 0 as *mut line_t;
    let mut p: *mut line_t = 0 as *mut line_t;
    if yank_lines(from, to) as u64 == 0 {
        return false_0;
    }
    disable_interrupts();
    if (push_undo_atom(UDEL as libc::c_int, from, to)).is_null() {
        enable_interrupts();
        return false_0;
    }
    n = search_line_node(inc_addr(to));
    p = search_line_node(from - 1 as libc::c_int);
    if isglobal as u64 != 0 {
        unset_active_nodes((*p).q_forw, n);
    }
    link_nodes(p, n);
    last_addr_ -= to - from + 1 as libc::c_int;
    current_addr_ = if from < last_addr_ { from } else { last_addr_ };
    modified_ = true_0;
    enable_interrupts();
    return true_0;
}
pub unsafe extern "C" fn get_line_node_addr(lp: *const line_t) -> libc::c_int {
    let mut p: *const line_t = &mut buffer_head;
    let mut addr: libc::c_int = 0 as libc::c_int;
    while p != lp
        && {
            p = (*p).q_forw;
            p != &mut buffer_head as *mut line_t as *const line_t
        }
    {
        addr += 1;
        addr;
    }
    if addr != 0 && p == &mut buffer_head as *mut line_t as *const line_t {
        invalid_address();
        return -(1 as libc::c_int);
    }
    return addr;
}
pub unsafe extern "C" fn get_sbuf_line(lp: *const line_t) -> *mut libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    if lp == &mut buffer_head as *mut line_t as *const line_t {
        return 0 as *mut libc::c_char;
    }
    seek_write = true_0;
    if sfpos != (*lp).pos {
        sfpos = (*lp).pos;
        if fseek(sfp, sfpos, 0 as libc::c_int) != 0 as libc::c_int {
            show_strerror(0 as *const libc::c_char, *__errno_location());
            set_error_msg(
                b"Cannot seek temp file\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut libc::c_char;
        }
    }
    len = (*lp).len;
    if resize_buffer(&mut buf, &mut bufsz, (len + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return 0 as *mut libc::c_char;
    }
    if fread(
        buf as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len as libc::c_ulong,
        sfp,
    ) as libc::c_int != len
    {
        show_strerror(0 as *const libc::c_char, *__errno_location());
        set_error_msg(b"Cannot read temp file\0" as *const u8 as *const libc::c_char);
        return 0 as *mut libc::c_char;
    }
    sfpos += len as libc::c_long;
    *buf.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return buf;
}
pub unsafe extern "C" fn init_buffers() -> bool_0 {
    setvbuf(stdin, 0 as *mut libc::c_char, 2 as libc::c_int, 0 as libc::c_int as size_t);
    if open_sbuf() as u64 == 0 {
        return false_0;
    }
    link_nodes(&mut buffer_head, &mut buffer_head);
    link_nodes(&mut yank_buffer_head, &mut yank_buffer_head);
    return true_0;
}
pub unsafe extern "C" fn join_lines(
    from: libc::c_int,
    to: libc::c_int,
    isglobal: bool_0,
) -> bool_0 {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 0 as libc::c_int;
    let ep: *mut line_t = search_line_node(inc_addr(to));
    let mut bp: *mut line_t = search_line_node(from);
    while bp != ep {
        let s: *const libc::c_char = get_sbuf_line(bp);
        if s.is_null()
            || resize_buffer(&mut buf, &mut bufsz, (size + (*bp).len) as libc::c_uint)
                as u64 == 0
        {
            return false_0;
        }
        memcpy(
            buf.offset(size as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            (*bp).len as libc::c_ulong,
        );
        size += (*bp).len;
        bp = (*bp).q_forw;
    }
    if resize_buffer(&mut buf, &mut bufsz, (size + 2 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return false_0;
    }
    let fresh2 = size;
    size = size + 1;
    *buf.offset(fresh2 as isize) = '\n' as i32 as libc::c_char;
    let fresh3 = size;
    size = size + 1;
    *buf.offset(fresh3 as isize) = 0 as libc::c_int as libc::c_char;
    if delete_lines(from, to, isglobal) as u64 == 0 {
        return false_0;
    }
    current_addr_ = from - 1 as libc::c_int;
    disable_interrupts();
    if (put_sbuf_line(buf, size)).is_null()
        || (push_undo_atom(UADD as libc::c_int, current_addr_, current_addr_)).is_null()
    {
        enable_interrupts();
        return false_0;
    }
    modified_ = true_0;
    enable_interrupts();
    return true_0;
}
pub unsafe extern "C" fn move_lines(
    first_addr: libc::c_int,
    second_addr: libc::c_int,
    addr: libc::c_int,
    isglobal: bool_0,
) -> bool_0 {
    let mut b1: *mut line_t = 0 as *mut line_t;
    let mut a1: *mut line_t = 0 as *mut line_t;
    let mut b2: *mut line_t = 0 as *mut line_t;
    let mut a2: *mut line_t = 0 as *mut line_t;
    let mut n: libc::c_int = inc_addr(second_addr);
    let mut p: libc::c_int = first_addr - 1 as libc::c_int;
    disable_interrupts();
    if addr == first_addr - 1 as libc::c_int || addr == second_addr {
        a2 = search_line_node(n);
        b2 = search_line_node(p);
        current_addr_ = second_addr;
    } else if (push_undo_atom(UMOV as libc::c_int, p, n)).is_null()
        || (push_undo_atom(UMOV as libc::c_int, addr, inc_addr(addr))).is_null()
    {
        enable_interrupts();
        return false_0;
    } else {
        a1 = search_line_node(n);
        if addr < first_addr {
            b1 = search_line_node(p);
            b2 = search_line_node(addr);
        } else {
            b2 = search_line_node(addr);
            b1 = search_line_node(p);
        }
        a2 = (*b2).q_forw;
        link_nodes(b2, (*b1).q_forw);
        link_nodes((*a1).q_back, a2);
        link_nodes(b1, a1);
        current_addr_ = addr
            + (if addr < first_addr {
                second_addr - first_addr + 1 as libc::c_int
            } else {
                0 as libc::c_int
            });
    }
    if isglobal as u64 != 0 {
        unset_active_nodes((*b2).q_forw, a2);
    }
    modified_ = true_0;
    enable_interrupts();
    return true_0;
}
pub unsafe extern "C" fn open_sbuf() -> bool_0 {
    isbinary_ = false_0;
    reset_unterminated_line();
    sfp = tmpfile();
    if sfp.is_null() {
        show_strerror(0 as *const libc::c_char, *__errno_location());
        set_error_msg(b"Cannot open temp file\0" as *const u8 as *const libc::c_char);
        return false_0;
    }
    return true_0;
}
pub unsafe extern "C" fn path_max(mut filename: *const libc::c_char) -> libc::c_int {
    let mut result: libc::c_long = 0;
    if filename.is_null() {
        filename = b"/\0" as *const u8 as *const libc::c_char;
    }
    *__errno_location() = 0 as libc::c_int;
    result = pathconf(filename, _PC_PATH_MAX as libc::c_int);
    if result < 0 as libc::c_int as libc::c_long {
        if *__errno_location() != 0 {
            result = 256 as libc::c_int as libc::c_long;
        } else {
            result = 1024 as libc::c_int as libc::c_long;
        }
    } else if result < 256 as libc::c_int as libc::c_long {
        result = 256 as libc::c_int as libc::c_long;
    }
    return result as libc::c_int;
}
pub unsafe extern "C" fn put_lines(addr: libc::c_int) -> bool_0 {
    let mut up: *mut undo_t = 0 as *mut undo_t;
    let mut p: *mut line_t = 0 as *mut line_t;
    let mut lp: *mut line_t = yank_buffer_head.q_forw;
    if lp == &mut yank_buffer_head as *mut line_t {
        set_error_msg(b"Nothing to put\0" as *const u8 as *const libc::c_char);
        return false_0;
    }
    current_addr_ = addr;
    while lp != &mut yank_buffer_head as *mut line_t {
        if too_many_lines() as u64 != 0 {
            return false_0;
        }
        disable_interrupts();
        p = dup_line_node(lp);
        if p.is_null() {
            enable_interrupts();
            return false_0;
        }
        add_line_node(p);
        if !up.is_null() {
            (*up).tail = p;
        } else {
            up = push_undo_atom(UADD as libc::c_int, current_addr_, current_addr_);
            if up.is_null() {
                enable_interrupts();
                return false_0;
            }
        }
        modified_ = true_0;
        lp = (*lp).q_forw;
        enable_interrupts();
    }
    return true_0;
}
pub unsafe extern "C" fn put_sbuf_line(
    buf: *const libc::c_char,
    size: libc::c_int,
) -> *const libc::c_char {
    let p: *const libc::c_char = memchr(
        buf as *const libc::c_void,
        '\n' as i32,
        size as libc::c_ulong,
    ) as *const libc::c_char;
    if p.is_null() {
        set_error_msg(
            b"internal error: unterminated line passed to put_sbuf_line\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *const libc::c_char;
    }
    let len: libc::c_int = p.offset_from(buf) as libc::c_long as libc::c_int;
    if too_many_lines() as u64 != 0 {
        return 0 as *const libc::c_char;
    }
    if seek_write as u64 != 0 {
        if fseek(sfp, 0 as libc::c_long, 2 as libc::c_int) != 0 as libc::c_int {
            show_strerror(0 as *const libc::c_char, *__errno_location());
            set_error_msg(
                b"Cannot seek temp file\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *const libc::c_char;
        }
        sfpos = ftell(sfp);
        seek_write = false_0;
    }
    if fwrite(
        buf as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len as libc::c_ulong,
        sfp,
    ) as libc::c_int != len
    {
        sfpos = -(1 as libc::c_int) as libc::c_long;
        show_strerror(0 as *const libc::c_char, *__errno_location());
        set_error_msg(b"Cannot write temp file\0" as *const u8 as *const libc::c_char);
        return 0 as *const libc::c_char;
    }
    let mut lp: *mut line_t = dup_line_node(0 as *mut line_t);
    if lp.is_null() {
        return 0 as *const libc::c_char;
    }
    (*lp).pos = sfpos;
    (*lp).len = len;
    add_line_node(lp);
    sfpos += len as libc::c_long;
    return p.offset(1 as libc::c_int as isize);
}
pub unsafe extern "C" fn search_line_node(addr: libc::c_int) -> *mut line_t {
    static mut lp: *mut line_t = unsafe { &buffer_head as *const line_t as *mut line_t };
    static mut o_addr: libc::c_int = 0 as libc::c_int;
    disable_interrupts();
    if o_addr < addr {
        if o_addr + last_addr_ >= 2 as libc::c_int * addr {
            while o_addr < addr {
                o_addr += 1;
                o_addr;
                lp = (*lp).q_forw;
            }
        } else {
            lp = buffer_head.q_back;
            o_addr = last_addr_;
            while o_addr > addr {
                o_addr -= 1;
                o_addr;
                lp = (*lp).q_back;
            }
        }
    } else if o_addr <= 2 as libc::c_int * addr {
        while o_addr > addr {
            o_addr -= 1;
            o_addr;
            lp = (*lp).q_back;
        }
    } else {
        lp = &mut buffer_head;
        o_addr = 0 as libc::c_int;
        while o_addr < addr {
            o_addr += 1;
            o_addr;
            lp = (*lp).q_forw;
        }
    }
    enable_interrupts();
    return lp;
}
pub unsafe extern "C" fn yank_lines(from: libc::c_int, to: libc::c_int) -> bool_0 {
    let ep: *mut line_t = search_line_node(inc_addr(to));
    let mut bp: *mut line_t = search_line_node(from);
    let mut lp: *mut line_t = &mut yank_buffer_head;
    let mut p: *mut line_t = 0 as *mut line_t;
    clear_yank_buffer();
    while bp != ep {
        disable_interrupts();
        p = dup_line_node(bp);
        if p.is_null() {
            enable_interrupts();
            return false_0;
        }
        insert_node(p, lp);
        bp = (*bp).q_forw;
        lp = p;
        enable_interrupts();
    }
    return true_0;
}
static mut ustack: *mut undo_t = 0 as *const undo_t as *mut undo_t;
static mut usize: libc::c_int = 0 as libc::c_int;
static mut u_idx: libc::c_int = 0 as libc::c_int;
static mut u_current_addr: libc::c_int = -(1 as libc::c_int);
static mut u_last_addr: libc::c_int = -(1 as libc::c_int);
static mut u_modified: bool_0 = false_0;
pub unsafe extern "C" fn clear_undo_stack() {
    loop {
        let fresh4 = u_idx;
        u_idx = u_idx - 1;
        if !(fresh4 != 0) {
            break;
        }
        if (*ustack.offset(u_idx as isize)).type_0 as libc::c_uint
            == UDEL as libc::c_int as libc::c_uint
        {
            let ep: *mut line_t = (*(*ustack.offset(u_idx as isize)).tail).q_forw;
            let mut bp: *mut line_t = (*ustack.offset(u_idx as isize)).head;
            while bp != ep {
                let lp: *mut line_t = (*bp).q_forw;
                unmark_line_node(bp);
                unmark_unterminated_line(bp);
                free(bp as *mut libc::c_void);
                bp = lp;
            }
        }
    }
    u_idx = 0 as libc::c_int;
    u_current_addr = current_addr_;
    u_last_addr = last_addr_;
    u_modified = modified_;
}
pub unsafe extern "C" fn reset_undo_state() {
    clear_undo_stack();
    u_last_addr = -(1 as libc::c_int);
    u_current_addr = u_last_addr;
    u_modified = false_0;
}
unsafe extern "C" fn free_undo_stack() {
    if !ustack.is_null() {
        clear_undo_stack();
        free(ustack as *mut libc::c_void);
        ustack = 0 as *mut undo_t;
        u_idx = 0 as libc::c_int;
        usize = u_idx;
        u_last_addr = -(1 as libc::c_int);
        u_current_addr = u_last_addr;
    }
}
pub unsafe extern "C" fn push_undo_atom(
    type_0: libc::c_int,
    from: libc::c_int,
    to: libc::c_int,
) -> *mut undo_t {
    let min_size: libc::c_uint = ((u_idx + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<undo_t>() as libc::c_ulong) as libc::c_uint;
    disable_interrupts();
    if (usize as libc::c_uint) < min_size {
        if min_size >= 2147483647 as libc::c_int as libc::c_uint {
            set_error_msg(b"Undo stack too long\0" as *const u8 as *const libc::c_char);
            free_undo_stack();
            enable_interrupts();
            return 0 as *mut undo_t;
        }
        let new_size: libc::c_int = (if min_size < 512 as libc::c_int as libc::c_uint {
            512 as libc::c_int as libc::c_uint
        } else if min_size
            >= (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_uint
        {
            (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        } else {
            min_size
                .wrapping_div(512 as libc::c_int as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint)
        }) as libc::c_int;
        let mut new_buf: *mut libc::c_void = 0 as *mut libc::c_void;
        if !ustack.is_null() {
            new_buf = realloc(ustack as *mut libc::c_void, new_size as libc::c_ulong);
        } else {
            new_buf = malloc(new_size as libc::c_ulong);
        }
        if new_buf.is_null() {
            show_strerror(0 as *const libc::c_char, *__errno_location());
            set_error_msg(mem_msg);
            free_undo_stack();
            enable_interrupts();
            return 0 as *mut undo_t;
        }
        usize = new_size;
        ustack = new_buf as *mut undo_t;
    }
    (*ustack.offset(u_idx as isize)).type_0 = type_0 as C2RustUnnamed_0;
    let ref mut fresh5 = (*ustack.offset(u_idx as isize)).tail;
    *fresh5 = search_line_node(to);
    let ref mut fresh6 = (*ustack.offset(u_idx as isize)).head;
    *fresh6 = search_line_node(from);
    enable_interrupts();
    let fresh7 = u_idx;
    u_idx = u_idx + 1;
    return ustack.offset(fresh7 as isize);
}
pub unsafe extern "C" fn undo(isglobal: bool_0) -> bool_0 {
    let mut n: libc::c_int = 0;
    let o_current_addr: libc::c_int = current_addr_;
    let o_last_addr: libc::c_int = last_addr_;
    let o_modified: bool_0 = modified_;
    if u_idx <= 0 as libc::c_int || u_current_addr < 0 as libc::c_int
        || u_last_addr < 0 as libc::c_int
    {
        set_error_msg(b"Nothing to undo\0" as *const u8 as *const libc::c_char);
        return false_0;
    }
    search_line_node(0 as libc::c_int);
    disable_interrupts();
    n = u_idx - 1 as libc::c_int;
    while n >= 0 as libc::c_int {
        match (*ustack.offset(n as isize)).type_0 as libc::c_uint {
            0 => {
                link_nodes(
                    (*(*ustack.offset(n as isize)).head).q_back,
                    (*(*ustack.offset(n as isize)).tail).q_forw,
                );
            }
            1 => {
                link_nodes(
                    (*(*ustack.offset(n as isize)).head).q_back,
                    (*ustack.offset(n as isize)).head,
                );
                link_nodes(
                    (*ustack.offset(n as isize)).tail,
                    (*(*ustack.offset(n as isize)).tail).q_forw,
                );
            }
            2 | 3 => {
                link_nodes(
                    (*ustack.offset((n - 1 as libc::c_int) as isize)).head,
                    (*(*ustack.offset(n as isize)).head).q_forw,
                );
                link_nodes(
                    (*(*ustack.offset(n as isize)).tail).q_back,
                    (*ustack.offset((n - 1 as libc::c_int) as isize)).tail,
                );
                link_nodes(
                    (*ustack.offset(n as isize)).head,
                    (*ustack.offset(n as isize)).tail,
                );
                n -= 1;
                n;
            }
            _ => {}
        }
        let ref mut fresh8 = (*ustack.offset(n as isize)).type_0;
        *fresh8 = ::std::mem::transmute::<
            libc::c_uint,
            C2RustUnnamed_0,
        >(*fresh8 as libc::c_uint ^ 1 as libc::c_int as libc::c_uint);
        n -= 1;
        n;
    }
    n = 0 as libc::c_int;
    while 2 as libc::c_int * n < u_idx - 1 as libc::c_int {
        let mut tmp: undo_t = *ustack.offset(n as isize);
        *ustack
            .offset(
                n as isize,
            ) = *ustack.offset((u_idx - 1 as libc::c_int - n) as isize);
        *ustack.offset((u_idx - 1 as libc::c_int - n) as isize) = tmp;
        n += 1;
        n;
    }
    if isglobal as u64 != 0 {
        clear_active_list();
    }
    current_addr_ = u_current_addr;
    u_current_addr = o_current_addr;
    last_addr_ = u_last_addr;
    u_last_addr = o_last_addr;
    modified_ = u_modified;
    u_modified = o_modified;
    enable_interrupts();
    return true_0;
}
