use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_int_value(_: strm_int) -> strm_value;
    static mut strm_event_loop_started: libc::c_int;
    fn strm_ns_new(_: *mut strm_state, _: *const libc::c_char) -> *mut strm_state;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn strm_ary_new(_: *const strm_value, _: strm_int) -> strm_array;
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    static mut _etext: [libc::c_char; 0];
    static mut __init_array_start: [libc::c_char; 0];
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intptr_t = libc::c_long;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type intptr_t = __intptr_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value_tag = libc::c_ulong;
pub const STRM_TAG_FOREIGN: strm_value_tag = 18446462598732840960;
pub const STRM_TAG_PTR: strm_value_tag = 18445899648779419648;
pub const STRM_TAG_CFUNC: strm_value_tag = 18445336698825998336;
pub const STRM_TAG_STRING_F: strm_value_tag = 18445055223849287680;
pub const STRM_TAG_STRING_O: strm_value_tag = 18444773748872577024;
pub const STRM_TAG_STRING_6: strm_value_tag = 18444492273895866368;
pub const STRM_TAG_STRING_I: strm_value_tag = 18444210798919155712;
pub const STRM_TAG_STRUCT: strm_value_tag = 18443647848965734400;
pub const STRM_TAG_ARRAY: strm_value_tag = 18443366373989023744;
pub const STRM_TAG_LIST: strm_value_tag = 18443084899012313088;
pub const STRM_TAG_INT: strm_value_tag = 18442803424035602432;
pub const STRM_TAG_BOOL: strm_value_tag = 18442521949058891776;
pub const STRM_TAG_NAN: strm_value_tag = 18442240474082181120;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_cfunc = Option::<
    unsafe extern "C" fn(
        *mut strm_stream,
        libc::c_int,
        *mut strm_value,
        *mut strm_value,
    ) -> libc::c_int,
>;
pub type strm_string = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_string_0 {
    pub ptr: *const libc::c_char,
    pub len: strm_int,
}
pub type khiter_t = khint_t;
pub type khint_t = khint32_t;
pub type khint32_t = libc::c_uint;
pub type kh_sym_t = kh_sym_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_sym_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut sym_key,
    pub vals: *mut strm_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sym_key {
    pub ptr: *const libc::c_char,
    pub len: strm_int,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type strm_array = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_array_0 {
    pub len: strm_int,
    pub ptr: *mut strm_value,
    pub headers: strm_array,
    pub ns: *mut strm_state,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
static mut khash_ac_HASH_UPPER: libc::c_double = 0.77f64;
#[inline]
unsafe extern "C" fn readonly_data_p(mut p: *const libc::c_char) -> libc::c_int {
    return (_etext.as_mut_ptr() < p as *mut libc::c_char
        && p
            < &mut __init_array_start as *mut [libc::c_char; 0] as *mut libc::c_char
                as *const libc::c_char) as libc::c_int;
}
unsafe extern "C" fn sym_hash(mut key: sym_key) -> khint_t {
    let mut s: *const libc::c_char = key.ptr;
    let mut h: khint_t = 0;
    let mut len: strm_int = key.len;
    let fresh0 = s;
    s = s.offset(1);
    h = *fresh0 as khint_t;
    loop {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 != 0) {
            break;
        }
        let fresh2 = s;
        s = s.offset(1);
        h = (h << 5 as libc::c_int).wrapping_sub(h).wrapping_add(*fresh2 as khint_t);
    }
    return h;
}
unsafe extern "C" fn sym_eq(mut a: sym_key, mut b: sym_key) -> khint_t {
    if a.len != b.len {
        return 0 as libc::c_int as khint_t;
    }
    if memcmp(
        a.ptr as *const libc::c_void,
        b.ptr as *const libc::c_void,
        a.len as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 1 as libc::c_int as khint_t;
    }
    return 0 as libc::c_int as khint_t;
}
#[inline]
unsafe extern "C" fn kh_put_sym(
    mut h: *mut kh_sym_t,
    mut key: sym_key,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            if kh_resize_sym(
                h,
                ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else if kh_resize_sym(
            h,
            ((*h).n_buckets).wrapping_add(1 as libc::c_int as libc::c_uint),
        ) < 0 as libc::c_int
        {
            *ret = -(1 as libc::c_int);
            return (*h).n_buckets;
        }
    }
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = ((*h).n_buckets)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut step: khint_t = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = sym_hash(key);
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || sym_eq(*((*h).keys).offset(i as isize), key) == 0)
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 2 as libc::c_int as libc::c_uint != 0 && site != (*h).n_buckets
            {
                x = site;
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh3 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh3 = (*fresh3 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        (*h).n_occupied;
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh4 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh4 = (*fresh4 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn kh_resize_sym(
    mut h: *mut kh_sym_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets;
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    new_n_buckets;
    if new_n_buckets < 4 as libc::c_int as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * khash_ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        new_flags = malloc(
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        ) as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        memset(
            new_flags as *mut libc::c_void,
            0xaa as libc::c_int,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            let mut new_keys: *mut sym_key = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<sym_key>() as libc::c_ulong),
            ) as *mut sym_key;
            if new_keys.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            let mut new_vals: *mut strm_string = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_string>() as libc::c_ulong),
            ) as *mut strm_string;
            if new_vals.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let mut key: sym_key = *((*h).keys).offset(j as isize);
                let mut val: strm_string = 0;
                let mut new_mask: khint_t = 0;
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_int as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                let ref mut fresh5 = *((*h).flags)
                    .offset((j >> 4 as libc::c_int) as isize);
                *fresh5 = (*fresh5 as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 0xf as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut step: khint_t = 0 as libc::c_int as khint_t;
                    k = sym_hash(key);
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_int as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    let ref mut fresh6 = *new_flags
                        .offset((i >> 4 as libc::c_int) as isize);
                    *fresh6 = (*fresh6 as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 0xf as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets
                        && *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        let mut tmp: sym_key = *((*h).keys).offset(i as isize);
                        *((*h).keys).offset(i as isize) = key;
                        key = tmp;
                        let mut tmp_0: strm_string = *((*h).vals).offset(i as isize);
                        *((*h).vals).offset(i as isize) = val;
                        val = tmp_0;
                        let ref mut fresh7 = *((*h).flags)
                            .offset((i >> 4 as libc::c_int) as isize);
                        *fresh7 = (*fresh7 as libc::c_ulong
                            | (1 as libc::c_ulong)
                                << ((i & 0xf as libc::c_uint) << 1 as libc::c_int))
                            as khint32_t;
                    } else {
                        *((*h).keys).offset(i as isize) = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if (*h).n_buckets > new_n_buckets {
            (*h)
                .keys = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<sym_key>() as libc::c_ulong),
            ) as *mut sym_key;
            (*h)
                .vals = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<strm_string>() as libc::c_ulong),
            ) as *mut strm_string;
        }
        free((*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * khash_ac_HASH_UPPER
            + 0.5f64) as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_init_sym() -> *mut kh_sym_t {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<kh_sym_t>() as libc::c_ulong,
    ) as *mut kh_sym_t;
}
static mut sym_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut sym_table: *mut kh_sym_t = 0 as *const kh_sym_t as *mut kh_sym_t;
unsafe extern "C" fn str_new(
    mut p: *const libc::c_char,
    mut len: strm_int,
    mut foreign: libc::c_int,
) -> strm_string {
    let mut str: *mut strm_string_0 = 0 as *mut strm_string_0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut tag: strm_value = 0;
    let mut val: strm_value = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if p.is_null() {
        current_block = 13896207889982612395;
    } else if len < 6 as libc::c_int {
        tag = STRM_TAG_STRING_I as libc::c_ulong;
        val = 0 as libc::c_int as strm_value;
        s = (&mut val as *mut strm_value as *mut libc::c_char)
            .offset(1 as libc::c_int as isize);
        memcpy(s as *mut libc::c_void, p as *const libc::c_void, len as libc::c_ulong);
        *s.offset(-(1 as libc::c_int) as isize) = len as libc::c_char;
        current_block = 9828876828309294594;
    } else if len == 6 as libc::c_int {
        tag = STRM_TAG_STRING_6 as libc::c_ulong;
        val = 0 as libc::c_int as strm_value;
        s = &mut val as *mut strm_value as *mut libc::c_char;
        memcpy(s as *mut libc::c_void, p as *const libc::c_void, len as libc::c_ulong);
        current_block = 9828876828309294594;
    } else {
        str = 0 as *mut strm_string_0;
        if !p.is_null() && (foreign != 0 || readonly_data_p(p) != 0) {
            tag = STRM_TAG_STRING_F as libc::c_ulong;
            str = malloc(::std::mem::size_of::<strm_string_0>() as libc::c_ulong)
                as *mut strm_string_0;
            (*str).ptr = p;
            current_block = 1109700713171191020;
        } else {
            buf = 0 as *mut libc::c_char;
            current_block = 13896207889982612395;
        }
    }
    match current_block {
        13896207889982612395 => {
            tag = STRM_TAG_STRING_O as libc::c_ulong;
            str = malloc(
                (::std::mem::size_of::<strm_string_0>() as libc::c_ulong)
                    .wrapping_add(len as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut strm_string_0;
            buf = &mut *str.offset(1 as libc::c_int as isize) as *mut strm_string_0
                as *mut libc::c_char;
            if !p.is_null() {
                memcpy(
                    buf as *mut libc::c_void,
                    p as *const libc::c_void,
                    len as libc::c_ulong,
                );
            } else {
                memset(buf as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
            }
            *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
            (*str).ptr = buf;
            current_block = 1109700713171191020;
        }
        _ => {}
    }
    match current_block {
        1109700713171191020 => {
            (*str).len = len;
            val = 0 as libc::c_int as libc::c_ulong
                | str as intptr_t as strm_value
                    & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                        << 48 as libc::c_int);
        }
        _ => {}
    }
    return tag
        | val
            & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int);
}
unsafe extern "C" fn str_intern(
    mut p: *const libc::c_char,
    mut len: strm_int,
    mut foreign: libc::c_int,
) -> strm_string {
    let mut k: khiter_t = 0;
    let mut key: sym_key = sym_key {
        ptr: 0 as *const libc::c_char,
        len: 0,
    };
    let mut ret: libc::c_int = 0;
    let mut str: strm_string = 0;
    if len <= 6 as libc::c_int {
        return str_new(p, len, foreign);
    }
    if sym_table.is_null() {
        sym_table = kh_init_sym();
    }
    key.ptr = p;
    key.len = len;
    k = kh_put_sym(sym_table, key, &mut ret);
    if ret == 0 as libc::c_int {
        return *((*sym_table).vals).offset(k as isize);
    }
    str = str_new(p, len, foreign);
    let ref mut fresh8 = (*((*sym_table).keys).offset(k as isize)).ptr;
    *fresh8 = strm_strp_ptr(&mut str);
    *((*sym_table).vals).offset(k as isize) = str;
    return str;
}
pub unsafe extern "C" fn strm_str_new(
    mut p: *const libc::c_char,
    mut len: strm_int,
) -> strm_string {
    if strm_event_loop_started == 0 {
        if !p.is_null() && (len < 64 as libc::c_int || readonly_data_p(p) != 0) {
            return str_intern(p, len, 0 as libc::c_int);
        }
    }
    return str_new(p, len, 0 as libc::c_int);
}
pub unsafe extern "C" fn strm_str_static(
    mut p: *const libc::c_char,
    mut len: strm_int,
) -> strm_string {
    return str_new(p, len, 1 as libc::c_int);
}
pub unsafe extern "C" fn strm_str_intern(
    mut p: *const libc::c_char,
    mut len: strm_int,
) -> strm_string {
    let mut str: strm_string = 0;
    if !p.is_null() {} else {
        __assert_fail(
            b"p!=NULL\0" as *const u8 as *const libc::c_char,
            b"string.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"strm_string strm_str_intern(const char *, strm_int)\0"))
                .as_ptr(),
        );
    }
    'c_4767: {
        if !p.is_null() {} else {
            __assert_fail(
                b"p!=NULL\0" as *const u8 as *const libc::c_char,
                b"string.c\0" as *const u8 as *const libc::c_char,
                182 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"strm_string strm_str_intern(const char *, strm_int)\0"))
                    .as_ptr(),
            );
        }
    };
    if strm_event_loop_started == 0 {
        return str_intern(p, len, 0 as libc::c_int);
    }
    pthread_mutex_lock(&mut sym_mutex);
    str = str_intern(p, len, 0 as libc::c_int);
    pthread_mutex_unlock(&mut sym_mutex);
    return str;
}
pub unsafe extern "C" fn strm_str_intern_str(mut str: strm_string) -> strm_string {
    if strm_str_intern_p(str) != 0 {
        return str;
    }
    if strm_event_loop_started == 0 {
        return str_intern(strm_strp_ptr(&mut str), strm_str_len(str), 0 as libc::c_int);
    }
    pthread_mutex_lock(&mut sym_mutex);
    str = str_intern(strm_strp_ptr(&mut str), strm_str_len(str), 0 as libc::c_int);
    pthread_mutex_unlock(&mut sym_mutex);
    return str;
}
pub unsafe extern "C" fn strm_str_intern_static(
    mut p: *const libc::c_char,
    mut len: strm_int,
) -> strm_string {
    return str_intern(p, len, 1 as libc::c_int);
}
pub unsafe extern "C" fn strm_str_intern_p(mut s: strm_string) -> libc::c_int {
    match s
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18444210798919155712 | 18444492273895866368 | 18445055223849287680 => {
            return 1 as libc::c_int;
        }
        18444773748872577024 | _ => return 0 as libc::c_int,
    };
}
pub unsafe extern "C" fn strm_str_eq(
    mut a: strm_string,
    mut b: strm_string,
) -> libc::c_int {
    if a == b {
        return 1 as libc::c_int;
    }
    if a
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
        == STRM_TAG_STRING_F as libc::c_ulong
        && b
            & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                << 48 as libc::c_int == STRM_TAG_STRING_F as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if strm_str_len(a) != strm_str_len(b) {
        return 0 as libc::c_int;
    }
    if memcmp(
        strm_strp_ptr(&mut a) as *const libc::c_void,
        strm_strp_ptr(&mut b) as *const libc::c_void,
        strm_str_len(a) as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_str_p(mut v: strm_value) -> libc::c_int {
    match v
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18444210798919155712 | 18444492273895866368 | 18445055223849287680
        | 18444773748872577024 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
pub unsafe extern "C" fn strm_strp_ptr(mut s: *mut strm_string) -> *const libc::c_char {
    match *s
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18444210798919155712 => {
            return (s as *mut libc::c_char).offset(1 as libc::c_int as isize);
        }
        18444492273895866368 => return s as *mut libc::c_char,
        18444773748872577024 | 18445055223849287680 => {
            let mut str: *mut strm_string_0 = (*s
                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                    << 48 as libc::c_int)) as intptr_t as *mut libc::c_void
                as *mut strm_string_0;
            return (*str).ptr;
        }
        _ => return 0 as *const libc::c_char,
    };
}
pub unsafe extern "C" fn strm_str_cstr(
    mut s: strm_string,
    mut buf: *mut libc::c_char,
) -> *const libc::c_char {
    let mut len: strm_int = 0;
    match s
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18444210798919155712 => {
            len = *(&mut s as *mut strm_string as *mut libc::c_char)
                .offset(0 as libc::c_int as isize) as strm_int;
            memcpy(
                buf as *mut libc::c_void,
                (&mut s as *mut strm_string as *mut libc::c_char)
                    .offset(1 as libc::c_int as isize) as *const libc::c_void,
                len as libc::c_ulong,
            );
            *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
            return buf as *const libc::c_char;
        }
        18444492273895866368 => {
            memcpy(
                buf as *mut libc::c_void,
                &mut s as *mut strm_string as *mut libc::c_char as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            );
            *buf.offset(6 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            return buf as *const libc::c_char;
        }
        18444773748872577024 | 18445055223849287680 => {
            let mut str: *mut strm_string_0 = (s
                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                    << 48 as libc::c_int)) as intptr_t as *mut libc::c_void
                as *mut strm_string_0;
            return (*str).ptr;
        }
        _ => return 0 as *const libc::c_char,
    };
}
pub unsafe extern "C" fn strm_str_len(mut s: strm_string) -> strm_int {
    match s
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18444210798919155712 => {
            return *(&mut s as *mut strm_string as *mut libc::c_char)
                .offset(0 as libc::c_int as isize) as strm_int;
        }
        18444492273895866368 => return 6 as libc::c_int,
        18444773748872577024 | 18445055223849287680 => {
            let mut str: *mut strm_string_0 = (s
                & !(((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t)
                    << 48 as libc::c_int)) as intptr_t as *mut libc::c_void
                as *mut strm_string_0;
            return (*str).len;
        }
        _ => return 0 as libc::c_int,
    };
}
pub unsafe extern "C" fn strm_string_p(mut s: strm_string) -> libc::c_int {
    match s
        & ((0xfff0 as libc::c_int | 0xf as libc::c_int) as uint64_t) << 48 as libc::c_int
    {
        18444210798919155712 | 18444492273895866368 | 18444773748872577024
        | 18445055223849287680 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
pub static mut strm_ns_string: *mut strm_state = 0 as *const strm_state
    as *mut strm_state;
unsafe extern "C" fn str_length(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"s\0" as *const u8 as *const libc::c_char,
        &mut p as *mut *mut libc::c_char,
        &mut len as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    *ret = strm_int_value(len);
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_split(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut slen: strm_int = 0;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut plen: strm_int = 0;
    let mut pend: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_char = 0;
    let mut n: strm_int = 0 as libc::c_int;
    let mut ary: strm_array = 0;
    let mut sps: *mut strm_value = 0 as *mut strm_value;
    let mut i: strm_int = 0;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"s|s\0" as *const u8 as *const libc::c_char,
        &mut p as *mut *const libc::c_char,
        &mut plen as *mut strm_int,
        &mut s as *mut *const libc::c_char,
        &mut slen as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        s = b" \0" as *const u8 as *const libc::c_char;
        slen = 1 as libc::c_int;
    }
    c = *s.offset(0 as libc::c_int as isize);
    t = p;
    b = t;
    pend = p.offset(plen as isize).offset(-(slen as isize));
    n = 0 as libc::c_int;
    while p < pend {
        if *p as libc::c_int == c as libc::c_int {
            if memcmp(
                p as *const libc::c_void,
                s as *const libc::c_void,
                slen as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if !(slen == 1 as libc::c_int && c as libc::c_int == ' ' as i32
                    && p.offset_from(t) as libc::c_long
                        == 0 as libc::c_int as libc::c_long)
                {
                    n += 1;
                    n;
                }
                t = p.offset(slen as isize);
            }
        }
        p = p.offset(1);
        p;
    }
    n += 1;
    n;
    ary = strm_ary_new(0 as *const strm_value, n);
    sps = (*strm_ary_struct(ary)).ptr;
    c = *s.offset(0 as libc::c_int as isize);
    t = b;
    p = t;
    i = 0 as libc::c_int;
    while p < pend {
        if *p as libc::c_int == c as libc::c_int {
            if memcmp(
                p as *const libc::c_void,
                s as *const libc::c_void,
                slen as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if !(slen == 1 as libc::c_int && c as libc::c_int == ' ' as i32
                    && p.offset_from(t) as libc::c_long
                        == 0 as libc::c_int as libc::c_long)
                {
                    let fresh9 = i;
                    i = i + 1;
                    *sps
                        .offset(
                            fresh9 as isize,
                        ) = strm_str_new(
                        t,
                        p.offset_from(t) as libc::c_long as strm_int,
                    );
                }
                t = p.offset(slen as isize);
            }
        }
        p = p.offset(1);
        p;
    }
    pend = b.offset(plen as isize);
    let fresh10 = i;
    i = i + 1;
    *sps
        .offset(
            fresh10 as isize,
        ) = strm_str_new(t, pend.offset_from(t) as libc::c_long as strm_int);
    *ret = ary;
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_plus(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut str1: strm_string = 0;
    let mut str2: strm_string = 0;
    let mut str3: strm_string = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"SS\0" as *const u8 as *const libc::c_char,
        &mut str1 as *mut strm_string,
        &mut str2 as *mut strm_string,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    str3 = strm_str_new(
        0 as *const libc::c_char,
        strm_str_len(str1) + strm_str_len(str2),
    );
    p = strm_strp_ptr(&mut str3) as *mut libc::c_char;
    memcpy(
        p as *mut libc::c_void,
        strm_strp_ptr(&mut str1) as *const libc::c_void,
        strm_str_len(str1) as libc::c_ulong,
    );
    memcpy(
        p.offset(strm_str_len(str1) as isize) as *mut libc::c_void,
        strm_strp_ptr(&mut str2) as *const libc::c_void,
        strm_str_len(str2) as libc::c_ulong,
    );
    *p.offset(strm_str_len(str3) as isize) = '\0' as i32 as libc::c_char;
    *ret = str3;
    return 0 as libc::c_int;
}
static mut utf8len_codepage: [libc::c_char; 256] = [
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn utf8len(
    mut p: *const libc::c_char,
    mut e: *const libc::c_char,
) -> libc::c_int {
    let mut len: strm_int = 0;
    let mut i: strm_int = 0;
    len = utf8len_codepage[*p as libc::c_uchar as usize] as strm_int;
    if p.offset(len as isize) > e {
        return 1 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < len {
        if *p.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int
            != 0x80 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return len;
}
unsafe extern "C" fn str_chars(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut prev: *const libc::c_char = 0 as *const libc::c_char;
    let mut slen: strm_int = 0;
    let mut ary: strm_array = 0;
    let mut n: strm_int = 0 as libc::c_int;
    let mut sps: *mut strm_value = 0 as *mut strm_value;
    let mut i: strm_int = 0 as libc::c_int;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"s\0" as *const u8 as *const libc::c_char,
        &mut str as *mut *const libc::c_char,
        &mut slen as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    s = str;
    while *s != 0 {
        s = s.offset(utf8len(s, s.offset(slen as isize)) as isize);
        n += 1;
        n;
    }
    ary = strm_ary_new(0 as *const strm_value, n);
    sps = (*strm_ary_struct(ary)).ptr;
    s = str;
    while *s != 0 {
        prev = s;
        s = s.offset(utf8len(s, s.offset(slen as isize)) as isize);
        let fresh11 = i;
        i = i + 1;
        *sps
            .offset(
                fresh11 as isize,
            ) = strm_str_new(prev, s.offset_from(prev) as libc::c_long as strm_int);
    }
    *ret = ary;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_string_init(mut state: *mut strm_state) {
    strm_ns_string = strm_ns_new(
        0 as *mut strm_state,
        b"string\0" as *const u8 as *const libc::c_char,
    );
    strm_var_def(
        strm_ns_string,
        b"length\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_length
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        strm_ns_string,
        b"split\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_split
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        strm_ns_string,
        b"+\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_plus
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
    strm_var_def(
        strm_ns_string,
        b"chars\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                str_chars
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
}
