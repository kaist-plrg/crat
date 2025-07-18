use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md2_ctx {
    pub C: [uint8_t; 16],
    pub X: [uint8_t; 48],
    pub index: libc::c_uint,
    pub block: [uint8_t; 16],
}
static mut S: [uint8_t; 256] = [
    41 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    67 as libc::c_int as uint8_t,
    201 as libc::c_int as uint8_t,
    162 as libc::c_int as uint8_t,
    216 as libc::c_int as uint8_t,
    124 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    61 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    84 as libc::c_int as uint8_t,
    161 as libc::c_int as uint8_t,
    236 as libc::c_int as uint8_t,
    240 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    98 as libc::c_int as uint8_t,
    167 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    243 as libc::c_int as uint8_t,
    192 as libc::c_int as uint8_t,
    199 as libc::c_int as uint8_t,
    115 as libc::c_int as uint8_t,
    140 as libc::c_int as uint8_t,
    152 as libc::c_int as uint8_t,
    147 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    217 as libc::c_int as uint8_t,
    188 as libc::c_int as uint8_t,
    76 as libc::c_int as uint8_t,
    130 as libc::c_int as uint8_t,
    202 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    155 as libc::c_int as uint8_t,
    87 as libc::c_int as uint8_t,
    60 as libc::c_int as uint8_t,
    253 as libc::c_int as uint8_t,
    212 as libc::c_int as uint8_t,
    224 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    103 as libc::c_int as uint8_t,
    66 as libc::c_int as uint8_t,
    111 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    138 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    229 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    190 as libc::c_int as uint8_t,
    78 as libc::c_int as uint8_t,
    196 as libc::c_int as uint8_t,
    214 as libc::c_int as uint8_t,
    218 as libc::c_int as uint8_t,
    158 as libc::c_int as uint8_t,
    222 as libc::c_int as uint8_t,
    73 as libc::c_int as uint8_t,
    160 as libc::c_int as uint8_t,
    251 as libc::c_int as uint8_t,
    245 as libc::c_int as uint8_t,
    142 as libc::c_int as uint8_t,
    187 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    238 as libc::c_int as uint8_t,
    122 as libc::c_int as uint8_t,
    169 as libc::c_int as uint8_t,
    104 as libc::c_int as uint8_t,
    121 as libc::c_int as uint8_t,
    145 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    178 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    63 as libc::c_int as uint8_t,
    148 as libc::c_int as uint8_t,
    194 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    137 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    95 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    128 as libc::c_int as uint8_t,
    127 as libc::c_int as uint8_t,
    93 as libc::c_int as uint8_t,
    154 as libc::c_int as uint8_t,
    90 as libc::c_int as uint8_t,
    144 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    62 as libc::c_int as uint8_t,
    204 as libc::c_int as uint8_t,
    231 as libc::c_int as uint8_t,
    191 as libc::c_int as uint8_t,
    247 as libc::c_int as uint8_t,
    151 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    179 as libc::c_int as uint8_t,
    72 as libc::c_int as uint8_t,
    165 as libc::c_int as uint8_t,
    181 as libc::c_int as uint8_t,
    209 as libc::c_int as uint8_t,
    215 as libc::c_int as uint8_t,
    94 as libc::c_int as uint8_t,
    146 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    172 as libc::c_int as uint8_t,
    86 as libc::c_int as uint8_t,
    170 as libc::c_int as uint8_t,
    198 as libc::c_int as uint8_t,
    79 as libc::c_int as uint8_t,
    184 as libc::c_int as uint8_t,
    56 as libc::c_int as uint8_t,
    210 as libc::c_int as uint8_t,
    150 as libc::c_int as uint8_t,
    164 as libc::c_int as uint8_t,
    125 as libc::c_int as uint8_t,
    182 as libc::c_int as uint8_t,
    118 as libc::c_int as uint8_t,
    252 as libc::c_int as uint8_t,
    107 as libc::c_int as uint8_t,
    226 as libc::c_int as uint8_t,
    156 as libc::c_int as uint8_t,
    116 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    241 as libc::c_int as uint8_t,
    69 as libc::c_int as uint8_t,
    157 as libc::c_int as uint8_t,
    112 as libc::c_int as uint8_t,
    89 as libc::c_int as uint8_t,
    100 as libc::c_int as uint8_t,
    113 as libc::c_int as uint8_t,
    135 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    134 as libc::c_int as uint8_t,
    91 as libc::c_int as uint8_t,
    207 as libc::c_int as uint8_t,
    101 as libc::c_int as uint8_t,
    230 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    168 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    96 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    173 as libc::c_int as uint8_t,
    174 as libc::c_int as uint8_t,
    176 as libc::c_int as uint8_t,
    185 as libc::c_int as uint8_t,
    246 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    70 as libc::c_int as uint8_t,
    97 as libc::c_int as uint8_t,
    105 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    64 as libc::c_int as uint8_t,
    126 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    85 as libc::c_int as uint8_t,
    71 as libc::c_int as uint8_t,
    163 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    221 as libc::c_int as uint8_t,
    81 as libc::c_int as uint8_t,
    175 as libc::c_int as uint8_t,
    58 as libc::c_int as uint8_t,
    195 as libc::c_int as uint8_t,
    92 as libc::c_int as uint8_t,
    249 as libc::c_int as uint8_t,
    206 as libc::c_int as uint8_t,
    186 as libc::c_int as uint8_t,
    197 as libc::c_int as uint8_t,
    234 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    83 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    110 as libc::c_int as uint8_t,
    133 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    132 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    211 as libc::c_int as uint8_t,
    223 as libc::c_int as uint8_t,
    205 as libc::c_int as uint8_t,
    244 as libc::c_int as uint8_t,
    65 as libc::c_int as uint8_t,
    129 as libc::c_int as uint8_t,
    77 as libc::c_int as uint8_t,
    82 as libc::c_int as uint8_t,
    106 as libc::c_int as uint8_t,
    220 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    200 as libc::c_int as uint8_t,
    108 as libc::c_int as uint8_t,
    193 as libc::c_int as uint8_t,
    171 as libc::c_int as uint8_t,
    250 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    225 as libc::c_int as uint8_t,
    123 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    189 as libc::c_int as uint8_t,
    177 as libc::c_int as uint8_t,
    74 as libc::c_int as uint8_t,
    120 as libc::c_int as uint8_t,
    136 as libc::c_int as uint8_t,
    149 as libc::c_int as uint8_t,
    139 as libc::c_int as uint8_t,
    227 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    232 as libc::c_int as uint8_t,
    109 as libc::c_int as uint8_t,
    233 as libc::c_int as uint8_t,
    203 as libc::c_int as uint8_t,
    213 as libc::c_int as uint8_t,
    254 as libc::c_int as uint8_t,
    59 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    57 as libc::c_int as uint8_t,
    242 as libc::c_int as uint8_t,
    239 as libc::c_int as uint8_t,
    183 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    102 as libc::c_int as uint8_t,
    88 as libc::c_int as uint8_t,
    208 as libc::c_int as uint8_t,
    228 as libc::c_int as uint8_t,
    166 as libc::c_int as uint8_t,
    119 as libc::c_int as uint8_t,
    114 as libc::c_int as uint8_t,
    248 as libc::c_int as uint8_t,
    235 as libc::c_int as uint8_t,
    117 as libc::c_int as uint8_t,
    75 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    68 as libc::c_int as uint8_t,
    80 as libc::c_int as uint8_t,
    180 as libc::c_int as uint8_t,
    143 as libc::c_int as uint8_t,
    237 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    219 as libc::c_int as uint8_t,
    153 as libc::c_int as uint8_t,
    141 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    159 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    131 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
];
unsafe extern "C" fn md2_transform(mut ctx: *mut md2_ctx, mut data: *const uint8_t) {
    let mut i: libc::c_uint = 0;
    let mut t: uint8_t = 0;
    memcpy(
        ((*ctx).X).as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    t = (*ctx).C[15 as libc::c_int as usize];
    while i < 16 as libc::c_int as libc::c_uint {
        (*ctx)
            .X[((2 as libc::c_int * 16 as libc::c_int) as libc::c_uint).wrapping_add(i)
            as usize] = ((*ctx).X[i as usize] as libc::c_int
            ^ (*ctx).X[(16 as libc::c_int as libc::c_uint).wrapping_add(i) as usize]
                as libc::c_int) as uint8_t;
        (*ctx)
            .C[i
            as usize] = ((*ctx).C[i as usize] as libc::c_int
            ^ S[(*data.offset(i as isize) as libc::c_int ^ t as libc::c_int) as usize]
                as libc::c_int) as uint8_t;
        t = (*ctx).C[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    t = 0 as libc::c_int as uint8_t;
    i = t as libc::c_uint;
    while i < (16 as libc::c_int + 2 as libc::c_int) as libc::c_uint {
        let mut j: libc::c_uint = 0;
        j = 0 as libc::c_int as libc::c_uint;
        while j < (3 as libc::c_int * 16 as libc::c_int) as libc::c_uint {
            (*ctx)
                .X[j
                as usize] = ((*ctx).X[j as usize] as libc::c_int
                ^ S[t as usize] as libc::c_int) as uint8_t;
            t = (*ctx).X[j as usize];
            j = j.wrapping_add(1);
            j;
        }
        t = ((t as libc::c_uint).wrapping_add(i) & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn nettle_md2_init(mut ctx: *mut md2_ctx) {
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<md2_ctx>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn nettle_md2_update(
    mut ctx: *mut md2_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block_16: u64;
    if (*ctx).index != 0 {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 16]>()
            as libc::c_ulong)
            .wrapping_sub((*ctx).index as libc::c_ulong) as libc::c_uint;
        if length < __md_left as libc::c_ulong {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx)
                .index = ((*ctx).index as libc::c_ulong).wrapping_add(length)
                as libc::c_uint as libc::c_uint;
            current_block_16 = 13056961889198038528;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            md2_transform(ctx, ((*ctx).block).as_mut_ptr());
            data = data.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_16 = 5720623009719927633;
        }
    } else {
        current_block_16 = 5720623009719927633;
    }
    match current_block_16 {
        5720623009719927633 => {
            while length >= ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong {
                md2_transform(ctx, data);
                data = data
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong as isize,
                    );
                length = (length as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            memcpy(
                ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx).index = length as libc::c_uint;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn nettle_md2_digest(
    mut ctx: *mut md2_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut left: libc::c_uint = 0;
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= MD2_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"md2.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void nettle_md2_digest(struct md2_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1504: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= MD2_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"md2.c\0" as *const u8 as *const libc::c_char,
                130 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void nettle_md2_digest(struct md2_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    left = (16 as libc::c_int as libc::c_uint).wrapping_sub((*ctx).index);
    memset(
        ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize) as *mut libc::c_void,
        left as libc::c_int,
        left as libc::c_ulong,
    );
    md2_transform(ctx, ((*ctx).block).as_mut_ptr());
    md2_transform(ctx, ((*ctx).C).as_mut_ptr());
    memcpy(
        digest as *mut libc::c_void,
        ((*ctx).X).as_mut_ptr() as *const libc::c_void,
        length,
    );
    nettle_md2_init(ctx);
}
