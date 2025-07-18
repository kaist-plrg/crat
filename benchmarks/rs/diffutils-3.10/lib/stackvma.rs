use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpagesize() -> libc::c_int;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mincore(
        __start: *mut libc::c_void,
        __len: size_t,
        __vec: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
}
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vma_struct {
    pub start: uintptr_t,
    pub end: uintptr_t,
    pub is_near_this: Option::<
        unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> libc::c_int,
    >,
    pub prev_end: uintptr_t,
}
pub type pageinfo_t = libc::c_uchar;
pub type MINCORE_ADDR_T = *mut libc::c_void;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callback_locals {
    pub address: uintptr_t,
    pub vma: *mut vma_struct,
    pub prev: uintptr_t,
    pub retval: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rofile {
    pub position: size_t,
    pub filled: size_t,
    pub eof_seen: libc::c_int,
    pub buffer: *mut libc::c_char,
    pub auxmap: *mut libc::c_char,
    pub auxmap_length: size_t,
    pub auxmap_start: uintptr_t,
    pub auxmap_end: uintptr_t,
    pub stack_allocated_buffer: [libc::c_char; 1],
}
pub type ssize_t = __ssize_t;
unsafe extern "C" fn simple_is_near_this(
    mut addr: uintptr_t,
    mut vma: *mut vma_struct,
) -> libc::c_int {
    return (((*vma).start).wrapping_sub(addr)
        <= ((*vma).start)
            .wrapping_sub((*vma).prev_end)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)) as libc::c_int;
}
unsafe extern "C" fn rof_open(
    mut rof: *mut rofile,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut pagesize_0: uintptr_t = 0;
    let mut size: size_t = 0;
    fd = open(filename, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*rof).position = 0 as libc::c_int as size_t;
    (*rof).eof_seen = 0 as libc::c_int;
    pagesize_0 = 0 as libc::c_int as uintptr_t;
    (*rof).buffer = ((*rof).stack_allocated_buffer).as_mut_ptr();
    size = ::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong;
    (*rof).auxmap = 0 as *mut libc::c_char;
    (*rof).auxmap_start = 0 as libc::c_int as uintptr_t;
    (*rof).auxmap_end = 0 as libc::c_int as uintptr_t;
    's_46: loop {
        if size > (73 as libc::c_int + 4096 as libc::c_int) as libc::c_ulong {
            let mut n: libc::c_int = read(fd, (*rof).buffer as *mut libc::c_void, size)
                as libc::c_int;
            if n < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int {
                current_block = 5340060979581474949;
            } else if n <= 0 as libc::c_int {
                current_block = 16729136981902490101;
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
                        current_block = 5340060979581474949;
                        break;
                    }
                    if n < 0 as libc::c_int {
                        current_block = 16729136981902490101;
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
                if pagesize_0 == 0 as libc::c_int as libc::c_ulong {
                    pagesize_0 = getpagesize() as uintptr_t;
                    size = pagesize_0;
                    while size
                        <= (73 as libc::c_int + 4096 as libc::c_int) as libc::c_ulong
                    {
                        size = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
                    }
                } else {
                    size = (2 as libc::c_int as libc::c_ulong).wrapping_mul(size);
                    if size == 0 as libc::c_int as libc::c_ulong {
                        current_block = 16729136981902490101;
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
                (*rof).auxmap_start = (*rof).auxmap as uintptr_t;
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
        fd = open(filename, 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            current_block = 93868743582534055;
            break;
        }
    }
    match current_block {
        16729136981902490101 => {
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
    mut valuep: *mut uintptr_t,
) -> libc::c_int {
    let mut value: uintptr_t = 0 as libc::c_int as uintptr_t;
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
unsafe extern "C" fn vma_iterate_proc(mut locals: *mut callback_locals) -> libc::c_int {
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
        let mut auxmap_start: uintptr_t = rof.auxmap_start;
        let mut auxmap_end: uintptr_t = rof.auxmap_end;
        loop {
            let mut start: uintptr_t = 0;
            let mut end: uintptr_t = 0;
            let mut c: libc::c_int = 0;
            if !(rof_scanf_lx(&mut rof, &mut start) >= 0 as libc::c_int
                && rof_getchar(&mut rof) == '-' as i32
                && rof_scanf_lx(&mut rof, &mut end) >= 0 as libc::c_int)
            {
                break;
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
                    if callback(locals, start, auxmap_start) != 0 {
                        break;
                    }
                }
                if !(auxmap_end.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < end.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                {
                    continue;
                }
                if callback(locals, auxmap_end, end) != 0 {
                    break;
                }
            } else if callback(locals, start, end) != 0 {
                break;
            }
        }
        rof_close(&mut rof);
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn vma_iterate(mut locals: *mut callback_locals) -> libc::c_int {
    let mut retval: libc::c_int = vma_iterate_proc(locals);
    if retval == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
static mut pagesize: uintptr_t = 0;
unsafe extern "C" fn init_pagesize() {
    pagesize = getpagesize() as uintptr_t;
}
unsafe extern "C" fn is_mapped(mut addr: uintptr_t) -> libc::c_int {
    let mut vec: [pageinfo_t; 1] = [0; 1];
    return (mincore(addr as MINCORE_ADDR_T, pagesize, vec.as_mut_ptr())
        >= 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn mapped_range_start(mut addr: uintptr_t) -> uintptr_t {
    let mut vec: [pageinfo_t; 1024] = [0; 1024];
    let mut stepsize: uintptr_t = ::std::mem::size_of::<[pageinfo_t; 1024]>()
        as libc::c_ulong;
    loop {
        let mut max_remaining: uintptr_t = 0;
        if addr == 0 as libc::c_int as libc::c_ulong {
            return addr;
        }
        max_remaining = addr.wrapping_div(pagesize);
        if stepsize > max_remaining {
            stepsize = max_remaining;
        }
        if mincore(
            addr.wrapping_sub(stepsize.wrapping_mul(pagesize)) as MINCORE_ADDR_T,
            stepsize.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as libc::c_int
        {
            break;
        }
        addr = (addr as libc::c_ulong).wrapping_sub(stepsize.wrapping_mul(pagesize))
            as uintptr_t as uintptr_t;
    }
    loop {
        let mut halfstepsize1: uintptr_t = 0;
        let mut halfstepsize2: uintptr_t = 0;
        if stepsize == 1 as libc::c_int as libc::c_ulong {
            return addr;
        }
        halfstepsize1 = stepsize
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        halfstepsize2 = stepsize.wrapping_div(2 as libc::c_int as libc::c_ulong);
        if mincore(
            addr.wrapping_sub(halfstepsize1.wrapping_mul(pagesize)) as MINCORE_ADDR_T,
            halfstepsize1.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as libc::c_int
        {
            stepsize = halfstepsize1;
        } else {
            addr = (addr as libc::c_ulong)
                .wrapping_sub(halfstepsize1.wrapping_mul(pagesize)) as uintptr_t
                as uintptr_t;
            stepsize = halfstepsize2;
        }
    };
}
unsafe extern "C" fn mapped_range_end(mut addr: uintptr_t) -> uintptr_t {
    let mut vec: [pageinfo_t; 1024] = [0; 1024];
    let mut stepsize: uintptr_t = ::std::mem::size_of::<[pageinfo_t; 1024]>()
        as libc::c_ulong;
    addr = (addr as libc::c_ulong).wrapping_add(pagesize) as uintptr_t as uintptr_t;
    loop {
        let mut max_remaining: uintptr_t = 0;
        if addr == 0 as libc::c_int as libc::c_ulong {
            return addr;
        }
        max_remaining = addr.wrapping_neg().wrapping_div(pagesize);
        if stepsize > max_remaining {
            stepsize = max_remaining;
        }
        if mincore(
            addr as MINCORE_ADDR_T,
            stepsize.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as libc::c_int
        {
            break;
        }
        addr = (addr as libc::c_ulong).wrapping_add(stepsize.wrapping_mul(pagesize))
            as uintptr_t as uintptr_t;
    }
    loop {
        let mut halfstepsize1: uintptr_t = 0;
        let mut halfstepsize2: uintptr_t = 0;
        if stepsize == 1 as libc::c_int as libc::c_ulong {
            return addr;
        }
        halfstepsize1 = stepsize
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        halfstepsize2 = stepsize.wrapping_div(2 as libc::c_int as libc::c_ulong);
        if mincore(
            addr as MINCORE_ADDR_T,
            halfstepsize1.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as libc::c_int
        {
            stepsize = halfstepsize1;
        } else {
            addr = (addr as libc::c_ulong)
                .wrapping_add(halfstepsize1.wrapping_mul(pagesize)) as uintptr_t
                as uintptr_t;
            stepsize = halfstepsize2;
        }
    };
}
unsafe extern "C" fn is_unmapped(
    mut addr1: uintptr_t,
    mut addr2: uintptr_t,
) -> libc::c_int {
    let mut count: uintptr_t = 0;
    let mut stepsize: uintptr_t = 0;
    addr1 = addr1.wrapping_div(pagesize).wrapping_mul(pagesize);
    addr2 = addr2
        .wrapping_div(pagesize)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(pagesize);
    count = addr2.wrapping_sub(addr1).wrapping_div(pagesize);
    stepsize = 1 as libc::c_int as uintptr_t;
    while stepsize < count {
        stepsize = (2 as libc::c_int as libc::c_ulong).wrapping_mul(stepsize);
    }
    loop {
        let mut addr_stepsize: uintptr_t = 0;
        let mut i: uintptr_t = 0;
        let mut addr: uintptr_t = 0;
        stepsize = stepsize.wrapping_div(2 as libc::c_int as libc::c_ulong);
        if stepsize == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        addr_stepsize = stepsize.wrapping_mul(pagesize);
        i = stepsize;
        addr = addr1.wrapping_add(addr_stepsize);
        while i < count {
            if is_mapped(addr) != 0 {
                return 0 as libc::c_int;
            }
            i = (i as libc::c_ulong)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(stepsize))
                as uintptr_t as uintptr_t;
            addr = (addr as libc::c_ulong)
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong).wrapping_mul(addr_stepsize),
                ) as uintptr_t as uintptr_t;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mincore_is_near_this(
    mut addr: uintptr_t,
    mut vma: *mut vma_struct,
) -> libc::c_int {
    let mut testaddr: uintptr_t = addr.wrapping_sub(((*vma).start).wrapping_sub(addr));
    if testaddr > addr {
        return 0 as libc::c_int;
    }
    return is_unmapped(
        testaddr,
        ((*vma).start).wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn mincore_get_vma(
    mut address: uintptr_t,
    mut vma: *mut vma_struct,
) -> libc::c_int {
    if pagesize == 0 as libc::c_int as libc::c_ulong {
        init_pagesize();
    }
    address = address.wrapping_div(pagesize).wrapping_mul(pagesize);
    (*vma).start = mapped_range_start(address);
    (*vma).end = mapped_range_end(address);
    (*vma)
        .is_near_this = Some(
        mincore_is_near_this
            as unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn callback(
    mut locals: *mut callback_locals,
    mut start: uintptr_t,
    mut end: uintptr_t,
) -> libc::c_int {
    if (*locals).address >= start
        && (*locals).address <= end.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        (*(*locals).vma).start = start;
        (*(*locals).vma).end = end;
        (*(*locals).vma).prev_end = (*locals).prev;
        (*locals).retval = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    (*locals).prev = end;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sigsegv_get_vma(
    mut address: uintptr_t,
    mut vma: *mut vma_struct,
) -> libc::c_int {
    let mut locals: callback_locals = callback_locals {
        address: 0,
        vma: 0 as *mut vma_struct,
        prev: 0,
        retval: 0,
    };
    locals.address = address;
    locals.vma = vma;
    locals.prev = 0 as libc::c_int as uintptr_t;
    locals.retval = -(1 as libc::c_int);
    vma_iterate(&mut locals);
    if locals.retval == 0 as libc::c_int {
        (*vma)
            .is_near_this = Some(
            simple_is_near_this
                as unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> libc::c_int,
        );
        return 0 as libc::c_int;
    }
    return mincore_get_vma(address, vma);
}
