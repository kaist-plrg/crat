use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut exit_failure: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type uintmax_t = __uintmax_t;
pub type __uintmax_t = libc::c_ulong;
unsafe extern "C" fn call_chunkfun(
    mut h: *mut obstack,
    mut size: size_t,
) -> *mut libc::c_void {
    if (*h).use_extra_arg() != 0 {
        return ((*h).chunkfun.extra).unwrap()((*h).extra_arg, size)
    } else {
        return ((*h).chunkfun.plain).unwrap()(size)
    };
}
unsafe extern "C" fn call_freefun(
    mut h: *mut obstack,
    mut old_chunk: *mut libc::c_void,
) {
    if (*h).use_extra_arg() != 0 {
        ((*h).freefun.extra).unwrap()((*h).extra_arg, old_chunk);
    } else {
        ((*h).freefun.plain).unwrap()(old_chunk);
    };
}
unsafe extern "C" fn _obstack_begin_worker(
    mut h: *mut obstack,
    mut size: size_t,
    mut alignment: size_t,
) -> libc::c_int {
    let mut chunk: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    if alignment == 0 as libc::c_int as libc::c_ulong {
        alignment = if ::std::mem::align_of::<f128::f128>() as libc::c_ulong
            > (if ::std::mem::align_of::<uintmax_t>() as libc::c_ulong
                > ::std::mem::align_of::<*mut libc::c_void>() as libc::c_ulong
            {
                ::std::mem::align_of::<uintmax_t>() as libc::c_ulong
            } else {
                ::std::mem::align_of::<*mut libc::c_void>() as libc::c_ulong
            })
        {
            ::std::mem::align_of::<f128::f128>() as libc::c_ulong
        } else if ::std::mem::align_of::<uintmax_t>() as libc::c_ulong
            > ::std::mem::align_of::<*mut libc::c_void>() as libc::c_ulong
        {
            ::std::mem::align_of::<uintmax_t>() as libc::c_ulong
        } else {
            ::std::mem::align_of::<*mut libc::c_void>() as libc::c_ulong
        };
    }
    if size == 0 as libc::c_int as libc::c_ulong {
        let mut extra: libc::c_int = (((12 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                if ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                    > (if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                        > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    } else {
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    })
                {
                    ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                } else {
                    if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                        > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    } else {
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    }
                },
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(if ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                > (if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                })
            {
                ::std::mem::size_of::<f128::f128>() as libc::c_ulong
            } else {
                if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                }
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                if ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                    > (if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                        > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    } else {
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    })
                {
                    ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                } else {
                    if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                        > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    } else {
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    }
                },
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(if ::std::mem::size_of::<f128::f128>() as libc::c_ulong
                > (if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                })
            {
                ::std::mem::size_of::<f128::f128>() as libc::c_ulong
            } else {
                if ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                    > ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    ::std::mem::size_of::<uintmax_t>() as libc::c_ulong
                } else {
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                }
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        size = (4096 as libc::c_int - extra) as size_t;
    }
    (*h).chunk_size = size;
    (*h).alignment_mask = alignment.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*h).chunk = call_chunkfun(h, (*h).chunk_size) as *mut _obstack_chunk;
    chunk = (*h).chunk;
    if chunk.is_null() {
        (Some(obstack_alloc_failed_handler.unwrap())).unwrap()();
    }
    (*h)
        .object_base = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
    {
        chunk as *mut libc::c_char
    } else {
        0 as *mut libc::c_char
    })
        .offset(
            ((((*chunk).contents)
                .as_mut_ptr()
                .offset_from(
                    if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        chunk as *mut libc::c_char
                    } else {
                        0 as *mut libc::c_char
                    },
                ) as libc::c_long as libc::c_ulong)
                .wrapping_add(alignment.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                & !alignment.wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        );
    (*h).next_free = (*h).object_base;
    (*chunk).limit = (chunk as *mut libc::c_char).offset((*h).chunk_size as isize);
    (*h).chunk_limit = (*chunk).limit;
    (*chunk).prev = 0 as *mut _obstack_chunk;
    (*h).set_maybe_empty_object(0 as libc::c_int as libc::c_uint);
    (*h).set_alloc_failed(0 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn _obstack_begin(
    mut h: *mut obstack,
    mut size: size_t,
    mut alignment: size_t,
    mut chunkfun: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut freefun: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> libc::c_int {
    (*h).chunkfun.plain = chunkfun;
    (*h).freefun.plain = freefun;
    (*h).set_use_extra_arg(0 as libc::c_int as libc::c_uint);
    return _obstack_begin_worker(h, size, alignment);
}
pub unsafe extern "C" fn _obstack_begin_1(
    mut h: *mut obstack,
    mut size: size_t,
    mut alignment: size_t,
    mut chunkfun: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    mut freefun: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    (*h).chunkfun.extra = chunkfun;
    (*h).freefun.extra = freefun;
    (*h).extra_arg = arg;
    (*h).set_use_extra_arg(1 as libc::c_int as libc::c_uint);
    return _obstack_begin_worker(h, size, alignment);
}
pub unsafe extern "C" fn _obstack_newchunk(mut h: *mut obstack, mut length: size_t) {
    let mut old_chunk: *mut _obstack_chunk = (*h).chunk;
    let mut new_chunk: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut obj_size: size_t = ((*h).next_free).offset_from((*h).object_base)
        as libc::c_long as size_t;
    let mut object_base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sum1: size_t = obj_size.wrapping_add(length);
    let mut sum2: size_t = sum1.wrapping_add((*h).alignment_mask);
    let mut new_size: size_t = sum2
        .wrapping_add(obj_size >> 3 as libc::c_int)
        .wrapping_add(100 as libc::c_int as libc::c_ulong);
    if new_size < sum2 {
        new_size = sum2;
    }
    if new_size < (*h).chunk_size {
        new_size = (*h).chunk_size;
    }
    if obj_size <= sum1 && sum1 <= sum2 {
        new_chunk = call_chunkfun(h, new_size) as *mut _obstack_chunk;
    }
    if new_chunk.is_null() {
        (Some(obstack_alloc_failed_handler.unwrap())).unwrap()();
    }
    (*h).chunk = new_chunk;
    (*new_chunk).prev = old_chunk;
    (*h).chunk_limit = (new_chunk as *mut libc::c_char).offset(new_size as isize);
    (*new_chunk).limit = (*h).chunk_limit;
    object_base = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
    {
        new_chunk as *mut libc::c_char
    } else {
        0 as *mut libc::c_char
    })
        .offset(
            ((((*new_chunk).contents)
                .as_mut_ptr()
                .offset_from(
                    if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                        < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    {
                        new_chunk as *mut libc::c_char
                    } else {
                        0 as *mut libc::c_char
                    },
                ) as libc::c_long as libc::c_ulong)
                .wrapping_add((*h).alignment_mask) & !(*h).alignment_mask) as isize,
        );
    memcpy(
        object_base as *mut libc::c_void,
        (*h).object_base as *const libc::c_void,
        obj_size,
    );
    if (*h).maybe_empty_object() == 0
        && (*h).object_base
            == (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                old_chunk as *mut libc::c_char
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*old_chunk).contents)
                        .as_mut_ptr()
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                old_chunk as *mut libc::c_char
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*h).alignment_mask) & !(*h).alignment_mask)
                        as isize,
                )
    {
        (*new_chunk).prev = (*old_chunk).prev;
        call_freefun(h, old_chunk as *mut libc::c_void);
    }
    (*h).object_base = object_base;
    (*h).next_free = ((*h).object_base).offset(obj_size as isize);
    (*h).set_maybe_empty_object(0 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn _obstack_allocated_p(
    mut h: *mut obstack,
    mut obj: *mut libc::c_void,
) -> libc::c_int {
    let mut lp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut plp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    lp = (*h).chunk;
    while !lp.is_null()
        && (lp as *mut libc::c_void >= obj || ((*lp).limit as *mut libc::c_void) < obj)
    {
        plp = (*lp).prev;
        lp = plp;
    }
    return (lp != 0 as *mut _obstack_chunk) as libc::c_int;
}
pub unsafe extern "C" fn _obstack_free(mut h: *mut obstack, mut obj: *mut libc::c_void) {
    let mut lp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut plp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    lp = (*h).chunk;
    while !lp.is_null()
        && (lp as *mut libc::c_void >= obj || ((*lp).limit as *mut libc::c_void) < obj)
    {
        plp = (*lp).prev;
        call_freefun(h, lp as *mut libc::c_void);
        lp = plp;
        (*h).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
    }
    if !lp.is_null() {
        (*h).next_free = obj as *mut libc::c_char;
        (*h).object_base = (*h).next_free;
        (*h).chunk_limit = (*lp).limit;
        (*h).chunk = lp;
    } else if !obj.is_null() {
        abort();
    }
}
pub unsafe extern "C" fn _obstack_memory_used(mut h: *mut obstack) -> size_t {
    let mut lp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut nbytes: size_t = 0 as libc::c_int as size_t;
    lp = (*h).chunk;
    while !lp.is_null() {
        nbytes = (nbytes as libc::c_ulong)
            .wrapping_add(
                ((*lp).limit).offset_from(lp as *mut libc::c_char) as libc::c_long
                    as libc::c_ulong,
            ) as size_t as size_t;
        lp = (*lp).prev;
    }
    return nbytes;
}
unsafe extern "C" fn print_and_abort() -> ! {
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        dcgettext(
            0 as *const libc::c_char,
            b"memory exhausted\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    exit(exit_failure);
}
pub static mut obstack_alloc_failed_handler: Option::<unsafe extern "C" fn() -> !> = unsafe {
    Some(print_and_abort as unsafe extern "C" fn() -> !)
};
