use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stringhash_st;
    fn exit(_: libc::c_int) -> !;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn re_search(
        buffer: *mut re_pattern_buffer,
        string: *const libc::c_char,
        length: libc::c_int,
        start: libc::c_int,
        range: libc::c_int,
        regs: *mut re_registers,
    ) -> libc::c_int;
    fn strhash_get(
        hash: StringHashPtr,
        key: *const libc::c_char,
        keylen: libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    static mut program: *mut libc::c_char;
    static mut ofp: *mut FILE;
    static mut ns_vars: StringHashPtr;
    static mut start_stmts: *mut List;
    static mut nvoid: *mut Node;
    static mut ifp: *mut FILE;
    static mut inbuf: *mut libc::c_char;
    static mut data_in_buffer: libc::c_uint;
    static mut bufpos: libc::c_uint;
    static mut eof_seen: libc::c_int;
    static mut current_fname: *mut libc::c_char;
    static mut current_linenum: libc::c_uint;
    static mut current_match: *mut re_registers;
    static mut current_match_buf: *mut libc::c_char;
    static mut start_state_arg: *mut libc::c_char;
    static mut start_state: *mut libc::c_char;
    fn node_free(node: *mut Node);
    fn enter_system_variable(name: *mut libc::c_char, value: *mut libc::c_char);
    fn compile_regexp(regexp: *mut Node);
    fn eval_statement_list(
        lst: *mut List,
        env: *mut Environment,
        return_seen: *mut libc::c_int,
    ) -> *mut Node;
    fn lookup_state(name: *mut libc::c_char) -> *mut State;
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
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut libc::c_uchar,
    pub allocated: libc::c_ulong,
    pub used: libc::c_ulong,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_char,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: libc::c_uint,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_item_st {
    pub next: *mut list_item_st,
    pub data: *mut libc::c_void,
}
pub type ListItem = list_item_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_st {
    pub head: *mut ListItem,
    pub tail: *mut ListItem,
}
pub type List = list_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_st {
    pub name: *mut libc::c_char,
    pub super_name: *mut libc::c_char,
    pub super_0: *mut state_st,
    pub rules: *mut List,
}
pub type State = state_st;
pub type NodeType = libc::c_uint;
pub const nARRAY: NodeType = 6;
pub const nSYMBOL: NodeType = 5;
pub const nREAL: NodeType = 4;
pub const nINTEGER: NodeType = 3;
pub const nREGEXP: NodeType = 2;
pub const nSTRING: NodeType = 1;
pub const nVOID: NodeType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub type_0: NodeType,
    pub refcount: libc::c_uint,
    pub linenum: libc::c_uint,
    pub filename: *mut libc::c_char,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: C2RustUnnamed_2,
    pub re: C2RustUnnamed_1,
    pub integer: libc::c_int,
    pub real: libc::c_double,
    pub sym: *mut libc::c_char,
    pub array: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub array: *mut *mut node_st,
    pub len: libc::c_uint,
    pub allocated: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: *mut libc::c_char,
    pub len: libc::c_uint,
    pub flags: libc::c_uint,
    pub compiled: regex_t,
    pub matches: re_registers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: *mut libc::c_char,
    pub len: libc::c_uint,
}
pub type Node = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cons_st {
    pub car: *mut libc::c_void,
    pub cdr: *mut libc::c_void,
}
pub type Cons = cons_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment_st {
    pub next: *mut environment_st,
    pub name: *mut libc::c_char,
    pub val: *mut Node,
}
pub type Environment = environment_st;
pub unsafe extern "C" fn process_file(mut fname: *mut libc::c_char) {
    let mut result: *mut Node = 0 as *mut Node;
    let mut return_seen: libc::c_int = 0 as libc::c_int;
    start_state = 0 as *mut libc::c_char;
    current_fname = fname;
    current_linenum = 1 as libc::c_int as libc::c_uint;
    data_in_buffer = 0 as libc::c_int as libc::c_uint;
    bufpos = 0 as libc::c_int as libc::c_uint;
    eof_seen = 0 as libc::c_int;
    enter_system_variable(
        b"filename\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        fname,
    );
    data_in_buffer = fread(
        inbuf as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (20 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
        ifp,
    ) as libc::c_uint;
    if data_in_buffer < (20 as libc::c_int * 1024 as libc::c_int) as libc::c_uint {
        eof_seen = 1 as libc::c_int;
    }
    if !start_state_arg.is_null() {
        start_state = start_state_arg;
    }
    result = eval_statement_list(start_stmts, 0 as *mut Environment, &mut return_seen);
    node_free(result);
    if start_state.is_null() {
        while data_in_buffer != 0 {
            fwrite(
                inbuf as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                data_in_buffer as libc::c_ulong,
                ofp,
            );
            data_in_buffer = fread(
                inbuf as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                (20 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
                ifp,
            ) as libc::c_uint;
        }
    } else {
        result = execute_state(start_state);
        node_free(result);
    };
}
pub unsafe extern "C" fn execute_state(mut name: *mut libc::c_char) -> *mut Node {
    let mut current_block: u64;
    let mut state: *mut State = 0 as *mut State;
    let mut s: *mut State = 0 as *mut State;
    let mut to_read: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut rule: *mut ListItem = 0 as *mut ListItem;
    let mut first_rule: *mut ListItem = 0 as *mut ListItem;
    let mut first_idx: libc::c_uint = 0;
    let mut match_len: libc::c_uint = 0;
    let mut result: *mut Node = nvoid;
    let mut r: *mut Cons = 0 as *mut Cons;
    let mut exp: *mut Node = 0 as *mut Node;
    let mut return_seen: libc::c_int = 0 as libc::c_int;
    let mut idx: libc::c_int = 0;
    state = lookup_state(name);
    if state.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: undefined state `%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program,
            name,
        );
        exit(1 as libc::c_int);
    }
    result = eval_begin_rules(state, &mut return_seen);
    if return_seen != 0 {
        current_block = 13322496309399262736;
    } else {
        current_block = 3640593987805443782;
    }
    loop {
        match current_block {
            13322496309399262736 => {
                let mut found: libc::c_int = 0 as libc::c_int;
                let mut result2: *mut Node = 0 as *mut Node;
                result2 = eval_end_rules(state, &mut found);
                if found != 0 {
                    node_free(result);
                    result = result2;
                }
                break;
            }
            _ => {
                let mut eol: libc::c_int = 0;
                if bufpos >= data_in_buffer {
                    if eof_seen != 0 {
                        current_block = 13322496309399262736;
                        continue;
                    }
                    data_in_buffer = fread(
                        inbuf as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        (20 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
                        ifp,
                    ) as libc::c_uint;
                    if data_in_buffer
                        < (20 as libc::c_int * 1024 as libc::c_int) as libc::c_uint
                    {
                        eof_seen = 1 as libc::c_int;
                    }
                    bufpos = 0 as libc::c_int as libc::c_uint;
                    current_block = 3640593987805443782;
                } else {
                    if bufpos > 0 as libc::c_int as libc::c_uint
                        && *inbuf
                            .offset(
                                bufpos.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ) as libc::c_int == '\n' as i32
                    {
                        current_linenum = current_linenum.wrapping_add(1);
                        current_linenum;
                    }
                    eol = bufpos as libc::c_int;
                    while (eol as libc::c_uint) < data_in_buffer
                        && *inbuf.offset(eol as isize) as libc::c_int != '\n' as i32
                    {
                        eol += 1;
                        eol;
                    }
                    if (eol as libc::c_uint) < data_in_buffer
                        && *inbuf.offset(eol as isize) as libc::c_int == '\n' as i32
                    {
                        eol += 1;
                        eol;
                    }
                    if eol as libc::c_uint >= data_in_buffer && eof_seen == 0
                        && bufpos > 0 as libc::c_int as libc::c_uint
                    {
                        memmove(
                            inbuf as *mut libc::c_void,
                            inbuf.offset(bufpos as isize) as *const libc::c_void,
                            (eol as libc::c_uint).wrapping_sub(bufpos) as libc::c_ulong,
                        );
                        data_in_buffer = (eol as libc::c_uint).wrapping_sub(bufpos);
                        bufpos = 0 as libc::c_int as libc::c_uint;
                        to_read = ((20 as libc::c_int * 1024 as libc::c_int)
                            as libc::c_uint)
                            .wrapping_sub(data_in_buffer) as libc::c_int;
                        got = fread(
                            inbuf.offset(data_in_buffer as isize) as *mut libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            to_read as libc::c_ulong,
                            ifp,
                        ) as libc::c_int;
                        if got < to_read {
                            eof_seen = 1 as libc::c_int;
                        }
                        data_in_buffer = data_in_buffer
                            .wrapping_add(got as libc::c_uint);
                        current_block = 3640593987805443782;
                    } else {
                        first_idx = eol as libc::c_uint;
                        match_len = 0 as libc::c_int as libc::c_uint;
                        first_rule = 0 as *mut ListItem;
                        current_match = 0 as *mut re_registers;
                        s = state;
                        while !s.is_null() {
                            let mut current_block_46: u64;
                            rule = (*(*s).rules).head;
                            while !rule.is_null() {
                                let mut err: libc::c_int = 0;
                                r = (*rule).data as *mut Cons;
                                exp = (*r).car as *mut Node;
                                if !(exp.is_null()
                                    || exp
                                        == 1 as libc::c_int as *mut libc::c_void as *mut Node)
                                {
                                    if (*exp).type_0 as libc::c_uint
                                        == nSYMBOL as libc::c_int as libc::c_uint
                                    {
                                        let mut n: *mut Node = 0 as *mut Node;
                                        if strhash_get(
                                            ns_vars,
                                            (*exp).u.sym,
                                            strlen((*exp).u.sym) as libc::c_int,
                                            &mut n as *mut *mut Node as *mut *mut libc::c_void,
                                        ) == 0
                                        {
                                            fprintf(
                                                stderr,
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"%s: error: undefined variable `%s'\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                program,
                                                (*exp).u.sym,
                                            );
                                            exit(1 as libc::c_int);
                                        }
                                        if (*n).type_0 as libc::c_uint
                                            != nREGEXP as libc::c_int as libc::c_uint
                                        {
                                            current_block_46 = 11913429853522160501;
                                        } else {
                                            exp = n;
                                            current_block_46 = 15512526488502093901;
                                        }
                                    } else {
                                        current_block_46 = 15512526488502093901;
                                    }
                                    match current_block_46 {
                                        11913429853522160501 => {}
                                        _ => {
                                            err = re_search(
                                                if ((*exp).u.re.compiled).fastmap_accurate() as libc::c_int
                                                    != 0
                                                {
                                                    &mut (*exp).u.re.compiled
                                                } else {
                                                    compile_regexp(exp);
                                                    &mut (*exp).u.re.compiled
                                                },
                                                inbuf,
                                                eol,
                                                bufpos as libc::c_int,
                                                (eol as libc::c_uint).wrapping_sub(bufpos) as libc::c_int,
                                                &mut (*exp).u.re.matches,
                                            );
                                            if !(err < 0 as libc::c_int) {
                                                idx = *((*exp).u.re.matches.start)
                                                    .offset(0 as libc::c_int as isize);
                                                if idx >= 0 as libc::c_int
                                                    && ((idx as libc::c_uint) < first_idx
                                                        || idx as libc::c_uint == first_idx
                                                            && (*((*exp).u.re.matches.end)
                                                                .offset(0 as libc::c_int as isize)
                                                                - *((*exp).u.re.matches.start)
                                                                    .offset(0 as libc::c_int as isize)) as libc::c_uint
                                                                > match_len)
                                                {
                                                    first_idx = idx as libc::c_uint;
                                                    first_rule = rule;
                                                    match_len = (*((*exp).u.re.matches.end)
                                                        .offset(0 as libc::c_int as isize)
                                                        - *((*exp).u.re.matches.start)
                                                            .offset(0 as libc::c_int as isize)) as libc::c_uint;
                                                    current_match = &mut (*exp).u.re.matches;
                                                    current_match_buf = inbuf;
                                                }
                                            }
                                        }
                                    }
                                }
                                rule = (*rule).next;
                            }
                            s = (*s).super_0;
                        }
                        fwrite(
                            inbuf.offset(bufpos as isize) as *const libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            first_idx.wrapping_sub(bufpos) as libc::c_ulong,
                            ofp,
                        );
                        if !first_rule.is_null() {
                            bufpos = *((*current_match).end)
                                .offset(0 as libc::c_int as isize) as libc::c_uint;
                            node_free(result);
                            result = eval_statement_list(
                                (*((*first_rule).data as *mut Cons)).cdr as *mut List,
                                0 as *mut Environment,
                                &mut return_seen,
                            );
                            if return_seen != 0 {
                                current_block = 13322496309399262736;
                            } else {
                                current_block = 3640593987805443782;
                            }
                        } else {
                            bufpos = first_idx;
                            current_block = 3640593987805443782;
                        }
                    }
                }
            }
        }
    }
    return result;
}
unsafe extern "C" fn eval_begin_rules(
    mut state: *mut State,
    mut return_seen: *mut libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = nvoid;
    let mut r: *mut Cons = 0 as *mut Cons;
    let mut rule: *mut ListItem = 0 as *mut ListItem;
    if !((*state).super_name).is_null() && ((*state).super_0).is_null() {
        (*state).super_0 = lookup_state((*state).super_name);
        if ((*state).super_0).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: undefined super state `%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program,
                (*state).super_name,
            );
            exit(1 as libc::c_int);
        }
    }
    if !((*state).super_0).is_null() {
        result = eval_begin_rules((*state).super_0, return_seen);
        if *return_seen != 0 {
            return result;
        }
    }
    rule = (*(*state).rules).head;
    while !rule.is_null() {
        r = (*rule).data as *mut Cons;
        if ((*r).car).is_null() {
            node_free(result);
            result = eval_statement_list(
                (*r).cdr as *mut List,
                0 as *mut Environment,
                return_seen,
            );
            if *return_seen != 0 {
                break;
            }
        }
        rule = (*rule).next;
    }
    return result;
}
unsafe extern "C" fn eval_end_rules(
    mut state: *mut State,
    mut found_return: *mut libc::c_int,
) -> *mut Node {
    let mut rule: *mut ListItem = 0 as *mut ListItem;
    let mut r: *mut Cons = 0 as *mut Cons;
    let mut result: *mut Node = nvoid;
    let mut return_seen: libc::c_int = 0;
    while !state.is_null() {
        rule = (*(*state).rules).head;
        while !rule.is_null() {
            r = (*rule).data as *mut Cons;
            if (*r).car == 1 as libc::c_int as *mut libc::c_void {
                *found_return = 1 as libc::c_int;
                node_free(result);
                result = eval_statement_list(
                    (*r).cdr as *mut List,
                    0 as *mut Environment,
                    &mut return_seen,
                );
            }
            rule = (*rule).next;
        }
        state = (*state).super_0;
    }
    return result;
}
