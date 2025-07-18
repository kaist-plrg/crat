use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cast128_ctx {
    pub rounds: libc::c_uint,
    pub Kr: [libc::c_uchar; 16],
    pub Km: [uint32_t; 16],
}
static mut cast_sbox1: [uint32_t; 256] = [
    0x30fb40d4 as libc::c_int as uint32_t,
    0x9fa0ff0b as libc::c_uint,
    0x6beccd2f as libc::c_int as uint32_t,
    0x3f258c7a as libc::c_int as uint32_t,
    0x1e213f2f as libc::c_int as uint32_t,
    0x9c004dd3 as libc::c_uint,
    0x6003e540 as libc::c_int as uint32_t,
    0xcf9fc949 as libc::c_uint,
    0xbfd4af27 as libc::c_uint,
    0x88bbbdb5 as libc::c_uint,
    0xe2034090 as libc::c_uint,
    0x98d09675 as libc::c_uint,
    0x6e63a0e0 as libc::c_int as uint32_t,
    0x15c361d2 as libc::c_int as uint32_t,
    0xc2e7661d as libc::c_uint,
    0x22d4ff8e as libc::c_int as uint32_t,
    0x28683b6f as libc::c_int as uint32_t,
    0xc07fd059 as libc::c_uint,
    0xff2379c8 as libc::c_uint,
    0x775f50e2 as libc::c_int as uint32_t,
    0x43c340d3 as libc::c_int as uint32_t,
    0xdf2f8656 as libc::c_uint,
    0x887ca41a as libc::c_uint,
    0xa2d2bd2d as libc::c_uint,
    0xa1c9e0d6 as libc::c_uint,
    0x346c4819 as libc::c_int as uint32_t,
    0x61b76d87 as libc::c_int as uint32_t,
    0x22540f2f as libc::c_int as uint32_t,
    0x2abe32e1 as libc::c_int as uint32_t,
    0xaa54166b as libc::c_uint,
    0x22568e3a as libc::c_int as uint32_t,
    0xa2d341d0 as libc::c_uint,
    0x66db40c8 as libc::c_int as uint32_t,
    0xa784392f as libc::c_uint,
    0x4dff2f as libc::c_int as uint32_t,
    0x2db9d2de as libc::c_int as uint32_t,
    0x97943fac as libc::c_uint,
    0x4a97c1d8 as libc::c_int as uint32_t,
    0x527644b7 as libc::c_int as uint32_t,
    0xb5f437a7 as libc::c_uint,
    0xb82cbaef as libc::c_uint,
    0xd751d159 as libc::c_uint,
    0x6ff7f0ed as libc::c_int as uint32_t,
    0x5a097a1f as libc::c_int as uint32_t,
    0x827b68d0 as libc::c_uint,
    0x90ecf52e as libc::c_uint,
    0x22b0c054 as libc::c_int as uint32_t,
    0xbc8e5935 as libc::c_uint,
    0x4b6d2f7f as libc::c_int as uint32_t,
    0x50bb64a2 as libc::c_int as uint32_t,
    0xd2664910 as libc::c_uint,
    0xbee5812d as libc::c_uint,
    0xb7332290 as libc::c_uint,
    0xe93b159f as libc::c_uint,
    0xb48ee411 as libc::c_uint,
    0x4bff345d as libc::c_int as uint32_t,
    0xfd45c240 as libc::c_uint,
    0xad31973f as libc::c_uint,
    0xc4f6d02e as libc::c_uint,
    0x55fc8165 as libc::c_int as uint32_t,
    0xd5b1caad as libc::c_uint,
    0xa1ac2dae as libc::c_uint,
    0xa2d4b76d as libc::c_uint,
    0xc19b0c50 as libc::c_uint,
    0x882240f2 as libc::c_uint,
    0xc6e4f38 as libc::c_int as uint32_t,
    0xa4e4bfd7 as libc::c_uint,
    0x4f5ba272 as libc::c_int as uint32_t,
    0x564c1d2f as libc::c_int as uint32_t,
    0xc59c5319 as libc::c_uint,
    0xb949e354 as libc::c_uint,
    0xb04669fe as libc::c_uint,
    0xb1b6ab8a as libc::c_uint,
    0xc71358dd as libc::c_uint,
    0x6385c545 as libc::c_int as uint32_t,
    0x110f935d as libc::c_int as uint32_t,
    0x57538ad5 as libc::c_int as uint32_t,
    0x6a390493 as libc::c_int as uint32_t,
    0xe63d37e0 as libc::c_uint,
    0x2a54f6b3 as libc::c_int as uint32_t,
    0x3a787d5f as libc::c_int as uint32_t,
    0x6276a0b5 as libc::c_int as uint32_t,
    0x19a6fcdf as libc::c_int as uint32_t,
    0x7a42206a as libc::c_int as uint32_t,
    0x29f9d4d5 as libc::c_int as uint32_t,
    0xf61b1891 as libc::c_uint,
    0xbb72275e as libc::c_uint,
    0xaa508167 as libc::c_uint,
    0x38901091 as libc::c_int as uint32_t,
    0xc6b505eb as libc::c_uint,
    0x84c7cb8c as libc::c_uint,
    0x2ad75a0f as libc::c_int as uint32_t,
    0x874a1427 as libc::c_uint,
    0xa2d1936b as libc::c_uint,
    0x2ad286af as libc::c_int as uint32_t,
    0xaa56d291 as libc::c_uint,
    0xd7894360 as libc::c_uint,
    0x425c750d as libc::c_int as uint32_t,
    0x93b39e26 as libc::c_uint,
    0x187184c9 as libc::c_int as uint32_t,
    0x6c00b32d as libc::c_int as uint32_t,
    0x73e2bb14 as libc::c_int as uint32_t,
    0xa0bebc3c as libc::c_uint,
    0x54623779 as libc::c_int as uint32_t,
    0x64459eab as libc::c_int as uint32_t,
    0x3f328b82 as libc::c_int as uint32_t,
    0x7718cf82 as libc::c_int as uint32_t,
    0x59a2cea6 as libc::c_int as uint32_t,
    0x4ee002e as libc::c_int as uint32_t,
    0x89fe78e6 as libc::c_uint,
    0x3fab0950 as libc::c_int as uint32_t,
    0x325ff6c2 as libc::c_int as uint32_t,
    0x81383f05 as libc::c_uint,
    0x6963c5c8 as libc::c_int as uint32_t,
    0x76cb5ad6 as libc::c_int as uint32_t,
    0xd49974c9 as libc::c_uint,
    0xca180dcf as libc::c_uint,
    0x380782d5 as libc::c_int as uint32_t,
    0xc7fa5cf6 as libc::c_uint,
    0x8ac31511 as libc::c_uint,
    0x35e79e13 as libc::c_int as uint32_t,
    0x47da91d0 as libc::c_int as uint32_t,
    0xf40f9086 as libc::c_uint,
    0xa7e2419e as libc::c_uint,
    0x31366241 as libc::c_int as uint32_t,
    0x51ef495 as libc::c_int as uint32_t,
    0xaa573b04 as libc::c_uint,
    0x4a805d8d as libc::c_int as uint32_t,
    0x548300d0 as libc::c_int as uint32_t,
    0x322a3c as libc::c_int as uint32_t,
    0xbf64cddf as libc::c_uint,
    0xba57a68e as libc::c_uint,
    0x75c6372b as libc::c_int as uint32_t,
    0x50afd341 as libc::c_int as uint32_t,
    0xa7c13275 as libc::c_uint,
    0x915a0bf5 as libc::c_uint,
    0x6b54bfab as libc::c_int as uint32_t,
    0x2b0b1426 as libc::c_int as uint32_t,
    0xab4cc9d7 as libc::c_uint,
    0x449ccd82 as libc::c_int as uint32_t,
    0xf7fbf265 as libc::c_uint,
    0xab85c5f3 as libc::c_uint,
    0x1b55db94 as libc::c_int as uint32_t,
    0xaad4e324 as libc::c_uint,
    0xcfa4bd3f as libc::c_uint,
    0x2deaa3e2 as libc::c_int as uint32_t,
    0x9e204d02 as libc::c_uint,
    0xc8bd25ac as libc::c_uint,
    0xeadf55b3 as libc::c_uint,
    0xd5bd9e98 as libc::c_uint,
    0xe31231b2 as libc::c_uint,
    0x2ad5ad6c as libc::c_int as uint32_t,
    0x954329de as libc::c_uint,
    0xadbe4528 as libc::c_uint,
    0xd8710f69 as libc::c_uint,
    0xaa51c90f as libc::c_uint,
    0xaa786bf6 as libc::c_uint,
    0x22513f1e as libc::c_int as uint32_t,
    0xaa51a79b as libc::c_uint,
    0x2ad344cc as libc::c_int as uint32_t,
    0x7b5a41f0 as libc::c_int as uint32_t,
    0xd37cfbad as libc::c_uint,
    0x1b069505 as libc::c_int as uint32_t,
    0x41ece491 as libc::c_int as uint32_t,
    0xb4c332e6 as libc::c_uint,
    0x32268d4 as libc::c_int as uint32_t,
    0xc9600acc as libc::c_uint,
    0xce387e6d as libc::c_uint,
    0xbf6bb16c as libc::c_uint,
    0x6a70fb78 as libc::c_int as uint32_t,
    0xd03d9c9 as libc::c_int as uint32_t,
    0xd4df39de as libc::c_uint,
    0xe01063da as libc::c_uint,
    0x4736f464 as libc::c_int as uint32_t,
    0x5ad328d8 as libc::c_int as uint32_t,
    0xb347cc96 as libc::c_uint,
    0x75bb0fc3 as libc::c_int as uint32_t,
    0x98511bfb as libc::c_uint,
    0x4ffbcc35 as libc::c_int as uint32_t,
    0xb58bcf6a as libc::c_uint,
    0xe11f0abc as libc::c_uint,
    0xbfc5fe4a as libc::c_uint,
    0xa70aec10 as libc::c_uint,
    0xac39570a as libc::c_uint,
    0x3f04442f as libc::c_int as uint32_t,
    0x6188b153 as libc::c_int as uint32_t,
    0xe0397a2e as libc::c_uint,
    0x5727cb79 as libc::c_int as uint32_t,
    0x9ceb418f as libc::c_uint,
    0x1cacd68d as libc::c_int as uint32_t,
    0x2ad37c96 as libc::c_int as uint32_t,
    0x175cb9d as libc::c_int as uint32_t,
    0xc69dff09 as libc::c_uint,
    0xc75b65f0 as libc::c_uint,
    0xd9db40d8 as libc::c_uint,
    0xec0e7779 as libc::c_uint,
    0x4744ead4 as libc::c_int as uint32_t,
    0xb11c3274 as libc::c_uint,
    0xdd24cb9e as libc::c_uint,
    0x7e1c54bd as libc::c_int as uint32_t,
    0xf01144f9 as libc::c_uint,
    0xd2240eb1 as libc::c_uint,
    0x9675b3fd as libc::c_uint,
    0xa3ac3755 as libc::c_uint,
    0xd47c27af as libc::c_uint,
    0x51c85f4d as libc::c_int as uint32_t,
    0x56907596 as libc::c_int as uint32_t,
    0xa5bb15e6 as libc::c_uint,
    0x580304f0 as libc::c_int as uint32_t,
    0xca042cf1 as libc::c_uint,
    0x11a37ea as libc::c_int as uint32_t,
    0x8dbfaadb as libc::c_uint,
    0x35ba3e4a as libc::c_int as uint32_t,
    0x3526ffa0 as libc::c_int as uint32_t,
    0xc37b4d09 as libc::c_uint,
    0xbc306ed9 as libc::c_uint,
    0x98a52666 as libc::c_uint,
    0x5648f725 as libc::c_int as uint32_t,
    0xff5e569d as libc::c_uint,
    0xced63d0 as libc::c_int as uint32_t,
    0x7c63b2cf as libc::c_int as uint32_t,
    0x700b45e1 as libc::c_int as uint32_t,
    0xd5ea50f1 as libc::c_uint,
    0x85a92872 as libc::c_uint,
    0xaf1fbda7 as libc::c_uint,
    0xd4234870 as libc::c_uint,
    0xa7870bf3 as libc::c_uint,
    0x2d3b4d79 as libc::c_int as uint32_t,
    0x42e04198 as libc::c_int as uint32_t,
    0xcd0ede7 as libc::c_int as uint32_t,
    0x26470db8 as libc::c_int as uint32_t,
    0xf881814c as libc::c_uint,
    0x474d6ad7 as libc::c_int as uint32_t,
    0x7c0c5e5c as libc::c_int as uint32_t,
    0xd1231959 as libc::c_uint,
    0x381b7298 as libc::c_int as uint32_t,
    0xf5d2f4db as libc::c_uint,
    0xab838653 as libc::c_uint,
    0x6e2f1e23 as libc::c_int as uint32_t,
    0x83719c9e as libc::c_uint,
    0xbd91e046 as libc::c_uint,
    0x9a56456e as libc::c_uint,
    0xdc39200c as libc::c_uint,
    0x20c8c571 as libc::c_int as uint32_t,
    0x962bda1c as libc::c_uint,
    0xe1e696ff as libc::c_uint,
    0xb141ab08 as libc::c_uint,
    0x7cca89b9 as libc::c_int as uint32_t,
    0x1a69e783 as libc::c_int as uint32_t,
    0x2cc4843 as libc::c_int as uint32_t,
    0xa2f7c579 as libc::c_uint,
    0x429ef47d as libc::c_int as uint32_t,
    0x427b169c as libc::c_int as uint32_t,
    0x5ac9f049 as libc::c_int as uint32_t,
    0xdd8f0f00 as libc::c_uint,
    0x5c8165bf as libc::c_int as uint32_t,
];
static mut cast_sbox2: [uint32_t; 256] = [
    0x1f201094 as libc::c_int as uint32_t,
    0xef0ba75b as libc::c_uint,
    0x69e3cf7e as libc::c_int as uint32_t,
    0x393f4380 as libc::c_int as uint32_t,
    0xfe61cf7a as libc::c_uint,
    0xeec5207a as libc::c_uint,
    0x55889c94 as libc::c_int as uint32_t,
    0x72fc0651 as libc::c_int as uint32_t,
    0xada7ef79 as libc::c_uint,
    0x4e1d7235 as libc::c_int as uint32_t,
    0xd55a63ce as libc::c_uint,
    0xde0436ba as libc::c_uint,
    0x99c430ef as libc::c_uint,
    0x5f0c0794 as libc::c_int as uint32_t,
    0x18dcdb7d as libc::c_int as uint32_t,
    0xa1d6eff3 as libc::c_uint,
    0xa0b52f7b as libc::c_uint,
    0x59e83605 as libc::c_int as uint32_t,
    0xee15b094 as libc::c_uint,
    0xe9ffd909 as libc::c_uint,
    0xdc440086 as libc::c_uint,
    0xef944459 as libc::c_uint,
    0xba83ccb3 as libc::c_uint,
    0xe0c3cdfb as libc::c_uint,
    0xd1da4181 as libc::c_uint,
    0x3b092ab1 as libc::c_int as uint32_t,
    0xf997f1c1 as libc::c_uint,
    0xa5e6cf7b as libc::c_uint,
    0x1420ddb as libc::c_int as uint32_t,
    0xe4e7ef5b as libc::c_uint,
    0x25a1ff41 as libc::c_int as uint32_t,
    0xe180f806 as libc::c_uint,
    0x1fc41080 as libc::c_int as uint32_t,
    0x179bee7a as libc::c_int as uint32_t,
    0xd37ac6a9 as libc::c_uint,
    0xfe5830a4 as libc::c_uint,
    0x98de8b7f as libc::c_uint,
    0x77e83f4e as libc::c_int as uint32_t,
    0x79929269 as libc::c_int as uint32_t,
    0x24fa9f7b as libc::c_int as uint32_t,
    0xe113c85b as libc::c_uint,
    0xacc40083 as libc::c_uint,
    0xd7503525 as libc::c_uint,
    0xf7ea615f as libc::c_uint,
    0x62143154 as libc::c_int as uint32_t,
    0xd554b63 as libc::c_int as uint32_t,
    0x5d681121 as libc::c_int as uint32_t,
    0xc866c359 as libc::c_uint,
    0x3d63cf73 as libc::c_int as uint32_t,
    0xcee234c0 as libc::c_uint,
    0xd4d87e87 as libc::c_uint,
    0x5c672b21 as libc::c_int as uint32_t,
    0x71f6181 as libc::c_int as uint32_t,
    0x39f7627f as libc::c_int as uint32_t,
    0x361e3084 as libc::c_int as uint32_t,
    0xe4eb573b as libc::c_uint,
    0x602f64a4 as libc::c_int as uint32_t,
    0xd63acd9c as libc::c_uint,
    0x1bbc4635 as libc::c_int as uint32_t,
    0x9e81032d as libc::c_uint,
    0x2701f50c as libc::c_int as uint32_t,
    0x99847ab4 as libc::c_uint,
    0xa0e3df79 as libc::c_uint,
    0xba6cf38c as libc::c_uint,
    0x10843094 as libc::c_int as uint32_t,
    0x2537a95e as libc::c_int as uint32_t,
    0xf46f6ffe as libc::c_uint,
    0xa1ff3b1f as libc::c_uint,
    0x208cfb6a as libc::c_int as uint32_t,
    0x8f458c74 as libc::c_uint,
    0xd9e0a227 as libc::c_uint,
    0x4ec73a34 as libc::c_int as uint32_t,
    0xfc884f69 as libc::c_uint,
    0x3e4de8df as libc::c_int as uint32_t,
    0xef0e0088 as libc::c_uint,
    0x3559648d as libc::c_int as uint32_t,
    0x8a45388c as libc::c_uint,
    0x1d804366 as libc::c_int as uint32_t,
    0x721d9bfd as libc::c_int as uint32_t,
    0xa58684bb as libc::c_uint,
    0xe8256333 as libc::c_uint,
    0x844e8212 as libc::c_uint,
    0x128d8098 as libc::c_int as uint32_t,
    0xfed33fb4 as libc::c_uint,
    0xce280ae1 as libc::c_uint,
    0x27e19ba5 as libc::c_int as uint32_t,
    0xd5a6c252 as libc::c_uint,
    0xe49754bd as libc::c_uint,
    0xc5d655dd as libc::c_uint,
    0xeb667064 as libc::c_uint,
    0x77840b4d as libc::c_int as uint32_t,
    0xa1b6a801 as libc::c_uint,
    0x84db26a9 as libc::c_uint,
    0xe0b56714 as libc::c_uint,
    0x21f043b7 as libc::c_int as uint32_t,
    0xe5d05860 as libc::c_uint,
    0x54f03084 as libc::c_int as uint32_t,
    0x66ff472 as libc::c_int as uint32_t,
    0xa31aa153 as libc::c_uint,
    0xdadc4755 as libc::c_uint,
    0xb5625dbf as libc::c_uint,
    0x68561be6 as libc::c_int as uint32_t,
    0x83ca6b94 as libc::c_uint,
    0x2d6ed23b as libc::c_int as uint32_t,
    0xeccf01db as libc::c_uint,
    0xa6d3d0ba as libc::c_uint,
    0xb6803d5c as libc::c_uint,
    0xaf77a709 as libc::c_uint,
    0x33b4a34c as libc::c_int as uint32_t,
    0x397bc8d6 as libc::c_int as uint32_t,
    0x5ee22b95 as libc::c_int as uint32_t,
    0x5f0e5304 as libc::c_int as uint32_t,
    0x81ed6f61 as libc::c_uint,
    0x20e74364 as libc::c_int as uint32_t,
    0xb45e1378 as libc::c_uint,
    0xde18639b as libc::c_uint,
    0x881ca122 as libc::c_uint,
    0xb96726d1 as libc::c_uint,
    0x8049a7e8 as libc::c_uint,
    0x22b7da7b as libc::c_int as uint32_t,
    0x5e552d25 as libc::c_int as uint32_t,
    0x5272d237 as libc::c_int as uint32_t,
    0x79d2951c as libc::c_int as uint32_t,
    0xc60d894c as libc::c_uint,
    0x488cb402 as libc::c_int as uint32_t,
    0x1ba4fe5b as libc::c_int as uint32_t,
    0xa4b09f6b as libc::c_uint,
    0x1ca815cf as libc::c_int as uint32_t,
    0xa20c3005 as libc::c_uint,
    0x8871df63 as libc::c_uint,
    0xb9de2fcb as libc::c_uint,
    0xcc6c9e9 as libc::c_int as uint32_t,
    0xbeeff53 as libc::c_int as uint32_t,
    0xe3214517 as libc::c_uint,
    0xb4542835 as libc::c_uint,
    0x9f63293c as libc::c_uint,
    0xee41e729 as libc::c_uint,
    0x6e1d2d7c as libc::c_int as uint32_t,
    0x50045286 as libc::c_int as uint32_t,
    0x1e6685f3 as libc::c_int as uint32_t,
    0xf33401c6 as libc::c_uint,
    0x30a22c95 as libc::c_int as uint32_t,
    0x31a70850 as libc::c_int as uint32_t,
    0x60930f13 as libc::c_int as uint32_t,
    0x73f98417 as libc::c_int as uint32_t,
    0xa1269859 as libc::c_uint,
    0xec645c44 as libc::c_uint,
    0x52c877a9 as libc::c_int as uint32_t,
    0xcdff33a6 as libc::c_uint,
    0xa02b1741 as libc::c_uint,
    0x7cbad9a2 as libc::c_int as uint32_t,
    0x2180036f as libc::c_int as uint32_t,
    0x50d99c08 as libc::c_int as uint32_t,
    0xcb3f4861 as libc::c_uint,
    0xc26bd765 as libc::c_uint,
    0x64a3f6ab as libc::c_int as uint32_t,
    0x80342676 as libc::c_uint,
    0x25a75e7b as libc::c_int as uint32_t,
    0xe4e6d1fc as libc::c_uint,
    0x20c710e6 as libc::c_int as uint32_t,
    0xcdf0b680 as libc::c_uint,
    0x17844d3b as libc::c_int as uint32_t,
    0x31eef84d as libc::c_int as uint32_t,
    0x7e0824e4 as libc::c_int as uint32_t,
    0x2ccb49eb as libc::c_int as uint32_t,
    0x846a3bae as libc::c_uint,
    0x8ff77888 as libc::c_uint,
    0xee5d60f6 as libc::c_uint,
    0x7af75673 as libc::c_int as uint32_t,
    0x2fdd5cdb as libc::c_int as uint32_t,
    0xa11631c1 as libc::c_uint,
    0x30f66f43 as libc::c_int as uint32_t,
    0xb3faec54 as libc::c_uint,
    0x157fd7fa as libc::c_int as uint32_t,
    0xef8579cc as libc::c_uint,
    0xd152de58 as libc::c_uint,
    0xdb2ffd5e as libc::c_uint,
    0x8f32ce19 as libc::c_uint,
    0x306af97a as libc::c_int as uint32_t,
    0x2f03ef8 as libc::c_int as uint32_t,
    0x99319ad5 as libc::c_uint,
    0xc242fa0f as libc::c_uint,
    0xa7e3ebb0 as libc::c_uint,
    0xc68e4906 as libc::c_uint,
    0xb8da230c as libc::c_uint,
    0x80823028 as libc::c_uint,
    0xdcdef3c8 as libc::c_uint,
    0xd35fb171 as libc::c_uint,
    0x88a1bc8 as libc::c_int as uint32_t,
    0xbec0c560 as libc::c_uint,
    0x61a3c9e8 as libc::c_int as uint32_t,
    0xbca8f54d as libc::c_uint,
    0xc72feffa as libc::c_uint,
    0x22822e99 as libc::c_int as uint32_t,
    0x82c570b4 as libc::c_uint,
    0xd8d94e89 as libc::c_uint,
    0x8b1c34bc as libc::c_uint,
    0x301e16e6 as libc::c_int as uint32_t,
    0x273be979 as libc::c_int as uint32_t,
    0xb0ffeaa6 as libc::c_uint,
    0x61d9b8c6 as libc::c_int as uint32_t,
    0xb24869 as libc::c_int as uint32_t,
    0xb7ffce3f as libc::c_uint,
    0x8dc283b as libc::c_int as uint32_t,
    0x43daf65a as libc::c_int as uint32_t,
    0xf7e19798 as libc::c_uint,
    0x7619b72f as libc::c_int as uint32_t,
    0x8f1c9ba4 as libc::c_uint,
    0xdc8637a0 as libc::c_uint,
    0x16a7d3b1 as libc::c_int as uint32_t,
    0x9fc393b7 as libc::c_uint,
    0xa7136eeb as libc::c_uint,
    0xc6bcc63e as libc::c_uint,
    0x1a513742 as libc::c_int as uint32_t,
    0xef6828bc as libc::c_uint,
    0x520365d6 as libc::c_int as uint32_t,
    0x2d6a77ab as libc::c_int as uint32_t,
    0x3527ed4b as libc::c_int as uint32_t,
    0x821fd216 as libc::c_uint,
    0x95c6e2e as libc::c_int as uint32_t,
    0xdb92f2fb as libc::c_uint,
    0x5eea29cb as libc::c_int as uint32_t,
    0x145892f5 as libc::c_int as uint32_t,
    0x91584f7f as libc::c_uint,
    0x5483697b as libc::c_int as uint32_t,
    0x2667a8cc as libc::c_int as uint32_t,
    0x85196048 as libc::c_uint,
    0x8c4bacea as libc::c_uint,
    0x833860d4 as libc::c_uint,
    0xd23e0f9 as libc::c_int as uint32_t,
    0x6c387e8a as libc::c_int as uint32_t,
    0xae6d249 as libc::c_int as uint32_t,
    0xb284600c as libc::c_uint,
    0xd835731d as libc::c_uint,
    0xdcb1c647 as libc::c_uint,
    0xac4c56ea as libc::c_uint,
    0x3ebd81b3 as libc::c_int as uint32_t,
    0x230eabb0 as libc::c_int as uint32_t,
    0x6438bc87 as libc::c_int as uint32_t,
    0xf0b5b1fa as libc::c_uint,
    0x8f5ea2b3 as libc::c_uint,
    0xfc184642 as libc::c_uint,
    0xa036b7a as libc::c_int as uint32_t,
    0x4fb089bd as libc::c_int as uint32_t,
    0x649da589 as libc::c_int as uint32_t,
    0xa345415e as libc::c_uint,
    0x5c038323 as libc::c_int as uint32_t,
    0x3e5d3bb9 as libc::c_int as uint32_t,
    0x43d79572 as libc::c_int as uint32_t,
    0x7e6dd07c as libc::c_int as uint32_t,
    0x6dfdf1e as libc::c_int as uint32_t,
    0x6c6cc4ef as libc::c_int as uint32_t,
    0x7160a539 as libc::c_int as uint32_t,
    0x73bfbe70 as libc::c_int as uint32_t,
    0x83877605 as libc::c_uint,
    0x4523ecf1 as libc::c_int as uint32_t,
];
static mut cast_sbox3: [uint32_t; 256] = [
    0x8defc240 as libc::c_uint,
    0x25fa5d9f as libc::c_int as uint32_t,
    0xeb903dbf as libc::c_uint,
    0xe810c907 as libc::c_uint,
    0x47607fff as libc::c_int as uint32_t,
    0x369fe44b as libc::c_int as uint32_t,
    0x8c1fc644 as libc::c_uint,
    0xaececa90 as libc::c_uint,
    0xbeb1f9bf as libc::c_uint,
    0xeefbcaea as libc::c_uint,
    0xe8cf1950 as libc::c_uint,
    0x51df07ae as libc::c_int as uint32_t,
    0x920e8806 as libc::c_uint,
    0xf0ad0548 as libc::c_uint,
    0xe13c8d83 as libc::c_uint,
    0x927010d5 as libc::c_uint,
    0x11107d9f as libc::c_int as uint32_t,
    0x7647db9 as libc::c_int as uint32_t,
    0xb2e3e4d4 as libc::c_uint,
    0x3d4f285e as libc::c_int as uint32_t,
    0xb9afa820 as libc::c_uint,
    0xfade82e0 as libc::c_uint,
    0xa067268b as libc::c_uint,
    0x8272792e as libc::c_uint,
    0x553fb2c0 as libc::c_int as uint32_t,
    0x489ae22b as libc::c_int as uint32_t,
    0xd4ef9794 as libc::c_uint,
    0x125e3fbc as libc::c_int as uint32_t,
    0x21fffcee as libc::c_int as uint32_t,
    0x825b1bfd as libc::c_uint,
    0x9255c5ed as libc::c_uint,
    0x1257a240 as libc::c_int as uint32_t,
    0x4e1a8302 as libc::c_int as uint32_t,
    0xbae07fff as libc::c_uint,
    0x528246e7 as libc::c_int as uint32_t,
    0x8e57140e as libc::c_uint,
    0x3373f7bf as libc::c_int as uint32_t,
    0x8c9f8188 as libc::c_uint,
    0xa6fc4ee8 as libc::c_uint,
    0xc982b5a5 as libc::c_uint,
    0xa8c01db7 as libc::c_uint,
    0x579fc264 as libc::c_int as uint32_t,
    0x67094f31 as libc::c_int as uint32_t,
    0xf2bd3f5f as libc::c_uint,
    0x40fff7c1 as libc::c_int as uint32_t,
    0x1fb78dfc as libc::c_int as uint32_t,
    0x8e6bd2c1 as libc::c_uint,
    0x437be59b as libc::c_int as uint32_t,
    0x99b03dbf as libc::c_uint,
    0xb5dbc64b as libc::c_uint,
    0x638dc0e6 as libc::c_int as uint32_t,
    0x55819d99 as libc::c_int as uint32_t,
    0xa197c81c as libc::c_uint,
    0x4a012d6e as libc::c_int as uint32_t,
    0xc5884a28 as libc::c_uint,
    0xccc36f71 as libc::c_uint,
    0xb843c213 as libc::c_uint,
    0x6c0743f1 as libc::c_int as uint32_t,
    0x8309893c as libc::c_uint,
    0xfeddd5f as libc::c_int as uint32_t,
    0x2f7fe850 as libc::c_int as uint32_t,
    0xd7c07f7e as libc::c_uint,
    0x2507fbf as libc::c_int as uint32_t,
    0x5afb9a04 as libc::c_int as uint32_t,
    0xa747d2d0 as libc::c_uint,
    0x1651192e as libc::c_int as uint32_t,
    0xaf70bf3e as libc::c_uint,
    0x58c31380 as libc::c_int as uint32_t,
    0x5f98302e as libc::c_int as uint32_t,
    0x727cc3c4 as libc::c_int as uint32_t,
    0xa0fb402 as libc::c_int as uint32_t,
    0xf7fef82 as libc::c_int as uint32_t,
    0x8c96fdad as libc::c_uint,
    0x5d2c2aae as libc::c_int as uint32_t,
    0x8ee99a49 as libc::c_uint,
    0x50da88b8 as libc::c_int as uint32_t,
    0x8427f4a0 as libc::c_uint,
    0x1eac5790 as libc::c_int as uint32_t,
    0x796fb449 as libc::c_int as uint32_t,
    0x8252dc15 as libc::c_uint,
    0xefbd7d9b as libc::c_uint,
    0xa672597d as libc::c_uint,
    0xada840d8 as libc::c_uint,
    0x45f54504 as libc::c_int as uint32_t,
    0xfa5d7403 as libc::c_uint,
    0xe83ec305 as libc::c_uint,
    0x4f91751a as libc::c_int as uint32_t,
    0x925669c2 as libc::c_uint,
    0x23efe941 as libc::c_int as uint32_t,
    0xa903f12e as libc::c_uint,
    0x60270df2 as libc::c_int as uint32_t,
    0x276e4b6 as libc::c_int as uint32_t,
    0x94fd6574 as libc::c_uint,
    0x927985b2 as libc::c_uint,
    0x8276dbcb as libc::c_uint,
    0x2778176 as libc::c_int as uint32_t,
    0xf8af918d as libc::c_uint,
    0x4e48f79e as libc::c_int as uint32_t,
    0x8f616ddf as libc::c_uint,
    0xe29d840e as libc::c_uint,
    0x842f7d83 as libc::c_uint,
    0x340ce5c8 as libc::c_int as uint32_t,
    0x96bbb682 as libc::c_uint,
    0x93b4b148 as libc::c_uint,
    0xef303cab as libc::c_uint,
    0x984faf28 as libc::c_uint,
    0x779faf9b as libc::c_int as uint32_t,
    0x92dc560d as libc::c_uint,
    0x224d1e20 as libc::c_int as uint32_t,
    0x8437aa88 as libc::c_uint,
    0x7d29dc96 as libc::c_int as uint32_t,
    0x2756d3dc as libc::c_int as uint32_t,
    0x8b907cee as libc::c_uint,
    0xb51fd240 as libc::c_uint,
    0xe7c07ce3 as libc::c_uint,
    0xe566b4a1 as libc::c_uint,
    0xc3e9615e as libc::c_uint,
    0x3cf8209d as libc::c_int as uint32_t,
    0x6094d1e3 as libc::c_int as uint32_t,
    0xcd9ca341 as libc::c_uint,
    0x5c76460e as libc::c_int as uint32_t,
    0xea983b as libc::c_int as uint32_t,
    0xd4d67881 as libc::c_uint,
    0xfd47572c as libc::c_uint,
    0xf76cedd9 as libc::c_uint,
    0xbda8229c as libc::c_uint,
    0x127dadaa as libc::c_int as uint32_t,
    0x438a074e as libc::c_int as uint32_t,
    0x1f97c090 as libc::c_int as uint32_t,
    0x81bdb8a as libc::c_int as uint32_t,
    0x93a07ebe as libc::c_uint,
    0xb938ca15 as libc::c_uint,
    0x97b03cff as libc::c_uint,
    0x3dc2c0f8 as libc::c_int as uint32_t,
    0x8d1ab2ec as libc::c_uint,
    0x64380e51 as libc::c_int as uint32_t,
    0x68cc7bfb as libc::c_int as uint32_t,
    0xd90f2788 as libc::c_uint,
    0x12490181 as libc::c_int as uint32_t,
    0x5de5ffd4 as libc::c_int as uint32_t,
    0xdd7ef86a as libc::c_uint,
    0x76a2e214 as libc::c_int as uint32_t,
    0xb9a40368 as libc::c_uint,
    0x925d958f as libc::c_uint,
    0x4b39fffa as libc::c_int as uint32_t,
    0xba39aee9 as libc::c_uint,
    0xa4ffd30b as libc::c_uint,
    0xfaf7933b as libc::c_uint,
    0x6d498623 as libc::c_int as uint32_t,
    0x193cbcfa as libc::c_int as uint32_t,
    0x27627545 as libc::c_int as uint32_t,
    0x825cf47a as libc::c_uint,
    0x61bd8ba0 as libc::c_int as uint32_t,
    0xd11e42d1 as libc::c_uint,
    0xcead04f4 as libc::c_uint,
    0x127ea392 as libc::c_int as uint32_t,
    0x10428db7 as libc::c_int as uint32_t,
    0x8272a972 as libc::c_uint,
    0x9270c4a8 as libc::c_uint,
    0x127de50b as libc::c_int as uint32_t,
    0x285ba1c8 as libc::c_int as uint32_t,
    0x3c62f44f as libc::c_int as uint32_t,
    0x35c0eaa5 as libc::c_int as uint32_t,
    0xe805d231 as libc::c_uint,
    0x428929fb as libc::c_int as uint32_t,
    0xb4fcdf82 as libc::c_uint,
    0x4fb66a53 as libc::c_int as uint32_t,
    0xe7dc15b as libc::c_int as uint32_t,
    0x1f081fab as libc::c_int as uint32_t,
    0x108618ae as libc::c_int as uint32_t,
    0xfcfd086d as libc::c_uint,
    0xf9ff2889 as libc::c_uint,
    0x694bcc11 as libc::c_int as uint32_t,
    0x236a5cae as libc::c_int as uint32_t,
    0x12deca4d as libc::c_int as uint32_t,
    0x2c3f8cc5 as libc::c_int as uint32_t,
    0xd2d02dfe as libc::c_uint,
    0xf8ef5896 as libc::c_uint,
    0xe4cf52da as libc::c_uint,
    0x95155b67 as libc::c_uint,
    0x494a488c as libc::c_int as uint32_t,
    0xb9b6a80c as libc::c_uint,
    0x5c8f82bc as libc::c_int as uint32_t,
    0x89d36b45 as libc::c_uint,
    0x3a609437 as libc::c_int as uint32_t,
    0xec00c9a9 as libc::c_uint,
    0x44715253 as libc::c_int as uint32_t,
    0xa874b49 as libc::c_int as uint32_t,
    0xd773bc40 as libc::c_uint,
    0x7c34671c as libc::c_int as uint32_t,
    0x2717ef6 as libc::c_int as uint32_t,
    0x4feb5536 as libc::c_int as uint32_t,
    0xa2d02fff as libc::c_uint,
    0xd2bf60c4 as libc::c_uint,
    0xd43f03c0 as libc::c_uint,
    0x50b4ef6d as libc::c_int as uint32_t,
    0x7478cd1 as libc::c_int as uint32_t,
    0x6e1888 as libc::c_int as uint32_t,
    0xa2e53f55 as libc::c_uint,
    0xb9e6d4bc as libc::c_uint,
    0xa2048016 as libc::c_uint,
    0x97573833 as libc::c_uint,
    0xd7207d67 as libc::c_uint,
    0xde0f8f3d as libc::c_uint,
    0x72f87b33 as libc::c_int as uint32_t,
    0xabcc4f33 as libc::c_uint,
    0x7688c55d as libc::c_int as uint32_t,
    0x7b00a6b0 as libc::c_int as uint32_t,
    0x947b0001 as libc::c_uint,
    0x570075d2 as libc::c_int as uint32_t,
    0xf9bb88f8 as libc::c_uint,
    0x8942019e as libc::c_uint,
    0x4264a5ff as libc::c_int as uint32_t,
    0x856302e0 as libc::c_uint,
    0x72dbd92b as libc::c_int as uint32_t,
    0xee971b69 as libc::c_uint,
    0x6ea22fde as libc::c_int as uint32_t,
    0x5f08ae2b as libc::c_int as uint32_t,
    0xaf7a616d as libc::c_uint,
    0xe5c98767 as libc::c_uint,
    0xcf1febd2 as libc::c_uint,
    0x61efc8c2 as libc::c_int as uint32_t,
    0xf1ac2571 as libc::c_uint,
    0xcc8239c2 as libc::c_uint,
    0x67214cb8 as libc::c_int as uint32_t,
    0xb1e583d1 as libc::c_uint,
    0xb7dc3e62 as libc::c_uint,
    0x7f10bdce as libc::c_int as uint32_t,
    0xf90a5c38 as libc::c_uint,
    0xff0443d as libc::c_int as uint32_t,
    0x606e6dc6 as libc::c_int as uint32_t,
    0x60543a49 as libc::c_int as uint32_t,
    0x5727c148 as libc::c_int as uint32_t,
    0x2be98a1d as libc::c_int as uint32_t,
    0x8ab41738 as libc::c_uint,
    0x20e1be24 as libc::c_int as uint32_t,
    0xaf96da0f as libc::c_uint,
    0x68458425 as libc::c_int as uint32_t,
    0x99833be5 as libc::c_uint,
    0x600d457d as libc::c_int as uint32_t,
    0x282f9350 as libc::c_int as uint32_t,
    0x8334b362 as libc::c_uint,
    0xd91d1120 as libc::c_uint,
    0x2b6d8da0 as libc::c_int as uint32_t,
    0x642b1e31 as libc::c_int as uint32_t,
    0x9c305a00 as libc::c_uint,
    0x52bce688 as libc::c_int as uint32_t,
    0x1b03588a as libc::c_int as uint32_t,
    0xf7baefd5 as libc::c_uint,
    0x4142ed9c as libc::c_int as uint32_t,
    0xa4315c11 as libc::c_uint,
    0x83323ec5 as libc::c_uint,
    0xdfef4636 as libc::c_uint,
    0xa133c501 as libc::c_uint,
    0xe9d3531c as libc::c_uint,
    0xee353783 as libc::c_uint,
];
static mut cast_sbox4: [uint32_t; 256] = [
    0x9db30420 as libc::c_uint,
    0x1fb6e9de as libc::c_int as uint32_t,
    0xa7be7bef as libc::c_uint,
    0xd273a298 as libc::c_uint,
    0x4a4f7bdb as libc::c_int as uint32_t,
    0x64ad8c57 as libc::c_int as uint32_t,
    0x85510443 as libc::c_uint,
    0xfa020ed1 as libc::c_uint,
    0x7e287aff as libc::c_int as uint32_t,
    0xe60fb663 as libc::c_uint,
    0x95f35a1 as libc::c_int as uint32_t,
    0x79ebf120 as libc::c_int as uint32_t,
    0xfd059d43 as libc::c_uint,
    0x6497b7b1 as libc::c_int as uint32_t,
    0xf3641f63 as libc::c_uint,
    0x241e4adf as libc::c_int as uint32_t,
    0x28147f5f as libc::c_int as uint32_t,
    0x4fa2b8cd as libc::c_int as uint32_t,
    0xc9430040 as libc::c_uint,
    0xcc32220 as libc::c_int as uint32_t,
    0xfdd30b30 as libc::c_uint,
    0xc0a5374f as libc::c_uint,
    0x1d2d00d9 as libc::c_int as uint32_t,
    0x24147b15 as libc::c_int as uint32_t,
    0xee4d111a as libc::c_uint,
    0xfca5167 as libc::c_int as uint32_t,
    0x71ff904c as libc::c_int as uint32_t,
    0x2d195ffe as libc::c_int as uint32_t,
    0x1a05645f as libc::c_int as uint32_t,
    0xc13fefe as libc::c_int as uint32_t,
    0x81b08ca as libc::c_int as uint32_t,
    0x5170121 as libc::c_int as uint32_t,
    0x80530100 as libc::c_uint,
    0xe83e5efe as libc::c_uint,
    0xac9af4f8 as libc::c_uint,
    0x7fe72701 as libc::c_int as uint32_t,
    0xd2b8ee5f as libc::c_uint,
    0x6df4261 as libc::c_int as uint32_t,
    0xbb9e9b8a as libc::c_uint,
    0x7293ea25 as libc::c_int as uint32_t,
    0xce84ffdf as libc::c_uint,
    0xf5718801 as libc::c_uint,
    0x3dd64b04 as libc::c_int as uint32_t,
    0xa26f263b as libc::c_uint,
    0x7ed48400 as libc::c_int as uint32_t,
    0x547eebe6 as libc::c_int as uint32_t,
    0x446d4ca0 as libc::c_int as uint32_t,
    0x6cf3d6f5 as libc::c_int as uint32_t,
    0x2649abdf as libc::c_int as uint32_t,
    0xaea0c7f5 as libc::c_uint,
    0x36338cc1 as libc::c_int as uint32_t,
    0x503f7e93 as libc::c_int as uint32_t,
    0xd3772061 as libc::c_uint,
    0x11b638e1 as libc::c_int as uint32_t,
    0x72500e03 as libc::c_int as uint32_t,
    0xf80eb2bb as libc::c_uint,
    0xabe0502e as libc::c_uint,
    0xec8d77de as libc::c_uint,
    0x57971e81 as libc::c_int as uint32_t,
    0xe14f6746 as libc::c_uint,
    0xc9335400 as libc::c_uint,
    0x6920318f as libc::c_int as uint32_t,
    0x81dbb99 as libc::c_int as uint32_t,
    0xffc304a5 as libc::c_uint,
    0x4d351805 as libc::c_int as uint32_t,
    0x7f3d5ce3 as libc::c_int as uint32_t,
    0xa6c866c6 as libc::c_uint,
    0x5d5bcca9 as libc::c_int as uint32_t,
    0xdaec6fea as libc::c_uint,
    0x9f926f91 as libc::c_uint,
    0x9f46222f as libc::c_uint,
    0x3991467d as libc::c_int as uint32_t,
    0xa5bf6d8e as libc::c_uint,
    0x1143c44f as libc::c_int as uint32_t,
    0x43958302 as libc::c_int as uint32_t,
    0xd0214eeb as libc::c_uint,
    0x22083b8 as libc::c_int as uint32_t,
    0x3fb6180c as libc::c_int as uint32_t,
    0x18f8931e as libc::c_int as uint32_t,
    0x281658e6 as libc::c_int as uint32_t,
    0x26486e3e as libc::c_int as uint32_t,
    0x8bd78a70 as libc::c_uint,
    0x7477e4c1 as libc::c_int as uint32_t,
    0xb506e07c as libc::c_uint,
    0xf32d0a25 as libc::c_uint,
    0x79098b02 as libc::c_int as uint32_t,
    0xe4eabb81 as libc::c_uint,
    0x28123b23 as libc::c_int as uint32_t,
    0x69dead38 as libc::c_int as uint32_t,
    0x1574ca16 as libc::c_int as uint32_t,
    0xdf871b62 as libc::c_uint,
    0x211c40b7 as libc::c_int as uint32_t,
    0xa51a9ef9 as libc::c_uint,
    0x14377b as libc::c_int as uint32_t,
    0x41e8ac8 as libc::c_int as uint32_t,
    0x9114003 as libc::c_int as uint32_t,
    0xbd59e4d2 as libc::c_uint,
    0xe3d156d5 as libc::c_uint,
    0x4fe876d5 as libc::c_int as uint32_t,
    0x2f91a340 as libc::c_int as uint32_t,
    0x557be8de as libc::c_int as uint32_t,
    0xeae4a7 as libc::c_int as uint32_t,
    0xce5c2ec as libc::c_int as uint32_t,
    0x4db4bba6 as libc::c_int as uint32_t,
    0xe756bdff as libc::c_uint,
    0xdd3369ac as libc::c_uint,
    0xec17b035 as libc::c_uint,
    0x6572327 as libc::c_int as uint32_t,
    0x99afc8b0 as libc::c_uint,
    0x56c8c391 as libc::c_int as uint32_t,
    0x6b65811c as libc::c_int as uint32_t,
    0x5e146119 as libc::c_int as uint32_t,
    0x6e85cb75 as libc::c_int as uint32_t,
    0xbe07c002 as libc::c_uint,
    0xc2325577 as libc::c_uint,
    0x893ff4ec as libc::c_uint,
    0x5bbfc92d as libc::c_int as uint32_t,
    0xd0ec3b25 as libc::c_uint,
    0xb7801ab7 as libc::c_uint,
    0x8d6d3b24 as libc::c_uint,
    0x20c763ef as libc::c_int as uint32_t,
    0xc366a5fc as libc::c_uint,
    0x9c382880 as libc::c_uint,
    0xace3205 as libc::c_int as uint32_t,
    0xaac9548a as libc::c_uint,
    0xeca1d7c7 as libc::c_uint,
    0x41afa32 as libc::c_int as uint32_t,
    0x1d16625a as libc::c_int as uint32_t,
    0x6701902c as libc::c_int as uint32_t,
    0x9b757a54 as libc::c_uint,
    0x31d477f7 as libc::c_int as uint32_t,
    0x9126b031 as libc::c_uint,
    0x36cc6fdb as libc::c_int as uint32_t,
    0xc70b8b46 as libc::c_uint,
    0xd9e66a48 as libc::c_uint,
    0x56e55a79 as libc::c_int as uint32_t,
    0x26a4ceb as libc::c_int as uint32_t,
    0x52437eff as libc::c_int as uint32_t,
    0x2f8f76b4 as libc::c_int as uint32_t,
    0xdf980a5 as libc::c_int as uint32_t,
    0x8674cde3 as libc::c_uint,
    0xedda04eb as libc::c_uint,
    0x17a9be04 as libc::c_int as uint32_t,
    0x2c18f4df as libc::c_int as uint32_t,
    0xb7747f9d as libc::c_uint,
    0xab2af7b4 as libc::c_uint,
    0xefc34d20 as libc::c_uint,
    0x2e096b7c as libc::c_int as uint32_t,
    0x1741a254 as libc::c_int as uint32_t,
    0xe5b6a035 as libc::c_uint,
    0x213d42f6 as libc::c_int as uint32_t,
    0x2c1c7c26 as libc::c_int as uint32_t,
    0x61c2f50f as libc::c_int as uint32_t,
    0x6552daf9 as libc::c_int as uint32_t,
    0xd2c231f8 as libc::c_uint,
    0x25130f69 as libc::c_int as uint32_t,
    0xd8167fa2 as libc::c_uint,
    0x418f2c8 as libc::c_int as uint32_t,
    0x1a96a6 as libc::c_int as uint32_t,
    0xd1526ab as libc::c_int as uint32_t,
    0x63315c21 as libc::c_int as uint32_t,
    0x5e0a72ec as libc::c_int as uint32_t,
    0x49bafefd as libc::c_int as uint32_t,
    0x187908d9 as libc::c_int as uint32_t,
    0x8d0dbd86 as libc::c_uint,
    0x311170a7 as libc::c_int as uint32_t,
    0x3e9b640c as libc::c_int as uint32_t,
    0xcc3e10d7 as libc::c_uint,
    0xd5cad3b6 as libc::c_uint,
    0xcaec388 as libc::c_int as uint32_t,
    0xf73001e1 as libc::c_uint,
    0x6c728aff as libc::c_int as uint32_t,
    0x71eae2a1 as libc::c_int as uint32_t,
    0x1f9af36e as libc::c_int as uint32_t,
    0xcfcbd12f as libc::c_uint,
    0xc1de8417 as libc::c_uint,
    0xac07be6b as libc::c_uint,
    0xcb44a1d8 as libc::c_uint,
    0x8b9b0f56 as libc::c_uint,
    0x13988c3 as libc::c_int as uint32_t,
    0xb1c52fca as libc::c_uint,
    0xb4be31cd as libc::c_uint,
    0xd8782806 as libc::c_uint,
    0x12a3a4e2 as libc::c_int as uint32_t,
    0x6f7de532 as libc::c_int as uint32_t,
    0x58fd7eb6 as libc::c_int as uint32_t,
    0xd01ee900 as libc::c_uint,
    0x24adffc2 as libc::c_int as uint32_t,
    0xf4990fc5 as libc::c_uint,
    0x9711aac5 as libc::c_uint,
    0x1d7b95 as libc::c_int as uint32_t,
    0x82e5e7d2 as libc::c_uint,
    0x109873f6 as libc::c_int as uint32_t,
    0x613096 as libc::c_int as uint32_t,
    0xc32d9521 as libc::c_uint,
    0xada121ff as libc::c_uint,
    0x29908415 as libc::c_int as uint32_t,
    0x7fbb977f as libc::c_int as uint32_t,
    0xaf9eb3db as libc::c_uint,
    0x29c9ed2a as libc::c_int as uint32_t,
    0x5ce2a465 as libc::c_int as uint32_t,
    0xa730f32c as libc::c_uint,
    0xd0aa3fe8 as libc::c_uint,
    0x8a5cc091 as libc::c_uint,
    0xd49e2ce7 as libc::c_uint,
    0xce454a9 as libc::c_int as uint32_t,
    0xd60acd86 as libc::c_uint,
    0x15f1919 as libc::c_int as uint32_t,
    0x77079103 as libc::c_int as uint32_t,
    0xdea03af6 as libc::c_uint,
    0x78a8565e as libc::c_int as uint32_t,
    0xdee356df as libc::c_uint,
    0x21f05cbe as libc::c_int as uint32_t,
    0x8b75e387 as libc::c_uint,
    0xb3c50651 as libc::c_uint,
    0xb8a5c3ef as libc::c_uint,
    0xd8eeb6d2 as libc::c_uint,
    0xe523be77 as libc::c_uint,
    0xc2154529 as libc::c_uint,
    0x2f69efdf as libc::c_int as uint32_t,
    0xafe67afb as libc::c_uint,
    0xf470c4b2 as libc::c_uint,
    0xf3e0eb5b as libc::c_uint,
    0xd6cc9876 as libc::c_uint,
    0x39e4460c as libc::c_int as uint32_t,
    0x1fda8538 as libc::c_int as uint32_t,
    0x1987832f as libc::c_int as uint32_t,
    0xca007367 as libc::c_uint,
    0xa99144f8 as libc::c_uint,
    0x296b299e as libc::c_int as uint32_t,
    0x492fc295 as libc::c_int as uint32_t,
    0x9266beab as libc::c_uint,
    0xb5676e69 as libc::c_uint,
    0x9bd3ddda as libc::c_uint,
    0xdf7e052f as libc::c_uint,
    0xdb25701c as libc::c_uint,
    0x1b5e51ee as libc::c_int as uint32_t,
    0xf65324e6 as libc::c_uint,
    0x6afce36c as libc::c_int as uint32_t,
    0x316cc04 as libc::c_int as uint32_t,
    0x8644213e as libc::c_uint,
    0xb7dc59d0 as libc::c_uint,
    0x7965291f as libc::c_int as uint32_t,
    0xccd6fd43 as libc::c_uint,
    0x41823979 as libc::c_int as uint32_t,
    0x932bcdf6 as libc::c_uint,
    0xb657c34d as libc::c_uint,
    0x4edfd282 as libc::c_int as uint32_t,
    0x7ae5290c as libc::c_int as uint32_t,
    0x3cb9536b as libc::c_int as uint32_t,
    0x851e20fe as libc::c_uint,
    0x9833557e as libc::c_uint,
    0x13ecf0b0 as libc::c_int as uint32_t,
    0xd3ffb372 as libc::c_uint,
    0x3f85c5c1 as libc::c_int as uint32_t,
    0xaef7ed2 as libc::c_int as uint32_t,
];
static mut cast_sbox5: [uint32_t; 256] = [
    0x7ec90c04 as libc::c_int as uint32_t,
    0x2c6e74b9 as libc::c_int as uint32_t,
    0x9b0e66df as libc::c_uint,
    0xa6337911 as libc::c_uint,
    0xb86a7fff as libc::c_uint,
    0x1dd358f5 as libc::c_int as uint32_t,
    0x44dd9d44 as libc::c_int as uint32_t,
    0x1731167f as libc::c_int as uint32_t,
    0x8fbf1fa as libc::c_int as uint32_t,
    0xe7f511cc as libc::c_uint,
    0xd2051b00 as libc::c_uint,
    0x735aba00 as libc::c_int as uint32_t,
    0x2ab722d8 as libc::c_int as uint32_t,
    0x386381cb as libc::c_int as uint32_t,
    0xacf6243a as libc::c_uint,
    0x69befd7a as libc::c_int as uint32_t,
    0xe6a2e77f as libc::c_uint,
    0xf0c720cd as libc::c_uint,
    0xc4494816 as libc::c_uint,
    0xccf5c180 as libc::c_uint,
    0x38851640 as libc::c_int as uint32_t,
    0x15b0a848 as libc::c_int as uint32_t,
    0xe68b18cb as libc::c_uint,
    0x4caadeff as libc::c_int as uint32_t,
    0x5f480a01 as libc::c_int as uint32_t,
    0x412b2aa as libc::c_int as uint32_t,
    0x259814fc as libc::c_int as uint32_t,
    0x41d0efe2 as libc::c_int as uint32_t,
    0x4e40b48d as libc::c_int as uint32_t,
    0x248eb6fb as libc::c_int as uint32_t,
    0x8dba1cfe as libc::c_uint,
    0x41a99b02 as libc::c_int as uint32_t,
    0x1a550a04 as libc::c_int as uint32_t,
    0xba8f65cb as libc::c_uint,
    0x7251f4e7 as libc::c_int as uint32_t,
    0x95a51725 as libc::c_uint,
    0xc106ecd7 as libc::c_uint,
    0x97a5980a as libc::c_uint,
    0xc539b9aa as libc::c_uint,
    0x4d79fe6a as libc::c_int as uint32_t,
    0xf2f3f763 as libc::c_uint,
    0x68af8040 as libc::c_int as uint32_t,
    0xed0c9e56 as libc::c_uint,
    0x11b4958b as libc::c_int as uint32_t,
    0xe1eb5a88 as libc::c_uint,
    0x8709e6b0 as libc::c_uint,
    0xd7e07156 as libc::c_uint,
    0x4e29fea7 as libc::c_int as uint32_t,
    0x6366e52d as libc::c_int as uint32_t,
    0x2d1c000 as libc::c_int as uint32_t,
    0xc4ac8e05 as libc::c_uint,
    0x9377f571 as libc::c_uint,
    0xc05372a as libc::c_int as uint32_t,
    0x578535f2 as libc::c_int as uint32_t,
    0x2261be02 as libc::c_int as uint32_t,
    0xd642a0c9 as libc::c_uint,
    0xdf13a280 as libc::c_uint,
    0x74b55bd2 as libc::c_int as uint32_t,
    0x682199c0 as libc::c_int as uint32_t,
    0xd421e5ec as libc::c_uint,
    0x53fb3ce8 as libc::c_int as uint32_t,
    0xc8adedb3 as libc::c_uint,
    0x28a87fc9 as libc::c_int as uint32_t,
    0x3d959981 as libc::c_int as uint32_t,
    0x5c1ff900 as libc::c_int as uint32_t,
    0xfe38d399 as libc::c_uint,
    0xc4eff0b as libc::c_int as uint32_t,
    0x62407ea as libc::c_int as uint32_t,
    0xaa2f4fb1 as libc::c_uint,
    0x4fb96976 as libc::c_int as uint32_t,
    0x90c79505 as libc::c_uint,
    0xb0a8a774 as libc::c_uint,
    0xef55a1ff as libc::c_uint,
    0xe59ca2c2 as libc::c_uint,
    0xa6b62d27 as libc::c_uint,
    0xe66a4263 as libc::c_uint,
    0xdf65001f as libc::c_uint,
    0xec50966 as libc::c_int as uint32_t,
    0xdfdd55bc as libc::c_uint,
    0x29de0655 as libc::c_int as uint32_t,
    0x911e739a as libc::c_uint,
    0x17af8975 as libc::c_int as uint32_t,
    0x32c7911c as libc::c_int as uint32_t,
    0x89f89468 as libc::c_uint,
    0xd01e980 as libc::c_int as uint32_t,
    0x524755f4 as libc::c_int as uint32_t,
    0x3b63cc9 as libc::c_int as uint32_t,
    0xcc844b2 as libc::c_int as uint32_t,
    0xbcf3f0aa as libc::c_uint,
    0x87ac36e9 as libc::c_uint,
    0xe53a7426 as libc::c_uint,
    0x1b3d82b as libc::c_int as uint32_t,
    0x1a9e7449 as libc::c_int as uint32_t,
    0x64ee2d7e as libc::c_int as uint32_t,
    0xcddbb1da as libc::c_uint,
    0x1c94910 as libc::c_int as uint32_t,
    0xb868bf80 as libc::c_uint,
    0xd26f3fd as libc::c_int as uint32_t,
    0x9342ede7 as libc::c_uint,
    0x4a5c284 as libc::c_int as uint32_t,
    0x636737b6 as libc::c_int as uint32_t,
    0x50f5b616 as libc::c_int as uint32_t,
    0xf24766e3 as libc::c_uint,
    0x8eca36c1 as libc::c_uint,
    0x136e05db as libc::c_int as uint32_t,
    0xfef18391 as libc::c_uint,
    0xfb887a37 as libc::c_uint,
    0xd6e7f7d4 as libc::c_uint,
    0xc7fb7dc9 as libc::c_uint,
    0x3063fcdf as libc::c_int as uint32_t,
    0xb6f589de as libc::c_uint,
    0xec2941da as libc::c_uint,
    0x26e46695 as libc::c_int as uint32_t,
    0xb7566419 as libc::c_uint,
    0xf654efc5 as libc::c_uint,
    0xd08d58b7 as libc::c_uint,
    0x48925401 as libc::c_int as uint32_t,
    0xc1bacb7f as libc::c_uint,
    0xe5ff550f as libc::c_uint,
    0xb6083049 as libc::c_uint,
    0x5bb5d0e8 as libc::c_int as uint32_t,
    0x87d72e5a as libc::c_uint,
    0xab6a6ee1 as libc::c_uint,
    0x223a66ce as libc::c_int as uint32_t,
    0xc62bf3cd as libc::c_uint,
    0x9e0885f9 as libc::c_uint,
    0x68cb3e47 as libc::c_int as uint32_t,
    0x86c010f as libc::c_int as uint32_t,
    0xa21de820 as libc::c_uint,
    0xd18b69de as libc::c_uint,
    0xf3f65777 as libc::c_uint,
    0xfa02c3f6 as libc::c_uint,
    0x407edac3 as libc::c_int as uint32_t,
    0xcbb3d550 as libc::c_uint,
    0x1793084d as libc::c_int as uint32_t,
    0xb0d70eba as libc::c_uint,
    0xab378d5 as libc::c_int as uint32_t,
    0xd951fb0c as libc::c_uint,
    0xded7da56 as libc::c_uint,
    0x4124bbe4 as libc::c_int as uint32_t,
    0x94ca0b56 as libc::c_uint,
    0xf5755d1 as libc::c_int as uint32_t,
    0xe0e1e56e as libc::c_uint,
    0x6184b5be as libc::c_int as uint32_t,
    0x580a249f as libc::c_int as uint32_t,
    0x94f74bc0 as libc::c_uint,
    0xe327888e as libc::c_uint,
    0x9f7b5561 as libc::c_uint,
    0xc3dc0280 as libc::c_uint,
    0x5687715 as libc::c_int as uint32_t,
    0x646c6bd7 as libc::c_int as uint32_t,
    0x44904db3 as libc::c_int as uint32_t,
    0x66b4f0a3 as libc::c_int as uint32_t,
    0xc0f1648a as libc::c_uint,
    0x697ed5af as libc::c_int as uint32_t,
    0x49e92ff6 as libc::c_int as uint32_t,
    0x309e374f as libc::c_int as uint32_t,
    0x2cb6356a as libc::c_int as uint32_t,
    0x85808573 as libc::c_uint,
    0x4991f840 as libc::c_int as uint32_t,
    0x76f0ae02 as libc::c_int as uint32_t,
    0x83be84d as libc::c_int as uint32_t,
    0x28421c9a as libc::c_int as uint32_t,
    0x44489406 as libc::c_int as uint32_t,
    0x736e4cb8 as libc::c_int as uint32_t,
    0xc1092910 as libc::c_uint,
    0x8bc95fc6 as libc::c_uint,
    0x7d869cf4 as libc::c_int as uint32_t,
    0x134f616f as libc::c_int as uint32_t,
    0x2e77118d as libc::c_int as uint32_t,
    0xb31b2be1 as libc::c_uint,
    0xaa90b472 as libc::c_uint,
    0x3ca5d717 as libc::c_int as uint32_t,
    0x7d161bba as libc::c_int as uint32_t,
    0x9cad9010 as libc::c_uint,
    0xaf462ba2 as libc::c_uint,
    0x9fe459d2 as libc::c_uint,
    0x45d34559 as libc::c_int as uint32_t,
    0xd9f2da13 as libc::c_uint,
    0xdbc65487 as libc::c_uint,
    0xf3e4f94e as libc::c_uint,
    0x176d486f as libc::c_int as uint32_t,
    0x97c13ea as libc::c_int as uint32_t,
    0x631da5c7 as libc::c_int as uint32_t,
    0x445f7382 as libc::c_int as uint32_t,
    0x175683f4 as libc::c_int as uint32_t,
    0xcdc66a97 as libc::c_uint,
    0x70be0288 as libc::c_int as uint32_t,
    0xb3cdcf72 as libc::c_uint,
    0x6e5dd2f3 as libc::c_int as uint32_t,
    0x20936079 as libc::c_int as uint32_t,
    0x459b80a5 as libc::c_int as uint32_t,
    0xbe60e2db as libc::c_uint,
    0xa9c23101 as libc::c_uint,
    0xeba5315c as libc::c_uint,
    0x224e42f2 as libc::c_int as uint32_t,
    0x1c5c1572 as libc::c_int as uint32_t,
    0xf6721b2c as libc::c_uint,
    0x1ad2fff3 as libc::c_int as uint32_t,
    0x8c25404e as libc::c_uint,
    0x324ed72f as libc::c_int as uint32_t,
    0x4067b7fd as libc::c_int as uint32_t,
    0x523138e as libc::c_int as uint32_t,
    0x5ca3bc78 as libc::c_int as uint32_t,
    0xdc0fd66e as libc::c_uint,
    0x75922283 as libc::c_int as uint32_t,
    0x784d6b17 as libc::c_int as uint32_t,
    0x58ebb16e as libc::c_int as uint32_t,
    0x44094f85 as libc::c_int as uint32_t,
    0x3f481d87 as libc::c_int as uint32_t,
    0xfcfeae7b as libc::c_uint,
    0x77b5ff76 as libc::c_int as uint32_t,
    0x8c2302bf as libc::c_uint,
    0xaaf47556 as libc::c_uint,
    0x5f46b02a as libc::c_int as uint32_t,
    0x2b092801 as libc::c_int as uint32_t,
    0x3d38f5f7 as libc::c_int as uint32_t,
    0xca81f36 as libc::c_int as uint32_t,
    0x52af4a8a as libc::c_int as uint32_t,
    0x66d5e7c0 as libc::c_int as uint32_t,
    0xdf3b0874 as libc::c_uint,
    0x95055110 as libc::c_uint,
    0x1b5ad7a8 as libc::c_int as uint32_t,
    0xf61ed5ad as libc::c_uint,
    0x6cf6e479 as libc::c_int as uint32_t,
    0x20758184 as libc::c_int as uint32_t,
    0xd0cefa65 as libc::c_uint,
    0x88f7be58 as libc::c_uint,
    0x4a046826 as libc::c_int as uint32_t,
    0xff6f8f3 as libc::c_int as uint32_t,
    0xa09c7f70 as libc::c_uint,
    0x5346aba0 as libc::c_int as uint32_t,
    0x5ce96c28 as libc::c_int as uint32_t,
    0xe176eda3 as libc::c_uint,
    0x6bac307f as libc::c_int as uint32_t,
    0x376829d2 as libc::c_int as uint32_t,
    0x85360fa9 as libc::c_uint,
    0x17e3fe2a as libc::c_int as uint32_t,
    0x24b79767 as libc::c_int as uint32_t,
    0xf5a96b20 as libc::c_uint,
    0xd6cd2595 as libc::c_uint,
    0x68ff1ebf as libc::c_int as uint32_t,
    0x7555442c as libc::c_int as uint32_t,
    0xf19f06be as libc::c_uint,
    0xf9e0659a as libc::c_uint,
    0xeeb9491d as libc::c_uint,
    0x34010718 as libc::c_int as uint32_t,
    0xbb30cab8 as libc::c_uint,
    0xe822fe15 as libc::c_uint,
    0x88570983 as libc::c_uint,
    0x750e6249 as libc::c_int as uint32_t,
    0xda627e55 as libc::c_uint,
    0x5e76ffa8 as libc::c_int as uint32_t,
    0xb1534546 as libc::c_uint,
    0x6d47de08 as libc::c_int as uint32_t,
    0xefe9e7d4 as libc::c_uint,
];
static mut cast_sbox6: [uint32_t; 256] = [
    0xf6fa8f9d as libc::c_uint,
    0x2cac6ce1 as libc::c_int as uint32_t,
    0x4ca34867 as libc::c_int as uint32_t,
    0xe2337f7c as libc::c_uint,
    0x95db08e7 as libc::c_uint,
    0x16843b4 as libc::c_int as uint32_t,
    0xeced5cbc as libc::c_uint,
    0x325553ac as libc::c_int as uint32_t,
    0xbf9f0960 as libc::c_uint,
    0xdfa1e2ed as libc::c_uint,
    0x83f0579d as libc::c_uint,
    0x63ed86b9 as libc::c_int as uint32_t,
    0x1ab6a6b8 as libc::c_int as uint32_t,
    0xde5ebe39 as libc::c_uint,
    0xf38ff732 as libc::c_uint,
    0x8989b138 as libc::c_uint,
    0x33f14961 as libc::c_int as uint32_t,
    0xc01937bd as libc::c_uint,
    0xf506c6da as libc::c_uint,
    0xe4625e7e as libc::c_uint,
    0xa308ea99 as libc::c_uint,
    0x4e23e33c as libc::c_int as uint32_t,
    0x79cbd7cc as libc::c_int as uint32_t,
    0x48a14367 as libc::c_int as uint32_t,
    0xa3149619 as libc::c_uint,
    0xfec94bd5 as libc::c_uint,
    0xa114174a as libc::c_uint,
    0xeaa01866 as libc::c_uint,
    0xa084db2d as libc::c_uint,
    0x9a8486f as libc::c_int as uint32_t,
    0xa888614a as libc::c_uint,
    0x2900af98 as libc::c_int as uint32_t,
    0x1665991 as libc::c_int as uint32_t,
    0xe1992863 as libc::c_uint,
    0xc8f30c60 as libc::c_uint,
    0x2e78ef3c as libc::c_int as uint32_t,
    0xd0d51932 as libc::c_uint,
    0xcf0fec14 as libc::c_uint,
    0xf7ca07d2 as libc::c_uint,
    0xd0a82072 as libc::c_uint,
    0xfd41197e as libc::c_uint,
    0x9305a6b0 as libc::c_uint,
    0xe86be3da as libc::c_uint,
    0x74bed3cd as libc::c_int as uint32_t,
    0x372da53c as libc::c_int as uint32_t,
    0x4c7f4448 as libc::c_int as uint32_t,
    0xdab5d440 as libc::c_uint,
    0x6dba0ec3 as libc::c_int as uint32_t,
    0x83919a7 as libc::c_int as uint32_t,
    0x9fbaeed9 as libc::c_uint,
    0x49dbcfb0 as libc::c_int as uint32_t,
    0x4e670c53 as libc::c_int as uint32_t,
    0x5c3d9c01 as libc::c_int as uint32_t,
    0x64bdb941 as libc::c_int as uint32_t,
    0x2c0e636a as libc::c_int as uint32_t,
    0xba7dd9cd as libc::c_uint,
    0xea6f7388 as libc::c_uint,
    0xe70bc762 as libc::c_uint,
    0x35f29adb as libc::c_int as uint32_t,
    0x5c4cdd8d as libc::c_int as uint32_t,
    0xf0d48d8c as libc::c_uint,
    0xb88153e2 as libc::c_uint,
    0x8a19866 as libc::c_int as uint32_t,
    0x1ae2eac8 as libc::c_int as uint32_t,
    0x284caf89 as libc::c_int as uint32_t,
    0xaa928223 as libc::c_uint,
    0x9334be53 as libc::c_uint,
    0x3b3a21bf as libc::c_int as uint32_t,
    0x16434be3 as libc::c_int as uint32_t,
    0x9aea3906 as libc::c_uint,
    0xefe8c36e as libc::c_uint,
    0xf890cdd9 as libc::c_uint,
    0x80226dae as libc::c_uint,
    0xc340a4a3 as libc::c_uint,
    0xdf7e9c09 as libc::c_uint,
    0xa694a807 as libc::c_uint,
    0x5b7c5ecc as libc::c_int as uint32_t,
    0x221db3a6 as libc::c_int as uint32_t,
    0x9a69a02f as libc::c_uint,
    0x68818a54 as libc::c_int as uint32_t,
    0xceb2296f as libc::c_uint,
    0x53c0843a as libc::c_int as uint32_t,
    0xfe893655 as libc::c_uint,
    0x25bfe68a as libc::c_int as uint32_t,
    0xb4628abc as libc::c_uint,
    0xcf222ebf as libc::c_uint,
    0x25ac6f48 as libc::c_int as uint32_t,
    0xa9a99387 as libc::c_uint,
    0x53bddb65 as libc::c_int as uint32_t,
    0xe76ffbe7 as libc::c_uint,
    0xe967fd78 as libc::c_uint,
    0xba93563 as libc::c_int as uint32_t,
    0x8e342bc1 as libc::c_uint,
    0xe8a11be9 as libc::c_uint,
    0x4980740d as libc::c_int as uint32_t,
    0xc8087dfc as libc::c_uint,
    0x8de4bf99 as libc::c_uint,
    0xa11101a0 as libc::c_uint,
    0x7fd37975 as libc::c_int as uint32_t,
    0xda5a26c0 as libc::c_uint,
    0xe81f994f as libc::c_uint,
    0x9528cd89 as libc::c_uint,
    0xfd339fed as libc::c_uint,
    0xb87834bf as libc::c_uint,
    0x5f04456d as libc::c_int as uint32_t,
    0x22258698 as libc::c_int as uint32_t,
    0xc9c4c83b as libc::c_uint,
    0x2dc156be as libc::c_int as uint32_t,
    0x4f628daa as libc::c_int as uint32_t,
    0x57f55ec5 as libc::c_int as uint32_t,
    0xe2220abe as libc::c_uint,
    0xd2916ebf as libc::c_uint,
    0x4ec75b95 as libc::c_int as uint32_t,
    0x24f2c3c0 as libc::c_int as uint32_t,
    0x42d15d99 as libc::c_int as uint32_t,
    0xcd0d7fa0 as libc::c_uint,
    0x7b6e27ff as libc::c_int as uint32_t,
    0xa8dc8af0 as libc::c_uint,
    0x7345c106 as libc::c_int as uint32_t,
    0xf41e232f as libc::c_uint,
    0x35162386 as libc::c_int as uint32_t,
    0xe6ea8926 as libc::c_uint,
    0x3333b094 as libc::c_int as uint32_t,
    0x157ec6f2 as libc::c_int as uint32_t,
    0x372b74af as libc::c_int as uint32_t,
    0x692573e4 as libc::c_int as uint32_t,
    0xe9a9d848 as libc::c_uint,
    0xf3160289 as libc::c_uint,
    0x3a62ef1d as libc::c_int as uint32_t,
    0xa787e238 as libc::c_uint,
    0xf3a5f676 as libc::c_uint,
    0x74364853 as libc::c_int as uint32_t,
    0x20951063 as libc::c_int as uint32_t,
    0x4576698d as libc::c_int as uint32_t,
    0xb6fad407 as libc::c_uint,
    0x592af950 as libc::c_int as uint32_t,
    0x36f73523 as libc::c_int as uint32_t,
    0x4cfb6e87 as libc::c_int as uint32_t,
    0x7da4cec0 as libc::c_int as uint32_t,
    0x6c152daa as libc::c_int as uint32_t,
    0xcb0396a8 as libc::c_uint,
    0xc50dfe5d as libc::c_uint,
    0xfcd707ab as libc::c_uint,
    0x921c42f as libc::c_int as uint32_t,
    0x89dff0bb as libc::c_uint,
    0x5fe2be78 as libc::c_int as uint32_t,
    0x448f4f33 as libc::c_int as uint32_t,
    0x754613c9 as libc::c_int as uint32_t,
    0x2b05d08d as libc::c_int as uint32_t,
    0x48b9d585 as libc::c_int as uint32_t,
    0xdc049441 as libc::c_uint,
    0xc8098f9b as libc::c_uint,
    0x7dede786 as libc::c_int as uint32_t,
    0xc39a3373 as libc::c_uint,
    0x42410005 as libc::c_int as uint32_t,
    0x6a091751 as libc::c_int as uint32_t,
    0xef3c8a6 as libc::c_int as uint32_t,
    0x890072d6 as libc::c_uint,
    0x28207682 as libc::c_int as uint32_t,
    0xa9a9f7be as libc::c_uint,
    0xbf32679d as libc::c_uint,
    0xd45b5b75 as libc::c_uint,
    0xb353fd00 as libc::c_uint,
    0xcbb0e358 as libc::c_uint,
    0x830f220a as libc::c_uint,
    0x1f8fb214 as libc::c_int as uint32_t,
    0xd372cf08 as libc::c_uint,
    0xcc3c4a13 as libc::c_uint,
    0x8cf63166 as libc::c_uint,
    0x61c87be as libc::c_int as uint32_t,
    0x88c98f88 as libc::c_uint,
    0x6062e397 as libc::c_int as uint32_t,
    0x47cf8e7a as libc::c_int as uint32_t,
    0xb6c85283 as libc::c_uint,
    0x3cc2acfb as libc::c_int as uint32_t,
    0x3fc06976 as libc::c_int as uint32_t,
    0x4e8f0252 as libc::c_int as uint32_t,
    0x64d8314d as libc::c_int as uint32_t,
    0xda3870e3 as libc::c_uint,
    0x1e665459 as libc::c_int as uint32_t,
    0xc10908f0 as libc::c_uint,
    0x513021a5 as libc::c_int as uint32_t,
    0x6c5b68b7 as libc::c_int as uint32_t,
    0x822f8aa0 as libc::c_uint,
    0x3007cd3e as libc::c_int as uint32_t,
    0x74719eef as libc::c_int as uint32_t,
    0xdc872681 as libc::c_uint,
    0x73340d4 as libc::c_int as uint32_t,
    0x7e432fd9 as libc::c_int as uint32_t,
    0xc5ec241 as libc::c_int as uint32_t,
    0x8809286c as libc::c_uint,
    0xf592d891 as libc::c_uint,
    0x8a930f6 as libc::c_int as uint32_t,
    0x957ef305 as libc::c_uint,
    0xb7fbffbd as libc::c_uint,
    0xc266e96f as libc::c_uint,
    0x6fe4ac98 as libc::c_int as uint32_t,
    0xb173ecc0 as libc::c_uint,
    0xbc60b42a as libc::c_uint,
    0x953498da as libc::c_uint,
    0xfba1ae12 as libc::c_uint,
    0x2d4bd736 as libc::c_int as uint32_t,
    0xf25faab as libc::c_int as uint32_t,
    0xa4f3fceb as libc::c_uint,
    0xe2969123 as libc::c_uint,
    0x257f0c3d as libc::c_int as uint32_t,
    0x9348af49 as libc::c_uint,
    0x361400bc as libc::c_int as uint32_t,
    0xe8816f4a as libc::c_uint,
    0x3814f200 as libc::c_int as uint32_t,
    0xa3f94043 as libc::c_uint,
    0x9c7a54c2 as libc::c_uint,
    0xbc704f57 as libc::c_uint,
    0xda41e7f9 as libc::c_uint,
    0xc25ad33a as libc::c_uint,
    0x54f4a084 as libc::c_int as uint32_t,
    0xb17f5505 as libc::c_uint,
    0x59357cbe as libc::c_int as uint32_t,
    0xedbd15c8 as libc::c_uint,
    0x7f97c5ab as libc::c_int as uint32_t,
    0xba5ac7b5 as libc::c_uint,
    0xb6f6deaf as libc::c_uint,
    0x3a479c3a as libc::c_int as uint32_t,
    0x5302da25 as libc::c_int as uint32_t,
    0x653d7e6a as libc::c_int as uint32_t,
    0x54268d49 as libc::c_int as uint32_t,
    0x51a477ea as libc::c_int as uint32_t,
    0x5017d55b as libc::c_int as uint32_t,
    0xd7d25d88 as libc::c_uint,
    0x44136c76 as libc::c_int as uint32_t,
    0x404a8c8 as libc::c_int as uint32_t,
    0xb8e5a121 as libc::c_uint,
    0xb81a928a as libc::c_uint,
    0x60ed5869 as libc::c_int as uint32_t,
    0x97c55b96 as libc::c_uint,
    0xeaec991b as libc::c_uint,
    0x29935913 as libc::c_int as uint32_t,
    0x1fdb7f1 as libc::c_int as uint32_t,
    0x88e8dfa as libc::c_int as uint32_t,
    0x9ab6f6f5 as libc::c_uint,
    0x3b4cbf9f as libc::c_int as uint32_t,
    0x4a5de3ab as libc::c_int as uint32_t,
    0xe6051d35 as libc::c_uint,
    0xa0e1d855 as libc::c_uint,
    0xd36b4cf1 as libc::c_uint,
    0xf544edeb as libc::c_uint,
    0xb0e93524 as libc::c_uint,
    0xbebb8fbd as libc::c_uint,
    0xa2d762cf as libc::c_uint,
    0x49c92f54 as libc::c_int as uint32_t,
    0x38b5f331 as libc::c_int as uint32_t,
    0x7128a454 as libc::c_int as uint32_t,
    0x48392905 as libc::c_int as uint32_t,
    0xa65b1db8 as libc::c_uint,
    0x851c97bd as libc::c_uint,
    0xd675cf2f as libc::c_uint,
];
static mut cast_sbox7: [uint32_t; 256] = [
    0x85e04019 as libc::c_uint,
    0x332bf567 as libc::c_int as uint32_t,
    0x662dbfff as libc::c_int as uint32_t,
    0xcfc65693 as libc::c_uint,
    0x2a8d7f6f as libc::c_int as uint32_t,
    0xab9bc912 as libc::c_uint,
    0xde6008a1 as libc::c_uint,
    0x2028da1f as libc::c_int as uint32_t,
    0x227bce7 as libc::c_int as uint32_t,
    0x4d642916 as libc::c_int as uint32_t,
    0x18fac300 as libc::c_int as uint32_t,
    0x50f18b82 as libc::c_int as uint32_t,
    0x2cb2cb11 as libc::c_int as uint32_t,
    0xb232e75c as libc::c_uint,
    0x4b3695f2 as libc::c_int as uint32_t,
    0xb28707de as libc::c_uint,
    0xa05fbcf6 as libc::c_uint,
    0xcd4181e9 as libc::c_uint,
    0xe150210c as libc::c_uint,
    0xe24ef1bd as libc::c_uint,
    0xb168c381 as libc::c_uint,
    0xfde4e789 as libc::c_uint,
    0x5c79b0d8 as libc::c_int as uint32_t,
    0x1e8bfd43 as libc::c_int as uint32_t,
    0x4d495001 as libc::c_int as uint32_t,
    0x38be4341 as libc::c_int as uint32_t,
    0x913cee1d as libc::c_uint,
    0x92a79c3f as libc::c_uint,
    0x89766be as libc::c_int as uint32_t,
    0xbaeeadf4 as libc::c_uint,
    0x1286becf as libc::c_int as uint32_t,
    0xb6eacb19 as libc::c_uint,
    0x2660c200 as libc::c_int as uint32_t,
    0x7565bde4 as libc::c_int as uint32_t,
    0x64241f7a as libc::c_int as uint32_t,
    0x8248dca9 as libc::c_uint,
    0xc3b3ad66 as libc::c_uint,
    0x28136086 as libc::c_int as uint32_t,
    0xbd8dfa8 as libc::c_int as uint32_t,
    0x356d1cf2 as libc::c_int as uint32_t,
    0x107789be as libc::c_int as uint32_t,
    0xb3b2e9ce as libc::c_uint,
    0x502aa8f as libc::c_int as uint32_t,
    0xbc0351e as libc::c_int as uint32_t,
    0x166bf52a as libc::c_int as uint32_t,
    0xeb12ff82 as libc::c_uint,
    0xe3486911 as libc::c_uint,
    0xd34d7516 as libc::c_uint,
    0x4e7b3aff as libc::c_int as uint32_t,
    0x5f43671b as libc::c_int as uint32_t,
    0x9cf6e037 as libc::c_uint,
    0x4981ac83 as libc::c_int as uint32_t,
    0x334266ce as libc::c_int as uint32_t,
    0x8c9341b7 as libc::c_uint,
    0xd0d854c0 as libc::c_uint,
    0xcb3a6c88 as libc::c_uint,
    0x47bc2829 as libc::c_int as uint32_t,
    0x4725ba37 as libc::c_int as uint32_t,
    0xa66ad22b as libc::c_uint,
    0x7ad61f1e as libc::c_int as uint32_t,
    0xc5cbafa as libc::c_int as uint32_t,
    0x4437f107 as libc::c_int as uint32_t,
    0xb6e79962 as libc::c_uint,
    0x42d2d816 as libc::c_int as uint32_t,
    0xa961288 as libc::c_int as uint32_t,
    0xe1a5c06e as libc::c_uint,
    0x13749e67 as libc::c_int as uint32_t,
    0x72fc081a as libc::c_int as uint32_t,
    0xb1d139f7 as libc::c_uint,
    0xf9583745 as libc::c_uint,
    0xcf19df58 as libc::c_uint,
    0xbec3f756 as libc::c_uint,
    0xc06eba30 as libc::c_uint,
    0x7211b24 as libc::c_int as uint32_t,
    0x45c28829 as libc::c_int as uint32_t,
    0xc95e317f as libc::c_uint,
    0xbc8ec511 as libc::c_uint,
    0x38bc46e9 as libc::c_int as uint32_t,
    0xc6e6fa14 as libc::c_uint,
    0xbae8584a as libc::c_uint,
    0xad4ebc46 as libc::c_uint,
    0x468f508b as libc::c_int as uint32_t,
    0x7829435f as libc::c_int as uint32_t,
    0xf124183b as libc::c_uint,
    0x821dba9f as libc::c_uint,
    0xaff60ff4 as libc::c_uint,
    0xea2c4e6d as libc::c_uint,
    0x16e39264 as libc::c_int as uint32_t,
    0x92544a8b as libc::c_uint,
    0x9b4fc3 as libc::c_int as uint32_t,
    0xaba68ced as libc::c_uint,
    0x9ac96f78 as libc::c_uint,
    0x6a5b79a as libc::c_int as uint32_t,
    0xb2856e6e as libc::c_uint,
    0x1aec3ca9 as libc::c_int as uint32_t,
    0xbe838688 as libc::c_uint,
    0xe0804e9 as libc::c_int as uint32_t,
    0x55f1be56 as libc::c_int as uint32_t,
    0xe7e5363b as libc::c_uint,
    0xb3a1f25d as libc::c_uint,
    0xf7debb85 as libc::c_uint,
    0x61fe033c as libc::c_int as uint32_t,
    0x16746233 as libc::c_int as uint32_t,
    0x3c034c28 as libc::c_int as uint32_t,
    0xda6d0c74 as libc::c_uint,
    0x79aac56c as libc::c_int as uint32_t,
    0x3ce4e1ad as libc::c_int as uint32_t,
    0x51f0c802 as libc::c_int as uint32_t,
    0x98f8f35a as libc::c_uint,
    0x1626a49f as libc::c_int as uint32_t,
    0xeed82b29 as libc::c_uint,
    0x1d382fe3 as libc::c_int as uint32_t,
    0xc4fb99a as libc::c_int as uint32_t,
    0xbb325778 as libc::c_uint,
    0x3ec6d97b as libc::c_int as uint32_t,
    0x6e77a6a9 as libc::c_int as uint32_t,
    0xcb658b5c as libc::c_uint,
    0xd45230c7 as libc::c_uint,
    0x2bd1408b as libc::c_int as uint32_t,
    0x60c03eb7 as libc::c_int as uint32_t,
    0xb9068d78 as libc::c_uint,
    0xa33754f4 as libc::c_uint,
    0xf430c87d as libc::c_uint,
    0xc8a71302 as libc::c_uint,
    0xb96d8c32 as libc::c_uint,
    0xebd4e7be as libc::c_uint,
    0xbe8b9d2d as libc::c_uint,
    0x7979fb06 as libc::c_int as uint32_t,
    0xe7225308 as libc::c_uint,
    0x8b75cf77 as libc::c_uint,
    0x11ef8da4 as libc::c_int as uint32_t,
    0xe083c858 as libc::c_uint,
    0x8d6b786f as libc::c_uint,
    0x5a6317a6 as libc::c_int as uint32_t,
    0xfa5cf7a0 as libc::c_uint,
    0x5dda0033 as libc::c_int as uint32_t,
    0xf28ebfb0 as libc::c_uint,
    0xf5b9c310 as libc::c_uint,
    0xa0eac280 as libc::c_uint,
    0x8b9767a as libc::c_int as uint32_t,
    0xa3d9d2b0 as libc::c_uint,
    0x79d34217 as libc::c_int as uint32_t,
    0x21a718d as libc::c_int as uint32_t,
    0x9ac6336a as libc::c_uint,
    0x2711fd60 as libc::c_int as uint32_t,
    0x438050e3 as libc::c_int as uint32_t,
    0x69908a8 as libc::c_int as uint32_t,
    0x3d7fedc4 as libc::c_int as uint32_t,
    0x826d2bef as libc::c_uint,
    0x4eeb8476 as libc::c_int as uint32_t,
    0x488dcf25 as libc::c_int as uint32_t,
    0x36c9d566 as libc::c_int as uint32_t,
    0x28e74e41 as libc::c_int as uint32_t,
    0xc2610aca as libc::c_uint,
    0x3d49a9cf as libc::c_int as uint32_t,
    0xbae3b9df as libc::c_uint,
    0xb65f8de6 as libc::c_uint,
    0x92aeaf64 as libc::c_uint,
    0x3ac7d5e6 as libc::c_int as uint32_t,
    0x9ea80509 as libc::c_uint,
    0xf22b017d as libc::c_uint,
    0xa4173f70 as libc::c_uint,
    0xdd1e16c3 as libc::c_uint,
    0x15e0d7f9 as libc::c_int as uint32_t,
    0x50b1b887 as libc::c_int as uint32_t,
    0x2b9f4fd5 as libc::c_int as uint32_t,
    0x625aba82 as libc::c_int as uint32_t,
    0x6a017962 as libc::c_int as uint32_t,
    0x2ec01b9c as libc::c_int as uint32_t,
    0x15488aa9 as libc::c_int as uint32_t,
    0xd716e740 as libc::c_uint,
    0x40055a2c as libc::c_int as uint32_t,
    0x93d29a22 as libc::c_uint,
    0xe32dbf9a as libc::c_uint,
    0x58745b9 as libc::c_int as uint32_t,
    0x3453dc1e as libc::c_int as uint32_t,
    0xd699296e as libc::c_uint,
    0x496cff6f as libc::c_int as uint32_t,
    0x1c9f4986 as libc::c_int as uint32_t,
    0xdfe2ed07 as libc::c_uint,
    0xb87242d1 as libc::c_uint,
    0x19de7eae as libc::c_int as uint32_t,
    0x53e561a as libc::c_int as uint32_t,
    0x15ad6f8c as libc::c_int as uint32_t,
    0x66626c1c as libc::c_int as uint32_t,
    0x7154c24c as libc::c_int as uint32_t,
    0xea082b2a as libc::c_uint,
    0x93eb2939 as libc::c_uint,
    0x17dcb0f0 as libc::c_int as uint32_t,
    0x58d4f2ae as libc::c_int as uint32_t,
    0x9ea294fb as libc::c_uint,
    0x52cf564c as libc::c_int as uint32_t,
    0x9883fe66 as libc::c_uint,
    0x2ec40581 as libc::c_int as uint32_t,
    0x763953c3 as libc::c_int as uint32_t,
    0x1d6692e as libc::c_int as uint32_t,
    0xd3a0c108 as libc::c_uint,
    0xa1e7160e as libc::c_uint,
    0xe4f2dfa6 as libc::c_uint,
    0x693ed285 as libc::c_int as uint32_t,
    0x74904698 as libc::c_int as uint32_t,
    0x4c2b0edd as libc::c_int as uint32_t,
    0x4f757656 as libc::c_int as uint32_t,
    0x5d393378 as libc::c_int as uint32_t,
    0xa132234f as libc::c_uint,
    0x3d321c5d as libc::c_int as uint32_t,
    0xc3f5e194 as libc::c_uint,
    0x4b269301 as libc::c_int as uint32_t,
    0xc79f022f as libc::c_uint,
    0x3c997e7e as libc::c_int as uint32_t,
    0x5e4f9504 as libc::c_int as uint32_t,
    0x3ffafbbd as libc::c_int as uint32_t,
    0x76f7ad0e as libc::c_int as uint32_t,
    0x296693f4 as libc::c_int as uint32_t,
    0x3d1fce6f as libc::c_int as uint32_t,
    0xc61e45be as libc::c_uint,
    0xd3b5ab34 as libc::c_uint,
    0xf72bf9b7 as libc::c_uint,
    0x1b0434c0 as libc::c_int as uint32_t,
    0x4e72b567 as libc::c_int as uint32_t,
    0x5592a33d as libc::c_int as uint32_t,
    0xb5229301 as libc::c_uint,
    0xcfd2a87f as libc::c_uint,
    0x60aeb767 as libc::c_int as uint32_t,
    0x1814386b as libc::c_int as uint32_t,
    0x30bcc33d as libc::c_int as uint32_t,
    0x38a0c07d as libc::c_int as uint32_t,
    0xfd1606f2 as libc::c_uint,
    0xc363519b as libc::c_uint,
    0x589dd390 as libc::c_int as uint32_t,
    0x5479f8e6 as libc::c_int as uint32_t,
    0x1cb8d647 as libc::c_int as uint32_t,
    0x97fd61a9 as libc::c_uint,
    0xea7759f4 as libc::c_uint,
    0x2d57539d as libc::c_int as uint32_t,
    0x569a58cf as libc::c_int as uint32_t,
    0xe84e63ad as libc::c_uint,
    0x462e1b78 as libc::c_int as uint32_t,
    0x6580f87e as libc::c_int as uint32_t,
    0xf3817914 as libc::c_uint,
    0x91da55f4 as libc::c_uint,
    0x40a230f3 as libc::c_int as uint32_t,
    0xd1988f35 as libc::c_uint,
    0xb6e318d2 as libc::c_uint,
    0x3ffa50bc as libc::c_int as uint32_t,
    0x3d40f021 as libc::c_int as uint32_t,
    0xc3c0bdae as libc::c_uint,
    0x4958c24c as libc::c_int as uint32_t,
    0x518f36b2 as libc::c_int as uint32_t,
    0x84b1d370 as libc::c_uint,
    0xfedce83 as libc::c_int as uint32_t,
    0x878ddada as libc::c_uint,
    0xf2a279c7 as libc::c_uint,
    0x94e01be8 as libc::c_uint,
    0x90716f4b as libc::c_uint,
    0x954b8aa3 as libc::c_uint,
];
static mut cast_sbox8: [uint32_t; 256] = [
    0xe216300d as libc::c_uint,
    0xbbddfffc as libc::c_uint,
    0xa7ebdabd as libc::c_uint,
    0x35648095 as libc::c_int as uint32_t,
    0x7789f8b7 as libc::c_int as uint32_t,
    0xe6c1121b as libc::c_uint,
    0xe241600 as libc::c_int as uint32_t,
    0x52ce8b5 as libc::c_int as uint32_t,
    0x11a9cfb0 as libc::c_int as uint32_t,
    0xe5952f11 as libc::c_uint,
    0xece7990a as libc::c_uint,
    0x9386d174 as libc::c_uint,
    0x2a42931c as libc::c_int as uint32_t,
    0x76e38111 as libc::c_int as uint32_t,
    0xb12def3a as libc::c_uint,
    0x37ddddfc as libc::c_int as uint32_t,
    0xde9adeb1 as libc::c_uint,
    0xa0cc32c as libc::c_int as uint32_t,
    0xbe197029 as libc::c_uint,
    0x84a00940 as libc::c_uint,
    0xbb243a0f as libc::c_uint,
    0xb4d137cf as libc::c_uint,
    0xb44e79f0 as libc::c_uint,
    0x49eedfd as libc::c_int as uint32_t,
    0xb15a15d as libc::c_int as uint32_t,
    0x480d3168 as libc::c_int as uint32_t,
    0x8bbbde5a as libc::c_uint,
    0x669ded42 as libc::c_int as uint32_t,
    0xc7ece831 as libc::c_uint,
    0x3f8f95e7 as libc::c_int as uint32_t,
    0x72df191b as libc::c_int as uint32_t,
    0x7580330d as libc::c_int as uint32_t,
    0x94074251 as libc::c_uint,
    0x5c7dcdfa as libc::c_int as uint32_t,
    0xabbe6d63 as libc::c_uint,
    0xaa402164 as libc::c_uint,
    0xb301d40a as libc::c_uint,
    0x2e7d1ca as libc::c_int as uint32_t,
    0x53571dae as libc::c_int as uint32_t,
    0x7a3182a2 as libc::c_int as uint32_t,
    0x12a8ddec as libc::c_int as uint32_t,
    0xfdaa335d as libc::c_uint,
    0x176f43e8 as libc::c_int as uint32_t,
    0x71fb46d4 as libc::c_int as uint32_t,
    0x38129022 as libc::c_int as uint32_t,
    0xce949ad4 as libc::c_uint,
    0xb84769ad as libc::c_uint,
    0x965bd862 as libc::c_uint,
    0x82f3d055 as libc::c_uint,
    0x66fb9767 as libc::c_int as uint32_t,
    0x15b80b4e as libc::c_int as uint32_t,
    0x1d5b47a0 as libc::c_int as uint32_t,
    0x4cfde06f as libc::c_int as uint32_t,
    0xc28ec4b8 as libc::c_uint,
    0x57e8726e as libc::c_int as uint32_t,
    0x647a78fc as libc::c_int as uint32_t,
    0x99865d44 as libc::c_uint,
    0x608bd593 as libc::c_int as uint32_t,
    0x6c200e03 as libc::c_int as uint32_t,
    0x39dc5ff6 as libc::c_int as uint32_t,
    0x5d0b00a3 as libc::c_int as uint32_t,
    0xae63aff2 as libc::c_uint,
    0x7e8bd632 as libc::c_int as uint32_t,
    0x70108c0c as libc::c_int as uint32_t,
    0xbbd35049 as libc::c_uint,
    0x2998df04 as libc::c_int as uint32_t,
    0x980cf42a as libc::c_uint,
    0x9b6df491 as libc::c_uint,
    0x9e7edd53 as libc::c_uint,
    0x6918548 as libc::c_int as uint32_t,
    0x58cb7e07 as libc::c_int as uint32_t,
    0x3b74ef2e as libc::c_int as uint32_t,
    0x522fffb1 as libc::c_int as uint32_t,
    0xd24708cc as libc::c_uint,
    0x1c7e27cd as libc::c_int as uint32_t,
    0xa4eb215b as libc::c_uint,
    0x3cf1d2e2 as libc::c_int as uint32_t,
    0x19b47a38 as libc::c_int as uint32_t,
    0x424f7618 as libc::c_int as uint32_t,
    0x35856039 as libc::c_int as uint32_t,
    0x9d17dee7 as libc::c_uint,
    0x27eb35e6 as libc::c_int as uint32_t,
    0xc9aff67b as libc::c_uint,
    0x36baf5b8 as libc::c_int as uint32_t,
    0x9c467cd as libc::c_int as uint32_t,
    0xc18910b1 as libc::c_uint,
    0xe11dbf7b as libc::c_uint,
    0x6cd1af8 as libc::c_int as uint32_t,
    0x7170c608 as libc::c_int as uint32_t,
    0x2d5e3354 as libc::c_int as uint32_t,
    0xd4de495a as libc::c_uint,
    0x64c6d006 as libc::c_int as uint32_t,
    0xbcc0c62c as libc::c_uint,
    0x3dd00db3 as libc::c_int as uint32_t,
    0x708f8f34 as libc::c_int as uint32_t,
    0x77d51b42 as libc::c_int as uint32_t,
    0x264f620f as libc::c_int as uint32_t,
    0x24b8d2bf as libc::c_int as uint32_t,
    0x15c1b79e as libc::c_int as uint32_t,
    0x46a52564 as libc::c_int as uint32_t,
    0xf8d7e54e as libc::c_uint,
    0x3e378160 as libc::c_int as uint32_t,
    0x7895cda5 as libc::c_int as uint32_t,
    0x859c15a5 as libc::c_uint,
    0xe6459788 as libc::c_uint,
    0xc37bc75f as libc::c_uint,
    0xdb07ba0c as libc::c_uint,
    0x676a3ab as libc::c_int as uint32_t,
    0x7f229b1e as libc::c_int as uint32_t,
    0x31842e7b as libc::c_int as uint32_t,
    0x24259fd7 as libc::c_int as uint32_t,
    0xf8bef472 as libc::c_uint,
    0x835ffcb8 as libc::c_uint,
    0x6df4c1f2 as libc::c_int as uint32_t,
    0x96f5b195 as libc::c_uint,
    0xfd0af0fc as libc::c_uint,
    0xb0fe134c as libc::c_uint,
    0xe2506d3d as libc::c_uint,
    0x4f9b12ea as libc::c_int as uint32_t,
    0xf215f225 as libc::c_uint,
    0xa223736f as libc::c_uint,
    0x9fb4c428 as libc::c_uint,
    0x25d04979 as libc::c_int as uint32_t,
    0x34c713f8 as libc::c_int as uint32_t,
    0xc4618187 as libc::c_uint,
    0xea7a6e98 as libc::c_uint,
    0x7cd16efc as libc::c_int as uint32_t,
    0x1436876c as libc::c_int as uint32_t,
    0xf1544107 as libc::c_uint,
    0xbedeee14 as libc::c_uint,
    0x56e9af27 as libc::c_int as uint32_t,
    0xa04aa441 as libc::c_uint,
    0x3cf7c899 as libc::c_int as uint32_t,
    0x92ecbae6 as libc::c_uint,
    0xdd67016d as libc::c_uint,
    0x151682eb as libc::c_int as uint32_t,
    0xa842eedf as libc::c_uint,
    0xfdba60b4 as libc::c_uint,
    0xf1907b75 as libc::c_uint,
    0x20e3030f as libc::c_int as uint32_t,
    0x24d8c29e as libc::c_int as uint32_t,
    0xe139673b as libc::c_uint,
    0xefa63fb8 as libc::c_uint,
    0x71873054 as libc::c_int as uint32_t,
    0xb6f2cf3b as libc::c_uint,
    0x9f326442 as libc::c_uint,
    0xcb15a4cc as libc::c_uint,
    0xb01a4504 as libc::c_uint,
    0xf1e47d8d as libc::c_uint,
    0x844a1be5 as libc::c_uint,
    0xbae7dfdc as libc::c_uint,
    0x42cbda70 as libc::c_int as uint32_t,
    0xcd7dae0a as libc::c_uint,
    0x57e85b7a as libc::c_int as uint32_t,
    0xd53f5af6 as libc::c_uint,
    0x20cf4d8c as libc::c_int as uint32_t,
    0xcea4d428 as libc::c_uint,
    0x79d130a4 as libc::c_int as uint32_t,
    0x3486ebfb as libc::c_int as uint32_t,
    0x33d3cddc as libc::c_int as uint32_t,
    0x77853b53 as libc::c_int as uint32_t,
    0x37effcb5 as libc::c_int as uint32_t,
    0xc5068778 as libc::c_uint,
    0xe580b3e6 as libc::c_uint,
    0x4e68b8f4 as libc::c_int as uint32_t,
    0xc5c8b37e as libc::c_uint,
    0xd809ea2 as libc::c_int as uint32_t,
    0x398feb7c as libc::c_int as uint32_t,
    0x132a4f94 as libc::c_int as uint32_t,
    0x43b7950e as libc::c_int as uint32_t,
    0x2fee7d1c as libc::c_int as uint32_t,
    0x223613bd as libc::c_int as uint32_t,
    0xdd06caa2 as libc::c_uint,
    0x37df932b as libc::c_int as uint32_t,
    0xc4248289 as libc::c_uint,
    0xacf3ebc3 as libc::c_uint,
    0x5715f6b7 as libc::c_int as uint32_t,
    0xef3478dd as libc::c_uint,
    0xf267616f as libc::c_uint,
    0xc148cbe4 as libc::c_uint,
    0x9052815e as libc::c_uint,
    0x5e410fab as libc::c_int as uint32_t,
    0xb48a2465 as libc::c_uint,
    0x2eda7fa4 as libc::c_int as uint32_t,
    0xe87b40e4 as libc::c_uint,
    0xe98ea084 as libc::c_uint,
    0x5889e9e1 as libc::c_int as uint32_t,
    0xefd390fc as libc::c_uint,
    0xdd07d35b as libc::c_uint,
    0xdb485694 as libc::c_uint,
    0x38d7e5b2 as libc::c_int as uint32_t,
    0x57720101 as libc::c_int as uint32_t,
    0x730edebc as libc::c_int as uint32_t,
    0x5b643113 as libc::c_int as uint32_t,
    0x94917e4f as libc::c_uint,
    0x503c2fba as libc::c_int as uint32_t,
    0x646f1282 as libc::c_int as uint32_t,
    0x7523d24a as libc::c_int as uint32_t,
    0xe0779695 as libc::c_uint,
    0xf9c17a8f as libc::c_uint,
    0x7a5b2121 as libc::c_int as uint32_t,
    0xd187b896 as libc::c_uint,
    0x29263a4d as libc::c_int as uint32_t,
    0xba510cdf as libc::c_uint,
    0x81f47c9f as libc::c_uint,
    0xad1163ed as libc::c_uint,
    0xea7b5965 as libc::c_uint,
    0x1a00726e as libc::c_int as uint32_t,
    0x11403092 as libc::c_int as uint32_t,
    0xda6d77 as libc::c_int as uint32_t,
    0x4a0cdd61 as libc::c_int as uint32_t,
    0xad1f4603 as libc::c_uint,
    0x605bdfb0 as libc::c_int as uint32_t,
    0x9eedc364 as libc::c_uint,
    0x22ebe6a8 as libc::c_int as uint32_t,
    0xcee7d28a as libc::c_uint,
    0xa0e736a0 as libc::c_uint,
    0x5564a6b9 as libc::c_int as uint32_t,
    0x10853209 as libc::c_int as uint32_t,
    0xc7eb8f37 as libc::c_uint,
    0x2de705ca as libc::c_int as uint32_t,
    0x8951570f as libc::c_uint,
    0xdf09822b as libc::c_uint,
    0xbd691a6c as libc::c_uint,
    0xaa12e4f2 as libc::c_uint,
    0x87451c0f as libc::c_uint,
    0xe0f6a27a as libc::c_uint,
    0x3ada4819 as libc::c_int as uint32_t,
    0x4cf1764f as libc::c_int as uint32_t,
    0xd771c2b as libc::c_int as uint32_t,
    0x67cdb156 as libc::c_int as uint32_t,
    0x350d8384 as libc::c_int as uint32_t,
    0x5938fa0f as libc::c_int as uint32_t,
    0x42399ef3 as libc::c_int as uint32_t,
    0x36997b07 as libc::c_int as uint32_t,
    0xe84093d as libc::c_int as uint32_t,
    0x4aa93e61 as libc::c_int as uint32_t,
    0x8360d87b as libc::c_uint,
    0x1fa98b0c as libc::c_int as uint32_t,
    0x1149382c as libc::c_int as uint32_t,
    0xe97625a5 as libc::c_uint,
    0x614d1b7 as libc::c_int as uint32_t,
    0xe25244b as libc::c_int as uint32_t,
    0xc768347 as libc::c_int as uint32_t,
    0x589e8d82 as libc::c_int as uint32_t,
    0xd2059d1 as libc::c_int as uint32_t,
    0xa466bb1e as libc::c_uint,
    0xf8da0a82 as libc::c_uint,
    0x4f19130 as libc::c_int as uint32_t,
    0xba6e4ec0 as libc::c_uint,
    0x99265164 as libc::c_uint,
    0x1ee7230d as libc::c_int as uint32_t,
    0x50b2ad80 as libc::c_int as uint32_t,
    0xeaee6801 as libc::c_uint,
    0x8db2a283 as libc::c_uint,
    0xea8bf59e as libc::c_uint,
];
pub unsafe extern "C" fn nettle_cast128_encrypt(
    mut ctx: *const cast128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
            b"cast128.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"void nettle_cast128_encrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12503: {
        if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
                b"cast128.c\0" as *const u8 as *const libc::c_char,
                99 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 92],
                    &[libc::c_char; 92],
                >(
                    b"void nettle_cast128_encrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut t: uint32_t = 0;
        let mut l: uint32_t = 0;
        let mut r: uint32_t = 0;
        l = (*src.offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*src.offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*src.offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *src.offset(3 as libc::c_int as isize) as uint32_t;
        r = (*src.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t) << 24 as libc::c_int
            | (*src.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
                as uint32_t) << 16 as libc::c_int
            | (*src.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                as uint32_t) << 8 as libc::c_int
            | *src.offset(4 as libc::c_int as isize).offset(3 as libc::c_int as isize)
                as uint32_t;
        t = ((*ctx).Km[0 as libc::c_int as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[0 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[0 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[1 as libc::c_int as usize] ^ l;
        t = t << (*ctx).Kr[1 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[1 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[2 as libc::c_int as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[2 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[2 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = ((*ctx).Km[3 as libc::c_int as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[3 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[3 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[4 as libc::c_int as usize] ^ r;
        t = t << (*ctx).Kr[4 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[4 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[5 as libc::c_int as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[5 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[5 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = ((*ctx).Km[6 as libc::c_int as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[6 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[6 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[7 as libc::c_int as usize] ^ l;
        t = t << (*ctx).Kr[7 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[7 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[8 as libc::c_int as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[8 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[8 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = ((*ctx).Km[9 as libc::c_int as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[9 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[9 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[10 as libc::c_int as usize] ^ r;
        t = t << (*ctx).Kr[10 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[10 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[11 as libc::c_int as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[11 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[11 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        if (*ctx).rounds & 16 as libc::c_int as libc::c_uint != 0 {
            t = ((*ctx).Km[12 as libc::c_int as usize]).wrapping_add(r);
            t = t << (*ctx).Kr[12 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[12 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            l
                ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                            as usize],
                    );
            t = (*ctx).Km[13 as libc::c_int as usize] ^ l;
            t = t << (*ctx).Kr[13 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[13 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            r
                ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox2[(t >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    .wrapping_add(
                        cast_sbox3[(t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize];
            t = ((*ctx).Km[14 as libc::c_int as usize]).wrapping_sub(r);
            t = t << (*ctx).Kr[14 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[14 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            l
                ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                    .wrapping_add(
                        cast_sbox2[(t >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    ^ cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                            as usize],
                    );
            t = ((*ctx).Km[15 as libc::c_int as usize]).wrapping_add(l);
            t = t << (*ctx).Kr[15 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[15 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            r
                ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                            as usize],
                    );
        }
        *dst
            .offset(
                0 as libc::c_int as isize,
            ) = (r >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(
                1 as libc::c_int as isize,
            ) = (r >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(
                2 as libc::c_int as isize,
            ) = (r >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(
                3 as libc::c_int as isize,
            ) = (r & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (l >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (l >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (l >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (l & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        length = (length as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        dst = dst.offset(8 as libc::c_int as isize);
        src = src.offset(8 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn nettle_cast128_decrypt(
    mut ctx: *const cast128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
            b"cast128.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 92],
                &[libc::c_char; 92],
            >(
                b"void nettle_cast128_decrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14699: {
        if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const libc::c_char,
                b"cast128.c\0" as *const u8 as *const libc::c_char,
                141 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 92],
                    &[libc::c_char; 92],
                >(
                    b"void nettle_cast128_decrypt(const struct cast128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut t: uint32_t = 0;
        let mut l: uint32_t = 0;
        let mut r: uint32_t = 0;
        r = (*src.offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
            | (*src.offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*src.offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *src.offset(3 as libc::c_int as isize) as uint32_t;
        l = (*src.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t) << 24 as libc::c_int
            | (*src.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
                as uint32_t) << 16 as libc::c_int
            | (*src.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                as uint32_t) << 8 as libc::c_int
            | *src.offset(4 as libc::c_int as isize).offset(3 as libc::c_int as isize)
                as uint32_t;
        if (*ctx).rounds & 16 as libc::c_int as libc::c_uint != 0 {
            t = ((*ctx).Km[15 as libc::c_int as usize]).wrapping_add(l);
            t = t << (*ctx).Kr[15 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[15 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            r
                ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                            as usize],
                    );
            t = ((*ctx).Km[14 as libc::c_int as usize]).wrapping_sub(r);
            t = t << (*ctx).Kr[14 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[14 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            l
                ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                    .wrapping_add(
                        cast_sbox2[(t >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    ^ cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                            as usize],
                    );
            t = (*ctx).Km[13 as libc::c_int as usize] ^ l;
            t = t << (*ctx).Kr[13 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[13 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            r
                ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox2[(t >> 16 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    .wrapping_add(
                        cast_sbox3[(t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize];
            t = ((*ctx).Km[12 as libc::c_int as usize]).wrapping_add(r);
            t = t << (*ctx).Kr[12 as libc::c_int as usize] as libc::c_int
                | t
                    >> (-((*ctx).Kr[12 as libc::c_int as usize] as libc::c_int)
                        & 31 as libc::c_int);
            l
                ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                    ^ cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                    .wrapping_sub(
                        cast_sbox3[(t >> 8 as libc::c_int
                            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                    )
                    .wrapping_add(
                        cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                            as usize],
                    );
        }
        t = ((*ctx).Km[11 as libc::c_int as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[11 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[11 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[10 as libc::c_int as usize] ^ r;
        t = t << (*ctx).Kr[10 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[10 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[9 as libc::c_int as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[9 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[9 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = ((*ctx).Km[8 as libc::c_int as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[8 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[8 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[7 as libc::c_int as usize] ^ l;
        t = t << (*ctx).Kr[7 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[7 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[6 as libc::c_int as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[6 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[6 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = ((*ctx).Km[5 as libc::c_int as usize]).wrapping_sub(l);
        t = t << (*ctx).Kr[5 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[5 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[4 as libc::c_int as usize] ^ r;
        t = t << (*ctx).Kr[4 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[4 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[3 as libc::c_int as usize]).wrapping_add(l);
        t = t << (*ctx).Kr[3 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[3 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = ((*ctx).Km[2 as libc::c_int as usize]).wrapping_sub(r);
        t = t << (*ctx).Kr[2 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[2 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= ((cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_add(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox3[(t >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        t = (*ctx).Km[1 as libc::c_int as usize] ^ l;
        t = t << (*ctx).Kr[1 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[1 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        r
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox2[(t >> 16 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                ^ cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                    as usize];
        t = ((*ctx).Km[0 as libc::c_int as usize]).wrapping_add(r);
        t = t << (*ctx).Kr[0 as libc::c_int as usize] as libc::c_int
            | t
                >> (-((*ctx).Kr[0 as libc::c_int as usize] as libc::c_int)
                    & 31 as libc::c_int);
        l
            ^= (cast_sbox1[(t >> 24 as libc::c_int) as uint8_t as usize]
                ^ cast_sbox2[(t >> 16 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
                .wrapping_sub(
                    cast_sbox3[(t >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize],
                )
                .wrapping_add(
                    cast_sbox4[(t & 0xff as libc::c_int as libc::c_uint) as uint8_t
                        as usize],
                );
        *dst
            .offset(
                0 as libc::c_int as isize,
            ) = (l >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(
                1 as libc::c_int as isize,
            ) = (l >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(
                2 as libc::c_int as isize,
            ) = (l >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(
                3 as libc::c_int as isize,
            ) = (l & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (r >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (r >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (r >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        *dst
            .offset(4 as libc::c_int as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (r & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        length = (length as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        dst = dst.offset(8 as libc::c_int as isize);
        src = src.offset(8 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn nettle_cast5_set_key(
    mut ctx: *mut cast128_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    let mut x0: uint32_t = 0;
    let mut x1: uint32_t = 0;
    let mut x2: uint32_t = 0;
    let mut x3: uint32_t = 0;
    let mut z0: uint32_t = 0;
    let mut z1: uint32_t = 0;
    let mut z2: uint32_t = 0;
    let mut z3: uint32_t = 0;
    let mut w: uint32_t = 0;
    let mut full: libc::c_int = 0;
    if length >= 5 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length >= CAST5_MIN_KEY_SIZE\0" as *const u8 as *const libc::c_char,
            b"cast128.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8759: {
        if length >= 5 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length >= CAST5_MIN_KEY_SIZE\0" as *const u8 as *const libc::c_char,
                b"cast128.c\0" as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= CAST5_MAX_KEY_SIZE\0" as *const u8 as *const libc::c_char,
            b"cast128.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8716: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= CAST5_MAX_KEY_SIZE\0" as *const u8 as *const libc::c_char,
                b"cast128.c\0" as *const u8 as *const libc::c_char,
                234 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void nettle_cast5_set_key(struct cast128_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    full = (length > 10 as libc::c_int as libc::c_ulong) as libc::c_int;
    x0 = (*key.offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*key.offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*key.offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *key.offset(3 as libc::c_int as isize) as uint32_t;
    match length & 3 as libc::c_int as libc::c_ulong {
        0 => {
            w = (*key
                .offset(length as isize)
                .offset(-(4 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
                | (*key
                    .offset(length as isize)
                    .offset(-(4 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                | (*key
                    .offset(length as isize)
                    .offset(-(4 as libc::c_int as isize))
                    .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *key
                    .offset(length as isize)
                    .offset(-(4 as libc::c_int as isize))
                    .offset(3 as libc::c_int as isize) as uint32_t;
        }
        3 => {
            w = ((*key
                .offset(length as isize)
                .offset(-(3 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                | (*key
                    .offset(length as isize)
                    .offset(-(3 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *key
                    .offset(length as isize)
                    .offset(-(3 as libc::c_int as isize))
                    .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int;
        }
        2 => {
            w = ((*key
                .offset(length as isize)
                .offset(-(2 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *key
                    .offset(length as isize)
                    .offset(-(2 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int;
        }
        1 => {
            w = (*key
                .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as uint32_t) << 24 as libc::c_int;
        }
        _ => {}
    }
    if length <= 8 as libc::c_int as libc::c_ulong {
        x1 = w;
        x3 = 0 as libc::c_int as uint32_t;
        x2 = x3;
    } else {
        x1 = (*key.offset(4 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as uint32_t) << 24 as libc::c_int
            | (*key.offset(4 as libc::c_int as isize).offset(1 as libc::c_int as isize)
                as uint32_t) << 16 as libc::c_int
            | (*key.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                as uint32_t) << 8 as libc::c_int
            | *key.offset(4 as libc::c_int as isize).offset(3 as libc::c_int as isize)
                as uint32_t;
        if length <= 12 as libc::c_int as libc::c_ulong {
            x2 = w;
            x3 = 0 as libc::c_int as uint32_t;
        } else {
            x2 = (*key
                .offset(8 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
                | (*key
                    .offset(8 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                | (*key
                    .offset(8 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *key
                    .offset(8 as libc::c_int as isize)
                    .offset(3 as libc::c_int as isize) as uint32_t;
            x3 = w;
        }
    }
    z0 = x0
        ^ cast_sbox5[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z3 = x1
        ^ cast_sbox5[(z2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    (*ctx)
        .Km[0 as libc::c_int
        as usize] = cast_sbox5[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    (*ctx)
        .Km[1 as libc::c_int
        as usize] = cast_sbox5[(z2 >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    (*ctx)
        .Km[2 as libc::c_int
        as usize] = cast_sbox5[(z3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    (*ctx)
        .Km[3 as libc::c_int
        as usize] = cast_sbox5[(z3 >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 24 as libc::c_int) as uint8_t as usize];
    x0 = z2
        ^ cast_sbox5[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as libc::c_int) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x3 = z3
        ^ cast_sbox5[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    (*ctx)
        .Km[4 as libc::c_int
        as usize] = cast_sbox5[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 24 as libc::c_int) as uint8_t as usize];
    (*ctx)
        .Km[5 as libc::c_int
        as usize] = cast_sbox5[(x0 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    (*ctx)
        .Km[6 as libc::c_int
        as usize] = cast_sbox5[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    (*ctx)
        .Km[7 as libc::c_int
        as usize] = cast_sbox5[(x1 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    z0 = x0
        ^ cast_sbox5[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z3 = x1
        ^ cast_sbox5[(z2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    (*ctx)
        .Km[8 as libc::c_int
        as usize] = cast_sbox5[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox5[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    (*ctx)
        .Km[9 as libc::c_int
        as usize] = cast_sbox5[(z0 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 24 as libc::c_int) as uint8_t as usize];
    (*ctx)
        .Km[10 as libc::c_int
        as usize] = cast_sbox5[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    (*ctx)
        .Km[11 as libc::c_int
        as usize] = cast_sbox5[(z1 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x0 = z2
        ^ cast_sbox5[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as libc::c_int) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x3 = z3
        ^ cast_sbox5[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    if full != 0 {
        (*ctx)
            .Km[12 as libc::c_int
            as usize] = cast_sbox5[(x2 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox6[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox7[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox5[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
        (*ctx)
            .Km[13 as libc::c_int
            as usize] = cast_sbox5[(x2 >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox6[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox7[(x1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox6[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
        (*ctx)
            .Km[14 as libc::c_int
            as usize] = cast_sbox5[(x3 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox6[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox7[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize];
        (*ctx)
            .Km[15 as libc::c_int
            as usize] = cast_sbox5[(x3 >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox6[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox7[(x0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox8[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize];
    }
    z0 = x0
        ^ cast_sbox5[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z3 = x1
        ^ cast_sbox5[(z2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    (*ctx)
        .Kr[0 as libc::c_int
        as usize] = ((cast_sbox5[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[1 as libc::c_int
        as usize] = ((cast_sbox5[(z2 >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[2 as libc::c_int
        as usize] = ((cast_sbox5[(z3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[3 as libc::c_int
        as usize] = ((cast_sbox5[(z3 >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 24 as libc::c_int) as uint8_t as usize])
        & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    x0 = z2
        ^ cast_sbox5[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as libc::c_int) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x3 = z3
        ^ cast_sbox5[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    (*ctx)
        .Kr[4 as libc::c_int
        as usize] = ((cast_sbox5[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 24 as libc::c_int) as uint8_t as usize])
        & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[5 as libc::c_int
        as usize] = ((cast_sbox5[(x0 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[6 as libc::c_int
        as usize] = ((cast_sbox5[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
        & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[7 as libc::c_int
        as usize] = ((cast_sbox5[(x1 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
        & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    z0 = x0
        ^ cast_sbox5[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(x3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(x3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize];
    z1 = x2 ^ cast_sbox5[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z2 = x3 ^ cast_sbox5[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    z3 = x1
        ^ cast_sbox5[(z2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    (*ctx)
        .Kr[8 as libc::c_int
        as usize] = ((cast_sbox5[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox5[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[9 as libc::c_int
        as usize] = ((cast_sbox5[(z0 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(z3 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z3 >> 24 as libc::c_int) as uint8_t as usize])
        & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[10 as libc::c_int
        as usize] = ((cast_sbox5[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t
        as usize]
        ^ cast_sbox6[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    (*ctx)
        .Kr[11 as libc::c_int
        as usize] = ((cast_sbox5[(z1 >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox7[(z2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(z2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
    x0 = z2
        ^ cast_sbox5[(z1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(z1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox7[(z1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox8[(z1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(z0 >> 24 as libc::c_int) as uint8_t as usize];
    x1 = z0 ^ cast_sbox5[(x0 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(z0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x2 = z1 ^ cast_sbox5[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox6[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox8[(x1 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox5[(z0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize];
    x3 = z3
        ^ cast_sbox5[(x2 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox6[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t as usize]
        ^ cast_sbox7[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
        ^ cast_sbox8[(x2 >> 24 as libc::c_int) as uint8_t as usize]
        ^ cast_sbox6[(z0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize];
    if full != 0 {
        (*ctx)
            .Kr[12 as libc::c_int
            as usize] = ((cast_sbox5[(x2 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox6[(x2 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox7[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox5[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
            & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
        (*ctx)
            .Kr[13 as libc::c_int
            as usize] = ((cast_sbox5[(x2 >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox6[(x2 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox7[(x1 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox8[(x1 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox6[(x1 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize])
            & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
        (*ctx)
            .Kr[14 as libc::c_int
            as usize] = ((cast_sbox5[(x3 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox6[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox7[(x0 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox7[(x2 >> 24 as libc::c_int) as uint8_t as usize])
            & 31 as libc::c_int as libc::c_uint) as libc::c_uchar;
        (*ctx)
            .Kr[15 as libc::c_int
            as usize] = ((cast_sbox5[(x3 >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox6[(x3 & 0xff as libc::c_int as libc::c_uint) as uint8_t as usize]
            ^ cast_sbox7[(x0 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]
            ^ cast_sbox8[(x0 >> 24 as libc::c_int) as uint8_t as usize]
            ^ cast_sbox8[(x3 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as uint8_t as usize]) & 31 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
    }
    (*ctx)
        .rounds = (if full != 0 { 16 as libc::c_int } else { 12 as libc::c_int })
        as libc::c_uint;
}
pub unsafe extern "C" fn nettle_cast128_set_key(
    mut ctx: *mut cast128_ctx,
    mut key: *const uint8_t,
) {
    nettle_cast5_set_key(ctx, 16 as libc::c_int as size_t, key);
}
