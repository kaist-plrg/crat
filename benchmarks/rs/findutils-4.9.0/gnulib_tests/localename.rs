use ::libc;
extern "C" {
    pub type __locale_data;
    fn rpl_free(ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn setlocale_null(category: libc::c_int) -> *const libc::c_char;
    fn uselocale(__dataset: locale_t) -> locale_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
}
pub type nl_item = libc::c_int;
pub type locale_t = __locale_t;
pub type __locale_t = *mut __locale_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const libc::c_int,
    pub __ctype_toupper: *const libc::c_int,
    pub __names: [*const libc::c_char; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struniq_hash_node {
    pub next: *mut struniq_hash_node,
    pub contents: [libc::c_char; 0],
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn string_hash(mut x: *const libc::c_void) -> size_t {
    let mut s: *const libc::c_char = x as *const libc::c_char;
    let mut h: size_t = 0 as libc::c_int as size_t;
    while *s != 0 {
        h = (*s as libc::c_ulong)
            .wrapping_add(
                h << 9 as libc::c_int
                    | h
                        >> (::std::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(9 as libc::c_int as libc::c_ulong),
            );
        s = s.offset(1);
        s;
    }
    return h;
}
static mut struniq_hash_table: [*mut struniq_hash_node; 257] = [0
    as *const struniq_hash_node as *mut struniq_hash_node; 257];
unsafe extern "C" fn struniq(mut string: *const libc::c_char) -> *const libc::c_char {
    let mut hashcode: size_t = string_hash(string as *const libc::c_void);
    let mut slot: size_t = hashcode.wrapping_rem(257 as libc::c_int as libc::c_ulong);
    let mut size: size_t = 0;
    let mut new_node: *mut struniq_hash_node = 0 as *mut struniq_hash_node;
    let mut p: *mut struniq_hash_node = 0 as *mut struniq_hash_node;
    p = struniq_hash_table[slot as usize];
    while !p.is_null() {
        if strcmp(((*p).contents).as_mut_ptr(), string) == 0 as libc::c_int {
            return ((*p).contents).as_mut_ptr();
        }
        p = (*p).next;
    }
    size = (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    new_node = malloc(
        (8 as libc::c_ulong)
            .wrapping_add(::std::mem::align_of::<struniq_hash_node>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(size)
            & !(::std::mem::align_of::<struniq_hash_node>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut struniq_hash_node;
    if new_node.is_null() {
        return b"C\0" as *const u8 as *const libc::c_char;
    }
    memcpy(
        ((*new_node).contents).as_mut_ptr() as *mut libc::c_void,
        string as *const libc::c_void,
        size,
    );
    let mut current_block_23: u64;
    let mut mt: bool = 1 as libc::c_int != 0;
    mt;
    p = struniq_hash_table[slot as usize];
    loop {
        if p.is_null() {
            current_block_23 = 5689001924483802034;
            break;
        }
        if strcmp(((*p).contents).as_mut_ptr(), string) == 0 as libc::c_int {
            rpl_free(new_node as *mut libc::c_void);
            new_node = p;
            current_block_23 = 3960125162310008031;
            break;
        } else {
            p = (*p).next;
        }
    }
    match current_block_23 {
        5689001924483802034 => {
            ::std::ptr::write_volatile(
                &mut (*new_node).next as *mut *mut struniq_hash_node,
                struniq_hash_table[slot as usize],
            );
            ::std::ptr::write_volatile(
                &mut struniq_hash_table[slot as usize] as *mut *mut struniq_hash_node,
                new_node,
            );
        }
        _ => {}
    }
    mt;
    return ((*new_node).contents).as_mut_ptr();
}
unsafe extern "C" fn gl_locale_name_thread_unsafe(
    mut category: libc::c_int,
    mut categoryname: *const libc::c_char,
) -> *const libc::c_char {
    let mut thread_locale: locale_t = uselocale(0 as locale_t);
    if thread_locale != -(1 as libc::c_long) as locale_t {
        let mut name: *const libc::c_char = nl_langinfo(
            category << 16 as libc::c_int | -(1 as libc::c_int) & 0xffff as libc::c_int,
        );
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            name = (*thread_locale).__names[category as usize];
        }
        return name;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn gl_locale_name_thread(
    mut category: libc::c_int,
    mut categoryname: *const libc::c_char,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = gl_locale_name_thread_unsafe(
        category,
        categoryname,
    );
    if !name.is_null() {
        return struniq(name);
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn gl_locale_name_posix(
    mut category: libc::c_int,
    mut categoryname: *const libc::c_char,
) -> *const libc::c_char {
    let mut locname: *const libc::c_char = 0 as *const libc::c_char;
    locname = setlocale_null(category);
    return locname;
}
pub unsafe extern "C" fn gl_locale_name_environ(
    mut category: libc::c_int,
    mut categoryname: *const libc::c_char,
) -> *const libc::c_char {
    let mut retval: *const libc::c_char = 0 as *const libc::c_char;
    retval = getenv(b"LC_ALL\0" as *const u8 as *const libc::c_char);
    if !retval.is_null()
        && *retval.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return retval;
    }
    retval = getenv(categoryname);
    if !retval.is_null()
        && *retval.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return retval;
    }
    retval = getenv(b"LANG\0" as *const u8 as *const libc::c_char);
    if !retval.is_null()
        && *retval.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return retval;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn gl_locale_name_default() -> *const libc::c_char {
    return b"C\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn gl_locale_name(
    mut category: libc::c_int,
    mut categoryname: *const libc::c_char,
) -> *const libc::c_char {
    let mut retval: *const libc::c_char = 0 as *const libc::c_char;
    retval = gl_locale_name_thread(category, categoryname);
    if !retval.is_null() {
        return retval;
    }
    retval = gl_locale_name_posix(category, categoryname);
    if !retval.is_null() {
        return retval;
    }
    return gl_locale_name_default();
}
