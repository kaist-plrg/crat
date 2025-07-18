use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ZopfliCleanBlockState(s: *mut ZopfliBlockState);
    fn ZopfliInitBlockState(
        options: *const ZopfliOptions,
        blockstart: size_t,
        blockend: size_t,
        add_lmc: libc::c_int,
        s: *mut ZopfliBlockState,
    );
    fn ZopfliLZ77GetHistogram(
        lz77: *const ZopfliLZ77Store,
        lstart: size_t,
        lend: size_t,
        ll_counts: *mut size_t,
        d_counts: *mut size_t,
    );
    fn ZopfliLZ77GetByteRange(
        lz77: *const ZopfliLZ77Store,
        lstart: size_t,
        lend: size_t,
    ) -> size_t;
    fn ZopfliAppendLZ77Store(
        store: *const ZopfliLZ77Store,
        target: *mut ZopfliLZ77Store,
    );
    fn ZopfliCleanLZ77Store(store: *mut ZopfliLZ77Store);
    fn ZopfliInitLZ77Store(data: *const libc::c_uchar, store: *mut ZopfliLZ77Store);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ZopfliBlockSplitLZ77(
        options: *const ZopfliOptions,
        lz77: *const ZopfliLZ77Store,
        maxblocks: size_t,
        splitpoints: *mut *mut size_t,
        npoints: *mut size_t,
    );
    fn ZopfliBlockSplit(
        options: *const ZopfliOptions,
        in_0: *const libc::c_uchar,
        instart: size_t,
        inend: size_t,
        maxblocks: size_t,
        splitpoints: *mut *mut size_t,
        npoints: *mut size_t,
    );
    fn ZopfliLZ77Optimal(
        s: *mut ZopfliBlockState,
        in_0: *const libc::c_uchar,
        instart: size_t,
        inend: size_t,
        numiterations: libc::c_int,
        store: *mut ZopfliLZ77Store,
    );
    fn ZopfliLZ77OptimalFixed(
        s: *mut ZopfliBlockState,
        in_0: *const libc::c_uchar,
        instart: size_t,
        inend: size_t,
        store: *mut ZopfliLZ77Store,
    );
    fn ZopfliGetDistExtraBits(dist: libc::c_int) -> libc::c_int;
    fn ZopfliGetDistExtraBitsValue(dist: libc::c_int) -> libc::c_int;
    fn ZopfliGetDistSymbol(dist: libc::c_int) -> libc::c_int;
    fn ZopfliGetLengthExtraBits(l: libc::c_int) -> libc::c_int;
    fn ZopfliGetLengthExtraBitsValue(l: libc::c_int) -> libc::c_int;
    fn ZopfliGetLengthSymbol(l: libc::c_int) -> libc::c_int;
    fn ZopfliGetLengthSymbolExtraBits(s: libc::c_int) -> libc::c_int;
    fn ZopfliGetDistSymbolExtraBits(s: libc::c_int) -> libc::c_int;
    fn ZopfliCalculateBitLengths(
        count: *const size_t,
        n: size_t,
        maxbits: libc::c_int,
        bitlengths: *mut libc::c_uint,
    );
    fn ZopfliLengthsToSymbols(
        lengths: *const libc::c_uint,
        n: size_t,
        maxbits: libc::c_uint,
        symbols: *mut libc::c_uint,
    );
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
unsafe extern "C" fn AddBit(
    mut bit: libc::c_int,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    if *bp as libc::c_int == 0 as libc::c_int {
        if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
            *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            } else {
                realloc(
                    *out as *mut libc::c_void,
                    (*outsize)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_uchar;
        }
        *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        *outsize;
    }
    let ref mut fresh0 = *(*out)
        .offset((*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    *fresh0 = (*fresh0 as libc::c_int | bit << *bp as libc::c_int) as libc::c_uchar;
    *bp = (*bp as libc::c_int + 1 as libc::c_int & 7 as libc::c_int) as libc::c_uchar;
}
unsafe extern "C" fn AddBits(
    mut symbol: libc::c_uint,
    mut length: libc::c_uint,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        let mut bit: libc::c_uint = symbol >> i & 1 as libc::c_int as libc::c_uint;
        if *bp as libc::c_int == 0 as libc::c_int {
            if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
            {
                *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                } else {
                    realloc(
                        *out as *mut libc::c_void,
                        (*outsize)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                            ),
                    )
                }) as *mut libc::c_uchar;
            }
            *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
            *outsize = (*outsize).wrapping_add(1);
            *outsize;
        }
        let ref mut fresh1 = *(*out)
            .offset((*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *fresh1 = (*fresh1 as libc::c_uint | bit << *bp as libc::c_int) as libc::c_uchar;
        *bp = (*bp as libc::c_int + 1 as libc::c_int & 7 as libc::c_int)
            as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn AddHuffmanBits(
    mut symbol: libc::c_uint,
    mut length: libc::c_uint,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        let mut bit: libc::c_uint = symbol
            >> length.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint;
        if *bp as libc::c_int == 0 as libc::c_int {
            if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
            {
                *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                } else {
                    realloc(
                        *out as *mut libc::c_void,
                        (*outsize)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                            ),
                    )
                }) as *mut libc::c_uchar;
            }
            *(*out).offset(*outsize as isize) = 0 as libc::c_int as libc::c_uchar;
            *outsize = (*outsize).wrapping_add(1);
            *outsize;
        }
        let ref mut fresh2 = *(*out)
            .offset((*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *fresh2 = (*fresh2 as libc::c_uint | bit << *bp as libc::c_int) as libc::c_uchar;
        *bp = (*bp as libc::c_int + 1 as libc::c_int & 7 as libc::c_int)
            as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn PatchDistanceCodesForBuggyDecoders(
    mut d_lengths: *mut libc::c_uint,
) {
    let mut num_dist_codes: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        if *d_lengths.offset(i as isize) != 0 {
            num_dist_codes += 1;
            num_dist_codes;
        }
        if num_dist_codes >= 2 as libc::c_int {
            return;
        }
        i += 1;
        i;
    }
    if num_dist_codes == 0 as libc::c_int {
        let ref mut fresh3 = *d_lengths.offset(1 as libc::c_int as isize);
        *fresh3 = 1 as libc::c_int as libc::c_uint;
        *d_lengths.offset(0 as libc::c_int as isize) = *fresh3;
    } else if num_dist_codes == 1 as libc::c_int {
        *d_lengths
            .offset(
                (if *d_lengths.offset(0 as libc::c_int as isize) != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as isize,
            ) = 1 as libc::c_int as libc::c_uint;
    }
}
unsafe extern "C" fn EncodeTree(
    mut ll_lengths: *const libc::c_uint,
    mut d_lengths: *const libc::c_uint,
    mut use_16: libc::c_int,
    mut use_17: libc::c_int,
    mut use_18: libc::c_int,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) -> size_t {
    let mut lld_total: libc::c_uint = 0;
    let mut rle: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut rle_bits: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut rle_size: size_t = 0 as libc::c_int as size_t;
    let mut rle_bits_size: size_t = 0 as libc::c_int as size_t;
    let mut hlit: libc::c_uint = 29 as libc::c_int as libc::c_uint;
    let mut hdist: libc::c_uint = 29 as libc::c_int as libc::c_uint;
    let mut hclen: libc::c_uint = 0;
    let mut hlit2: libc::c_uint = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut clcounts: [size_t; 19] = [0; 19];
    let mut clcl: [libc::c_uint; 19] = [0; 19];
    let mut clsymbols: [libc::c_uint; 19] = [0; 19];
    static mut order: [libc::c_uint; 19] = [
        16 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
        18 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        7 as libc::c_int as libc::c_uint,
        9 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        5 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        13 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        14 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        15 as libc::c_int as libc::c_uint,
    ];
    let mut size_only: libc::c_int = out.is_null() as libc::c_int;
    let mut result_size: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 19 as libc::c_int as libc::c_ulong {
        clcounts[i as usize] = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
    while hlit > 0 as libc::c_int as libc::c_uint
        && *ll_lengths
            .offset(
                (257 as libc::c_int as libc::c_uint)
                    .wrapping_add(hlit)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) == 0 as libc::c_int as libc::c_uint
    {
        hlit = hlit.wrapping_sub(1);
        hlit;
    }
    while hdist > 0 as libc::c_int as libc::c_uint
        && *d_lengths
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_add(hdist)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) == 0 as libc::c_int as libc::c_uint
    {
        hdist = hdist.wrapping_sub(1);
        hdist;
    }
    hlit2 = hlit.wrapping_add(257 as libc::c_int as libc::c_uint);
    lld_total = hlit2.wrapping_add(hdist).wrapping_add(1 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int as size_t;
    while i < lld_total as libc::c_ulong {
        let mut symbol: libc::c_uchar = (if i < hlit2 as libc::c_ulong {
            *ll_lengths.offset(i as isize)
        } else {
            *d_lengths.offset(i.wrapping_sub(hlit2 as libc::c_ulong) as isize)
        }) as libc::c_uchar;
        let mut count: libc::c_uint = 1 as libc::c_int as libc::c_uint;
        if use_16 != 0
            || symbol as libc::c_int == 0 as libc::c_int && (use_17 != 0 || use_18 != 0)
        {
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < lld_total as libc::c_ulong
                && symbol as libc::c_uint
                    == (if j < hlit2 as libc::c_ulong {
                        *ll_lengths.offset(j as isize)
                    } else {
                        *d_lengths
                            .offset(j.wrapping_sub(hlit2 as libc::c_ulong) as isize)
                    })
            {
                count = count.wrapping_add(1);
                count;
                j = j.wrapping_add(1);
                j;
            }
        }
        i = (i as libc::c_ulong)
            .wrapping_add(
                count.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ) as size_t as size_t;
        if symbol as libc::c_int == 0 as libc::c_int
            && count >= 3 as libc::c_int as libc::c_uint
        {
            if use_18 != 0 {
                while count >= 11 as libc::c_int as libc::c_uint {
                    let mut count2: libc::c_uint = if count
                        > 138 as libc::c_int as libc::c_uint
                    {
                        138 as libc::c_int as libc::c_uint
                    } else {
                        count
                    };
                    if size_only == 0 {
                        if rle_size
                            & rle_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            == 0
                        {
                            rle = (if rle_size == 0 as libc::c_int as libc::c_ulong {
                                malloc(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                )
                            } else {
                                realloc(
                                    rle as *mut libc::c_void,
                                    rle_size
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                        ),
                                )
                            }) as *mut libc::c_uint;
                        }
                        *rle
                            .offset(
                                rle_size as isize,
                            ) = 18 as libc::c_int as libc::c_uint;
                        rle_size = rle_size.wrapping_add(1);
                        rle_size;
                        if rle_bits_size
                            & rle_bits_size
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
                        {
                            rle_bits = (if rle_bits_size
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                malloc(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                )
                            } else {
                                realloc(
                                    rle_bits as *mut libc::c_void,
                                    rle_bits_size
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                        ),
                                )
                            }) as *mut libc::c_uint;
                        }
                        *rle_bits
                            .offset(
                                rle_bits_size as isize,
                            ) = count2.wrapping_sub(11 as libc::c_int as libc::c_uint);
                        rle_bits_size = rle_bits_size.wrapping_add(1);
                        rle_bits_size;
                    }
                    clcounts[18 as libc::c_int
                        as usize] = (clcounts[18 as libc::c_int as usize])
                        .wrapping_add(1);
                    clcounts[18 as libc::c_int as usize];
                    count = count.wrapping_sub(count2);
                }
            }
            if use_17 != 0 {
                while count >= 3 as libc::c_int as libc::c_uint {
                    let mut count2_0: libc::c_uint = if count
                        > 10 as libc::c_int as libc::c_uint
                    {
                        10 as libc::c_int as libc::c_uint
                    } else {
                        count
                    };
                    if size_only == 0 {
                        if rle_size
                            & rle_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            == 0
                        {
                            rle = (if rle_size == 0 as libc::c_int as libc::c_ulong {
                                malloc(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                )
                            } else {
                                realloc(
                                    rle as *mut libc::c_void,
                                    rle_size
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                        ),
                                )
                            }) as *mut libc::c_uint;
                        }
                        *rle
                            .offset(
                                rle_size as isize,
                            ) = 17 as libc::c_int as libc::c_uint;
                        rle_size = rle_size.wrapping_add(1);
                        rle_size;
                        if rle_bits_size
                            & rle_bits_size
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
                        {
                            rle_bits = (if rle_bits_size
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                malloc(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                )
                            } else {
                                realloc(
                                    rle_bits as *mut libc::c_void,
                                    rle_bits_size
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                        ),
                                )
                            }) as *mut libc::c_uint;
                        }
                        *rle_bits
                            .offset(
                                rle_bits_size as isize,
                            ) = count2_0.wrapping_sub(3 as libc::c_int as libc::c_uint);
                        rle_bits_size = rle_bits_size.wrapping_add(1);
                        rle_bits_size;
                    }
                    clcounts[17 as libc::c_int
                        as usize] = (clcounts[17 as libc::c_int as usize])
                        .wrapping_add(1);
                    clcounts[17 as libc::c_int as usize];
                    count = count.wrapping_sub(count2_0);
                }
            }
        }
        if use_16 != 0 && count >= 4 as libc::c_int as libc::c_uint {
            count = count.wrapping_sub(1);
            count;
            clcounts[symbol as usize] = (clcounts[symbol as usize]).wrapping_add(1);
            clcounts[symbol as usize];
            if size_only == 0 {
                if rle_size & rle_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    == 0
                {
                    rle = (if rle_size == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    } else {
                        realloc(
                            rle as *mut libc::c_void,
                            rle_size
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                ),
                        )
                    }) as *mut libc::c_uint;
                }
                *rle.offset(rle_size as isize) = symbol as libc::c_uint;
                rle_size = rle_size.wrapping_add(1);
                rle_size;
                if rle_bits_size
                    & rle_bits_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
                {
                    rle_bits = (if rle_bits_size == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    } else {
                        realloc(
                            rle_bits as *mut libc::c_void,
                            rle_bits_size
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                ),
                        )
                    }) as *mut libc::c_uint;
                }
                *rle_bits
                    .offset(rle_bits_size as isize) = 0 as libc::c_int as libc::c_uint;
                rle_bits_size = rle_bits_size.wrapping_add(1);
                rle_bits_size;
            }
            while count >= 3 as libc::c_int as libc::c_uint {
                let mut count2_1: libc::c_uint = if count
                    > 6 as libc::c_int as libc::c_uint
                {
                    6 as libc::c_int as libc::c_uint
                } else {
                    count
                };
                if size_only == 0 {
                    if rle_size
                        & rle_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
                    {
                        rle = (if rle_size == 0 as libc::c_int as libc::c_ulong {
                            malloc(
                                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                            )
                        } else {
                            realloc(
                                rle as *mut libc::c_void,
                                rle_size
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                    ),
                            )
                        }) as *mut libc::c_uint;
                    }
                    *rle.offset(rle_size as isize) = 16 as libc::c_int as libc::c_uint;
                    rle_size = rle_size.wrapping_add(1);
                    rle_size;
                    if rle_bits_size
                        & rle_bits_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        == 0
                    {
                        rle_bits = (if rle_bits_size == 0 as libc::c_int as libc::c_ulong
                        {
                            malloc(
                                ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                            )
                        } else {
                            realloc(
                                rle_bits as *mut libc::c_void,
                                rle_bits_size
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                    ),
                            )
                        }) as *mut libc::c_uint;
                    }
                    *rle_bits
                        .offset(
                            rle_bits_size as isize,
                        ) = count2_1.wrapping_sub(3 as libc::c_int as libc::c_uint);
                    rle_bits_size = rle_bits_size.wrapping_add(1);
                    rle_bits_size;
                }
                clcounts[16 as libc::c_int
                    as usize] = (clcounts[16 as libc::c_int as usize]).wrapping_add(1);
                clcounts[16 as libc::c_int as usize];
                count = count.wrapping_sub(count2_1);
            }
        }
        clcounts[symbol
            as usize] = (clcounts[symbol as usize] as libc::c_ulong)
            .wrapping_add(count as libc::c_ulong) as size_t as size_t;
        while count > 0 as libc::c_int as libc::c_uint {
            if size_only == 0 {
                if rle_size & rle_size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    == 0
                {
                    rle = (if rle_size == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    } else {
                        realloc(
                            rle as *mut libc::c_void,
                            rle_size
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                ),
                        )
                    }) as *mut libc::c_uint;
                }
                *rle.offset(rle_size as isize) = symbol as libc::c_uint;
                rle_size = rle_size.wrapping_add(1);
                rle_size;
                if rle_bits_size
                    & rle_bits_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
                {
                    rle_bits = (if rle_bits_size == 0 as libc::c_int as libc::c_ulong {
                        malloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    } else {
                        realloc(
                            rle_bits as *mut libc::c_void,
                            rle_bits_size
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                                ),
                        )
                    }) as *mut libc::c_uint;
                }
                *rle_bits
                    .offset(rle_bits_size as isize) = 0 as libc::c_int as libc::c_uint;
                rle_bits_size = rle_bits_size.wrapping_add(1);
                rle_bits_size;
            }
            count = count.wrapping_sub(1);
            count;
        }
        i = i.wrapping_add(1);
        i;
    }
    ZopfliCalculateBitLengths(
        clcounts.as_mut_ptr(),
        19 as libc::c_int as size_t,
        7 as libc::c_int,
        clcl.as_mut_ptr(),
    );
    if size_only == 0 {
        ZopfliLengthsToSymbols(
            clcl.as_mut_ptr(),
            19 as libc::c_int as size_t,
            7 as libc::c_int as libc::c_uint,
            clsymbols.as_mut_ptr(),
        );
    }
    hclen = 15 as libc::c_int as libc::c_uint;
    while hclen > 0 as libc::c_int as libc::c_uint
        && clcounts[order[hclen
            .wrapping_add(4 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] as usize]
            == 0 as libc::c_int as libc::c_ulong
    {
        hclen = hclen.wrapping_sub(1);
        hclen;
    }
    if size_only == 0 {
        AddBits(hlit, 5 as libc::c_int as libc::c_uint, bp, out, outsize);
        AddBits(hdist, 5 as libc::c_int as libc::c_uint, bp, out, outsize);
        AddBits(hclen, 4 as libc::c_int as libc::c_uint, bp, out, outsize);
        i = 0 as libc::c_int as size_t;
        while i < hclen.wrapping_add(4 as libc::c_int as libc::c_uint) as libc::c_ulong {
            AddBits(
                clcl[order[i as usize] as usize],
                3 as libc::c_int as libc::c_uint,
                bp,
                out,
                outsize,
            );
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < rle_size {
            let mut symbol_0: libc::c_uint = clsymbols[*rle.offset(i as isize) as usize];
            AddHuffmanBits(
                symbol_0,
                clcl[*rle.offset(i as isize) as usize],
                bp,
                out,
                outsize,
            );
            if *rle.offset(i as isize) == 16 as libc::c_int as libc::c_uint {
                AddBits(
                    *rle_bits.offset(i as isize),
                    2 as libc::c_int as libc::c_uint,
                    bp,
                    out,
                    outsize,
                );
            } else if *rle.offset(i as isize) == 17 as libc::c_int as libc::c_uint {
                AddBits(
                    *rle_bits.offset(i as isize),
                    3 as libc::c_int as libc::c_uint,
                    bp,
                    out,
                    outsize,
                );
            } else if *rle.offset(i as isize) == 18 as libc::c_int as libc::c_uint {
                AddBits(
                    *rle_bits.offset(i as isize),
                    7 as libc::c_int as libc::c_uint,
                    bp,
                    out,
                    outsize,
                );
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    result_size = (result_size as libc::c_ulong)
        .wrapping_add(14 as libc::c_int as libc::c_ulong) as size_t as size_t;
    result_size = (result_size as libc::c_ulong)
        .wrapping_add(
            hclen
                .wrapping_add(4 as libc::c_int as libc::c_uint)
                .wrapping_mul(3 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ) as size_t as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 19 as libc::c_int as libc::c_ulong {
        result_size = (result_size as libc::c_ulong)
            .wrapping_add(
                (clcl[i as usize] as libc::c_ulong).wrapping_mul(clcounts[i as usize]),
            ) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    result_size = (result_size as libc::c_ulong)
        .wrapping_add(
            (clcounts[16 as libc::c_int as usize])
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    result_size = (result_size as libc::c_ulong)
        .wrapping_add(
            (clcounts[17 as libc::c_int as usize])
                .wrapping_mul(3 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    result_size = (result_size as libc::c_ulong)
        .wrapping_add(
            (clcounts[18 as libc::c_int as usize])
                .wrapping_mul(7 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    free(rle as *mut libc::c_void);
    free(rle_bits as *mut libc::c_void);
    return result_size;
}
unsafe extern "C" fn AddDynamicTree(
    mut ll_lengths: *const libc::c_uint,
    mut d_lengths: *const libc::c_uint,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0 as libc::c_int;
    let mut bestsize: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut size: size_t = EncodeTree(
            ll_lengths,
            d_lengths,
            i & 1 as libc::c_int,
            i & 2 as libc::c_int,
            i & 4 as libc::c_int,
            0 as *mut libc::c_uchar,
            0 as *mut *mut libc::c_uchar,
            0 as *mut size_t,
        );
        if bestsize == 0 as libc::c_int as libc::c_ulong || size < bestsize {
            bestsize = size;
            best = i;
        }
        i += 1;
        i;
    }
    EncodeTree(
        ll_lengths,
        d_lengths,
        best & 1 as libc::c_int,
        best & 2 as libc::c_int,
        best & 4 as libc::c_int,
        bp,
        out,
        outsize,
    );
}
unsafe extern "C" fn CalculateTreeSize(
    mut ll_lengths: *const libc::c_uint,
    mut d_lengths: *const libc::c_uint,
) -> size_t {
    let mut result: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut size: size_t = EncodeTree(
            ll_lengths,
            d_lengths,
            i & 1 as libc::c_int,
            i & 2 as libc::c_int,
            i & 4 as libc::c_int,
            0 as *mut libc::c_uchar,
            0 as *mut *mut libc::c_uchar,
            0 as *mut size_t,
        );
        if result == 0 as libc::c_int as libc::c_ulong || size < result {
            result = size;
        }
        i += 1;
        i;
    }
    return result;
}
unsafe extern "C" fn AddLZ77Data(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut expected_data_size: size_t,
    mut ll_symbols: *const libc::c_uint,
    mut ll_lengths: *const libc::c_uint,
    mut d_symbols: *const libc::c_uint,
    mut d_lengths: *const libc::c_uint,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut testlength: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = lstart;
    while i < lend {
        let mut dist: libc::c_uint = *((*lz77).dists).offset(i as isize) as libc::c_uint;
        let mut litlen: libc::c_uint = *((*lz77).litlens).offset(i as isize)
            as libc::c_uint;
        if dist == 0 as libc::c_int as libc::c_uint {
            if litlen < 256 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"litlen < 256\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    311 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 199],
                        &[libc::c_char; 199],
                    >(
                        b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_3038: {
                if litlen < 256 as libc::c_int as libc::c_uint {} else {
                    __assert_fail(
                        b"litlen < 256\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/deflate.c\0" as *const u8
                            as *const libc::c_char,
                        311 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 199],
                            &[libc::c_char; 199],
                        >(
                            b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if *ll_lengths.offset(litlen as isize) > 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"ll_lengths[litlen] > 0\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    312 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 199],
                        &[libc::c_char; 199],
                    >(
                        b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2991: {
                if *ll_lengths.offset(litlen as isize) > 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"ll_lengths[litlen] > 0\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/deflate.c\0" as *const u8
                            as *const libc::c_char,
                        312 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 199],
                            &[libc::c_char; 199],
                        >(
                            b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            AddHuffmanBits(
                *ll_symbols.offset(litlen as isize),
                *ll_lengths.offset(litlen as isize),
                bp,
                out,
                outsize,
            );
            testlength = testlength.wrapping_add(1);
            testlength;
        } else {
            let mut lls: libc::c_uint = ZopfliGetLengthSymbol(litlen as libc::c_int)
                as libc::c_uint;
            let mut ds: libc::c_uint = ZopfliGetDistSymbol(dist as libc::c_int)
                as libc::c_uint;
            if litlen >= 3 as libc::c_int as libc::c_uint
                && litlen <= 288 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"litlen >= 3 && litlen <= 288\0" as *const u8
                        as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    318 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 199],
                        &[libc::c_char; 199],
                    >(
                        b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2913: {
                if litlen >= 3 as libc::c_int as libc::c_uint
                    && litlen <= 288 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"litlen >= 3 && litlen <= 288\0" as *const u8
                            as *const libc::c_char,
                        b"zopfli/src/zopfli/deflate.c\0" as *const u8
                            as *const libc::c_char,
                        318 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 199],
                            &[libc::c_char; 199],
                        >(
                            b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if *ll_lengths.offset(lls as isize) > 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"ll_lengths[lls] > 0\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    319 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 199],
                        &[libc::c_char; 199],
                    >(
                        b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2866: {
                if *ll_lengths.offset(lls as isize) > 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"ll_lengths[lls] > 0\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/deflate.c\0" as *const u8
                            as *const libc::c_char,
                        319 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 199],
                            &[libc::c_char; 199],
                        >(
                            b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if *d_lengths.offset(ds as isize) > 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"d_lengths[ds] > 0\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    320 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 199],
                        &[libc::c_char; 199],
                    >(
                        b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2819: {
                if *d_lengths.offset(ds as isize) > 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"d_lengths[ds] > 0\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/deflate.c\0" as *const u8
                            as *const libc::c_char,
                        320 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 199],
                            &[libc::c_char; 199],
                        >(
                            b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            AddHuffmanBits(
                *ll_symbols.offset(lls as isize),
                *ll_lengths.offset(lls as isize),
                bp,
                out,
                outsize,
            );
            AddBits(
                ZopfliGetLengthExtraBitsValue(litlen as libc::c_int) as libc::c_uint,
                ZopfliGetLengthExtraBits(litlen as libc::c_int) as libc::c_uint,
                bp,
                out,
                outsize,
            );
            AddHuffmanBits(
                *d_symbols.offset(ds as isize),
                *d_lengths.offset(ds as isize),
                bp,
                out,
                outsize,
            );
            AddBits(
                ZopfliGetDistExtraBitsValue(dist as libc::c_int) as libc::c_uint,
                ZopfliGetDistExtraBits(dist as libc::c_int) as libc::c_uint,
                bp,
                out,
                outsize,
            );
            testlength = (testlength as libc::c_ulong)
                .wrapping_add(litlen as libc::c_ulong) as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    if expected_data_size == 0 as libc::c_int as libc::c_ulong
        || testlength == expected_data_size
    {} else {
        __assert_fail(
            b"expected_data_size == 0 || testlength == expected_data_size\0" as *const u8
                as *const libc::c_char,
            b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
            332 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 199],
                &[libc::c_char; 199],
            >(
                b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2417: {
        if expected_data_size == 0 as libc::c_int as libc::c_ulong
            || testlength == expected_data_size
        {} else {
            __assert_fail(
                b"expected_data_size == 0 || testlength == expected_data_size\0"
                    as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                332 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 199],
                    &[libc::c_char; 199],
                >(
                    b"void AddLZ77Data(const ZopfliLZ77Store *, size_t, size_t, size_t, const unsigned int *, const unsigned int *, const unsigned int *, const unsigned int *, unsigned char *, unsigned char **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn GetFixedTree(
    mut ll_lengths: *mut libc::c_uint,
    mut d_lengths: *mut libc::c_uint,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 144 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 8 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    i = 144 as libc::c_int as size_t;
    while i < 256 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 9 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    i = 256 as libc::c_int as size_t;
    while i < 280 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 7 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    i = 280 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        *ll_lengths.offset(i as isize) = 8 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        *d_lengths.offset(i as isize) = 5 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn CalculateBlockSymbolSizeSmall(
    mut ll_lengths: *const libc::c_uint,
    mut d_lengths: *const libc::c_uint,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let mut result: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = lstart;
    while i < lend {
        if i < (*lz77).size {} else {
            __assert_fail(
                b"i < lz77->size\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                355 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"size_t CalculateBlockSymbolSizeSmall(const unsigned int *, const unsigned int *, const ZopfliLZ77Store *, size_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5648: {
            if i < (*lz77).size {} else {
                __assert_fail(
                    b"i < lz77->size\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    355 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"size_t CalculateBlockSymbolSizeSmall(const unsigned int *, const unsigned int *, const ZopfliLZ77Store *, size_t, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (*((*lz77).litlens).offset(i as isize) as libc::c_int) < 259 as libc::c_int
        {} else {
            __assert_fail(
                b"lz77->litlens[i] < 259\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                356 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 122],
                    &[libc::c_char; 122],
                >(
                    b"size_t CalculateBlockSymbolSizeSmall(const unsigned int *, const unsigned int *, const ZopfliLZ77Store *, size_t, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5597: {
            if (*((*lz77).litlens).offset(i as isize) as libc::c_int)
                < 259 as libc::c_int
            {} else {
                __assert_fail(
                    b"lz77->litlens[i] < 259\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    356 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 122],
                        &[libc::c_char; 122],
                    >(
                        b"size_t CalculateBlockSymbolSizeSmall(const unsigned int *, const unsigned int *, const ZopfliLZ77Store *, size_t, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if *((*lz77).dists).offset(i as isize) as libc::c_int == 0 as libc::c_int {
            result = (result as libc::c_ulong)
                .wrapping_add(
                    *ll_lengths.offset(*((*lz77).litlens).offset(i as isize) as isize)
                        as libc::c_ulong,
                ) as size_t as size_t;
        } else {
            let mut ll_symbol: libc::c_int = ZopfliGetLengthSymbol(
                *((*lz77).litlens).offset(i as isize) as libc::c_int,
            );
            let mut d_symbol: libc::c_int = ZopfliGetDistSymbol(
                *((*lz77).dists).offset(i as isize) as libc::c_int,
            );
            result = (result as libc::c_ulong)
                .wrapping_add(*ll_lengths.offset(ll_symbol as isize) as libc::c_ulong)
                as size_t as size_t;
            result = (result as libc::c_ulong)
                .wrapping_add(*d_lengths.offset(d_symbol as isize) as libc::c_ulong)
                as size_t as size_t;
            result = (result as libc::c_ulong)
                .wrapping_add(ZopfliGetLengthSymbolExtraBits(ll_symbol) as libc::c_ulong)
                as size_t as size_t;
            result = (result as libc::c_ulong)
                .wrapping_add(ZopfliGetDistSymbolExtraBits(d_symbol) as libc::c_ulong)
                as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    result = (result as libc::c_ulong)
        .wrapping_add(*ll_lengths.offset(256 as libc::c_int as isize) as libc::c_ulong)
        as size_t as size_t;
    return result;
}
unsafe extern "C" fn CalculateBlockSymbolSizeGivenCounts(
    mut ll_counts: *const size_t,
    mut d_counts: *const size_t,
    mut ll_lengths: *const libc::c_uint,
    mut d_lengths: *const libc::c_uint,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    let mut result: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    if lstart.wrapping_add((288 as libc::c_int * 3 as libc::c_int) as libc::c_ulong)
        > lend
    {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend)
    } else {
        i = 0 as libc::c_int as size_t;
        while i < 256 as libc::c_int as libc::c_ulong {
            result = (result as libc::c_ulong)
                .wrapping_add(
                    (*ll_lengths.offset(i as isize) as libc::c_ulong)
                        .wrapping_mul(*ll_counts.offset(i as isize)),
                ) as size_t as size_t;
            i = i.wrapping_add(1);
            i;
        }
        i = 257 as libc::c_int as size_t;
        while i < 286 as libc::c_int as libc::c_ulong {
            result = (result as libc::c_ulong)
                .wrapping_add(
                    (*ll_lengths.offset(i as isize) as libc::c_ulong)
                        .wrapping_mul(*ll_counts.offset(i as isize)),
                ) as size_t as size_t;
            result = (result as libc::c_ulong)
                .wrapping_add(
                    (ZopfliGetLengthSymbolExtraBits(i as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(*ll_counts.offset(i as isize)),
                ) as size_t as size_t;
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < 30 as libc::c_int as libc::c_ulong {
            result = (result as libc::c_ulong)
                .wrapping_add(
                    (*d_lengths.offset(i as isize) as libc::c_ulong)
                        .wrapping_mul(*d_counts.offset(i as isize)),
                ) as size_t as size_t;
            result = (result as libc::c_ulong)
                .wrapping_add(
                    (ZopfliGetDistSymbolExtraBits(i as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(*d_counts.offset(i as isize)),
                ) as size_t as size_t;
            i = i.wrapping_add(1);
            i;
        }
        result = (result as libc::c_ulong)
            .wrapping_add(
                *ll_lengths.offset(256 as libc::c_int as isize) as libc::c_ulong,
            ) as size_t as size_t;
        return result;
    };
}
unsafe extern "C" fn CalculateBlockSymbolSize(
    mut ll_lengths: *const libc::c_uint,
    mut d_lengths: *const libc::c_uint,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> size_t {
    if lstart.wrapping_add((288 as libc::c_int * 3 as libc::c_int) as libc::c_ulong)
        > lend
    {
        return CalculateBlockSymbolSizeSmall(ll_lengths, d_lengths, lz77, lstart, lend)
    } else {
        let mut ll_counts: [size_t; 288] = [0; 288];
        let mut d_counts: [size_t; 32] = [0; 32];
        ZopfliLZ77GetHistogram(
            lz77,
            lstart,
            lend,
            ll_counts.as_mut_ptr(),
            d_counts.as_mut_ptr(),
        );
        return CalculateBlockSymbolSizeGivenCounts(
            ll_counts.as_mut_ptr(),
            d_counts.as_mut_ptr(),
            ll_lengths,
            d_lengths,
            lz77,
            lstart,
            lend,
        );
    };
}
unsafe extern "C" fn AbsDiff(mut x: size_t, mut y: size_t) -> size_t {
    if x > y { return x.wrapping_sub(y) } else { return y.wrapping_sub(x) };
}
pub unsafe extern "C" fn OptimizeHuffmanForRle(
    mut length: libc::c_uint,
    mut counts: *mut size_t,
) {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_int = 0;
    let mut stride: libc::c_int = 0;
    let mut symbol: size_t = 0;
    let mut sum: size_t = 0;
    let mut limit: size_t = 0;
    let mut good_for_rle: *mut libc::c_int = 0 as *mut libc::c_int;
    while length > 0 as libc::c_int as libc::c_uint {
        if *counts.offset(length.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            != 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        length = length.wrapping_sub(1);
        length;
    }
    if length == 0 as libc::c_int as libc::c_uint {
        return;
    }
    good_for_rle = malloc(
        (length as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        *good_for_rle.offset(i as isize) = 0 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    symbol = *counts.offset(0 as libc::c_int as isize);
    stride = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length.wrapping_add(1 as libc::c_int as libc::c_uint) {
        if i == length || *counts.offset(i as isize) != symbol {
            if symbol == 0 as libc::c_int as libc::c_ulong && stride >= 5 as libc::c_int
                || symbol != 0 as libc::c_int as libc::c_ulong
                    && stride >= 7 as libc::c_int
            {
                k = 0 as libc::c_int;
                while k < stride {
                    *good_for_rle
                        .offset(
                            i
                                .wrapping_sub(k as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) = 1 as libc::c_int;
                    k += 1;
                    k;
                }
            }
            stride = 1 as libc::c_int;
            if i != length {
                symbol = *counts.offset(i as isize);
            }
        } else {
            stride += 1;
            stride;
        }
        i = i.wrapping_add(1);
        i;
    }
    stride = 0 as libc::c_int;
    limit = *counts.offset(0 as libc::c_int as isize);
    sum = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as libc::c_uint;
    while i < length.wrapping_add(1 as libc::c_int as libc::c_uint) {
        if i == length || *good_for_rle.offset(i as isize) != 0
            || AbsDiff(*counts.offset(i as isize), limit)
                >= 4 as libc::c_int as libc::c_ulong
        {
            if stride >= 4 as libc::c_int
                || stride >= 3 as libc::c_int && sum == 0 as libc::c_int as libc::c_ulong
            {
                let mut count: libc::c_int = sum
                    .wrapping_add((stride / 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_div(stride as libc::c_ulong) as libc::c_int;
                if count < 1 as libc::c_int {
                    count = 1 as libc::c_int;
                }
                if sum == 0 as libc::c_int as libc::c_ulong {
                    count = 0 as libc::c_int;
                }
                k = 0 as libc::c_int;
                while k < stride {
                    *counts
                        .offset(
                            i
                                .wrapping_sub(k as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) = count as size_t;
                    k += 1;
                    k;
                }
            }
            stride = 0 as libc::c_int;
            sum = 0 as libc::c_int as size_t;
            if i < length.wrapping_sub(3 as libc::c_int as libc::c_uint) {
                limit = (*counts.offset(i as isize))
                    .wrapping_add(
                        *counts
                            .offset(
                                i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ),
                    )
                    .wrapping_add(
                        *counts
                            .offset(
                                i.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                            ),
                    )
                    .wrapping_add(
                        *counts
                            .offset(
                                i.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                            ),
                    )
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_div(4 as libc::c_int as libc::c_ulong);
            } else if i < length {
                limit = *counts.offset(i as isize);
            } else {
                limit = 0 as libc::c_int as size_t;
            }
        }
        stride += 1;
        stride;
        if i != length {
            sum = (sum as libc::c_ulong).wrapping_add(*counts.offset(i as isize))
                as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(good_for_rle as *mut libc::c_void);
}
unsafe extern "C" fn TryOptimizeHuffmanForRle(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_counts: *const size_t,
    mut d_counts: *const size_t,
    mut ll_lengths: *mut libc::c_uint,
    mut d_lengths: *mut libc::c_uint,
) -> libc::c_double {
    let mut ll_counts2: [size_t; 288] = [0; 288];
    let mut d_counts2: [size_t; 32] = [0; 32];
    let mut ll_lengths2: [libc::c_uint; 288] = [0; 288];
    let mut d_lengths2: [libc::c_uint; 32] = [0; 32];
    let mut treesize: libc::c_double = 0.;
    let mut datasize: libc::c_double = 0.;
    let mut treesize2: libc::c_double = 0.;
    let mut datasize2: libc::c_double = 0.;
    treesize = CalculateTreeSize(ll_lengths, d_lengths) as libc::c_double;
    datasize = CalculateBlockSymbolSizeGivenCounts(
        ll_counts,
        d_counts,
        ll_lengths,
        d_lengths,
        lz77,
        lstart,
        lend,
    ) as libc::c_double;
    memcpy(
        ll_counts2.as_mut_ptr() as *mut libc::c_void,
        ll_counts as *const libc::c_void,
        ::std::mem::size_of::<[size_t; 288]>() as libc::c_ulong,
    );
    memcpy(
        d_counts2.as_mut_ptr() as *mut libc::c_void,
        d_counts as *const libc::c_void,
        ::std::mem::size_of::<[size_t; 32]>() as libc::c_ulong,
    );
    OptimizeHuffmanForRle(288 as libc::c_int as libc::c_uint, ll_counts2.as_mut_ptr());
    OptimizeHuffmanForRle(32 as libc::c_int as libc::c_uint, d_counts2.as_mut_ptr());
    ZopfliCalculateBitLengths(
        ll_counts2.as_mut_ptr(),
        288 as libc::c_int as size_t,
        15 as libc::c_int,
        ll_lengths2.as_mut_ptr(),
    );
    ZopfliCalculateBitLengths(
        d_counts2.as_mut_ptr(),
        32 as libc::c_int as size_t,
        15 as libc::c_int,
        d_lengths2.as_mut_ptr(),
    );
    PatchDistanceCodesForBuggyDecoders(d_lengths2.as_mut_ptr());
    treesize2 = CalculateTreeSize(ll_lengths2.as_mut_ptr(), d_lengths2.as_mut_ptr())
        as libc::c_double;
    datasize2 = CalculateBlockSymbolSizeGivenCounts(
        ll_counts,
        d_counts,
        ll_lengths2.as_mut_ptr(),
        d_lengths2.as_mut_ptr(),
        lz77,
        lstart,
        lend,
    ) as libc::c_double;
    if treesize2 + datasize2 < treesize + datasize {
        memcpy(
            ll_lengths as *mut libc::c_void,
            ll_lengths2.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uint; 288]>() as libc::c_ulong,
        );
        memcpy(
            d_lengths as *mut libc::c_void,
            d_lengths2.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uint; 32]>() as libc::c_ulong,
        );
        return treesize2 + datasize2;
    }
    return treesize + datasize;
}
unsafe extern "C" fn GetDynamicLengths(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut ll_lengths: *mut libc::c_uint,
    mut d_lengths: *mut libc::c_uint,
) -> libc::c_double {
    let mut ll_counts: [size_t; 288] = [0; 288];
    let mut d_counts: [size_t; 32] = [0; 32];
    ZopfliLZ77GetHistogram(
        lz77,
        lstart,
        lend,
        ll_counts.as_mut_ptr(),
        d_counts.as_mut_ptr(),
    );
    ll_counts[256 as libc::c_int as usize] = 1 as libc::c_int as size_t;
    ZopfliCalculateBitLengths(
        ll_counts.as_mut_ptr(),
        288 as libc::c_int as size_t,
        15 as libc::c_int,
        ll_lengths,
    );
    ZopfliCalculateBitLengths(
        d_counts.as_mut_ptr(),
        32 as libc::c_int as size_t,
        15 as libc::c_int,
        d_lengths,
    );
    PatchDistanceCodesForBuggyDecoders(d_lengths);
    return TryOptimizeHuffmanForRle(
        lz77,
        lstart,
        lend,
        ll_counts.as_mut_ptr(),
        d_counts.as_mut_ptr(),
        ll_lengths,
        d_lengths,
    );
}
pub unsafe extern "C" fn ZopfliCalculateBlockSize(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut btype: libc::c_int,
) -> libc::c_double {
    let mut ll_lengths: [libc::c_uint; 288] = [0; 288];
    let mut d_lengths: [libc::c_uint; 32] = [0; 32];
    let mut result: libc::c_double = 3 as libc::c_int as libc::c_double;
    if btype == 0 as libc::c_int {
        let mut length: size_t = ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let mut rem: size_t = length.wrapping_rem(65535 as libc::c_int as libc::c_ulong);
        let mut blocks: size_t = length
            .wrapping_div(65535 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (if rem != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
                    as libc::c_ulong,
            );
        return blocks
            .wrapping_mul(5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_add(length.wrapping_mul(8 as libc::c_int as libc::c_ulong))
            as libc::c_double;
    }
    if btype == 1 as libc::c_int {
        GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
        result
            += CalculateBlockSymbolSize(
                ll_lengths.as_mut_ptr(),
                d_lengths.as_mut_ptr(),
                lz77,
                lstart,
                lend,
            ) as libc::c_double;
    } else {
        result
            += GetDynamicLengths(
                lz77,
                lstart,
                lend,
                ll_lengths.as_mut_ptr(),
                d_lengths.as_mut_ptr(),
            );
    }
    return result;
}
pub unsafe extern "C" fn ZopfliCalculateBlockSizeAutoType(
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
) -> libc::c_double {
    let mut uncompressedcost: libc::c_double = ZopfliCalculateBlockSize(
        lz77,
        lstart,
        lend,
        0 as libc::c_int,
    );
    let mut fixedcost: libc::c_double = if (*lz77).size
        > 1000 as libc::c_int as libc::c_ulong
    {
        uncompressedcost
    } else {
        ZopfliCalculateBlockSize(lz77, lstart, lend, 1 as libc::c_int)
    };
    let mut dyncost: libc::c_double = ZopfliCalculateBlockSize(
        lz77,
        lstart,
        lend,
        2 as libc::c_int,
    );
    return if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        uncompressedcost
    } else if fixedcost < dyncost {
        fixedcost
    } else {
        dyncost
    };
}
unsafe extern "C" fn AddNonCompressedBlock(
    mut options: *const ZopfliOptions,
    mut final_0: libc::c_int,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut pos: size_t = instart;
    loop {
        let mut i: size_t = 0;
        let mut blocksize: libc::c_ushort = 65535 as libc::c_int as libc::c_ushort;
        let mut nlen: libc::c_ushort = 0;
        let mut currentfinal: libc::c_int = 0;
        if pos.wrapping_add(blocksize as libc::c_ulong) > inend {
            blocksize = inend.wrapping_sub(pos) as libc::c_ushort;
        }
        currentfinal = (pos.wrapping_add(blocksize as libc::c_ulong) >= inend)
            as libc::c_int;
        nlen = !(blocksize as libc::c_int) as libc::c_ushort;
        AddBit((final_0 != 0 && currentfinal != 0) as libc::c_int, bp, out, outsize);
        AddBit(0 as libc::c_int, bp, out, outsize);
        AddBit(0 as libc::c_int, bp, out, outsize);
        *bp = 0 as libc::c_int as libc::c_uchar;
        if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
            *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            } else {
                realloc(
                    *out as *mut libc::c_void,
                    (*outsize)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_uchar;
        }
        *(*out)
            .offset(
                *outsize as isize,
            ) = (blocksize as libc::c_int % 256 as libc::c_int) as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        *outsize;
        if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
            *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            } else {
                realloc(
                    *out as *mut libc::c_void,
                    (*outsize)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_uchar;
        }
        *(*out)
            .offset(
                *outsize as isize,
            ) = (blocksize as libc::c_int / 256 as libc::c_int % 256 as libc::c_int)
            as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        *outsize;
        if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
            *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            } else {
                realloc(
                    *out as *mut libc::c_void,
                    (*outsize)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_uchar;
        }
        *(*out)
            .offset(
                *outsize as isize,
            ) = (nlen as libc::c_int % 256 as libc::c_int) as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        *outsize;
        if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
            *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            } else {
                realloc(
                    *out as *mut libc::c_void,
                    (*outsize)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_uchar;
        }
        *(*out)
            .offset(
                *outsize as isize,
            ) = (nlen as libc::c_int / 256 as libc::c_int % 256 as libc::c_int)
            as libc::c_uchar;
        *outsize = (*outsize).wrapping_add(1);
        *outsize;
        i = 0 as libc::c_int as size_t;
        while i < blocksize as libc::c_ulong {
            if *outsize & (*outsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0
            {
                *out = (if *outsize == 0 as libc::c_int as libc::c_ulong {
                    malloc(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                } else {
                    realloc(
                        *out as *mut libc::c_void,
                        (*outsize)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                            ),
                    )
                }) as *mut libc::c_uchar;
            }
            *(*out)
                .offset(*outsize as isize) = *in_0.offset(pos.wrapping_add(i) as isize);
            *outsize = (*outsize).wrapping_add(1);
            *outsize;
            i = i.wrapping_add(1);
            i;
        }
        if currentfinal != 0 {
            break;
        }
        pos = (pos as libc::c_ulong).wrapping_add(blocksize as libc::c_ulong) as size_t
            as size_t;
    };
}
unsafe extern "C" fn AddLZ77Block(
    mut options: *const ZopfliOptions,
    mut btype: libc::c_int,
    mut final_0: libc::c_int,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut expected_data_size: size_t,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut ll_lengths: [libc::c_uint; 288] = [0; 288];
    let mut d_lengths: [libc::c_uint; 32] = [0; 32];
    let mut ll_symbols: [libc::c_uint; 288] = [0; 288];
    let mut d_symbols: [libc::c_uint; 32] = [0; 32];
    let mut detect_block_size: size_t = *outsize;
    let mut compressed_size: size_t = 0;
    let mut uncompressed_size: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    if btype == 0 as libc::c_int {
        let mut length: size_t = ZopfliLZ77GetByteRange(lz77, lstart, lend);
        let mut pos: size_t = if lstart == lend {
            0 as libc::c_int as libc::c_ulong
        } else {
            *((*lz77).pos).offset(lstart as isize)
        };
        let mut end: size_t = pos.wrapping_add(length);
        AddNonCompressedBlock(
            options,
            final_0,
            (*lz77).data,
            pos,
            end,
            bp,
            out,
            outsize,
        );
        return;
    }
    AddBit(final_0, bp, out, outsize);
    AddBit(btype & 1 as libc::c_int, bp, out, outsize);
    AddBit((btype & 2 as libc::c_int) >> 1 as libc::c_int, bp, out, outsize);
    if btype == 1 as libc::c_int {
        GetFixedTree(ll_lengths.as_mut_ptr(), d_lengths.as_mut_ptr());
    } else {
        let mut detect_tree_size: libc::c_uint = 0;
        if btype == 2 as libc::c_int {} else {
            __assert_fail(
                b"btype == 2\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                716 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 145],
                    &[libc::c_char; 145],
                >(
                    b"void AddLZ77Block(const ZopfliOptions *, int, int, const ZopfliLZ77Store *, size_t, size_t, size_t, unsigned char *, unsigned char **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_6508: {
            if btype == 2 as libc::c_int {} else {
                __assert_fail(
                    b"btype == 2\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/deflate.c\0" as *const u8 as *const libc::c_char,
                    716 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 145],
                        &[libc::c_char; 145],
                    >(
                        b"void AddLZ77Block(const ZopfliOptions *, int, int, const ZopfliLZ77Store *, size_t, size_t, size_t, unsigned char *, unsigned char **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        GetDynamicLengths(
            lz77,
            lstart,
            lend,
            ll_lengths.as_mut_ptr(),
            d_lengths.as_mut_ptr(),
        );
        detect_tree_size = *outsize as libc::c_uint;
        AddDynamicTree(
            ll_lengths.as_mut_ptr(),
            d_lengths.as_mut_ptr(),
            bp,
            out,
            outsize,
        );
        if (*options).verbose != 0 {
            fprintf(
                stderr,
                b"treesize: %d\n\0" as *const u8 as *const libc::c_char,
                (*outsize).wrapping_sub(detect_tree_size as libc::c_ulong) as libc::c_int,
            );
        }
    }
    ZopfliLengthsToSymbols(
        ll_lengths.as_mut_ptr(),
        288 as libc::c_int as size_t,
        15 as libc::c_int as libc::c_uint,
        ll_symbols.as_mut_ptr(),
    );
    ZopfliLengthsToSymbols(
        d_lengths.as_mut_ptr(),
        32 as libc::c_int as size_t,
        15 as libc::c_int as libc::c_uint,
        d_symbols.as_mut_ptr(),
    );
    detect_block_size = *outsize;
    AddLZ77Data(
        lz77,
        lstart,
        lend,
        expected_data_size,
        ll_symbols.as_mut_ptr(),
        ll_lengths.as_mut_ptr(),
        d_symbols.as_mut_ptr(),
        d_lengths.as_mut_ptr(),
        bp,
        out,
        outsize,
    );
    AddHuffmanBits(
        ll_symbols[256 as libc::c_int as usize],
        ll_lengths[256 as libc::c_int as usize],
        bp,
        out,
        outsize,
    );
    i = lstart;
    while i < lend {
        uncompressed_size = (uncompressed_size as libc::c_ulong)
            .wrapping_add(
                (if *((*lz77).dists).offset(i as isize) as libc::c_int
                    == 0 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    *((*lz77).litlens).offset(i as isize) as libc::c_int
                }) as libc::c_ulong,
            ) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    compressed_size = (*outsize).wrapping_sub(detect_block_size);
    if (*options).verbose != 0 {
        fprintf(
            stderr,
            b"compressed block size: %d (%dk) (unc: %d)\n\0" as *const u8
                as *const libc::c_char,
            compressed_size as libc::c_int,
            compressed_size.wrapping_div(1024 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            uncompressed_size as libc::c_int,
        );
    }
}
unsafe extern "C" fn AddLZ77BlockAutoType(
    mut options: *const ZopfliOptions,
    mut final_0: libc::c_int,
    mut lz77: *const ZopfliLZ77Store,
    mut lstart: size_t,
    mut lend: size_t,
    mut expected_data_size: size_t,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut uncompressedcost: libc::c_double = ZopfliCalculateBlockSize(
        lz77,
        lstart,
        lend,
        0 as libc::c_int,
    );
    let mut fixedcost: libc::c_double = ZopfliCalculateBlockSize(
        lz77,
        lstart,
        lend,
        1 as libc::c_int,
    );
    let mut dyncost: libc::c_double = ZopfliCalculateBlockSize(
        lz77,
        lstart,
        lend,
        2 as libc::c_int,
    );
    let mut expensivefixed: libc::c_int = ((*lz77).size
        < 1000 as libc::c_int as libc::c_ulong || fixedcost <= dyncost * 1.1f64)
        as libc::c_int;
    let mut fixedstore: ZopfliLZ77Store = ZopfliLZ77Store {
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
    if lstart == lend {
        AddBits(
            final_0 as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
            bp,
            out,
            outsize,
        );
        AddBits(
            1 as libc::c_int as libc::c_uint,
            2 as libc::c_int as libc::c_uint,
            bp,
            out,
            outsize,
        );
        AddBits(
            0 as libc::c_int as libc::c_uint,
            7 as libc::c_int as libc::c_uint,
            bp,
            out,
            outsize,
        );
        return;
    }
    ZopfliInitLZ77Store((*lz77).data, &mut fixedstore);
    if expensivefixed != 0 {
        let mut instart: size_t = *((*lz77).pos).offset(lstart as isize);
        let mut inend: size_t = instart
            .wrapping_add(ZopfliLZ77GetByteRange(lz77, lstart, lend));
        let mut s: ZopfliBlockState = ZopfliBlockState {
            options: 0 as *const ZopfliOptions,
            lmc: 0 as *mut ZopfliLongestMatchCache,
            blockstart: 0,
            blockend: 0,
        };
        ZopfliInitBlockState(options, instart, inend, 1 as libc::c_int, &mut s);
        ZopfliLZ77OptimalFixed(&mut s, (*lz77).data, instart, inend, &mut fixedstore);
        fixedcost = ZopfliCalculateBlockSize(
            &mut fixedstore,
            0 as libc::c_int as size_t,
            fixedstore.size,
            1 as libc::c_int,
        );
        ZopfliCleanBlockState(&mut s);
    }
    if uncompressedcost < fixedcost && uncompressedcost < dyncost {
        AddLZ77Block(
            options,
            0 as libc::c_int,
            final_0,
            lz77,
            lstart,
            lend,
            expected_data_size,
            bp,
            out,
            outsize,
        );
    } else if fixedcost < dyncost {
        if expensivefixed != 0 {
            AddLZ77Block(
                options,
                1 as libc::c_int,
                final_0,
                &mut fixedstore,
                0 as libc::c_int as size_t,
                fixedstore.size,
                expected_data_size,
                bp,
                out,
                outsize,
            );
        } else {
            AddLZ77Block(
                options,
                1 as libc::c_int,
                final_0,
                lz77,
                lstart,
                lend,
                expected_data_size,
                bp,
                out,
                outsize,
            );
        }
    } else {
        AddLZ77Block(
            options,
            2 as libc::c_int,
            final_0,
            lz77,
            lstart,
            lend,
            expected_data_size,
            bp,
            out,
            outsize,
        );
    }
    ZopfliCleanLZ77Store(&mut fixedstore);
}
pub unsafe extern "C" fn ZopfliDeflatePart(
    mut options: *const ZopfliOptions,
    mut btype: libc::c_int,
    mut final_0: libc::c_int,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut i: size_t = 0;
    let mut splitpoints_uncompressed: *mut size_t = 0 as *mut size_t;
    let mut npoints: size_t = 0 as libc::c_int as size_t;
    let mut splitpoints: *mut size_t = 0 as *mut size_t;
    let mut totalcost: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut lz77: ZopfliLZ77Store = ZopfliLZ77Store {
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
    if btype == 0 as libc::c_int {
        AddNonCompressedBlock(options, final_0, in_0, instart, inend, bp, out, outsize);
        return;
    } else if btype == 1 as libc::c_int {
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
        let mut s: ZopfliBlockState = ZopfliBlockState {
            options: 0 as *const ZopfliOptions,
            lmc: 0 as *mut ZopfliLongestMatchCache,
            blockstart: 0,
            blockend: 0,
        };
        ZopfliInitLZ77Store(in_0, &mut store);
        ZopfliInitBlockState(options, instart, inend, 1 as libc::c_int, &mut s);
        ZopfliLZ77OptimalFixed(&mut s, in_0, instart, inend, &mut store);
        AddLZ77Block(
            options,
            btype,
            final_0,
            &mut store,
            0 as libc::c_int as size_t,
            store.size,
            0 as libc::c_int as size_t,
            bp,
            out,
            outsize,
        );
        ZopfliCleanBlockState(&mut s);
        ZopfliCleanLZ77Store(&mut store);
        return;
    }
    if (*options).blocksplitting != 0 {
        ZopfliBlockSplit(
            options,
            in_0,
            instart,
            inend,
            (*options).blocksplittingmax as size_t,
            &mut splitpoints_uncompressed,
            &mut npoints,
        );
        splitpoints = malloc(
            (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(npoints),
        ) as *mut size_t;
    }
    ZopfliInitLZ77Store(in_0, &mut lz77);
    i = 0 as libc::c_int as size_t;
    while i <= npoints {
        let mut start: size_t = if i == 0 as libc::c_int as libc::c_ulong {
            instart
        } else {
            *splitpoints_uncompressed
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut end: size_t = if i == npoints {
            inend
        } else {
            *splitpoints_uncompressed.offset(i as isize)
        };
        let mut s_0: ZopfliBlockState = ZopfliBlockState {
            options: 0 as *const ZopfliOptions,
            lmc: 0 as *mut ZopfliLongestMatchCache,
            blockstart: 0,
            blockend: 0,
        };
        let mut store_0: ZopfliLZ77Store = ZopfliLZ77Store {
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
        ZopfliInitLZ77Store(in_0, &mut store_0);
        ZopfliInitBlockState(options, start, end, 1 as libc::c_int, &mut s_0);
        ZopfliLZ77Optimal(
            &mut s_0,
            in_0,
            start,
            end,
            (*options).numiterations,
            &mut store_0,
        );
        totalcost
            += ZopfliCalculateBlockSizeAutoType(
                &mut store_0,
                0 as libc::c_int as size_t,
                store_0.size,
            );
        ZopfliAppendLZ77Store(&mut store_0, &mut lz77);
        if i < npoints {
            *splitpoints.offset(i as isize) = lz77.size;
        }
        ZopfliCleanBlockState(&mut s_0);
        ZopfliCleanLZ77Store(&mut store_0);
        i = i.wrapping_add(1);
        i;
    }
    if (*options).blocksplitting != 0 && npoints > 1 as libc::c_int as libc::c_ulong {
        let mut splitpoints2: *mut size_t = 0 as *mut size_t;
        let mut npoints2: size_t = 0 as libc::c_int as size_t;
        let mut totalcost2: libc::c_double = 0 as libc::c_int as libc::c_double;
        ZopfliBlockSplitLZ77(
            options,
            &mut lz77,
            (*options).blocksplittingmax as size_t,
            &mut splitpoints2,
            &mut npoints2,
        );
        i = 0 as libc::c_int as size_t;
        while i <= npoints2 {
            let mut start_0: size_t = if i == 0 as libc::c_int as libc::c_ulong {
                0 as libc::c_int as libc::c_ulong
            } else {
                *splitpoints2
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            };
            let mut end_0: size_t = if i == npoints2 {
                lz77.size
            } else {
                *splitpoints2.offset(i as isize)
            };
            totalcost2 += ZopfliCalculateBlockSizeAutoType(&mut lz77, start_0, end_0);
            i = i.wrapping_add(1);
            i;
        }
        if totalcost2 < totalcost {
            free(splitpoints as *mut libc::c_void);
            splitpoints = splitpoints2;
            npoints = npoints2;
        } else {
            free(splitpoints2 as *mut libc::c_void);
        }
    }
    i = 0 as libc::c_int as size_t;
    while i <= npoints {
        let mut start_1: size_t = if i == 0 as libc::c_int as libc::c_ulong {
            0 as libc::c_int as libc::c_ulong
        } else {
            *splitpoints
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        };
        let mut end_1: size_t = if i == npoints {
            lz77.size
        } else {
            *splitpoints.offset(i as isize)
        };
        AddLZ77BlockAutoType(
            options,
            (i == npoints && final_0 != 0) as libc::c_int,
            &mut lz77,
            start_1,
            end_1,
            0 as libc::c_int as size_t,
            bp,
            out,
            outsize,
        );
        i = i.wrapping_add(1);
        i;
    }
    ZopfliCleanLZ77Store(&mut lz77);
    free(splitpoints as *mut libc::c_void);
    free(splitpoints_uncompressed as *mut libc::c_void);
}
pub unsafe extern "C" fn ZopfliDeflate(
    mut options: *const ZopfliOptions,
    mut btype: libc::c_int,
    mut final_0: libc::c_int,
    mut in_0: *const libc::c_uchar,
    mut insize: size_t,
    mut bp: *mut libc::c_uchar,
    mut out: *mut *mut libc::c_uchar,
    mut outsize: *mut size_t,
) {
    let mut offset: size_t = *outsize;
    let mut i: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut masterfinal: libc::c_int = (i
            .wrapping_add(1000000 as libc::c_int as libc::c_ulong) >= insize)
            as libc::c_int;
        let mut final2: libc::c_int = (final_0 != 0 && masterfinal != 0) as libc::c_int;
        let mut size: size_t = if masterfinal != 0 {
            insize.wrapping_sub(i)
        } else {
            1000000 as libc::c_int as libc::c_ulong
        };
        ZopfliDeflatePart(
            options,
            btype,
            final2,
            in_0,
            i,
            i.wrapping_add(size),
            bp,
            out,
            outsize,
        );
        i = (i as libc::c_ulong).wrapping_add(size) as size_t as size_t;
        if !(i < insize) {
            break;
        }
    }
    if (*options).verbose != 0 {
        fprintf(
            stderr,
            b"Original Size: %lu, Deflate: %lu, Compression: %f%% Removed\n\0"
                as *const u8 as *const libc::c_char,
            insize,
            (*outsize).wrapping_sub(offset),
            100.0f64
                * insize.wrapping_sub((*outsize).wrapping_sub(offset)) as libc::c_double
                / insize as libc::c_double,
        );
    }
}
