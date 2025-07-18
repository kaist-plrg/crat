use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn locale_charset() -> *const libc::c_char;
    fn iconv_open(
        __tocode: *const libc::c_char,
        __fromcode: *const libc::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
static mut conv_desc: [iconv_t; 2] = unsafe {
    [-(1 as libc::c_int) as iconv_t, -(1 as libc::c_int) as iconv_t]
};
unsafe extern "C" fn utf8_init(mut to_utf: bool) -> iconv_t {
    if conv_desc[to_utf as libc::c_int as usize] == -(1 as libc::c_int) as iconv_t {
        if to_utf {
            conv_desc[to_utf as libc::c_int
                as usize] = iconv_open(
                b"UTF-8\0" as *const u8 as *const libc::c_char,
                locale_charset(),
            );
        } else {
            conv_desc[to_utf as libc::c_int
                as usize] = iconv_open(
                locale_charset(),
                b"UTF-8\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return conv_desc[to_utf as libc::c_int as usize];
}
pub unsafe extern "C" fn utf8_convert(
    mut to_utf: bool,
    mut input: *const libc::c_char,
    mut output: *mut *mut libc::c_char,
) -> bool {
    let mut ib: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ob: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut inlen: size_t = 0;
    let mut outlen: size_t = 0;
    let mut cd: iconv_t = utf8_init(to_utf);
    if cd.is_null() {
        *output = xstrdup(input);
        return 1 as libc::c_int != 0;
    } else if cd == -(1 as libc::c_int) as iconv_t {
        return 0 as libc::c_int != 0
    }
    inlen = (strlen(input)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    outlen = inlen
        .wrapping_mul(16 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    ret = xmalloc(outlen) as *mut libc::c_char;
    ob = ret;
    ib = input as *mut libc::c_char;
    if iconv(cd, &mut ib, &mut inlen, &mut ob, &mut outlen)
        != 0 as libc::c_int as libc::c_ulong
    {
        rpl_free(ret as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    }
    *ob = 0 as libc::c_int as libc::c_char;
    *output = ret;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn string_ascii_p(mut p: *const libc::c_char) -> bool {
    while *p != 0 {
        if *p as libc::c_int & !(0x7f as libc::c_int) != 0 {
            return 0 as libc::c_int != 0;
        }
        p = p.offset(1);
        p;
    }
    return 1 as libc::c_int != 0;
}
