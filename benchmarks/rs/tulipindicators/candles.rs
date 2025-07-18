use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type TC_REAL = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_result {
    pub progress: libc::c_int,
    pub space: libc::c_int,
    pub count: libc::c_int,
    pub pattern_count: libc::c_int,
    pub hits: *mut tc_hit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tc_hit {
    pub index: libc::c_int,
    pub patterns: tc_set,
}
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
pub unsafe extern "C" fn tc_version() -> *const libc::c_char {
    return b"0.9.2\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn tc_build() -> libc::c_long {
    return 1645649572 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn tc_candle_count() -> libc::c_int {
    return 26 as libc::c_int;
}
pub unsafe extern "C" fn tc_config_default() -> *const tc_config {
    static mut default_config: tc_config = {
        let mut init = tc_config {
            period: 10 as libc::c_int,
            body_none: 0.05f64,
            body_short: 0.5f64,
            body_long: 1.4f64,
            wick_none: 0.05f64,
            wick_long: 0.6f64,
            near: 0.3f64,
        };
        init
    };
    return &mut default_config;
}
pub static mut tc_candles: [tc_candle_info; 27] = unsafe {
    [
        {
            let mut init = tc_candle_info {
                name: b"abandoned_baby_bear\0" as *const u8 as *const libc::c_char,
                full_name: b"Abandoned Baby Bear\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 0 as libc::c_int) as tc_set,
                candle: Some(
                    tc_abandoned_baby_bear
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"abandoned_baby_bull\0" as *const u8 as *const libc::c_char,
                full_name: b"Abandoned Baby Bull\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 1 as libc::c_int) as tc_set,
                candle: Some(
                    tc_abandoned_baby_bull
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"big_black_candle\0" as *const u8 as *const libc::c_char,
                full_name: b"Big Black Candle\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 2 as libc::c_int) as tc_set,
                candle: Some(
                    tc_big_black_candle
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"big_white_candle\0" as *const u8 as *const libc::c_char,
                full_name: b"Big White Candle\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 3 as libc::c_int) as tc_set,
                candle: Some(
                    tc_big_white_candle
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"black_marubozu\0" as *const u8 as *const libc::c_char,
                full_name: b"Black Marubozu\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 4 as libc::c_int) as tc_set,
                candle: Some(
                    tc_black_marubozu
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 5 as libc::c_int) as tc_set,
                candle: Some(
                    tc_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"dragonfly_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Dragonfly Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 6 as libc::c_int) as tc_set,
                candle: Some(
                    tc_dragonfly_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"engulfing_bear\0" as *const u8 as *const libc::c_char,
                full_name: b"Engulfing Bear\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 7 as libc::c_int) as tc_set,
                candle: Some(
                    tc_engulfing_bear
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"engulfing_bull\0" as *const u8 as *const libc::c_char,
                full_name: b"Engulfing Bull\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 8 as libc::c_int) as tc_set,
                candle: Some(
                    tc_engulfing_bull
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"evening_doji_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Evening Doji Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 9 as libc::c_int) as tc_set,
                candle: Some(
                    tc_evening_doji_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"evening_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Evening Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 10 as libc::c_int) as tc_set,
                candle: Some(
                    tc_evening_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"four_price_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Four Price Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 11 as libc::c_int) as tc_set,
                candle: Some(
                    tc_four_price_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"gravestone_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Gravestone Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 12 as libc::c_int) as tc_set,
                candle: Some(
                    tc_gravestone_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"hammer\0" as *const u8 as *const libc::c_char,
                full_name: b"Hammer\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 13 as libc::c_int) as tc_set,
                candle: Some(
                    tc_hammer
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"hanging_man\0" as *const u8 as *const libc::c_char,
                full_name: b"Hanging Man\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 14 as libc::c_int) as tc_set,
                candle: Some(
                    tc_hanging_man
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"inverted_hammer\0" as *const u8 as *const libc::c_char,
                full_name: b"Inverted Hammer\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 15 as libc::c_int) as tc_set,
                candle: Some(
                    tc_inverted_hammer
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"long_legged_doji\0" as *const u8 as *const libc::c_char,
                full_name: b"Long Legged Doji\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 16 as libc::c_int) as tc_set,
                candle: Some(
                    tc_long_legged_doji
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"marubozu\0" as *const u8 as *const libc::c_char,
                full_name: b"Marubozu\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 17 as libc::c_int) as tc_set,
                candle: Some(
                    tc_marubozu
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"morning_doji_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Morning Doji Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 18 as libc::c_int) as tc_set,
                candle: Some(
                    tc_morning_doji_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"morning_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Morning Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 19 as libc::c_int) as tc_set,
                candle: Some(
                    tc_morning_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"shooting_star\0" as *const u8 as *const libc::c_char,
                full_name: b"Shooting Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 20 as libc::c_int) as tc_set,
                candle: Some(
                    tc_shooting_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"spinning_top\0" as *const u8 as *const libc::c_char,
                full_name: b"Spinning Top\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 21 as libc::c_int) as tc_set,
                candle: Some(
                    tc_spinning_top
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"star\0" as *const u8 as *const libc::c_char,
                full_name: b"Star\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 22 as libc::c_int) as tc_set,
                candle: Some(
                    tc_star
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"three_black_crows\0" as *const u8 as *const libc::c_char,
                full_name: b"Three Black Crows\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 23 as libc::c_int) as tc_set,
                candle: Some(
                    tc_three_black_crows
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"three_white_soldiers\0" as *const u8 as *const libc::c_char,
                full_name: b"Three White Soldiers\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 24 as libc::c_int) as tc_set,
                candle: Some(
                    tc_three_white_soldiers
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: b"white_marubozu\0" as *const u8 as *const libc::c_char,
                full_name: b"White Marubozu\0" as *const u8 as *const libc::c_char,
                pattern: ((1 as libc::c_long) << 25 as libc::c_int) as tc_set,
                candle: Some(
                    tc_white_marubozu
                        as unsafe extern "C" fn(
                            libc::c_int,
                            *const *const TC_REAL,
                            *const tc_config,
                            *mut tc_result,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = tc_candle_info {
                name: 0 as *const libc::c_char,
                full_name: 0 as *const libc::c_char,
                pattern: 0 as libc::c_int as tc_set,
                candle: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn tc_find_candle(
    mut name: *const libc::c_char,
) -> *const tc_candle_info {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (::std::mem::size_of::<[tc_candle_info; 27]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<tc_candle_info>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        let c: libc::c_int = strcmp(name, tc_candles[i as usize].name);
        if c == 0 as libc::c_int {
            return tc_candles.as_mut_ptr().offset(i as isize)
        } else if c > 0 as libc::c_int {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as *const tc_candle_info;
}
pub unsafe extern "C" fn tc_get_info(mut pattern: tc_set) -> *const tc_candle_info {
    if pattern == 0 {
        return 0 as *const tc_candle_info;
    }
    let mut k: tc_set = pattern
        & !pattern.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (::std::mem::size_of::<[tc_candle_info; 27]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<tc_candle_info>() as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        if k == tc_candles[i as usize].pattern {
            return tc_candles.as_mut_ptr().offset(i as isize)
        } else if k > tc_candles[i as usize].pattern {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as *const tc_candle_info;
}
pub unsafe extern "C" fn tc_config_set_to_default(mut config: *mut tc_config) {
    memcpy(
        config as *mut libc::c_void,
        tc_config_default() as *const libc::c_void,
        ::std::mem::size_of::<tc_config>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn tc_result_new() -> *mut tc_result {
    let mut k: *mut tc_result = malloc(
        ::std::mem::size_of::<tc_result>() as libc::c_ulong,
    ) as *mut tc_result;
    if k.is_null() {
        return 0 as *mut tc_result;
    }
    memset(
        k as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tc_result>() as libc::c_ulong,
    );
    return k;
}
unsafe extern "C" fn tc_result_reset(mut result: *mut tc_result) {
    (*result).progress = 0 as libc::c_int;
    (*result).count = 0 as libc::c_int;
    (*result).pattern_count = 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_result_free(mut result: *mut tc_result) {
    if result.is_null() {
        return;
    }
    free((*result).hits as *mut libc::c_void);
    free(result as *mut libc::c_void);
}
pub unsafe extern "C" fn tc_result_count(mut result: *const tc_result) -> libc::c_int {
    return (*result).count;
}
pub unsafe extern "C" fn tc_result_pattern_count(
    mut result: *const tc_result,
) -> libc::c_int {
    return (*result).pattern_count;
}
pub unsafe extern "C" fn tc_result_get(
    mut result: *const tc_result,
    mut index: libc::c_int,
) -> tc_hit {
    if index < (*result).count {
        return *((*result).hits).offset(index as isize);
    }
    let mut hit: tc_hit = {
        let mut init = tc_hit {
            index: 0 as libc::c_int,
            patterns: 0,
        };
        init
    };
    return hit;
}
pub unsafe extern "C" fn tc_result_at(
    mut result: *const tc_result,
    mut index: libc::c_int,
) -> tc_set {
    let mut imin: libc::c_int = 0 as libc::c_int;
    let mut imax: libc::c_int = (*result).count - 1 as libc::c_int;
    if imax == 0 {
        return 0 as libc::c_int as tc_set;
    }
    while imax >= imin {
        let i: libc::c_int = imin + (imax - imin) / 2 as libc::c_int;
        if i >= 0 as libc::c_int {} else {
            __assert_fail(
                b"i >= 0\0" as *const u8 as *const libc::c_char,
                b"candles.c\0" as *const u8 as *const libc::c_char,
                219 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                    .as_ptr(),
            );
        };
        if i < (*result).count {} else {
            __assert_fail(
                b"i < result->count\0" as *const u8 as *const libc::c_char,
                b"candles.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"tc_set tc_result_at(const tc_result *, int)\0"))
                    .as_ptr(),
            );
        };
        if index == (*((*result).hits).offset(i as isize)).index {
            return (*((*result).hits).offset(i as isize)).patterns
        } else if index > (*((*result).hits).offset(i as isize)).index {
            imin = i + 1 as libc::c_int;
        } else {
            imax = i - 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int as tc_set;
}
unsafe extern "C" fn tc_result_add(
    mut result: *mut tc_result,
    mut hit: tc_hit,
) -> libc::c_int {
    (*result).pattern_count += 1;
    (*result).pattern_count;
    if (*result).count > 0 as libc::c_int {
        if (*((*result).hits).offset(((*result).count - 1 as libc::c_int) as isize))
            .index == hit.index
        {
            let ref mut fresh0 = (*((*result).hits)
                .offset(((*result).count - 1 as libc::c_int) as isize))
                .patterns;
            *fresh0 |= hit.patterns;
            return 0 as libc::c_int;
        }
    }
    if (*result).space == 0 as libc::c_int {
        (*result).space = 2048 as libc::c_int;
        (*result).count = 0 as libc::c_int;
        (*result)
            .hits = malloc(
            (::std::mem::size_of::<tc_hit>() as libc::c_ulong)
                .wrapping_mul((*result).space as libc::c_uint as libc::c_ulong),
        ) as *mut tc_hit;
    } else if (*result).space == (*result).count {
        (*result).space *= 2 as libc::c_int;
        let mut hits: *mut tc_hit = realloc(
            (*result).hits as *mut libc::c_void,
            (::std::mem::size_of::<tc_hit>() as libc::c_ulong)
                .wrapping_mul((*result).space as libc::c_uint as libc::c_ulong),
        ) as *mut tc_hit;
        if hits.is_null() {
            return 2 as libc::c_int;
        }
        (*result).hits = hits;
    }
    *((*result).hits).offset((*result).count as isize) = hit;
    (*result).count += 1;
    (*result).count;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_run(
    mut patterns: tc_set,
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    tc_result_reset(output);
    if patterns & patterns.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        if patterns == ((1 as libc::c_long) << 0 as libc::c_int) as libc::c_ulong {
            return tc_abandoned_baby_bear(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 1 as libc::c_int) as libc::c_ulong {
            return tc_abandoned_baby_bull(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 2 as libc::c_int) as libc::c_ulong {
            return tc_big_black_candle(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 3 as libc::c_int) as libc::c_ulong {
            return tc_big_white_candle(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 4 as libc::c_int) as libc::c_ulong {
            return tc_black_marubozu(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 5 as libc::c_int) as libc::c_ulong {
            return tc_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 6 as libc::c_int) as libc::c_ulong {
            return tc_dragonfly_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 7 as libc::c_int) as libc::c_ulong {
            return tc_engulfing_bear(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 8 as libc::c_int) as libc::c_ulong {
            return tc_engulfing_bull(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 9 as libc::c_int) as libc::c_ulong {
            return tc_evening_doji_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 10 as libc::c_int) as libc::c_ulong {
            return tc_evening_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 11 as libc::c_int) as libc::c_ulong {
            return tc_four_price_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 12 as libc::c_int) as libc::c_ulong {
            return tc_gravestone_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 13 as libc::c_int) as libc::c_ulong {
            return tc_hammer(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 14 as libc::c_int) as libc::c_ulong {
            return tc_hanging_man(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong {
            return tc_inverted_hammer(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 16 as libc::c_int) as libc::c_ulong {
            return tc_long_legged_doji(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 17 as libc::c_int) as libc::c_ulong {
            return tc_marubozu(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 18 as libc::c_int) as libc::c_ulong {
            return tc_morning_doji_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 19 as libc::c_int) as libc::c_ulong {
            return tc_morning_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 20 as libc::c_int) as libc::c_ulong {
            return tc_shooting_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 21 as libc::c_int) as libc::c_ulong {
            return tc_spinning_top(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 22 as libc::c_int) as libc::c_ulong {
            return tc_star(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 23 as libc::c_int) as libc::c_ulong {
            return tc_three_black_crows(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 24 as libc::c_int) as libc::c_ulong {
            return tc_three_white_soldiers(size, inputs, options, output);
        }
        if patterns == ((1 as libc::c_long) << 25 as libc::c_int) as libc::c_ulong {
            return tc_white_marubozu(size, inputs, options, output);
        }
    }
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if ((1 as libc::c_long) << 0 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *low.offset((i - 1 as libc::c_int) as isize)
                >= *high.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && *high.offset(i as isize) <= *low.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 0 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        if ((1 as libc::c_long) << 1 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *high.offset((i - 1 as libc::c_int) as isize)
                <= *low.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && *low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_0: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 1 as libc::c_int) as tc_set,
                };
                init
            };
            let r_0: libc::c_int = tc_result_add(output, hit_0);
            if r_0 != 0 as libc::c_int {
                return r_0;
            }
        }
        if ((1 as libc::c_long) << 2 as libc::c_int) as libc::c_ulong & patterns != 0
            && *open.offset(i as isize) > *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit_1: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 2 as libc::c_int) as tc_set,
                };
                init
            };
            let r_1: libc::c_int = tc_result_add(output, hit_1);
            if r_1 != 0 as libc::c_int {
                return r_1;
            }
        }
        if ((1 as libc::c_long) << 3 as libc::c_int) as libc::c_ulong & patterns != 0
            && *open.offset(i as isize) < *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit_2: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 3 as libc::c_int) as tc_set,
                };
                init
            };
            let r_2: libc::c_int = tc_result_add(output, hit_2);
            if r_2 != 0 as libc::c_int {
                return r_2;
            }
        }
        if ((1 as libc::c_long) << 4 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
        {
            let hit_3: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 4 as libc::c_int) as tc_set,
                };
                init
            };
            let r_3: libc::c_int = tc_result_add(output, hit_3);
            if r_3 != 0 as libc::c_int {
                return r_3;
            }
        }
        if ((1 as libc::c_long) << 5 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
        {
            let hit_4: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 5 as libc::c_int) as tc_set,
                };
                init
            };
            let r_4: libc::c_int = tc_result_add(output, hit_4);
            if r_4 != 0 as libc::c_int {
                return r_4;
            }
        }
        if ((1 as libc::c_long) << 6 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit_5: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 6 as libc::c_int) as tc_set,
                };
                init
            };
            let r_5: libc::c_int = tc_result_add(output, hit_5);
            if r_5 != 0 as libc::c_int {
                return r_5;
            }
        }
        if ((1 as libc::c_long) << 7 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 1 as libc::c_int
            && *open.offset(i as isize) > *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) < *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit_6: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 7 as libc::c_int) as tc_set,
                };
                init
            };
            let r_6: libc::c_int = tc_result_add(output, hit_6);
            if r_6 != 0 as libc::c_int {
                return r_6;
            }
        }
        if ((1 as libc::c_long) << 8 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 1 as libc::c_int
            && *open.offset(i as isize) < *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) > *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit_7: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 8 as libc::c_int) as tc_set,
                };
                init
            };
            let r_7: libc::c_int = tc_result_add(output, hit_7);
            if r_7 != 0 as libc::c_int {
                return r_7;
            }
        }
        if ((1 as libc::c_long) << 9 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_8: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 9 as libc::c_int) as tc_set,
                };
                init
            };
            let r_8: libc::c_int = tc_result_add(output, hit_8);
            if r_8 != 0 as libc::c_int {
                return r_8;
            }
        }
        if ((1 as libc::c_long) << 10 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_9: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 10 as libc::c_int) as tc_set,
                };
                init
            };
            let r_9: libc::c_int = tc_result_add(output, hit_9);
            if r_9 != 0 as libc::c_int {
                return r_9;
            }
        }
        if ((1 as libc::c_long) << 11 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit_10: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 11 as libc::c_int) as tc_set,
                };
                init
            };
            let r_10: libc::c_int = tc_result_add(output, hit_10);
            if r_10 != 0 as libc::c_int {
                return r_10;
            }
        }
        if ((1 as libc::c_long) << 12 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit_11: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 12 as libc::c_int) as tc_set,
                };
                init
            };
            let r_11: libc::c_int = tc_result_add(output, hit_11);
            if r_11 != 0 as libc::c_int {
                return r_11;
            }
        }
        if ((1 as libc::c_long) << 13 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= *low.offset((i - 1 as libc::c_int) as isize)
                    + (*options).near * avg_total
        {
            let hit_12: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 13 as libc::c_int) as tc_set,
                };
                init
            };
            let r_12: libc::c_int = tc_result_add(output, hit_12);
            if r_12 != 0 as libc::c_int {
                return r_12;
            }
        }
        if ((1 as libc::c_long) << 14 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= *high.offset((i - 1 as libc::c_int) as isize)
                    - (*options).near * avg_total
        {
            let hit_13: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 14 as libc::c_int) as tc_set,
                };
                init
            };
            let r_13: libc::c_int = tc_result_add(output, hit_13);
            if r_13 != 0 as libc::c_int {
                return r_13;
            }
        }
        if ((1 as libc::c_long) << 15 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit_14: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 15 as libc::c_int) as tc_set,
                };
                init
            };
            let r_14: libc::c_int = tc_result_add(output, hit_14);
            if r_14 != 0 as libc::c_int {
                return r_14;
            }
        }
        if ((1 as libc::c_long) << 16 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit_15: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 16 as libc::c_int) as tc_set,
                };
                init
            };
            let r_15: libc::c_int = tc_result_add(output, hit_15);
            if r_15 != 0 as libc::c_int {
                return r_15;
            }
        }
        if ((1 as libc::c_long) << 17 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit_16: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 17 as libc::c_int) as tc_set,
                };
                init
            };
            let r_16: libc::c_int = tc_result_add(output, hit_16);
            if r_16 != 0 as libc::c_int {
                return r_16;
            }
        }
        if ((1 as libc::c_long) << 18 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_17: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 18 as libc::c_int) as tc_set,
                };
                init
            };
            let r_17: libc::c_int = tc_result_add(output, hit_17);
            if r_17 != 0 as libc::c_int {
                return r_17;
            }
        }
        if ((1 as libc::c_long) << 19 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit_18: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 19 as libc::c_int) as tc_set,
                };
                init
            };
            let r_18: libc::c_int = tc_result_add(output, hit_18);
            if r_18 != 0 as libc::c_int {
                return r_18;
            }
        }
        if ((1 as libc::c_long) << 20 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit_19: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 20 as libc::c_int) as tc_set,
                };
                init
            };
            let r_19: libc::c_int = tc_result_add(output, hit_19);
            if r_19 != 0 as libc::c_int {
                return r_19;
            }
        }
        if ((1 as libc::c_long) << 21 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
            && bottom - *low.offset(i as isize)
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
        {
            let hit_20: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 21 as libc::c_int) as tc_set,
                };
                init
            };
            let r_20: libc::c_int = tc_result_add(output, hit_20);
            if r_20 != 0 as libc::c_int {
                return r_20;
            }
        }
        if ((1 as libc::c_long) << 22 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 1 as libc::c_int
            && (*low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
                || *high.offset(i as isize)
                    <= *low.offset((i - 1 as libc::c_int) as isize))
        {
            let hit_21: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 22 as libc::c_int) as tc_set,
                };
                init
            };
            let r_21: libc::c_int = tc_result_add(output, hit_21);
            if r_21 != 0 as libc::c_int {
                return r_21;
            }
        }
        if ((1 as libc::c_long) << 23 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit_22: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 23 as libc::c_int) as tc_set,
                };
                init
            };
            let r_22: libc::c_int = tc_result_add(output, hit_22);
            if r_22 != 0 as libc::c_int {
                return r_22;
            }
        }
        if ((1 as libc::c_long) << 24 as libc::c_int) as libc::c_ulong & patterns != 0
            && i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit_23: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 24 as libc::c_int) as tc_set,
                };
                init
            };
            let r_23: libc::c_int = tc_result_add(output, hit_23);
            if r_23 != 0 as libc::c_int {
                return r_23;
            }
        }
        if ((1 as libc::c_long) << 25 as libc::c_int) as libc::c_ulong & patterns != 0
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
        {
            let hit_24: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 25 as libc::c_int) as tc_set,
                };
                init
            };
            let r_24: libc::c_int = tc_result_add(output, hit_24);
            if r_24 != 0 as libc::c_int {
                return r_24;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_abandoned_baby_bear(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *low.offset((i - 1 as libc::c_int) as isize)
                >= *high.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && *high.offset(i as isize) <= *low.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 0 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_abandoned_baby_bull(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && *high.offset((i - 1 as libc::c_int) as isize)
                <= *low.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && *low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 1 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_big_black_candle(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if *open.offset(i as isize) > *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 2 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_big_white_candle(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if *open.offset(i as isize) < *close.offset(i as isize)
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                > (*options).body_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 3 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_black_marubozu(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 4 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 5 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_dragonfly_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 6 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_engulfing_bear(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int && *open.offset(i as isize) > *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) < *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 7 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_engulfing_bull(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int && *open.offset(i as isize) < *close.offset(i as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *close.offset(i as isize) > *open.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset((i - 1 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 8 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_evening_doji_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 9 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_evening_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                <= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 10 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_four_price_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 11 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_gravestone_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 12 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_hammer(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= *low.offset((i - 1 as libc::c_int) as isize)
                    + (*options).near * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 13 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_hanging_man(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= *high.offset((i - 1 as libc::c_int) as isize)
                    - (*options).near * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 14 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_inverted_hammer(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) > *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 15 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_long_legged_doji(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_none * avg_total
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) > (*options).wick_long * avg_body
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 16 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_marubozu(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 17 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_morning_doji_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 18 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_morning_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && fabs(
                *open.offset((i - 2 as libc::c_int) as isize)
                    - *close.offset((i - 2 as libc::c_int) as isize),
            ) > (*options).body_long * avg_body
            && (if *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            {
                *open.offset((i - 1 as libc::c_int) as isize)
            } else {
                *close.offset((i - 1 as libc::c_int) as isize)
            })
                <= (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    < *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
            && fabs(
                *open.offset((i - 1 as libc::c_int) as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize),
            ) < (*options).body_short * avg_body
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
            && *close.offset(i as isize)
                >= *close.offset((i - 2 as libc::c_int) as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 19 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_shooting_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && fabs(*open.offset(i as isize) - *close.offset(i as isize))
                < (*options).body_short * avg_body
            && *high.offset(i as isize) - top > (*options).wick_long * avg_body
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && (if *open.offset(i as isize) < *close.offset(i as isize) {
                *open.offset(i as isize)
            } else {
                *close.offset(i as isize)
            })
                >= (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 20 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_spinning_top(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            < (*options).body_short * avg_body
            && *high.offset(i as isize) - top
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
            && bottom - *low.offset(i as isize)
                > fabs(*open.offset(i as isize) - *close.offset(i as isize))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 21 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_star(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 1 as libc::c_int
            && (*low.offset(i as isize) >= *high.offset((i - 1 as libc::c_int) as isize)
                || *high.offset(i as isize)
                    <= *low.offset((i - 1 as libc::c_int) as isize))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 22 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_three_black_crows(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                > *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                > *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) > *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 23 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_three_white_soldiers(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if i >= 2 as libc::c_int
            && *open.offset((i - 2 as libc::c_int) as isize)
                < *close.offset((i - 2 as libc::c_int) as isize)
            && *open.offset((i - 1 as libc::c_int) as isize)
                < *close.offset((i - 1 as libc::c_int) as isize)
            && *open.offset(i as isize) < *close.offset(i as isize)
            && (*open.offset((i - 1 as libc::c_int) as isize)
                < (if *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                })
                && *open.offset((i - 1 as libc::c_int) as isize)
                    > (if *open
                        .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                        < *close
                            .offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int - 1 as libc::c_int) as isize)
                    }))
            && (*open.offset(i as isize)
                < (if *open.offset((i - 1 as libc::c_int) as isize)
                    > *close.offset((i - 1 as libc::c_int) as isize)
                {
                    *open.offset((i - 1 as libc::c_int) as isize)
                } else {
                    *close.offset((i - 1 as libc::c_int) as isize)
                })
                && *open.offset(i as isize)
                    > (if *open.offset((i - 1 as libc::c_int) as isize)
                        < *close.offset((i - 1 as libc::c_int) as isize)
                    {
                        *open.offset((i - 1 as libc::c_int) as isize)
                    } else {
                        *close.offset((i - 1 as libc::c_int) as isize)
                    }))
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 24 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tc_white_marubozu(
    mut size: libc::c_int,
    mut inputs: *const *const TC_REAL,
    mut options: *const tc_config,
    mut output: *mut tc_result,
) -> libc::c_int {
    let mut open: *const TC_REAL = *inputs.offset(0 as libc::c_int as isize);
    let mut high: *const TC_REAL = *inputs.offset(1 as libc::c_int as isize);
    let mut low: *const TC_REAL = *inputs.offset(2 as libc::c_int as isize);
    let mut close: *const TC_REAL = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = (*options).period;
    let div: TC_REAL = 1.0f64 / period as libc::c_double;
    let mut avg_body_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    let mut avg_total_sum: TC_REAL = 0 as libc::c_int as TC_REAL;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size < period {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        avg_body_sum += fabs(*open.offset(i as isize) - *close.offset(i as isize));
        avg_total_sum += *high.offset(i as isize) - *low.offset(i as isize);
        i += 1;
        i;
    }
    while i < size {
        let top: TC_REAL = if *open.offset(i as isize) > *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let bottom: TC_REAL = if *open.offset(i as isize) < *close.offset(i as isize) {
            *open.offset(i as isize)
        } else {
            *close.offset(i as isize)
        };
        let body: TC_REAL = fabs(*open.offset(i as isize) - *close.offset(i as isize));
        let total: TC_REAL = *high.offset(i as isize) - *low.offset(i as isize);
        let upper: TC_REAL = *high.offset(i as isize) - top;
        let lower: TC_REAL = bottom - *low.offset(i as isize);
        let avg_body: TC_REAL = avg_body_sum * div;
        let avg_total: TC_REAL = avg_total_sum * div;
        if fabs(*open.offset(i as isize) - *close.offset(i as isize))
            > (*options).body_long * avg_body
            && *high.offset(i as isize) - top < (*options).wick_none * avg_total
            && bottom - *low.offset(i as isize) < (*options).wick_none * avg_total
            && *open.offset(i as isize) < *close.offset(i as isize)
        {
            let hit: tc_hit = {
                let mut init = tc_hit {
                    index: i,
                    patterns: ((1 as libc::c_long) << 25 as libc::c_int) as tc_set,
                };
                init
            };
            let r: libc::c_int = tc_result_add(output, hit);
            if r != 0 as libc::c_int {
                return r;
            }
        }
        avg_body_sum += body;
        avg_body_sum
            -= fabs(
                *open.offset((i - period) as isize)
                    - *close.offset((i - period) as isize),
            );
        avg_total_sum += total;
        avg_total_sum
            -= *high.offset((i - period) as isize) - *low.offset((i - period) as isize);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
