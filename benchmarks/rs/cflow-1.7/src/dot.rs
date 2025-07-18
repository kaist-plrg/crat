use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type table_entry;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn include_symbol(sym: *mut Symbol) -> libc::c_int;
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
unsafe extern "C" fn dot_begin(mut fp: *mut FILE) {
    fprintf(fp, b"digraph cflow {\n\0" as *const u8 as *const libc::c_char);
    fprintf(fp, b"    node [shape=\"box\"]\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn declare_node(mut fp: *mut FILE, mut sym: *mut Symbol) {
    fprintf(fp, b"    %s [label=\"\0" as *const u8 as *const libc::c_char, (*sym).name);
    if !((*sym).decl).is_null() {
        fprintf(
            fp,
            b"%s\n%s:%d\0" as *const u8 as *const libc::c_char,
            (*sym).decl,
            (*sym).source,
            (*sym).def_line,
        );
    } else {
        fprintf(fp, b"%s()\0" as *const u8 as *const libc::c_char, (*sym).name);
    }
    fprintf(fp, b"\"]\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dot_print_symbol(
    mut fp: *mut FILE,
    mut line: libc::c_int,
    mut s: *mut output_symbol,
) {
    let mut lp: *mut linked_list = 0 as *mut linked_list;
    let mut ep: *mut linked_list_entry = 0 as *mut linked_list_entry;
    if (*(*s).sym).active != 0 {
        return;
    }
    if (*(*s).sym).expand_line != 0 {
        return
    } else {
        declare_node(fp, (*s).sym);
        (*(*s).sym).expand_line = line;
    }
    lp = if (*s).direct != 0 { (*(*s).sym).callee } else { (*(*s).sym).caller };
    ep = if !lp.is_null() { (*lp).head } else { 0 as *mut linked_list_entry };
    while !ep.is_null() {
        let mut sym: *mut Symbol = (*ep).data as *mut Symbol;
        if include_symbol(sym) != 0 {
            fprintf(
                fp,
                b"    %s -> %s\n\0" as *const u8 as *const libc::c_char,
                (*(*s).sym).name,
                (*sym).name,
            );
        }
        ep = (*ep).next;
    }
}
pub unsafe extern "C" fn dot_output_handler(
    mut cmd: cflow_output_command,
    mut outfile: *mut FILE,
    mut line: libc::c_int,
    mut data: *mut libc::c_void,
    mut handler_data: *mut libc::c_void,
) -> libc::c_int {
    match cmd as libc::c_uint {
        1 => {
            dot_begin(outfile);
        }
        2 => {
            fprintf(outfile, b"}\n\0" as *const u8 as *const libc::c_char);
        }
        5 => {
            dot_print_symbol(outfile, line, data as *mut output_symbol);
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
