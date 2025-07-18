use ::libc;
extern "C" {
    pub type ti_stream;
    pub type tc_result;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut ti_indicators: [ti_indicator_info; 0];
    fn ti_find_indicator(name: *const libc::c_char) -> *const ti_indicator_info;
    static mut tc_candles: [tc_candle_info; 0];
    fn tc_find_candle(name: *const libc::c_char) -> *const tc_candle_info;
    fn tc_config_set_to_default(config: *mut tc_config);
    fn tc_result_new() -> *mut tc_result;
    fn tc_result_free(result: *mut tc_result);
    fn tc_result_count(result: *const tc_result) -> libc::c_int;
    fn tc_result_pattern_count(result: *const tc_result) -> libc::c_int;
    fn tc_result_get(result: *const tc_result, index: libc::c_int) -> tc_hit;
    fn tc_result_at(result: *const tc_result, index: libc::c_int) -> tc_set;
}
pub type __uint64_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_hit {
    pub index: libc::c_int,
    pub patterns: tc_set,
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
pub static mut optionsd_small: [libc::c_double; 17] = [
    -(20 as libc::c_int) as libc::c_double,
    -(2 as libc::c_int) as libc::c_double,
    -(1 as libc::c_int) as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0.7f64,
    1 as libc::c_int as libc::c_double,
    2 as libc::c_int as libc::c_double,
    3 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
    6 as libc::c_int as libc::c_double,
    7 as libc::c_int as libc::c_double,
    8 as libc::c_int as libc::c_double,
    10 as libc::c_int as libc::c_double,
    20 as libc::c_int as libc::c_double,
    100 as libc::c_int as libc::c_double,
    999 as libc::c_int as libc::c_double,
];
pub static mut optionsd_large: [libc::c_double; 5] = [
    -(5 as libc::c_int) as libc::c_double,
    0 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    2 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
];
pub static mut dummy_in: [libc::c_double; 20] = [
    1 as libc::c_int as libc::c_double,
    2 as libc::c_int as libc::c_double,
    3 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
    6 as libc::c_int as libc::c_double,
    7 as libc::c_int as libc::c_double,
    8 as libc::c_int as libc::c_double,
    9 as libc::c_int as libc::c_double,
    10 as libc::c_int as libc::c_double,
    11 as libc::c_int as libc::c_double,
    12 as libc::c_int as libc::c_double,
    13 as libc::c_int as libc::c_double,
    14 as libc::c_int as libc::c_double,
    15 as libc::c_int as libc::c_double,
    16 as libc::c_int as libc::c_double,
    17 as libc::c_int as libc::c_double,
    18 as libc::c_int as libc::c_double,
    19 as libc::c_int as libc::c_double,
    20 as libc::c_int as libc::c_double,
];
pub static mut dummy_in0: [libc::c_double; 20] = [
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
    0 as libc::c_int as libc::c_double,
];
pub static mut dummy_ot: [libc::c_double; 20] = [
    1 as libc::c_int as libc::c_double,
    2 as libc::c_int as libc::c_double,
    3 as libc::c_int as libc::c_double,
    4 as libc::c_int as libc::c_double,
    5 as libc::c_int as libc::c_double,
    6 as libc::c_int as libc::c_double,
    7 as libc::c_int as libc::c_double,
    8 as libc::c_int as libc::c_double,
    9 as libc::c_int as libc::c_double,
    10 as libc::c_int as libc::c_double,
    11 as libc::c_int as libc::c_double,
    12 as libc::c_int as libc::c_double,
    13 as libc::c_int as libc::c_double,
    14 as libc::c_int as libc::c_double,
    15 as libc::c_int as libc::c_double,
    16 as libc::c_int as libc::c_double,
    17 as libc::c_int as libc::c_double,
    18 as libc::c_int as libc::c_double,
    19 as libc::c_int as libc::c_double,
    20 as libc::c_int as libc::c_double,
];
pub static mut input_size: libc::c_int = 0;
pub static mut sizes: [libc::c_int; 7] = [
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    8 as libc::c_int,
    20 as libc::c_int,
];
pub static mut sizes_count: libc::c_int = 0;
pub unsafe extern "C" fn banner() {
    printf(
        b"  ______ _    _ __________________ _____  \n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" |  ____| |  | |___  /___  /  ____|  __ \\ \n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" | |__  | |  | |  / /   / /| |__  | |__) |\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" |  __| | |  | | / /   / / |  __| |  _  / \n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" | |    | |__| |/ /__ / /__| |____| | \\ \\ \n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b" |_|     \\____//_____/_____|______|_|  \\_\\\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"This program tries each indicator with a lot of options.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"It's good for checking for crashes or assertion failures.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Use with a memory tool to check for memory errors in ti.\n\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub static mut errors_cnt: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn check_output(
    mut info: *const ti_indicator_info,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) {
    let s: libc::c_int = ((*info).start).unwrap()(options);
    let mut o: libc::c_int = 0;
    o = 0 as libc::c_int;
    while o < (*info).outputs {
        let mut max: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut min: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < size - s {
            let op: libc::c_double = *(*outputs.offset(o as isize)).offset(i as isize);
            let in_0: libc::c_double = *(*inputs.offset(0 as libc::c_int as isize))
                .offset((i + s) as isize);
            max = if in_0 > max { in_0 } else { max };
            min = if in_0 < min { in_0 } else { min };
            match (*info).type_0 {
                1 => {
                    if op > 4 as libc::c_int as libc::c_double
                        && op > max * 1.5f64 + 2 as libc::c_int as libc::c_double
                        || op < -(4 as libc::c_int) as libc::c_double
                            && op < min * 0.5f64 - 3 as libc::c_int as libc::c_double
                    {
                        let mut k: libc::c_int = 0;
                        let mut j: libc::c_int = 0;
                        printf(b"\nInputs:\0" as *const u8 as *const libc::c_char);
                        k = 0 as libc::c_int;
                        while k < input_size {
                            printf(
                                b" %f\0" as *const u8 as *const libc::c_char,
                                *(*inputs.offset(0 as libc::c_int as isize))
                                    .offset(k as isize),
                            );
                            k += 1;
                            k;
                        }
                        printf(b"\nOptions:\0" as *const u8 as *const libc::c_char);
                        k = 0 as libc::c_int;
                        while k < (*info).options {
                            printf(
                                b" %f\0" as *const u8 as *const libc::c_char,
                                *options.offset(k as isize),
                            );
                            k += 1;
                            k;
                        }
                        j = 0 as libc::c_int;
                        while j < (*info).outputs {
                            printf(
                                b"\nOutput[%d]:\0" as *const u8 as *const libc::c_char,
                                j,
                            );
                            k = 0 as libc::c_int;
                            while k < input_size {
                                printf(
                                    b" %f\0" as *const u8 as *const libc::c_char,
                                    *(*outputs.offset(j as isize)).offset(k as isize),
                                );
                                k += 1;
                                k;
                            }
                            j += 1;
                            j;
                        }
                        printf(
                            b"\nERROR Output is out of range for input: input: %f output: %f\n\0"
                                as *const u8 as *const libc::c_char,
                            in_0,
                            op,
                        );
                        errors_cnt += 1 as libc::c_int;
                        if 0 as libc::c_int != 0 {} else {
                            __assert_fail(
                                b"0\0" as *const u8 as *const libc::c_char,
                                b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                                102 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 106],
                                    &[libc::c_char; 106],
                                >(
                                    b"void check_output(const ti_indicator_info *, int, const double *const *, const double *, double *const *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                    }
                }
                _ => {}
            }
            i += 1;
            i;
        }
        o += 1;
        o;
    }
}
pub unsafe extern "C" fn stress_indicator(mut info: *const ti_indicator_info) {
    let opt_count: libc::c_int = (*info).options;
    printf(
        b"%s (%s) (%d options)\n\0" as *const u8 as *const libc::c_char,
        (*info).name,
        (*info).full_name,
        opt_count,
    );
    let mut inputs: [*const libc::c_double; 16] = [
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
    ];
    let mut inputs0: [*const libc::c_double; 16] = [
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
        0 as *const libc::c_double,
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
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        inputs[i
            as usize] = if i < (*info).inputs {
            dummy_in.as_mut_ptr()
        } else {
            0 as *mut libc::c_double
        };
        inputs0[i
            as usize] = if i < (*info).inputs {
            dummy_in0.as_mut_ptr()
        } else {
            0 as *mut libc::c_double
        };
        outputs[i
            as usize] = if i < (*info).outputs {
            dummy_ot.as_mut_ptr()
        } else {
            0 as *mut libc::c_double
        };
        i += 1;
        i;
    }
    let mut options_index: [libc::c_int; 17] = [
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
    ];
    let mut options: [libc::c_double; 16] = [0.; 16];
    let mut choices: libc::c_int = 0;
    let mut optionsd: *mut libc::c_double = 0 as *mut libc::c_double;
    if opt_count <= 4 as libc::c_int {
        choices = (::std::mem::size_of::<[libc::c_double; 17]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int;
        optionsd = optionsd_small.as_mut_ptr();
    } else {
        choices = (::std::mem::size_of::<[libc::c_double; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as libc::c_int;
        optionsd = optionsd_large.as_mut_ptr();
    }
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    loop {
        count += 1;
        count;
        j = 0 as libc::c_int;
        while j < opt_count {
            options[j as usize] = *optionsd.offset(options_index[j as usize] as isize);
            printf(b" %f\0" as *const u8 as *const libc::c_char, options[j as usize]);
            j += 1;
            j;
        }
        let rz: libc::c_int = ((*info).indicator)
            .unwrap()(
            input_size,
            inputs0.as_mut_ptr(),
            options.as_mut_ptr(),
            outputs.as_mut_ptr(),
        );
        if rz == 0 as libc::c_int {
            check_output(
                info,
                input_size,
                inputs0.as_mut_ptr(),
                options.as_mut_ptr(),
                outputs.as_mut_ptr(),
            );
        }
        let mut s: libc::c_int = 0 as libc::c_int;
        while s < sizes_count {
            let size: libc::c_int = sizes[s as usize];
            printf(b" %d\0" as *const u8 as *const libc::c_char, size);
            let r: libc::c_int = ((*info).indicator)
                .unwrap()(
                size,
                inputs.as_mut_ptr(),
                options.as_mut_ptr(),
                outputs.as_mut_ptr(),
            );
            if r == 0 as libc::c_int {
                check_output(
                    info,
                    size,
                    inputs.as_mut_ptr(),
                    options.as_mut_ptr(),
                    outputs.as_mut_ptr(),
                );
            }
            if ((*info).indicator_ref).is_some() {
                printf(b" r\0" as *const u8 as *const libc::c_char);
                let rr: libc::c_int = ((*info).indicator_ref)
                    .unwrap()(
                    size,
                    inputs.as_mut_ptr(),
                    options.as_mut_ptr(),
                    outputs.as_mut_ptr(),
                );
                if rr == 0 as libc::c_int {
                    check_output(
                        info,
                        size,
                        inputs.as_mut_ptr(),
                        options.as_mut_ptr(),
                        outputs.as_mut_ptr(),
                    );
                }
            }
            if ((*info).stream_new).is_some() {
                let mut stream: *mut ti_stream = 0 as *mut ti_stream;
                let rs: libc::c_int = ((*info).stream_new)
                    .unwrap()(options.as_mut_ptr(), &mut stream);
                if rs == 0 as libc::c_int && !stream.is_null() {
                    printf(b" s\0" as *const u8 as *const libc::c_char);
                    let mut rsc: libc::c_int = ((*info).stream_run)
                        .unwrap()(
                        stream,
                        size,
                        inputs.as_mut_ptr(),
                        outputs.as_mut_ptr(),
                    );
                    if rsc == 0 as libc::c_int {
                        check_output(
                            info,
                            size,
                            inputs.as_mut_ptr(),
                            options.as_mut_ptr(),
                            outputs.as_mut_ptr(),
                        );
                    }
                    let mut repeat: libc::c_int = 0 as libc::c_int;
                    while repeat < 5 as libc::c_int {
                        rsc = ((*info).stream_run)
                            .unwrap()(
                            stream,
                            size,
                            inputs.as_mut_ptr(),
                            outputs.as_mut_ptr(),
                        );
                        repeat += 1;
                        repeat;
                    }
                    ((*info).stream_free).unwrap()(stream);
                }
            }
            s += 1;
            s;
        }
        printf(
            b"\r                                                                          \r\0"
                as *const u8 as *const libc::c_char,
        );
        j = 0 as libc::c_int;
        loop {
            options_index[j
                as usize] = (options_index[j as usize] + 1 as libc::c_int) % choices;
            let fresh0 = j;
            j = j + 1;
            if !(options_index[fresh0 as usize] == 0 as libc::c_int) {
                break;
            }
        }
        if !(j <= opt_count) {
            break;
        }
    }
    if 0.1f64
        > fabs(
            count as libc::c_double
                - pow(choices as libc::c_double, opt_count as libc::c_double),
        )
    {} else {
        __assert_fail(
            b".1 > fabs(count - (pow(choices, opt_count)))\0" as *const u8
                as *const libc::c_char,
            b"fuzzer.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"void stress_indicator(const ti_indicator_info *)\0"))
                .as_ptr(),
        );
    };
}
pub unsafe extern "C" fn stress_candle(mut info: *const tc_candle_info) {
    printf(
        b"%s (%s)\n\0" as *const u8 as *const libc::c_char,
        (*info).name,
        (*info).full_name,
    );
    let mut inputs: [*const libc::c_double; 4] = [
        dummy_in.as_mut_ptr() as *const libc::c_double,
        dummy_in.as_mut_ptr() as *const libc::c_double,
        dummy_in.as_mut_ptr() as *const libc::c_double,
        dummy_in.as_mut_ptr() as *const libc::c_double,
    ];
    let mut inputs0: [*const libc::c_double; 4] = [
        dummy_in0.as_mut_ptr() as *const libc::c_double,
        dummy_in0.as_mut_ptr() as *const libc::c_double,
        dummy_in0.as_mut_ptr() as *const libc::c_double,
        dummy_in0.as_mut_ptr() as *const libc::c_double,
    ];
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
    let mut s: libc::c_int = 0 as libc::c_int;
    while s < sizes_count {
        let size: libc::c_int = sizes[s as usize];
        ((*info).candle)
            .unwrap()(
            size,
            inputs.as_mut_ptr() as *const *const TC_REAL,
            &mut config,
            output,
        );
        if tc_result_count(output) <= size {} else {
            __assert_fail(
                b"tc_result_count(output) <= size\0" as *const u8 as *const libc::c_char,
                b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void stress_candle(const tc_candle_info *)\0"))
                    .as_ptr(),
            );
        };
        if tc_result_pattern_count(output) <= size {} else {
            __assert_fail(
                b"tc_result_pattern_count(output) <= size\0" as *const u8
                    as *const libc::c_char,
                b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                221 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void stress_candle(const tc_candle_info *)\0"))
                    .as_ptr(),
            );
        };
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < tc_result_count(output) {
            let mut hit: tc_hit = tc_result_get(output, i);
            if hit.patterns
                != (((1 as libc::c_long) << 26 as libc::c_int)
                    - 1 as libc::c_int as libc::c_long) as libc::c_ulong
            {} else {
                __assert_fail(
                    b"hit.patterns != TC_ALL\0" as *const u8 as *const libc::c_char,
                    b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                    225 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"void stress_candle(const tc_candle_info *)\0"))
                        .as_ptr(),
                );
            };
            if hit.index >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"hit.index >= 0\0" as *const u8 as *const libc::c_char,
                    b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                    226 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"void stress_candle(const tc_candle_info *)\0"))
                        .as_ptr(),
                );
            };
            if hit.index < size {} else {
                __assert_fail(
                    b"hit.index < size\0" as *const u8 as *const libc::c_char,
                    b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                    227 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"void stress_candle(const tc_candle_info *)\0"))
                        .as_ptr(),
                );
            };
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < size {
            let mut patterns: tc_set = tc_result_at(output, i_0);
            if patterns
                != (((1 as libc::c_long) << 26 as libc::c_int)
                    - 1 as libc::c_int as libc::c_long) as libc::c_ulong
            {} else {
                __assert_fail(
                    b"patterns != TC_ALL\0" as *const u8 as *const libc::c_char,
                    b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                    231 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"void stress_candle(const tc_candle_info *)\0"))
                        .as_ptr(),
                );
            };
            i_0 += 1;
            i_0;
        }
        ((*info).candle)
            .unwrap()(
            size,
            inputs0.as_mut_ptr() as *const *const TC_REAL,
            &mut config,
            output,
        );
        if tc_result_count(output) <= size {} else {
            __assert_fail(
                b"tc_result_count(output) <= size\0" as *const u8 as *const libc::c_char,
                b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                235 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void stress_candle(const tc_candle_info *)\0"))
                    .as_ptr(),
            );
        };
        if tc_result_pattern_count(output) <= size {} else {
            __assert_fail(
                b"tc_result_pattern_count(output) <= size\0" as *const u8
                    as *const libc::c_char,
                b"fuzzer.c\0" as *const u8 as *const libc::c_char,
                236 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void stress_candle(const tc_candle_info *)\0"))
                    .as_ptr(),
            );
        };
        s += 1;
        s;
    }
    tc_result_free(output);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    banner();
    if argc > 1 as libc::c_int {
        let mut ind_info: *const ti_indicator_info = ti_find_indicator(
            *argv.offset(1 as libc::c_int as isize),
        );
        if !ind_info.is_null() {
            stress_indicator(ind_info);
        }
        let mut cnd_info: *const tc_candle_info = tc_find_candle(
            *argv.offset(1 as libc::c_int as isize),
        );
        if !cnd_info.is_null() {
            stress_candle(cnd_info);
        }
    } else {
        let mut ind_info_0: *const ti_indicator_info = ti_indicators.as_mut_ptr();
        loop {
            stress_indicator(ind_info_0);
            ind_info_0 = ind_info_0.offset(1);
            if ((*ind_info_0).name).is_null() {
                break;
            }
        }
        let mut cnd_info_0: *const tc_candle_info = tc_candles.as_mut_ptr();
        loop {
            stress_candle(cnd_info_0);
            cnd_info_0 = cnd_info_0.offset(1);
            if ((*cnd_info_0).name).is_null() {
                break;
            }
        }
    }
    printf(
        b"\r                                                  \rDone\n\0" as *const u8
            as *const libc::c_char,
    );
    return if errors_cnt != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    input_size = (::std::mem::size_of::<[libc::c_double; 20]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        as libc::c_int;
    sizes_count = (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
