use ::libc;
extern "C" {
    pub type hash_table;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn logputs(_: log_options, _: *const libc::c_char);
    fn string_set_add(_: *mut hash_table, _: *const libc::c_char);
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> libc::c_int;
    fn hash_table_count(_: *const hash_table) -> libc::c_int;
    fn make_string_hash_table(_: libc::c_int) -> *mut hash_table;
    fn is_robots_txt_url(_: *const libc::c_char) -> bool;
}
pub type log_options = libc::c_uint;
pub const LOG_PROGRESS: log_options = 4;
pub const LOG_ALWAYS: log_options = 3;
pub const LOG_NONVERBOSE: log_options = 2;
pub const LOG_NOTQUIET: log_options = 1;
pub const LOG_VERBOSE: log_options = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
static mut nonexisting_urls_set: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
pub unsafe extern "C" fn nonexisting_url(mut url: *const libc::c_char) {
    if is_robots_txt_url(url) {
        return;
    }
    if nonexisting_urls_set.is_null() {
        nonexisting_urls_set = make_string_hash_table(0 as libc::c_int);
    }
    string_set_add(nonexisting_urls_set, url);
}
pub unsafe extern "C" fn print_broken_links() {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    let mut num_elems: libc::c_int = 0;
    if nonexisting_urls_set.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Found no broken links.\n\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    num_elems = hash_table_count(nonexisting_urls_set);
    logprintf(
        LOG_NOTQUIET,
        dcngettext(
            0 as *const libc::c_char,
            b"Found %d broken link.\n\n\0" as *const u8 as *const libc::c_char,
            b"Found %d broken links.\n\n\0" as *const u8 as *const libc::c_char,
            num_elems as libc::c_ulong,
            5 as libc::c_int,
        ),
        num_elems,
    );
    hash_table_iterate(nonexisting_urls_set, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        let mut url: *const libc::c_char = iter.key as *const libc::c_char;
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            url,
        );
    }
    logputs(LOG_NOTQUIET, b"\n\0" as *const u8 as *const libc::c_char);
}
