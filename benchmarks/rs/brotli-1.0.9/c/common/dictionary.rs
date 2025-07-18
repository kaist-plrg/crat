use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
}
static mut kBrotliDictionaryData: [uint8_t; 122784] = [
0; 122784
];
static mut kBrotliDictionary: BrotliDictionary = unsafe {
    {
        let mut init = BrotliDictionary {
            size_bits_by_length: [
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                10 as libc::c_int as uint8_t,
                10 as libc::c_int as uint8_t,
                11 as libc::c_int as uint8_t,
                11 as libc::c_int as uint8_t,
                10 as libc::c_int as uint8_t,
                10 as libc::c_int as uint8_t,
                10 as libc::c_int as uint8_t,
                10 as libc::c_int as uint8_t,
                10 as libc::c_int as uint8_t,
                9 as libc::c_int as uint8_t,
                9 as libc::c_int as uint8_t,
                8 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
                8 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
                5 as libc::c_int as uint8_t,
                5 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
            ],
            offsets_by_length: [
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                4096 as libc::c_int as uint32_t,
                9216 as libc::c_int as uint32_t,
                21504 as libc::c_int as uint32_t,
                35840 as libc::c_int as uint32_t,
                44032 as libc::c_int as uint32_t,
                53248 as libc::c_int as uint32_t,
                63488 as libc::c_int as uint32_t,
                74752 as libc::c_int as uint32_t,
                87040 as libc::c_int as uint32_t,
                93696 as libc::c_int as uint32_t,
                100864 as libc::c_int as uint32_t,
                104704 as libc::c_int as uint32_t,
                106752 as libc::c_int as uint32_t,
                108928 as libc::c_int as uint32_t,
                113536 as libc::c_int as uint32_t,
                115968 as libc::c_int as uint32_t,
                118528 as libc::c_int as uint32_t,
                119872 as libc::c_int as uint32_t,
                121280 as libc::c_int as uint32_t,
                122016 as libc::c_int as uint32_t,
                122784 as libc::c_int as uint32_t,
                122784 as libc::c_int as uint32_t,
                122784 as libc::c_int as uint32_t,
                122784 as libc::c_int as uint32_t,
                122784 as libc::c_int as uint32_t,
                122784 as libc::c_int as uint32_t,
                122784 as libc::c_int as uint32_t,
            ],
            data_size: 122784 as libc::c_int as size_t,
            data: kBrotliDictionaryData.as_ptr(),
        };
        init
    }
};
pub unsafe extern "C" fn BrotliGetDictionary() -> *const BrotliDictionary {
    return &kBrotliDictionary;
}
pub unsafe extern "C" fn BrotliSetDictionaryData(mut data: *const uint8_t) {}
