use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn ZopfliInitCache(blocksize: size_t, lmc: *mut ZopfliLongestMatchCache);
    fn ZopfliCleanCache(lmc: *mut ZopfliLongestMatchCache);
    fn ZopfliSublenToCache(
        sublen: *const libc::c_ushort,
        pos: size_t,
        length: size_t,
        lmc: *mut ZopfliLongestMatchCache,
    );
    fn ZopfliCacheToSublen(
        lmc: *const ZopfliLongestMatchCache,
        pos: size_t,
        length: size_t,
        sublen: *mut libc::c_ushort,
    );
    fn ZopfliMaxCachedSublen(
        lmc: *const ZopfliLongestMatchCache,
        pos: size_t,
        length: size_t,
    ) -> libc::c_uint;
    fn ZopfliResetHash(window_size: size_t, h: *mut ZopfliHash);
    fn ZopfliUpdateHash(
        array: *const libc::c_uchar,
        pos: size_t,
        end: size_t,
        h: *mut ZopfliHash,
    );
    fn ZopfliWarmupHash(
        array: *const libc::c_uchar,
        pos: size_t,
        end: size_t,
        h: *mut ZopfliHash,
    );
    fn ZopfliGetDistSymbol(dist: libc::c_int) -> libc::c_int;
    fn ZopfliGetLengthSymbol(l: libc::c_int) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn ZopfliInitLZ77Store(
    mut data: *const libc::c_uchar,
    mut store: *mut ZopfliLZ77Store,
) {
    (*store).size = 0 as libc::c_int as size_t;
    (*store).litlens = 0 as *mut libc::c_ushort;
    (*store).dists = 0 as *mut libc::c_ushort;
    (*store).pos = 0 as *mut size_t;
    (*store).data = data;
    (*store).ll_symbol = 0 as *mut libc::c_ushort;
    (*store).d_symbol = 0 as *mut libc::c_ushort;
    (*store).ll_counts = 0 as *mut size_t;
    (*store).d_counts = 0 as *mut size_t;
}
pub unsafe extern "C" fn ZopfliCleanLZ77Store(mut store: *mut ZopfliLZ77Store) {
    free((*store).litlens as *mut libc::c_void);
    free((*store).dists as *mut libc::c_void);
    free((*store).pos as *mut libc::c_void);
    free((*store).ll_symbol as *mut libc::c_void);
    free((*store).d_symbol as *mut libc::c_void);
    free((*store).ll_counts as *mut libc::c_void);
    free((*store).d_counts as *mut libc::c_void);
}
unsafe extern "C" fn CeilDiv(mut a: size_t, mut b: size_t) -> size_t {
    return a
        .wrapping_add(b)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(b);
}
pub unsafe extern "C" fn ZopfliCopyLZ77Store(
    mut source: *const ZopfliLZ77Store,
    mut dest: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    let mut llsize: size_t = (288 as libc::c_int as libc::c_ulong)
        .wrapping_mul(CeilDiv((*source).size, 288 as libc::c_int as size_t));
    let mut dsize: size_t = (32 as libc::c_int as libc::c_ulong)
        .wrapping_mul(CeilDiv((*source).size, 32 as libc::c_int as size_t));
    ZopfliCleanLZ77Store(dest);
    ZopfliInitLZ77Store((*source).data, dest);
    (*dest)
        .litlens = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest)
        .dists = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest)
        .pos = malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul((*source).size),
    ) as *mut size_t;
    (*dest)
        .ll_symbol = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest)
        .d_symbol = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul((*source).size),
    ) as *mut libc::c_ushort;
    (*dest)
        .ll_counts = malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(llsize),
    ) as *mut size_t;
    (*dest)
        .d_counts = malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(dsize),
    ) as *mut size_t;
    if ((*dest).litlens).is_null() || ((*dest).dists).is_null() {
        exit(-(1 as libc::c_int));
    }
    if ((*dest).pos).is_null() {
        exit(-(1 as libc::c_int));
    }
    if ((*dest).ll_symbol).is_null() || ((*dest).d_symbol).is_null() {
        exit(-(1 as libc::c_int));
    }
    if ((*dest).ll_counts).is_null() || ((*dest).d_counts).is_null() {
        exit(-(1 as libc::c_int));
    }
    (*dest).size = (*source).size;
    i = 0 as libc::c_int as size_t;
    while i < (*source).size {
        *((*dest).litlens).offset(i as isize) = *((*source).litlens).offset(i as isize);
        *((*dest).dists).offset(i as isize) = *((*source).dists).offset(i as isize);
        *((*dest).pos).offset(i as isize) = *((*source).pos).offset(i as isize);
        *((*dest).ll_symbol)
            .offset(i as isize) = *((*source).ll_symbol).offset(i as isize);
        *((*dest).d_symbol)
            .offset(i as isize) = *((*source).d_symbol).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < llsize {
        *((*dest).ll_counts)
            .offset(i as isize) = *((*source).ll_counts).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < dsize {
        *((*dest).d_counts)
            .offset(i as isize) = *((*source).d_counts).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn ZopfliStoreLitLenDist(
    mut length: libc::c_ushort,
    mut dist: libc::c_ushort,
    mut pos: size_t,
    mut store: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    let mut origsize: size_t = (*store).size;
    let mut llstart: size_t = (288 as libc::c_int as libc::c_ulong)
        .wrapping_mul(origsize.wrapping_div(288 as libc::c_int as libc::c_ulong));
    let mut dstart: size_t = (32 as libc::c_int as libc::c_ulong)
        .wrapping_mul(origsize.wrapping_div(32 as libc::c_int as libc::c_ulong));
    if origsize.wrapping_rem(288 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        let mut llsize: size_t = origsize;
        i = 0 as libc::c_int as size_t;
        while i < 288 as libc::c_int as libc::c_ulong {
            if llsize & llsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
                (*store)
                    .ll_counts = (if llsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
                } else {
                    realloc(
                        (*store).ll_counts as *mut libc::c_void,
                        llsize
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<size_t>() as libc::c_ulong,
                            ),
                    )
                }) as *mut size_t;
            }
            *((*store).ll_counts)
                .offset(
                    llsize as isize,
                ) = if origsize == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int as libc::c_ulong
            } else {
                *((*store).ll_counts)
                    .offset(
                        origsize
                            .wrapping_sub(288 as libc::c_int as libc::c_ulong)
                            .wrapping_add(i) as isize,
                    )
            };
            llsize = llsize.wrapping_add(1);
            llsize;
            i = i.wrapping_add(1);
            i;
        }
    }
    if origsize.wrapping_rem(32 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        let mut dsize: size_t = origsize;
        i = 0 as libc::c_int as size_t;
        while i < 32 as libc::c_int as libc::c_ulong {
            if dsize & dsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
                (*store)
                    .d_counts = (if dsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
                } else {
                    realloc(
                        (*store).d_counts as *mut libc::c_void,
                        dsize
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<size_t>() as libc::c_ulong,
                            ),
                    )
                }) as *mut size_t;
            }
            *((*store).d_counts)
                .offset(
                    dsize as isize,
                ) = if origsize == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int as libc::c_ulong
            } else {
                *((*store).d_counts)
                    .offset(
                        origsize
                            .wrapping_sub(32 as libc::c_int as libc::c_ulong)
                            .wrapping_add(i) as isize,
                    )
            };
            dsize = dsize.wrapping_add(1);
            dsize;
            i = i.wrapping_add(1);
            i;
        }
    }
    if (*store).size & ((*store).size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0
    {
        (*store)
            .litlens = (if (*store).size == 0 as libc::c_int as libc::c_ulong {
            malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
        } else {
            realloc(
                (*store).litlens as *mut libc::c_void,
                ((*store).size)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                    ),
            )
        }) as *mut libc::c_ushort;
    }
    *((*store).litlens).offset((*store).size as isize) = length;
    (*store).size = ((*store).size).wrapping_add(1);
    (*store).size;
    (*store).size = origsize;
    if (*store).size & ((*store).size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0
    {
        (*store)
            .dists = (if (*store).size == 0 as libc::c_int as libc::c_ulong {
            malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
        } else {
            realloc(
                (*store).dists as *mut libc::c_void,
                ((*store).size)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                    ),
            )
        }) as *mut libc::c_ushort;
    }
    *((*store).dists).offset((*store).size as isize) = dist;
    (*store).size = ((*store).size).wrapping_add(1);
    (*store).size;
    (*store).size = origsize;
    if (*store).size & ((*store).size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        == 0
    {
        (*store)
            .pos = (if (*store).size == 0 as libc::c_int as libc::c_ulong {
            malloc(::std::mem::size_of::<size_t>() as libc::c_ulong)
        } else {
            realloc(
                (*store).pos as *mut libc::c_void,
                ((*store).size)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
            )
        }) as *mut size_t;
    }
    *((*store).pos).offset((*store).size as isize) = pos;
    (*store).size = ((*store).size).wrapping_add(1);
    (*store).size;
    if (length as libc::c_int) < 259 as libc::c_int {} else {
        __assert_fail(
            b"length < 259\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"void ZopfliStoreLitLenDist(unsigned short, unsigned short, size_t, ZopfliLZ77Store *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2781: {
        if (length as libc::c_int) < 259 as libc::c_int {} else {
            __assert_fail(
                b"length < 259\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                131 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void ZopfliStoreLitLenDist(unsigned short, unsigned short, size_t, ZopfliLZ77Store *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dist as libc::c_int == 0 as libc::c_int {
        (*store).size = origsize;
        if (*store).size
            & ((*store).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
        {
            (*store)
                .ll_symbol = (if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).ll_symbol as *mut libc::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_ushort;
        }
        *((*store).ll_symbol).offset((*store).size as isize) = length;
        (*store).size = ((*store).size).wrapping_add(1);
        (*store).size;
        (*store).size = origsize;
        if (*store).size
            & ((*store).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
        {
            (*store)
                .d_symbol = (if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).d_symbol as *mut libc::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_ushort;
        }
        *((*store).d_symbol)
            .offset((*store).size as isize) = 0 as libc::c_int as libc::c_ushort;
        (*store).size = ((*store).size).wrapping_add(1);
        (*store).size;
        let ref mut fresh0 = *((*store).ll_counts)
            .offset(llstart.wrapping_add(length as libc::c_ulong) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
    } else {
        (*store).size = origsize;
        if (*store).size
            & ((*store).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
        {
            (*store)
                .ll_symbol = (if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).ll_symbol as *mut libc::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_ushort;
        }
        *((*store).ll_symbol)
            .offset(
                (*store).size as isize,
            ) = ZopfliGetLengthSymbol(length as libc::c_int) as libc::c_ushort;
        (*store).size = ((*store).size).wrapping_add(1);
        (*store).size;
        (*store).size = origsize;
        if (*store).size
            & ((*store).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
        {
            (*store)
                .d_symbol = (if (*store).size == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    (*store).d_symbol as *mut libc::c_void,
                    ((*store).size)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_ushort;
        }
        *((*store).d_symbol)
            .offset(
                (*store).size as isize,
            ) = ZopfliGetDistSymbol(dist as libc::c_int) as libc::c_ushort;
        (*store).size = ((*store).size).wrapping_add(1);
        (*store).size;
        let ref mut fresh1 = *((*store).ll_counts)
            .offset(
                llstart
                    .wrapping_add(
                        ZopfliGetLengthSymbol(length as libc::c_int) as libc::c_ulong,
                    ) as isize,
            );
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
        let ref mut fresh2 = *((*store).d_counts)
            .offset(
                dstart
                    .wrapping_add(
                        ZopfliGetDistSymbol(dist as libc::c_int) as libc::c_ulong,
                    ) as isize,
            );
        *fresh2 = (*fresh2).wrapping_add(1);
        *fresh2;
    };
}
pub unsafe extern "C" fn ZopfliAppendLZ77Store(
    mut store: *const ZopfliLZ77Store,
    mut target: *mut ZopfliLZ77Store,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*store).size {
        ZopfliStoreLitLenDist(
            *((*store).litlens).offset(i as isize),
            *((*store).dists).offset(i as isize),
            *((*store).pos).offset(i as isize),
            target,
        );
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn ZopfliLZ77GetByteRange(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let mut l: size_t = lend.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if lstart == lend {
        return 0 as libc::c_int as size_t;
    }
    return (*((*lz77).pos).offset(l as isize))
        .wrapping_add(
            (if *((*lz77).dists).offset(l as isize) as libc::c_int == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                *((*lz77).litlens).offset(l as isize) as libc::c_int
            }) as libc::c_ulong,
        )
        .wrapping_sub(*((*lz77).pos).offset(lstart as isize));
}
unsafe extern "C" fn ZopfliLZ77GetHistogramAt(
    mut lz77: *const ZopfliLZ77Store,
    mut lpos: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    let mut llpos: size_t = (288 as libc::c_int as libc::c_ulong)
        .wrapping_mul(lpos.wrapping_div(288 as libc::c_int as libc::c_ulong));
    let mut dpos: size_t = (32 as libc::c_int as libc::c_ulong)
        .wrapping_mul(lpos.wrapping_div(32 as libc::c_int as libc::c_ulong));
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        *ll_counts
            .offset(
                i as isize,
            ) = *((*lz77).ll_counts).offset(llpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = lpos.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while i < llpos.wrapping_add(288 as libc::c_int as libc::c_ulong) && i < (*lz77).size
    {
        let ref mut fresh3 = *ll_counts
            .offset(*((*lz77).ll_symbol).offset(i as isize) as isize);
        *fresh3 = (*fresh3).wrapping_sub(1);
        *fresh3;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        *d_counts
            .offset(
                i as isize,
            ) = *((*lz77).d_counts).offset(dpos.wrapping_add(i) as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = lpos.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while i < dpos.wrapping_add(32 as libc::c_int as libc::c_ulong) && i < (*lz77).size {
        if *((*lz77).dists).offset(i as isize) as libc::c_int != 0 as libc::c_int {
            let ref mut fresh4 = *d_counts
                .offset(*((*lz77).d_symbol).offset(i as isize) as isize);
            *fresh4 = (*fresh4).wrapping_sub(1);
            *fresh4;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn ZopfliLZ77GetHistogram(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_counts: *mut size_t,
    mut d_counts: *mut size_t,
) {
    let mut i: size_t = 0;
    if lstart.wrapping_add((288 as libc::c_int * 3 as libc::c_int) as libc::c_ulong)
        > lend
    {
        memset(
            ll_counts as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(288 as libc::c_int as libc::c_ulong),
        );
        memset(
            d_counts as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(32 as libc::c_int as libc::c_ulong),
        );
        i = lstart;
        while i < lend {
            let ref mut fresh5 = *ll_counts
                .offset(*((*lz77).ll_symbol).offset(i as isize) as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
            *fresh5;
            if *((*lz77).dists).offset(i as isize) as libc::c_int != 0 as libc::c_int {
                let ref mut fresh6 = *d_counts
                    .offset(*((*lz77).d_symbol).offset(i as isize) as isize);
                *fresh6 = (*fresh6).wrapping_add(1);
                *fresh6;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        ZopfliLZ77GetHistogramAt(
            lz77,
            lend.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ll_counts,
            d_counts,
        );
        if lstart > 0 as libc::c_int as libc::c_ulong {
            let mut ll_counts2: [size_t; 288] = [0; 288];
            let mut d_counts2: [size_t; 32] = [0; 32];
            ZopfliLZ77GetHistogramAt(
                lz77,
                lstart.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ll_counts2.as_mut_ptr(),
                d_counts2.as_mut_ptr(),
            );
            i = 0 as libc::c_int as size_t;
            while i < 288 as libc::c_int as libc::c_ulong {
                let ref mut fresh7 = *ll_counts.offset(i as isize);
                *fresh7 = (*fresh7 as libc::c_ulong).wrapping_sub(ll_counts2[i as usize])
                    as size_t as size_t;
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < 32 as libc::c_int as libc::c_ulong {
                let ref mut fresh8 = *d_counts.offset(i as isize);
                *fresh8 = (*fresh8 as libc::c_ulong).wrapping_sub(d_counts2[i as usize])
                    as size_t as size_t;
                i = i.wrapping_add(1);
                i;
            }
        }
    };
}
pub unsafe extern "C" fn ZopfliInitBlockState(
    mut options: *const ZopfliOptions,
    mut blockstart: size_t,
    mut blockend: size_t,
    mut add_lmc: libc::c_int,
    mut s: *mut ZopfliBlockState,
) {
    (*s).options = options;
    (*s).blockstart = blockstart;
    (*s).blockend = blockend;
    if add_lmc != 0 {
        (*s)
            .lmc = malloc(
            ::std::mem::size_of::<ZopfliLongestMatchCache>() as libc::c_ulong,
        ) as *mut ZopfliLongestMatchCache;
        ZopfliInitCache(blockend.wrapping_sub(blockstart), (*s).lmc);
    } else {
        (*s).lmc = 0 as *mut ZopfliLongestMatchCache;
    };
}
pub unsafe extern "C" fn ZopfliCleanBlockState(mut s: *mut ZopfliBlockState) {
    if !((*s).lmc).is_null() {
        ZopfliCleanCache((*s).lmc);
        free((*s).lmc as *mut libc::c_void);
    }
}
unsafe extern "C" fn GetLengthScore(
    mut length: libc::c_int,
    mut distance: libc::c_int,
) -> libc::c_int {
    return if distance > 1024 as libc::c_int {
        length - 1 as libc::c_int
    } else {
        length
    };
}
pub unsafe extern "C" fn ZopfliVerifyLenDist(
    mut data: *const libc::c_uchar,
    mut datasize: size_t,
    mut pos: size_t,
    mut dist: libc::c_ushort,
    mut length: libc::c_ushort,
) {
    let mut i: size_t = 0;
    if pos.wrapping_add(length as libc::c_ulong) <= datasize {} else {
        __assert_fail(
            b"pos + length <= datasize\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 96],
                &[libc::c_char; 96],
            >(
                b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6328: {
        if pos.wrapping_add(length as libc::c_ulong) <= datasize {} else {
            __assert_fail(
                b"pos + length <= datasize\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 96],
                    &[libc::c_char; 96],
                >(
                    b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as size_t;
    while i < length as libc::c_ulong {
        if *data.offset(pos.wrapping_sub(dist as libc::c_ulong).wrapping_add(i) as isize)
            as libc::c_int != *data.offset(pos.wrapping_add(i) as isize) as libc::c_int
        {
            if *data
                .offset(pos.wrapping_sub(dist as libc::c_ulong).wrapping_add(i) as isize)
                as libc::c_int
                == *data.offset(pos.wrapping_add(i) as isize) as libc::c_int
            {} else {
                __assert_fail(
                    b"data[pos - dist + i] == data[pos + i]\0" as *const u8
                        as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    282 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 96],
                        &[libc::c_char; 96],
                    >(
                        b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_6213: {
                if *data
                    .offset(
                        pos.wrapping_sub(dist as libc::c_ulong).wrapping_add(i) as isize,
                    ) as libc::c_int
                    == *data.offset(pos.wrapping_add(i) as isize) as libc::c_int
                {} else {
                    __assert_fail(
                        b"data[pos - dist + i] == data[pos + i]\0" as *const u8
                            as *const libc::c_char,
                        b"zopfli/src/zopfli/lz77.c\0" as *const u8
                            as *const libc::c_char,
                        282 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 96],
                            &[libc::c_char; 96],
                        >(
                            b"void ZopfliVerifyLenDist(const unsigned char *, size_t, size_t, unsigned short, unsigned short)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
}
unsafe extern "C" fn GetMatch(
    mut scan: *const libc::c_uchar,
    mut match_0: *const libc::c_uchar,
    mut end: *const libc::c_uchar,
    mut safe_end: *const libc::c_uchar,
) -> *const libc::c_uchar {
    if ::std::mem::size_of::<size_t>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        while scan < safe_end && *(scan as *const size_t) == *(match_0 as *const size_t)
        {
            scan = scan.offset(8 as libc::c_int as isize);
            match_0 = match_0.offset(8 as libc::c_int as isize);
        }
    } else if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
        == 4 as libc::c_int as libc::c_ulong
    {
        while scan < safe_end
            && *(scan as *const libc::c_uint) == *(match_0 as *const libc::c_uint)
        {
            scan = scan.offset(4 as libc::c_int as isize);
            match_0 = match_0.offset(4 as libc::c_int as isize);
        }
    } else {
        while scan < safe_end && *scan as libc::c_int == *match_0 as libc::c_int
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as libc::c_int == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as libc::c_int == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as libc::c_int == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as libc::c_int == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as libc::c_int == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as libc::c_int == *match_0 as libc::c_int
            }
            && {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                *scan as libc::c_int == *match_0 as libc::c_int
            }
        {
            scan = scan.offset(1);
            scan;
            match_0 = match_0.offset(1);
            match_0;
        }
    }
    while scan != end && *scan as libc::c_int == *match_0 as libc::c_int {
        scan = scan.offset(1);
        scan;
        match_0 = match_0.offset(1);
        match_0;
    }
    return scan;
}
unsafe extern "C" fn TryGetFromLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: *mut size_t,
    mut sublen: *mut libc::c_ushort,
    mut distance: *mut libc::c_ushort,
    mut length: *mut libc::c_ushort,
) -> libc::c_int {
    let mut lmcpos: size_t = pos.wrapping_sub((*s).blockstart);
    let mut cache_available: libc::c_uchar = (!((*s).lmc).is_null()
        && (*((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_int
            == 0 as libc::c_int
            || *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                != 0 as libc::c_int)) as libc::c_int as libc::c_uchar;
    let mut limit_ok_for_cache: libc::c_uchar = (cache_available as libc::c_int != 0
        && (*limit == 258 as libc::c_int as libc::c_ulong
            || *((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_ulong <= *limit
            || !sublen.is_null()
                && ZopfliMaxCachedSublen(
                    (*s).lmc,
                    lmcpos,
                    *((*(*s).lmc).length).offset(lmcpos as isize) as size_t,
                ) as libc::c_ulong >= *limit)) as libc::c_int as libc::c_uchar;
    if !((*s).lmc).is_null() && limit_ok_for_cache as libc::c_int != 0
        && cache_available as libc::c_int != 0
    {
        if sublen.is_null()
            || *((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_uint
                <= ZopfliMaxCachedSublen(
                    (*s).lmc,
                    lmcpos,
                    *((*(*s).lmc).length).offset(lmcpos as isize) as size_t,
                )
        {
            *length = *((*(*s).lmc).length).offset(lmcpos as isize);
            if *length as libc::c_ulong > *limit {
                *length = *limit as libc::c_ushort;
            }
            if !sublen.is_null() {
                ZopfliCacheToSublen((*s).lmc, lmcpos, *length as size_t, sublen);
                *distance = *sublen.offset(*length as isize);
                if *limit == 258 as libc::c_int as libc::c_ulong
                    && *length as libc::c_int >= 3 as libc::c_int
                {
                    if *sublen.offset(*length as isize) as libc::c_int
                        == *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                    {} else {
                        __assert_fail(
                            b"sublen[*length] == s->lmc->dist[lmcpos]\0" as *const u8
                                as *const libc::c_char,
                            b"zopfli/src/zopfli/lz77.c\0" as *const u8
                                as *const libc::c_char,
                            366 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 124],
                                &[libc::c_char; 124],
                            >(
                                b"int TryGetFromLongestMatchCache(ZopfliBlockState *, size_t, size_t *, unsigned short *, unsigned short *, unsigned short *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5896: {
                        if *sublen.offset(*length as isize) as libc::c_int
                            == *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                        {} else {
                            __assert_fail(
                                b"sublen[*length] == s->lmc->dist[lmcpos]\0" as *const u8
                                    as *const libc::c_char,
                                b"zopfli/src/zopfli/lz77.c\0" as *const u8
                                    as *const libc::c_char,
                                366 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 124],
                                    &[libc::c_char; 124],
                                >(
                                    b"int TryGetFromLongestMatchCache(ZopfliBlockState *, size_t, size_t *, unsigned short *, unsigned short *, unsigned short *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                }
            } else {
                *distance = *((*(*s).lmc).dist).offset(lmcpos as isize);
            }
            return 1 as libc::c_int;
        }
        *limit = *((*(*s).lmc).length).offset(lmcpos as isize) as size_t;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn StoreInLongestMatchCache(
    mut s: *mut ZopfliBlockState,
    mut pos: size_t,
    mut limit: size_t,
    mut sublen: *const libc::c_ushort,
    mut distance: libc::c_ushort,
    mut length: libc::c_ushort,
) {
    let mut lmcpos: size_t = pos.wrapping_sub((*s).blockstart);
    let mut cache_available: libc::c_uchar = (!((*s).lmc).is_null()
        && (*((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_int
            == 0 as libc::c_int
            || *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                != 0 as libc::c_int)) as libc::c_int as libc::c_uchar;
    if !((*s).lmc).is_null() && limit == 258 as libc::c_int as libc::c_ulong
        && !sublen.is_null() && cache_available == 0
    {
        if *((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_int
            == 1 as libc::c_int
            && *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0\0"
                    as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                399 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4451: {
            if *((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_int
                == 1 as libc::c_int
                && *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                    == 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0\0"
                        as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    399 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *((*(*s).lmc).dist)
            .offset(
                lmcpos as isize,
            ) = (if (length as libc::c_int) < 3 as libc::c_int {
            0 as libc::c_int
        } else {
            distance as libc::c_int
        }) as libc::c_ushort;
        *((*(*s).lmc).length)
            .offset(
                lmcpos as isize,
            ) = (if (length as libc::c_int) < 3 as libc::c_int {
            0 as libc::c_int
        } else {
            length as libc::c_int
        }) as libc::c_ushort;
        if !(*((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_int
            == 1 as libc::c_int
            && *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                == 0 as libc::c_int)
        {} else {
            __assert_fail(
                b"!(s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0)\0"
                    as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                402 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4320: {
            if !(*((*(*s).lmc).length).offset(lmcpos as isize) as libc::c_int
                == 1 as libc::c_int
                && *((*(*s).lmc).dist).offset(lmcpos as isize) as libc::c_int
                    == 0 as libc::c_int)
            {} else {
                __assert_fail(
                    b"!(s->lmc->length[lmcpos] == 1 && s->lmc->dist[lmcpos] == 0)\0"
                        as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    402 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"void StoreInLongestMatchCache(ZopfliBlockState *, size_t, size_t, const unsigned short *, unsigned short, unsigned short)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        ZopfliSublenToCache(sublen, lmcpos, length as size_t, (*s).lmc);
    }
}
pub unsafe extern "C" fn ZopfliFindLongestMatch(
    mut s: *mut ZopfliBlockState,
    mut h: *const ZopfliHash,
    mut array: *const libc::c_uchar,
    mut pos: size_t,
    mut size: size_t,
    mut limit: size_t,
    mut sublen: *mut libc::c_ushort,
    mut distance: *mut libc::c_ushort,
    mut length: *mut libc::c_ushort,
) {
    let mut hpos: libc::c_ushort = (pos
        & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong) as libc::c_ushort;
    let mut p: libc::c_ushort = 0;
    let mut pp: libc::c_ushort = 0;
    let mut bestdist: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut bestlength: libc::c_ushort = 1 as libc::c_int as libc::c_ushort;
    let mut scan: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut match_0: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut arrayend: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut arrayend_safe: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut chain_counter: libc::c_int = 8192 as libc::c_int;
    let mut dist: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hhead: *mut libc::c_int = (*h).head;
    let mut hprev: *mut libc::c_ushort = (*h).prev;
    let mut hhashval: *mut libc::c_int = (*h).hashval;
    let mut hval: libc::c_int = (*h).val;
    if TryGetFromLongestMatchCache(s, pos, &mut limit, sublen, distance, length) != 0 {
        if pos.wrapping_add(*length as libc::c_ulong) <= size {} else {
            __assert_fail(
                b"pos + *length <= size\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                432 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5758: {
            if pos.wrapping_add(*length as libc::c_ulong) <= size {} else {
                __assert_fail(
                    b"pos + *length <= size\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    432 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 169],
                        &[libc::c_char; 169],
                    >(
                        b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return;
    }
    if limit <= 258 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"limit <= ZOPFLI_MAX_MATCH\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            437 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 169],
                &[libc::c_char; 169],
            >(
                b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5716: {
        if limit <= 258 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"limit <= ZOPFLI_MAX_MATCH\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                437 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if limit >= 3 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"limit >= ZOPFLI_MIN_MATCH\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 169],
                &[libc::c_char; 169],
            >(
                b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5677: {
        if limit >= 3 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"limit >= ZOPFLI_MIN_MATCH\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                438 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if pos < size {} else {
        __assert_fail(
            b"pos < size\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 169],
                &[libc::c_char; 169],
            >(
                b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5639: {
        if pos < size {} else {
            __assert_fail(
                b"pos < size\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                439 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if size.wrapping_sub(pos) < 3 as libc::c_int as libc::c_ulong {
        *length = 0 as libc::c_int as libc::c_ushort;
        *distance = 0 as libc::c_int as libc::c_ushort;
        return;
    }
    if pos.wrapping_add(limit) > size {
        limit = size.wrapping_sub(pos);
    }
    arrayend = (&*array.offset(pos as isize) as *const libc::c_uchar)
        .offset(limit as isize);
    arrayend_safe = arrayend.offset(-(8 as libc::c_int as isize));
    if hval < 65536 as libc::c_int {} else {
        __assert_fail(
            b"hval < 65536\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            455 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 169],
                &[libc::c_char; 169],
            >(
                b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5545: {
        if hval < 65536 as libc::c_int {} else {
            __assert_fail(
                b"hval < 65536\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                455 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    pp = *hhead.offset(hval as isize) as libc::c_ushort;
    p = *hprev.offset(pp as isize);
    if pp as libc::c_int == hpos as libc::c_int {} else {
        __assert_fail(
            b"pp == hpos\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            460 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 169],
                &[libc::c_char; 169],
            >(
                b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5486: {
        if pp as libc::c_int == hpos as libc::c_int {} else {
            __assert_fail(
                b"pp == hpos\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                460 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    dist = (if (p as libc::c_int) < pp as libc::c_int {
        pp as libc::c_int - p as libc::c_int
    } else {
        32768 as libc::c_int - p as libc::c_int + pp as libc::c_int
    }) as libc::c_uint;
    while dist < 32768 as libc::c_int as libc::c_uint {
        let mut currentlength: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
        if (p as libc::c_int) < 32768 as libc::c_int {} else {
            __assert_fail(
                b"p < ZOPFLI_WINDOW_SIZE\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                468 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5414: {
            if (p as libc::c_int) < 32768 as libc::c_int {} else {
                __assert_fail(
                    b"p < ZOPFLI_WINDOW_SIZE\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    468 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 169],
                        &[libc::c_char; 169],
                    >(
                        b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if p as libc::c_int == *hprev.offset(pp as isize) as libc::c_int {} else {
            __assert_fail(
                b"p == hprev[pp]\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                469 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5363: {
            if p as libc::c_int == *hprev.offset(pp as isize) as libc::c_int {} else {
                __assert_fail(
                    b"p == hprev[pp]\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    469 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 169],
                        &[libc::c_char; 169],
                    >(
                        b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if *hhashval.offset(p as isize) == hval {} else {
            __assert_fail(
                b"hhashval[p] == hval\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                470 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5317: {
            if *hhashval.offset(p as isize) == hval {} else {
                __assert_fail(
                    b"hhashval[p] == hval\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    470 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 169],
                        &[libc::c_char; 169],
                    >(
                        b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if dist > 0 as libc::c_int as libc::c_uint {
            if pos < size {} else {
                __assert_fail(
                    b"pos < size\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    473 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 169],
                        &[libc::c_char; 169],
                    >(
                        b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5274: {
                if pos < size {} else {
                    __assert_fail(
                        b"pos < size\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/lz77.c\0" as *const u8
                            as *const libc::c_char,
                        473 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 169],
                            &[libc::c_char; 169],
                        >(
                            b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if dist as libc::c_ulong <= pos {} else {
                __assert_fail(
                    b"dist <= pos\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                    474 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 169],
                        &[libc::c_char; 169],
                    >(
                        b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5233: {
                if dist as libc::c_ulong <= pos {} else {
                    __assert_fail(
                        b"dist <= pos\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/lz77.c\0" as *const u8
                            as *const libc::c_char,
                        474 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 169],
                            &[libc::c_char; 169],
                        >(
                            b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            scan = &*array.offset(pos as isize) as *const libc::c_uchar;
            match_0 = &*array.offset(pos.wrapping_sub(dist as libc::c_ulong) as isize)
                as *const libc::c_uchar;
            if pos.wrapping_add(bestlength as libc::c_ulong) >= size
                || *scan.offset(bestlength as libc::c_int as isize) as libc::c_int
                    == *match_0.offset(bestlength as libc::c_int as isize) as libc::c_int
            {
                let mut same0: libc::c_ushort = *((*h).same)
                    .offset(
                        (pos
                            & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                            as isize,
                    );
                if same0 as libc::c_int > 2 as libc::c_int
                    && *scan as libc::c_int == *match_0 as libc::c_int
                {
                    let mut same1: libc::c_ushort = *((*h).same)
                        .offset(
                            (pos.wrapping_sub(dist as libc::c_ulong)
                                & (32768 as libc::c_int - 1 as libc::c_int)
                                    as libc::c_ulong) as isize,
                        );
                    let mut same: libc::c_ushort = (if (same0 as libc::c_int)
                        < same1 as libc::c_int
                    {
                        same0 as libc::c_int
                    } else {
                        same1 as libc::c_int
                    }) as libc::c_ushort;
                    if same as libc::c_ulong > limit {
                        same = limit as libc::c_ushort;
                    }
                    scan = scan.offset(same as libc::c_int as isize);
                    match_0 = match_0.offset(same as libc::c_int as isize);
                }
                scan = GetMatch(scan, match_0, arrayend, arrayend_safe);
                currentlength = scan
                    .offset_from(&*array.offset(pos as isize) as *const libc::c_uchar)
                    as libc::c_long as libc::c_ushort;
            }
            if currentlength as libc::c_int > bestlength as libc::c_int {
                if !sublen.is_null() {
                    let mut j: libc::c_ushort = 0;
                    j = (bestlength as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
                    while j as libc::c_int <= currentlength as libc::c_int {
                        *sublen.offset(j as isize) = dist as libc::c_ushort;
                        j = j.wrapping_add(1);
                        j;
                    }
                }
                bestdist = dist as libc::c_ushort;
                bestlength = currentlength;
                if currentlength as libc::c_ulong >= limit {
                    break;
                }
            }
        }
        if hhead != (*h).head2
            && bestlength as libc::c_int
                >= *((*h).same).offset(hpos as isize) as libc::c_int
            && (*h).val2 == *((*h).hashval2).offset(p as isize)
        {
            hhead = (*h).head2;
            hprev = (*h).prev2;
            hhashval = (*h).hashval2;
            hval = (*h).val2;
        }
        pp = p;
        p = *hprev.offset(p as isize);
        if p as libc::c_int == pp as libc::c_int {
            break;
        }
        dist = dist
            .wrapping_add(
                (if (p as libc::c_int) < pp as libc::c_int {
                    pp as libc::c_int - p as libc::c_int
                } else {
                    32768 as libc::c_int - p as libc::c_int + pp as libc::c_int
                }) as libc::c_uint,
            );
        chain_counter -= 1;
        chain_counter;
        if chain_counter <= 0 as libc::c_int {
            break;
        }
    }
    StoreInLongestMatchCache(s, pos, limit, sublen, bestdist, bestlength);
    if bestlength as libc::c_ulong <= limit {} else {
        __assert_fail(
            b"bestlength <= limit\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            538 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 169],
                &[libc::c_char; 169],
            >(
                b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4220: {
        if bestlength as libc::c_ulong <= limit {} else {
            __assert_fail(
                b"bestlength <= limit\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                538 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *distance = bestdist;
    *length = bestlength;
    if pos.wrapping_add(*length as libc::c_ulong) <= size {} else {
        __assert_fail(
            b"pos + *length <= size\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 169],
                &[libc::c_char; 169],
            >(
                b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4150: {
        if pos.wrapping_add(*length as libc::c_ulong) <= size {} else {
            __assert_fail(
                b"pos + *length <= size\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/lz77.c\0" as *const u8 as *const libc::c_char,
                542 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 169],
                    &[libc::c_char; 169],
                >(
                    b"void ZopfliFindLongestMatch(ZopfliBlockState *, const ZopfliHash *, const unsigned char *, size_t, size_t, size_t, unsigned short *, unsigned short *, unsigned short *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn ZopfliLZ77Greedy(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0;
    let mut leng: libc::c_ushort = 0;
    let mut dist: libc::c_ushort = 0;
    let mut lengthscore: libc::c_int = 0;
    let mut windowstart: size_t = if instart > 32768 as libc::c_int as libc::c_ulong {
        instart.wrapping_sub(32768 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut dummysublen: [libc::c_ushort; 259] = [0; 259];
    let mut prev_length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut prev_match: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut prevlengthscore: libc::c_int = 0;
    let mut match_available: libc::c_int = 0 as libc::c_int;
    if instart == inend {
        return;
    }
    ZopfliResetHash(32768 as libc::c_int as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1);
        i;
    }
    let mut current_block_44: u64;
    i = instart;
    while i < inend {
        ZopfliUpdateHash(in_0, i, inend, h);
        ZopfliFindLongestMatch(
            s,
            h,
            in_0,
            i,
            inend,
            258 as libc::c_int as size_t,
            dummysublen.as_mut_ptr(),
            &mut dist,
            &mut leng,
        );
        lengthscore = GetLengthScore(leng as libc::c_int, dist as libc::c_int);
        prevlengthscore = GetLengthScore(
            prev_length as libc::c_int,
            prev_match as libc::c_int,
        );
        if match_available != 0 {
            match_available = 0 as libc::c_int;
            if lengthscore > prevlengthscore + 1 as libc::c_int {
                ZopfliStoreLitLenDist(
                    *in_0
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    store,
                );
                if lengthscore >= 3 as libc::c_int
                    && (leng as libc::c_int) < 258 as libc::c_int
                {
                    match_available = 1 as libc::c_int;
                    prev_length = leng as libc::c_uint;
                    prev_match = dist as libc::c_uint;
                    current_block_44 = 11650488183268122163;
                } else {
                    current_block_44 = 8704759739624374314;
                }
            } else {
                leng = prev_length as libc::c_ushort;
                dist = prev_match as libc::c_ushort;
                lengthscore = prevlengthscore;
                ZopfliVerifyLenDist(
                    in_0,
                    inend,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    dist,
                    leng,
                );
                ZopfliStoreLitLenDist(
                    leng,
                    dist,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    store,
                );
                j = 2 as libc::c_int as size_t;
                while j < leng as libc::c_ulong {
                    if i < inend {} else {
                        __assert_fail(
                            b"i < inend\0" as *const u8 as *const libc::c_char,
                            b"zopfli/src/zopfli/lz77.c\0" as *const u8
                                as *const libc::c_char,
                            601 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 114],
                                &[libc::c_char; 114],
                            >(
                                b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6613: {
                        if i < inend {} else {
                            __assert_fail(
                                b"i < inend\0" as *const u8 as *const libc::c_char,
                                b"zopfli/src/zopfli/lz77.c\0" as *const u8
                                    as *const libc::c_char,
                                601 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 114],
                                    &[libc::c_char; 114],
                                >(
                                    b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    i = i.wrapping_add(1);
                    i;
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1);
                    j;
                }
                current_block_44 = 11650488183268122163;
            }
        } else if lengthscore >= 3 as libc::c_int
            && (leng as libc::c_int) < 258 as libc::c_int
        {
            match_available = 1 as libc::c_int;
            prev_length = leng as libc::c_uint;
            prev_match = dist as libc::c_uint;
            current_block_44 = 11650488183268122163;
        } else {
            current_block_44 = 8704759739624374314;
        }
        match current_block_44 {
            8704759739624374314 => {
                if lengthscore >= 3 as libc::c_int {
                    ZopfliVerifyLenDist(in_0, inend, i, dist, leng);
                    ZopfliStoreLitLenDist(leng, dist, i, store);
                } else {
                    leng = 1 as libc::c_int as libc::c_ushort;
                    ZopfliStoreLitLenDist(
                        *in_0.offset(i as isize) as libc::c_ushort,
                        0 as libc::c_int as libc::c_ushort,
                        i,
                        store,
                    );
                }
                j = 1 as libc::c_int as size_t;
                while j < leng as libc::c_ulong {
                    if i < inend {} else {
                        __assert_fail(
                            b"i < inend\0" as *const u8 as *const libc::c_char,
                            b"zopfli/src/zopfli/lz77.c\0" as *const u8
                                as *const libc::c_char,
                            626 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 114],
                                &[libc::c_char; 114],
                            >(
                                b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6433: {
                        if i < inend {} else {
                            __assert_fail(
                                b"i < inend\0" as *const u8 as *const libc::c_char,
                                b"zopfli/src/zopfli/lz77.c\0" as *const u8
                                    as *const libc::c_char,
                                626 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 114],
                                    &[libc::c_char; 114],
                                >(
                                    b"void ZopfliLZ77Greedy(ZopfliBlockState *, const unsigned char *, size_t, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    i = i.wrapping_add(1);
                    i;
                    ZopfliUpdateHash(in_0, i, inend, h);
                    j = j.wrapping_add(1);
                    j;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
}
