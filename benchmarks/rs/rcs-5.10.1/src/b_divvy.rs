use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const libc::c_char,
        __args: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    static mut obstack_alloc_failed_handler: Option::<unsafe extern "C" fn() -> !>;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type off_t = __off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const libc::c_char,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
}
pub static mut plexus: *mut divvy = 0 as *const divvy as *mut divvy;
pub static mut single: *mut divvy = 0 as *const divvy as *mut divvy;
pub unsafe extern "C" fn make_space(mut name: *const libc::c_char) -> *mut divvy {
    let mut divvy: *mut divvy = xmalloc(::std::mem::size_of::<divvy>() as libc::c_ulong)
        as *mut divvy;
    (*divvy).name = name;
    obstack_alloc_failed_handler = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        Option::<unsafe extern "C" fn() -> !>,
    >(Some(xalloc_die as unsafe extern "C" fn() -> ()));
    _obstack_begin(
        &mut (*divvy).space,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    let mut widest: size_t = if ::std::mem::size_of::<*mut libc::c_void>()
        as libc::c_ulong > ::std::mem::size_of::<off_t>() as libc::c_ulong
    {
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
    } else {
        ::std::mem::size_of::<off_t>() as libc::c_ulong
    };
    (*divvy)
        .space
        .alignment_mask = widest.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*divvy).first = (*divvy).space.next_free as *mut libc::c_void;
    (*divvy).count = 0 as libc::c_int as size_t;
    return divvy;
}
pub unsafe extern "C" fn alloc(
    mut divvy: *mut divvy,
    mut len: size_t,
) -> *mut libc::c_void {
    (*divvy).count = ((*divvy).count).wrapping_add(1);
    (*divvy).count;
    return {
        let mut __h: *mut obstack = &mut (*divvy).space;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = len;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        {
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }
    };
}
pub unsafe extern "C" fn zlloc(
    mut divvy: *mut divvy,
    mut len: size_t,
) -> *mut libc::c_void {
    return memset(alloc(divvy, len), 0 as libc::c_int, len);
}
pub unsafe extern "C" fn intern(
    mut divvy: *mut divvy,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    (*divvy).count = ((*divvy).count).wrapping_add(1);
    (*divvy).count;
    return ({
        let mut __h: *mut obstack = &mut (*divvy).space as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = len;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            _obstack_newchunk(
                __o,
                __len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        memcpy((*__o).next_free as *mut libc::c_void, s as *const libc::c_void, __len);
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        let fresh0 = (*__o).next_free;
        (*__o).next_free = ((*__o).next_free).offset(1);
        *fresh0 = 0 as libc::c_int as libc::c_char;
        {
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn brush_off(mut divvy: *mut divvy, mut ptr: *mut libc::c_void) {
    (*divvy).count = ((*divvy).count).wrapping_sub(1);
    (*divvy).count;
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __obj: *mut libc::c_void = ptr;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut libc::c_char;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    };
}
pub unsafe extern "C" fn forget(mut divvy: *mut divvy) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __obj: *mut libc::c_void = (*divvy).first;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut libc::c_char;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    (*divvy).count = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn accf(
    mut divvy: *mut divvy,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    obstack_vprintf(&mut (*divvy).space, fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn accumulate_nbytes(
    mut divvy: *mut divvy,
    mut start: *const libc::c_char,
    mut count: size_t,
) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __len: size_t = count;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, start as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
}
pub unsafe extern "C" fn accumulate_byte(mut divvy: *mut divvy, mut c: libc::c_int) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o, 1 as libc::c_int as size_t);
    }
    let fresh1 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh1 = c as libc::c_char;
}
pub unsafe extern "C" fn accumulate_range(
    mut divvy: *mut divvy,
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __len: size_t = end.offset_from(beg) as libc::c_long as size_t;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, beg as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
}
pub unsafe extern "C" fn accs(mut divvy: *mut divvy, mut string: *const libc::c_char) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __len: size_t = strlen(string);
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, string as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
}
pub unsafe extern "C" fn finish_string(
    mut divvy: *mut divvy,
    mut result_len: *mut size_t,
) -> *mut libc::c_char {
    let mut o: *mut obstack = &mut (*divvy).space;
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    *result_len = {
        let mut __o: *const obstack = o;
        ((*__o).next_free).offset_from((*__o).object_base) as libc::c_long as size_t
    };
    let mut __o: *mut obstack = o;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o, 1 as libc::c_int as size_t);
    }
    let fresh2 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh2 = '\0' as i32 as libc::c_char;
    rv = ({
        let mut __o1: *mut obstack = o;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut libc::c_char {
            (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1)
            .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut libc::c_char
                        },
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut libc::c_char;
    return rv;
}
pub unsafe extern "C" fn pointer_array(
    mut divvy: *mut divvy,
    mut count: size_t,
) -> *mut libc::c_void {
    let mut o: *mut obstack = &mut (*divvy).space;
    loop {
        let fresh3 = count;
        count = count.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        let mut __o: *mut obstack = o;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            _obstack_newchunk(
                __o,
                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
            );
        }
        let mut __o1: *mut obstack = __o;
        let mut __p1: *mut libc::c_void = (*__o1).next_free as *mut libc::c_void;
        let ref mut fresh4 = *(__p1 as *mut *const libc::c_void);
        *fresh4 = 0 as *const libc::c_void;
        (*__o1)
            .next_free = ((*__o1).next_free)
            .offset(
                ::std::mem::size_of::<*const libc::c_void>() as libc::c_ulong as isize,
            );
    }
    return {
        let mut __o1_0: *mut obstack = o;
        let mut __value: *mut libc::c_void = (*__o1_0).object_base as *mut libc::c_void;
        if (*__o1_0).next_free == __value as *mut libc::c_char {
            (*__o1_0).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1_0)
            .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1_0).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1_0).next_free)
                    .offset_from(
                        if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        {
                            (*__o1_0).object_base
                        } else {
                            0 as *mut libc::c_char
                        },
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1_0).alignment_mask) & !(*__o1_0).alignment_mask)
                    as isize,
            );
        if ((*__o1_0).next_free).offset_from((*__o1_0).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1_0).chunk_limit).offset_from((*__o1_0).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1_0).next_free = (*__o1_0).chunk_limit;
        }
        (*__o1_0).object_base = (*__o1_0).next_free;
        __value
    };
}
pub unsafe extern "C" fn close_space(mut divvy: *mut divvy) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut libc::c_char;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    (*divvy).count = 0 as libc::c_int as size_t;
    (*divvy).first = 0 as *mut libc::c_void;
    rpl_free(divvy as *mut libc::c_void);
}
