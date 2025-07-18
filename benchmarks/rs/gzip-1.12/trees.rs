use ::libc;
extern "C" {
    fn warning(m: *const libc::c_char);
    fn gzip_error(m: *const libc::c_char);
    fn bi_windup();
    fn send_bits(value: libc::c_int, length: libc::c_int);
    fn copy_block(buf: *mut libc::c_char, len: libc::c_uint, header: libc::c_int);
    static mut inbuf: [uch; 0];
    static mut d_buf: [ush; 0];
    static mut level: libc::c_int;
    fn bi_reverse(value: libc::c_uint, length: libc::c_int) -> libc::c_uint;
    static mut block_start: libc::c_long;
    static mut strstart: libc::c_uint;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type ulg = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub freq: ush,
    pub code: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data {
    pub fc: C2RustUnnamed,
    pub dl: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc {
    pub dyn_tree: *mut ct_data,
    pub static_tree: *mut ct_data,
    pub extra_bits: *mut libc::c_int,
    pub extra_base: libc::c_int,
    pub elems: libc::c_int,
    pub max_length: libc::c_int,
    pub max_code: libc::c_int,
}
static mut extra_lbits: [libc::c_int; 29] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    0 as libc::c_int,
];
static mut extra_dbits: [libc::c_int; 30] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    13 as libc::c_int,
];
static mut extra_blbits: [libc::c_int; 19] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    7 as libc::c_int,
];
static mut dyn_ltree: [ct_data; 573] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 573];
static mut dyn_dtree: [ct_data; 61] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 61];
static mut static_ltree: [ct_data; 288] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 288];
static mut static_dtree: [ct_data; 30] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 30];
static mut bl_tree: [ct_data; 39] = [ct_data {
    fc: C2RustUnnamed { freq: 0 },
    dl: C2RustUnnamed_0 { dad: 0 },
}; 39];
static mut l_desc: tree_desc = unsafe {
    {
        let mut init = tree_desc {
            dyn_tree: dyn_ltree.as_ptr() as *mut _,
            static_tree: static_ltree.as_ptr() as *mut _,
            extra_bits: extra_lbits.as_ptr() as *mut _,
            extra_base: 256 as libc::c_int + 1 as libc::c_int,
            elems: 256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int,
            max_length: 15 as libc::c_int,
            max_code: 0 as libc::c_int,
        };
        init
    }
};
static mut d_desc: tree_desc = unsafe {
    {
        let mut init = tree_desc {
            dyn_tree: dyn_dtree.as_ptr() as *mut _,
            static_tree: static_dtree.as_ptr() as *mut _,
            extra_bits: extra_dbits.as_ptr() as *mut _,
            extra_base: 0 as libc::c_int,
            elems: 30 as libc::c_int,
            max_length: 15 as libc::c_int,
            max_code: 0 as libc::c_int,
        };
        init
    }
};
static mut bl_desc: tree_desc = unsafe {
    {
        let mut init = tree_desc {
            dyn_tree: bl_tree.as_ptr() as *mut _,
            static_tree: 0 as *const ct_data as *mut ct_data,
            extra_bits: extra_blbits.as_ptr() as *mut _,
            extra_base: 0 as libc::c_int,
            elems: 19 as libc::c_int,
            max_length: 7 as libc::c_int,
            max_code: 0 as libc::c_int,
        };
        init
    }
};
static mut bl_count: [ush; 16] = [0; 16];
static mut bl_order: [uch; 19] = [
    16 as libc::c_int as uch,
    17 as libc::c_int as uch,
    18 as libc::c_int as uch,
    0 as libc::c_int as uch,
    8 as libc::c_int as uch,
    7 as libc::c_int as uch,
    9 as libc::c_int as uch,
    6 as libc::c_int as uch,
    10 as libc::c_int as uch,
    5 as libc::c_int as uch,
    11 as libc::c_int as uch,
    4 as libc::c_int as uch,
    12 as libc::c_int as uch,
    3 as libc::c_int as uch,
    13 as libc::c_int as uch,
    2 as libc::c_int as uch,
    14 as libc::c_int as uch,
    1 as libc::c_int as uch,
    15 as libc::c_int as uch,
];
static mut heap: [libc::c_int; 573] = [0; 573];
static mut heap_len: libc::c_int = 0;
static mut heap_max: libc::c_int = 0;
static mut depth: [uch; 573] = [0; 573];
static mut length_code: [uch; 256] = [0; 256];
static mut dist_code: [uch; 512] = [0; 512];
static mut base_length: [libc::c_int; 29] = [0; 29];
static mut base_dist: [libc::c_int; 30] = [0; 30];
static mut flag_buf: [uch; 4096] = [0; 4096];
static mut last_lit: libc::c_uint = 0;
static mut last_dist: libc::c_uint = 0;
static mut last_flags: libc::c_uint = 0;
static mut flags: uch = 0;
static mut flag_bit: uch = 0;
static mut opt_len: ulg = 0;
static mut static_len: ulg = 0;
static mut compressed_len: off_t = 0;
static mut input_len: off_t = 0;
static mut file_type: *mut ush = 0 as *const ush as *mut ush;
static mut file_method: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
pub unsafe extern "C" fn ct_init(mut attr: *mut ush, mut methodp: *mut libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    file_type = attr;
    file_method = methodp;
    input_len = 0 as libc::c_long;
    compressed_len = input_len;
    if static_dtree[0 as libc::c_int as usize].dl.len as libc::c_int != 0 as libc::c_int
    {
        return;
    }
    length = 0 as libc::c_int;
    code = 0 as libc::c_int;
    while code < 29 as libc::c_int - 1 as libc::c_int {
        base_length[code as usize] = length;
        n = 0 as libc::c_int;
        while n < (1 as libc::c_int) << extra_lbits[code as usize] {
            let fresh0 = length;
            length = length + 1;
            length_code[fresh0 as usize] = code as uch;
            n += 1;
            n;
        }
        code += 1;
        code;
    }
    length_code[(length - 1 as libc::c_int) as usize] = code as uch;
    dist = 0 as libc::c_int;
    code = 0 as libc::c_int;
    while code < 16 as libc::c_int {
        base_dist[code as usize] = dist;
        n = 0 as libc::c_int;
        while n < (1 as libc::c_int) << extra_dbits[code as usize] {
            let fresh1 = dist;
            dist = dist + 1;
            dist_code[fresh1 as usize] = code as uch;
            n += 1;
            n;
        }
        code += 1;
        code;
    }
    dist >>= 7 as libc::c_int;
    while code < 30 as libc::c_int {
        base_dist[code as usize] = dist << 7 as libc::c_int;
        n = 0 as libc::c_int;
        while n < (1 as libc::c_int) << extra_dbits[code as usize] - 7 as libc::c_int {
            let fresh2 = dist;
            dist = dist + 1;
            dist_code[(256 as libc::c_int + fresh2) as usize] = code as uch;
            n += 1;
            n;
        }
        code += 1;
        code;
    }
    bits = 0 as libc::c_int;
    while bits <= 15 as libc::c_int {
        bl_count[bits as usize] = 0 as libc::c_int as ush;
        bits += 1;
        bits;
    }
    n = 0 as libc::c_int;
    while n <= 143 as libc::c_int {
        let fresh3 = n;
        n = n + 1;
        static_ltree[fresh3 as usize].dl.len = 8 as libc::c_int as ush;
        bl_count[8 as libc::c_int
            as usize] = (bl_count[8 as libc::c_int as usize]).wrapping_add(1);
        bl_count[8 as libc::c_int as usize];
    }
    while n <= 255 as libc::c_int {
        let fresh4 = n;
        n = n + 1;
        static_ltree[fresh4 as usize].dl.len = 9 as libc::c_int as ush;
        bl_count[9 as libc::c_int
            as usize] = (bl_count[9 as libc::c_int as usize]).wrapping_add(1);
        bl_count[9 as libc::c_int as usize];
    }
    while n <= 279 as libc::c_int {
        let fresh5 = n;
        n = n + 1;
        static_ltree[fresh5 as usize].dl.len = 7 as libc::c_int as ush;
        bl_count[7 as libc::c_int
            as usize] = (bl_count[7 as libc::c_int as usize]).wrapping_add(1);
        bl_count[7 as libc::c_int as usize];
    }
    while n <= 287 as libc::c_int {
        let fresh6 = n;
        n = n + 1;
        static_ltree[fresh6 as usize].dl.len = 8 as libc::c_int as ush;
        bl_count[8 as libc::c_int
            as usize] = (bl_count[8 as libc::c_int as usize]).wrapping_add(1);
        bl_count[8 as libc::c_int as usize];
    }
    gen_codes(
        static_ltree.as_mut_ptr(),
        256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int + 1 as libc::c_int,
    );
    n = 0 as libc::c_int;
    while n < 30 as libc::c_int {
        static_dtree[n as usize].dl.len = 5 as libc::c_int as ush;
        static_dtree[n as usize]
            .fc
            .code = bi_reverse(n as libc::c_uint, 5 as libc::c_int) as ush;
        n += 1;
        n;
    }
    init_block();
}
unsafe extern "C" fn init_block() {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < 256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int {
        dyn_ltree[n as usize].fc.freq = 0 as libc::c_int as ush;
        n += 1;
        n;
    }
    n = 0 as libc::c_int;
    while n < 30 as libc::c_int {
        dyn_dtree[n as usize].fc.freq = 0 as libc::c_int as ush;
        n += 1;
        n;
    }
    n = 0 as libc::c_int;
    while n < 19 as libc::c_int {
        bl_tree[n as usize].fc.freq = 0 as libc::c_int as ush;
        n += 1;
        n;
    }
    dyn_ltree[256 as libc::c_int as usize].fc.freq = 1 as libc::c_int as ush;
    static_len = 0 as libc::c_long as ulg;
    opt_len = static_len;
    last_flags = 0 as libc::c_int as libc::c_uint;
    last_dist = last_flags;
    last_lit = last_dist;
    flags = 0 as libc::c_int as uch;
    flag_bit = 1 as libc::c_int as uch;
}
unsafe extern "C" fn pqdownheap(mut tree: *mut ct_data, mut k: libc::c_int) {
    let mut v: libc::c_int = heap[k as usize];
    let mut j: libc::c_int = k << 1 as libc::c_int;
    while j <= heap_len {
        if j < heap_len
            && (((*tree.offset(heap[(j + 1 as libc::c_int) as usize] as isize)).fc.freq
                as libc::c_int)
                < (*tree.offset(heap[j as usize] as isize)).fc.freq as libc::c_int
                || (*tree.offset(heap[(j + 1 as libc::c_int) as usize] as isize)).fc.freq
                    as libc::c_int
                    == (*tree.offset(heap[j as usize] as isize)).fc.freq as libc::c_int
                    && depth[heap[(j + 1 as libc::c_int) as usize] as usize]
                        as libc::c_int
                        <= depth[heap[j as usize] as usize] as libc::c_int)
        {
            j += 1;
            j;
        }
        if ((*tree.offset(v as isize)).fc.freq as libc::c_int)
            < (*tree.offset(heap[j as usize] as isize)).fc.freq as libc::c_int
            || (*tree.offset(v as isize)).fc.freq as libc::c_int
                == (*tree.offset(heap[j as usize] as isize)).fc.freq as libc::c_int
                && depth[v as usize] as libc::c_int
                    <= depth[heap[j as usize] as usize] as libc::c_int
        {
            break;
        }
        heap[k as usize] = heap[j as usize];
        k = j;
        j <<= 1 as libc::c_int;
    }
    heap[k as usize] = v;
}
unsafe extern "C" fn gen_bitlen(mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut extra: *mut libc::c_int = (*desc).extra_bits;
    let mut base: libc::c_int = (*desc).extra_base;
    let mut max_code: libc::c_int = (*desc).max_code;
    let mut max_length: libc::c_int = (*desc).max_length;
    let mut stree: *mut ct_data = (*desc).static_tree;
    let mut h: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut xbits: libc::c_int = 0;
    let mut f: ush = 0;
    let mut overflow: libc::c_int = 0 as libc::c_int;
    bits = 0 as libc::c_int;
    while bits <= 15 as libc::c_int {
        bl_count[bits as usize] = 0 as libc::c_int as ush;
        bits += 1;
        bits;
    }
    (*tree.offset(heap[heap_max as usize] as isize)).dl.len = 0 as libc::c_int as ush;
    h = heap_max + 1 as libc::c_int;
    while h
        < 2 as libc::c_int * (256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int)
            + 1 as libc::c_int
    {
        n = heap[h as usize];
        bits = (*tree.offset((*tree.offset(n as isize)).dl.dad as isize)).dl.len
            as libc::c_int + 1 as libc::c_int;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
            overflow;
        }
        (*tree.offset(n as isize)).dl.len = bits as ush;
        if !(n > max_code) {
            bl_count[bits as usize] = (bl_count[bits as usize]).wrapping_add(1);
            bl_count[bits as usize];
            xbits = 0 as libc::c_int;
            if n >= base {
                xbits = *extra.offset((n - base) as isize);
            }
            f = (*tree.offset(n as isize)).fc.freq;
            opt_len = (opt_len as libc::c_ulong)
                .wrapping_add((f as ulg).wrapping_mul((bits + xbits) as libc::c_ulong))
                as ulg as ulg;
            if !stree.is_null() {
                static_len = (static_len as libc::c_ulong)
                    .wrapping_add(
                        (f as ulg)
                            .wrapping_mul(
                                ((*stree.offset(n as isize)).dl.len as libc::c_int + xbits)
                                    as libc::c_ulong,
                            ),
                    ) as ulg as ulg;
            }
        }
        h += 1;
        h;
    }
    if overflow == 0 as libc::c_int {
        return;
    }
    loop {
        bits = max_length - 1 as libc::c_int;
        while bl_count[bits as usize] as libc::c_int == 0 as libc::c_int {
            bits -= 1;
            bits;
        }
        bl_count[bits as usize] = (bl_count[bits as usize]).wrapping_sub(1);
        bl_count[bits as usize];
        bl_count[(bits + 1 as libc::c_int)
            as usize] = (bl_count[(bits + 1 as libc::c_int) as usize] as libc::c_int
            + 2 as libc::c_int) as ush;
        bl_count[max_length as usize] = (bl_count[max_length as usize]).wrapping_sub(1);
        bl_count[max_length as usize];
        overflow -= 2 as libc::c_int;
        if !(overflow > 0 as libc::c_int) {
            break;
        }
    }
    bits = max_length;
    while bits != 0 as libc::c_int {
        n = bl_count[bits as usize] as libc::c_int;
        while n != 0 as libc::c_int {
            h -= 1;
            m = heap[h as usize];
            if m > max_code {
                continue;
            }
            if (*tree.offset(m as isize)).dl.len as libc::c_uint != bits as libc::c_uint
            {
                opt_len = (opt_len as libc::c_ulong)
                    .wrapping_add(
                        ((bits as libc::c_long
                            - (*tree.offset(m as isize)).dl.len as libc::c_long)
                            * (*tree.offset(m as isize)).fc.freq as libc::c_long)
                            as libc::c_ulong,
                    ) as ulg as ulg;
                (*tree.offset(m as isize)).dl.len = bits as ush;
            }
            n -= 1;
            n;
        }
        bits -= 1;
        bits;
    }
}
unsafe extern "C" fn gen_codes(mut tree: *mut ct_data, mut max_code: libc::c_int) {
    let mut next_code: [ush; 16] = [0; 16];
    let mut code: ush = 0 as libc::c_int as ush;
    let mut bits: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    bits = 1 as libc::c_int;
    while bits <= 15 as libc::c_int {
        code = ((code as libc::c_int
            + bl_count[(bits - 1 as libc::c_int) as usize] as libc::c_int)
            << 1 as libc::c_int) as ush;
        next_code[bits as usize] = code;
        bits += 1;
        bits;
    }
    n = 0 as libc::c_int;
    while n <= max_code {
        let mut len: libc::c_int = (*tree.offset(n as isize)).dl.len as libc::c_int;
        if !(len == 0 as libc::c_int) {
            let fresh7 = next_code[len as usize];
            next_code[len as usize] = (next_code[len as usize]).wrapping_add(1);
            (*tree.offset(n as isize))
                .fc
                .code = bi_reverse(fresh7 as libc::c_uint, len) as ush;
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn build_tree(mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut stree: *mut ct_data = (*desc).static_tree;
    let mut elems: libc::c_int = (*desc).elems;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut max_code: libc::c_int = -(1 as libc::c_int);
    let mut node: libc::c_int = elems;
    heap_len = 0 as libc::c_int;
    heap_max = 2 as libc::c_int
        * (256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int) + 1 as libc::c_int;
    n = 0 as libc::c_int;
    while n < elems {
        if (*tree.offset(n as isize)).fc.freq as libc::c_int != 0 as libc::c_int {
            max_code = n;
            heap_len += 1;
            heap[heap_len as usize] = max_code;
            depth[n as usize] = 0 as libc::c_int as uch;
        } else {
            (*tree.offset(n as isize)).dl.len = 0 as libc::c_int as ush;
        }
        n += 1;
        n;
    }
    while heap_len < 2 as libc::c_int {
        heap_len += 1;
        heap[heap_len
            as usize] = if max_code < 2 as libc::c_int {
            max_code += 1;
            max_code
        } else {
            0 as libc::c_int
        };
        let mut new: libc::c_int = heap[heap_len as usize];
        (*tree.offset(new as isize)).fc.freq = 1 as libc::c_int as ush;
        depth[new as usize] = 0 as libc::c_int as uch;
        opt_len = opt_len.wrapping_sub(1);
        opt_len;
        if !stree.is_null() {
            static_len = (static_len as libc::c_ulong)
                .wrapping_sub((*stree.offset(new as isize)).dl.len as libc::c_ulong)
                as ulg as ulg;
        }
    }
    (*desc).max_code = max_code;
    n = heap_len / 2 as libc::c_int;
    while n >= 1 as libc::c_int {
        pqdownheap(tree, n);
        n -= 1;
        n;
    }
    loop {
        n = heap[1 as libc::c_int as usize];
        let fresh8 = heap_len;
        heap_len = heap_len - 1;
        heap[1 as libc::c_int as usize] = heap[fresh8 as usize];
        pqdownheap(tree, 1 as libc::c_int);
        m = heap[1 as libc::c_int as usize];
        heap_max -= 1;
        heap[heap_max as usize] = n;
        heap_max -= 1;
        heap[heap_max as usize] = m;
        (*tree.offset(node as isize))
            .fc
            .freq = ((*tree.offset(n as isize)).fc.freq as libc::c_int
            + (*tree.offset(m as isize)).fc.freq as libc::c_int) as ush;
        depth[node
            as usize] = ((if depth[n as usize] as libc::c_int
            >= depth[m as usize] as libc::c_int
        {
            depth[n as usize] as libc::c_int
        } else {
            depth[m as usize] as libc::c_int
        }) + 1 as libc::c_int) as uch;
        let ref mut fresh9 = (*tree.offset(m as isize)).dl.dad;
        *fresh9 = node as ush;
        (*tree.offset(n as isize)).dl.dad = *fresh9;
        let fresh10 = node;
        node = node + 1;
        heap[1 as libc::c_int as usize] = fresh10;
        pqdownheap(tree, 1 as libc::c_int);
        if !(heap_len >= 2 as libc::c_int) {
            break;
        }
    }
    heap_max -= 1;
    heap[heap_max as usize] = heap[1 as libc::c_int as usize];
    gen_bitlen(desc);
    gen_codes(tree, max_code);
}
unsafe extern "C" fn scan_tree(mut tree: *mut ct_data, mut max_code: libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut prevlen: libc::c_int = -(1 as libc::c_int);
    let mut curlen: libc::c_int = 0;
    let mut nextlen: libc::c_int = (*tree.offset(0 as libc::c_int as isize)).dl.len
        as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut max_count: libc::c_int = 7 as libc::c_int;
    let mut min_count: libc::c_int = 4 as libc::c_int;
    if nextlen == 0 as libc::c_int {
        max_count = 138 as libc::c_int;
        min_count = 3 as libc::c_int;
    }
    (*tree.offset((max_code + 1 as libc::c_int) as isize))
        .dl
        .len = 0xffff as libc::c_int as ush;
    n = 0 as libc::c_int;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as libc::c_int) as isize)).dl.len as libc::c_int;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                bl_tree[curlen as usize]
                    .fc
                    .freq = (bl_tree[curlen as usize].fc.freq as libc::c_int + count)
                    as ush;
            } else if curlen != 0 as libc::c_int {
                if curlen != prevlen {
                    bl_tree[curlen as usize]
                        .fc
                        .freq = (bl_tree[curlen as usize].fc.freq).wrapping_add(1);
                    bl_tree[curlen as usize].fc.freq;
                }
                bl_tree[16 as libc::c_int as usize]
                    .fc
                    .freq = (bl_tree[16 as libc::c_int as usize].fc.freq)
                    .wrapping_add(1);
                bl_tree[16 as libc::c_int as usize].fc.freq;
            } else if count <= 10 as libc::c_int {
                bl_tree[17 as libc::c_int as usize]
                    .fc
                    .freq = (bl_tree[17 as libc::c_int as usize].fc.freq)
                    .wrapping_add(1);
                bl_tree[17 as libc::c_int as usize].fc.freq;
            } else {
                bl_tree[18 as libc::c_int as usize]
                    .fc
                    .freq = (bl_tree[18 as libc::c_int as usize].fc.freq)
                    .wrapping_add(1);
                bl_tree[18 as libc::c_int as usize].fc.freq;
            }
            count = 0 as libc::c_int;
            prevlen = curlen;
            if nextlen == 0 as libc::c_int {
                max_count = 138 as libc::c_int;
                min_count = 3 as libc::c_int;
            } else if curlen == nextlen {
                max_count = 6 as libc::c_int;
                min_count = 3 as libc::c_int;
            } else {
                max_count = 7 as libc::c_int;
                min_count = 4 as libc::c_int;
            }
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn send_tree(mut tree: *mut ct_data, mut max_code: libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut prevlen: libc::c_int = -(1 as libc::c_int);
    let mut curlen: libc::c_int = 0;
    let mut nextlen: libc::c_int = (*tree.offset(0 as libc::c_int as isize)).dl.len
        as libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut max_count: libc::c_int = 7 as libc::c_int;
    let mut min_count: libc::c_int = 4 as libc::c_int;
    if nextlen == 0 as libc::c_int {
        max_count = 138 as libc::c_int;
        min_count = 3 as libc::c_int;
    }
    n = 0 as libc::c_int;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as libc::c_int) as isize)).dl.len as libc::c_int;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                loop {
                    send_bits(
                        bl_tree[curlen as usize].fc.code as libc::c_int,
                        bl_tree[curlen as usize].dl.len as libc::c_int,
                    );
                    count -= 1;
                    if !(count != 0 as libc::c_int) {
                        break;
                    }
                }
            } else if curlen != 0 as libc::c_int {
                if curlen != prevlen {
                    send_bits(
                        bl_tree[curlen as usize].fc.code as libc::c_int,
                        bl_tree[curlen as usize].dl.len as libc::c_int,
                    );
                    count -= 1;
                    count;
                }
                send_bits(
                    bl_tree[16 as libc::c_int as usize].fc.code as libc::c_int,
                    bl_tree[16 as libc::c_int as usize].dl.len as libc::c_int,
                );
                send_bits(count - 3 as libc::c_int, 2 as libc::c_int);
            } else if count <= 10 as libc::c_int {
                send_bits(
                    bl_tree[17 as libc::c_int as usize].fc.code as libc::c_int,
                    bl_tree[17 as libc::c_int as usize].dl.len as libc::c_int,
                );
                send_bits(count - 3 as libc::c_int, 3 as libc::c_int);
            } else {
                send_bits(
                    bl_tree[18 as libc::c_int as usize].fc.code as libc::c_int,
                    bl_tree[18 as libc::c_int as usize].dl.len as libc::c_int,
                );
                send_bits(count - 11 as libc::c_int, 7 as libc::c_int);
            }
            count = 0 as libc::c_int;
            prevlen = curlen;
            if nextlen == 0 as libc::c_int {
                max_count = 138 as libc::c_int;
                min_count = 3 as libc::c_int;
            } else if curlen == nextlen {
                max_count = 6 as libc::c_int;
                min_count = 3 as libc::c_int;
            } else {
                max_count = 7 as libc::c_int;
                min_count = 4 as libc::c_int;
            }
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn build_bl_tree() -> libc::c_int {
    let mut max_blindex: libc::c_int = 0;
    scan_tree(dyn_ltree.as_mut_ptr(), l_desc.max_code);
    scan_tree(dyn_dtree.as_mut_ptr(), d_desc.max_code);
    build_tree(&mut bl_desc as *mut tree_desc);
    max_blindex = 19 as libc::c_int - 1 as libc::c_int;
    while max_blindex >= 3 as libc::c_int {
        if bl_tree[bl_order[max_blindex as usize] as usize].dl.len as libc::c_int
            != 0 as libc::c_int
        {
            break;
        }
        max_blindex -= 1;
        max_blindex;
    }
    opt_len = (opt_len as libc::c_ulong)
        .wrapping_add(
            (3 as libc::c_int * (max_blindex + 1 as libc::c_int) + 5 as libc::c_int
                + 5 as libc::c_int + 4 as libc::c_int) as libc::c_ulong,
        ) as ulg as ulg;
    return max_blindex;
}
unsafe extern "C" fn send_all_trees(
    mut lcodes: libc::c_int,
    mut dcodes: libc::c_int,
    mut blcodes: libc::c_int,
) {
    let mut rank: libc::c_int = 0;
    send_bits(lcodes - 257 as libc::c_int, 5 as libc::c_int);
    send_bits(dcodes - 1 as libc::c_int, 5 as libc::c_int);
    send_bits(blcodes - 4 as libc::c_int, 4 as libc::c_int);
    rank = 0 as libc::c_int;
    while rank < blcodes {
        send_bits(
            bl_tree[bl_order[rank as usize] as usize].dl.len as libc::c_int,
            3 as libc::c_int,
        );
        rank += 1;
        rank;
    }
    send_tree(dyn_ltree.as_mut_ptr(), lcodes - 1 as libc::c_int);
    send_tree(dyn_dtree.as_mut_ptr(), dcodes - 1 as libc::c_int);
}
pub unsafe extern "C" fn flush_block(
    mut buf: *mut libc::c_char,
    mut stored_len: ulg,
    mut pad: libc::c_int,
    mut eof: libc::c_int,
) -> off_t {
    let mut opt_lenb: ulg = 0;
    let mut static_lenb: ulg = 0;
    let mut max_blindex: libc::c_int = 0;
    flag_buf[last_flags as usize] = flags;
    if *file_type as libc::c_int == 0xffff as libc::c_int as ush as libc::c_int {
        set_file_type();
    }
    build_tree(&mut l_desc as *mut tree_desc);
    build_tree(&mut d_desc as *mut tree_desc);
    max_blindex = build_bl_tree();
    opt_lenb = opt_len
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_add(7 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int;
    static_lenb = static_len
        .wrapping_add(3 as libc::c_int as libc::c_ulong)
        .wrapping_add(7 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int;
    input_len = (input_len as libc::c_ulong).wrapping_add(stored_len) as off_t as off_t;
    if static_lenb <= opt_lenb {
        opt_lenb = static_lenb;
    }
    if stored_len <= opt_lenb && eof != 0 && compressed_len == 0 as libc::c_long
        && 0 as libc::c_int != 0
    {
        if buf.is_null() {
            gzip_error(b"block vanished\0" as *const u8 as *const libc::c_char);
        }
        copy_block(buf, stored_len as libc::c_uint, 0 as libc::c_int);
        compressed_len = (stored_len << 3 as libc::c_int) as off_t;
        *file_method = 0 as libc::c_int;
    } else if stored_len.wrapping_add(4 as libc::c_int as libc::c_ulong) <= opt_lenb
        && !buf.is_null()
    {
        send_bits(((0 as libc::c_int) << 1 as libc::c_int) + eof, 3 as libc::c_int);
        compressed_len = compressed_len + 3 as libc::c_int as libc::c_long
            + 7 as libc::c_int as libc::c_long & !(7 as libc::c_long);
        compressed_len = (compressed_len as libc::c_ulong)
            .wrapping_add(
                stored_len.wrapping_add(4 as libc::c_int as libc::c_ulong)
                    << 3 as libc::c_int,
            ) as off_t as off_t;
        copy_block(buf, stored_len as libc::c_uint, 1 as libc::c_int);
    } else if static_lenb == opt_lenb {
        send_bits(((1 as libc::c_int) << 1 as libc::c_int) + eof, 3 as libc::c_int);
        compress_block(static_ltree.as_mut_ptr(), static_dtree.as_mut_ptr());
        compressed_len = (compressed_len as libc::c_ulong)
            .wrapping_add((3 as libc::c_int as libc::c_ulong).wrapping_add(static_len))
            as off_t as off_t;
    } else {
        send_bits(((2 as libc::c_int) << 1 as libc::c_int) + eof, 3 as libc::c_int);
        send_all_trees(
            l_desc.max_code + 1 as libc::c_int,
            d_desc.max_code + 1 as libc::c_int,
            max_blindex + 1 as libc::c_int,
        );
        compress_block(dyn_ltree.as_mut_ptr(), dyn_dtree.as_mut_ptr());
        compressed_len = (compressed_len as libc::c_ulong)
            .wrapping_add((3 as libc::c_int as libc::c_ulong).wrapping_add(opt_len))
            as off_t as off_t;
    }
    init_block();
    if eof != 0 {
        bi_windup();
        compressed_len += 7 as libc::c_int as libc::c_long;
    } else if pad != 0
        && compressed_len % 8 as libc::c_int as libc::c_long
            != 0 as libc::c_int as libc::c_long
    {
        send_bits(((0 as libc::c_int) << 1 as libc::c_int) + eof, 3 as libc::c_int);
        compressed_len = compressed_len + 3 as libc::c_int as libc::c_long
            + 7 as libc::c_int as libc::c_long & !(7 as libc::c_long);
        copy_block(buf, 0 as libc::c_int as libc::c_uint, 1 as libc::c_int);
    }
    return compressed_len >> 3 as libc::c_int;
}
pub unsafe extern "C" fn ct_tally(
    mut dist: libc::c_int,
    mut lc: libc::c_int,
) -> libc::c_int {
    let fresh11 = last_lit;
    last_lit = last_lit.wrapping_add(1);
    *inbuf.as_mut_ptr().offset(fresh11 as isize) = lc as uch;
    if dist == 0 as libc::c_int {
        dyn_ltree[lc as usize]
            .fc
            .freq = (dyn_ltree[lc as usize].fc.freq).wrapping_add(1);
        dyn_ltree[lc as usize].fc.freq;
    } else {
        dist -= 1;
        dist;
        dyn_ltree[(length_code[lc as usize] as libc::c_int + 256 as libc::c_int
                + 1 as libc::c_int) as usize]
            .fc
            .freq = (dyn_ltree[(length_code[lc as usize] as libc::c_int
                + 256 as libc::c_int + 1 as libc::c_int) as usize]
            .fc
            .freq)
            .wrapping_add(1);
        dyn_ltree[(length_code[lc as usize] as libc::c_int + 256 as libc::c_int
                + 1 as libc::c_int) as usize]
            .fc
            .freq;
        dyn_dtree[(if dist < 256 as libc::c_int {
                dist_code[dist as usize] as libc::c_int
            } else {
                dist_code[(256 as libc::c_int + (dist >> 7 as libc::c_int)) as usize]
                    as libc::c_int
            }) as usize]
            .fc
            .freq = (dyn_dtree[(if dist < 256 as libc::c_int {
                dist_code[dist as usize] as libc::c_int
            } else {
                dist_code[(256 as libc::c_int + (dist >> 7 as libc::c_int)) as usize]
                    as libc::c_int
            }) as usize]
            .fc
            .freq)
            .wrapping_add(1);
        dyn_dtree[(if dist < 256 as libc::c_int {
                dist_code[dist as usize] as libc::c_int
            } else {
                dist_code[(256 as libc::c_int + (dist >> 7 as libc::c_int)) as usize]
                    as libc::c_int
            }) as usize]
            .fc
            .freq;
        let fresh12 = last_dist;
        last_dist = last_dist.wrapping_add(1);
        *d_buf.as_mut_ptr().offset(fresh12 as isize) = dist as ush;
        flags = (flags as libc::c_int | flag_bit as libc::c_int) as uch;
    }
    flag_bit = ((flag_bit as libc::c_int) << 1 as libc::c_int) as uch;
    if last_lit & 7 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        let fresh13 = last_flags;
        last_flags = last_flags.wrapping_add(1);
        flag_buf[fresh13 as usize] = flags;
        flags = 0 as libc::c_int as uch;
        flag_bit = 1 as libc::c_int as uch;
    }
    if level > 2 as libc::c_int
        && last_lit & 0xfff as libc::c_int as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
    {
        let mut out_length: ulg = (last_lit as ulg)
            .wrapping_mul(8 as libc::c_long as libc::c_ulong);
        let mut in_length: ulg = (strstart as ulg)
            .wrapping_sub(block_start as libc::c_ulong);
        let mut dcode: libc::c_int = 0;
        dcode = 0 as libc::c_int;
        while dcode < 30 as libc::c_int {
            out_length = (out_length as libc::c_ulong)
                .wrapping_add(
                    (dyn_dtree[dcode as usize].fc.freq as ulg)
                        .wrapping_mul(
                            (5 as libc::c_long
                                + extra_dbits[dcode as usize] as libc::c_long)
                                as libc::c_ulong,
                        ),
                ) as ulg as ulg;
            dcode += 1;
            dcode;
        }
        out_length >>= 3 as libc::c_int;
        if last_dist < last_lit.wrapping_div(2 as libc::c_int as libc::c_uint)
            && out_length < in_length.wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            return 1 as libc::c_int;
        }
    }
    return (last_lit == (0x8000 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        || last_dist == 0x8000 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn compress_block(mut ltree: *mut ct_data, mut dtree: *mut ct_data) {
    let mut dist: libc::c_uint = 0;
    let mut lc: libc::c_int = 0;
    let mut lx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut fx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut flag: uch = 0 as libc::c_int as uch;
    let mut code: libc::c_uint = 0;
    let mut extra: libc::c_int = 0;
    if last_lit != 0 as libc::c_int as libc::c_uint {
        loop {
            if lx & 7 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let fresh14 = fx;
                fx = fx.wrapping_add(1);
                flag = flag_buf[fresh14 as usize];
            }
            let fresh15 = lx;
            lx = lx.wrapping_add(1);
            lc = *inbuf.as_mut_ptr().offset(fresh15 as isize) as libc::c_int;
            if flag as libc::c_int & 1 as libc::c_int == 0 as libc::c_int {
                send_bits(
                    (*ltree.offset(lc as isize)).fc.code as libc::c_int,
                    (*ltree.offset(lc as isize)).dl.len as libc::c_int,
                );
            } else {
                code = length_code[lc as usize] as libc::c_uint;
                send_bits(
                    (*ltree
                        .offset(
                            code
                                .wrapping_add(256 as libc::c_int as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ))
                        .fc
                        .code as libc::c_int,
                    (*ltree
                        .offset(
                            code
                                .wrapping_add(256 as libc::c_int as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ))
                        .dl
                        .len as libc::c_int,
                );
                extra = extra_lbits[code as usize];
                if extra != 0 as libc::c_int {
                    lc -= base_length[code as usize];
                    send_bits(lc, extra);
                }
                let fresh16 = dx;
                dx = dx.wrapping_add(1);
                dist = *d_buf.as_mut_ptr().offset(fresh16 as isize) as libc::c_uint;
                code = (if dist < 256 as libc::c_int as libc::c_uint {
                    dist_code[dist as usize] as libc::c_int
                } else {
                    dist_code[(256 as libc::c_int as libc::c_uint)
                        .wrapping_add(dist >> 7 as libc::c_int) as usize] as libc::c_int
                }) as libc::c_uint;
                send_bits(
                    (*dtree.offset(code as isize)).fc.code as libc::c_int,
                    (*dtree.offset(code as isize)).dl.len as libc::c_int,
                );
                extra = extra_dbits[code as usize];
                if extra != 0 as libc::c_int {
                    dist = dist.wrapping_sub(base_dist[code as usize] as libc::c_uint);
                    send_bits(dist as libc::c_int, extra);
                }
            }
            flag = (flag as libc::c_int >> 1 as libc::c_int) as uch;
            if !(lx < last_lit) {
                break;
            }
        }
    }
    send_bits(
        (*ltree.offset(256 as libc::c_int as isize)).fc.code as libc::c_int,
        (*ltree.offset(256 as libc::c_int as isize)).dl.len as libc::c_int,
    );
}
unsafe extern "C" fn set_file_type() {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut ascii_freq: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut bin_freq: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while n < 7 as libc::c_int {
        let fresh17 = n;
        n = n + 1;
        bin_freq = bin_freq
            .wrapping_add(dyn_ltree[fresh17 as usize].fc.freq as libc::c_uint);
    }
    while n < 128 as libc::c_int {
        let fresh18 = n;
        n = n + 1;
        ascii_freq = ascii_freq
            .wrapping_add(dyn_ltree[fresh18 as usize].fc.freq as libc::c_uint);
    }
    while n < 256 as libc::c_int {
        let fresh19 = n;
        n = n + 1;
        bin_freq = bin_freq
            .wrapping_add(dyn_ltree[fresh19 as usize].fc.freq as libc::c_uint);
    }
    *file_type = (if bin_freq > ascii_freq >> 2 as libc::c_int {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as ush;
    if *file_type as libc::c_int == 0 as libc::c_int && 0 as libc::c_int != 0 {
        warning(b"-l used on binary file\0" as *const u8 as *const libc::c_char);
    }
}
