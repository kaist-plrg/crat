use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    fn ZopfliLZ77Greedy(
        s: *mut ZopfliBlockState,
        in_0: *const libc::c_uchar,
        instart: size_t,
        inend: size_t,
        store: *mut ZopfliLZ77Store,
        h: *mut ZopfliHash,
    );
    fn ZopfliVerifyLenDist(
        data: *const libc::c_uchar,
        datasize: size_t,
        pos: size_t,
        dist: libc::c_ushort,
        length: libc::c_ushort,
    );
    fn ZopfliFindLongestMatch(
        s: *mut ZopfliBlockState,
        h: *const ZopfliHash,
        array: *const libc::c_uchar,
        pos: size_t,
        size: size_t,
        limit: size_t,
        sublen: *mut libc::c_ushort,
        distance: *mut libc::c_ushort,
        length: *mut libc::c_ushort,
    );
    fn ZopfliStoreLitLenDist(
        length: libc::c_ushort,
        dist: libc::c_ushort,
        pos: size_t,
        store: *mut ZopfliLZ77Store,
    );
    fn ZopfliCopyLZ77Store(source: *const ZopfliLZ77Store, dest: *mut ZopfliLZ77Store);
    fn ZopfliCleanLZ77Store(store: *mut ZopfliLZ77Store);
    fn ZopfliInitLZ77Store(data: *const libc::c_uchar, store: *mut ZopfliLZ77Store);
    fn ZopfliAllocHash(window_size: size_t, h: *mut ZopfliHash);
    fn ZopfliResetHash(window_size: size_t, h: *mut ZopfliHash);
    fn ZopfliCleanHash(h: *mut ZopfliHash);
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ZopfliCalculateBlockSize(
        lz77: *const ZopfliLZ77Store,
        lstart: size_t,
        lend: size_t,
        btype: libc::c_int,
    ) -> libc::c_double;
    fn ZopfliGetDistExtraBits(dist: libc::c_int) -> libc::c_int;
    fn ZopfliGetDistSymbol(dist: libc::c_int) -> libc::c_int;
    fn ZopfliGetLengthExtraBits(l: libc::c_int) -> libc::c_int;
    fn ZopfliGetLengthSymbol(l: libc::c_int) -> libc::c_int;
    fn ZopfliCalculateEntropy(
        count: *const size_t,
        n: size_t,
        bitlengths: *mut libc::c_double,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolStats {
    pub litlens: [size_t; 288],
    pub dists: [size_t; 32],
    pub ll_symbols: [libc::c_double; 288],
    pub d_symbols: [libc::c_double; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RanState {
    pub m_w: libc::c_uint,
    pub m_z: libc::c_uint,
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
pub type CostModelFun = unsafe extern "C" fn(
    libc::c_uint,
    libc::c_uint,
    *mut libc::c_void,
) -> libc::c_double;
unsafe extern "C" fn InitStats(mut stats: *mut SymbolStats) {
    memset(
        ((*stats).litlens).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (288 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    memset(
        ((*stats).dists).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    memset(
        ((*stats).ll_symbols).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (288 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memset(
        ((*stats).d_symbols).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
}
unsafe extern "C" fn CopyStats(
    mut source: *mut SymbolStats,
    mut dest: *mut SymbolStats,
) {
    memcpy(
        ((*dest).litlens).as_mut_ptr() as *mut libc::c_void,
        ((*source).litlens).as_mut_ptr() as *const libc::c_void,
        (288 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    memcpy(
        ((*dest).dists).as_mut_ptr() as *mut libc::c_void,
        ((*source).dists).as_mut_ptr() as *const libc::c_void,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    memcpy(
        ((*dest).ll_symbols).as_mut_ptr() as *mut libc::c_void,
        ((*source).ll_symbols).as_mut_ptr() as *const libc::c_void,
        (288 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        ((*dest).d_symbols).as_mut_ptr() as *mut libc::c_void,
        ((*source).d_symbols).as_mut_ptr() as *const libc::c_void,
        (32 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
}
unsafe extern "C" fn AddWeighedStatFreqs(
    mut stats1: *const SymbolStats,
    mut w1: libc::c_double,
    mut stats2: *const SymbolStats,
    mut w2: libc::c_double,
    mut result: *mut SymbolStats,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        (*result)
            .litlens[i
            as usize] = ((*stats1).litlens[i as usize] as libc::c_double * w1
            + (*stats2).litlens[i as usize] as libc::c_double * w2) as size_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*result)
            .dists[i
            as usize] = ((*stats1).dists[i as usize] as libc::c_double * w1
            + (*stats2).dists[i as usize] as libc::c_double * w2) as size_t;
        i = i.wrapping_add(1);
        i;
    }
    (*result).litlens[256 as libc::c_int as usize] = 1 as libc::c_int as size_t;
}
unsafe extern "C" fn InitRanState(mut state: *mut RanState) {
    (*state).m_w = 1 as libc::c_int as libc::c_uint;
    (*state).m_z = 2 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn Ran(mut state: *mut RanState) -> libc::c_uint {
    (*state)
        .m_z = (36969 as libc::c_int as libc::c_uint)
        .wrapping_mul((*state).m_z & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add((*state).m_z >> 16 as libc::c_int);
    (*state)
        .m_w = (18000 as libc::c_int as libc::c_uint)
        .wrapping_mul((*state).m_w & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add((*state).m_w >> 16 as libc::c_int);
    return ((*state).m_z << 16 as libc::c_int).wrapping_add((*state).m_w);
}
unsafe extern "C" fn RandomizeFreqs(
    mut state: *mut RanState,
    mut freqs: *mut size_t,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if (Ran(state) >> 4 as libc::c_int)
            .wrapping_rem(3 as libc::c_int as libc::c_uint)
            == 0 as libc::c_int as libc::c_uint
        {
            *freqs
                .offset(
                    i as isize,
                ) = *freqs.offset((Ran(state)).wrapping_rem(n as libc::c_uint) as isize);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn RandomizeStatFreqs(
    mut state: *mut RanState,
    mut stats: *mut SymbolStats,
) {
    RandomizeFreqs(state, ((*stats).litlens).as_mut_ptr(), 288 as libc::c_int);
    RandomizeFreqs(state, ((*stats).dists).as_mut_ptr(), 32 as libc::c_int);
    (*stats).litlens[256 as libc::c_int as usize] = 1 as libc::c_int as size_t;
}
unsafe extern "C" fn ClearStatFreqs(mut stats: *mut SymbolStats) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 288 as libc::c_int as libc::c_ulong {
        (*stats).litlens[i as usize] = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 32 as libc::c_int as libc::c_ulong {
        (*stats).dists[i as usize] = 0 as libc::c_int as size_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn GetCostFixed(
    mut litlen: libc::c_uint,
    mut dist: libc::c_uint,
    mut unused: *mut libc::c_void,
) -> libc::c_double {
    if dist == 0 as libc::c_int as libc::c_uint {
        if litlen <= 143 as libc::c_int as libc::c_uint {
            return 8 as libc::c_int as libc::c_double
        } else {
            return 9 as libc::c_int as libc::c_double
        }
    } else {
        let mut dbits: libc::c_int = ZopfliGetDistExtraBits(dist as libc::c_int);
        let mut lbits: libc::c_int = ZopfliGetLengthExtraBits(litlen as libc::c_int);
        let mut lsym: libc::c_int = ZopfliGetLengthSymbol(litlen as libc::c_int);
        let mut cost: libc::c_int = 0 as libc::c_int;
        if lsym <= 279 as libc::c_int {
            cost += 7 as libc::c_int;
        } else {
            cost += 8 as libc::c_int;
        }
        cost += 5 as libc::c_int;
        return (cost + dbits + lbits) as libc::c_double;
    };
}
unsafe extern "C" fn GetCostStat(
    mut litlen: libc::c_uint,
    mut dist: libc::c_uint,
    mut context: *mut libc::c_void,
) -> libc::c_double {
    let mut stats: *mut SymbolStats = context as *mut SymbolStats;
    if dist == 0 as libc::c_int as libc::c_uint {
        return (*stats).ll_symbols[litlen as usize]
    } else {
        let mut lsym: libc::c_int = ZopfliGetLengthSymbol(litlen as libc::c_int);
        let mut lbits: libc::c_int = ZopfliGetLengthExtraBits(litlen as libc::c_int);
        let mut dsym: libc::c_int = ZopfliGetDistSymbol(dist as libc::c_int);
        let mut dbits: libc::c_int = ZopfliGetDistExtraBits(dist as libc::c_int);
        return (lbits + dbits) as libc::c_double + (*stats).ll_symbols[lsym as usize]
            + (*stats).d_symbols[dsym as usize];
    };
}
unsafe extern "C" fn GetCostModelMinCost(
    mut costmodel: Option::<CostModelFun>,
    mut costcontext: *mut libc::c_void,
) -> libc::c_double {
    let mut mincost: libc::c_double = 0.;
    let mut bestlength: libc::c_int = 0 as libc::c_int;
    let mut bestdist: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    static mut dsymbols: [libc::c_int; 30] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
        13 as libc::c_int,
        17 as libc::c_int,
        25 as libc::c_int,
        33 as libc::c_int,
        49 as libc::c_int,
        65 as libc::c_int,
        97 as libc::c_int,
        129 as libc::c_int,
        193 as libc::c_int,
        257 as libc::c_int,
        385 as libc::c_int,
        513 as libc::c_int,
        769 as libc::c_int,
        1025 as libc::c_int,
        1537 as libc::c_int,
        2049 as libc::c_int,
        3073 as libc::c_int,
        4097 as libc::c_int,
        6145 as libc::c_int,
        8193 as libc::c_int,
        12289 as libc::c_int,
        16385 as libc::c_int,
        24577 as libc::c_int,
    ];
    mincost = 1e30f64;
    i = 3 as libc::c_int;
    while i < 259 as libc::c_int {
        let mut c: libc::c_double = costmodel
            .unwrap()(i as libc::c_uint, 1 as libc::c_int as libc::c_uint, costcontext);
        if c < mincost {
            bestlength = i;
            mincost = c;
        }
        i += 1;
        i;
    }
    mincost = 1e30f64;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        let mut c_0: libc::c_double = costmodel
            .unwrap()(
            3 as libc::c_int as libc::c_uint,
            dsymbols[i as usize] as libc::c_uint,
            costcontext,
        );
        if c_0 < mincost {
            bestdist = dsymbols[i as usize];
            mincost = c_0;
        }
        i += 1;
        i;
    }
    return costmodel
        .unwrap()(bestlength as libc::c_uint, bestdist as libc::c_uint, costcontext);
}
unsafe extern "C" fn zopfli_min(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
unsafe extern "C" fn GetBestLengths(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut costmodel: Option::<CostModelFun>,
    mut costcontext: *mut libc::c_void,
    mut length_array: *mut libc::c_ushort,
    mut h: *mut ZopfliHash,
    mut costs: *mut libc::c_float,
) -> libc::c_double {
    let mut blocksize: size_t = inend.wrapping_sub(instart);
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 0;
    let mut kend: size_t = 0;
    let mut leng: libc::c_ushort = 0;
    let mut dist: libc::c_ushort = 0;
    let mut sublen: [libc::c_ushort; 259] = [0; 259];
    let mut windowstart: size_t = if instart > 32768 as libc::c_int as libc::c_ulong {
        instart.wrapping_sub(32768 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut result: libc::c_double = 0.;
    let mut mincost: libc::c_double = GetCostModelMinCost(costmodel, costcontext);
    let mut mincostaddcostj: libc::c_double = 0.;
    if instart == inend {
        return 0 as libc::c_int as libc::c_double;
    }
    ZopfliResetHash(32768 as libc::c_int as size_t, h);
    ZopfliWarmupHash(in_0, windowstart, inend, h);
    i = windowstart;
    while i < instart {
        ZopfliUpdateHash(in_0, i, inend, h);
        i = i.wrapping_add(1);
        i;
    }
    i = 1 as libc::c_int as size_t;
    while i < blocksize.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        *costs.offset(i as isize) = 1e30f64 as libc::c_float;
        i = i.wrapping_add(1);
        i;
    }
    *costs.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
    *length_array.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_ushort;
    i = instart;
    while i < inend {
        let mut j: size_t = i.wrapping_sub(instart);
        ZopfliUpdateHash(in_0, i, inend, h);
        if *((*h).same)
            .offset(
                (i & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong) as isize,
            ) as libc::c_int > 258 as libc::c_int * 2 as libc::c_int
            && i
                > instart
                    .wrapping_add(258 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            && i
                .wrapping_add((258 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) < inend
            && *((*h).same)
                .offset(
                    (i.wrapping_sub(258 as libc::c_int as libc::c_ulong)
                        & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                        as isize,
                ) as libc::c_int > 258 as libc::c_int
        {
            let mut symbolcost: libc::c_double = costmodel
                .unwrap()(
                258 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
                costcontext,
            );
            k = 0 as libc::c_int as size_t;
            while k < 258 as libc::c_int as libc::c_ulong {
                *costs
                    .offset(
                        j.wrapping_add(258 as libc::c_int as libc::c_ulong) as isize,
                    ) = (*costs.offset(j as isize) as libc::c_double + symbolcost)
                    as libc::c_float;
                *length_array
                    .offset(
                        j.wrapping_add(258 as libc::c_int as libc::c_ulong) as isize,
                    ) = 258 as libc::c_int as libc::c_ushort;
                i = i.wrapping_add(1);
                i;
                j = j.wrapping_add(1);
                j;
                ZopfliUpdateHash(in_0, i, inend, h);
                k = k.wrapping_add(1);
                k;
            }
        }
        ZopfliFindLongestMatch(
            s,
            h,
            in_0,
            i,
            inend,
            258 as libc::c_int as size_t,
            sublen.as_mut_ptr(),
            &mut dist,
            &mut leng,
        );
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong) <= inend {
            let mut newCost: libc::c_double = costmodel
                .unwrap()(
                *in_0.offset(i as isize) as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
                costcontext,
            ) + *costs.offset(j as isize) as libc::c_double;
            if newCost >= 0 as libc::c_int as libc::c_double {} else {
                __assert_fail(
                    b"newCost >= 0\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                    279 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 146],
                        &[libc::c_char; 146],
                    >(
                        b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_3413: {
                if newCost >= 0 as libc::c_int as libc::c_double {} else {
                    __assert_fail(
                        b"newCost >= 0\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/squeeze.c\0" as *const u8
                            as *const libc::c_char,
                        279 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 146],
                            &[libc::c_char; 146],
                        >(
                            b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if newCost
                < *costs
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_double
            {
                *costs
                    .offset(
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = newCost as libc::c_float;
                *length_array
                    .offset(
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = 1 as libc::c_int as libc::c_ushort;
            }
        }
        kend = zopfli_min(leng as size_t, inend.wrapping_sub(i));
        mincostaddcostj = mincost + *costs.offset(j as isize) as libc::c_double;
        k = 3 as libc::c_int as size_t;
        while k <= kend {
            let mut newCost_0: libc::c_double = 0.;
            if !(*costs.offset(j.wrapping_add(k) as isize) as libc::c_double
                <= mincostaddcostj)
            {
                newCost_0 = costmodel
                    .unwrap()(
                    k as libc::c_uint,
                    sublen[k as usize] as libc::c_uint,
                    costcontext,
                ) + *costs.offset(j as isize) as libc::c_double;
                if newCost_0 >= 0 as libc::c_int as libc::c_double {} else {
                    __assert_fail(
                        b"newCost >= 0\0" as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/squeeze.c\0" as *const u8
                            as *const libc::c_char,
                        296 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 146],
                            &[libc::c_char; 146],
                        >(
                            b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3043: {
                    if newCost_0 >= 0 as libc::c_int as libc::c_double {} else {
                        __assert_fail(
                            b"newCost >= 0\0" as *const u8 as *const libc::c_char,
                            b"zopfli/src/zopfli/squeeze.c\0" as *const u8
                                as *const libc::c_char,
                            296 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 146],
                                &[libc::c_char; 146],
                            >(
                                b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if newCost_0
                    < *costs.offset(j.wrapping_add(k) as isize) as libc::c_double
                {
                    if k <= 258 as libc::c_int as libc::c_ulong {} else {
                        __assert_fail(
                            b"k <= ZOPFLI_MAX_MATCH\0" as *const u8
                                as *const libc::c_char,
                            b"zopfli/src/zopfli/squeeze.c\0" as *const u8
                                as *const libc::c_char,
                            298 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 146],
                                &[libc::c_char; 146],
                            >(
                                b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2993: {
                        if k <= 258 as libc::c_int as libc::c_ulong {} else {
                            __assert_fail(
                                b"k <= ZOPFLI_MAX_MATCH\0" as *const u8
                                    as *const libc::c_char,
                                b"zopfli/src/zopfli/squeeze.c\0" as *const u8
                                    as *const libc::c_char,
                                298 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 146],
                                    &[libc::c_char; 146],
                                >(
                                    b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    *costs
                        .offset(j.wrapping_add(k) as isize) = newCost_0 as libc::c_float;
                    *length_array
                        .offset(j.wrapping_add(k) as isize) = k as libc::c_ushort;
                }
            }
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    if *costs.offset(blocksize as isize) >= 0 as libc::c_int as libc::c_float {} else {
        __assert_fail(
            b"costs[blocksize] >= 0\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
            305 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 146],
                &[libc::c_char; 146],
            >(
                b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2876: {
        if *costs.offset(blocksize as isize) >= 0 as libc::c_int as libc::c_float
        {} else {
            __assert_fail(
                b"costs[blocksize] >= 0\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                305 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 146],
                    &[libc::c_char; 146],
                >(
                    b"double GetBestLengths(ZopfliBlockState *, const unsigned char *, size_t, size_t, CostModelFun *, void *, unsigned short *, ZopfliHash *, float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    result = *costs.offset(blocksize as isize) as libc::c_double;
    return result;
}
unsafe extern "C" fn TraceBackwards(
    mut size: size_t,
    mut length_array: *const libc::c_ushort,
    mut path: *mut *mut libc::c_ushort,
    mut pathsize: *mut size_t,
) {
    let mut index: size_t = size;
    if size == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    loop {
        if *pathsize & (*pathsize).wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0 {
            *path = (if *pathsize == 0 as libc::c_int as libc::c_ulong {
                malloc(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            } else {
                realloc(
                    *path as *mut libc::c_void,
                    (*pathsize)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                        ),
                )
            }) as *mut libc::c_ushort;
        }
        *(*path).offset(*pathsize as isize) = *length_array.offset(index as isize);
        *pathsize = (*pathsize).wrapping_add(1);
        *pathsize;
        if *length_array.offset(index as isize) as libc::c_ulong <= index {} else {
            __assert_fail(
                b"length_array[index] <= index\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                323 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4460: {
            if *length_array.offset(index as isize) as libc::c_ulong <= index {} else {
                __assert_fail(
                    b"length_array[index] <= index\0" as *const u8
                        as *const libc::c_char,
                    b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                    323 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 81],
                        &[libc::c_char; 81],
                    >(
                        b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if *length_array.offset(index as isize) as libc::c_int <= 258 as libc::c_int
        {} else {
            __assert_fail(
                b"length_array[index] <= ZOPFLI_MAX_MATCH\0" as *const u8
                    as *const libc::c_char,
                b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                324 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4413: {
            if *length_array.offset(index as isize) as libc::c_int <= 258 as libc::c_int
            {} else {
                __assert_fail(
                    b"length_array[index] <= ZOPFLI_MAX_MATCH\0" as *const u8
                        as *const libc::c_char,
                    b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                    324 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 81],
                        &[libc::c_char; 81],
                    >(
                        b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if *length_array.offset(index as isize) as libc::c_int != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"length_array[index] != 0\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                325 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4365: {
            if *length_array.offset(index as isize) as libc::c_int != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"length_array[index] != 0\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                    325 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 81],
                        &[libc::c_char; 81],
                    >(
                        b"void TraceBackwards(size_t, const unsigned short *, unsigned short **, size_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        index = (index as libc::c_ulong)
            .wrapping_sub(*length_array.offset(index as isize) as libc::c_ulong)
            as size_t as size_t;
        if index == 0 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    index = 0 as libc::c_int as size_t;
    while index < (*pathsize).wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut temp: libc::c_ushort = *(*path).offset(index as isize);
        *(*path)
            .offset(
                index as isize,
            ) = *(*path)
            .offset(
                (*pathsize)
                    .wrapping_sub(index)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        *(*path)
            .offset(
                (*pathsize)
                    .wrapping_sub(index)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = temp;
        index = index.wrapping_add(1);
        index;
    }
}
unsafe extern "C" fn FollowPath(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut path: *mut libc::c_ushort,
    mut pathsize: size_t,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut windowstart: size_t = if instart > 32768 as libc::c_int as libc::c_ulong {
        instart.wrapping_sub(32768 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut total_length_test: size_t = 0 as libc::c_int as size_t;
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
    pos = instart;
    i = 0 as libc::c_int as size_t;
    while i < pathsize {
        let mut length: libc::c_ushort = *path.offset(i as isize);
        let mut dummy_length: libc::c_ushort = 0;
        let mut dist: libc::c_ushort = 0;
        if pos < inend {} else {
            __assert_fail(
                b"pos < inend\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 134],
                    &[libc::c_char; 134],
                >(
                    b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4121: {
            if pos < inend {} else {
                __assert_fail(
                    b"pos < inend\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                    361 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 134],
                        &[libc::c_char; 134],
                    >(
                        b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        ZopfliUpdateHash(in_0, pos, inend, h);
        if length as libc::c_int >= 3 as libc::c_int {
            ZopfliFindLongestMatch(
                s,
                h,
                in_0,
                pos,
                inend,
                length as size_t,
                0 as *mut libc::c_ushort,
                &mut dist,
                &mut dummy_length,
            );
            if !(dummy_length as libc::c_int != length as libc::c_int
                && length as libc::c_int > 2 as libc::c_int
                && dummy_length as libc::c_int > 2 as libc::c_int)
            {} else {
                __assert_fail(
                    b"!(dummy_length != length && length > 2 && dummy_length > 2)\0"
                        as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                    371 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 134],
                        &[libc::c_char; 134],
                    >(
                        b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_4010: {
                if !(dummy_length as libc::c_int != length as libc::c_int
                    && length as libc::c_int > 2 as libc::c_int
                    && dummy_length as libc::c_int > 2 as libc::c_int)
                {} else {
                    __assert_fail(
                        b"!(dummy_length != length && length > 2 && dummy_length > 2)\0"
                            as *const u8 as *const libc::c_char,
                        b"zopfli/src/zopfli/squeeze.c\0" as *const u8
                            as *const libc::c_char,
                        371 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 134],
                            &[libc::c_char; 134],
                        >(
                            b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            ZopfliVerifyLenDist(in_0, inend, pos, dist, length);
            ZopfliStoreLitLenDist(length, dist, pos, store);
            total_length_test = (total_length_test as libc::c_ulong)
                .wrapping_add(length as libc::c_ulong) as size_t as size_t;
        } else {
            length = 1 as libc::c_int as libc::c_ushort;
            ZopfliStoreLitLenDist(
                *in_0.offset(pos as isize) as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                pos,
                store,
            );
            total_length_test = total_length_test.wrapping_add(1);
            total_length_test;
        }
        if pos.wrapping_add(length as libc::c_ulong) <= inend {} else {
            __assert_fail(
                b"pos + length <= inend\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                382 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 134],
                    &[libc::c_char; 134],
                >(
                    b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_3895: {
            if pos.wrapping_add(length as libc::c_ulong) <= inend {} else {
                __assert_fail(
                    b"pos + length <= inend\0" as *const u8 as *const libc::c_char,
                    b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                    382 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 134],
                        &[libc::c_char; 134],
                    >(
                        b"void FollowPath(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short *, size_t, ZopfliLZ77Store *, ZopfliHash *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        j = 1 as libc::c_int as size_t;
        while j < length as libc::c_ulong {
            ZopfliUpdateHash(in_0, pos.wrapping_add(j), inend, h);
            j = j.wrapping_add(1);
            j;
        }
        pos = (pos as libc::c_ulong).wrapping_add(length as libc::c_ulong) as size_t
            as size_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn CalculateStatistics(mut stats: *mut SymbolStats) {
    ZopfliCalculateEntropy(
        ((*stats).litlens).as_mut_ptr(),
        288 as libc::c_int as size_t,
        ((*stats).ll_symbols).as_mut_ptr(),
    );
    ZopfliCalculateEntropy(
        ((*stats).dists).as_mut_ptr(),
        32 as libc::c_int as size_t,
        ((*stats).d_symbols).as_mut_ptr(),
    );
}
unsafe extern "C" fn GetStatistics(
    mut store: *const ZopfliLZ77Store,
    mut stats: *mut SymbolStats,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*store).size {
        if *((*store).dists).offset(i as isize) as libc::c_int == 0 as libc::c_int {
            (*stats)
                .litlens[*((*store).litlens).offset(i as isize)
                as usize] = ((*stats)
                .litlens[*((*store).litlens).offset(i as isize) as usize])
                .wrapping_add(1);
            (*stats).litlens[*((*store).litlens).offset(i as isize) as usize];
        } else {
            (*stats)
                .litlens[ZopfliGetLengthSymbol(
                *((*store).litlens).offset(i as isize) as libc::c_int,
            )
                as usize] = ((*stats)
                .litlens[ZopfliGetLengthSymbol(
                *((*store).litlens).offset(i as isize) as libc::c_int,
            ) as usize])
                .wrapping_add(1);
            (*stats)
                .litlens[ZopfliGetLengthSymbol(
                *((*store).litlens).offset(i as isize) as libc::c_int,
            ) as usize];
            (*stats)
                .dists[ZopfliGetDistSymbol(
                *((*store).dists).offset(i as isize) as libc::c_int,
            )
                as usize] = ((*stats)
                .dists[ZopfliGetDistSymbol(
                *((*store).dists).offset(i as isize) as libc::c_int,
            ) as usize])
                .wrapping_add(1);
            (*stats)
                .dists[ZopfliGetDistSymbol(
                *((*store).dists).offset(i as isize) as libc::c_int,
            ) as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    (*stats).litlens[256 as libc::c_int as usize] = 1 as libc::c_int as size_t;
    CalculateStatistics(stats);
}
unsafe extern "C" fn LZ77OptimalRun(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut path: *mut *mut libc::c_ushort,
    mut pathsize: *mut size_t,
    mut length_array: *mut libc::c_ushort,
    mut costmodel: Option::<CostModelFun>,
    mut costcontext: *mut libc::c_void,
    mut store: *mut ZopfliLZ77Store,
    mut h: *mut ZopfliHash,
    mut costs: *mut libc::c_float,
) -> libc::c_double {
    let mut cost: libc::c_double = GetBestLengths(
        s,
        in_0,
        instart,
        inend,
        costmodel,
        costcontext,
        length_array,
        h,
        costs,
    );
    free(*path as *mut libc::c_void);
    *path = 0 as *mut libc::c_ushort;
    *pathsize = 0 as libc::c_int as size_t;
    TraceBackwards(inend.wrapping_sub(instart), length_array, path, pathsize);
    FollowPath(s, in_0, instart, inend, *path, *pathsize, store, h);
    if cost < 1e30f64 {} else {
        __assert_fail(
            b"cost < ZOPFLI_LARGE_FLOAT\0" as *const u8 as *const libc::c_char,
            b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
            442 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 194],
                &[libc::c_char; 194],
            >(
                b"double LZ77OptimalRun(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short **, size_t *, unsigned short *, CostModelFun *, void *, ZopfliLZ77Store *, ZopfliHash *, float *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3754: {
        if cost < 1e30f64 {} else {
            __assert_fail(
                b"cost < ZOPFLI_LARGE_FLOAT\0" as *const u8 as *const libc::c_char,
                b"zopfli/src/zopfli/squeeze.c\0" as *const u8 as *const libc::c_char,
                442 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 194],
                    &[libc::c_char; 194],
                >(
                    b"double LZ77OptimalRun(ZopfliBlockState *, const unsigned char *, size_t, size_t, unsigned short **, size_t *, unsigned short *, CostModelFun *, void *, ZopfliLZ77Store *, ZopfliHash *, float *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return cost;
}
pub unsafe extern "C" fn ZopfliLZ77Optimal(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut numiterations: libc::c_int,
    mut store: *mut ZopfliLZ77Store,
) {
    let mut blocksize: size_t = inend.wrapping_sub(instart);
    let mut length_array: *mut libc::c_ushort = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul(blocksize.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_ushort;
    let mut path: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut pathsize: size_t = 0 as libc::c_int as size_t;
    let mut currentstore: ZopfliLZ77Store = ZopfliLZ77Store {
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
    let mut stats: SymbolStats = SymbolStats {
        litlens: [0; 288],
        dists: [0; 32],
        ll_symbols: [0.; 288],
        d_symbols: [0.; 32],
    };
    let mut beststats: SymbolStats = SymbolStats {
        litlens: [0; 288],
        dists: [0; 32],
        ll_symbols: [0.; 288],
        d_symbols: [0.; 32],
    };
    let mut laststats: SymbolStats = SymbolStats {
        litlens: [0; 288],
        dists: [0; 32],
        ll_symbols: [0.; 288],
        d_symbols: [0.; 32],
    };
    let mut i: libc::c_int = 0;
    let mut costs: *mut libc::c_float = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(blocksize.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_float;
    let mut cost: libc::c_double = 0.;
    let mut bestcost: libc::c_double = 1e30f64;
    let mut lastcost: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ran_state: RanState = RanState { m_w: 0, m_z: 0 };
    let mut lastrandomstep: libc::c_int = -(1 as libc::c_int);
    if costs.is_null() {
        exit(-(1 as libc::c_int));
    }
    if length_array.is_null() {
        exit(-(1 as libc::c_int));
    }
    InitRanState(&mut ran_state);
    InitStats(&mut stats);
    ZopfliInitLZ77Store(in_0, &mut currentstore);
    ZopfliAllocHash(32768 as libc::c_int as size_t, h);
    ZopfliLZ77Greedy(s, in_0, instart, inend, &mut currentstore, h);
    GetStatistics(&mut currentstore, &mut stats);
    i = 0 as libc::c_int;
    while i < numiterations {
        ZopfliCleanLZ77Store(&mut currentstore);
        ZopfliInitLZ77Store(in_0, &mut currentstore);
        LZ77OptimalRun(
            s,
            in_0,
            instart,
            inend,
            &mut path,
            &mut pathsize,
            length_array,
            Some(
                GetCostStat
                    as unsafe extern "C" fn(
                        libc::c_uint,
                        libc::c_uint,
                        *mut libc::c_void,
                    ) -> libc::c_double,
            ),
            &mut stats as *mut SymbolStats as *mut libc::c_void,
            &mut currentstore,
            h,
            costs,
        );
        cost = ZopfliCalculateBlockSize(
            &mut currentstore,
            0 as libc::c_int as size_t,
            currentstore.size,
            2 as libc::c_int,
        );
        if (*(*s).options).verbose_more != 0
            || (*(*s).options).verbose != 0 && cost < bestcost
        {
            fprintf(
                stderr,
                b"Iteration %d: %d bit\n\0" as *const u8 as *const libc::c_char,
                i,
                cost as libc::c_int,
            );
        }
        if cost < bestcost {
            ZopfliCopyLZ77Store(&mut currentstore, store);
            CopyStats(&mut stats, &mut beststats);
            bestcost = cost;
        }
        CopyStats(&mut stats, &mut laststats);
        ClearStatFreqs(&mut stats);
        GetStatistics(&mut currentstore, &mut stats);
        if lastrandomstep != -(1 as libc::c_int) {
            AddWeighedStatFreqs(&mut stats, 1.0f64, &mut laststats, 0.5f64, &mut stats);
            CalculateStatistics(&mut stats);
        }
        if i > 5 as libc::c_int && cost == lastcost {
            CopyStats(&mut beststats, &mut stats);
            RandomizeStatFreqs(&mut ran_state, &mut stats);
            CalculateStatistics(&mut stats);
            lastrandomstep = i;
        }
        lastcost = cost;
        i += 1;
        i;
    }
    free(length_array as *mut libc::c_void);
    free(path as *mut libc::c_void);
    free(costs as *mut libc::c_void);
    ZopfliCleanLZ77Store(&mut currentstore);
    ZopfliCleanHash(h);
}
pub unsafe extern "C" fn ZopfliLZ77OptimalFixed(
    mut s: *mut ZopfliBlockState,
    mut in_0: *const libc::c_uchar,
    mut instart: size_t,
    mut inend: size_t,
    mut store: *mut ZopfliLZ77Store,
) {
    let mut blocksize: size_t = inend.wrapping_sub(instart);
    let mut length_array: *mut libc::c_ushort = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul(blocksize.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_ushort;
    let mut path: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut pathsize: size_t = 0 as libc::c_int as size_t;
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
    let mut costs: *mut libc::c_float = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(blocksize.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_float;
    if costs.is_null() {
        exit(-(1 as libc::c_int));
    }
    if length_array.is_null() {
        exit(-(1 as libc::c_int));
    }
    ZopfliAllocHash(32768 as libc::c_int as size_t, h);
    (*s).blockstart = instart;
    (*s).blockend = inend;
    LZ77OptimalRun(
        s,
        in_0,
        instart,
        inend,
        &mut path,
        &mut pathsize,
        length_array,
        Some(
            GetCostFixed
                as unsafe extern "C" fn(
                    libc::c_uint,
                    libc::c_uint,
                    *mut libc::c_void,
                ) -> libc::c_double,
        ),
        0 as *mut libc::c_void,
        store,
        h,
        costs,
    );
    free(length_array as *mut libc::c_void);
    free(path as *mut libc::c_void);
    free(costs as *mut libc::c_void);
    ZopfliCleanHash(h);
}
