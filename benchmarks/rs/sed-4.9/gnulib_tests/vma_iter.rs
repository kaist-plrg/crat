use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpagesize() -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type vma_iterate_callback_fn = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        uintptr_t,
        uintptr_t,
        libc::c_uint,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rofile {
    pub position: size_t,
    pub filled: size_t,
    pub eof_seen: libc::c_int,
    pub buffer: *mut libc::c_char,
    pub auxmap: *mut libc::c_char,
    pub auxmap_length: size_t,
    pub auxmap_start: libc::c_ulong,
    pub auxmap_end: libc::c_ulong,
    pub stack_allocated_buffer: [libc::c_char; 1],
}
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
unsafe extern "C" fn rof_open(
    mut rof: *mut rofile,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut pagesize: libc::c_ulong = 0;
    let mut size: size_t = 0;
    fd = open(filename, 0 as libc::c_int | 0o2000000 as libc::c_int);
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*rof).position = 0 as libc::c_int as size_t;
    (*rof).eof_seen = 0 as libc::c_int;
    pagesize = 0 as libc::c_int as libc::c_ulong;
    (*rof).buffer = ((*rof).stack_allocated_buffer).as_mut_ptr();
    size = ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong;
    (*rof).auxmap = 0 as *mut libc::c_char;
    (*rof).auxmap_start = 0 as libc::c_int as libc::c_ulong;
    (*rof).auxmap_end = 0 as libc::c_int as libc::c_ulong;
    's_46: loop {
        if size > (73 as libc::c_int + 4096 as libc::c_int) as libc::c_ulong {
            let mut n: libc::c_int = read(fd, (*rof).buffer as *mut libc::c_void, size)
                as libc::c_int;
            if n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
                current_block = 727635968136152077;
            } else if n <= 0 as libc::c_int {
                current_block = 10512490070937240170;
                break;
            } else if (n + (73 as libc::c_int + 4096 as libc::c_int)) as libc::c_ulong
                <= size
            {
                (*rof).filled = n as size_t;
                loop {
                    n = read(
                        fd,
                        ((*rof).buffer).offset((*rof).filled as isize)
                            as *mut libc::c_void,
                        size.wrapping_sub((*rof).filled),
                    ) as libc::c_int;
                    if n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
                        current_block = 727635968136152077;
                        break;
                    }
                    if n < 0 as libc::c_int {
                        current_block = 10512490070937240170;
                        break 's_46;
                    } else if (n + (73 as libc::c_int + 4096 as libc::c_int))
                        as libc::c_ulong > size.wrapping_sub((*rof).filled)
                    {
                        current_block = 10043043949733653460;
                        break;
                    } else {
                        if n == 0 as libc::c_int {
                            close(fd);
                            return 0 as libc::c_int;
                        }
                        (*rof)
                            .filled = ((*rof).filled as libc::c_ulong)
                            .wrapping_add(n as libc::c_ulong) as size_t as size_t;
                    }
                }
            } else {
                current_block = 10043043949733653460;
            }
        } else {
            current_block = 10043043949733653460;
        }
        match current_block {
            10043043949733653460 => {
                if pagesize == 0 as libc::c_int as libc::c_ulong {
                    pagesize = getpagesize() as libc::c_ulong;
                    size = pagesize;
                    while size
                        <= (73 as libc::c_int + 4096 as libc::c_int) as libc::c_ulong
                    {
                        size = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
                    }
                } else {
                    size = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
                    if size == 0 as libc::c_int as libc::c_ulong {
                        current_block = 10512490070937240170;
                        break;
                    }
                    if !((*rof).auxmap).is_null() {
                        munmap((*rof).auxmap as *mut libc::c_void, (*rof).auxmap_length);
                    }
                }
                (*rof)
                    .auxmap = mmap(
                    0 as *mut libc::c_void,
                    size,
                    0x1 as libc::c_int | 0x2 as libc::c_int,
                    0x20 as libc::c_int | 0x2 as libc::c_int,
                    -(1 as libc::c_int),
                    0 as libc::c_int as __off_t,
                ) as *mut libc::c_char;
                if (*rof).auxmap
                    == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char
                {
                    close(fd);
                    return -(1 as libc::c_int);
                }
                (*rof).auxmap_length = size;
                (*rof).auxmap_start = (*rof).auxmap as libc::c_ulong;
                (*rof).auxmap_end = ((*rof).auxmap_start).wrapping_add(size);
                (*rof).buffer = (*rof).auxmap;
            }
            _ => {}
        }
        if !(lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int)
            < 0 as libc::c_int as libc::c_long)
        {
            continue;
        }
        close(fd);
        fd = open(filename, 0 as libc::c_int | 0o2000000 as libc::c_int);
        if fd < 0 as libc::c_int {
            current_block = 310044674672274345;
            break;
        }
    }
    match current_block {
        10512490070937240170 => {
            close(fd);
        }
        _ => {}
    }
    if !((*rof).auxmap).is_null() {
        munmap((*rof).auxmap as *mut libc::c_void, (*rof).auxmap_length);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn rof_peekchar(mut rof: *mut rofile) -> libc::c_int {
    if (*rof).position == (*rof).filled {
        (*rof).eof_seen = 1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return *((*rof).buffer).offset((*rof).position as isize) as libc::c_uchar
        as libc::c_int;
}
unsafe extern "C" fn rof_getchar(mut rof: *mut rofile) -> libc::c_int {
    let mut c: libc::c_int = rof_peekchar(rof);
    if c >= 0 as libc::c_int {
        (*rof).position = ((*rof).position).wrapping_add(1);
        (*rof).position;
    }
    return c;
}
unsafe extern "C" fn rof_scanf_lx(
    mut rof: *mut rofile,
    mut valuep: *mut libc::c_ulong,
) -> libc::c_int {
    let mut value: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut numdigits: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        let mut c: libc::c_int = rof_peekchar(rof);
        if c >= '0' as i32 && c <= '9' as i32 {
            value = (value << 4 as libc::c_int)
                .wrapping_add((c - '0' as i32) as libc::c_ulong);
        } else if c >= 'A' as i32 && c <= 'F' as i32 {
            value = (value << 4 as libc::c_int)
                .wrapping_add((c - 'A' as i32 + 10 as libc::c_int) as libc::c_ulong);
        } else {
            if !(c >= 'a' as i32 && c <= 'f' as i32) {
                break;
            }
            value = (value << 4 as libc::c_int)
                .wrapping_add((c - 'a' as i32 + 10 as libc::c_int) as libc::c_ulong);
        }
        rof_getchar(rof);
        numdigits = numdigits.wrapping_add(1);
        numdigits;
    }
    if numdigits == 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    *valuep = value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn rof_close(mut rof: *mut rofile) {
    if !((*rof).auxmap).is_null() {
        munmap((*rof).auxmap as *mut libc::c_void, (*rof).auxmap_length);
    }
}
unsafe extern "C" fn vma_iterate_proc(
    mut callback: vma_iterate_callback_fn,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut rof: rofile = rofile {
        position: 0,
        filled: 0,
        eof_seen: 0,
        buffer: 0 as *mut libc::c_char,
        auxmap: 0 as *mut libc::c_char,
        auxmap_length: 0,
        auxmap_start: 0,
        auxmap_end: 0,
        stack_allocated_buffer: [0; 1],
    };
    if rof_open(&mut rof, b"/proc/self/maps\0" as *const u8 as *const libc::c_char)
        >= 0 as libc::c_int
    {
        let mut auxmap_start: libc::c_ulong = rof.auxmap_start;
        let mut auxmap_end: libc::c_ulong = rof.auxmap_end;
        loop {
            let mut start: libc::c_ulong = 0;
            let mut end: libc::c_ulong = 0;
            let mut flags: libc::c_uint = 0;
            let mut c: libc::c_int = 0;
            if !(rof_scanf_lx(&mut rof, &mut start) >= 0 as libc::c_int
                && rof_getchar(&mut rof) == '-' as i32
                && rof_scanf_lx(&mut rof, &mut end) >= 0 as libc::c_int)
            {
                break;
            }
            loop {
                c = rof_getchar(&mut rof);
                if !(c == ' ' as i32) {
                    break;
                }
            }
            flags = 0 as libc::c_int as libc::c_uint;
            if c == 'r' as i32 {
                flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            }
            c = rof_getchar(&mut rof);
            if c == 'w' as i32 {
                flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
            }
            c = rof_getchar(&mut rof);
            if c == 'x' as i32 {
                flags |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
            }
            loop {
                c = rof_getchar(&mut rof);
                if !(c != -(1 as libc::c_int) && c != '\n' as i32) {
                    break;
                }
            }
            if start <= auxmap_start
                && auxmap_end.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    <= end.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                if start < auxmap_start {
                    if callback.unwrap()(data, start, auxmap_start, flags) != 0 {
                        break;
                    }
                }
                if !(auxmap_end.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < end.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                {
                    continue;
                }
                if callback.unwrap()(data, auxmap_end, end, flags) != 0 {
                    break;
                }
            } else if callback.unwrap()(data, start, end, flags) != 0 {
                break;
            }
        }
        rof_close(&mut rof);
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn vma_iterate_bsd(
    mut callback: vma_iterate_callback_fn,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn vma_iterate(
    mut callback: vma_iterate_callback_fn,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut retval: libc::c_int = vma_iterate_proc(callback, data);
    if retval == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return vma_iterate_bsd(callback, data);
}
