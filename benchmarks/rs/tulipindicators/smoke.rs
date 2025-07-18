use ::libc;
extern "C" {
    pub type ti_stream;
    pub type tc_result;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn ti_version() -> *const libc::c_char;
    fn ti_build() -> libc::c_long;
    fn ti_indicator_count() -> libc::c_int;
    static mut ti_indicators: [ti_indicator_info; 0];
    fn ti_find_indicator(name: *const libc::c_char) -> *const ti_indicator_info;
    fn ti_stream_get_progress(stream: *mut ti_stream) -> libc::c_int;
    fn tc_version() -> *const libc::c_char;
    fn tc_build() -> libc::c_long;
    fn tc_candle_count() -> libc::c_int;
    static mut tc_candles: [tc_candle_info; 0];
    fn tc_find_candle(name: *const libc::c_char) -> *const tc_candle_info;
    fn tc_config_set_to_default(config: *mut tc_config);
    fn tc_result_new() -> *mut tc_result;
    fn tc_result_free(result: *mut tc_result);
    fn tc_result_at(result: *const tc_result, index: libc::c_int) -> tc_set;
    fn tc_run(
        patterns: tc_set,
        size: libc::c_int,
        inputs: *const *const TC_REAL,
        options: *const tc_config,
        output: *mut tc_result,
    ) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn clock() -> clock_t;
    fn ti_buffer_new(size: libc::c_int) -> *mut ti_buffer;
    fn ti_buffer_free(buffer: *mut ti_buffer);
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ti_indicator_start_function = Option::<
    unsafe extern "C" fn(*const libc::c_double) -> libc::c_int,
>;
pub type ti_indicator_function = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const *const libc::c_double,
        *const libc::c_double,
        *const *mut libc::c_double,
    ) -> libc::c_int,
>;
pub type ti_indicator_stream_new = Option::<
    unsafe extern "C" fn(*const libc::c_double, *mut *mut ti_stream) -> libc::c_int,
>;
pub type ti_indicator_stream_run = Option::<
    unsafe extern "C" fn(
        *mut ti_stream,
        libc::c_int,
        *const *const libc::c_double,
        *const *mut libc::c_double,
    ) -> libc::c_int,
>;
pub type ti_indicator_stream_free = Option::<unsafe extern "C" fn(*mut ti_stream) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_indicator_info {
    pub name: *const libc::c_char,
    pub full_name: *const libc::c_char,
    pub start: ti_indicator_start_function,
    pub indicator: ti_indicator_function,
    pub indicator_ref: ti_indicator_function,
    pub type_0: libc::c_int,
    pub inputs: libc::c_int,
    pub options: libc::c_int,
    pub outputs: libc::c_int,
    pub input_names: [*const libc::c_char; 16],
    pub option_names: [*const libc::c_char; 16],
    pub output_names: [*const libc::c_char; 16],
    pub stream_new: ti_indicator_stream_new,
    pub stream_run: ti_indicator_stream_run,
    pub stream_free: ti_indicator_stream_free,
}
pub type uint64_t = __uint64_t;
pub type TC_REAL = libc::c_double;
pub type tc_set = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_config {
    pub period: libc::c_int,
    pub body_none: TC_REAL,
    pub body_short: TC_REAL,
    pub body_long: TC_REAL,
    pub wick_none: TC_REAL,
    pub wick_long: TC_REAL,
    pub near: TC_REAL,
}
pub type tc_candle_function = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const *const TC_REAL,
        *const tc_config,
        *mut tc_result,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_candle_info {
    pub name: *const libc::c_char,
    pub full_name: *const libc::c_char,
    pub pattern: tc_set,
    pub candle: tc_candle_function,
}
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
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_buffer {
    pub size: libc::c_int,
    pub pushes: libc::c_int,
    pub index: libc::c_int,
    pub sum: libc::c_double,
    pub vals: [libc::c_double; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub padding: libc::c_double,
    pub zero: [libc::c_int; 4],
    pub offset_a: libc::c_int,
    pub size_a: libc::c_int,
    pub index_a: libc::c_int,
    pub padding_a: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s {
    pub buf_info: C2RustUnnamed,
    pub padding: [libc::c_double; 5],
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut lfails: libc::c_int = 0 as libc::c_int;
static mut ltests: libc::c_int = 0 as libc::c_int;
pub static mut tested_ind: [libc::c_int; 104] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub static mut tested_cnd: [libc::c_int; 26] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn compare_arrays(
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut size_a: libc::c_int,
    mut size_b: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    ltests += 1;
    ltests;
    if size_a != size_b {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            size_a,
            size_b,
        );
    }
    if size_a != size_b {
        printf(b"Size mismatch.\n\0" as *const u8 as *const libc::c_char);
    }
    let f: libc::c_int = lfails;
    i = 0 as libc::c_int;
    while i < size_a {
        ltests += 1;
        ltests;
        if fabs(*a.offset(i as isize) - *b.offset(i as isize)) > 0.001f64 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"smoke.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int,
                *a.offset(i as isize),
                *b.offset(i as isize),
            );
        }
        i += 1;
        i;
    }
    if lfails != f {
        printf(b"Expected      Got\n\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < size_a {
            printf(
                b"%8.3f %8.3f\n\0" as *const u8 as *const libc::c_char,
                *a.offset(i as isize),
                *b.offset(i as isize),
            );
            i += 1;
            i;
        }
    }
}
pub unsafe extern "C" fn next_line(mut fp: *mut FILE) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 4096] = [0; 4096];
    while !(fgets(buf.as_mut_ptr(), 4096 as libc::c_int, fp)).is_null() {
        if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue;
        }
        if buf[0 as libc::c_int as usize] as libc::c_int == 10 as libc::c_int
            || buf[0 as libc::c_int as usize] as libc::c_int == 13 as libc::c_int
        {
            continue;
        }
        return buf.as_mut_ptr();
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn get_array(
    mut fp: *mut FILE,
    mut s: *mut libc::c_double,
) -> libc::c_int {
    let mut line: *mut libc::c_char = next_line(fp);
    if *line.offset(0 as libc::c_int as isize) as libc::c_int != '{' as i32 {
        printf(b"Bad input\n\0" as *const u8 as *const libc::c_char);
        ltests += 1;
        ltests;
        if 0 as libc::c_int == 0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"smoke.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
            );
        }
        return 0 as libc::c_int;
    }
    let mut num: *mut libc::c_char = strtok(
        line.offset(1 as libc::c_int as isize),
        b",}\r\n\0" as *const u8 as *const libc::c_char,
    );
    if num.is_null() {
        return 0 as libc::c_int;
    }
    let mut inp: *mut libc::c_double = s;
    loop {
        *inp = atof(num);
        inp = inp.offset(1);
        inp;
        num = strtok(
            0 as *mut libc::c_char,
            b",}\r\n\0" as *const u8 as *const libc::c_char,
        );
        if num.is_null() {
            break;
        }
    }
    return inp.offset_from(s) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn test_ind_name(
    mut fp: *mut FILE,
    mut name: *const libc::c_char,
    mut count: libc::c_int,
) {
    let mut info: *const ti_indicator_info = ti_find_indicator(name);
    if info.is_null() {
        printf(
            b"(WARN) Couldn't find indicator %s\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        printf(
            b"---------------------------------------------\0" as *const u8
                as *const libc::c_char,
        );
        static mut buf: [libc::c_char; 4096] = [0; 4096];
        while !(fgets(buf.as_mut_ptr(), 4096 as libc::c_int, fp)).is_null()
            && (buf[0 as libc::c_int as usize] as libc::c_int == '{' as i32
                || buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32)
        {}
        return;
    }
    let ind_num: libc::c_int = info.offset_from(ti_indicators.as_mut_ptr())
        as libc::c_long as libc::c_int;
    if count != 0 {
        tested_ind[ind_num as usize] = 1 as libc::c_int;
    }
    let mut options: [libc::c_double; 16] = [0.; 16];
    let mut o: *mut libc::c_double = options.as_mut_ptr();
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        s = strtok(
            0 as *mut libc::c_char,
            b" \n\r\0" as *const u8 as *const libc::c_char,
        );
        if s.is_null() {
            break;
        }
        *o = atof(s);
        o = o.offset(1);
        o;
    }
    ltests += 1;
    ltests;
    if o.offset_from(options.as_mut_ptr()) as libc::c_long as libc::c_int
        != (*info).options
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            o.offset_from(options.as_mut_ptr()) as libc::c_long as libc::c_int,
            (*info).options,
        );
    }
    let mut i: libc::c_int = 0;
    let mut inputs: [*mut libc::c_double; 16] = [
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
    ];
    let mut answers: [*mut libc::c_double; 16] = [
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
    ];
    let mut outputs: [*mut libc::c_double; 16] = [
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
    ];
    let mut input_size: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*info).inputs {
        inputs[i
            as usize] = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_double;
        input_size = get_array(fp, inputs[i as usize]);
        i += 1;
        i;
    }
    let mut answer_size: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*info).outputs {
        answers[i
            as usize] = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_double;
        outputs[i
            as usize] = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_double;
        answer_size = get_array(fp, answers[i as usize]);
        i += 1;
        i;
    }
    let ret: libc::c_int = ((*info).indicator)
        .unwrap()(
        input_size,
        inputs.as_mut_ptr() as *const *const libc::c_double,
        options.as_mut_ptr(),
        outputs.as_mut_ptr(),
    );
    ltests += 1;
    ltests;
    if !(ret == 0 as libc::c_int) {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
        );
    }
    let mut output_size: libc::c_int = input_size
        - ((*info).start).unwrap()(options.as_mut_ptr());
    if output_size < 0 as libc::c_int {
        output_size = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*info).outputs {
        compare_arrays(
            answers[i as usize],
            outputs[i as usize],
            answer_size,
            output_size,
        );
        i += 1;
        i;
    }
    if ((*info).indicator_ref).is_some() {
        i = 0 as libc::c_int;
        while i < (*info).outputs {
            memset(
                outputs[i as usize] as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
            );
            i += 1;
            i;
        }
        let ret_0: libc::c_int = ((*info).indicator_ref)
            .unwrap()(
            input_size,
            inputs.as_mut_ptr() as *const *const libc::c_double,
            options.as_mut_ptr(),
            outputs.as_mut_ptr(),
        );
        ltests += 1;
        ltests;
        if !(ret_0 == 0 as libc::c_int) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"smoke.c\0" as *const u8 as *const libc::c_char,
                176 as libc::c_int,
            );
        }
        let mut output_size_0: libc::c_int = input_size
            - ((*info).start).unwrap()(options.as_mut_ptr());
        if output_size_0 < 0 as libc::c_int {
            output_size_0 = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < (*info).outputs {
            compare_arrays(
                answers[i as usize],
                outputs[i as usize],
                answer_size,
                output_size_0,
            );
            i += 1;
            i;
        }
    }
    if ((*info).stream_new).is_some() {
        let steps: [libc::c_int; 9] = [
            1 as libc::c_int,
            2 as libc::c_int,
            3 as libc::c_int,
            4 as libc::c_int,
            5 as libc::c_int,
            7 as libc::c_int,
            13 as libc::c_int,
            100 as libc::c_int,
            1024 as libc::c_int,
        ];
        let step_count: libc::c_int = (::std::mem::size_of::<[libc::c_int; 9]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int;
        let mut step_i: libc::c_int = 0 as libc::c_int;
        while step_i < step_count {
            let step: libc::c_int = steps[step_i as usize];
            i = 0 as libc::c_int;
            while i < (*info).outputs {
                memset(
                    outputs[i as usize] as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                        .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
                );
                i += 1;
                i;
            }
            let mut stream: *mut ti_stream = 0 as *mut ti_stream;
            let stream_ret: libc::c_int = ((*info).stream_new)
                .unwrap()(options.as_mut_ptr(), &mut stream);
            ltests += 1;
            ltests;
            if !(stream_ret == 0 as libc::c_int) {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"smoke.c\0" as *const u8 as *const libc::c_char,
                    201 as libc::c_int,
                );
            }
            ltests += 1;
            ltests;
            if stream.is_null() {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"smoke.c\0" as *const u8 as *const libc::c_char,
                    202 as libc::c_int,
                );
            }
            let mut bar: libc::c_int = 0 as libc::c_int;
            while bar < input_size {
                let mut ins: [*mut libc::c_double; 16] = [
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                ];
                let mut outs: [*mut libc::c_double; 16] = [
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                ];
                i = 0 as libc::c_int;
                while i < (*info).inputs {
                    ins[i as usize] = (inputs[i as usize]).offset(bar as isize);
                    i += 1;
                    i;
                }
                i = 0 as libc::c_int;
                while i < (*info).outputs {
                    let mut progress: libc::c_int = ti_stream_get_progress(stream);
                    if progress < 0 as libc::c_int {
                        progress = 0 as libc::c_int;
                    }
                    outs[i as usize] = (outputs[i as usize]).offset(progress as isize);
                    i += 1;
                    i;
                }
                let mut stride: libc::c_int = step;
                if stride + bar >= input_size {
                    stride = input_size - bar;
                }
                let ret_1: libc::c_int = ((*info).stream_run)
                    .unwrap()(
                    stream,
                    stride,
                    ins.as_mut_ptr() as *const *const libc::c_double,
                    outs.as_mut_ptr(),
                );
                ltests += 1;
                ltests;
                if !(ret_1 == 0 as libc::c_int) {
                    lfails += 1;
                    lfails;
                    printf(
                        b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                        b"smoke.c\0" as *const u8 as *const libc::c_char,
                        222 as libc::c_int,
                    );
                }
                bar += step;
            }
            let mut output_size_1: libc::c_int = input_size
                - ((*info).start).unwrap()(options.as_mut_ptr());
            if output_size_1 < 0 as libc::c_int {
                output_size_1 = 0 as libc::c_int;
            }
            ltests += 1;
            ltests;
            if ti_stream_get_progress(stream) != output_size_1 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
                    b"smoke.c\0" as *const u8 as *const libc::c_char,
                    227 as libc::c_int,
                    ti_stream_get_progress(stream),
                    output_size_1,
                );
            }
            i = 0 as libc::c_int;
            while i < (*info).outputs {
                compare_arrays(
                    answers[i as usize],
                    outputs[i as usize],
                    answer_size,
                    output_size_1,
                );
                i += 1;
                i;
            }
            ((*info).stream_free).unwrap()(stream);
            step_i += 1;
            step_i;
        }
    }
    i = 0 as libc::c_int;
    while i < (*info).inputs {
        free(inputs[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*info).outputs {
        free(answers[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*info).outputs {
        free(outputs[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn test_candles(mut fp: *mut FILE, mut count: libc::c_int) {
    let mut inputs: [*mut libc::c_double; 16] = [
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
        0 as *mut libc::c_double,
    ];
    let mut input_size: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        inputs[i
            as usize] = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(1024 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_double;
        input_size = get_array(fp, inputs[i as usize]);
        i += 1;
        i;
    }
    let mut config: tc_config = tc_config {
        period: 0,
        body_none: 0.,
        body_short: 0.,
        body_long: 0.,
        wick_none: 0.,
        wick_long: 0.,
        near: 0.,
    };
    tc_config_set_to_default(&mut config);
    let mut output: *mut tc_result = tc_result_new();
    tc_run(
        (((1 as libc::c_long) << 26 as libc::c_int) - 1 as libc::c_int as libc::c_long)
            as tc_set,
        input_size,
        inputs.as_mut_ptr() as *const *const TC_REAL,
        &mut config,
        output,
    );
    static mut buf: [libc::c_char; 4096] = [0; 4096];
    while !(feof(fp) != 0) {
        fgets(buf.as_mut_ptr(), 4096 as libc::c_int, fp);
        if buf[0 as libc::c_int as usize] as libc::c_int != '!' as i32
            && ((buf[0 as libc::c_int as usize] as libc::c_int) < 'a' as i32
                || buf[0 as libc::c_int as usize] as libc::c_int > 'z' as i32)
        {
            break;
        }
        let mut name: *mut libc::c_char = strtok(
            buf.as_mut_ptr(),
            b" \0" as *const u8 as *const libc::c_char,
        );
        let mut negate: libc::c_int = 0 as libc::c_int;
        if buf[0 as libc::c_int as usize] as libc::c_int == '!' as i32 {
            negate = 1 as libc::c_int;
            name = name.offset(1);
            name;
        }
        let mut info: *const tc_candle_info = tc_find_candle(name);
        ltests += 1;
        ltests;
        if info.is_null() {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"smoke.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
            );
        }
        let mut num: *mut libc::c_char = 0 as *mut libc::c_char;
        loop {
            num = strtok(
                0 as *mut libc::c_char,
                b" \0" as *const u8 as *const libc::c_char,
            );
            if num.is_null() {
                break;
            }
            let pos: libc::c_int = atoi(num);
            if negate != 0 {
                ltests += 1;
                ltests;
                if tc_result_at(output, pos) & (*info).pattern != 0 {
                    lfails += 1;
                    lfails;
                    printf(
                        b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                        b"smoke.c\0" as *const u8 as *const libc::c_char,
                        290 as libc::c_int,
                    );
                }
            } else {
                ltests += 1;
                ltests;
                if tc_result_at(output, pos) & (*info).pattern == 0 {
                    lfails += 1;
                    lfails;
                    printf(
                        b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                        b"smoke.c\0" as *const u8 as *const libc::c_char,
                        292 as libc::c_int,
                    );
                }
            }
        }
        let cnd_num: libc::c_int = info.offset_from(tc_candles.as_mut_ptr())
            as libc::c_long as libc::c_int;
        if count != 0 {
            tested_cnd[cnd_num as usize] = 1 as libc::c_int;
        }
    }
    tc_result_free(output);
    free(inputs[0 as libc::c_int as usize] as *mut libc::c_void);
    free(inputs[1 as libc::c_int as usize] as *mut libc::c_void);
    free(inputs[2 as libc::c_int as usize] as *mut libc::c_void);
    free(inputs[3 as libc::c_int as usize] as *mut libc::c_void);
}
pub unsafe extern "C" fn test(mut fname: *const libc::c_char, mut count: libc::c_int) {
    printf(b"%s:\n\0" as *const u8 as *const libc::c_char, fname);
    let mut fp: *mut FILE = fopen(fname, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        ltests += 1;
        ltests;
        if 0 as libc::c_int == 0 {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"smoke.c\0" as *const u8 as *const libc::c_char,
                316 as libc::c_int,
            );
        }
        return;
    }
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        line = next_line(fp);
        if line.is_null() {
            break;
        }
        if (*line.offset(0 as libc::c_int as isize) as libc::c_int) < 'a' as i32
            || *line.offset(0 as libc::c_int as isize) as libc::c_int > 'z' as i32
        {
            ltests += 1;
            ltests;
            if 0 as libc::c_int == 0 {
                lfails += 1;
                lfails;
                printf(
                    b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                    b"smoke.c\0" as *const u8 as *const libc::c_char,
                    326 as libc::c_int,
                );
            }
        } else {
            let mut name: *mut libc::c_char = strtok(
                line,
                b" \n\r\0" as *const u8 as *const libc::c_char,
            );
            if strcmp(name, b"candle\0" as *const u8 as *const libc::c_char) == 0 {
                let ts: libc::c_int = ltests;
                let fs: libc::c_int = lfails;
                let start: clock_t = clock();
                printf(b"\t%-16s\0" as *const u8 as *const libc::c_char, name);
                test_candles(fp, count);
                printf(
                    b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8
                        as *const libc::c_char,
                    ltests - ts - (lfails - fs),
                    lfails - fs,
                    ((clock() - start) * 1000 as libc::c_int as libc::c_long
                        / 1000000 as libc::c_int as __clock_t) as libc::c_int,
                );
            } else {
                let ts_0: libc::c_int = ltests;
                let fs_0: libc::c_int = lfails;
                let start_0: clock_t = clock();
                printf(b"\t%-16s\0" as *const u8 as *const libc::c_char, name);
                test_ind_name(fp, name, count);
                printf(
                    b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8
                        as *const libc::c_char,
                    ltests - ts_0 - (lfails - fs_0),
                    lfails - fs_0,
                    ((clock() - start_0) * 1000 as libc::c_int as libc::c_long
                        / 1000000 as libc::c_int as __clock_t) as libc::c_int,
                );
            }
        }
    }
    fclose(fp);
}
pub unsafe extern "C" fn test_version() {
    ltests += 1;
    ltests;
    if !(strcmp(b"0.9.2\0" as *const u8 as *const libc::c_char, ti_version())
        == 0 as libc::c_int)
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            345 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if !(1645649572 as libc::c_int as libc::c_long == ti_build()) {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if !(104 as libc::c_int == ti_indicator_count()) {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if !(strcmp(b"0.9.2\0" as *const u8 as *const libc::c_char, tc_version())
        == 0 as libc::c_int)
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if !(1645649572 as libc::c_int as libc::c_long == tc_build()) {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if !(26 as libc::c_int == tc_candle_count()) {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            351 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if !(strcmp(
        b"0.9.2\0" as *const u8 as *const libc::c_char,
        b"0.9.2\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int)
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            353 as libc::c_int,
        );
    }
    ltests += 1;
    ltests;
    if 1645649572 as libc::c_int != 1645649572 as libc::c_int {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            354 as libc::c_int,
            1645649572 as libc::c_int,
            1645649572 as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn test_buffer() {
    let mut b: *mut ti_buffer = ti_buffer_new(3 as libc::c_int);
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 5.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 5.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 5.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            (*b).sum,
            5.0f64,
        );
    }
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 5.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 5.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 10.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int,
            (*b).sum,
            10.0f64,
        );
    }
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 1.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 1.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 11.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int,
            (*b).sum,
            11.0f64,
        );
    }
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 1.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 1.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 7.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int,
            (*b).sum,
            7.0f64,
        );
    }
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 3.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 3.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 5.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int,
            (*b).sum,
            5.0f64,
        );
    }
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 1.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 1.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 5.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            365 as libc::c_int,
            (*b).sum,
            5.0f64,
        );
    }
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 2.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 2.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 6.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int,
            (*b).sum,
            6.0f64,
        );
    }
    if (*b).pushes >= (*b).size {
        (*b).sum -= *((*b).vals).as_mut_ptr().offset((*b).index as isize);
    }
    (*b).sum += 3.0f64;
    *((*b).vals).as_mut_ptr().offset((*b).index as isize) = 3.0f64;
    (*b).pushes += 1 as libc::c_int;
    (*b).index = (*b).index + 1 as libc::c_int;
    if (*b).index >= (*b).size {
        (*b).index = 0 as libc::c_int;
    }
    ltests += 1;
    ltests;
    if fabs((*b).sum - 6.0f64) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int,
            (*b).sum,
            6.0f64,
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        *((*b).vals)
            .as_mut_ptr()
            .offset(
                (((*b).index + (*b).size - 1 as libc::c_int + 0 as libc::c_int)
                    % (*b).size) as isize,
            ) - 3.0f64,
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            369 as libc::c_int,
            *((*b).vals)
                .as_mut_ptr()
                .offset(
                    (((*b).index + (*b).size - 1 as libc::c_int + 0 as libc::c_int)
                        % (*b).size) as isize,
                ),
            3.0f64,
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        *((*b).vals)
            .as_mut_ptr()
            .offset(
                (((*b).index + (*b).size - 1 as libc::c_int + -(1 as libc::c_int))
                    % (*b).size) as isize,
            ) - 2.0f64,
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            370 as libc::c_int,
            *((*b).vals)
                .as_mut_ptr()
                .offset(
                    (((*b).index + (*b).size - 1 as libc::c_int + -(1 as libc::c_int))
                        % (*b).size) as isize,
                ),
            2.0f64,
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        *((*b).vals)
            .as_mut_ptr()
            .offset(
                (((*b).index + (*b).size - 1 as libc::c_int + -(2 as libc::c_int))
                    % (*b).size) as isize,
            ) - 1.0f64,
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int,
            *((*b).vals)
                .as_mut_ptr()
                .offset(
                    (((*b).index + (*b).size - 1 as libc::c_int + -(2 as libc::c_int))
                        % (*b).size) as isize,
                ),
            1.0f64,
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        *((*b).vals)
            .as_mut_ptr()
            .offset(
                (((*b).index + (*b).size - 1 as libc::c_int + -(3 as libc::c_int))
                    % (*b).size) as isize,
            ) - 3.0f64,
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            *((*b).vals)
                .as_mut_ptr()
                .offset(
                    (((*b).index + (*b).size - 1 as libc::c_int + -(3 as libc::c_int))
                        % (*b).size) as isize,
                ),
            3.0f64,
        );
    }
    ti_buffer_free(b);
}
pub unsafe extern "C" fn test_localbuffer() {
    let mut stream: s = {
        let mut init = s {
            buf_info: {
                let mut init = C2RustUnnamed {
                    padding: 0 as libc::c_int as libc::c_double,
                    zero: [0; 4],
                    offset_a: 0,
                    size_a: 0,
                    index_a: 0,
                    padding_a: 0,
                };
                init
            },
            padding: [0.; 5],
        };
        init
    };
    stream
        .buf_info
        .offset_a = *(&mut stream.buf_info.offset_a as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *(&mut stream.buf_info.offset_a as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize));
    stream.buf_info.size_a = 5 as libc::c_int;
    stream.buf_info.index_a = -(1 as libc::c_int);
    ltests += 1;
    ltests;
    if *((&mut stream.buf_info as *mut C2RustUnnamed).offset(1 as libc::c_int as isize)
        as *mut libc::c_int)
        .offset(-(3 as libc::c_int as isize))
        + *((&mut stream.buf_info as *mut C2RustUnnamed)
            .offset(1 as libc::c_int as isize) as *mut libc::c_int)
            .offset(-(4 as libc::c_int as isize)) != 5 as libc::c_int
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int,
            *((&mut stream.buf_info as *mut C2RustUnnamed)
                .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                .offset(-(3 as libc::c_int as isize))
                + *((&mut stream.buf_info as *mut C2RustUnnamed)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_int)
                    .offset(-(4 as libc::c_int as isize)),
            5 as libc::c_int,
        );
    }
    let mut idx: libc::c_int = stream.buf_info.index_a + 1 as libc::c_int;
    if idx == stream.buf_info.size_a {
        idx = 0 as libc::c_int;
    }
    *((&mut stream.buf_info as *mut C2RustUnnamed).offset(1 as libc::c_int as isize)
        as *mut libc::c_double)
        .offset(stream.buf_info.offset_a as isize)
        .offset(idx as isize) = 100 as libc::c_int as libc::c_double;
    stream.buf_info.index_a = idx;
    let mut get: libc::c_double = 0.;
    let mut idx_0: libc::c_int = stream.buf_info.index_a + 0 as libc::c_int;
    while idx_0 >= stream.buf_info.size_a {
        idx_0 -= stream.buf_info.size_a;
    }
    while idx_0 < 0 as libc::c_int {
        idx_0 += stream.buf_info.size_a;
    }
    get = *((&mut stream.buf_info as *mut C2RustUnnamed)
        .offset(1 as libc::c_int as isize) as *mut libc::c_double)
        .offset(stream.buf_info.offset_a as isize)
        .offset(idx_0 as isize);
    ltests += 1;
    ltests;
    if fabs(get - 100 as libc::c_int as libc::c_double) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"smoke.c\0" as *const u8 as *const libc::c_char,
            400 as libc::c_int,
            get,
            100 as libc::c_int as libc::c_double,
        );
    }
}
unsafe fn main_0() -> libc::c_int {
    printf(b"TI TEST SUITE\n\0" as *const u8 as *const libc::c_char);
    let ts: libc::c_int = ltests;
    let fs: libc::c_int = lfails;
    let start: clock_t = clock();
    printf(
        b"\t%-16s\0" as *const u8 as *const libc::c_char,
        b"buffer\0" as *const u8 as *const libc::c_char,
    );
    test_buffer();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts - (lfails - fs),
        lfails - fs,
        ((clock() - start) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_0: libc::c_int = ltests;
    let fs_0: libc::c_int = lfails;
    let start_0: clock_t = clock();
    printf(
        b"\t%-16s\0" as *const u8 as *const libc::c_char,
        b"localbuffer\0" as *const u8 as *const libc::c_char,
    );
    test_localbuffer();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_0 - (lfails - fs_0),
        lfails - fs_0,
        ((clock() - start_0) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_1: libc::c_int = ltests;
    let fs_1: libc::c_int = lfails;
    let start_1: clock_t = clock();
    printf(
        b"\t%-16s\0" as *const u8 as *const libc::c_char,
        b"version\0" as *const u8 as *const libc::c_char,
    );
    test_version();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_1 - (lfails - fs_1),
        lfails - fs_1,
        ((clock() - start_1) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    test(b"tests/untest.txt\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    test(b"tests/atoz.txt\0" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    test(b"tests/extra.txt\0" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    test(b"tests/candles.txt\0" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 104 as libc::c_int {
        if tested_ind[i as usize] == 0 {
            printf(
                b"WARNING: no test for %s\n\0" as *const u8 as *const libc::c_char,
                (*ti_indicators.as_mut_ptr().offset(i as isize)).name,
            );
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 26 as libc::c_int {
        if tested_cnd[i_0 as usize] == 0 {
            printf(
                b"WARNING: no test for %s\n\0" as *const u8 as *const libc::c_char,
                (*tc_candles.as_mut_ptr().offset(i_0 as isize)).name,
            );
        }
        i_0 += 1;
        i_0;
    }
    if lfails == 0 as libc::c_int {
        printf(
            b"ALL TESTS PASSED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            ltests,
            ltests,
        );
    } else {
        printf(
            b"%d TESTS FAILED (of %d)\n\0" as *const u8 as *const libc::c_char,
            lfails,
            ltests,
        );
    }
    return (lfails != 0 as libc::c_int) as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
