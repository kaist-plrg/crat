use ::libc;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
static mut mtable: [uint32_t; 256] = [
    0 as libc::c_int as uint32_t,
    0xb0d090e as libc::c_int as uint32_t,
    0x161a121c as libc::c_int as uint32_t,
    0x1d171b12 as libc::c_int as uint32_t,
    0x2c342438 as libc::c_int as uint32_t,
    0x27392d36 as libc::c_int as uint32_t,
    0x3a2e3624 as libc::c_int as uint32_t,
    0x31233f2a as libc::c_int as uint32_t,
    0x58684870 as libc::c_int as uint32_t,
    0x5365417e as libc::c_int as uint32_t,
    0x4e725a6c as libc::c_int as uint32_t,
    0x457f5362 as libc::c_int as uint32_t,
    0x745c6c48 as libc::c_int as uint32_t,
    0x7f516546 as libc::c_int as uint32_t,
    0x62467e54 as libc::c_int as uint32_t,
    0x694b775a as libc::c_int as uint32_t,
    0xb0d090e0 as libc::c_uint,
    0xbbdd99ee as libc::c_uint,
    0xa6ca82fc as libc::c_uint,
    0xadc78bf2 as libc::c_uint,
    0x9ce4b4d8 as libc::c_uint,
    0x97e9bdd6 as libc::c_uint,
    0x8afea6c4 as libc::c_uint,
    0x81f3afca as libc::c_uint,
    0xe8b8d890 as libc::c_uint,
    0xe3b5d19e as libc::c_uint,
    0xfea2ca8c as libc::c_uint,
    0xf5afc382 as libc::c_uint,
    0xc48cfca8 as libc::c_uint,
    0xcf81f5a6 as libc::c_uint,
    0xd296eeb4 as libc::c_uint,
    0xd99be7ba as libc::c_uint,
    0x7bbb3bdb as libc::c_int as uint32_t,
    0x70b632d5 as libc::c_int as uint32_t,
    0x6da129c7 as libc::c_int as uint32_t,
    0x66ac20c9 as libc::c_int as uint32_t,
    0x578f1fe3 as libc::c_int as uint32_t,
    0x5c8216ed as libc::c_int as uint32_t,
    0x41950dff as libc::c_int as uint32_t,
    0x4a9804f1 as libc::c_int as uint32_t,
    0x23d373ab as libc::c_int as uint32_t,
    0x28de7aa5 as libc::c_int as uint32_t,
    0x35c961b7 as libc::c_int as uint32_t,
    0x3ec468b9 as libc::c_int as uint32_t,
    0xfe75793 as libc::c_int as uint32_t,
    0x4ea5e9d as libc::c_int as uint32_t,
    0x19fd458f as libc::c_int as uint32_t,
    0x12f04c81 as libc::c_int as uint32_t,
    0xcb6bab3b as libc::c_uint,
    0xc066a235 as libc::c_uint,
    0xdd71b927 as libc::c_uint,
    0xd67cb029 as libc::c_uint,
    0xe75f8f03 as libc::c_uint,
    0xec52860d as libc::c_uint,
    0xf1459d1f as libc::c_uint,
    0xfa489411 as libc::c_uint,
    0x9303e34b as libc::c_uint,
    0x980eea45 as libc::c_uint,
    0x8519f157 as libc::c_uint,
    0x8e14f859 as libc::c_uint,
    0xbf37c773 as libc::c_uint,
    0xb43ace7d as libc::c_uint,
    0xa92dd56f as libc::c_uint,
    0xa220dc61 as libc::c_uint,
    0xf66d76ad as libc::c_uint,
    0xfd607fa3 as libc::c_uint,
    0xe07764b1 as libc::c_uint,
    0xeb7a6dbf as libc::c_uint,
    0xda595295 as libc::c_uint,
    0xd1545b9b as libc::c_uint,
    0xcc434089 as libc::c_uint,
    0xc74e4987 as libc::c_uint,
    0xae053edd as libc::c_uint,
    0xa50837d3 as libc::c_uint,
    0xb81f2cc1 as libc::c_uint,
    0xb31225cf as libc::c_uint,
    0x82311ae5 as libc::c_uint,
    0x893c13eb as libc::c_uint,
    0x942b08f9 as libc::c_uint,
    0x9f2601f7 as libc::c_uint,
    0x46bde64d as libc::c_int as uint32_t,
    0x4db0ef43 as libc::c_int as uint32_t,
    0x50a7f451 as libc::c_int as uint32_t,
    0x5baafd5f as libc::c_int as uint32_t,
    0x6a89c275 as libc::c_int as uint32_t,
    0x6184cb7b as libc::c_int as uint32_t,
    0x7c93d069 as libc::c_int as uint32_t,
    0x779ed967 as libc::c_int as uint32_t,
    0x1ed5ae3d as libc::c_int as uint32_t,
    0x15d8a733 as libc::c_int as uint32_t,
    0x8cfbc21 as libc::c_int as uint32_t,
    0x3c2b52f as libc::c_int as uint32_t,
    0x32e18a05 as libc::c_int as uint32_t,
    0x39ec830b as libc::c_int as uint32_t,
    0x24fb9819 as libc::c_int as uint32_t,
    0x2ff69117 as libc::c_int as uint32_t,
    0x8dd64d76 as libc::c_uint,
    0x86db4478 as libc::c_uint,
    0x9bcc5f6a as libc::c_uint,
    0x90c15664 as libc::c_uint,
    0xa1e2694e as libc::c_uint,
    0xaaef6040 as libc::c_uint,
    0xb7f87b52 as libc::c_uint,
    0xbcf5725c as libc::c_uint,
    0xd5be0506 as libc::c_uint,
    0xdeb30c08 as libc::c_uint,
    0xc3a4171a as libc::c_uint,
    0xc8a91e14 as libc::c_uint,
    0xf98a213e as libc::c_uint,
    0xf2872830 as libc::c_uint,
    0xef903322 as libc::c_uint,
    0xe49d3a2c as libc::c_uint,
    0x3d06dd96 as libc::c_int as uint32_t,
    0x360bd498 as libc::c_int as uint32_t,
    0x2b1ccf8a as libc::c_int as uint32_t,
    0x2011c684 as libc::c_int as uint32_t,
    0x1132f9ae as libc::c_int as uint32_t,
    0x1a3ff0a0 as libc::c_int as uint32_t,
    0x728ebb2 as libc::c_int as uint32_t,
    0xc25e2bc as libc::c_int as uint32_t,
    0x656e95e6 as libc::c_int as uint32_t,
    0x6e639ce8 as libc::c_int as uint32_t,
    0x737487fa as libc::c_int as uint32_t,
    0x78798ef4 as libc::c_int as uint32_t,
    0x495ab1de as libc::c_int as uint32_t,
    0x4257b8d0 as libc::c_int as uint32_t,
    0x5f40a3c2 as libc::c_int as uint32_t,
    0x544daacc as libc::c_int as uint32_t,
    0xf7daec41 as libc::c_uint,
    0xfcd7e54f as libc::c_uint,
    0xe1c0fe5d as libc::c_uint,
    0xeacdf753 as libc::c_uint,
    0xdbeec879 as libc::c_uint,
    0xd0e3c177 as libc::c_uint,
    0xcdf4da65 as libc::c_uint,
    0xc6f9d36b as libc::c_uint,
    0xafb2a431 as libc::c_uint,
    0xa4bfad3f as libc::c_uint,
    0xb9a8b62d as libc::c_uint,
    0xb2a5bf23 as libc::c_uint,
    0x83868009 as libc::c_uint,
    0x888b8907 as libc::c_uint,
    0x959c9215 as libc::c_uint,
    0x9e919b1b as libc::c_uint,
    0x470a7ca1 as libc::c_int as uint32_t,
    0x4c0775af as libc::c_int as uint32_t,
    0x51106ebd as libc::c_int as uint32_t,
    0x5a1d67b3 as libc::c_int as uint32_t,
    0x6b3e5899 as libc::c_int as uint32_t,
    0x60335197 as libc::c_int as uint32_t,
    0x7d244a85 as libc::c_int as uint32_t,
    0x7629438b as libc::c_int as uint32_t,
    0x1f6234d1 as libc::c_int as uint32_t,
    0x146f3ddf as libc::c_int as uint32_t,
    0x97826cd as libc::c_int as uint32_t,
    0x2752fc3 as libc::c_int as uint32_t,
    0x335610e9 as libc::c_int as uint32_t,
    0x385b19e7 as libc::c_int as uint32_t,
    0x254c02f5 as libc::c_int as uint32_t,
    0x2e410bfb as libc::c_int as uint32_t,
    0x8c61d79a as libc::c_uint,
    0x876cde94 as libc::c_uint,
    0x9a7bc586 as libc::c_uint,
    0x9176cc88 as libc::c_uint,
    0xa055f3a2 as libc::c_uint,
    0xab58faac as libc::c_uint,
    0xb64fe1be as libc::c_uint,
    0xbd42e8b0 as libc::c_uint,
    0xd4099fea as libc::c_uint,
    0xdf0496e4 as libc::c_uint,
    0xc2138df6 as libc::c_uint,
    0xc91e84f8 as libc::c_uint,
    0xf83dbbd2 as libc::c_uint,
    0xf330b2dc as libc::c_uint,
    0xee27a9ce as libc::c_uint,
    0xe52aa0c0 as libc::c_uint,
    0x3cb1477a as libc::c_int as uint32_t,
    0x37bc4e74 as libc::c_int as uint32_t,
    0x2aab5566 as libc::c_int as uint32_t,
    0x21a65c68 as libc::c_int as uint32_t,
    0x10856342 as libc::c_int as uint32_t,
    0x1b886a4c as libc::c_int as uint32_t,
    0x69f715e as libc::c_int as uint32_t,
    0xd927850 as libc::c_int as uint32_t,
    0x64d90f0a as libc::c_int as uint32_t,
    0x6fd40604 as libc::c_int as uint32_t,
    0x72c31d16 as libc::c_int as uint32_t,
    0x79ce1418 as libc::c_int as uint32_t,
    0x48ed2b32 as libc::c_int as uint32_t,
    0x43e0223c as libc::c_int as uint32_t,
    0x5ef7392e as libc::c_int as uint32_t,
    0x55fa3020 as libc::c_int as uint32_t,
    0x1b79aec as libc::c_int as uint32_t,
    0xaba93e2 as libc::c_int as uint32_t,
    0x17ad88f0 as libc::c_int as uint32_t,
    0x1ca081fe as libc::c_int as uint32_t,
    0x2d83bed4 as libc::c_int as uint32_t,
    0x268eb7da as libc::c_int as uint32_t,
    0x3b99acc8 as libc::c_int as uint32_t,
    0x3094a5c6 as libc::c_int as uint32_t,
    0x59dfd29c as libc::c_int as uint32_t,
    0x52d2db92 as libc::c_int as uint32_t,
    0x4fc5c080 as libc::c_int as uint32_t,
    0x44c8c98e as libc::c_int as uint32_t,
    0x75ebf6a4 as libc::c_int as uint32_t,
    0x7ee6ffaa as libc::c_int as uint32_t,
    0x63f1e4b8 as libc::c_int as uint32_t,
    0x68fcedb6 as libc::c_int as uint32_t,
    0xb1670a0c as libc::c_uint,
    0xba6a0302 as libc::c_uint,
    0xa77d1810 as libc::c_uint,
    0xac70111e as libc::c_uint,
    0x9d532e34 as libc::c_uint,
    0x965e273a as libc::c_uint,
    0x8b493c28 as libc::c_uint,
    0x80443526 as libc::c_uint,
    0xe90f427c as libc::c_uint,
    0xe2024b72 as libc::c_uint,
    0xff155060 as libc::c_uint,
    0xf418596e as libc::c_uint,
    0xc53b6644 as libc::c_uint,
    0xce366f4a as libc::c_uint,
    0xd3217458 as libc::c_uint,
    0xd82c7d56 as libc::c_uint,
    0x7a0ca137 as libc::c_int as uint32_t,
    0x7101a839 as libc::c_int as uint32_t,
    0x6c16b32b as libc::c_int as uint32_t,
    0x671bba25 as libc::c_int as uint32_t,
    0x5638850f as libc::c_int as uint32_t,
    0x5d358c01 as libc::c_int as uint32_t,
    0x40229713 as libc::c_int as uint32_t,
    0x4b2f9e1d as libc::c_int as uint32_t,
    0x2264e947 as libc::c_int as uint32_t,
    0x2969e049 as libc::c_int as uint32_t,
    0x347efb5b as libc::c_int as uint32_t,
    0x3f73f255 as libc::c_int as uint32_t,
    0xe50cd7f as libc::c_int as uint32_t,
    0x55dc471 as libc::c_int as uint32_t,
    0x184adf63 as libc::c_int as uint32_t,
    0x1347d66d as libc::c_int as uint32_t,
    0xcadc31d7 as libc::c_uint,
    0xc1d138d9 as libc::c_uint,
    0xdcc623cb as libc::c_uint,
    0xd7cb2ac5 as libc::c_uint,
    0xe6e815ef as libc::c_uint,
    0xede51ce1 as libc::c_uint,
    0xf0f207f3 as libc::c_uint,
    0xfbff0efd as libc::c_uint,
    0x92b479a7 as libc::c_uint,
    0x99b970a9 as libc::c_uint,
    0x84ae6bbb as libc::c_uint,
    0x8fa362b5 as libc::c_uint,
    0xbe805d9f as libc::c_uint,
    0xb58d5491 as libc::c_uint,
    0xa89a4f83 as libc::c_uint,
    0xa397468d as libc::c_uint,
];
pub unsafe extern "C" fn _nettle_aes_invert(
    mut rounds: libc::c_uint,
    mut dst: *mut uint32_t,
    mut src: *const uint32_t,
) {
    let mut i: libc::c_uint = 0;
    if src == dst as *const uint32_t {
        let mut j: libc::c_uint = 0;
        let mut k: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        j = rounds.wrapping_mul(4 as libc::c_int as libc::c_uint);
        while i < j {
            k = 0 as libc::c_int as libc::c_uint;
            while k < 4 as libc::c_int as libc::c_uint {
                let mut t_swap: uint32_t = *dst.offset(i.wrapping_add(k) as isize);
                *dst
                    .offset(
                        i.wrapping_add(k) as isize,
                    ) = *dst.offset(j.wrapping_add(k) as isize);
                *dst.offset(j.wrapping_add(k) as isize) = t_swap;
                k = k.wrapping_add(1);
                k;
            }
            i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
            j = j.wrapping_sub(4 as libc::c_int as libc::c_uint);
        }
    } else {
        let mut k_0: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i <= rounds.wrapping_mul(4 as libc::c_int as libc::c_uint) {
            k_0 = 0 as libc::c_int as libc::c_uint;
            while k_0 < 4 as libc::c_int as libc::c_uint {
                *dst
                    .offset(
                        i.wrapping_add(k_0) as isize,
                    ) = *src
                    .offset(
                        rounds
                            .wrapping_mul(4 as libc::c_int as libc::c_uint)
                            .wrapping_sub(i)
                            .wrapping_add(k_0) as isize,
                    );
                k_0 = k_0.wrapping_add(1);
                k_0;
            }
            i = i.wrapping_add(4 as libc::c_int as libc::c_uint);
        }
    }
    i = 4 as libc::c_int as libc::c_uint;
    while i < (4 as libc::c_int as libc::c_uint).wrapping_mul(rounds) {
        let mut _k: uint32_t = 0;
        let mut _nk: uint32_t = 0;
        let mut _t: uint32_t = 0;
        _k = *dst.offset(i as isize);
        _nk = mtable[(_k & 0xff as libc::c_int as libc::c_uint) as usize];
        _k >>= 8 as libc::c_int;
        _t = mtable[(_k & 0xff as libc::c_int as libc::c_uint) as usize];
        _nk ^= _t << 8 as libc::c_int | _t >> (-(8 as libc::c_int) & 31 as libc::c_int);
        _k >>= 8 as libc::c_int;
        _t = mtable[(_k & 0xff as libc::c_int as libc::c_uint) as usize];
        _nk
            ^= _t << 16 as libc::c_int
                | _t >> (-(16 as libc::c_int) & 31 as libc::c_int);
        _k >>= 8 as libc::c_int;
        _t = mtable[(_k & 0xff as libc::c_int as libc::c_uint) as usize];
        _nk
            ^= _t << 24 as libc::c_int
                | _t >> (-(24 as libc::c_int) & 31 as libc::c_int);
        *dst.offset(i as isize) = _nk;
        i = i.wrapping_add(1);
        i;
    }
}
