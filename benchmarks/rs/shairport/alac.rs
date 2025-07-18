use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type FILE = _IO_FILE;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alac_file {
    pub input_buffer: *mut libc::c_uchar,
    pub input_buffer_bitaccumulator: libc::c_int,
    pub samplesize: libc::c_int,
    pub numchannels: libc::c_int,
    pub bytespersample: libc::c_int,
    pub predicterror_buffer_a: *mut int32_t,
    pub predicterror_buffer_b: *mut int32_t,
    pub outputsamples_buffer_a: *mut int32_t,
    pub outputsamples_buffer_b: *mut int32_t,
    pub uncompressed_bytes_buffer_a: *mut int32_t,
    pub uncompressed_bytes_buffer_b: *mut int32_t,
    pub setinfo_max_samples_per_frame: uint32_t,
    pub setinfo_7a: uint8_t,
    pub setinfo_sample_size: uint8_t,
    pub setinfo_rice_historymult: uint8_t,
    pub setinfo_rice_initialhistory: uint8_t,
    pub setinfo_rice_kmodifier: uint8_t,
    pub setinfo_7f: uint8_t,
    pub setinfo_80: uint16_t,
    pub setinfo_82: uint32_t,
    pub setinfo_86: uint32_t,
    pub setinfo_8a_rate: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed {
    #[bitfield(name = "x", ty = "libc::c_int", bits = "0..=23")]
    pub x: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
static mut host_bigendian: libc::c_int = 0 as libc::c_int;
pub static mut se_struct_24: C2RustUnnamed = C2RustUnnamed {
    x: [0; 3],
    c2rust_padding: [0; 1],
};
pub unsafe extern "C" fn alac_free(mut alac: *mut alac_file) {
    if !((*alac).predicterror_buffer_a).is_null() {
        free((*alac).predicterror_buffer_a as *mut libc::c_void);
    }
    if !((*alac).predicterror_buffer_b).is_null() {
        free((*alac).predicterror_buffer_b as *mut libc::c_void);
    }
    if !((*alac).outputsamples_buffer_a).is_null() {
        free((*alac).outputsamples_buffer_a as *mut libc::c_void);
    }
    if !((*alac).outputsamples_buffer_b).is_null() {
        free((*alac).outputsamples_buffer_b as *mut libc::c_void);
    }
    if !((*alac).uncompressed_bytes_buffer_a).is_null() {
        free((*alac).uncompressed_bytes_buffer_a as *mut libc::c_void);
    }
    if !((*alac).uncompressed_bytes_buffer_b).is_null() {
        free((*alac).uncompressed_bytes_buffer_b as *mut libc::c_void);
    }
    free(alac as *mut libc::c_void);
}
pub unsafe extern "C" fn alac_allocate_buffers(mut alac: *mut alac_file) {
    (*alac)
        .predicterror_buffer_a = malloc(
        ((*alac).setinfo_max_samples_per_frame)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut int32_t;
    (*alac)
        .predicterror_buffer_b = malloc(
        ((*alac).setinfo_max_samples_per_frame)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut int32_t;
    (*alac)
        .outputsamples_buffer_a = malloc(
        ((*alac).setinfo_max_samples_per_frame)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut int32_t;
    (*alac)
        .outputsamples_buffer_b = malloc(
        ((*alac).setinfo_max_samples_per_frame)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut int32_t;
    (*alac)
        .uncompressed_bytes_buffer_a = malloc(
        ((*alac).setinfo_max_samples_per_frame)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut int32_t;
    (*alac)
        .uncompressed_bytes_buffer_b = malloc(
        ((*alac).setinfo_max_samples_per_frame)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut int32_t;
}
pub unsafe extern "C" fn alac_set_info(
    mut alac: *mut alac_file,
    mut inputbuffer: *mut libc::c_char,
) {
    let mut ptr: *mut libc::c_char = inputbuffer;
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_max_samples_per_frame = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_max_samples_per_frame = ((*alac).setinfo_max_samples_per_frame
            & 0xff as libc::c_int as libc::c_uint) << 0x18 as libc::c_int
            | ((*alac).setinfo_max_samples_per_frame
                & 0xff00 as libc::c_int as libc::c_uint) << 0x8 as libc::c_int
            | ((*alac).setinfo_max_samples_per_frame
                & 0xff0000 as libc::c_int as libc::c_uint) >> 0x8 as libc::c_int
            | ((*alac).setinfo_max_samples_per_frame & 0xff000000 as libc::c_uint)
                >> 0x18 as libc::c_int;
    }
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_7a = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1 as libc::c_int as isize);
    (*alac).setinfo_sample_size = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1 as libc::c_int as isize);
    (*alac).setinfo_rice_historymult = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1 as libc::c_int as isize);
    (*alac).setinfo_rice_initialhistory = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1 as libc::c_int as isize);
    (*alac).setinfo_rice_kmodifier = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1 as libc::c_int as isize);
    (*alac).setinfo_7f = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1 as libc::c_int as isize);
    (*alac).setinfo_80 = *(ptr as *mut uint16_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_80 = (((*alac).setinfo_80 as libc::c_int & 0xff as libc::c_int)
            << 0x8 as libc::c_int
            | ((*alac).setinfo_80 as libc::c_int & 0xff00 as libc::c_int)
                >> 0x8 as libc::c_int) as uint16_t;
    }
    ptr = ptr.offset(2 as libc::c_int as isize);
    (*alac).setinfo_82 = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_82 = ((*alac).setinfo_82 & 0xff as libc::c_int as libc::c_uint)
            << 0x18 as libc::c_int
            | ((*alac).setinfo_82 & 0xff00 as libc::c_int as libc::c_uint)
                << 0x8 as libc::c_int
            | ((*alac).setinfo_82 & 0xff0000 as libc::c_int as libc::c_uint)
                >> 0x8 as libc::c_int
            | ((*alac).setinfo_82 & 0xff000000 as libc::c_uint) >> 0x18 as libc::c_int;
    }
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_86 = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_86 = ((*alac).setinfo_86 & 0xff as libc::c_int as libc::c_uint)
            << 0x18 as libc::c_int
            | ((*alac).setinfo_86 & 0xff00 as libc::c_int as libc::c_uint)
                << 0x8 as libc::c_int
            | ((*alac).setinfo_86 & 0xff0000 as libc::c_int as libc::c_uint)
                >> 0x8 as libc::c_int
            | ((*alac).setinfo_86 & 0xff000000 as libc::c_uint) >> 0x18 as libc::c_int;
    }
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_8a_rate = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_8a_rate = ((*alac).setinfo_8a_rate
            & 0xff as libc::c_int as libc::c_uint) << 0x18 as libc::c_int
            | ((*alac).setinfo_8a_rate & 0xff00 as libc::c_int as libc::c_uint)
                << 0x8 as libc::c_int
            | ((*alac).setinfo_8a_rate & 0xff0000 as libc::c_int as libc::c_uint)
                >> 0x8 as libc::c_int
            | ((*alac).setinfo_8a_rate & 0xff000000 as libc::c_uint)
                >> 0x18 as libc::c_int;
    }
    alac_allocate_buffers(alac);
}
unsafe extern "C" fn readbits_16(
    mut alac: *mut alac_file,
    mut bits: libc::c_int,
) -> uint32_t {
    let mut result: uint32_t = 0;
    let mut new_accumulator: libc::c_int = 0;
    result = ((*((*alac).input_buffer).offset(0 as libc::c_int as isize) as libc::c_int)
        << 16 as libc::c_int
        | (*((*alac).input_buffer).offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int
        | *((*alac).input_buffer).offset(2 as libc::c_int as isize) as libc::c_int)
        as uint32_t;
    result = result << (*alac).input_buffer_bitaccumulator;
    result = result & 0xffffff as libc::c_int as libc::c_uint;
    result = result >> 24 as libc::c_int - bits;
    new_accumulator = (*alac).input_buffer_bitaccumulator + bits;
    (*alac)
        .input_buffer = ((*alac).input_buffer)
        .offset((new_accumulator >> 3 as libc::c_int) as isize);
    (*alac).input_buffer_bitaccumulator = new_accumulator & 7 as libc::c_int;
    return result;
}
unsafe extern "C" fn readbits(
    mut alac: *mut alac_file,
    mut bits: libc::c_int,
) -> uint32_t {
    let mut result: int32_t = 0 as libc::c_int;
    if bits > 16 as libc::c_int {
        bits -= 16 as libc::c_int;
        result = (readbits_16(alac, 16 as libc::c_int) << bits) as int32_t;
    }
    result = (result as libc::c_uint | readbits_16(alac, bits)) as int32_t;
    return result as uint32_t;
}
unsafe extern "C" fn readbit(mut alac: *mut alac_file) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut new_accumulator: libc::c_int = 0;
    result = *((*alac).input_buffer).offset(0 as libc::c_int as isize) as libc::c_int;
    result = result << (*alac).input_buffer_bitaccumulator;
    result = result >> 7 as libc::c_int & 1 as libc::c_int;
    new_accumulator = (*alac).input_buffer_bitaccumulator + 1 as libc::c_int;
    (*alac)
        .input_buffer = ((*alac).input_buffer)
        .offset((new_accumulator / 8 as libc::c_int) as isize);
    (*alac).input_buffer_bitaccumulator = new_accumulator % 8 as libc::c_int;
    return result;
}
unsafe extern "C" fn unreadbits(mut alac: *mut alac_file, mut bits: libc::c_int) {
    let mut new_accumulator: libc::c_int = (*alac).input_buffer_bitaccumulator - bits;
    (*alac)
        .input_buffer = ((*alac).input_buffer)
        .offset((new_accumulator >> 3 as libc::c_int) as isize);
    (*alac).input_buffer_bitaccumulator = new_accumulator & 7 as libc::c_int;
    if (*alac).input_buffer_bitaccumulator < 0 as libc::c_int {
        (*alac).input_buffer_bitaccumulator *= -(1 as libc::c_int);
    }
}
unsafe extern "C" fn count_leading_zeros(mut input: libc::c_int) -> libc::c_int {
    return (input as libc::c_uint).leading_zeros() as i32;
}
unsafe extern "C" fn entropy_decode_value(
    mut alac: *mut alac_file,
    mut readSampleSize: libc::c_int,
    mut k: libc::c_int,
    mut rice_kmodifier_mask: libc::c_int,
) -> int32_t {
    let mut x: int32_t = 0 as libc::c_int;
    while x <= 8 as libc::c_int && readbit(alac) != 0 {
        x += 1;
        x;
    }
    if x > 8 as libc::c_int {
        let mut value: int32_t = 0;
        value = readbits(alac, readSampleSize) as int32_t;
        value = (value as libc::c_uint
            & 0xffffffff as libc::c_uint >> 32 as libc::c_int - readSampleSize)
            as int32_t;
        x = value;
    } else if k != 1 as libc::c_int {
        let mut extraBits: libc::c_int = readbits(alac, k) as libc::c_int;
        x *= ((1 as libc::c_int) << k) - 1 as libc::c_int & rice_kmodifier_mask;
        if extraBits > 1 as libc::c_int {
            x += extraBits - 1 as libc::c_int;
        } else {
            unreadbits(alac, 1 as libc::c_int);
        }
    }
    return x;
}
unsafe extern "C" fn entropy_rice_decode(
    mut alac: *mut alac_file,
    mut outputBuffer: *mut int32_t,
    mut outputSize: libc::c_int,
    mut readSampleSize: libc::c_int,
    mut rice_initialhistory: libc::c_int,
    mut rice_kmodifier: libc::c_int,
    mut rice_historymult: libc::c_int,
    mut rice_kmodifier_mask: libc::c_int,
) {
    let mut outputCount: libc::c_int = 0;
    let mut history: libc::c_int = rice_initialhistory;
    let mut signModifier: libc::c_int = 0 as libc::c_int;
    outputCount = 0 as libc::c_int;
    while outputCount < outputSize {
        let mut decodedValue: int32_t = 0;
        let mut finalValue: int32_t = 0;
        let mut k: int32_t = 0;
        k = 31 as libc::c_int - rice_kmodifier
            - count_leading_zeros((history >> 9 as libc::c_int) + 3 as libc::c_int);
        if k < 0 as libc::c_int {
            k += rice_kmodifier;
        } else {
            k = rice_kmodifier;
        }
        decodedValue = entropy_decode_value(
            alac,
            readSampleSize,
            k,
            0xffffffff as libc::c_uint as libc::c_int,
        );
        decodedValue += signModifier;
        finalValue = (decodedValue + 1 as libc::c_int) / 2 as libc::c_int;
        if decodedValue & 1 as libc::c_int != 0 {
            finalValue *= -(1 as libc::c_int);
        }
        *outputBuffer.offset(outputCount as isize) = finalValue;
        signModifier = 0 as libc::c_int;
        history
            += decodedValue * rice_historymult
                - (history * rice_historymult >> 9 as libc::c_int);
        if decodedValue > 0xffff as libc::c_int {
            history = 0xffff as libc::c_int;
        }
        if history < 128 as libc::c_int && (outputCount + 1 as libc::c_int) < outputSize
        {
            let mut blockSize: int32_t = 0;
            signModifier = 1 as libc::c_int;
            k = count_leading_zeros(history)
                + (history + 16 as libc::c_int) / 64 as libc::c_int - 24 as libc::c_int;
            blockSize = entropy_decode_value(
                alac,
                16 as libc::c_int,
                k,
                rice_kmodifier_mask,
            );
            if blockSize > 0 as libc::c_int {
                memset(
                    &mut *outputBuffer.offset((outputCount + 1 as libc::c_int) as isize)
                        as *mut int32_t as *mut libc::c_void,
                    0 as libc::c_int,
                    (blockSize as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
                );
                outputCount += blockSize;
            }
            if blockSize > 0xffff as libc::c_int {
                signModifier = 0 as libc::c_int;
            }
            history = 0 as libc::c_int;
        }
        outputCount += 1;
        outputCount;
    }
}
unsafe extern "C" fn predictor_decompress_fir_adapt(
    mut error_buffer: *mut int32_t,
    mut buffer_out: *mut int32_t,
    mut output_size: libc::c_int,
    mut readsamplesize: libc::c_int,
    mut predictor_coef_table: *mut int16_t,
    mut predictor_coef_num: libc::c_int,
    mut predictor_quantitization: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    *buffer_out = *error_buffer;
    if predictor_coef_num == 0 {
        if output_size <= 1 as libc::c_int {
            return;
        }
        memcpy(
            buffer_out.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            error_buffer.offset(1 as libc::c_int as isize) as *const libc::c_void,
            ((output_size - 1 as libc::c_int) * 4 as libc::c_int) as libc::c_ulong,
        );
        return;
    }
    if predictor_coef_num == 0x1f as libc::c_int {
        if output_size <= 1 as libc::c_int {
            return;
        }
        i = 0 as libc::c_int;
        while i < output_size - 1 as libc::c_int {
            let mut prev_value: int32_t = 0;
            let mut error_value: int32_t = 0;
            prev_value = *buffer_out.offset(i as isize);
            error_value = *error_buffer.offset((i + 1 as libc::c_int) as isize);
            *buffer_out
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = prev_value + error_value << 32 as libc::c_int - readsamplesize
                >> 32 as libc::c_int - readsamplesize;
            i += 1;
            i;
        }
        return;
    }
    if predictor_coef_num > 0 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < predictor_coef_num {
            let mut val: int32_t = 0;
            val = *buffer_out.offset(i_0 as isize)
                + *error_buffer.offset((i_0 + 1 as libc::c_int) as isize);
            val = val << 32 as libc::c_int - readsamplesize
                >> 32 as libc::c_int - readsamplesize;
            *buffer_out.offset((i_0 + 1 as libc::c_int) as isize) = val;
            i_0 += 1;
            i_0;
        }
    }
    if predictor_coef_num > 0 as libc::c_int {
        i = predictor_coef_num + 1 as libc::c_int;
        while i < output_size {
            let mut j: libc::c_int = 0;
            let mut sum: libc::c_int = 0 as libc::c_int;
            let mut outval: libc::c_int = 0;
            let mut error_val: libc::c_int = *error_buffer.offset(i as isize);
            j = 0 as libc::c_int;
            while j < predictor_coef_num {
                sum
                    += (*buffer_out.offset((predictor_coef_num - j) as isize)
                        - *buffer_out.offset(0 as libc::c_int as isize))
                        * *predictor_coef_table.offset(j as isize) as libc::c_int;
                j += 1;
                j;
            }
            outval = ((1 as libc::c_int) << predictor_quantitization - 1 as libc::c_int)
                + sum;
            outval = outval >> predictor_quantitization;
            outval = outval + *buffer_out.offset(0 as libc::c_int as isize) + error_val;
            outval = outval << 32 as libc::c_int - readsamplesize
                >> 32 as libc::c_int - readsamplesize;
            *buffer_out
                .offset((predictor_coef_num + 1 as libc::c_int) as isize) = outval;
            if error_val > 0 as libc::c_int {
                let mut predictor_num: libc::c_int = predictor_coef_num
                    - 1 as libc::c_int;
                while predictor_num >= 0 as libc::c_int && error_val > 0 as libc::c_int {
                    let mut val_0: libc::c_int = *buffer_out
                        .offset(0 as libc::c_int as isize)
                        - *buffer_out
                            .offset((predictor_coef_num - predictor_num) as isize);
                    let mut sign: libc::c_int = if val_0 < 0 as libc::c_int {
                        -(1 as libc::c_int)
                    } else if val_0 > 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    let ref mut fresh0 = *predictor_coef_table
                        .offset(predictor_num as isize);
                    *fresh0 = (*fresh0 as libc::c_int - sign) as int16_t;
                    val_0 *= sign;
                    error_val
                        -= (val_0 >> predictor_quantitization)
                            * (predictor_coef_num - predictor_num);
                    predictor_num -= 1;
                    predictor_num;
                }
            } else if error_val < 0 as libc::c_int {
                let mut predictor_num_0: libc::c_int = predictor_coef_num
                    - 1 as libc::c_int;
                while predictor_num_0 >= 0 as libc::c_int && error_val < 0 as libc::c_int
                {
                    let mut val_1: libc::c_int = *buffer_out
                        .offset(0 as libc::c_int as isize)
                        - *buffer_out
                            .offset((predictor_coef_num - predictor_num_0) as isize);
                    let mut sign_0: libc::c_int = -if val_1 < 0 as libc::c_int {
                        -(1 as libc::c_int)
                    } else if val_1 > 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    let ref mut fresh1 = *predictor_coef_table
                        .offset(predictor_num_0 as isize);
                    *fresh1 = (*fresh1 as libc::c_int - sign_0) as int16_t;
                    val_1 *= sign_0;
                    error_val
                        -= (val_1 >> predictor_quantitization)
                            * (predictor_coef_num - predictor_num_0);
                    predictor_num_0 -= 1;
                    predictor_num_0;
                }
            }
            buffer_out = buffer_out.offset(1);
            buffer_out;
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn deinterlace_16(
    mut buffer_a: *mut int32_t,
    mut buffer_b: *mut int32_t,
    mut buffer_out: *mut int16_t,
    mut numchannels: libc::c_int,
    mut numsamples: libc::c_int,
    mut interlacing_shift: uint8_t,
    mut interlacing_leftweight: uint8_t,
) {
    let mut i: libc::c_int = 0;
    if numsamples <= 0 as libc::c_int {
        return;
    }
    if interlacing_leftweight != 0 {
        i = 0 as libc::c_int;
        while i < numsamples {
            let mut difference: int32_t = 0;
            let mut midright: int32_t = 0;
            let mut left: int16_t = 0;
            let mut right: int16_t = 0;
            midright = *buffer_a.offset(i as isize);
            difference = *buffer_b.offset(i as isize);
            right = (midright
                - (difference * interlacing_leftweight as libc::c_int
                    >> interlacing_shift as libc::c_int)) as int16_t;
            left = (right as libc::c_int + difference) as int16_t;
            if host_bigendian != 0 {
                left = ((left as libc::c_int & 0xff as libc::c_int) << 0x8 as libc::c_int
                    | (left as libc::c_int & 0xff00 as libc::c_int)
                        >> 0x8 as libc::c_int) as int16_t;
                right = ((right as libc::c_int & 0xff as libc::c_int)
                    << 0x8 as libc::c_int
                    | (right as libc::c_int & 0xff00 as libc::c_int)
                        >> 0x8 as libc::c_int) as int16_t;
            }
            *buffer_out.offset((i * numchannels) as isize) = left;
            *buffer_out.offset((i * numchannels + 1 as libc::c_int) as isize) = right;
            i += 1;
            i;
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < numsamples {
        let mut left_0: int16_t = 0;
        let mut right_0: int16_t = 0;
        left_0 = *buffer_a.offset(i as isize) as int16_t;
        right_0 = *buffer_b.offset(i as isize) as int16_t;
        if host_bigendian != 0 {
            left_0 = ((left_0 as libc::c_int & 0xff as libc::c_int) << 0x8 as libc::c_int
                | (left_0 as libc::c_int & 0xff00 as libc::c_int) >> 0x8 as libc::c_int)
                as int16_t;
            right_0 = ((right_0 as libc::c_int & 0xff as libc::c_int)
                << 0x8 as libc::c_int
                | (right_0 as libc::c_int & 0xff00 as libc::c_int) >> 0x8 as libc::c_int)
                as int16_t;
        }
        *buffer_out.offset((i * numchannels) as isize) = left_0;
        *buffer_out.offset((i * numchannels + 1 as libc::c_int) as isize) = right_0;
        i += 1;
        i;
    }
}
unsafe extern "C" fn deinterlace_24(
    mut buffer_a: *mut int32_t,
    mut buffer_b: *mut int32_t,
    mut uncompressed_bytes: libc::c_int,
    mut uncompressed_bytes_buffer_a: *mut int32_t,
    mut uncompressed_bytes_buffer_b: *mut int32_t,
    mut buffer_out: *mut libc::c_void,
    mut numchannels: libc::c_int,
    mut numsamples: libc::c_int,
    mut interlacing_shift: uint8_t,
    mut interlacing_leftweight: uint8_t,
) {
    let mut i: libc::c_int = 0;
    if numsamples <= 0 as libc::c_int {
        return;
    }
    if interlacing_leftweight != 0 {
        i = 0 as libc::c_int;
        while i < numsamples {
            let mut difference: int32_t = 0;
            let mut midright: int32_t = 0;
            let mut left: int32_t = 0;
            let mut right: int32_t = 0;
            midright = *buffer_a.offset(i as isize);
            difference = *buffer_b.offset(i as isize);
            right = midright
                - (difference * interlacing_leftweight as libc::c_int
                    >> interlacing_shift as libc::c_int);
            left = right + difference;
            if uncompressed_bytes != 0 {
                let mut mask: uint32_t = !((0xffffffff as libc::c_uint)
                    << uncompressed_bytes * 8 as libc::c_int);
                left <<= uncompressed_bytes * 8 as libc::c_int;
                right <<= uncompressed_bytes * 8 as libc::c_int;
                left = (left as libc::c_uint
                    | *uncompressed_bytes_buffer_a.offset(i as isize) as libc::c_uint
                        & mask) as int32_t;
                right = (right as libc::c_uint
                    | *uncompressed_bytes_buffer_b.offset(i as isize) as libc::c_uint
                        & mask) as int32_t;
            }
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int) as isize,
                ) = (left & 0xff as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (left >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = (left >> 16 as libc::c_int & 0xff as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 3 as libc::c_int) as isize,
                ) = (right & 0xff as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 4 as libc::c_int) as isize,
                ) = (right >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 5 as libc::c_int) as isize,
                ) = (right >> 16 as libc::c_int & 0xff as libc::c_int) as uint8_t;
            i += 1;
            i;
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < numsamples {
        let mut left_0: int32_t = 0;
        let mut right_0: int32_t = 0;
        left_0 = *buffer_a.offset(i as isize);
        right_0 = *buffer_b.offset(i as isize);
        if uncompressed_bytes != 0 {
            let mut mask_0: uint32_t = !((0xffffffff as libc::c_uint)
                << uncompressed_bytes * 8 as libc::c_int);
            left_0 <<= uncompressed_bytes * 8 as libc::c_int;
            right_0 <<= uncompressed_bytes * 8 as libc::c_int;
            left_0 = (left_0 as libc::c_uint
                | *uncompressed_bytes_buffer_a.offset(i as isize) as libc::c_uint
                    & mask_0) as int32_t;
            right_0 = (right_0 as libc::c_uint
                | *uncompressed_bytes_buffer_b.offset(i as isize) as libc::c_uint
                    & mask_0) as int32_t;
        }
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int) as isize,
            ) = (left_0 & 0xff as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 1 as libc::c_int) as isize,
            ) = (left_0 >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 2 as libc::c_int) as isize,
            ) = (left_0 >> 16 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 3 as libc::c_int) as isize,
            ) = (right_0 & 0xff as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 4 as libc::c_int) as isize,
            ) = (right_0 >> 8 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 5 as libc::c_int) as isize,
            ) = (right_0 >> 16 as libc::c_int & 0xff as libc::c_int) as uint8_t;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn alac_decode_frame(
    mut alac: *mut alac_file,
    mut inbuffer: *mut libc::c_uchar,
    mut outbuffer: *mut libc::c_void,
    mut outputsize: *mut libc::c_int,
) {
    let mut channels: libc::c_int = 0;
    let mut outputsamples: int32_t = (*alac).setinfo_max_samples_per_frame as int32_t;
    (*alac).input_buffer = inbuffer;
    (*alac).input_buffer_bitaccumulator = 0 as libc::c_int;
    channels = readbits(alac, 3 as libc::c_int) as libc::c_int;
    *outputsize = outputsamples * (*alac).bytespersample;
    match channels {
        0 => {
            let mut hassize: libc::c_int = 0;
            let mut isnotcompressed: libc::c_int = 0;
            let mut readsamplesize: libc::c_int = 0;
            let mut uncompressed_bytes: libc::c_int = 0;
            let mut ricemodifier: libc::c_int = 0;
            readbits(alac, 4 as libc::c_int);
            readbits(alac, 12 as libc::c_int);
            hassize = readbits(alac, 1 as libc::c_int) as libc::c_int;
            uncompressed_bytes = readbits(alac, 2 as libc::c_int) as libc::c_int;
            isnotcompressed = readbits(alac, 1 as libc::c_int) as libc::c_int;
            if hassize != 0 {
                outputsamples = readbits(alac, 32 as libc::c_int) as int32_t;
                *outputsize = outputsamples * (*alac).bytespersample;
            }
            readsamplesize = (*alac).setinfo_sample_size as libc::c_int
                - uncompressed_bytes * 8 as libc::c_int;
            if isnotcompressed == 0 {
                let mut predictor_coef_table: [int16_t; 32] = [0; 32];
                let mut predictor_coef_num: libc::c_int = 0;
                let mut prediction_type: libc::c_int = 0;
                let mut prediction_quantitization: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                readbits(alac, 8 as libc::c_int);
                readbits(alac, 8 as libc::c_int);
                prediction_type = readbits(alac, 4 as libc::c_int) as libc::c_int;
                prediction_quantitization = readbits(alac, 4 as libc::c_int)
                    as libc::c_int;
                ricemodifier = readbits(alac, 3 as libc::c_int) as libc::c_int;
                predictor_coef_num = readbits(alac, 5 as libc::c_int) as libc::c_int;
                i = 0 as libc::c_int;
                while i < predictor_coef_num {
                    predictor_coef_table[i
                        as usize] = readbits(alac, 16 as libc::c_int) as int16_t;
                    i += 1;
                    i;
                }
                if uncompressed_bytes != 0 {
                    let mut i_0: libc::c_int = 0;
                    i_0 = 0 as libc::c_int;
                    while i_0 < outputsamples {
                        *((*alac).uncompressed_bytes_buffer_a)
                            .offset(
                                i_0 as isize,
                            ) = readbits(alac, uncompressed_bytes * 8 as libc::c_int)
                            as int32_t;
                        i_0 += 1;
                        i_0;
                    }
                }
                entropy_rice_decode(
                    alac,
                    (*alac).predicterror_buffer_a,
                    outputsamples,
                    readsamplesize,
                    (*alac).setinfo_rice_initialhistory as libc::c_int,
                    (*alac).setinfo_rice_kmodifier as libc::c_int,
                    ricemodifier * (*alac).setinfo_rice_historymult as libc::c_int
                        / 4 as libc::c_int,
                    ((1 as libc::c_int) << (*alac).setinfo_rice_kmodifier as libc::c_int)
                        - 1 as libc::c_int,
                );
                if prediction_type == 0 as libc::c_int {
                    predictor_decompress_fir_adapt(
                        (*alac).predicterror_buffer_a,
                        (*alac).outputsamples_buffer_a,
                        outputsamples,
                        readsamplesize,
                        predictor_coef_table.as_mut_ptr(),
                        predictor_coef_num,
                        prediction_quantitization,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"FIXME: unhandled predicition type: %i\n\0" as *const u8
                            as *const libc::c_char,
                        prediction_type,
                    );
                }
            } else {
                if (*alac).setinfo_sample_size as libc::c_int <= 16 as libc::c_int {
                    let mut i_1: libc::c_int = 0;
                    i_1 = 0 as libc::c_int;
                    while i_1 < outputsamples {
                        let mut audiobits: int32_t = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int,
                        ) as int32_t;
                        audiobits = audiobits
                            << 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int
                            >> 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int;
                        *((*alac).outputsamples_buffer_a)
                            .offset(i_1 as isize) = audiobits;
                        i_1 += 1;
                        i_1;
                    }
                } else {
                    let mut i_2: libc::c_int = 0;
                    i_2 = 0 as libc::c_int;
                    while i_2 < outputsamples {
                        let mut audiobits_0: int32_t = 0;
                        audiobits_0 = readbits(alac, 16 as libc::c_int) as int32_t;
                        audiobits_0 = audiobits_0
                            << (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int;
                        audiobits_0 = (audiobits_0 as libc::c_uint
                            | readbits(
                                alac,
                                (*alac).setinfo_sample_size as libc::c_int
                                    - 16 as libc::c_int,
                            )) as int32_t;
                        se_struct_24.set_x(audiobits_0);
                        audiobits_0 = se_struct_24.x();
                        *((*alac).outputsamples_buffer_a)
                            .offset(i_2 as isize) = audiobits_0;
                        i_2 += 1;
                        i_2;
                    }
                }
                uncompressed_bytes = 0 as libc::c_int;
            }
            match (*alac).setinfo_sample_size as libc::c_int {
                16 => {
                    let mut i_3: libc::c_int = 0;
                    i_3 = 0 as libc::c_int;
                    while i_3 < outputsamples {
                        let mut sample: int16_t = *((*alac).outputsamples_buffer_a)
                            .offset(i_3 as isize) as int16_t;
                        if host_bigendian != 0 {
                            sample = ((sample as libc::c_int & 0xff as libc::c_int)
                                << 0x8 as libc::c_int
                                | (sample as libc::c_int & 0xff00 as libc::c_int)
                                    >> 0x8 as libc::c_int) as int16_t;
                        }
                        *(outbuffer as *mut int16_t)
                            .offset((i_3 * (*alac).numchannels) as isize) = sample;
                        i_3 += 1;
                        i_3;
                    }
                }
                24 => {
                    let mut i_4: libc::c_int = 0;
                    i_4 = 0 as libc::c_int;
                    while i_4 < outputsamples {
                        let mut sample_0: int32_t = *((*alac).outputsamples_buffer_a)
                            .offset(i_4 as isize);
                        if uncompressed_bytes != 0 {
                            let mut mask: uint32_t = 0;
                            sample_0 = sample_0 << uncompressed_bytes * 8 as libc::c_int;
                            mask = !((0xffffffff as libc::c_uint)
                                << uncompressed_bytes * 8 as libc::c_int);
                            sample_0 = (sample_0 as libc::c_uint
                                | *((*alac).uncompressed_bytes_buffer_a)
                                    .offset(i_4 as isize) as libc::c_uint & mask) as int32_t;
                        }
                        *(outbuffer as *mut uint8_t)
                            .offset(
                                (i_4 * (*alac).numchannels * 3 as libc::c_int) as isize,
                            ) = (sample_0 & 0xff as libc::c_int) as uint8_t;
                        *(outbuffer as *mut uint8_t)
                            .offset(
                                (i_4 * (*alac).numchannels * 3 as libc::c_int
                                    + 1 as libc::c_int) as isize,
                            ) = (sample_0 >> 8 as libc::c_int & 0xff as libc::c_int)
                            as uint8_t;
                        *(outbuffer as *mut uint8_t)
                            .offset(
                                (i_4 * (*alac).numchannels * 3 as libc::c_int
                                    + 2 as libc::c_int) as isize,
                            ) = (sample_0 >> 16 as libc::c_int & 0xff as libc::c_int)
                            as uint8_t;
                        i_4 += 1;
                        i_4;
                    }
                }
                20 | 32 => {
                    fprintf(
                        stderr,
                        b"FIXME: unimplemented sample size %i\n\0" as *const u8
                            as *const libc::c_char,
                        (*alac).setinfo_sample_size as libc::c_int,
                    );
                }
                _ => {}
            }
        }
        1 => {
            let mut hassize_0: libc::c_int = 0;
            let mut isnotcompressed_0: libc::c_int = 0;
            let mut readsamplesize_0: libc::c_int = 0;
            let mut uncompressed_bytes_0: libc::c_int = 0;
            let mut interlacing_shift: uint8_t = 0;
            let mut interlacing_leftweight: uint8_t = 0;
            readbits(alac, 4 as libc::c_int);
            readbits(alac, 12 as libc::c_int);
            hassize_0 = readbits(alac, 1 as libc::c_int) as libc::c_int;
            uncompressed_bytes_0 = readbits(alac, 2 as libc::c_int) as libc::c_int;
            isnotcompressed_0 = readbits(alac, 1 as libc::c_int) as libc::c_int;
            if hassize_0 != 0 {
                outputsamples = readbits(alac, 32 as libc::c_int) as int32_t;
                *outputsize = outputsamples * (*alac).bytespersample;
            }
            readsamplesize_0 = (*alac).setinfo_sample_size as libc::c_int
                - uncompressed_bytes_0 * 8 as libc::c_int + 1 as libc::c_int;
            if isnotcompressed_0 == 0 {
                let mut predictor_coef_table_a: [int16_t; 32] = [0; 32];
                let mut predictor_coef_num_a: libc::c_int = 0;
                let mut prediction_type_a: libc::c_int = 0;
                let mut prediction_quantitization_a: libc::c_int = 0;
                let mut ricemodifier_a: libc::c_int = 0;
                let mut predictor_coef_table_b: [int16_t; 32] = [0; 32];
                let mut predictor_coef_num_b: libc::c_int = 0;
                let mut prediction_type_b: libc::c_int = 0;
                let mut prediction_quantitization_b: libc::c_int = 0;
                let mut ricemodifier_b: libc::c_int = 0;
                let mut i_5: libc::c_int = 0;
                interlacing_shift = readbits(alac, 8 as libc::c_int) as uint8_t;
                interlacing_leftweight = readbits(alac, 8 as libc::c_int) as uint8_t;
                prediction_type_a = readbits(alac, 4 as libc::c_int) as libc::c_int;
                prediction_quantitization_a = readbits(alac, 4 as libc::c_int)
                    as libc::c_int;
                ricemodifier_a = readbits(alac, 3 as libc::c_int) as libc::c_int;
                predictor_coef_num_a = readbits(alac, 5 as libc::c_int) as libc::c_int;
                i_5 = 0 as libc::c_int;
                while i_5 < predictor_coef_num_a {
                    predictor_coef_table_a[i_5
                        as usize] = readbits(alac, 16 as libc::c_int) as int16_t;
                    i_5 += 1;
                    i_5;
                }
                prediction_type_b = readbits(alac, 4 as libc::c_int) as libc::c_int;
                prediction_quantitization_b = readbits(alac, 4 as libc::c_int)
                    as libc::c_int;
                ricemodifier_b = readbits(alac, 3 as libc::c_int) as libc::c_int;
                predictor_coef_num_b = readbits(alac, 5 as libc::c_int) as libc::c_int;
                i_5 = 0 as libc::c_int;
                while i_5 < predictor_coef_num_b {
                    predictor_coef_table_b[i_5
                        as usize] = readbits(alac, 16 as libc::c_int) as int16_t;
                    i_5 += 1;
                    i_5;
                }
                if uncompressed_bytes_0 != 0 {
                    let mut i_6: libc::c_int = 0;
                    i_6 = 0 as libc::c_int;
                    while i_6 < outputsamples {
                        *((*alac).uncompressed_bytes_buffer_a)
                            .offset(
                                i_6 as isize,
                            ) = readbits(alac, uncompressed_bytes_0 * 8 as libc::c_int)
                            as int32_t;
                        *((*alac).uncompressed_bytes_buffer_b)
                            .offset(
                                i_6 as isize,
                            ) = readbits(alac, uncompressed_bytes_0 * 8 as libc::c_int)
                            as int32_t;
                        i_6 += 1;
                        i_6;
                    }
                }
                entropy_rice_decode(
                    alac,
                    (*alac).predicterror_buffer_a,
                    outputsamples,
                    readsamplesize_0,
                    (*alac).setinfo_rice_initialhistory as libc::c_int,
                    (*alac).setinfo_rice_kmodifier as libc::c_int,
                    ricemodifier_a * (*alac).setinfo_rice_historymult as libc::c_int
                        / 4 as libc::c_int,
                    ((1 as libc::c_int) << (*alac).setinfo_rice_kmodifier as libc::c_int)
                        - 1 as libc::c_int,
                );
                if prediction_type_a == 0 as libc::c_int {
                    predictor_decompress_fir_adapt(
                        (*alac).predicterror_buffer_a,
                        (*alac).outputsamples_buffer_a,
                        outputsamples,
                        readsamplesize_0,
                        predictor_coef_table_a.as_mut_ptr(),
                        predictor_coef_num_a,
                        prediction_quantitization_a,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"FIXME: unhandled predicition type: %i\n\0" as *const u8
                            as *const libc::c_char,
                        prediction_type_a,
                    );
                }
                entropy_rice_decode(
                    alac,
                    (*alac).predicterror_buffer_b,
                    outputsamples,
                    readsamplesize_0,
                    (*alac).setinfo_rice_initialhistory as libc::c_int,
                    (*alac).setinfo_rice_kmodifier as libc::c_int,
                    ricemodifier_b * (*alac).setinfo_rice_historymult as libc::c_int
                        / 4 as libc::c_int,
                    ((1 as libc::c_int) << (*alac).setinfo_rice_kmodifier as libc::c_int)
                        - 1 as libc::c_int,
                );
                if prediction_type_b == 0 as libc::c_int {
                    predictor_decompress_fir_adapt(
                        (*alac).predicterror_buffer_b,
                        (*alac).outputsamples_buffer_b,
                        outputsamples,
                        readsamplesize_0,
                        predictor_coef_table_b.as_mut_ptr(),
                        predictor_coef_num_b,
                        prediction_quantitization_b,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"FIXME: unhandled predicition type: %i\n\0" as *const u8
                            as *const libc::c_char,
                        prediction_type_b,
                    );
                }
            } else {
                if (*alac).setinfo_sample_size as libc::c_int <= 16 as libc::c_int {
                    let mut i_7: libc::c_int = 0;
                    i_7 = 0 as libc::c_int;
                    while i_7 < outputsamples {
                        let mut audiobits_a: int32_t = 0;
                        let mut audiobits_b: int32_t = 0;
                        audiobits_a = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int,
                        ) as int32_t;
                        audiobits_b = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int,
                        ) as int32_t;
                        audiobits_a = audiobits_a
                            << 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int
                            >> 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int;
                        audiobits_b = audiobits_b
                            << 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int
                            >> 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int;
                        *((*alac).outputsamples_buffer_a)
                            .offset(i_7 as isize) = audiobits_a;
                        *((*alac).outputsamples_buffer_b)
                            .offset(i_7 as isize) = audiobits_b;
                        i_7 += 1;
                        i_7;
                    }
                } else {
                    let mut i_8: libc::c_int = 0;
                    i_8 = 0 as libc::c_int;
                    while i_8 < outputsamples {
                        let mut audiobits_a_0: int32_t = 0;
                        let mut audiobits_b_0: int32_t = 0;
                        audiobits_a_0 = readbits(alac, 16 as libc::c_int) as int32_t;
                        audiobits_a_0 = audiobits_a_0
                            << (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int;
                        audiobits_a_0 = (audiobits_a_0 as libc::c_uint
                            | readbits(
                                alac,
                                (*alac).setinfo_sample_size as libc::c_int
                                    - 16 as libc::c_int,
                            )) as int32_t;
                        se_struct_24.set_x(audiobits_a_0);
                        audiobits_a_0 = se_struct_24.x();
                        audiobits_b_0 = readbits(alac, 16 as libc::c_int) as int32_t;
                        audiobits_b_0 = audiobits_b_0
                            << (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int;
                        audiobits_b_0 = (audiobits_b_0 as libc::c_uint
                            | readbits(
                                alac,
                                (*alac).setinfo_sample_size as libc::c_int
                                    - 16 as libc::c_int,
                            )) as int32_t;
                        se_struct_24.set_x(audiobits_b_0);
                        audiobits_b_0 = se_struct_24.x();
                        *((*alac).outputsamples_buffer_a)
                            .offset(i_8 as isize) = audiobits_a_0;
                        *((*alac).outputsamples_buffer_b)
                            .offset(i_8 as isize) = audiobits_b_0;
                        i_8 += 1;
                        i_8;
                    }
                }
                uncompressed_bytes_0 = 0 as libc::c_int;
                interlacing_shift = 0 as libc::c_int as uint8_t;
                interlacing_leftweight = 0 as libc::c_int as uint8_t;
            }
            match (*alac).setinfo_sample_size as libc::c_int {
                16 => {
                    deinterlace_16(
                        (*alac).outputsamples_buffer_a,
                        (*alac).outputsamples_buffer_b,
                        outbuffer as *mut int16_t,
                        (*alac).numchannels,
                        outputsamples,
                        interlacing_shift,
                        interlacing_leftweight,
                    );
                }
                24 => {
                    deinterlace_24(
                        (*alac).outputsamples_buffer_a,
                        (*alac).outputsamples_buffer_b,
                        uncompressed_bytes_0,
                        (*alac).uncompressed_bytes_buffer_a,
                        (*alac).uncompressed_bytes_buffer_b,
                        outbuffer as *mut int16_t as *mut libc::c_void,
                        (*alac).numchannels,
                        outputsamples,
                        interlacing_shift,
                        interlacing_leftweight,
                    );
                }
                20 | 32 => {
                    fprintf(
                        stderr,
                        b"FIXME: unimplemented sample size %i\n\0" as *const u8
                            as *const libc::c_char,
                        (*alac).setinfo_sample_size as libc::c_int,
                    );
                }
                _ => {}
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn alac_create(
    mut samplesize: libc::c_int,
    mut numchannels: libc::c_int,
) -> *mut alac_file {
    let mut newfile: *mut alac_file = malloc(
        ::std::mem::size_of::<alac_file>() as libc::c_ulong,
    ) as *mut alac_file;
    memset(
        newfile as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<alac_file>() as libc::c_ulong,
    );
    (*newfile).samplesize = samplesize;
    (*newfile).numchannels = numchannels;
    (*newfile).bytespersample = samplesize / 8 as libc::c_int * numchannels;
    return newfile;
}
