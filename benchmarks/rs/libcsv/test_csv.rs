use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn csv_init(p: *mut csv_parser, options: libc::c_uchar) -> libc::c_int;
    fn csv_fini(
        p: *mut csv_parser,
        cb1_0: Option::<
            unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
        >,
        cb2_0: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn csv_free(p: *mut csv_parser);
    fn csv_parse(
        p: *mut csv_parser,
        s: *const libc::c_void,
        len: size_t,
        cb1_0: Option::<
            unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
        >,
        cb2_0: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
        data: *mut libc::c_void,
    ) -> size_t;
    fn csv_write(
        dest: *mut libc::c_void,
        dest_size: size_t,
        src: *const libc::c_void,
        src_size: size_t,
    ) -> size_t;
    fn csv_write2(
        dest: *mut libc::c_void,
        dest_size: size_t,
        src: *const libc::c_void,
        src_size: size_t,
        quote: libc::c_uchar,
    ) -> size_t;
    fn csv_set_delim(p: *mut csv_parser, c: libc::c_uchar);
    fn csv_set_quote(p: *mut csv_parser, c: libc::c_uchar);
    fn csv_set_space_func(
        p: *mut csv_parser,
        f: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    );
    fn csv_set_term_func(
        p: *mut csv_parser,
        f: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    );
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
pub struct csv_parser {
    pub pstate: libc::c_int,
    pub quoted: libc::c_int,
    pub spaces: size_t,
    pub entry_buf: *mut libc::c_uchar,
    pub entry_pos: size_t,
    pub entry_size: size_t,
    pub status: libc::c_int,
    pub options: libc::c_uchar,
    pub quote_char: libc::c_uchar,
    pub delim_char: libc::c_uchar,
    pub is_space: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub is_term: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    pub blk_size: size_t,
    pub malloc_func: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub realloc_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub free_func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub event_type: libc::c_int,
    pub retval: libc::c_int,
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub static mut event_ptr: *mut event = 0 as *const event as *mut event;
pub static mut event_idx: libc::c_int = 0;
pub static mut row: size_t = 0;
pub static mut col: size_t = 0;
pub unsafe extern "C" fn fail_parser(
    mut test_name: *mut libc::c_char,
    mut message: *mut libc::c_char,
) {
    fprintf(
        stderr,
        b"Parser test %s failed on event %d: %s\n\0" as *const u8 as *const libc::c_char,
        test_name,
        event_idx,
        message,
    );
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn fail_writer(
    mut test_name: *mut libc::c_char,
    mut message: *mut libc::c_char,
) {
    fprintf(
        stderr,
        b"Writer test %s failed: %s\n\0" as *const u8 as *const libc::c_char,
        test_name,
        message,
    );
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn cb1(
    mut data: *mut libc::c_void,
    mut len: size_t,
    mut t: *mut libc::c_void,
) {
    let mut test_name: *mut libc::c_char = t as *mut libc::c_char;
    if (*event_ptr).event_type != 1 as libc::c_int {
        fail_parser(
            test_name,
            b"didn't expect a column\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*event_ptr).size != len {
        fail_parser(
            test_name,
            b"actual data length doesn't match expected data length\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if ((*event_ptr).data).is_null() || data.is_null() {
        if (*event_ptr).data != data as *mut libc::c_char {
            fail_parser(
                test_name,
                b"actual data doesn't match expected data\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    } else if memcmp((*event_ptr).data as *const libc::c_void, data, len)
        != 0 as libc::c_int
    {
        fail_parser(
            test_name,
            b"actual data doesn't match expected data\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    event_idx += 1;
    event_idx;
    event_ptr = event_ptr.offset(1);
    event_ptr;
    col = col.wrapping_add(1);
    col;
}
pub unsafe extern "C" fn cb2(mut c: libc::c_int, mut t: *mut libc::c_void) {
    let mut test_name: *mut libc::c_char = t as *mut libc::c_char;
    if (*event_ptr).event_type != 2 as libc::c_int {
        fail_parser(
            test_name,
            b"didn't expect end of row\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*event_ptr).retval != c {
        fail_parser(
            test_name,
            b"row ended with unexpected character\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    event_idx += 1;
    event_idx;
    event_ptr = event_ptr.offset(1);
    event_ptr;
    col = 1 as libc::c_int as size_t;
    row = row.wrapping_add(1);
    row;
}
pub unsafe extern "C" fn test_parser(
    mut test_name: *mut libc::c_char,
    mut options: libc::c_uchar,
    mut input: *mut libc::c_void,
    mut len: size_t,
    mut expected: *mut event,
    mut delimiter: libc::c_char,
    mut quote: libc::c_char,
    mut space_func: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
    mut term_func: Option::<unsafe extern "C" fn(libc::c_uchar) -> libc::c_int>,
) {
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut retval: size_t = 0;
    let mut p: csv_parser = csv_parser {
        pstate: 0,
        quoted: 0,
        spaces: 0,
        entry_buf: 0 as *mut libc::c_uchar,
        entry_pos: 0,
        entry_size: 0,
        status: 0,
        options: 0,
        quote_char: 0,
        delim_char: 0,
        is_space: None,
        is_term: None,
        blk_size: 0,
        malloc_func: None,
        realloc_func: None,
        free_func: None,
    };
    let mut size: size_t = 0;
    size = 1 as libc::c_int as size_t;
    while size <= len {
        let mut bytes_processed: size_t = 0 as libc::c_int as size_t;
        csv_init(&mut p, options);
        csv_set_delim(&mut p, delimiter as libc::c_uchar);
        csv_set_quote(&mut p, quote as libc::c_uchar);
        csv_set_space_func(&mut p, space_func);
        csv_set_term_func(&mut p, term_func);
        col = 1 as libc::c_int as size_t;
        row = col;
        event_ptr = &mut *expected.offset(0 as libc::c_int as isize) as *mut event;
        event_idx = 1 as libc::c_int;
        loop {
            let mut bytes: size_t = if size < len.wrapping_sub(bytes_processed) {
                size
            } else {
                len.wrapping_sub(bytes_processed)
            };
            retval = csv_parse(
                &mut p,
                input.offset(bytes_processed as isize),
                bytes,
                Some(
                    cb1
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            size_t,
                            *mut libc::c_void,
                        ) -> (),
                ),
                Some(cb2 as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()),
                test_name as *mut libc::c_void,
            );
            if retval != bytes {
                if (*event_ptr).event_type != 3 as libc::c_int {
                    fail_parser(
                        test_name,
                        b"unexpected parse error occurred\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    csv_free(&mut p);
                    return;
                }
            }
            bytes_processed = (bytes_processed as libc::c_ulong).wrapping_add(bytes)
                as size_t as size_t;
            if !(bytes_processed < len) {
                break;
            }
        }
        result = csv_fini(
            &mut p,
            Some(
                cb1
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> (),
            ),
            Some(cb2 as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()),
            test_name as *mut libc::c_void,
        );
        if result != 0 as libc::c_int {
            if (*event_ptr).event_type != 3 as libc::c_int {
                fail_parser(
                    test_name,
                    b"unexpected parse error occurred\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                csv_free(&mut p);
                return;
            }
        }
        csv_free(&mut p);
        if (*event_ptr).event_type != 0 as libc::c_int {
            fail_parser(
                test_name,
                b"unexpected end of input\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        size = size.wrapping_add(1);
        size;
    }
}
pub unsafe extern "C" fn test_writer(
    mut test_name: *mut libc::c_char,
    mut input: *mut libc::c_char,
    mut input_len: size_t,
    mut expected: *mut libc::c_char,
    mut expected_len: size_t,
) {
    let mut actual_len: size_t = 0;
    let mut temp: *mut libc::c_char = malloc(
        input_len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if temp.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory in test_writer!\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    actual_len = csv_write(
        temp as *mut libc::c_void,
        input_len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
        input as *const libc::c_void,
        input_len,
    );
    if actual_len != expected_len {
        fail_writer(
            test_name,
            b"actual length doesn't match expected length\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if memcmp(temp as *const libc::c_void, expected as *const libc::c_void, actual_len)
        != 0 as libc::c_int
    {
        fail_writer(
            test_name,
            b"actual data doesn't match expected data\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
}
pub unsafe extern "C" fn test_writer2(
    mut test_name: *mut libc::c_char,
    mut input: *mut libc::c_char,
    mut input_len: size_t,
    mut expected: *mut libc::c_char,
    mut expected_len: size_t,
    mut quote: libc::c_char,
) {
    let mut actual_len: size_t = 0;
    let mut temp: *mut libc::c_char = malloc(
        input_len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if temp.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory in test_writer!\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    actual_len = csv_write2(
        temp as *mut libc::c_void,
        input_len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
        input as *const libc::c_void,
        input_len,
        quote as libc::c_uchar,
    );
    if actual_len != expected_len {
        fail_writer(
            test_name,
            b"actual length doesn't match expected length\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if memcmp(temp as *const libc::c_void, expected as *const libc::c_void, actual_len)
        != 0 as libc::c_int
    {
        fail_writer(
            test_name,
            b"actual data doesn't match expected data\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut test01_data: [libc::c_char; 25] = *::std::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_char; 25],
    >(b" 1,2 ,  3         ,4,5\r\n\0");
    let mut test02_data: [libc::c_char; 7] = *::std::mem::transmute::<
        &[u8; 7],
        &mut [libc::c_char; 7],
    >(b",,,,,\n\0");
    let mut test03_data: [libc::c_char; 11] = *::std::mem::transmute::<
        &[u8; 11],
        &mut [libc::c_char; 11],
    >(b"\",\",\",\",\"\"\0");
    let mut test04_data: [libc::c_char; 150] = *::std::mem::transmute::<
        &[u8; 150],
        &mut [libc::c_char; 150],
    >(
        b"\"I call our world Flatland,\nnot because we call it so,\nbut to make its nature clearer\nto you, my happy readers,\nwho are privileged to live in Space.\"\0",
    );
    let mut test05_data: [libc::c_char; 43] = *::std::mem::transmute::<
        &[u8; 43],
        &mut [libc::c_char; 43],
    >(b"\"\"\"a,b\"\"\",,\" \"\"\"\" \",\"\"\"\"\" \",\" \"\"\"\"\",\"\"\"\"\"\"\0");
    let mut test06_data: [libc::c_char; 21] = *::std::mem::transmute::<
        &[u8; 21],
        &mut [libc::c_char; 21],
    >(b"\" a, b ,c \", a b  c,\0");
    let mut test07_data: [libc::c_char; 14] = *::std::mem::transmute::<
        &[u8; 14],
        &mut [libc::c_char; 14],
    >(b"\" \"\" \" \" \"\" \"\0");
    let mut test07b_data: [libc::c_char; 14] = *::std::mem::transmute::<
        &[u8; 14],
        &mut [libc::c_char; 14],
    >(b"\" \"\" \" \" \"\" \"\0");
    let mut test08_data: [libc::c_char; 473] = *::std::mem::transmute::<
        &[u8; 473],
        &mut [libc::c_char; 473],
    >(
        b"\" abc\"                                                                                                                                                                                                                                                                                                                                                                                                                                                                          \", \"123\"\0",
    );
    let mut test09_data: [libc::c_char; 1] = *::std::mem::transmute::<
        &[u8; 1],
        &mut [libc::c_char; 1],
    >(b"\0");
    let mut test10_data: [libc::c_char; 3] = *::std::mem::transmute::<
        &[u8; 3],
        &mut [libc::c_char; 3],
    >(b"a\n\0");
    let mut test11_data: [libc::c_char; 10] = *::std::mem::transmute::<
        &[u8; 10],
        &mut [libc::c_char; 10],
    >(b"1,2 ,3,4\n\0");
    let mut test12_data: [libc::c_char; 5] = *::std::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"\n\n\n\n\0");
    let mut test12b_data: [libc::c_char; 5] = *::std::mem::transmute::<
        &[u8; 5],
        &mut [libc::c_char; 5],
    >(b"\n\n\n\n\0");
    let mut test13_data: [libc::c_char; 6] = *::std::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"\"abc\"\0");
    let mut test14_data: [libc::c_char; 21] = *::std::mem::transmute::<
        &[u8; 21],
        &mut [libc::c_char; 21],
    >(b"1, 2, 3,\n\r\n  \"4\", \r,\0");
    let mut test15_data: [libc::c_char; 22] = *::std::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"1, 2, 3,\n\r\n  \"4\", \r\"\"\0");
    let mut test16_data: [libc::c_char; 13] = *::std::mem::transmute::<
        &[u8; 13],
        &mut [libc::c_char; 13],
    >(b"\"1\",\"2\",\" 3 \0");
    let mut test16b_data: [libc::c_char; 13] = *::std::mem::transmute::<
        &[u8; 13],
        &mut [libc::c_char; 13],
    >(b"\"1\",\"2\",\" 3 \0");
    let mut test17_data: [libc::c_char; 8] = *::std::mem::transmute::<
        &[u8; 8],
        &mut [libc::c_char; 8],
    >(b" a\0b\0c \0");
    let mut test18_data: [libc::c_char; 33] = *::std::mem::transmute::<
        &[u8; 33],
        &mut [libc::c_char; 33],
    >(b"12345678901234567890123456789012\0");
    let mut test19_data: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"  , \"\" ,\0");
    let mut custom01_data: [libc::c_char; 43] = *::std::mem::transmute::<
        &[u8; 43],
        &mut [libc::c_char; 43],
    >(b"'''a;b''';;' '''' ';''''' ';' ''''';''''''\0");
    let mut test01_results: [event; 7] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\r' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test02_results: [event; 8] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test03_results: [event; 5] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test04_results: [event; 3] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 147 as libc::c_int as size_t,
                data: b"I call our world Flatland,\nnot because we call it so,\nbut to make its nature clearer\nto you, my happy readers,\nwho are privileged to live in Space.\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test05_results: [event; 8] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 5 as libc::c_int as size_t,
                data: b"\"a,b\"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 4 as libc::c_int as size_t,
                data: b" \"\" \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 3 as libc::c_int as size_t,
                data: b"\"\" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 3 as libc::c_int as size_t,
                data: b" \"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 2 as libc::c_int as size_t,
                data: b"\"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test06_results: [event; 5] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 9 as libc::c_int as size_t,
                data: b" a, b ,c \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 6 as libc::c_int as size_t,
                data: b"a b  c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test07_results: [event; 3] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 9 as libc::c_int as size_t,
                data: b" \" \" \" \" \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test07b_results: [event; 1] = [
        {
            let mut init = event {
                event_type: 3 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test08_results: [event; 4] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 463 as libc::c_int as size_t,
                data: b" abc\"                                                                                                                                                                                                                                                                                                                                                                                                                                                                          \0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 3 as libc::c_int as size_t,
                data: b"123\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test09_results: [event; 1] = [
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test10_results: [event; 3] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test11_results: [event; 6] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test12_results: [event; 1] = [
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test12b_results: [event; 5] = [
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test13_results: [event; 3] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 3 as libc::c_int as size_t,
                data: b"abc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test14_results: [event; 12] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\r' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test15_results: [event; 11] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\n' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: '\r' as i32,
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test16_results: [event; 5] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 3 as libc::c_int as size_t,
                data: b" 3 \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test16b_results: [event; 3] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 1 as libc::c_int as size_t,
                data: b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 3 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test17_results: [event; 3] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 5 as libc::c_int as size_t,
                data: b"a\0b\0c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut test19_results: [event; 5] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    let mut custom01_results: [event; 8] = [
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 5 as libc::c_int as size_t,
                data: b"'a;b'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 4 as libc::c_int as size_t,
                data: b" '' \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 3 as libc::c_int as size_t,
                data: b"'' \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 3 as libc::c_int as size_t,
                data: b" ''\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 1 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 2 as libc::c_int as size_t,
                data: b"''\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 2 as libc::c_int,
                retval: -(1 as libc::c_int),
                size: 1 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = event {
                event_type: 0 as libc::c_int,
                retval: 0 as libc::c_int,
                size: 0 as libc::c_int as size_t,
                data: 0 as *mut libc::c_char,
            };
            init
        },
    ];
    test_parser(
        b"test01\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test01_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test01_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test01\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test01_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test01_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test01\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (1 as libc::c_int | 16 as libc::c_int) as libc::c_uchar,
        test01_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test01_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test02\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test02_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test02_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test02\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test02_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test02_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test03_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test03_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test03\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test03_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test03_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test04\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test04_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test04_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test04\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test04_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test04_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test05\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test05_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test05_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test05\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test05_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test05_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test05\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (1 as libc::c_int | 4 as libc::c_int) as libc::c_uchar,
        test05_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test05_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test06\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test06_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test06_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test06\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test06_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test06_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test07\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test07_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test07_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test07b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test07b_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test07b_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test08\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test08_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 473]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test08_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test09\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test09_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test09_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test09\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int as libc::c_uchar,
        test09_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test09_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test10_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test10_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test11_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test11_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int as libc::c_uchar,
        test11_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test11_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test12_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test12_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int as libc::c_uchar,
        test12_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test12_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test12b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int as libc::c_uchar,
        test12b_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test12b_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test12b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (2 as libc::c_int | 16 as libc::c_int) as libc::c_uchar,
        test12b_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test12b_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test13_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test13_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test14_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test14_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test14_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test14_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test15_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test15_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test15_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test15_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test16_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test16_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test16_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test16_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test16b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (1 as libc::c_int | 4 as libc::c_int) as libc::c_uchar,
        test16b_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test16b_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test16_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test16_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test16_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test16_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test17\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        test17_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test17_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test17\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int as libc::c_uchar,
        test17_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test17_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test17\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (1 as libc::c_int | 16 as libc::c_int) as libc::c_uchar,
        test17_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test17_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"test19\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int as libc::c_uchar,
        test19_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        test19_results.as_mut_ptr(),
        0x2c as libc::c_int as libc::c_char,
        0x22 as libc::c_int as libc::c_char,
        None,
        None,
    );
    test_parser(
        b"custom01\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_uchar,
        custom01_data.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 43]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        custom01_results.as_mut_ptr(),
        ';' as i32 as libc::c_char,
        '\'' as i32 as libc::c_char,
        None,
        None,
    );
    test_writer(
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"abc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as size_t,
        b"\"abc\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as size_t,
    );
    test_writer(
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\"\"\"\"\"\"\"\"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as size_t,
        b"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        18 as libc::c_int as size_t,
    );
    test_writer2(
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"abc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as size_t,
        b"'abc'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 as libc::c_int as size_t,
        '\'' as i32 as libc::c_char,
    );
    test_writer2(
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"''''''''\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int as size_t,
        b"''''''''''''''''''\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        18 as libc::c_int as size_t,
        '\'' as i32 as libc::c_char,
    );
    puts(b"All tests passed\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
