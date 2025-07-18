use ::libc;
extern "C" {
    static mut window: [uch; 0];
    static mut prev: [ush; 0];
    static mut rsync: libc::c_int;
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
    fn gzip_error(m: *const libc::c_char);
    fn flush_block(
        buf: *mut libc::c_char,
        stored_len: ulg,
        pad: libc::c_int,
        eof: libc::c_int,
    ) -> off_t;
    fn ct_tally(dist: libc::c_int, lc: libc::c_int) -> libc::c_int;
    static mut read_buf: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_uint) -> libc::c_int,
    >;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type voidp = *mut libc::c_void;
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type ulg = libc::c_ulong;
pub type Pos = ush;
pub type IPos = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub good_length: ush,
    pub max_lazy: ush,
    pub nice_length: ush,
    pub max_chain: ush,
}
static mut window_size: ulg = 0;
pub static mut block_start: libc::c_long = 0;
static mut ins_h: libc::c_uint = 0;
pub static mut prev_length: libc::c_uint = 0;
pub static mut strstart: libc::c_uint = 0;
pub static mut match_start: libc::c_uint = 0;
static mut eofile: libc::c_int = 0;
static mut lookahead: libc::c_uint = 0;
pub static mut max_chain_length: libc::c_uint = 0;
static mut max_lazy_match: libc::c_uint = 0;
pub static mut good_match: libc::c_uint = 0;
static mut rsync_sum: ulg = 0;
static mut rsync_chunk_end: ulg = 0;
static mut nice_match: libc::c_int = 0;
static mut configuration_table: [config; 10] = [
    {
        let mut init = config {
            good_length: 0 as libc::c_int as ush,
            max_lazy: 0 as libc::c_int as ush,
            nice_length: 0 as libc::c_int as ush,
            max_chain: 0 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as libc::c_int as ush,
            max_lazy: 4 as libc::c_int as ush,
            nice_length: 8 as libc::c_int as ush,
            max_chain: 4 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as libc::c_int as ush,
            max_lazy: 5 as libc::c_int as ush,
            nice_length: 16 as libc::c_int as ush,
            max_chain: 8 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as libc::c_int as ush,
            max_lazy: 6 as libc::c_int as ush,
            nice_length: 32 as libc::c_int as ush,
            max_chain: 32 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 4 as libc::c_int as ush,
            max_lazy: 4 as libc::c_int as ush,
            nice_length: 16 as libc::c_int as ush,
            max_chain: 16 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 8 as libc::c_int as ush,
            max_lazy: 16 as libc::c_int as ush,
            nice_length: 32 as libc::c_int as ush,
            max_chain: 32 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 8 as libc::c_int as ush,
            max_lazy: 16 as libc::c_int as ush,
            nice_length: 128 as libc::c_int as ush,
            max_chain: 128 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 8 as libc::c_int as ush,
            max_lazy: 32 as libc::c_int as ush,
            nice_length: 128 as libc::c_int as ush,
            max_chain: 256 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 32 as libc::c_int as ush,
            max_lazy: 128 as libc::c_int as ush,
            nice_length: 258 as libc::c_int as ush,
            max_chain: 1024 as libc::c_int as ush,
        };
        init
    },
    {
        let mut init = config {
            good_length: 32 as libc::c_int as ush,
            max_lazy: 258 as libc::c_int as ush,
            nice_length: 258 as libc::c_int as ush,
            max_chain: 4096 as libc::c_int as ush,
        };
        init
    },
];
unsafe extern "C" fn lm_init(mut pack_level: libc::c_int) {
    let mut j: libc::c_uint = 0;
    if pack_level < 1 as libc::c_int || pack_level > 9 as libc::c_int {
        gzip_error(b"bad pack level\0" as *const u8 as *const libc::c_char);
    }
    memset(
        prev.as_mut_ptr().offset(0x8000 as libc::c_int as isize) as *mut libc::c_char
            as voidp,
        0 as libc::c_int,
        (((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<ush>() as libc::c_ulong),
    );
    rsync_chunk_end = 0xffffffff as libc::c_ulong;
    rsync_sum = 0 as libc::c_int as ulg;
    max_lazy_match = configuration_table[pack_level as usize].max_lazy as libc::c_uint;
    good_match = configuration_table[pack_level as usize].good_length as libc::c_uint;
    nice_match = configuration_table[pack_level as usize].nice_length as libc::c_int;
    max_chain_length = configuration_table[pack_level as usize].max_chain
        as libc::c_uint;
    strstart = 0 as libc::c_int as libc::c_uint;
    block_start = 0 as libc::c_long;
    lookahead = read_buf
        .unwrap()(
        window.as_mut_ptr() as *mut libc::c_char,
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            <= 2 as libc::c_int as libc::c_ulong
        {
            0x8000 as libc::c_int as libc::c_uint
        } else {
            (2 as libc::c_int * 0x8000 as libc::c_int) as libc::c_uint
        },
    ) as libc::c_uint;
    if lookahead == 0 as libc::c_int as libc::c_uint
        || lookahead == -(1 as libc::c_int) as libc::c_uint
    {
        eofile = 1 as libc::c_int;
        lookahead = 0 as libc::c_int as libc::c_uint;
        return;
    }
    eofile = 0 as libc::c_int;
    while lookahead
        < (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int) as libc::c_uint
        && eofile == 0
    {
        fill_window();
    }
    ins_h = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while j < (3 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        ins_h = (ins_h
            << (15 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int)
                / 3 as libc::c_int
            ^ *window.as_mut_ptr().offset(j as isize) as libc::c_uint)
            & (((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        j = j.wrapping_add(1);
        j;
    }
}
unsafe extern "C" fn longest_match(mut cur_match: IPos) -> libc::c_int {
    let mut chain_length: libc::c_uint = max_chain_length;
    let mut scan: *mut uch = window.as_mut_ptr().offset(strstart as isize);
    let mut match_0: *mut uch = 0 as *mut uch;
    let mut len: libc::c_int = 0;
    let mut best_len: libc::c_int = prev_length as libc::c_int;
    let mut limit: IPos = if strstart
        > (0x8000 as libc::c_int
            - (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)) as IPos
    {
        strstart
            .wrapping_sub(
                (0x8000 as libc::c_int
                    - (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)) as IPos,
            )
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let mut strend: *mut uch = window
        .as_mut_ptr()
        .offset(strstart as isize)
        .offset(258 as libc::c_int as isize);
    let mut scan_end1: uch = *scan.offset((best_len - 1 as libc::c_int) as isize);
    let mut scan_end: uch = *scan.offset(best_len as isize);
    if prev_length >= good_match {
        chain_length >>= 2 as libc::c_int;
    }
    loop {
        match_0 = window.as_mut_ptr().offset(cur_match as isize);
        if !(*match_0.offset(best_len as isize) as libc::c_int != scan_end as libc::c_int
            || *match_0.offset((best_len - 1 as libc::c_int) as isize) as libc::c_int
                != scan_end1 as libc::c_int
            || *match_0 as libc::c_int != *scan as libc::c_int
            || {
                match_0 = match_0.offset(1);
                *match_0 as libc::c_int
                    != *scan.offset(1 as libc::c_int as isize) as libc::c_int
            })
        {
            scan = scan.offset(2 as libc::c_int as isize);
            match_0 = match_0.offset(1);
            match_0;
            loop {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                if !(*scan as libc::c_int == *match_0 as libc::c_int
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
                    } && scan < strend)
                {
                    break;
                }
            }
            len = 258 as libc::c_int
                - strend.offset_from(scan) as libc::c_long as libc::c_int;
            scan = strend.offset(-(258 as libc::c_int as isize));
            if len > best_len {
                match_start = cur_match;
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = *scan.offset((best_len - 1 as libc::c_int) as isize);
                scan_end = *scan.offset(best_len as isize);
            }
        }
        cur_match = *prev
            .as_mut_ptr()
            .offset(
                (cur_match & (0x8000 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                    as isize,
            ) as IPos;
        if !(cur_match > limit
            && {
                chain_length = chain_length.wrapping_sub(1);
                chain_length != 0 as libc::c_int as libc::c_uint
            })
        {
            break;
        }
    }
    return best_len;
}
unsafe extern "C" fn fill_window() {
    let mut n: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    let mut more: libc::c_uint = window_size
        .wrapping_sub(lookahead as ulg)
        .wrapping_sub(strstart as ulg) as libc::c_uint;
    if more == -(1 as libc::c_int) as libc::c_uint {
        more = more.wrapping_sub(1);
        more;
    } else if strstart
        >= (0x8000 as libc::c_int
            + (0x8000 as libc::c_int
                - (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)))
            as libc::c_uint
    {
        memcpy(
            window.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
            (window.as_mut_ptr() as *mut libc::c_char)
                .offset(0x8000 as libc::c_int as isize) as *const libc::c_void,
            0x8000 as libc::c_int as libc::c_uint as libc::c_ulong,
        );
        match_start = match_start.wrapping_sub(0x8000 as libc::c_int as libc::c_uint);
        strstart = strstart.wrapping_sub(0x8000 as libc::c_int as libc::c_uint);
        if rsync_chunk_end != 0xffffffff as libc::c_ulong {
            rsync_chunk_end = (rsync_chunk_end as libc::c_ulong)
                .wrapping_sub(0x8000 as libc::c_int as libc::c_ulong) as ulg as ulg;
        }
        block_start -= 0x8000 as libc::c_int as libc::c_long;
        n = 0 as libc::c_int as libc::c_uint;
        while n < ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint {
            m = *prev
                .as_mut_ptr()
                .offset(0x8000 as libc::c_int as isize)
                .offset(n as isize) as libc::c_uint;
            *prev
                .as_mut_ptr()
                .offset(0x8000 as libc::c_int as isize)
                .offset(
                    n as isize,
                ) = (if m >= 0x8000 as libc::c_int as libc::c_uint {
                m.wrapping_sub(0x8000 as libc::c_int as libc::c_uint)
            } else {
                0 as libc::c_int as libc::c_uint
            }) as Pos;
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as libc::c_int as libc::c_uint;
        while n < 0x8000 as libc::c_int as libc::c_uint {
            m = *prev.as_mut_ptr().offset(n as isize) as libc::c_uint;
            *prev
                .as_mut_ptr()
                .offset(
                    n as isize,
                ) = (if m >= 0x8000 as libc::c_int as libc::c_uint {
                m.wrapping_sub(0x8000 as libc::c_int as libc::c_uint)
            } else {
                0 as libc::c_int as libc::c_uint
            }) as Pos;
            n = n.wrapping_add(1);
            n;
        }
        more = more.wrapping_add(0x8000 as libc::c_int as libc::c_uint);
    }
    if eofile == 0 {
        n = read_buf
            .unwrap()(
            (window.as_mut_ptr() as *mut libc::c_char)
                .offset(strstart as isize)
                .offset(lookahead as isize),
            more,
        ) as libc::c_uint;
        if n == 0 as libc::c_int as libc::c_uint
            || n == -(1 as libc::c_int) as libc::c_uint
        {
            eofile = 1 as libc::c_int;
            memset(
                window.as_mut_ptr().offset(strstart as isize).offset(lookahead as isize)
                    as voidp,
                0 as libc::c_int,
                (3 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
        } else {
            lookahead = lookahead.wrapping_add(n);
        }
    }
}
unsafe extern "C" fn rsync_roll(mut start: libc::c_uint, mut num: libc::c_uint) {
    let mut i: libc::c_uint = 0;
    if start < 4096 as libc::c_int as libc::c_uint {
        i = start;
        while i < 4096 as libc::c_int as libc::c_uint {
            if i == start.wrapping_add(num) {
                return;
            }
            rsync_sum = (rsync_sum as libc::c_ulong)
                .wrapping_add(*window.as_mut_ptr().offset(i as isize) as ulg) as ulg
                as ulg;
            i = i.wrapping_add(1);
            i;
        }
        num = num
            .wrapping_sub((4096 as libc::c_int as libc::c_uint).wrapping_sub(start));
        start = 4096 as libc::c_int as libc::c_uint;
    }
    i = start;
    while i < start.wrapping_add(num) {
        rsync_sum = (rsync_sum as libc::c_ulong)
            .wrapping_add(*window.as_mut_ptr().offset(i as isize) as ulg) as ulg as ulg;
        rsync_sum = (rsync_sum as libc::c_ulong)
            .wrapping_sub(
                *window
                    .as_mut_ptr()
                    .offset(i.wrapping_sub(4096 as libc::c_int as libc::c_uint) as isize)
                    as ulg,
            ) as ulg as ulg;
        if rsync_chunk_end == 0xffffffff as libc::c_ulong
            && rsync_sum.wrapping_rem(4096 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
        {
            rsync_chunk_end = i as ulg;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn deflate_fast() -> off_t {
    let mut hash_head: IPos = 0;
    let mut flush: libc::c_int = 0 as libc::c_int;
    let mut match_length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    prev_length = (3 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    while lookahead != 0 as libc::c_int as libc::c_uint {
        ins_h = (ins_h
            << (15 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int)
                / 3 as libc::c_int
            ^ *window
                .as_mut_ptr()
                .offset(
                    strstart
                        .wrapping_add(3 as libc::c_int as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_uint)
            & (((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        hash_head = *prev
            .as_mut_ptr()
            .offset(0x8000 as libc::c_int as isize)
            .offset(ins_h as isize) as IPos;
        *prev
            .as_mut_ptr()
            .offset(
                (strstart & (0x8000 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                    as isize,
            ) = hash_head as ush;
        *prev
            .as_mut_ptr()
            .offset(0x8000 as libc::c_int as isize)
            .offset(ins_h as isize) = strstart as ush;
        if hash_head != 0 as libc::c_int as libc::c_uint
            && strstart.wrapping_sub(hash_head)
                <= (0x8000 as libc::c_int
                    - (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int))
                    as libc::c_uint
            && strstart as libc::c_ulong
                <= window_size
                    .wrapping_sub(
                        (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong,
                    )
        {
            match_length = longest_match(hash_head) as libc::c_uint;
            if match_length > lookahead {
                match_length = lookahead;
            }
        }
        if match_length >= 3 as libc::c_int as libc::c_uint {
            flush = ct_tally(
                strstart.wrapping_sub(match_start) as libc::c_int,
                match_length.wrapping_sub(3 as libc::c_int as libc::c_uint)
                    as libc::c_int,
            );
            lookahead = lookahead.wrapping_sub(match_length);
            if rsync != 0 {
                rsync_roll(strstart, match_length);
            }
            if match_length <= max_lazy_match {
                match_length = match_length.wrapping_sub(1);
                match_length;
                loop {
                    strstart = strstart.wrapping_add(1);
                    strstart;
                    ins_h = (ins_h
                        << (15 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int)
                            / 3 as libc::c_int
                        ^ *window
                            .as_mut_ptr()
                            .offset(
                                strstart
                                    .wrapping_add(3 as libc::c_int as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_uint)
                        & (((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint);
                    hash_head = *prev
                        .as_mut_ptr()
                        .offset(0x8000 as libc::c_int as isize)
                        .offset(ins_h as isize) as IPos;
                    *prev
                        .as_mut_ptr()
                        .offset(
                            (strstart
                                & (0x8000 as libc::c_int - 1 as libc::c_int)
                                    as libc::c_uint) as isize,
                        ) = hash_head as ush;
                    *prev
                        .as_mut_ptr()
                        .offset(0x8000 as libc::c_int as isize)
                        .offset(ins_h as isize) = strstart as ush;
                    match_length = match_length.wrapping_sub(1);
                    if !(match_length != 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
                strstart = strstart.wrapping_add(1);
                strstart;
            } else {
                strstart = strstart.wrapping_add(match_length);
                match_length = 0 as libc::c_int as libc::c_uint;
                ins_h = *window.as_mut_ptr().offset(strstart as isize) as libc::c_uint;
                ins_h = (ins_h
                    << (15 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int)
                        / 3 as libc::c_int
                    ^ *window
                        .as_mut_ptr()
                        .offset(
                            strstart.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) as libc::c_uint)
                    & (((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
            }
        } else {
            flush = ct_tally(
                0 as libc::c_int,
                *window.as_mut_ptr().offset(strstart as isize) as libc::c_int,
            );
            if rsync != 0 {
                rsync_roll(strstart, 1 as libc::c_int as libc::c_uint);
            }
            lookahead = lookahead.wrapping_sub(1);
            lookahead;
            strstart = strstart.wrapping_add(1);
            strstart;
        }
        if rsync != 0 && strstart as libc::c_ulong > rsync_chunk_end {
            rsync_chunk_end = 0xffffffff as libc::c_ulong;
            flush = 2 as libc::c_int;
        }
        if flush != 0 {
            flush_block(
                (if block_start >= 0 as libc::c_long {
                    &mut *window
                        .as_mut_ptr()
                        .offset(block_start as libc::c_uint as isize) as *mut uch
                        as *mut libc::c_char
                } else {
                    0 as *mut libc::c_void as *mut libc::c_char
                }),
                (strstart as libc::c_long - block_start) as ulg,
                flush - 1 as libc::c_int,
                0 as libc::c_int,
            );
            block_start = strstart as libc::c_long;
        }
        while lookahead
            < (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int) as libc::c_uint
            && eofile == 0
        {
            fill_window();
        }
    }
    return flush_block(
        if block_start >= 0 as libc::c_long {
            &mut *window.as_mut_ptr().offset(block_start as libc::c_uint as isize)
                as *mut uch as *mut libc::c_char
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        },
        (strstart as libc::c_long - block_start) as ulg,
        flush - 1 as libc::c_int,
        1 as libc::c_int,
    );
}
pub unsafe extern "C" fn deflate(mut pack_level: libc::c_int) -> off_t {
    let mut hash_head: IPos = 0;
    let mut prev_match: IPos = 0;
    let mut flush: libc::c_int = 0 as libc::c_int;
    let mut match_available: libc::c_int = 0 as libc::c_int;
    let mut match_length: libc::c_uint = (3 as libc::c_int - 1 as libc::c_int)
        as libc::c_uint;
    lm_init(pack_level);
    if pack_level <= 3 as libc::c_int {
        return deflate_fast();
    }
    while lookahead != 0 as libc::c_int as libc::c_uint {
        ins_h = (ins_h
            << (15 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int)
                / 3 as libc::c_int
            ^ *window
                .as_mut_ptr()
                .offset(
                    strstart
                        .wrapping_add(3 as libc::c_int as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                ) as libc::c_uint)
            & (((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        hash_head = *prev
            .as_mut_ptr()
            .offset(0x8000 as libc::c_int as isize)
            .offset(ins_h as isize) as IPos;
        *prev
            .as_mut_ptr()
            .offset(
                (strstart & (0x8000 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                    as isize,
            ) = hash_head as ush;
        *prev
            .as_mut_ptr()
            .offset(0x8000 as libc::c_int as isize)
            .offset(ins_h as isize) = strstart as ush;
        prev_length = match_length;
        prev_match = match_start;
        match_length = (3 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        if hash_head != 0 as libc::c_int as libc::c_uint && prev_length < max_lazy_match
            && strstart.wrapping_sub(hash_head)
                <= (0x8000 as libc::c_int
                    - (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int))
                    as libc::c_uint
            && strstart as libc::c_ulong
                <= window_size
                    .wrapping_sub(
                        (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int)
                            as libc::c_ulong,
                    )
        {
            match_length = longest_match(hash_head) as libc::c_uint;
            if match_length > lookahead {
                match_length = lookahead;
            }
            if match_length == 3 as libc::c_int as libc::c_uint
                && strstart.wrapping_sub(match_start)
                    > 4096 as libc::c_int as libc::c_uint
            {
                match_length = match_length.wrapping_sub(1);
                match_length;
            }
        }
        if prev_length >= 3 as libc::c_int as libc::c_uint && match_length <= prev_length
        {
            flush = ct_tally(
                strstart
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(prev_match) as libc::c_int,
                prev_length.wrapping_sub(3 as libc::c_int as libc::c_uint) as libc::c_int,
            );
            lookahead = lookahead
                .wrapping_sub(
                    prev_length.wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            prev_length = prev_length.wrapping_sub(2 as libc::c_int as libc::c_uint);
            if rsync != 0 {
                rsync_roll(
                    strstart,
                    prev_length.wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            }
            loop {
                strstart = strstart.wrapping_add(1);
                strstart;
                ins_h = (ins_h
                    << (15 as libc::c_int + 3 as libc::c_int - 1 as libc::c_int)
                        / 3 as libc::c_int
                    ^ *window
                        .as_mut_ptr()
                        .offset(
                            strstart
                                .wrapping_add(3 as libc::c_int as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_uint)
                    & (((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                hash_head = *prev
                    .as_mut_ptr()
                    .offset(0x8000 as libc::c_int as isize)
                    .offset(ins_h as isize) as IPos;
                *prev
                    .as_mut_ptr()
                    .offset(
                        (strstart
                            & (0x8000 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                            as isize,
                    ) = hash_head as ush;
                *prev
                    .as_mut_ptr()
                    .offset(0x8000 as libc::c_int as isize)
                    .offset(ins_h as isize) = strstart as ush;
                prev_length = prev_length.wrapping_sub(1);
                if !(prev_length != 0 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
            match_available = 0 as libc::c_int;
            match_length = (3 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
            strstart = strstart.wrapping_add(1);
            strstart;
            if rsync != 0 && strstart as libc::c_ulong > rsync_chunk_end {
                rsync_chunk_end = 0xffffffff as libc::c_ulong;
                flush = 2 as libc::c_int;
            }
            if flush != 0 {
                flush_block(
                    (if block_start >= 0 as libc::c_long {
                        &mut *window
                            .as_mut_ptr()
                            .offset(block_start as libc::c_uint as isize) as *mut uch
                            as *mut libc::c_char
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    }),
                    (strstart as libc::c_long - block_start) as ulg,
                    flush - 1 as libc::c_int,
                    0 as libc::c_int,
                );
                block_start = strstart as libc::c_long;
            }
        } else if match_available != 0 {
            flush = ct_tally(
                0 as libc::c_int,
                *window
                    .as_mut_ptr()
                    .offset(
                        strstart.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ) as libc::c_int,
            );
            if rsync != 0 && strstart as libc::c_ulong > rsync_chunk_end {
                rsync_chunk_end = 0xffffffff as libc::c_ulong;
                flush = 2 as libc::c_int;
            }
            if flush != 0 {
                flush_block(
                    (if block_start >= 0 as libc::c_long {
                        &mut *window
                            .as_mut_ptr()
                            .offset(block_start as libc::c_uint as isize) as *mut uch
                            as *mut libc::c_char
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    }),
                    (strstart as libc::c_long - block_start) as ulg,
                    flush - 1 as libc::c_int,
                    0 as libc::c_int,
                );
                block_start = strstart as libc::c_long;
            }
            if rsync != 0 {
                rsync_roll(strstart, 1 as libc::c_int as libc::c_uint);
            }
            strstart = strstart.wrapping_add(1);
            strstart;
            lookahead = lookahead.wrapping_sub(1);
            lookahead;
        } else {
            if rsync != 0 && strstart as libc::c_ulong > rsync_chunk_end {
                rsync_chunk_end = 0xffffffff as libc::c_ulong;
                flush = 2 as libc::c_int;
                flush_block(
                    (if block_start >= 0 as libc::c_long {
                        &mut *window
                            .as_mut_ptr()
                            .offset(block_start as libc::c_uint as isize) as *mut uch
                            as *mut libc::c_char
                    } else {
                        0 as *mut libc::c_void as *mut libc::c_char
                    }),
                    (strstart as libc::c_long - block_start) as ulg,
                    flush - 1 as libc::c_int,
                    0 as libc::c_int,
                );
                block_start = strstart as libc::c_long;
            }
            match_available = 1 as libc::c_int;
            if rsync != 0 {
                rsync_roll(strstart, 1 as libc::c_int as libc::c_uint);
            }
            strstart = strstart.wrapping_add(1);
            strstart;
            lookahead = lookahead.wrapping_sub(1);
            lookahead;
        }
        while lookahead
            < (258 as libc::c_int + 3 as libc::c_int + 1 as libc::c_int) as libc::c_uint
            && eofile == 0
        {
            fill_window();
        }
    }
    if match_available != 0 {
        ct_tally(
            0 as libc::c_int,
            *window
                .as_mut_ptr()
                .offset(strstart.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as libc::c_int,
        );
    }
    return flush_block(
        if block_start >= 0 as libc::c_long {
            &mut *window.as_mut_ptr().offset(block_start as libc::c_uint as isize)
                as *mut uch as *mut libc::c_char
        } else {
            0 as *mut libc::c_void as *mut libc::c_char
        },
        (strstart as libc::c_long - block_start) as ulg,
        flush - 1 as libc::c_int,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn run_static_initializers() {
    window_size = (2 as libc::c_int as ulg)
        .wrapping_mul(0x8000 as libc::c_int as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
