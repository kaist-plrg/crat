use ::libc;
extern "C" {
    fn BrotliGetDictionary() -> *const BrotliDictionary;
    fn BrotliGetTransforms() -> *const BrotliTransforms;
    static kStaticDictionaryHashWords: [uint16_t; 32768];
    static kStaticDictionaryHashLengths: [uint8_t; 32768];
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [uint8_t; 32],
    pub offsets_by_length: [uint32_t; 32],
    pub data_size: size_t,
    pub data: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DictWord {
    pub len: uint8_t,
    pub transform: uint8_t,
    pub idx: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderDictionary {
    pub words: *const BrotliDictionary,
    pub num_transforms: uint32_t,
    pub cutoffTransformsCount: uint32_t,
    pub cutoffTransforms: uint64_t,
    pub hash_table_words: *const uint16_t,
    pub hash_table_lengths: *const uint8_t,
    pub buckets: *const uint16_t,
    pub dict_words: *const DictWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTransforms {
    pub prefix_suffix_size: uint16_t,
    pub prefix_suffix: *const uint8_t,
    pub prefix_suffix_map: *const uint16_t,
    pub num_transforms: uint32_t,
    pub transforms: *const uint8_t,
    pub params: *const uint8_t,
    pub cutOffTransforms: [int16_t; 10],
}
static mut kStaticDictionaryBuckets: [uint16_t; 32768] = [
0; 32768
];
static mut kStaticDictionaryWords: [DictWord; 31705] = [
DictWord { len: 0, transform: 0, idx: 0 }; 31705
];
static mut kCutoffTransforms: uint64_t = (0x71b520a as libc::c_int as uint64_t)
    << 32 as libc::c_int | 0xda2d3200 as libc::c_uint as libc::c_ulong;
static mut kCutoffTransformsCount: uint32_t = 10 as libc::c_int as uint32_t;
pub unsafe extern "C" fn BrotliInitEncoderDictionary(
    mut dict: *mut BrotliEncoderDictionary,
) {
    (*dict).words = BrotliGetDictionary();
    (*dict).num_transforms = (*BrotliGetTransforms()).num_transforms;
    (*dict).hash_table_words = kStaticDictionaryHashWords.as_ptr();
    (*dict).hash_table_lengths = kStaticDictionaryHashLengths.as_ptr();
    (*dict).buckets = kStaticDictionaryBuckets.as_ptr();
    (*dict).dict_words = kStaticDictionaryWords.as_ptr();
    (*dict).cutoffTransformsCount = kCutoffTransformsCount;
    (*dict).cutoffTransforms = kCutoffTransforms;
}
