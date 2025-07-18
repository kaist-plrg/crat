use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn ZopfliLZ77Greedy(
        s: *mut ZopfliBlockState,
        in_0: *const libc::c_uchar,
        instart: size_t,
        inend: size_t,
        store: *mut ZopfliLZ77Store,
        h: *mut ZopfliHash,
    );
    fn ZopfliCleanBlockState(s: *mut ZopfliBlockState);
    fn ZopfliInitBlockState(
        options: *const ZopfliOptions,
        blockstart: size_t,
        blockend: size_t,
        add_lmc: libc::c_int,
        s: *mut ZopfliBlockState,
    );
    fn ZopfliCleanLZ77Store(store: *mut ZopfliLZ77Store);
    fn ZopfliInitLZ77Store(data: *const libc::c_uchar, store: *mut ZopfliLZ77Store);
    fn ZopfliCleanHash(h: *mut ZopfliHash);
    fn ZopfliAllocHash(window_size: size_t, h: *mut ZopfliHash);
    fn exit(_: libc::c_int) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ZopfliCalculateBlockSizeAutoType(
        lz77: *const ZopfliLZ77Store,
        lstart: size_t,
        lend: size_t,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLongestMatchCache {
    pub length: *mut libc::c_ushort,
    pub dist: *mut libc::c_ushort,
    pub sublen: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliHash {
    pub head: *mut libc::c_int,
    pub prev: *mut libc::c_ushort,
    pub hashval: *mut libc::c_int,
    pub val: libc::c_int,
    pub head2: *mut libc::c_int,
    pub prev2: *mut libc::c_ushort,
    pub hashval2: *mut libc::c_int,
    pub val2: libc::c_int,
    pub same: *mut libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliOptions {
    pub verbose: libc::c_int,
    pub verbose_more: libc::c_int,
    pub numiterations: libc::c_int,
    pub blocksplitting: libc::c_int,
    pub blocksplittinglast: libc::c_int,
    pub blocksplittingmax: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliLZ77Store {
    pub litlens: *mut libc::c_ushort,
    pub dists: *mut libc::c_ushort,
    pub size: size_t,
    pub data: *const libc::c_uchar,
    pub pos: *mut size_t,
    pub ll_symbol: *mut libc::c_ushort,
    pub d_symbol: *mut libc::c_ushort,
    pub ll_counts: *mut size_t,
    pub d_counts: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliBlockState {
    pub options: *const ZopfliOptions,
    pub lmc: *mut ZopfliLongestMatchCache,
    pub blockstart: size_t,
    pub blockend: size_t,
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
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SplitCostContext {
    pub lz77: *const ZopfliLZ77Store,
    pub start: size_t,
    pub end: size_t,
}
pub type FindMinimumFun = unsafe extern "C" fn(
    size_t,
    *mut libc::c_void,
) -> libc::c_double;
unsafe extern "C" fn FindMinimum(
    mut f: Option::<FindMinimumFun>,
    mut context: *mut libc::c_void,
    mut start: size_t,
    mut end: size_t,
    mut smallest: *mut libc::c_double,
) -> size_t {
    if end.wrapping_sub(start) < 1024 as libc::c_int as libc::c_ulong {
        let mut best: libc::c_double = 1e30f64;
        let mut result: size_t = start;
        let mut i: size_t = 0;
        i = start;
        while i < end {
            let mut v: libc::c_double = f.unwrap()(i, context);
            if v < best {
                best = v;
                result = i;
            }
            i = i.wrapping_add(1);
            i;
        }
        *smallest = best;
        return result;
    } else {
        let mut i_0: size_t = 0;
        let mut p: [size_t; 9] = [0; 9];
        let mut vp: [libc::c_double; 9] = [0.; 9];
        let mut besti: size_t = 0;
        let mut best_0: libc::c_double = 0.;
        let mut lastbest: libc::c_double = 1e30f64;
        let mut pos: size_t = start;
        while !(end.wrapping_sub(start) <= 9 as libc::c_int as libc::c_ulong) {
            i_0 = 0 as libc::c_int as size_t;
            while i_0 < 9 as libc::c_int as libc::c_ulong {
                p[i_0
                    as usize] = start
                    .wrapping_add(
                        i_0
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                end
                                    .wrapping_sub(start)
                                    .wrapping_div(
                                        (9 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                                    ),
                            ),
                    );
                vp[i_0 as usize] = f.unwrap()(p[i_0 as usize], context);
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            besti = 0 as libc::c_int as size_t;
            best_0 = vp[0 as libc::c_int as usize];
            i_0 = 1 as libc::c_int as size_t;
            while i_0 < 9 as libc::c_int as libc::c_ulong {
                if vp[i_0 as usize] < best_0 {
                    best_0 = vp[i_0 as usize];
                    besti = i_0;
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            if best_0 > lastbest {
                break;
            }
            start = if besti == 0 as libc::c_int as libc::c_ulong {
                start
            } else {
                p[besti.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            };
            end = if besti == (9 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
                end
            } else {
                p[besti.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize]
            };
            pos = p[besti as usize];
            lastbest = best_0;
        }
        *smallest = lastbest;
        return pos;
    };
}
unsafe extern "C" fn EstimateCost(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> libc::c_double {
    return ZopfliCalculateBlockSizeAutoType(lz77, lstart, lend);
}
unsafe extern "C" fn SplitCost(
    mut i: size_t,
    mut context: *mut libc::c_void,
) -> libc::c_double {
    let mut c: *mut SplitCostContext = context as *mut SplitCostContext;
    return EstimateCost((*c).lz77, (*c).start, i) + EstimateCost((*c).lz77, i, (*c).end);
}
unsafe extern "C" fn AddSorted(
    mut value: size_t,
    mut out: *mut *mut size_t,
    mut outsize: *mut size_t,
) {
    let mut i: size_t = 0;
    if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
        *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
            malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
        } else {
            realloc(
                *out as *mut libc::c_void,
                (*outsize)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
            )
        }) as *mut size_t;
    }
    *(*out).offset(*outsize as isize) = value;
    *outsize = (*outsize).wrapping_add(1);
    *outsize;
    i = 0 as libc::c_int as size_t;
    while i.wrapping_add(1 as libc::c_int as libc::c_ulong) < *outsize {
        if *(*out).offset(i as isize) > value {
            let mut j: size_t = 0;
            j = (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong);
            while j > i {
                *(*out)
                    .offset(
                        j as isize,
                    ) = *(*out)
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
                j = j.wrapping_sub(1);
                j;
            }
            *(*out).offset(i as isize) = value;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
}
unsafe extern "C" fn PrintBlockSplitPoints(
    mut lz77: *const ZopfliLZ77Store,
    mut lz77splitpoints: *const size_t,
    mut nlz77points: size_t,
) {
    let mut splitpoints: *mut size_t = 0 as *mut size_t;
    let mut npoints: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    if nlz77points > 0 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < (*lz77).size {
            let mut length: size_t = (if *((*lz77).dists).offset(i as isize)
                as libc::c_int == 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                *((*lz77).litlens).offset(i as isize) as libc::c_int
            }) as size_t;
            if *lz77splitpoints.offset(npoints as isize) == i {
                if npoints & npoints.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
                {
                    splitpoints = (if npoints == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    } else {
                        realloc(
                            splitpoints as *mut libc::c_void,
                            npoints
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<size_t>() as libc::c_ulong,
                                ),
                        )
                    }) as *mut size_t;
                }
                *splitpoints.offset(npoints as isize) = pos;
                npoints = npoints.wrapping_add(1);
                npoints;
                if npoints == nlz77points {
                    break;
                }
            }
            pos = (pos as libc::c_ulong).wrapping_add(length) as size_t as size_t;
            i = i.wrapping_add(1);
            i;
        }
    }
    if npoints == nlz77points {} else {
        __assert_fail(
            b"npoints == nlz77points\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void PrintBlockSplitPoints(const ZopfliLZ77Store *, const size_t *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1879: {
        if npoints == nlz77points {} else {
            __assert_fail(
                b"npoints == nlz77points\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                    as *const libc::c_char,
                167 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void PrintBlockSplitPoints(const ZopfliLZ77Store *, const size_t *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    fprintf(stderr, b"block split points: \0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < npoints {
        fprintf(
            stderr,
            b"%d \0" as *const u8 as *const libc::c_char,
            *splitpoints.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    fprintf(stderr, b"(hex:\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < npoints {
        fprintf(
            stderr,
            b" %x\0" as *const u8 as *const libc::c_char,
            *splitpoints.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    fprintf(stderr, b")\n\0" as *const u8 as *const libc::c_char);
    free(splitpoints as *mut libc::c_void);
}
unsafe extern "C" fn FindLargestSplittableBlock(
    mut lz77size: size_t,
    mut done: *const libc::c_uchar,
    mut splitpoints: *const size_t,
    mut npoints: size_t,
    mut lstart: *mut size_t,
    mut lend: *mut size_t,
) -> libc::c_int {
    let mut longest: size_t = 0 as libc::c_int as size_t;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= npoints {
        let mut start: size_t = if i == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int as libc::c_ulong
        } else {
            *splitpoints
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut end: size_t = if i == npoints {
            lz77size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            *splitpoints.offset(i as isize)
        };
        if *done.offset(start as isize) == 0 && end.wrapping_sub(start) > longest {
            *lstart = start;
            *lend = end;
            found = 1 as libc::c_int;
            longest = end.wrapping_sub(start);
        }
        i = i.wrapping_add(1);
        i;
    }
    return found;
}
pub unsafe extern "C" fn ZopfliBlockSplitLZ77(
    mut options: *const ZopfliOptions,
    mut lz77: *const ZopfliLZ77Store,
    mut maxblocks: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut lstart: size_t = 0;
    let mut lend: size_t = 0;
    let mut i: size_t = 0;
    let mut llpos: size_t = 0 as libc::c_int as size_t;
    let mut numblocks: size_t = 1 as libc::c_int as size_t;
    let mut done: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut splitcost: libc::c_double = 0.;
    let mut origcost: libc::c_double = 0.;
    if (*lz77).size < 10 as libc::c_int as libc::c_ulong {
        return;
    }
    done = malloc((*lz77).size) as *mut libc::c_uchar;
    if done.is_null() {
        exit(-(1 as libc::c_int));
    }
    i = 0 as libc::c_int as size_t;
    while i < (*lz77).size {
        *done.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    lstart = 0 as libc::c_int as size_t;
    lend = (*lz77).size;
    loop {
        let mut c: SplitCostContext = SplitCostContext {
            lz77: 0 as *const ZopfliLZ77Store,
            start: 0,
            end: 0,
        };
        if maxblocks > 0 as libc::c_int as libc::c_ulong && numblocks >= maxblocks {
            break;
        }
        c.lz77 = lz77;
        c.start = lstart;
        c.end = lend;
        if lstart < lend {} else {
            __assert_fail(
                b"lstart < lend\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                    as *const libc::c_char,
                243 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2996: {
            if lstart < lend {} else {
                __assert_fail(
                    b"lstart < lend\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                        as *const libc::c_char,
                    243 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 103],
                        &[libc::c_char; 103],
                    >(
                        b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        llpos = FindMinimum(
            Some(
                SplitCost
                    as unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_double,
            ),
            &mut c as *mut SplitCostContext as *mut libc::c_void,
            lstart.wrapping_add(1 as libc::c_int as libc::c_ulong),
            lend,
            &mut splitcost,
        );
        if llpos > lstart {} else {
            __assert_fail(
                b"llpos > lstart\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                    as *const libc::c_char,
                246 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2586: {
            if llpos > lstart {} else {
                __assert_fail(
                    b"llpos > lstart\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                        as *const libc::c_char,
                    246 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 103],
                        &[libc::c_char; 103],
                    >(
                        b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if llpos < lend {} else {
            __assert_fail(
                b"llpos < lend\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                    as *const libc::c_char,
                247 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 103],
                    &[libc::c_char; 103],
                >(
                    b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2546: {
            if llpos < lend {} else {
                __assert_fail(
                    b"llpos < lend\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                        as *const libc::c_char,
                    247 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 103],
                        &[libc::c_char; 103],
                    >(
                        b"void ZopfliBlockSplitLZ77(const ZopfliOptions *, const ZopfliLZ77Store *, size_t, size_t **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        origcost = EstimateCost(lz77, lstart, lend);
        if splitcost > origcost
            || llpos == lstart.wrapping_add(1 as libc::c_int as libc::c_ulong)
            || llpos == lend
        {
            *done.offset(lstart as isize) = 1 as libc::c_int as libc::c_uchar;
        } else {
            AddSorted(llpos, splitpoints, npoints);
            numblocks = numblocks.wrapping_add(1);
            numblocks;
        }
        if FindLargestSplittableBlock(
            (*lz77).size,
            done,
            *splitpoints,
            *npoints,
            &mut lstart,
            &mut lend,
        ) == 0
        {
            break;
        }
        if lend.wrapping_sub(lstart) < 10 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    if (*options).verbose != 0 {
        PrintBlockSplitPoints(lz77, *splitpoints, *npoints);
    }
    free(done as *mut libc::c_void);
}
pub unsafe extern "C" fn ZopfliBlockSplit(
    mut options: *const ZopfliOptions,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut maxblocks: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut s: ZopfliBlockState = ZopfliBlockState {
        options: 0 as *const ZopfliOptions,
        lmc: 0 as *mut ZopfliLongestMatchCache,
        blockstart: 0,
        blockend: 0,
    };
    let mut lz77splitpoints: *mut size_t = 0 as *mut size_t;
    let mut nlz77points: size_t = 0 as libc::c_int as size_t;
    let mut store: ZopfliLZ77Store = ZopfliLZ77Store {
        litlens: 0 as *mut libc::c_ushort,
        dists: 0 as *mut libc::c_ushort,
        size: 0,
        data: 0 as *const libc::c_uchar,
        pos: 0 as *mut size_t,
        ll_symbol: 0 as *mut libc::c_ushort,
        d_symbol: 0 as *mut libc::c_ushort,
        ll_counts: 0 as *mut size_t,
        d_counts: 0 as *mut size_t,
    };
    let mut hash: ZopfliHash = ZopfliHash {
        head: 0 as *mut libc::c_int,
        prev: 0 as *mut libc::c_ushort,
        hashval: 0 as *mut libc::c_int,
        val: 0,
        head2: 0 as *mut libc::c_int,
        prev2: 0 as *mut libc::c_ushort,
        hashval2: 0 as *mut libc::c_int,
        val2: 0,
        same: 0 as *mut libc::c_ushort,
    };
    let mut h: *mut ZopfliHash = &mut hash;
    ZopfliInitLZ77Store(in_0, &mut store);
    ZopfliInitBlockState(options, instart, inend, 0 as libc::c_int, &mut s);
    ZopfliAllocHash(32768 as libc::c_int as size_t, h);
    *npoints = 0 as libc::c_int as size_t;
    *splitpoints = 0 as *mut size_t;
    ZopfliLZ77Greedy(&mut s, in_0, instart, inend, &mut store, h);
    ZopfliBlockSplitLZ77(
        options,
        &mut store,
        maxblocks,
        &mut lz77splitpoints,
        &mut nlz77points,
    );
    pos = instart;
    if nlz77points > 0 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as size_t;
        while i < store.size {
            let mut length: size_t = (if *(store.dists).offset(i as isize) as libc::c_int
                == 0 as libc::c_int
            {
                1 as libc::c_int
            } else {
                *(store.litlens).offset(i as isize) as libc::c_int
            }) as size_t;
            if *lz77splitpoints.offset(*npoints as isize) == i {
                if *npoints & (*npoints).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    == 0
                {
                    *splitpoints = (if *npoints == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    } else {
                        realloc(
                            *splitpoints as *mut libc::c_void,
                            (*npoints)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<size_t>() as libc::c_ulong,
                                ),
                        )
                    }) as *mut size_t;
                }
                *(*splitpoints).offset(*npoints as isize) = pos;
                *npoints = (*npoints).wrapping_add(1);
                *npoints;
                if *npoints == nlz77points {
                    break;
                }
            }
            pos = (pos as libc::c_ulong).wrapping_add(length) as size_t as size_t;
            i = i.wrapping_add(1);
            i;
        }
    }
    if *npoints == nlz77points {} else {
        __assert_fail(
            b"*npoints == nlz77points\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8 as *const libc::c_char,
            314 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 113],
                &[libc::c_char; 113],
            >(
                b"void ZopfliBlockSplit(const ZopfliOptions *, const unsigned char *, size_t, size_t, size_t, size_t **, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3190: {
        if *npoints == nlz77points {} else {
            __assert_fail(
                b"*npoints == nlz77points\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/blocksplitter.c\0" as *const u8
                    as *const libc::c_char,
                314 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 113],
                    &[libc::c_char; 113],
                >(
                    b"void ZopfliBlockSplit(const ZopfliOptions *, const unsigned char *, size_t, size_t, size_t, size_t **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    free(lz77splitpoints as *mut libc::c_void);
    ZopfliCleanBlockState(&mut s);
    ZopfliCleanLZ77Store(&mut store);
    ZopfliCleanHash(h);
}
pub unsafe extern "C" fn ZopfliBlockSplitSimple(
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut blocksize: size_t,
    mut splitpoints: *mut *mut size_t,
    mut npoints: *mut size_t,
) {
    let mut i: size_t = instart;
    while i < inend {
        if *npoints & (*npoints).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
            *splitpoints = (if *npoints == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
            } else {
                realloc(
                    *splitpoints as *mut libc::c_void,
                    (*npoints)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
                )
            }) as *mut size_t;
        }
        *(*splitpoints).offset(*npoints as isize) = i;
        *npoints = (*npoints).wrapping_add(1);
        *npoints;
        i = (i as libc::c_ulong).wrapping_add(blocksize) as size_t as size_t;
    }
}
