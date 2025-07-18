use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type table_entry;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut print_line_numbers: libc::c_int;
    static mut brief_listing: libc::c_int;
    static mut emacs_option: libc::c_int;
    static mut omit_symbol_names_option: libc::c_int;
    fn print_level(lev: libc::c_int, last: libc::c_int);
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
pub struct linked_list_entry {
    pub next: *mut linked_list_entry,
    pub prev: *mut linked_list_entry,
    pub list: *mut linked_list,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list {
    pub free_data: linked_list_free_data_fp,
    pub head: *mut linked_list_entry,
    pub tail: *mut linked_list_entry,
}
pub type linked_list_free_data_fp = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type symtype = libc::c_uint;
pub const SymIdentifier: symtype = 2;
pub const SymToken: symtype = 1;
pub const SymUndefined: symtype = 0;
pub type storage = libc::c_uint;
pub const AnyStorage: storage = 4;
pub const AutoStorage: storage = 3;
pub const StaticStorage: storage = 2;
pub const ExplicitExternStorage: storage = 1;
pub const ExternStorage: storage = 0;
pub type symbol_flag = libc::c_uint;
pub const symbol_alias: symbol_flag = 5;
pub const symbol_parm: symbol_flag = 4;
pub const symbol_local: symbol_flag = 3;
pub const symbol_target: symbol_flag = 2;
pub const symbol_start: symbol_flag = 1;
pub const symbol_none: symbol_flag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub owner: *mut table_entry,
    pub next: *mut Symbol,
    pub entry: *mut linked_list_entry,
    pub type_0: symtype,
    pub name: *mut libc::c_char,
    pub flag: symbol_flag,
    pub alias: *mut symbol,
    pub active: libc::c_int,
    pub expand_line: libc::c_int,
    pub visible: libc::c_int,
    pub token_type: libc::c_int,
    pub source: *mut libc::c_char,
    pub def_line: libc::c_int,
    pub ref_line: *mut linked_list,
    pub level: libc::c_int,
    pub decl: *mut libc::c_char,
    pub storage: storage,
    pub arity: libc::c_int,
    pub recursive: libc::c_int,
    pub ord: size_t,
    pub caller: *mut linked_list,
    pub callee: *mut linked_list,
}
pub type Symbol = symbol;
pub type cflow_output_command = libc::c_uint;
pub const cflow_output_text: cflow_output_command = 6;
pub const cflow_output_symbol: cflow_output_command = 5;
pub const cflow_output_separator: cflow_output_command = 4;
pub const cflow_output_newline: cflow_output_command = 3;
pub const cflow_output_end: cflow_output_command = 2;
pub const cflow_output_begin: cflow_output_command = 1;
pub const cflow_output_init: cflow_output_command = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_symbol {
    pub direct: libc::c_int,
    pub level: libc::c_int,
    pub last: libc::c_int,
    pub sym: *mut Symbol,
}
unsafe extern "C" fn print_symbol_type(mut outfile: *mut FILE, mut sym: *mut Symbol) {
    if !((*sym).decl).is_null() {
        fprintf(
            outfile,
            b"%s, <%s %d>\0" as *const u8 as *const libc::c_char,
            (*sym).decl,
            (*sym).source,
            (*sym).def_line,
        );
    } else {
        fprintf(outfile, b"<>\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn print_symbol(
    mut outfile: *mut FILE,
    mut line: libc::c_int,
    mut s: *mut output_symbol,
) -> libc::c_int {
    print_level((*s).level, (*s).last);
    fprintf(outfile, b"%s: \0" as *const u8 as *const libc::c_char, (*(*s).sym).name);
    if brief_listing != 0 {
        if (*(*s).sym).expand_line != 0 {
            fprintf(
                outfile,
                b"%d\0" as *const u8 as *const libc::c_char,
                (*(*s).sym).expand_line,
            );
            return 1 as libc::c_int;
        } else if !((*(*s).sym).callee).is_null() {
            (*(*s).sym).expand_line = line;
        }
    }
    print_symbol_type(outfile, (*s).sym);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn posix_output_handler(
    mut cmd: cflow_output_command,
    mut outfile: *mut FILE,
    mut line: libc::c_int,
    mut data: *mut libc::c_void,
    mut handler_data: *mut libc::c_void,
) -> libc::c_int {
    match cmd as libc::c_uint {
        0 => {
            if emacs_option != 0 {
                error(
                    3 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--format=posix is not compatible with --emacs\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            omit_symbol_names_option = 1 as libc::c_int;
            print_line_numbers = omit_symbol_names_option;
            brief_listing = print_line_numbers;
        }
        3 => {
            fprintf(outfile, b"\n\0" as *const u8 as *const libc::c_char);
        }
        6 => {
            fprintf(
                outfile,
                b"%s\0" as *const u8 as *const libc::c_char,
                data as *mut libc::c_char,
            );
        }
        5 => return print_symbol(outfile, line, data as *mut output_symbol),
        1 | 2 | 4 | _ => {}
    }
    return 0 as libc::c_int;
}
