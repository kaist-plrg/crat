use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bio_st;
    pub type rsa_st;
    pub type bio_method_st;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fork() -> __pid_t;
    fn BIO_set_flags(b: *mut BIO, flags: libc::c_int);
    fn BIO_new(type_0: *const BIO_METHOD) -> *mut BIO;
    fn BIO_free(a: *mut BIO) -> libc::c_int;
    fn BIO_read(b: *mut BIO, data: *mut libc::c_void, dlen: libc::c_int) -> libc::c_int;
    fn BIO_write(
        b: *mut BIO,
        data: *const libc::c_void,
        dlen: libc::c_int,
    ) -> libc::c_int;
    fn BIO_ctrl(
        bp: *mut BIO,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn BIO_push(b: *mut BIO, append: *mut BIO) -> *mut BIO;
    fn BIO_free_all(a: *mut BIO);
    fn BIO_s_mem() -> *const BIO_METHOD;
    fn BIO_new_mem_buf(buf: *const libc::c_void, len: libc::c_int) -> *mut BIO;
    fn RSA_private_decrypt(
        flen: libc::c_int,
        from: *const libc::c_uchar,
        to: *mut libc::c_uchar,
        rsa: *mut RSA,
        padding: libc::c_int,
    ) -> libc::c_int;
    fn RSA_private_encrypt(
        flen: libc::c_int,
        from: *const libc::c_uchar,
        to: *mut libc::c_uchar,
        rsa: *mut RSA,
        padding: libc::c_int,
    ) -> libc::c_int;
    fn RSA_size(rsa: *const RSA) -> libc::c_int;
    fn BIO_f_base64() -> *const BIO_METHOD;
    fn PEM_read_bio_RSAPrivateKey(
        bp: *mut BIO,
        x: *mut *mut RSA,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut RSA;
    fn shairport_shutdown(retval: libc::c_int);
    fn daemon_fail(format: *const libc::c_char, arg: ::std::ffi::VaList);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type uint8_t = __uint8_t;
pub type BIO = bio_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
    pub flags: libc::c_ulong,
}
pub type BUF_MEM = buf_mem_st;
pub type RSA = rsa_st;
pub type BIO_METHOD = bio_method_st;
pub type pem_password_cb = unsafe extern "C" fn(
    *mut libc::c_char,
    libc::c_int,
    libc::c_int,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_backend {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shairport_cfg {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
pub static mut config: shairport_cfg = shairport_cfg {
    password: 0 as *const libc::c_char as *mut libc::c_char,
    apname: 0 as *const libc::c_char as *mut libc::c_char,
    hw_addr: [0; 6],
    port: 0,
    output_name: 0 as *const libc::c_char as *mut libc::c_char,
    output: 0 as *const audio_output as *mut audio_output,
    mdns_name: 0 as *const libc::c_char as *mut libc::c_char,
    mdns: 0 as *const mdns_backend as *mut mdns_backend,
    buffer_start_fill: 0,
    daemonise: 0,
    cmd_start: 0 as *const libc::c_char as *mut libc::c_char,
    cmd_stop: 0 as *const libc::c_char as *mut libc::c_char,
    cmd_blocking: 0,
    meta_dir: 0 as *const libc::c_char as *mut libc::c_char,
    pidfile: 0 as *const libc::c_char as *mut libc::c_char,
    logfile: 0 as *const libc::c_char as *mut libc::c_char,
    errfile: 0 as *const libc::c_char as *mut libc::c_char,
};
pub static mut debuglev: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn die(mut format: *mut libc::c_char, mut args: ...) {
    fprintf(stderr, b"FATAL: \0" as *const u8 as *const libc::c_char);
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    if config.daemonise != 0 {
        daemon_fail(format, args_0.as_va_list());
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    shairport_shutdown(1 as libc::c_int);
}
pub unsafe extern "C" fn warn(mut format: *mut libc::c_char, mut args: ...) {
    fprintf(stderr, b"WARNING: \0" as *const u8 as *const libc::c_char);
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn debug(
    mut level: libc::c_int,
    mut format: *mut libc::c_char,
    mut args: ...
) {
    if level > debuglev {
        return;
    }
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
}
pub unsafe extern "C" fn base64_enc(
    mut input: *mut uint8_t,
    mut length: libc::c_int,
) -> *mut libc::c_char {
    let mut bmem: *mut BIO = 0 as *mut BIO;
    let mut b64: *mut BIO = 0 as *mut BIO;
    let mut bptr: *mut BUF_MEM = 0 as *mut BUF_MEM;
    b64 = BIO_new(BIO_f_base64());
    bmem = BIO_new(BIO_s_mem());
    b64 = BIO_push(b64, bmem);
    BIO_set_flags(b64, 0x100 as libc::c_int);
    BIO_write(b64, input as *const libc::c_void, length);
    BIO_ctrl(
        b64,
        11 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_void,
    );
    BIO_ctrl(
        b64,
        115 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        &mut bptr as *mut *mut BUF_MEM as *mut libc::c_char as *mut libc::c_void,
    );
    let mut buf: *mut libc::c_char = malloc((*bptr).length) as *mut libc::c_char;
    if (*bptr).length != 0 {
        memcpy(
            buf as *mut libc::c_void,
            (*bptr).data as *const libc::c_void,
            ((*bptr).length).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        *buf
            .offset(
                ((*bptr).length).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    BIO_free_all(bmem);
    return buf;
}
pub unsafe extern "C" fn base64_dec(
    mut input: *mut libc::c_char,
    mut outlen: *mut libc::c_int,
) -> *mut uint8_t {
    let mut bmem: *mut BIO = 0 as *mut BIO;
    let mut b64: *mut BIO = 0 as *mut BIO;
    let mut inlen: libc::c_int = strlen(input) as libc::c_int;
    b64 = BIO_new(BIO_f_base64());
    BIO_set_flags(b64, 0x100 as libc::c_int);
    bmem = BIO_new(BIO_s_mem());
    b64 = BIO_push(b64, bmem);
    BIO_write(bmem, input as *const libc::c_void, inlen);
    loop {
        let fresh0 = inlen;
        inlen = inlen + 1;
        if !(fresh0 & 3 as libc::c_int != 0) {
            break;
        }
        BIO_write(
            bmem,
            b"=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
    }
    BIO_ctrl(
        bmem,
        11 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        0 as *mut libc::c_void,
    );
    let mut bufsize: libc::c_int = (strlen(input))
        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
        .wrapping_div(4 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut buf: *mut uint8_t = malloc(bufsize as libc::c_ulong) as *mut uint8_t;
    let mut nread: libc::c_int = 0;
    nread = BIO_read(b64, buf as *mut libc::c_void, bufsize);
    BIO_free_all(bmem);
    *outlen = nread;
    return buf;
}
static mut super_secret_key: [libc::c_char; 1675] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1675],
        &mut [libc::c_char; 1675],
    >(
        b"-----BEGIN RSA PRIVATE KEY-----\nMIIEpQIBAAKCAQEA59dE8qLieItsH1WgjrcFRKj6eUWqi+bGLOX1HL3U3GhC/j0Qg90u3sG/1CUt\nwC5vOYvfDmFI6oSFXi5ELabWJmT2dKHzBJKa3k9ok+8t9ucRqMd6DZHJ2YCCLlDRKSKv6kDqnw4U\nwPdpOMXziC/AMj3Z/lUVX1G7WSHCAWKf1zNS1eLvqr+boEjXuBOitnZ/bDzPHrTOZz0Dew0uowxf\n/+sG+NCK3eQJVxqcaJ/vEHKIVd2M+5qL71yJQ+87X6oV3eaYvt3zWZYD6z5vYTcrtij2VZ9Zmni/\nUAaHqn9JdsBWLUEpVviYnhimNVvYFZeCXg/IdTQ+x4IRdiXNv5hEewIDAQABAoIBAQDl8Axy9XfW\nBLmkzkEiqoSwF0PsmVrPzH9KsnwLGH+QZlvjWd8SWYGN7u1507HvhF5N3drJoVU3O14nDY4TFQAa\nLlJ9VM35AApXaLyY1ERrN7u9ALKd2LUwYhM7Km539O4yUFYikE2nIPscEsA5ltpxOgUGCY7b7ez5\nNtD6nL1ZKauw7aNXmVAvmJTcuPxWmoktF3gDJKK2wxZuNGcJE0uFQEG4Z3BrWP7yoNuSK3dii2jm\nlpPHr0O/KnPQtzI3eguhe0TwUem/eYSdyzMyVx/YpwkzwtYL3sR5k0o9rKQLtvLzfAqdBxBurciz\naaA/L0HIgAmOit1GJA2saMxTVPNhAoGBAPfgv1oeZxgxmotiCcMXFEQEWflzhWYTsXrhUIuz5jFu\na39GLS99ZEErhLdrwj8rDDViRVJ5skOp9zFvlYAHs0xh92ji1E7V/ysnKBfsMrPkk5KSKPrnjndM\noPdevWnVkgJ5jxFuNgxkOLMuG9i53B4yMvDTCRiIPMQ++N2iLDaRAoGBAO9v//mU8eVkQaoANf0Z\noMjW8CN4xwWA2cSEIHkd9AfFkftuv8oyLDCG3ZAf0vrhrrtkrfa7ef+AUb69DNggq4mHQAYBp7L+\nk5DKzJrKuO0r+R0YbY9pZD1+/g9dVt91d6LQNepUE/yY2PP5CNoFmjedpLHMOPFdVgqDzDFxU8hL\nAoGBANDrr7xAJbqBjHVwIzQ4To9pb4BNeqDndk5Qe7fT3+/H1njGaC0/rXE0Qb7q5ySgnsCb3DvA\ncJyRM9SJ7OKlGt0FMSdJD5KG0XPIpAVNwgpXXH5MDJg09KHeh0kXo+QA6viFBi21y340NonnEfdf\n54PX4ZGS/Xac1UK+pLkBB+zRAoGAf0AY3H3qKS2lMEI4bzEFoHeK3G895pDaK3TFBVmD7fV0Zhov\n17fegFPMwOII8MisYm9ZfT2Z0s5Ro3s5rkt+nvLAdfC/PYPKzTLalpGSwomSNYJcB9HNMlmhkGzc\n1JnLYT4iyUyx6pcZBmCd8bD0iwY/FzcgNDaUmbX9+XDvRA0CgYEAkE7pIPlE71qvfJQgoA9em0gI\nLAuE4Pu13aKiJnfft7hIjbK+5kyb3TysZvoyDnb3HOKvInK7vXbKuU4ISgxB2bB3HcYzQMGsz1qJ\n2gG0N5hvJpzwwhbhXqFKA4zaaSrw622wDniAK5MlIE0tIAKKP4yxNGjoD2QYjhBGuhvkWKY=\n-----END RSA PRIVATE KEY-----\0",
    )
};
pub unsafe extern "C" fn rsa_apply(
    mut input: *mut uint8_t,
    mut inlen: libc::c_int,
    mut outlen: *mut libc::c_int,
    mut mode: libc::c_int,
) -> *mut uint8_t {
    static mut rsa: *mut RSA = 0 as *const RSA as *mut RSA;
    if rsa.is_null() {
        let mut bmem: *mut BIO = BIO_new_mem_buf(
            super_secret_key.as_mut_ptr() as *const libc::c_void,
            -(1 as libc::c_int),
        );
        rsa = PEM_read_bio_RSAPrivateKey(
            bmem,
            0 as *mut *mut RSA,
            None,
            0 as *mut libc::c_void,
        );
        BIO_free(bmem);
    }
    let mut out: *mut uint8_t = malloc(RSA_size(rsa) as libc::c_ulong) as *mut uint8_t;
    match mode {
        0 => {
            *outlen = RSA_private_encrypt(inlen, input, out, rsa, 1 as libc::c_int);
        }
        1 => {
            *outlen = RSA_private_decrypt(inlen, input, out, rsa, 4 as libc::c_int);
        }
        _ => {
            die(
                b"bad rsa mode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return out;
}
pub unsafe extern "C" fn command_start() {
    if (config.cmd_start).is_null() {
        return;
    }
    if config.cmd_blocking == 0 && fork() != 0 {
        return;
    }
    debug(
        1 as libc::c_int,
        b"running start command: %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        config.cmd_start,
    );
    if system(config.cmd_start) != 0 {
        warn(
            b"exec of external start command failed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if config.cmd_blocking == 0 {
        exit(0 as libc::c_int);
    }
}
pub unsafe extern "C" fn command_stop() {
    if (config.cmd_stop).is_null() {
        return;
    }
    if config.cmd_blocking == 0 && fork() != 0 {
        return;
    }
    debug(
        1 as libc::c_int,
        b"running stop command: %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        config.cmd_stop,
    );
    if system(config.cmd_stop) != 0 {
        warn(
            b"exec of external stop command failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if config.cmd_blocking == 0 {
        exit(0 as libc::c_int);
    }
}
