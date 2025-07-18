use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stringhash_st;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn re_set_syntax(syntax: reg_syntax_t) -> reg_syntax_t;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xcalloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *mut libc::c_char) -> *mut libc::c_char;
    fn strhash_init() -> StringHashPtr;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut libc::c_char,
        keylen: libc::c_int,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn load_states_file(name: *mut libc::c_char);
    fn process_file(fname: *mut libc::c_char);
    fn enter_system_variable(name: *mut libc::c_char, value: *mut libc::c_char);
    fn node_free(node: *mut Node);
    fn node_alloc(type_0: NodeType) -> *mut Node;
    fn init_primitives();
    fn list() -> *mut List;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
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
pub struct variable_definition_st {
    pub next: *mut variable_definition_st,
    pub sym: *mut libc::c_char,
    pub val: *mut libc::c_char,
}
pub type VariableDef = variable_definition_st;
pub type WarningLevel = libc::c_uint;
pub const WARN_ALL: WarningLevel = 100;
pub const WARN_LIGHT: WarningLevel = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub static mut program: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ns_prims: StringHashPtr = 0 as *const stringhash_st as StringHashPtr;
pub static mut ns_vars: StringHashPtr = 0 as *const stringhash_st as StringHashPtr;
pub static mut ns_subs: StringHashPtr = 0 as *const stringhash_st as StringHashPtr;
pub static mut ns_states: StringHashPtr = 0 as *const stringhash_st as StringHashPtr;
pub static mut global_stmts: *mut List = 0 as *const List as *mut List;
pub static mut start_stmts: *mut List = 0 as *const List as *mut List;
pub static mut startrules: *mut List = 0 as *const List as *mut List;
pub static mut namerules: *mut List = 0 as *const List as *mut List;
pub static mut nvoid: *mut Node = 0 as *const Node as *mut Node;
pub static mut ifp: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut inbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut data_in_buffer: libc::c_uint = 0;
pub static mut bufpos: libc::c_uint = 0;
pub static mut eof_seen: libc::c_int = 0;
pub static mut current_fname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut current_linenum: libc::c_uint = 0;
pub static mut current_match: *mut re_registers = 0 as *const re_registers
    as *mut re_registers;
pub static mut current_match_buf: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut vardefs: *mut VariableDef = 0 as *const VariableDef as *mut VariableDef;
pub static mut vardefs_tail: *mut VariableDef = 0 as *const VariableDef
    as *mut VariableDef;
pub static mut defs_file: *mut libc::c_char = b"states.st\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut linenum: libc::c_uint = 1 as libc::c_int as libc::c_uint;
pub static mut yyin_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ofp: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut path: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut start_state_arg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut start_state: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut verbose: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut warning_level: WarningLevel = WARN_LIGHT;
static mut long_options: [option; 10] = [
    {
        let mut init = option {
            name: b"define\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"file\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"path\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"state\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"warning\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'W' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
pub static mut version: [libc::c_char; 256] = [0; 256];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut vardef: *mut VariableDef = 0 as *mut VariableDef;
    ofp = stdout;
    program = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if program.is_null() {
        program = *argv.offset(0 as libc::c_int as isize);
    } else {
        program = program.offset(1);
        program;
    }
    let ref mut fresh0 = *argv.offset(0 as libc::c_int as isize);
    *fresh0 = program;
    sprintf(
        version.as_mut_ptr(),
        dcgettext(
            0 as *const libc::c_char,
            b"states for %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        b"GNU Enscript 1.6.6\0" as *const u8 as *const libc::c_char,
    );
    setlocale(5 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"enscript\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"enscript\0" as *const u8 as *const libc::c_char);
    ns_prims = strhash_init();
    ns_vars = strhash_init();
    ns_subs = strhash_init();
    ns_states = strhash_init();
    global_stmts = list();
    start_stmts = list();
    startrules = list();
    namerules = list();
    nvoid = node_alloc(nVOID);
    inbuf = xmalloc((20 as libc::c_int * 1024 as libc::c_int) as size_t)
        as *mut libc::c_char;
    init_primitives();
    re_set_syntax(
        (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | ((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int
            | (((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int | 1 as libc::c_int as libc::c_ulong
            | ((((((((((((((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int)
            & !((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int
                | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int
                | ((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                    << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
            | (((((((((1 as libc::c_int as libc::c_ulong) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int) << 1 as libc::c_int)
                << 1 as libc::c_int) << 1 as libc::c_int,
    );
    enter_system_variable(
        b"program\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        program,
    );
    enter_system_variable(
        b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        version.as_mut_ptr(),
    );
    loop {
        let mut option_index: libc::c_int = 0 as libc::c_int;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"D:f:ho:p:s:vVW:\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            68 => {
                vardef = xcalloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<VariableDef>() as libc::c_ulong,
                ) as *mut VariableDef;
                (*vardef)
                    .sym = xmalloc(
                    (strlen(optarg)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                strcpy((*vardef).sym, optarg);
                (*vardef).val = strchr((*vardef).sym, '=' as i32);
                if ((*vardef).val).is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: malformed variable definition \"%s\"\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        program,
                        (*vardef).sym,
                    );
                    exit(1 as libc::c_int);
                }
                *(*vardef).val = '\0' as i32 as libc::c_char;
                (*vardef).val = ((*vardef).val).offset(1);
                (*vardef).val;
                if !vardefs.is_null() {
                    (*vardefs_tail).next = vardef;
                } else {
                    vardefs = vardef;
                }
                vardefs_tail = vardef;
            }
            102 => {
                defs_file = optarg;
            }
            104 => {
                usage();
                exit(0 as libc::c_int);
            }
            111 => {
                ofp = fopen(optarg, b"w\0" as *const u8 as *const libc::c_char);
                if ofp.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: couldn't create output file \"%s\": %s\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        program,
                        optarg,
                        strerror(*__errno_location()),
                    );
                    exit(1 as libc::c_int);
                }
            }
            112 => {
                path = optarg;
            }
            115 => {
                start_state_arg = optarg;
            }
            118 => {
                verbose = verbose.wrapping_add(1);
                verbose;
            }
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    version.as_mut_ptr(),
                );
                exit(0 as libc::c_int);
            }
            87 => {
                if strcmp(optarg, b"light\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    warning_level = WARN_LIGHT;
                } else if strcmp(optarg, b"all\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    warning_level = WARN_ALL;
                } else {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: unknown warning level `%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        program,
                        optarg,
                    );
                    exit(1 as libc::c_int);
                }
            }
            63 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Try `%s --help' for more information.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    program,
                );
                exit(1 as libc::c_int);
            }
            _ => {
                printf(
                    b"Hey! main() didn't handle option \"%c\" (%d)\0" as *const u8
                        as *const libc::c_char,
                    c,
                    c,
                );
                if !optarg.is_null() {
                    printf(
                        b" with arg %s\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                abort();
            }
        }
    }
    let mut v: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    v = node_alloc(nARRAY);
    (*v).u.array.allocated = (argc - optind + 1 as libc::c_int) as libc::c_uint;
    (*v).u.array.len = (argc - optind) as libc::c_uint;
    (*v)
        .u
        .array
        .array = xcalloc(
        (*v).u.array.allocated as size_t,
        ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
    ) as *mut *mut Node;
    i = optind;
    while i < argc {
        let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
        n = node_alloc(nSTRING);
        if strcmp(*argv.offset(i as isize), b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            data = b"(stdin)\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            data = *argv.offset(i as isize);
        }
        (*n).u.str_0.len = strlen(data) as libc::c_uint;
        (*n).u.str_0.data = xstrdup(data);
        let ref mut fresh1 = *((*v).u.array.array).offset((i - optind) as isize);
        *fresh1 = n;
        i += 1;
        i;
    }
    if strhash_put(
        ns_vars,
        b"argv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
        v as *mut libc::c_void,
        &mut n as *mut *mut Node as *mut *mut libc::c_void,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: out of memory\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program,
        );
        exit(1 as libc::c_int);
    }
    node_free(n);
    if path.is_null() {
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        cp = strrchr(defs_file, '/' as i32);
        if !cp.is_null() {
            path = xmalloc(
                (cp.offset_from(defs_file) as libc::c_long
                    + 3 as libc::c_int as libc::c_long) as size_t,
            ) as *mut libc::c_char;
            sprintf(
                path,
                b".%c%.*s\0" as *const u8 as *const libc::c_char,
                ':' as i32,
                cp.offset_from(defs_file) as libc::c_long,
                defs_file,
            );
        } else {
            path = b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    load_states_file(defs_file);
    vardef = vardefs;
    while !vardef.is_null() {
        let mut val: *mut Node = 0 as *mut Node;
        let mut old_val: *mut Node = 0 as *mut Node;
        val = node_alloc(nSTRING);
        (*val).u.str_0.len = strlen((*vardef).val) as libc::c_uint;
        (*val).u.str_0.data = xstrdup((*vardef).val);
        if strhash_put(
            ns_vars,
            (*vardef).sym,
            strlen((*vardef).sym) as libc::c_int,
            val as *mut libc::c_void,
            &mut old_val as *mut *mut Node as *mut *mut libc::c_void,
        ) == 0
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: out of memory\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program,
            );
            exit(1 as libc::c_int);
        }
        node_free(old_val);
        vardef = (*vardef).next;
    }
    if optind == argc {
        ifp = stdin;
        process_file(
            b"(stdin)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        while optind < argc {
            if strcmp(
                *argv.offset(optind as isize),
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                ifp = stdin;
                process_file(
                    b"(stdin)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                ifp = fopen(
                    *argv.offset(optind as isize),
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if ifp.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: couldn't open input file `%s': %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        program,
                        *argv.offset(optind as isize),
                        strerror(*__errno_location()),
                    );
                    exit(1 as libc::c_int);
                }
                process_file(*argv.offset(optind as isize));
                fclose(ifp);
            }
            optind += 1;
            optind;
        }
    }
    if ofp != stdout {
        fclose(ofp);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... [FILE]...\nMandatory arguments to long options are mandatory for short options too.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"  -D, --define=VAR=VAL       define variable VAR to have value VAL\n  -f, --file=NAME            read state definitions from file NAME\n  -h, --help                 print this help and exit\n  -o, --output=NAME          save output to file NAME\n  -p, --path=PATH            set the load path to PATH\n  -s, --state=NAME           start from state NAME\n  -v, --verbose              increase the program verbosity\n  -V, --version              print version number\n  -W, --warning=LEVEL        set the warning level to LEVEL\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
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
