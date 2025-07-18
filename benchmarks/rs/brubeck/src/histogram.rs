use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn floor(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
    fn gh_log_die() -> !;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type value_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo {
    pub values: *mut value_t,
    pub count: uint32_t,
    pub alloc: uint16_t,
    pub size: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo_sample {
    pub sum: value_t,
    pub min: value_t,
    pub max: value_t,
    pub mean: value_t,
    pub median: value_t,
    pub count: value_t,
    pub percentile: [value_t; 5],
}
pub type C2RustUnnamed = libc::c_uint;
pub const PC_999: C2RustUnnamed = 4;
pub const PC_99: C2RustUnnamed = 3;
pub const PC_98: C2RustUnnamed = 2;
pub const PC_95: C2RustUnnamed = 1;
pub const PC_75: C2RustUnnamed = 0;
#[inline]
unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_ptr: *mut libc::c_void = realloc(ptr, size);
    if (new_ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return new_ptr;
}
pub unsafe extern "C" fn brubeck_histo_push(
    mut histo: *mut brubeck_histo,
    mut value: value_t,
    mut sample_freq: value_t,
) {
    (*histo).count = ((*histo).count as libc::c_double + sample_freq) as uint32_t;
    if (*histo).size as libc::c_int == (*histo).alloc as libc::c_int {
        let mut new_size: size_t = 0;
        if (*histo).size as libc::c_int
            == 32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
        {
            return;
        }
        new_size = ((*histo).alloc as libc::c_int * 2 as libc::c_int) as size_t;
        if new_size
            > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong
        {
            new_size = (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                as size_t;
        }
        if new_size < 16 as libc::c_int as libc::c_ulong {
            new_size = 16 as libc::c_int as size_t;
        }
        if new_size != (*histo).alloc as libc::c_ulong {
            (*histo).alloc = new_size as uint16_t;
            (*histo)
                .values = xrealloc(
                (*histo).values as *mut libc::c_void,
                ((*histo).alloc as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<value_t>() as libc::c_ulong),
            ) as *mut value_t;
        }
    }
    let fresh0 = (*histo).size;
    (*histo).size = ((*histo).size).wrapping_add(1);
    *((*histo).values).offset(fresh0 as isize) = value;
}
#[inline]
unsafe extern "C" fn histo_percentile(
    mut histo: *mut brubeck_histo,
    mut rank: libc::c_float,
) -> value_t {
    let mut irank: size_t = floor(
        (rank * (*histo).size as libc::c_int as libc::c_float + 0.5f32) as libc::c_double,
    ) as size_t;
    return *((*histo).values)
        .offset(irank.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
}
#[inline]
unsafe extern "C" fn histo_sum(mut histo: *mut brubeck_histo) -> value_t {
    let mut i: size_t = 0;
    let mut sum: value_t = 0.0f64;
    i = 0 as libc::c_int as size_t;
    while i < (*histo).size as libc::c_ulong {
        sum += *((*histo).values).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
unsafe extern "C" fn value_cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let va: value_t = *(a as *const value_t);
    let vb: value_t = *(b as *const value_t);
    if va < vb {
        return -(1 as libc::c_int);
    }
    if va > vb {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn histo_sort(mut histo: *mut brubeck_histo) {
    qsort(
        (*histo).values as *mut libc::c_void,
        (*histo).size as size_t,
        ::std::mem::size_of::<value_t>() as libc::c_ulong,
        Some(
            value_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn brubeck_histo_sample(
    mut sample: *mut brubeck_histo_sample,
    mut histo: *mut brubeck_histo,
) {
    if (*histo).size as libc::c_int == 0 as libc::c_int {
        memset(
            sample as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<brubeck_histo_sample>() as libc::c_ulong,
        );
        return;
    }
    histo_sort(histo);
    (*sample).sum = histo_sum(histo);
    (*sample).min = *((*histo).values).offset(0 as libc::c_int as isize);
    (*sample)
        .max = *((*histo).values)
        .offset(((*histo).size as libc::c_int - 1 as libc::c_int) as isize);
    (*sample).mean = (*sample).sum / (*histo).size as libc::c_int as libc::c_double;
    (*sample).median = histo_percentile(histo, 0.5f32);
    (*sample).count = (*histo).count as value_t;
    (*sample)
        .percentile[PC_75 as libc::c_int as usize] = histo_percentile(histo, 0.75f32);
    (*sample)
        .percentile[PC_95 as libc::c_int as usize] = histo_percentile(histo, 0.95f32);
    (*sample)
        .percentile[PC_98 as libc::c_int as usize] = histo_percentile(histo, 0.98f32);
    (*sample)
        .percentile[PC_99 as libc::c_int as usize] = histo_percentile(histo, 0.99f32);
    (*sample)
        .percentile[PC_999 as libc::c_int as usize] = histo_percentile(histo, 0.999f32);
    (*histo).size = 0 as libc::c_int as uint16_t;
    (*histo).count = 0 as libc::c_int as uint32_t;
}
