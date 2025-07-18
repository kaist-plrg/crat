use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type ec_key_st;
    pub type ec_group_st;
    pub type ec_point_st;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn OPENSSL_cleanse(ptr: *mut libc::c_void, len: size_t);
    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn BN_init(_: *mut BIGNUM);
    fn BN_clear_free(a: *mut BIGNUM);
    fn BN_mod_add(
        r: *mut BIGNUM,
        a: *const BIGNUM,
        b: *const BIGNUM,
        m: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EVP_read_pw_string(
        buf: *mut libc::c_char,
        length: libc::c_int,
        prompt: *const libc::c_char,
        verify: libc::c_int,
    ) -> libc::c_int;
    fn OPENSSL_add_all_algorithms_noconf();
    fn EC_GROUP_get_order(
        group: *const EC_GROUP,
        order: *mut BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn EC_KEY_get0_group(key: *const EC_KEY) -> *const EC_GROUP;
    fn EC_KEY_get0_private_key(key: *const EC_KEY) -> *const BIGNUM;
    fn EC_KEY_get0_public_key(key: *const EC_KEY) -> *const EC_POINT;
    fn EC_KEY_generate_key(key: *mut EC_KEY) -> libc::c_int;
    fn i2o_ECPublicKey(key: *mut EC_KEY, out: *mut *mut libc::c_uchar) -> libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn vg_encode_address(
        ppoint: *const EC_POINT,
        pgroup: *const EC_GROUP,
        addrtype: libc::c_int,
        result: *mut libc::c_char,
    );
    fn dumphex(src: *const libc::c_uchar, len: size_t);
    fn dumpbn(bn: *const BIGNUM);
    fn vg_encode_privkey(
        pkey: *const EC_KEY,
        addrtype: libc::c_int,
        result: *mut libc::c_char,
    );
    fn vg_set_privkey(bnpriv: *const BIGNUM, pkey: *mut EC_KEY) -> libc::c_int;
    fn vg_protect_encode_privkey(
        out: *mut libc::c_char,
        pkey: *const EC_KEY,
        keytype: libc::c_int,
        parameter_group: libc::c_int,
        pass: *const libc::c_char,
    ) -> libc::c_int;
    fn vg_pkcs8_encode_privkey(
        out: *mut libc::c_char,
        outlen: libc::c_int,
        pkey: *const EC_KEY,
        pass: *const libc::c_char,
    ) -> libc::c_int;
    fn vg_decode_privkey_any(
        pkey: *mut EC_KEY,
        addrtype: *mut libc::c_int,
        input: *const libc::c_char,
        pass: *const libc::c_char,
    ) -> libc::c_int;
    fn vg_check_password_complexity(
        pass: *const libc::c_char,
        verbose: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type BIGNUM = bignum_st;
pub type BN_CTX = bignum_ctx;
pub type EC_GROUP = ec_group_st;
pub type EC_POINT = ec_point_st;
pub type EC_KEY = ec_key_st;
pub static mut version: *const libc::c_char = b"0.22\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn usage(mut progname: *const libc::c_char) {
    fprintf(
        stderr,
        b"Vanitygen keyconv %s\nUsage: %s [-8] [-e|-E <password>] [-c <key>] [<key>]\n-G            Generate a key pair and output the full public key\n-8            Output key in PKCS#8 form\n-e            Encrypt output key, prompt for password\n-E <password> Encrypt output key with <password> (UNSAFE)\n-c <key>      Combine private key parts to make complete private key\n-v            Verbose output\n\0"
            as *const u8 as *const libc::c_char,
        version,
        progname,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pwbuf: [libc::c_char; 128] = [0; 128];
    let mut ecprot: [libc::c_char; 128] = [0; 128];
    let mut pbuf: [libc::c_char; 1024] = [0; 1024];
    let mut key_in: *const libc::c_char = 0 as *const libc::c_char;
    let mut pass_in: *const libc::c_char = 0 as *const libc::c_char;
    let mut key2_in: *const libc::c_char = 0 as *const libc::c_char;
    let mut pkey: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut parameter_group: libc::c_int = -(1 as libc::c_int);
    let mut privtype: libc::c_int = 0;
    let mut addrtype: libc::c_int = 0;
    let mut pkcs8: libc::c_int = 0 as libc::c_int;
    let mut pass_prompt: libc::c_int = 0 as libc::c_int;
    let mut verbose: libc::c_int = 0 as libc::c_int;
    let mut generate: libc::c_int = 0 as libc::c_int;
    let mut opt: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    loop {
        opt = getopt(argc, argv, b"8E:ec:vG\0" as *const u8 as *const libc::c_char);
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            56 => {
                pkcs8 = 1 as libc::c_int;
            }
            69 => {
                if pass_prompt != 0 {
                    usage(*argv.offset(0 as libc::c_int as isize));
                    return 1 as libc::c_int;
                }
                pass_in = optarg;
                if vg_check_password_complexity(pass_in, 1 as libc::c_int) == 0 {
                    fprintf(
                        stderr,
                        b"WARNING: Using weak password\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            101 => {
                if !pass_in.is_null() {
                    usage(*argv.offset(0 as libc::c_int as isize));
                    return 1 as libc::c_int;
                }
                pass_prompt = 1 as libc::c_int;
            }
            99 => {
                key2_in = optarg;
            }
            118 => {
                verbose = 1 as libc::c_int;
            }
            71 => {
                generate = 1 as libc::c_int;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize));
                return 1 as libc::c_int;
            }
        }
    }
    OPENSSL_add_all_algorithms_noconf();
    pkey = EC_KEY_new_by_curve_name(714 as libc::c_int);
    if generate != 0 {
        let mut pend: *mut libc::c_uchar = pbuf.as_mut_ptr() as *mut libc::c_uchar;
        addrtype = 0 as libc::c_int;
        privtype = 128 as libc::c_int;
        EC_KEY_generate_key(pkey);
        res = i2o_ECPublicKey(pkey, &mut pend);
        fprintf(stderr, b"Pubkey (hex): \0" as *const u8 as *const libc::c_char);
        dumphex(pbuf.as_mut_ptr() as *mut libc::c_uchar, res as size_t);
        fprintf(stderr, b"Privkey (hex): \0" as *const u8 as *const libc::c_char);
        dumpbn(EC_KEY_get0_private_key(pkey));
        vg_encode_address(
            EC_KEY_get0_public_key(pkey),
            EC_KEY_get0_group(pkey),
            addrtype,
            ecprot.as_mut_ptr(),
        );
        printf(
            b"Address: %s\n\0" as *const u8 as *const libc::c_char,
            ecprot.as_mut_ptr(),
        );
        vg_encode_privkey(pkey, privtype, ecprot.as_mut_ptr());
        printf(
            b"Privkey: %s\n\0" as *const u8 as *const libc::c_char,
            ecprot.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    }
    if optind >= argc {
        res = fread(
            pbuf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            stdin,
        ) as libc::c_int;
        pbuf[res as usize] = '\0' as i32 as libc::c_char;
        key_in = pbuf.as_mut_ptr();
    } else {
        key_in = *argv.offset(optind as isize);
    }
    res = vg_decode_privkey_any(pkey, &mut privtype, key_in, 0 as *const libc::c_char);
    if res < 0 as libc::c_int {
        if EVP_read_pw_string(
            pwbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"Enter import password:\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        ) != 0
            || vg_decode_privkey_any(pkey, &mut privtype, key_in, pwbuf.as_mut_ptr())
                == 0
        {
            return 1 as libc::c_int;
        }
    }
    if res == 0 {
        fprintf(
            stderr,
            b"ERROR: Unrecognized key format\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if !key2_in.is_null() {
        let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
        let mut bntmp: BIGNUM = BIGNUM {
            d: 0 as *mut libc::c_ulong,
            top: 0,
            dmax: 0,
            neg: 0,
            flags: 0,
        };
        let mut bntmp2: BIGNUM = BIGNUM {
            d: 0 as *mut libc::c_ulong,
            top: 0,
            dmax: 0,
            neg: 0,
            flags: 0,
        };
        let mut pkey2: *mut EC_KEY = 0 as *mut EC_KEY;
        pkey2 = EC_KEY_new_by_curve_name(714 as libc::c_int);
        res = vg_decode_privkey_any(
            pkey2,
            &mut privtype,
            key2_in,
            0 as *const libc::c_char,
        );
        if res < 0 as libc::c_int {
            if EVP_read_pw_string(
                pwbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                    as libc::c_int,
                b"Enter import password:\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            ) != 0
                || vg_decode_privkey_any(
                    pkey2,
                    &mut privtype,
                    key2_in,
                    pwbuf.as_mut_ptr(),
                ) == 0
            {
                return 1 as libc::c_int;
            }
        }
        if res == 0 {
            fprintf(
                stderr,
                b"ERROR: Unrecognized key format\n\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        BN_init(&mut bntmp);
        BN_init(&mut bntmp2);
        bnctx = BN_CTX_new();
        EC_GROUP_get_order(EC_KEY_get0_group(pkey), &mut bntmp2, 0 as *mut BN_CTX);
        BN_mod_add(
            &mut bntmp,
            EC_KEY_get0_private_key(pkey),
            EC_KEY_get0_private_key(pkey2),
            &mut bntmp2,
            bnctx,
        );
        vg_set_privkey(&mut bntmp, pkey);
        EC_KEY_free(pkey2);
        BN_clear_free(&mut bntmp);
        BN_clear_free(&mut bntmp2);
        BN_CTX_free(bnctx);
    }
    if pass_prompt != 0 {
        res = EVP_read_pw_string(
            pwbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"Enter password:\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if res != 0 {
            return 1 as libc::c_int;
        }
        pass_in = pwbuf.as_mut_ptr();
        if vg_check_password_complexity(pwbuf.as_mut_ptr(), 1 as libc::c_int) == 0 {
            fprintf(
                stderr,
                b"WARNING: Using weak password\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    match privtype {
        128 => {
            addrtype = 0 as libc::c_int;
        }
        239 => {
            addrtype = 111 as libc::c_int;
        }
        _ => {
            addrtype = 0 as libc::c_int;
        }
    }
    if verbose != 0 {
        let mut pend_0: *mut libc::c_uchar = pbuf.as_mut_ptr() as *mut libc::c_uchar;
        res = i2o_ECPublicKey(pkey, &mut pend_0);
        fprintf(stderr, b"Pubkey (hex): \0" as *const u8 as *const libc::c_char);
        dumphex(pbuf.as_mut_ptr() as *mut libc::c_uchar, res as size_t);
        fprintf(stderr, b"Privkey (hex): \0" as *const u8 as *const libc::c_char);
        dumpbn(EC_KEY_get0_private_key(pkey));
    }
    if pkcs8 != 0 {
        res = vg_pkcs8_encode_privkey(
            pbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            pkey,
            pass_in,
        );
        if res == 0 {
            fprintf(
                stderr,
                b"ERROR: Could not encode private key\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        printf(b"%s\0" as *const u8 as *const libc::c_char, pbuf.as_mut_ptr());
    } else if !pass_in.is_null() {
        res = vg_protect_encode_privkey(
            ecprot.as_mut_ptr(),
            pkey,
            privtype,
            parameter_group,
            pass_in,
        );
        if res == 0 {
            fprintf(
                stderr,
                b"ERROR: could not password-protect private key\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        vg_encode_address(
            EC_KEY_get0_public_key(pkey),
            EC_KEY_get0_group(pkey),
            addrtype,
            pwbuf.as_mut_ptr(),
        );
        printf(
            b"Address: %s\n\0" as *const u8 as *const libc::c_char,
            pwbuf.as_mut_ptr(),
        );
        printf(
            b"Protkey: %s\n\0" as *const u8 as *const libc::c_char,
            ecprot.as_mut_ptr(),
        );
    } else {
        vg_encode_address(
            EC_KEY_get0_public_key(pkey),
            EC_KEY_get0_group(pkey),
            addrtype,
            ecprot.as_mut_ptr(),
        );
        printf(
            b"Address: %s\n\0" as *const u8 as *const libc::c_char,
            ecprot.as_mut_ptr(),
        );
        vg_encode_privkey(pkey, privtype, ecprot.as_mut_ptr());
        printf(
            b"Privkey: %s\n\0" as *const u8 as *const libc::c_char,
            ecprot.as_mut_ptr(),
        );
    }
    OPENSSL_cleanse(
        pwbuf.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    EC_KEY_free(pkey);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
