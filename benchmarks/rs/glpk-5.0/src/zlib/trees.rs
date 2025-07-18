use ::libc;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type charf = libc::c_char;
pub type intf = libc::c_int;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub strm: z_streamp,
    pub status: libc::c_int,
    pub pending_buf: *mut Bytef,
    pub pending_buf_size: ulg,
    pub pending_out: *mut Bytef,
    pub pending: uInt,
    pub wrap: libc::c_int,
    pub gzhead: gz_headerp,
    pub gzindex: uInt,
    pub method: Byte,
    pub last_flush: libc::c_int,
    pub w_size: uInt,
    pub w_bits: uInt,
    pub w_mask: uInt,
    pub window: *mut Bytef,
    pub window_size: ulg,
    pub prev: *mut Posf,
    pub head: *mut Posf,
    pub ins_h: uInt,
    pub hash_size: uInt,
    pub hash_bits: uInt,
    pub hash_mask: uInt,
    pub hash_shift: uInt,
    pub block_start: libc::c_long,
    pub match_length: uInt,
    pub prev_match: IPos,
    pub match_available: libc::c_int,
    pub strstart: uInt,
    pub match_start: uInt,
    pub lookahead: uInt,
    pub prev_length: uInt,
    pub max_chain_length: uInt,
    pub max_lazy_match: uInt,
    pub level: libc::c_int,
    pub strategy: libc::c_int,
    pub good_match: uInt,
    pub nice_match: libc::c_int,
    pub dyn_ltree: [ct_data_s; 573],
    pub dyn_dtree: [ct_data_s; 61],
    pub bl_tree: [ct_data_s; 39],
    pub l_desc: tree_desc_s,
    pub d_desc: tree_desc_s,
    pub bl_desc: tree_desc_s,
    pub bl_count: [ush; 16],
    pub heap: [libc::c_int; 573],
    pub heap_len: libc::c_int,
    pub heap_max: libc::c_int,
    pub depth: [uch; 573],
    pub l_buf: *mut uchf,
    pub lit_bufsize: uInt,
    pub last_lit: uInt,
    pub d_buf: *mut ushf,
    pub opt_len: ulg,
    pub static_len: ulg,
    pub matches: uInt,
    pub last_eob_len: libc::c_int,
    pub bi_buf: ush,
    pub bi_valid: libc::c_int,
    pub high_water: ulg,
}
pub type ulg = libc::c_ulong;
pub type ush = libc::c_ushort;
pub type ushf = ush;
pub type uchf = uch;
pub type uch = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc_s {
    pub dyn_tree: *mut ct_data,
    pub max_code: libc::c_int,
    pub stat_desc: *mut static_tree_desc,
}
pub type static_tree_desc = static_tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct static_tree_desc_s {
    pub static_tree: *const ct_data,
    pub extra_bits: *const intf,
    pub extra_base: libc::c_int,
    pub elems: libc::c_int,
    pub max_length: libc::c_int,
}
pub type ct_data = ct_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data_s {
    pub fc: C2RustUnnamed_0,
    pub dl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub freq: ush,
    pub code: ush,
}
pub type IPos = libc::c_uint;
pub type Posf = Pos;
pub type Pos = ush;
pub type gz_headerp = *mut gz_header;
pub type gz_header = gz_header_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_header_s {
    pub text: libc::c_int,
    pub time: uLong,
    pub xflags: libc::c_int,
    pub os: libc::c_int,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: libc::c_int,
    pub done: libc::c_int,
}
pub type z_streamp = *mut z_stream;
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type tree_desc = tree_desc_s;
pub type deflate_state = internal_state;
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
static mut static_ltree: [ct_data; 288] = [
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 12 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 140 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 76 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 204 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 44 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 172 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 108 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 236 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 28 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 156 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 92 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 220 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 60 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 188 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 124 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 252 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 2 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 130 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 66 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 194 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 34 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 162 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 98 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 226 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 18 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 146 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 82 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 210 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 50 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 178 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 114 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 242 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 10 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 138 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 74 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 202 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 42 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 170 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 106 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 234 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 26 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 154 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 90 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 218 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 58 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 186 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 122 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 250 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 6 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 134 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 70 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 198 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 38 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 166 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 102 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 230 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 22 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 150 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 86 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 214 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 54 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 182 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 118 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 246 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 14 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 142 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 78 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 206 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 46 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 174 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 110 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 238 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 30 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 158 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 94 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 222 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 62 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 190 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 126 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 254 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 1 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 129 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 65 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 193 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 33 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 161 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 97 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 225 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 17 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 145 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 81 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 209 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 49 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 177 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 113 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 241 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 9 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 137 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 73 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 201 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 41 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 169 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 105 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 233 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 25 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 153 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 89 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 217 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 57 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 185 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 121 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 249 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 5 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 133 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 69 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 197 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 37 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 165 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 101 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 229 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 21 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 149 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 85 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 213 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 53 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 181 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 117 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 245 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 13 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 141 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 77 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 205 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 45 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 173 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 109 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 237 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 29 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 157 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 93 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 221 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 61 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 189 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 125 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 253 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 19 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 275 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 147 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 403 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 83 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 339 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 211 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 467 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 51 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 307 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 179 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 435 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 115 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 371 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 243 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 499 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 11 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 267 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 139 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 395 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 75 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 331 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 203 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 459 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 43 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 299 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 171 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 427 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 107 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 363 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 235 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 491 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 27 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 283 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 155 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 411 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 91 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 347 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 219 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 475 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 59 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 315 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 187 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 443 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 123 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 379 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 251 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 507 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 7 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 263 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 135 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 391 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 71 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 327 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 199 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 455 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 39 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 295 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 167 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 423 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 103 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 359 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 231 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 487 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 23 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 279 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 151 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 407 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 87 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 343 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 215 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 471 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 55 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 311 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 183 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 439 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 119 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 375 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 247 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 503 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 15 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 271 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 143 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 399 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 79 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 335 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 207 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 463 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 47 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 303 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 175 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 431 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 111 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 367 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 239 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 495 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 31 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 287 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 159 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 415 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 95 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 351 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 223 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 479 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 63 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 319 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 191 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 447 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 127 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 383 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 255 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 511 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 9 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 0 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 64 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 32 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 96 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 16 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 80 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 48 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 112 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 8 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 72 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 40 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 104 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 24 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 88 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 56 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 120 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 4 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 68 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 36 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 100 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 20 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 84 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 52 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 116 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 7 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 3 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 131 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 67 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 195 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 35 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 163 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 99 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 227 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 8 as libc::c_int as ush,
            },
        };
        init
    },
];
static mut static_dtree: [ct_data; 30] = [
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 0 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 16 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 8 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 24 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 4 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 20 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 12 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 28 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 2 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 18 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 10 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 26 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 6 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 22 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 14 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 30 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 1 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 17 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 9 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 25 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 5 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 21 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 13 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 29 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 3 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 19 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 11 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 27 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 7 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
    {
        let mut init = ct_data_s {
            fc: C2RustUnnamed_0 {
                freq: 23 as libc::c_int as ush,
            },
            dl: C2RustUnnamed {
                dad: 5 as libc::c_int as ush,
            },
        };
        init
    },
];
pub static mut _glp_zlib_dist_code: [uch; 512] = [
    0 as libc::c_int as uch,
    1 as libc::c_int as uch,
    2 as libc::c_int as uch,
    3 as libc::c_int as uch,
    4 as libc::c_int as uch,
    4 as libc::c_int as uch,
    5 as libc::c_int as uch,
    5 as libc::c_int as uch,
    6 as libc::c_int as uch,
    6 as libc::c_int as uch,
    6 as libc::c_int as uch,
    6 as libc::c_int as uch,
    7 as libc::c_int as uch,
    7 as libc::c_int as uch,
    7 as libc::c_int as uch,
    7 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    0 as libc::c_int as uch,
    0 as libc::c_int as uch,
    16 as libc::c_int as uch,
    17 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    28 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
    29 as libc::c_int as uch,
];
pub static mut _glp_zlib_length_code: [uch; 256] = [
    0 as libc::c_int as uch,
    1 as libc::c_int as uch,
    2 as libc::c_int as uch,
    3 as libc::c_int as uch,
    4 as libc::c_int as uch,
    5 as libc::c_int as uch,
    6 as libc::c_int as uch,
    7 as libc::c_int as uch,
    8 as libc::c_int as uch,
    8 as libc::c_int as uch,
    9 as libc::c_int as uch,
    9 as libc::c_int as uch,
    10 as libc::c_int as uch,
    10 as libc::c_int as uch,
    11 as libc::c_int as uch,
    11 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    12 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    13 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    14 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    15 as libc::c_int as uch,
    16 as libc::c_int as uch,
    16 as libc::c_int as uch,
    16 as libc::c_int as uch,
    16 as libc::c_int as uch,
    16 as libc::c_int as uch,
    16 as libc::c_int as uch,
    16 as libc::c_int as uch,
    16 as libc::c_int as uch,
    17 as libc::c_int as uch,
    17 as libc::c_int as uch,
    17 as libc::c_int as uch,
    17 as libc::c_int as uch,
    17 as libc::c_int as uch,
    17 as libc::c_int as uch,
    17 as libc::c_int as uch,
    17 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    18 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    19 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    20 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    21 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    22 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    23 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    24 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    25 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    26 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    27 as libc::c_int as uch,
    28 as libc::c_int as uch,
];
static mut base_length: [libc::c_int; 29] = [
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    10 as libc::c_int,
    12 as libc::c_int,
    14 as libc::c_int,
    16 as libc::c_int,
    20 as libc::c_int,
    24 as libc::c_int,
    28 as libc::c_int,
    32 as libc::c_int,
    40 as libc::c_int,
    48 as libc::c_int,
    56 as libc::c_int,
    64 as libc::c_int,
    80 as libc::c_int,
    96 as libc::c_int,
    112 as libc::c_int,
    128 as libc::c_int,
    160 as libc::c_int,
    192 as libc::c_int,
    224 as libc::c_int,
    0 as libc::c_int,
];
static mut base_dist: [libc::c_int; 30] = [
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    6 as libc::c_int,
    8 as libc::c_int,
    12 as libc::c_int,
    16 as libc::c_int,
    24 as libc::c_int,
    32 as libc::c_int,
    48 as libc::c_int,
    64 as libc::c_int,
    96 as libc::c_int,
    128 as libc::c_int,
    192 as libc::c_int,
    256 as libc::c_int,
    384 as libc::c_int,
    512 as libc::c_int,
    768 as libc::c_int,
    1024 as libc::c_int,
    1536 as libc::c_int,
    2048 as libc::c_int,
    3072 as libc::c_int,
    4096 as libc::c_int,
    6144 as libc::c_int,
    8192 as libc::c_int,
    12288 as libc::c_int,
    16384 as libc::c_int,
    24576 as libc::c_int,
];
static mut static_l_desc: static_tree_desc = unsafe {
    {
        let mut init = static_tree_desc_s {
            static_tree: static_ltree.as_ptr(),
            extra_bits: extra_lbits.as_ptr(),
            extra_base: 256 as libc::c_int + 1 as libc::c_int,
            elems: 256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int,
            max_length: 15 as libc::c_int,
        };
        init
    }
};
static mut static_d_desc: static_tree_desc = unsafe {
    {
        let mut init = static_tree_desc_s {
            static_tree: static_dtree.as_ptr(),
            extra_bits: extra_dbits.as_ptr(),
            extra_base: 0 as libc::c_int,
            elems: 30 as libc::c_int,
            max_length: 15 as libc::c_int,
        };
        init
    }
};
static mut static_bl_desc: static_tree_desc = unsafe {
    {
        let mut init = static_tree_desc_s {
            static_tree: 0 as *const ct_data,
            extra_bits: extra_blbits.as_ptr(),
            extra_base: 0 as libc::c_int,
            elems: 19 as libc::c_int,
            max_length: 7 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn tr_static_init() {}
pub unsafe extern "C" fn _glp_zlib_tr_init(mut s: *mut deflate_state) {
    tr_static_init();
    (*s).l_desc.dyn_tree = ((*s).dyn_ltree).as_mut_ptr();
    (*s).l_desc.stat_desc = &mut static_l_desc;
    (*s).d_desc.dyn_tree = ((*s).dyn_dtree).as_mut_ptr();
    (*s).d_desc.stat_desc = &mut static_d_desc;
    (*s).bl_desc.dyn_tree = ((*s).bl_tree).as_mut_ptr();
    (*s).bl_desc.stat_desc = &mut static_bl_desc;
    (*s).bi_buf = 0 as libc::c_int as ush;
    (*s).bi_valid = 0 as libc::c_int;
    (*s).last_eob_len = 8 as libc::c_int;
    init_block(s);
}
unsafe extern "C" fn init_block(mut s: *mut deflate_state) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < 256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int {
        (*s).dyn_ltree[n as usize].fc.freq = 0 as libc::c_int as ush;
        n += 1;
        n;
    }
    n = 0 as libc::c_int;
    while n < 30 as libc::c_int {
        (*s).dyn_dtree[n as usize].fc.freq = 0 as libc::c_int as ush;
        n += 1;
        n;
    }
    n = 0 as libc::c_int;
    while n < 19 as libc::c_int {
        (*s).bl_tree[n as usize].fc.freq = 0 as libc::c_int as ush;
        n += 1;
        n;
    }
    (*s).dyn_ltree[256 as libc::c_int as usize].fc.freq = 1 as libc::c_int as ush;
    (*s).static_len = 0 as libc::c_long as ulg;
    (*s).opt_len = (*s).static_len;
    (*s).matches = 0 as libc::c_int as uInt;
    (*s).last_lit = (*s).matches;
}
unsafe extern "C" fn pqdownheap(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut k: libc::c_int,
) {
    let mut v: libc::c_int = (*s).heap[k as usize];
    let mut j: libc::c_int = k << 1 as libc::c_int;
    while j <= (*s).heap_len {
        if j < (*s).heap_len
            && (((*tree.offset((*s).heap[(j + 1 as libc::c_int) as usize] as isize))
                .fc
                .freq as libc::c_int)
                < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as libc::c_int
                || (*tree.offset((*s).heap[(j + 1 as libc::c_int) as usize] as isize))
                    .fc
                    .freq as libc::c_int
                    == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq
                        as libc::c_int
                    && (*s).depth[(*s).heap[(j + 1 as libc::c_int) as usize] as usize]
                        as libc::c_int
                        <= (*s).depth[(*s).heap[j as usize] as usize] as libc::c_int)
        {
            j += 1;
            j;
        }
        if ((*tree.offset(v as isize)).fc.freq as libc::c_int)
            < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as libc::c_int
            || (*tree.offset(v as isize)).fc.freq as libc::c_int
                == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as libc::c_int
                && (*s).depth[v as usize] as libc::c_int
                    <= (*s).depth[(*s).heap[j as usize] as usize] as libc::c_int
        {
            break;
        }
        (*s).heap[k as usize] = (*s).heap[j as usize];
        k = j;
        j <<= 1 as libc::c_int;
    }
    (*s).heap[k as usize] = v;
}
unsafe extern "C" fn gen_bitlen(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut max_code: libc::c_int = (*desc).max_code;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut extra: *const intf = (*(*desc).stat_desc).extra_bits;
    let mut base: libc::c_int = (*(*desc).stat_desc).extra_base;
    let mut max_length: libc::c_int = (*(*desc).stat_desc).max_length;
    let mut h: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut xbits: libc::c_int = 0;
    let mut f: ush = 0;
    let mut overflow: libc::c_int = 0 as libc::c_int;
    bits = 0 as libc::c_int;
    while bits <= 15 as libc::c_int {
        (*s).bl_count[bits as usize] = 0 as libc::c_int as ush;
        bits += 1;
        bits;
    }
    (*tree.offset((*s).heap[(*s).heap_max as usize] as isize))
        .dl
        .len = 0 as libc::c_int as ush;
    h = (*s).heap_max + 1 as libc::c_int;
    while h
        < 2 as libc::c_int * (256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int)
            + 1 as libc::c_int
    {
        n = (*s).heap[h as usize];
        bits = (*tree.offset((*tree.offset(n as isize)).dl.dad as isize)).dl.len
            as libc::c_int + 1 as libc::c_int;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
            overflow;
        }
        (*tree.offset(n as isize)).dl.len = bits as ush;
        if !(n > max_code) {
            (*s)
                .bl_count[bits
                as usize] = ((*s).bl_count[bits as usize]).wrapping_add(1);
            (*s).bl_count[bits as usize];
            xbits = 0 as libc::c_int;
            if n >= base {
                xbits = *extra.offset((n - base) as isize);
            }
            f = (*tree.offset(n as isize)).fc.freq;
            (*s)
                .opt_len = ((*s).opt_len as libc::c_ulong)
                .wrapping_add((f as ulg).wrapping_mul((bits + xbits) as libc::c_ulong))
                as ulg as ulg;
            if !stree.is_null() {
                (*s)
                    .static_len = ((*s).static_len as libc::c_ulong)
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
        while (*s).bl_count[bits as usize] as libc::c_int == 0 as libc::c_int {
            bits -= 1;
            bits;
        }
        (*s).bl_count[bits as usize] = ((*s).bl_count[bits as usize]).wrapping_sub(1);
        (*s).bl_count[bits as usize];
        (*s)
            .bl_count[(bits + 1 as libc::c_int)
            as usize] = ((*s).bl_count[(bits + 1 as libc::c_int) as usize] as libc::c_int
            + 2 as libc::c_int) as ush;
        (*s)
            .bl_count[max_length
            as usize] = ((*s).bl_count[max_length as usize]).wrapping_sub(1);
        (*s).bl_count[max_length as usize];
        overflow -= 2 as libc::c_int;
        if !(overflow > 0 as libc::c_int) {
            break;
        }
    }
    bits = max_length;
    while bits != 0 as libc::c_int {
        n = (*s).bl_count[bits as usize] as libc::c_int;
        while n != 0 as libc::c_int {
            h -= 1;
            m = (*s).heap[h as usize];
            if m > max_code {
                continue;
            }
            if (*tree.offset(m as isize)).dl.len as libc::c_uint != bits as libc::c_uint
            {
                (*s)
                    .opt_len = ((*s).opt_len as libc::c_ulong)
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
unsafe extern "C" fn gen_codes(
    mut tree: *mut ct_data,
    mut max_code: libc::c_int,
    mut bl_count: *mut ushf,
) {
    let mut next_code: [ush; 16] = [0; 16];
    let mut code: ush = 0 as libc::c_int as ush;
    let mut bits: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    bits = 1 as libc::c_int;
    while bits <= 15 as libc::c_int {
        code = ((code as libc::c_int
            + *bl_count.offset((bits - 1 as libc::c_int) as isize) as libc::c_int)
            << 1 as libc::c_int) as ush;
        next_code[bits as usize] = code;
        bits += 1;
        bits;
    }
    n = 0 as libc::c_int;
    while n <= max_code {
        let mut len: libc::c_int = (*tree.offset(n as isize)).dl.len as libc::c_int;
        if !(len == 0 as libc::c_int) {
            let fresh0 = next_code[len as usize];
            next_code[len as usize] = (next_code[len as usize]).wrapping_add(1);
            (*tree.offset(n as isize))
                .fc
                .code = bi_reverse(fresh0 as libc::c_uint, len) as ush;
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn build_tree(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut elems: libc::c_int = (*(*desc).stat_desc).elems;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut max_code: libc::c_int = -(1 as libc::c_int);
    let mut node: libc::c_int = 0;
    (*s).heap_len = 0 as libc::c_int;
    (*s)
        .heap_max = 2 as libc::c_int
        * (256 as libc::c_int + 1 as libc::c_int + 29 as libc::c_int) + 1 as libc::c_int;
    n = 0 as libc::c_int;
    while n < elems {
        if (*tree.offset(n as isize)).fc.freq as libc::c_int != 0 as libc::c_int {
            max_code = n;
            (*s).heap_len += 1;
            (*s).heap[(*s).heap_len as usize] = max_code;
            (*s).depth[n as usize] = 0 as libc::c_int as uch;
        } else {
            (*tree.offset(n as isize)).dl.len = 0 as libc::c_int as ush;
        }
        n += 1;
        n;
    }
    while (*s).heap_len < 2 as libc::c_int {
        (*s).heap_len += 1;
        (*s)
            .heap[(*s).heap_len
            as usize] = if max_code < 2 as libc::c_int {
            max_code += 1;
            max_code
        } else {
            0 as libc::c_int
        };
        node = (*s).heap[(*s).heap_len as usize];
        (*tree.offset(node as isize)).fc.freq = 1 as libc::c_int as ush;
        (*s).depth[node as usize] = 0 as libc::c_int as uch;
        (*s).opt_len = ((*s).opt_len).wrapping_sub(1);
        (*s).opt_len;
        if !stree.is_null() {
            (*s)
                .static_len = ((*s).static_len as libc::c_ulong)
                .wrapping_sub((*stree.offset(node as isize)).dl.len as libc::c_ulong)
                as ulg as ulg;
        }
    }
    (*desc).max_code = max_code;
    n = (*s).heap_len / 2 as libc::c_int;
    while n >= 1 as libc::c_int {
        pqdownheap(s, tree, n);
        n -= 1;
        n;
    }
    node = elems;
    loop {
        n = (*s).heap[1 as libc::c_int as usize];
        let fresh1 = (*s).heap_len;
        (*s).heap_len = (*s).heap_len - 1;
        (*s).heap[1 as libc::c_int as usize] = (*s).heap[fresh1 as usize];
        pqdownheap(s, tree, 1 as libc::c_int);
        m = (*s).heap[1 as libc::c_int as usize];
        (*s).heap_max -= 1;
        (*s).heap[(*s).heap_max as usize] = n;
        (*s).heap_max -= 1;
        (*s).heap[(*s).heap_max as usize] = m;
        (*tree.offset(node as isize))
            .fc
            .freq = ((*tree.offset(n as isize)).fc.freq as libc::c_int
            + (*tree.offset(m as isize)).fc.freq as libc::c_int) as ush;
        (*s)
            .depth[node
            as usize] = ((if (*s).depth[n as usize] as libc::c_int
            >= (*s).depth[m as usize] as libc::c_int
        {
            (*s).depth[n as usize] as libc::c_int
        } else {
            (*s).depth[m as usize] as libc::c_int
        }) + 1 as libc::c_int) as uch;
        let ref mut fresh2 = (*tree.offset(m as isize)).dl.dad;
        *fresh2 = node as ush;
        (*tree.offset(n as isize)).dl.dad = *fresh2;
        let fresh3 = node;
        node = node + 1;
        (*s).heap[1 as libc::c_int as usize] = fresh3;
        pqdownheap(s, tree, 1 as libc::c_int);
        if !((*s).heap_len >= 2 as libc::c_int) {
            break;
        }
    }
    (*s).heap_max -= 1;
    (*s).heap[(*s).heap_max as usize] = (*s).heap[1 as libc::c_int as usize];
    gen_bitlen(s, desc);
    gen_codes(tree, max_code, ((*s).bl_count).as_mut_ptr());
}
unsafe extern "C" fn scan_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: libc::c_int,
) {
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
                (*s)
                    .bl_tree[curlen as usize]
                    .fc
                    .freq = ((*s).bl_tree[curlen as usize].fc.freq as libc::c_int
                    + count) as ush;
            } else if curlen != 0 as libc::c_int {
                if curlen != prevlen {
                    (*s)
                        .bl_tree[curlen as usize]
                        .fc
                        .freq = ((*s).bl_tree[curlen as usize].fc.freq).wrapping_add(1);
                    (*s).bl_tree[curlen as usize].fc.freq;
                }
                (*s)
                    .bl_tree[16 as libc::c_int as usize]
                    .fc
                    .freq = ((*s).bl_tree[16 as libc::c_int as usize].fc.freq)
                    .wrapping_add(1);
                (*s).bl_tree[16 as libc::c_int as usize].fc.freq;
            } else if count <= 10 as libc::c_int {
                (*s)
                    .bl_tree[17 as libc::c_int as usize]
                    .fc
                    .freq = ((*s).bl_tree[17 as libc::c_int as usize].fc.freq)
                    .wrapping_add(1);
                (*s).bl_tree[17 as libc::c_int as usize].fc.freq;
            } else {
                (*s)
                    .bl_tree[18 as libc::c_int as usize]
                    .fc
                    .freq = ((*s).bl_tree[18 as libc::c_int as usize].fc.freq)
                    .wrapping_add(1);
                (*s).bl_tree[18 as libc::c_int as usize].fc.freq;
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
unsafe extern "C" fn send_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: libc::c_int,
) {
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
                    let mut len: libc::c_int = (*s).bl_tree[curlen as usize].dl.len
                        as libc::c_int;
                    if (*s).bi_valid
                        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int - len
                    {
                        let mut val: libc::c_int = (*s).bl_tree[curlen as usize].fc.code
                            as libc::c_int;
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | (val as ush as libc::c_int) << (*s).bi_valid) as ush;
                        let fresh4 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh4 as isize,
                            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int)
                            as uch;
                        let fresh5 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh5 as isize,
                            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                        (*s)
                            .bi_buf = (val as ush as libc::c_int
                            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                        (*s)
                            .bi_valid = ((*s).bi_valid as libc::c_ulong)
                            .wrapping_add(
                                (len as libc::c_ulong)
                                    .wrapping_sub(
                                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    ),
                            ) as libc::c_int as libc::c_int;
                    } else {
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | ((*s).bl_tree[curlen as usize].fc.code as libc::c_int)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len;
                    }
                    count -= 1;
                    if !(count != 0 as libc::c_int) {
                        break;
                    }
                }
            } else if curlen != 0 as libc::c_int {
                if curlen != prevlen {
                    let mut len_0: libc::c_int = (*s).bl_tree[curlen as usize].dl.len
                        as libc::c_int;
                    if (*s).bi_valid
                        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int - len_0
                    {
                        let mut val_0: libc::c_int = (*s)
                            .bl_tree[curlen as usize]
                            .fc
                            .code as libc::c_int;
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | (val_0 as ush as libc::c_int) << (*s).bi_valid) as ush;
                        let fresh6 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh6 as isize,
                            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int)
                            as uch;
                        let fresh7 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh7 as isize,
                            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                        (*s)
                            .bi_buf = (val_0 as ush as libc::c_int
                            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                        (*s)
                            .bi_valid = ((*s).bi_valid as libc::c_ulong)
                            .wrapping_add(
                                (len_0 as libc::c_ulong)
                                    .wrapping_sub(
                                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    ),
                            ) as libc::c_int as libc::c_int;
                    } else {
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | ((*s).bl_tree[curlen as usize].fc.code as libc::c_int)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_0;
                    }
                    count -= 1;
                    count;
                }
                let mut len_1: libc::c_int = (*s)
                    .bl_tree[16 as libc::c_int as usize]
                    .dl
                    .len as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_1
                {
                    let mut val_1: libc::c_int = (*s)
                        .bl_tree[16 as libc::c_int as usize]
                        .fc
                        .code as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_1 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh8 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh8 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh9 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh9 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_1 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_1 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((*s).bl_tree[16 as libc::c_int as usize].fc.code
                            as libc::c_int) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_1;
                }
                let mut len_2: libc::c_int = 2 as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_2
                {
                    let mut val_2: libc::c_int = count - 3 as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_2 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh10 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh10 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh11 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh11 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_2 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_2 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((count - 3 as libc::c_int) as ush as libc::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2;
                }
            } else if count <= 10 as libc::c_int {
                let mut len_3: libc::c_int = (*s)
                    .bl_tree[17 as libc::c_int as usize]
                    .dl
                    .len as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_3
                {
                    let mut val_3: libc::c_int = (*s)
                        .bl_tree[17 as libc::c_int as usize]
                        .fc
                        .code as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_3 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh12 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh12 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh13 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh13 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_3 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_3 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((*s).bl_tree[17 as libc::c_int as usize].fc.code
                            as libc::c_int) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_3;
                }
                let mut len_4: libc::c_int = 3 as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_4
                {
                    let mut val_4: libc::c_int = count - 3 as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_4 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh14 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh14 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh15 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh15 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_4 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_4 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((count - 3 as libc::c_int) as ush as libc::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_4;
                }
            } else {
                let mut len_5: libc::c_int = (*s)
                    .bl_tree[18 as libc::c_int as usize]
                    .dl
                    .len as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_5
                {
                    let mut val_5: libc::c_int = (*s)
                        .bl_tree[18 as libc::c_int as usize]
                        .fc
                        .code as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_5 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh16 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh16 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh17 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh17 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_5 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_5 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((*s).bl_tree[18 as libc::c_int as usize].fc.code
                            as libc::c_int) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_5;
                }
                let mut len_6: libc::c_int = 7 as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_6
                {
                    let mut val_6: libc::c_int = count - 11 as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_6 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh18 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh18 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh19 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh19 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_6 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_6 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((count - 11 as libc::c_int) as ush as libc::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_6;
                }
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
unsafe extern "C" fn build_bl_tree(mut s: *mut deflate_state) -> libc::c_int {
    let mut max_blindex: libc::c_int = 0;
    scan_tree(s, ((*s).dyn_ltree).as_mut_ptr() as *mut ct_data, (*s).l_desc.max_code);
    scan_tree(s, ((*s).dyn_dtree).as_mut_ptr() as *mut ct_data, (*s).d_desc.max_code);
    build_tree(s, &mut (*s).bl_desc as *mut tree_desc_s as *mut tree_desc);
    max_blindex = 19 as libc::c_int - 1 as libc::c_int;
    while max_blindex >= 3 as libc::c_int {
        if (*s).bl_tree[bl_order[max_blindex as usize] as usize].dl.len as libc::c_int
            != 0 as libc::c_int
        {
            break;
        }
        max_blindex -= 1;
        max_blindex;
    }
    (*s)
        .opt_len = ((*s).opt_len as libc::c_ulong)
        .wrapping_add(
            (3 as libc::c_int * (max_blindex + 1 as libc::c_int) + 5 as libc::c_int
                + 5 as libc::c_int + 4 as libc::c_int) as libc::c_ulong,
        ) as ulg as ulg;
    return max_blindex;
}
unsafe extern "C" fn send_all_trees(
    mut s: *mut deflate_state,
    mut lcodes: libc::c_int,
    mut dcodes: libc::c_int,
    mut blcodes: libc::c_int,
) {
    let mut rank: libc::c_int = 0;
    let mut len: libc::c_int = 5 as libc::c_int;
    if (*s).bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - len
    {
        let mut val: libc::c_int = lcodes - 257 as libc::c_int;
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (val as ush as libc::c_int) << (*s).bi_valid) as ush;
        let fresh20 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh20 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh21 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh21 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s)
            .bi_buf = (val as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
        (*s)
            .bi_valid = ((*s).bi_valid as libc::c_ulong)
            .wrapping_add(
                (len as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | ((lcodes - 257 as libc::c_int) as ush as libc::c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: libc::c_int = 5 as libc::c_int;
    if (*s).bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - len_0
    {
        let mut val_0: libc::c_int = dcodes - 1 as libc::c_int;
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (val_0 as ush as libc::c_int) << (*s).bi_valid) as ush;
        let fresh22 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh22 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh23 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh23 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s)
            .bi_buf = (val_0 as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
        (*s)
            .bi_valid = ((*s).bi_valid as libc::c_ulong)
            .wrapping_add(
                (len_0 as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | ((dcodes - 1 as libc::c_int) as ush as libc::c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_0;
    }
    let mut len_1: libc::c_int = 4 as libc::c_int;
    if (*s).bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - len_1
    {
        let mut val_1: libc::c_int = blcodes - 4 as libc::c_int;
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (val_1 as ush as libc::c_int) << (*s).bi_valid) as ush;
        let fresh24 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh24 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh25 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh25 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s)
            .bi_buf = (val_1 as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
        (*s)
            .bi_valid = ((*s).bi_valid as libc::c_ulong)
            .wrapping_add(
                (len_1 as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | ((blcodes - 4 as libc::c_int) as ush as libc::c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_1;
    }
    rank = 0 as libc::c_int;
    while rank < blcodes {
        let mut len_2: libc::c_int = 3 as libc::c_int;
        if (*s).bi_valid
            > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int - len_2
        {
            let mut val_2: libc::c_int = (*s)
                .bl_tree[bl_order[rank as usize] as usize]
                .dl
                .len as libc::c_int;
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | (val_2 as ush as libc::c_int) << (*s).bi_valid) as ush;
            let fresh26 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh26 as isize,
                ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh27 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh27 as isize,
                ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
            (*s)
                .bi_buf = (val_2 as ush as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
            (*s)
                .bi_valid = ((*s).bi_valid as libc::c_ulong)
                .wrapping_add(
                    (len_2 as libc::c_ulong)
                        .wrapping_sub(
                            ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ),
                        ),
                ) as libc::c_int as libc::c_int;
        } else {
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | ((*s).bl_tree[bl_order[rank as usize] as usize].dl.len as libc::c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_2;
        }
        rank += 1;
        rank;
    }
    send_tree(
        s,
        ((*s).dyn_ltree).as_mut_ptr() as *mut ct_data,
        lcodes - 1 as libc::c_int,
    );
    send_tree(
        s,
        ((*s).dyn_dtree).as_mut_ptr() as *mut ct_data,
        dcodes - 1 as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_zlib_tr_stored_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut stored_len: ulg,
    mut last: libc::c_int,
) {
    let mut len: libc::c_int = 3 as libc::c_int;
    if (*s).bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - len
    {
        let mut val: libc::c_int = ((0 as libc::c_int) << 1 as libc::c_int) + last;
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (val as ush as libc::c_int) << (*s).bi_valid) as ush;
        let fresh28 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh28 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh29 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh29 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s)
            .bi_buf = (val as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
        (*s)
            .bi_valid = ((*s).bi_valid as libc::c_ulong)
            .wrapping_add(
                (len as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | ((((0 as libc::c_int) << 1 as libc::c_int) + last) as ush as libc::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    copy_block(s, buf, stored_len as libc::c_uint, 1 as libc::c_int);
}
pub unsafe extern "C" fn _glp_zlib_tr_align(mut s: *mut deflate_state) {
    let mut len: libc::c_int = 3 as libc::c_int;
    if (*s).bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - len
    {
        let mut val: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (val as ush as libc::c_int) << (*s).bi_valid) as ush;
        let fresh30 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh30 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh31 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh31 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s)
            .bi_buf = (val as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
        (*s)
            .bi_valid = ((*s).bi_valid as libc::c_ulong)
            .wrapping_add(
                (len as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (((1 as libc::c_int) << 1 as libc::c_int) as ush as libc::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: libc::c_int = static_ltree[256 as libc::c_int as usize].dl.len
        as libc::c_int;
    if (*s).bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - len_0
    {
        let mut val_0: libc::c_int = static_ltree[256 as libc::c_int as usize].fc.code
            as libc::c_int;
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (val_0 as ush as libc::c_int) << (*s).bi_valid) as ush;
        let fresh32 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh32 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh33 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh33 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s)
            .bi_buf = (val_0 as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
        (*s)
            .bi_valid = ((*s).bi_valid as libc::c_ulong)
            .wrapping_add(
                (len_0 as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (static_ltree[256 as libc::c_int as usize].fc.code as libc::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len_0;
    }
    bi_flush(s);
    if 1 as libc::c_int + (*s).last_eob_len + 10 as libc::c_int - (*s).bi_valid
        < 9 as libc::c_int
    {
        let mut len_1: libc::c_int = 3 as libc::c_int;
        if (*s).bi_valid
            > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int - len_1
        {
            let mut val_1: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | (val_1 as ush as libc::c_int) << (*s).bi_valid) as ush;
            let fresh34 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh34 as isize,
                ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh35 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh35 as isize,
                ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
            (*s)
                .bi_buf = (val_1 as ush as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
            (*s)
                .bi_valid = ((*s).bi_valid as libc::c_ulong)
                .wrapping_add(
                    (len_1 as libc::c_ulong)
                        .wrapping_sub(
                            ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ),
                        ),
                ) as libc::c_int as libc::c_int;
        } else {
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | (((1 as libc::c_int) << 1 as libc::c_int) as ush as libc::c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_1;
        }
        let mut len_2: libc::c_int = static_ltree[256 as libc::c_int as usize].dl.len
            as libc::c_int;
        if (*s).bi_valid
            > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int - len_2
        {
            let mut val_2: libc::c_int = static_ltree[256 as libc::c_int as usize]
                .fc
                .code as libc::c_int;
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | (val_2 as ush as libc::c_int) << (*s).bi_valid) as ush;
            let fresh36 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh36 as isize,
                ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh37 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh37 as isize,
                ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
            (*s)
                .bi_buf = (val_2 as ush as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
            (*s)
                .bi_valid = ((*s).bi_valid as libc::c_ulong)
                .wrapping_add(
                    (len_2 as libc::c_ulong)
                        .wrapping_sub(
                            ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ),
                        ),
                ) as libc::c_int as libc::c_int;
        } else {
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | (static_ltree[256 as libc::c_int as usize].fc.code as libc::c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_2;
        }
        bi_flush(s);
    }
    (*s).last_eob_len = 7 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_tr_flush_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut stored_len: ulg,
    mut last: libc::c_int,
) {
    let mut opt_lenb: ulg = 0;
    let mut static_lenb: ulg = 0;
    let mut max_blindex: libc::c_int = 0 as libc::c_int;
    if (*s).level > 0 as libc::c_int {
        if (*(*s).strm).data_type == 2 as libc::c_int {
            (*(*s).strm).data_type = detect_data_type(s);
        }
        build_tree(s, &mut (*s).l_desc as *mut tree_desc_s as *mut tree_desc);
        build_tree(s, &mut (*s).d_desc as *mut tree_desc_s as *mut tree_desc);
        max_blindex = build_bl_tree(s);
        opt_lenb = ((*s).opt_len)
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            .wrapping_add(7 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int;
        static_lenb = ((*s).static_len)
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            .wrapping_add(7 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int;
        if static_lenb <= opt_lenb {
            opt_lenb = static_lenb;
        }
    } else {
        static_lenb = stored_len.wrapping_add(5 as libc::c_int as libc::c_ulong);
        opt_lenb = static_lenb;
    }
    if stored_len.wrapping_add(4 as libc::c_int as libc::c_ulong) <= opt_lenb
        && !buf.is_null()
    {
        _glp_zlib_tr_stored_block(s, buf, stored_len, last);
    } else if (*s).strategy == 4 as libc::c_int || static_lenb == opt_lenb {
        let mut len: libc::c_int = 3 as libc::c_int;
        if (*s).bi_valid
            > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int - len
        {
            let mut val: libc::c_int = ((1 as libc::c_int) << 1 as libc::c_int) + last;
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | (val as ush as libc::c_int) << (*s).bi_valid) as ush;
            let fresh38 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh38 as isize,
                ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh39 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh39 as isize,
                ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
            (*s)
                .bi_buf = (val as ush as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
            (*s)
                .bi_valid = ((*s).bi_valid as libc::c_ulong)
                .wrapping_add(
                    (len as libc::c_ulong)
                        .wrapping_sub(
                            ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ),
                        ),
                ) as libc::c_int as libc::c_int;
        } else {
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | ((((1 as libc::c_int) << 1 as libc::c_int) + last) as ush
                    as libc::c_int) << (*s).bi_valid) as ush;
            (*s).bi_valid += len;
        }
        compress_block(
            s,
            static_ltree.as_ptr() as *mut ct_data,
            static_dtree.as_ptr() as *mut ct_data,
        );
    } else {
        let mut len_0: libc::c_int = 3 as libc::c_int;
        if (*s).bi_valid
            > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int - len_0
        {
            let mut val_0: libc::c_int = ((2 as libc::c_int) << 1 as libc::c_int) + last;
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | (val_0 as ush as libc::c_int) << (*s).bi_valid) as ush;
            let fresh40 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh40 as isize,
                ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh41 = (*s).pending;
            (*s).pending = ((*s).pending).wrapping_add(1);
            *((*s).pending_buf)
                .offset(
                    fresh41 as isize,
                ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
            (*s)
                .bi_buf = (val_0 as ush as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
            (*s)
                .bi_valid = ((*s).bi_valid as libc::c_ulong)
                .wrapping_add(
                    (len_0 as libc::c_ulong)
                        .wrapping_sub(
                            ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ),
                        ),
                ) as libc::c_int as libc::c_int;
        } else {
            (*s)
                .bi_buf = ((*s).bi_buf as libc::c_int
                | ((((2 as libc::c_int) << 1 as libc::c_int) + last) as ush
                    as libc::c_int) << (*s).bi_valid) as ush;
            (*s).bi_valid += len_0;
        }
        send_all_trees(
            s,
            (*s).l_desc.max_code + 1 as libc::c_int,
            (*s).d_desc.max_code + 1 as libc::c_int,
            max_blindex + 1 as libc::c_int,
        );
        compress_block(
            s,
            ((*s).dyn_ltree).as_mut_ptr() as *mut ct_data,
            ((*s).dyn_dtree).as_mut_ptr() as *mut ct_data,
        );
    }
    init_block(s);
    if last != 0 {
        bi_windup(s);
    }
}
pub unsafe extern "C" fn _glp_zlib_tr_tally(
    mut s: *mut deflate_state,
    mut dist: libc::c_uint,
    mut lc: libc::c_uint,
) -> libc::c_int {
    *((*s).d_buf).offset((*s).last_lit as isize) = dist as ush;
    let fresh42 = (*s).last_lit;
    (*s).last_lit = ((*s).last_lit).wrapping_add(1);
    *((*s).l_buf).offset(fresh42 as isize) = lc as uch;
    if dist == 0 as libc::c_int as libc::c_uint {
        (*s)
            .dyn_ltree[lc as usize]
            .fc
            .freq = ((*s).dyn_ltree[lc as usize].fc.freq).wrapping_add(1);
        (*s).dyn_ltree[lc as usize].fc.freq;
    } else {
        (*s).matches = ((*s).matches).wrapping_add(1);
        (*s).matches;
        dist = dist.wrapping_sub(1);
        dist;
        (*s)
            .dyn_ltree[(_glp_zlib_length_code[lc as usize] as libc::c_int
                + 256 as libc::c_int + 1 as libc::c_int) as usize]
            .fc
            .freq = ((*s)
            .dyn_ltree[(_glp_zlib_length_code[lc as usize] as libc::c_int
                + 256 as libc::c_int + 1 as libc::c_int) as usize]
            .fc
            .freq)
            .wrapping_add(1);
        (*s)
            .dyn_ltree[(_glp_zlib_length_code[lc as usize] as libc::c_int
                + 256 as libc::c_int + 1 as libc::c_int) as usize]
            .fc
            .freq;
        (*s)
            .dyn_dtree[(if dist < 256 as libc::c_int as libc::c_uint {
                _glp_zlib_dist_code[dist as usize] as libc::c_int
            } else {
                _glp_zlib_dist_code[(256 as libc::c_int as libc::c_uint)
                    .wrapping_add(dist >> 7 as libc::c_int) as usize] as libc::c_int
            }) as usize]
            .fc
            .freq = ((*s)
            .dyn_dtree[(if dist < 256 as libc::c_int as libc::c_uint {
                _glp_zlib_dist_code[dist as usize] as libc::c_int
            } else {
                _glp_zlib_dist_code[(256 as libc::c_int as libc::c_uint)
                    .wrapping_add(dist >> 7 as libc::c_int) as usize] as libc::c_int
            }) as usize]
            .fc
            .freq)
            .wrapping_add(1);
        (*s)
            .dyn_dtree[(if dist < 256 as libc::c_int as libc::c_uint {
                _glp_zlib_dist_code[dist as usize] as libc::c_int
            } else {
                _glp_zlib_dist_code[(256 as libc::c_int as libc::c_uint)
                    .wrapping_add(dist >> 7 as libc::c_int) as usize] as libc::c_int
            }) as usize]
            .fc
            .freq;
    }
    return ((*s).last_lit
        == ((*s).lit_bufsize).wrapping_sub(1 as libc::c_int as libc::c_uint))
        as libc::c_int;
}
unsafe extern "C" fn compress_block(
    mut s: *mut deflate_state,
    mut ltree: *mut ct_data,
    mut dtree: *mut ct_data,
) {
    let mut dist: libc::c_uint = 0;
    let mut lc: libc::c_int = 0;
    let mut lx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut code: libc::c_uint = 0;
    let mut extra: libc::c_int = 0;
    if (*s).last_lit != 0 as libc::c_int as libc::c_uint {
        loop {
            dist = *((*s).d_buf).offset(lx as isize) as libc::c_uint;
            let fresh43 = lx;
            lx = lx.wrapping_add(1);
            lc = *((*s).l_buf).offset(fresh43 as isize) as libc::c_int;
            if dist == 0 as libc::c_int as libc::c_uint {
                let mut len: libc::c_int = (*ltree.offset(lc as isize)).dl.len
                    as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len
                {
                    let mut val: libc::c_int = (*ltree.offset(lc as isize)).fc.code
                        as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh44 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh44 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh45 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh45 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((*ltree.offset(lc as isize)).fc.code as libc::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len;
                }
            } else {
                code = _glp_zlib_length_code[lc as usize] as libc::c_uint;
                let mut len_0: libc::c_int = (*ltree
                    .offset(
                        code
                            .wrapping_add(256 as libc::c_int as libc::c_uint)
                            .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ))
                    .dl
                    .len as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_0
                {
                    let mut val_0: libc::c_int = (*ltree
                        .offset(
                            code
                                .wrapping_add(256 as libc::c_int as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ))
                        .fc
                        .code as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_0 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh46 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh46 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh47 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh47 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_0 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_0 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((*ltree
                            .offset(
                                code
                                    .wrapping_add(256 as libc::c_int as libc::c_uint)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ))
                            .fc
                            .code as libc::c_int) << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_0;
                }
                extra = extra_lbits[code as usize];
                if extra != 0 as libc::c_int {
                    lc -= base_length[code as usize];
                    let mut len_1: libc::c_int = extra;
                    if (*s).bi_valid
                        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int - len_1
                    {
                        let mut val_1: libc::c_int = lc;
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | (val_1 as ush as libc::c_int) << (*s).bi_valid) as ush;
                        let fresh48 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh48 as isize,
                            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int)
                            as uch;
                        let fresh49 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh49 as isize,
                            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                        (*s)
                            .bi_buf = (val_1 as ush as libc::c_int
                            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                        (*s)
                            .bi_valid = ((*s).bi_valid as libc::c_ulong)
                            .wrapping_add(
                                (len_1 as libc::c_ulong)
                                    .wrapping_sub(
                                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    ),
                            ) as libc::c_int as libc::c_int;
                    } else {
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | (lc as ush as libc::c_int) << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_1;
                    }
                }
                dist = dist.wrapping_sub(1);
                dist;
                code = (if dist < 256 as libc::c_int as libc::c_uint {
                    _glp_zlib_dist_code[dist as usize] as libc::c_int
                } else {
                    _glp_zlib_dist_code[(256 as libc::c_int as libc::c_uint)
                        .wrapping_add(dist >> 7 as libc::c_int) as usize] as libc::c_int
                }) as libc::c_uint;
                let mut len_2: libc::c_int = (*dtree.offset(code as isize)).dl.len
                    as libc::c_int;
                if (*s).bi_valid
                    > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ) as libc::c_int - len_2
                {
                    let mut val_2: libc::c_int = (*dtree.offset(code as isize)).fc.code
                        as libc::c_int;
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | (val_2 as ush as libc::c_int) << (*s).bi_valid) as ush;
                    let fresh50 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh50 as isize,
                        ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
                    let fresh51 = (*s).pending;
                    (*s).pending = ((*s).pending).wrapping_add(1);
                    *((*s).pending_buf)
                        .offset(
                            fresh51 as isize,
                        ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                    (*s)
                        .bi_buf = (val_2 as ush as libc::c_int
                        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                            .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                    (*s)
                        .bi_valid = ((*s).bi_valid as libc::c_ulong)
                        .wrapping_add(
                            (len_2 as libc::c_ulong)
                                .wrapping_sub(
                                    ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        ),
                                ),
                        ) as libc::c_int as libc::c_int;
                } else {
                    (*s)
                        .bi_buf = ((*s).bi_buf as libc::c_int
                        | ((*dtree.offset(code as isize)).fc.code as libc::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2;
                }
                extra = extra_dbits[code as usize];
                if extra != 0 as libc::c_int {
                    dist = dist.wrapping_sub(base_dist[code as usize] as libc::c_uint);
                    let mut len_3: libc::c_int = extra;
                    if (*s).bi_valid
                        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ) as libc::c_int - len_3
                    {
                        let mut val_3: libc::c_int = dist as libc::c_int;
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | (val_3 as ush as libc::c_int) << (*s).bi_valid) as ush;
                        let fresh52 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh52 as isize,
                            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int)
                            as uch;
                        let fresh53 = (*s).pending;
                        (*s).pending = ((*s).pending).wrapping_add(1);
                        *((*s).pending_buf)
                            .offset(
                                fresh53 as isize,
                            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
                        (*s)
                            .bi_buf = (val_3 as ush as libc::c_int
                            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                )
                                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
                        (*s)
                            .bi_valid = ((*s).bi_valid as libc::c_ulong)
                            .wrapping_add(
                                (len_3 as libc::c_ulong)
                                    .wrapping_sub(
                                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    ),
                            ) as libc::c_int as libc::c_int;
                    } else {
                        (*s)
                            .bi_buf = ((*s).bi_buf as libc::c_int
                            | (dist as ush as libc::c_int) << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_3;
                    }
                }
            }
            if !(lx < (*s).last_lit) {
                break;
            }
        }
    }
    let mut len_4: libc::c_int = (*ltree.offset(256 as libc::c_int as isize)).dl.len
        as libc::c_int;
    if (*s).bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - len_4
    {
        let mut val_4: libc::c_int = (*ltree.offset(256 as libc::c_int as isize)).fc.code
            as libc::c_int;
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | (val_4 as ush as libc::c_int) << (*s).bi_valid) as ush;
        let fresh54 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh54 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh55 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh55 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s)
            .bi_buf = (val_4 as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub((*s).bi_valid as libc::c_ulong)) as ush;
        (*s)
            .bi_valid = ((*s).bi_valid as libc::c_ulong)
            .wrapping_add(
                (len_4 as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        (*s)
            .bi_buf = ((*s).bi_buf as libc::c_int
            | ((*ltree.offset(256 as libc::c_int as isize)).fc.code as libc::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len_4;
    }
    (*s)
        .last_eob_len = (*ltree.offset(256 as libc::c_int as isize)).dl.len
        as libc::c_int;
}
unsafe extern "C" fn detect_data_type(mut s: *mut deflate_state) -> libc::c_int {
    let mut black_mask: libc::c_ulong = 0xf3ffc07f as libc::c_ulong;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n <= 31 as libc::c_int {
        if black_mask & 1 as libc::c_int as libc::c_ulong != 0
            && (*s).dyn_ltree[n as usize].fc.freq as libc::c_int != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        n += 1;
        n;
        black_mask >>= 1 as libc::c_int;
    }
    if (*s).dyn_ltree[9 as libc::c_int as usize].fc.freq as libc::c_int
        != 0 as libc::c_int
        || (*s).dyn_ltree[10 as libc::c_int as usize].fc.freq as libc::c_int
            != 0 as libc::c_int
        || (*s).dyn_ltree[13 as libc::c_int as usize].fc.freq as libc::c_int
            != 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    n = 32 as libc::c_int;
    while n < 256 as libc::c_int {
        if (*s).dyn_ltree[n as usize].fc.freq as libc::c_int != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        n += 1;
        n;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn bi_reverse(
    mut code: libc::c_uint,
    mut len: libc::c_int,
) -> libc::c_uint {
    let mut res: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        res |= code & 1 as libc::c_int as libc::c_uint;
        code >>= 1 as libc::c_int;
        res <<= 1 as libc::c_int;
        len -= 1;
        if !(len > 0 as libc::c_int) {
            break;
        }
    }
    return res >> 1 as libc::c_int;
}
unsafe extern "C" fn bi_flush(mut s: *mut deflate_state) {
    if (*s).bi_valid == 16 as libc::c_int {
        let fresh56 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh56 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh57 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh57 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        (*s).bi_buf = 0 as libc::c_int as ush;
        (*s).bi_valid = 0 as libc::c_int;
    } else if (*s).bi_valid >= 8 as libc::c_int {
        let fresh58 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh58 as isize) = (*s).bi_buf as Byte;
        (*s).bi_buf = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as ush;
        (*s).bi_valid -= 8 as libc::c_int;
    }
}
unsafe extern "C" fn bi_windup(mut s: *mut deflate_state) {
    if (*s).bi_valid > 8 as libc::c_int {
        let fresh59 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh59 as isize,
            ) = ((*s).bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh60 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh60 as isize,
            ) = ((*s).bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
    } else if (*s).bi_valid > 0 as libc::c_int {
        let fresh61 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh61 as isize) = (*s).bi_buf as Byte;
    }
    (*s).bi_buf = 0 as libc::c_int as ush;
    (*s).bi_valid = 0 as libc::c_int;
}
unsafe extern "C" fn copy_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut len: libc::c_uint,
    mut header: libc::c_int,
) {
    bi_windup(s);
    (*s).last_eob_len = 8 as libc::c_int;
    if header != 0 {
        let fresh62 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh62 as isize,
            ) = (len as ush as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh63 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh63 as isize,
            ) = (len as ush as libc::c_int >> 8 as libc::c_int) as uch;
        let fresh64 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh64 as isize,
            ) = (!len as ush as libc::c_int & 0xff as libc::c_int) as uch;
        let fresh65 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf)
            .offset(
                fresh65 as isize,
            ) = (!len as ush as libc::c_int >> 8 as libc::c_int) as uch;
    }
    loop {
        let fresh66 = len;
        len = len.wrapping_sub(1);
        if !(fresh66 != 0) {
            break;
        }
        let fresh67 = buf;
        buf = buf.offset(1);
        let fresh68 = (*s).pending;
        (*s).pending = ((*s).pending).wrapping_add(1);
        *((*s).pending_buf).offset(fresh68 as isize) = *fresh67 as Bytef;
    };
}
