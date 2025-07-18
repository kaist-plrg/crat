use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type table_entry;
    pub type cflow_depmap;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn depmap_alloc(count: size_t) -> cflow_depmap_t;
    fn depmap_set(dmap: cflow_depmap_t, row: size_t, col: size_t);
    fn depmap_tc(dmap: cflow_depmap_t);
    fn depmap_isset(dmap: cflow_depmap_t, row: size_t, col: size_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn include_symbol(sym: *mut Symbol) -> libc::c_int;
    static mut outname: *mut libc::c_char;
    static mut print_option: libc::c_int;
    static mut level_indent: [*mut libc::c_char; 0];
    static mut level_end: [*mut libc::c_char; 0];
    static mut level_begin: *mut libc::c_char;
    static mut print_levels: libc::c_int;
    static mut print_line_numbers: libc::c_int;
    static mut reverse_tree: libc::c_int;
    static mut all_functions: libc::c_int;
    static mut max_depth: libc::c_int;
    fn first_starter(itr: *mut libc::c_void) -> *mut Symbol;
    fn next_starter(itr: *mut libc::c_void) -> *mut Symbol;
    fn eliminate_non_targets();
    fn collect_symbols(
        _: *mut *mut *mut Symbol,
        sel: Option::<unsafe extern "C" fn(*mut Symbol) -> libc::c_int>,
        rescnt: size_t,
    ) -> size_t;
    fn collect_functions(return_sym: *mut *mut *mut Symbol) -> size_t;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub line: libc::c_int,
    pub source: *mut libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_driver {
    pub name: *mut libc::c_char,
    pub handler: Option::<
        unsafe extern "C" fn(
            cflow_output_command,
            *mut FILE,
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub handler_data: *mut libc::c_void,
}
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
pub type cflow_depmap_t = *mut cflow_depmap;
pub static mut level_mark: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
pub static mut level_mark_size: libc::c_int = 0 as libc::c_int;
pub static mut level_mark_incr: libc::c_int = 128 as libc::c_int;
pub static mut out_line: libc::c_int = 1 as libc::c_int;
pub static mut outfile: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn set_level_mark(mut lev: libc::c_int, mut mark: libc::c_int) {
    if lev >= level_mark_size {
        level_mark_size += level_mark_incr;
        level_mark = xrealloc(level_mark as *mut libc::c_void, level_mark_size as size_t)
            as *mut libc::c_uchar;
    }
    *level_mark.offset(lev as isize) = mark as libc::c_uchar;
}
pub unsafe extern "C" fn print_level(mut lev: libc::c_int, mut last: libc::c_int) {
    let mut i: libc::c_int = 0;
    if print_line_numbers != 0 {
        fprintf(outfile, b"%5d \0" as *const u8 as *const libc::c_char, out_line);
    }
    if print_levels != 0 {
        fprintf(outfile, b"{%4d} \0" as *const u8 as *const libc::c_char, lev);
    }
    fprintf(outfile, b"%s\0" as *const u8 as *const libc::c_char, level_begin);
    i = 0 as libc::c_int;
    while i < lev {
        fprintf(
            outfile,
            b"%s\0" as *const u8 as *const libc::c_char,
            *level_indent.as_mut_ptr().offset(*level_mark.offset(i as isize) as isize),
        );
        i += 1;
        i;
    }
    fprintf(
        outfile,
        b"%s\0" as *const u8 as *const libc::c_char,
        *level_end.as_mut_ptr().offset(last as isize),
    );
}
static mut driver_index: libc::c_int = 0;
static mut driver_max: libc::c_int = 0 as libc::c_int;
pub static mut output_driver: [output_driver; 8] = [output_driver {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    handler: None,
    handler_data: 0 as *const libc::c_void as *mut libc::c_void,
}; 8];
pub unsafe extern "C" fn register_output(
    mut name: *const libc::c_char,
    mut handler: Option::<
        unsafe extern "C" fn(
            cflow_output_command,
            *mut FILE,
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut handler_data: *mut libc::c_void,
) -> libc::c_int {
    if driver_max == 8 as libc::c_int - 1 as libc::c_int {
        abort();
    }
    output_driver[driver_max as usize].name = strdup(name);
    output_driver[driver_max as usize].handler = handler;
    output_driver[driver_max as usize].handler_data = handler_data;
    let fresh0 = driver_max;
    driver_max = driver_max + 1;
    return fresh0;
}
pub unsafe extern "C" fn select_output_driver(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < driver_max {
        if strcmp(output_driver[i as usize].name, name) == 0 as libc::c_int {
            driver_index = i;
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn output_init() {
    (output_driver[driver_index as usize].handler)
        .unwrap()(
        cflow_output_init,
        0 as *mut FILE,
        0 as libc::c_int,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
pub unsafe extern "C" fn newline() {
    (output_driver[driver_index as usize].handler)
        .unwrap()(
        cflow_output_newline,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
    out_line += 1;
    out_line;
}
unsafe extern "C" fn begin() {
    (output_driver[driver_index as usize].handler)
        .unwrap()(
        cflow_output_begin,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn end() {
    (output_driver[driver_index as usize].handler)
        .unwrap()(
        cflow_output_end,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn separator() {
    (output_driver[driver_index as usize].handler)
        .unwrap()(
        cflow_output_separator,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn print_symbol(
    mut direct: libc::c_int,
    mut level: libc::c_int,
    mut last: libc::c_int,
    mut sym: *mut Symbol,
) -> libc::c_int {
    let mut output_symbol: output_symbol = output_symbol {
        direct: 0,
        level: 0,
        last: 0,
        sym: 0 as *mut Symbol,
    };
    output_symbol.direct = direct;
    output_symbol.level = level;
    output_symbol.last = last;
    output_symbol.sym = sym;
    return (output_driver[driver_index as usize].handler)
        .unwrap()(
        cflow_output_symbol,
        outfile,
        out_line,
        &mut output_symbol as *mut output_symbol as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn compare(
    mut ap: *const libc::c_void,
    mut bp: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *mut Symbol = ap as *const *mut Symbol;
    let mut b: *const *mut Symbol = bp as *const *mut Symbol;
    return strcmp((**a).name, (**b).name);
}
unsafe extern "C" fn is_var(mut symp: *mut Symbol) -> libc::c_int {
    if include_symbol(symp) != 0 {
        if (*symp).type_0 as libc::c_uint == SymIdentifier as libc::c_int as libc::c_uint
        {
            return ((*symp).storage as libc::c_uint
                == ExternStorage as libc::c_int as libc::c_uint
                || (*symp).storage as libc::c_uint
                    == StaticStorage as libc::c_int as libc::c_uint) as libc::c_int
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn symbol_is_function(mut symp: *mut Symbol) -> libc::c_int {
    return ((*symp).type_0 as libc::c_uint
        == SymIdentifier as libc::c_int as libc::c_uint
        && (*symp).arity >= 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn clear_active(mut sym: *mut Symbol) {
    (*sym).active = 0 as libc::c_int;
}
pub unsafe extern "C" fn print_refs(
    mut name: *mut libc::c_char,
    mut reflist: *mut linked_list,
) {
    let mut refptr: *mut Ref = 0 as *mut Ref;
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    p = if !reflist.is_null() { (*reflist).head } else { 0 as *mut linked_list_entry };
    while !p.is_null() {
        refptr = (*p).data as *mut Ref;
        fprintf(
            outfile,
            b"%s   %s:%d\n\0" as *const u8 as *const libc::c_char,
            name,
            (*refptr).source,
            (*refptr).line,
        );
        p = (*p).next;
    }
}
unsafe extern "C" fn print_function(mut symp: *mut Symbol) {
    if !((*symp).source).is_null() {
        fprintf(
            outfile,
            b"%s * %s:%d %s\n\0" as *const u8 as *const libc::c_char,
            (*symp).name,
            (*symp).source,
            (*symp).def_line,
            (*symp).decl,
        );
    }
    print_refs((*symp).name, (*symp).ref_line);
}
unsafe extern "C" fn print_type(mut symp: *mut Symbol) {
    if !((*symp).source).is_null() {
        fprintf(
            outfile,
            b"%s t %s:%d\n\0" as *const u8 as *const libc::c_char,
            (*symp).name,
            (*symp).source,
            (*symp).def_line,
        );
    }
}
pub unsafe extern "C" fn xref_output() {
    let mut symbols: *mut *mut Symbol = 0 as *mut *mut Symbol;
    let mut symp: *mut Symbol = 0 as *mut Symbol;
    let mut i: size_t = 0;
    let mut num: size_t = 0;
    num = collect_symbols(
        &mut symbols,
        Some(is_var as unsafe extern "C" fn(*mut Symbol) -> libc::c_int),
        0 as libc::c_int as size_t,
    );
    qsort(
        symbols as *mut libc::c_void,
        num,
        ::std::mem::size_of::<*mut Symbol>() as libc::c_ulong,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int as size_t;
    while i < num {
        symp = *symbols.offset(i as isize);
        match (*symp).type_0 as libc::c_uint {
            2 => {
                print_function(symp);
            }
            1 => {
                print_type(symp);
            }
            0 | _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    free(symbols as *mut libc::c_void);
}
unsafe extern "C" fn set_active(mut sym: *mut Symbol) {
    (*sym).active = out_line;
}
unsafe extern "C" fn is_printable(mut p: *mut linked_list_entry) -> libc::c_int {
    return (!p.is_null() && include_symbol((*p).data as *mut Symbol) != 0)
        as libc::c_int;
}
unsafe extern "C" fn is_last(mut p: *mut linked_list_entry) -> libc::c_int {
    loop {
        p = (*p).next;
        if p.is_null() {
            break;
        }
        if is_printable(p) != 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn direct_tree(
    mut lev: libc::c_int,
    mut last: libc::c_int,
    mut sym: *mut Symbol,
) {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    let mut rc: libc::c_int = 0;
    if (*sym).type_0 as libc::c_uint == SymUndefined as libc::c_int as libc::c_uint
        || max_depth != 0 && lev >= max_depth || include_symbol(sym) == 0
    {
        return;
    }
    rc = print_symbol(1 as libc::c_int, lev, last, sym);
    newline();
    if rc != 0 || (*sym).active != 0 {
        return;
    }
    set_active(sym);
    p = if !((*sym).callee).is_null() {
        (*(*sym).callee).head
    } else {
        0 as *mut linked_list_entry
    };
    while !p.is_null() {
        set_level_mark(lev + 1 as libc::c_int, (is_last(p) == 0) as libc::c_int);
        direct_tree(lev + 1 as libc::c_int, is_last(p), (*p).data as *mut Symbol);
        p = (*p).next;
    }
    clear_active(sym);
}
unsafe extern "C" fn inverted_tree(
    mut lev: libc::c_int,
    mut last: libc::c_int,
    mut sym: *mut Symbol,
) {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    let mut rc: libc::c_int = 0;
    if (*sym).type_0 as libc::c_uint == SymUndefined as libc::c_int as libc::c_uint
        || max_depth != 0 && lev >= max_depth || include_symbol(sym) == 0
    {
        return;
    }
    rc = print_symbol(0 as libc::c_int, lev, last, sym);
    newline();
    if rc != 0 || (*sym).active != 0 {
        return;
    }
    set_active(sym);
    p = if !((*sym).caller).is_null() {
        (*(*sym).caller).head
    } else {
        0 as *mut linked_list_entry
    };
    while !p.is_null() {
        set_level_mark(lev + 1 as libc::c_int, (is_last(p) == 0) as libc::c_int);
        inverted_tree(lev + 1 as libc::c_int, is_last(p), (*p).data as *mut Symbol);
        p = (*p).next;
    }
    clear_active(sym);
}
unsafe extern "C" fn tree_output() {
    let mut symbols: *mut *mut Symbol = 0 as *mut *mut Symbol;
    let mut main_sym: *mut Symbol = 0 as *mut Symbol;
    let mut i: size_t = 0;
    let mut num: size_t = 0;
    let mut depmap: cflow_depmap_t = 0 as *mut cflow_depmap;
    num = collect_functions(&mut symbols);
    i = 0 as libc::c_int as size_t;
    while i < num {
        (**symbols.offset(i as isize)).ord = i;
        i = i.wrapping_add(1);
        i;
    }
    depmap = depmap_alloc(num);
    i = 0 as libc::c_int as size_t;
    while i < num {
        if !((**symbols.offset(i as isize)).callee).is_null() {
            let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
            p = if !((**symbols.offset(i as isize)).callee).is_null() {
                (*(**symbols.offset(i as isize)).callee).head
            } else {
                0 as *mut linked_list_entry
            };
            while !p.is_null() {
                let mut s: *mut Symbol = (*p).data as *mut Symbol;
                if (*s).ord != -(1 as libc::c_int) as libc::c_ulong
                    && symbol_is_function(s) != 0
                {
                    depmap_set(depmap, i, (*s).ord);
                }
                p = (*p).next;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    depmap_tc(depmap);
    i = 0 as libc::c_int as size_t;
    while i < num {
        if depmap_isset(depmap, i, i) != 0 {
            (**symbols.offset(i as isize)).recursive = 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(depmap as *mut libc::c_void);
    free(symbols as *mut libc::c_void);
    num = collect_symbols(
        &mut symbols,
        Some(is_var as unsafe extern "C" fn(*mut Symbol) -> libc::c_int),
        0 as libc::c_int as size_t,
    );
    qsort(
        symbols as *mut libc::c_void,
        num,
        ::std::mem::size_of::<*mut Symbol>() as libc::c_ulong,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    begin();
    if reverse_tree != 0 {
        i = 0 as libc::c_int as size_t;
        while i < num {
            inverted_tree(
                0 as libc::c_int,
                0 as libc::c_int,
                *symbols.offset(i as isize),
            );
            separator();
            i = i.wrapping_add(1);
            i;
        }
    } else {
        let mut state: *mut libc::c_void = 0 as *mut libc::c_void;
        main_sym = first_starter(
            &mut state as *mut *mut libc::c_void as *mut libc::c_void,
        );
        if !main_sym.is_null() {
            loop {
                direct_tree(0 as libc::c_int, 0 as libc::c_int, main_sym);
                separator();
                main_sym = next_starter(
                    &mut state as *mut *mut libc::c_void as *mut libc::c_void,
                );
                if main_sym.is_null() {
                    break;
                }
            }
        } else if all_functions == 0 {
            all_functions = 1 as libc::c_int;
        }
        if all_functions != 0 {
            i = 0 as libc::c_int as size_t;
            while i < num {
                if (**symbols.offset(i as isize)).flag as libc::c_uint
                    != symbol_start as libc::c_int as libc::c_uint
                    && !((**symbols.offset(i as isize)).source).is_null()
                    && (all_functions > 1 as libc::c_int
                        || ((**symbols.offset(i as isize)).caller).is_null())
                {
                    direct_tree(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        *symbols.offset(i as isize),
                    );
                    separator();
                }
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    end();
    free(symbols as *mut libc::c_void);
}
pub unsafe extern "C" fn output() {
    if strcmp(outname, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        outfile = stdout;
    } else {
        outfile = fopen(outname, b"w\0" as *const u8 as *const libc::c_char);
        if outfile.is_null() {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot open file `%s'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                outname,
            );
        }
    }
    set_level_mark(0 as libc::c_int, 0 as libc::c_int);
    if print_option & 0x1 as libc::c_int != 0 {
        xref_output();
    }
    if print_option & 0x2 as libc::c_int != 0 {
        eliminate_non_targets();
        tree_output();
    }
    fclose(outfile);
}
