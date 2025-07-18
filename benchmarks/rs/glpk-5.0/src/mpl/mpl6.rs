use ::libc;
extern "C" {
    pub type AVL;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type DMP;
    pub type glp_file;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _glp_xstrerr(_: libc::c_int) -> *mut libc::c_char;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
    fn _glp_strspx(str: *mut libc::c_char) -> *mut libc::c_char;
    fn _glp_strtrim(str: *mut libc::c_char) -> *mut libc::c_char;
    fn _glp_mpl_tab_num_args(dca: *mut TABDCA) -> libc::c_int;
    fn _glp_mpl_tab_get_arg(dca: *mut TABDCA, k: libc::c_int) -> *const libc::c_char;
    fn _glp_mpl_tab_num_flds(dca: *mut TABDCA) -> libc::c_int;
    fn _glp_mpl_tab_get_name(dca: *mut TABDCA, k: libc::c_int) -> *const libc::c_char;
    fn _glp_mpl_tab_get_type(dca: *mut TABDCA, k: libc::c_int) -> libc::c_int;
    fn _glp_mpl_tab_get_num(dca: *mut TABDCA, k: libc::c_int) -> libc::c_double;
    fn _glp_mpl_tab_get_str(dca: *mut TABDCA, k: libc::c_int) -> *const libc::c_char;
    fn _glp_mpl_tab_set_num(dca: *mut TABDCA, k: libc::c_int, num: libc::c_double);
    fn _glp_mpl_tab_set_str(dca: *mut TABDCA, k: libc::c_int, str: *const libc::c_char);
    fn _glp_mpl_error(mpl: *mut MPL, fmt: *mut libc::c_char, _: ...);
    fn _glp_db_iodbc_open(dca: *mut TABDCA, mode: libc::c_int) -> *mut libc::c_void;
    fn _glp_db_iodbc_read(dca: *mut TABDCA, link: *mut libc::c_void) -> libc::c_int;
    fn _glp_db_iodbc_write(dca: *mut TABDCA, link: *mut libc::c_void) -> libc::c_int;
    fn _glp_db_iodbc_close(dca: *mut TABDCA, link: *mut libc::c_void) -> libc::c_int;
    fn _glp_db_mysql_open(dca: *mut TABDCA, mode: libc::c_int) -> *mut libc::c_void;
    fn _glp_db_mysql_read(dca: *mut TABDCA, link: *mut libc::c_void) -> libc::c_int;
    fn _glp_db_mysql_write(dca: *mut TABDCA, link: *mut libc::c_void) -> libc::c_int;
    fn _glp_db_mysql_close(dca: *mut TABDCA, link: *mut libc::c_void) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
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
pub struct RNG {
    pub A: [libc::c_int; 56],
    pub fptr: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tran {
    pub line: libc::c_int,
    pub c: libc::c_int,
    pub token: libc::c_int,
    pub imlen: libc::c_int,
    pub image: *mut libc::c_char,
    pub value: libc::c_double,
    pub b_token: libc::c_int,
    pub b_imlen: libc::c_int,
    pub b_image: *mut libc::c_char,
    pub b_value: libc::c_double,
    pub f_dots: libc::c_int,
    pub f_scan: libc::c_int,
    pub f_token: libc::c_int,
    pub f_imlen: libc::c_int,
    pub f_image: *mut libc::c_char,
    pub f_value: libc::c_double,
    pub context: *mut libc::c_char,
    pub c_ptr: libc::c_int,
    pub flag_d: libc::c_int,
    pub pool: *mut DMP,
    pub tree: *mut AVL,
    pub model: *mut STATEMENT,
    pub flag_x: libc::c_int,
    pub as_within: libc::c_int,
    pub as_in: libc::c_int,
    pub as_binary: libc::c_int,
    pub flag_s: libc::c_int,
    pub strings: *mut DMP,
    pub symbols: *mut DMP,
    pub tuples: *mut DMP,
    pub arrays: *mut DMP,
    pub members: *mut DMP,
    pub elemvars: *mut DMP,
    pub formulae: *mut DMP,
    pub elemcons: *mut DMP,
    pub a_list: *mut ARRAY,
    pub sym_buf: *mut libc::c_char,
    pub tup_buf: *mut libc::c_char,
    pub rand: *mut RNG,
    pub flag_p: libc::c_int,
    pub stmt: *mut STATEMENT,
    pub dca: *mut TABDCA,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub row: *mut *mut ELEMCON,
    pub col: *mut *mut ELEMVAR,
    pub in_fp: *mut glp_file,
    pub in_file: *mut libc::c_char,
    pub out_fp: *mut glp_file,
    pub out_file: *mut libc::c_char,
    pub prt_fp: *mut glp_file,
    pub prt_file: *mut libc::c_char,
    pub jump: jmp_buf,
    pub phase: libc::c_int,
    pub mod_file: *mut libc::c_char,
    pub mpl_buf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMVAR {
    pub j: libc::c_int,
    pub var: *mut VARIABLE,
    pub memb: *mut MEMBER,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub temp: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEMBER {
    pub tuple: *mut TUPLE,
    pub next: *mut MEMBER,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VALUE {
    pub none: *mut libc::c_void,
    pub num: libc::c_double,
    pub sym: *mut SYMBOL,
    pub bit: libc::c_int,
    pub tuple: *mut TUPLE,
    pub set: *mut ELEMSET,
    pub var: *mut ELEMVAR,
    pub form: *mut FORMULA,
    pub con: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMCON {
    pub i: libc::c_int,
    pub con: *mut CONSTRAINT,
    pub memb: *mut MEMBER,
    pub form: *mut FORMULA,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMULA {
    pub coef: libc::c_double,
    pub var: *mut ELEMVAR,
    pub next: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONSTRAINT {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub code: *mut CODE,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY {
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub size: libc::c_int,
    pub head: *mut MEMBER,
    pub tail: *mut MEMBER,
    pub tree: *mut AVL,
    pub prev: *mut ARRAY,
    pub next: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODE {
    pub op: libc::c_int,
    pub arg: OPERANDS,
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub up: *mut CODE,
    pub vflag: libc::c_int,
    pub valid: libc::c_int,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OPERANDS {
    pub num: libc::c_double,
    pub str_0: *mut libc::c_char,
    pub index: C2RustUnnamed_6,
    pub par: C2RustUnnamed_5,
    pub set: C2RustUnnamed_4,
    pub var: C2RustUnnamed_3,
    pub con: C2RustUnnamed_2,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_1,
    pub loop_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub domain: *mut DOMAIN1,
    pub x: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN1 {
    pub list: *mut DOMAIN_BLOCK,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_BLOCK {
    pub list: *mut DOMAIN_SLOT,
    pub code: *mut CODE,
    pub backup: *mut TUPLE,
    pub next: *mut DOMAIN_BLOCK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUPLE {
    pub sym: *mut SYMBOL,
    pub next: *mut TUPLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub num: libc::c_double,
    pub str_0: *mut STRING,
}
pub type STRING = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_SLOT {
    pub name: *mut libc::c_char,
    pub code: *mut CODE,
    pub value: *mut SYMBOL,
    pub list: *mut CODE,
    pub next: *mut DOMAIN_SLOT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: *mut CODE,
    pub y: *mut CODE,
    pub z: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARG_LIST {
    pub x: *mut CODE,
    pub next: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub var: *mut VARIABLE,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VARIABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub set: *mut SET,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SET {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub dimen: libc::c_int,
    pub within: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub gadget: *mut GADGET,
    pub data: libc::c_int,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GADGET {
    pub set: *mut SET,
    pub ind: [libc::c_int; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WITHIN {
    pub code: *mut CODE,
    pub next: *mut WITHIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub par: *mut PARAMETER,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARAMETER {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub cond: *mut CONDITION,
    pub in_0: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub data: libc::c_int,
    pub defval: *mut SYMBOL,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONDITION {
    pub rho: libc::c_int,
    pub code: *mut CODE,
    pub next: *mut CONDITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub slot: *mut DOMAIN_SLOT,
    pub next: *mut CODE,
}
pub type ELEMSET = ARRAY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: libc::c_int,
    pub link: *mut libc::c_void,
    pub na: libc::c_int,
    pub arg: *mut *mut libc::c_char,
    pub nf: libc::c_int,
    pub name: *mut *mut libc::c_char,
    pub type_0: *mut libc::c_int,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATEMENT {
    pub line: libc::c_int,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_7,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub tab: *mut TABLE,
    pub slv: *mut libc::c_void,
    pub chk: *mut CHECK,
    pub dpy: *mut DISPLAY,
    pub prt: *mut PRINTF,
    pub fur: *mut FOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FOR {
    pub domain: *mut DOMAIN1,
    pub list: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF {
    pub domain: *mut DOMAIN1,
    pub fmt: *mut CODE,
    pub list: *mut PRINTF1,
    pub fname: *mut CODE,
    pub app: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF1 {
    pub code: *mut CODE,
    pub next: *mut PRINTF1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY {
    pub domain: *mut DOMAIN1,
    pub list: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY1 {
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_8,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub slot: *mut DOMAIN_SLOT,
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHECK {
    pub domain: *mut DOMAIN1,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub arg: *mut TABARG,
    pub u: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub in_0: C2RustUnnamed_11,
    pub out: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub domain: *mut DOMAIN1,
    pub list: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABOUT {
    pub code: *mut CODE,
    pub name: *mut libc::c_char,
    pub next: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub set: *mut SET,
    pub fld: *mut TABFLD,
    pub list: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABIN {
    pub par: *mut PARAMETER,
    pub name: *mut libc::c_char,
    pub next: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABFLD {
    pub name: *mut libc::c_char,
    pub next: *mut TABFLD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABARG {
    pub code: *mut CODE,
    pub next: *mut TABARG,
}
pub type MPL = glp_tran;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbf {
    pub mode: libc::c_int,
    pub fname: *mut libc::c_char,
    pub fp: *mut FILE,
    pub jump: jmp_buf,
    pub offset: libc::c_int,
    pub count: libc::c_int,
    pub nf: libc::c_int,
    pub ref_0: [libc::c_int; 51],
    pub type_0: [libc::c_int; 51],
    pub len: [libc::c_int; 51],
    pub prec: [libc::c_int; 51],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv {
    pub mode: libc::c_int,
    pub fname: *mut libc::c_char,
    pub fp: *mut FILE,
    pub jump: jmp_buf,
    pub count: libc::c_int,
    pub c: libc::c_int,
    pub what: libc::c_int,
    pub field: [libc::c_char; 101],
    pub nf: libc::c_int,
    pub ref_0: [libc::c_int; 51],
    pub nskip: libc::c_int,
}
unsafe extern "C" fn read_char(mut csv: *mut csv) {
    let mut c: libc::c_int = 0;
    ((*csv).c != -(1 as libc::c_int)
        || {
            glp_assert_(
                b"csv->c != EOF\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*csv).c == '\n' as i32 {
        (*csv).count += 1;
        (*csv).count;
    }
    loop {
        c = fgetc((*csv).fp);
        if ferror((*csv).fp) != 0 {
            glp_printf(
                b"%s:%d: read error - %s\n\0" as *const u8 as *const libc::c_char,
                (*csv).fname,
                (*csv).count,
                _glp_xstrerr(*__errno_location()),
            );
            longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        if feof((*csv).fp) != 0 {
            if (*csv).c == '\n' as i32 {
                (*csv).count -= 1;
                (*csv).count;
                c = -(1 as libc::c_int);
            } else {
                glp_printf(
                    b"%s:%d: warning: missing final end-of-line\n\0" as *const u8
                        as *const libc::c_char,
                    (*csv).fname,
                    (*csv).count,
                );
                c = '\n' as i32;
            }
            break;
        } else {
            if c == '\r' as i32 {
                continue;
            }
            if !(c == '\n' as i32) {
                if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    glp_printf(
                        b"%s:%d: invalid control character 0x%02X\n\0" as *const u8
                            as *const libc::c_char,
                        (*csv).fname,
                        (*csv).count,
                        c,
                    );
                    longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                }
            }
            break;
        }
    }
    (*csv).c = c;
}
unsafe extern "C" fn read_field(mut csv: *mut csv) {
    let mut quote: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut current_block: u64;
    if (*csv).c == -(1 as libc::c_int) {
        (*csv).what = 0 as libc::c_int;
        strcpy(
            ((*csv).field).as_mut_ptr(),
            b"EOF\0" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*csv).c == '\n' as i32 {
            (*csv).what = 1 as libc::c_int;
            strcpy(
                ((*csv).field).as_mut_ptr(),
                b"EOR\0" as *const u8 as *const libc::c_char,
            );
            read_char(csv);
            if (*csv).c == ',' as i32 {
                current_block = 5306725316661857025;
            } else {
                if (*csv).c == '\n' as i32 {
                    glp_printf(
                        b"%s:%d: empty record not allowed\n\0" as *const u8
                            as *const libc::c_char,
                        (*csv).fname,
                        (*csv).count,
                    );
                    longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                }
                if (*csv).c == '#' as i32 && (*csv).count == 1 as libc::c_int {
                    while (*csv).c == '#' as i32 {
                        while (*csv).c != '\n' as i32 {
                            read_char(csv);
                        }
                        read_char(csv);
                        (*csv).nskip += 1;
                        (*csv).nskip;
                    }
                }
                current_block = 17318156935761887089;
            }
        } else {
            if (*csv).c == ',' as i32 {
                read_char(csv);
            }
            if (*csv).c == '\'' as i32 || (*csv).c == '"' as i32 {
                quote = (*csv).c;
                len = 0 as libc::c_int;
                (*csv).what = 3 as libc::c_int;
                read_char(csv);
                loop {
                    if (*csv).c == quote {
                        read_char(csv);
                        if !((*csv).c == quote) {
                            if (*csv).c == ',' as i32 || (*csv).c == '\n' as i32 {
                                current_block = 1118134448028020070;
                                break;
                            } else {
                                current_block = 18386322304582297246;
                                break;
                            }
                        }
                    }
                    if len == 100 as libc::c_int {
                        current_block = 17642600974087721525;
                        break;
                    }
                    let fresh0 = len;
                    len = len + 1;
                    (*csv).field[fresh0 as usize] = (*csv).c as libc::c_char;
                    read_char(csv);
                }
                match current_block {
                    17642600974087721525 => {}
                    _ => {
                        match current_block {
                            18386322304582297246 => {
                                glp_printf(
                                    b"%s:%d: invalid field\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*csv).fname,
                                    (*csv).count,
                                );
                                longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                            }
                            _ => {
                                if len == 0 as libc::c_int {
                                    current_block = 5306725316661857025;
                                } else {
                                    (*csv).field[len as usize] = '\0' as i32 as libc::c_char;
                                    current_block = 17318156935761887089;
                                }
                            }
                        }
                    }
                }
            } else {
                let mut len_0: libc::c_int = 0 as libc::c_int;
                let mut temp: libc::c_double = 0.;
                (*csv).what = 2 as libc::c_int;
                loop {
                    if (*csv).c == ',' as i32 || (*csv).c == '\n' as i32 {
                        current_block = 721385680381463314;
                        break;
                    }
                    if (*csv).c == '\'' as i32 || (*csv).c == '"' as i32 {
                        glp_printf(
                            b"%s:%d: invalid use of single or double quote within field\n\0"
                                as *const u8 as *const libc::c_char,
                            (*csv).fname,
                            (*csv).count,
                        );
                        longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                    }
                    if len_0 == 100 as libc::c_int {
                        current_block = 17642600974087721525;
                        break;
                    }
                    let fresh1 = len_0;
                    len_0 = len_0 + 1;
                    (*csv).field[fresh1 as usize] = (*csv).c as libc::c_char;
                    read_char(csv);
                }
                match current_block {
                    17642600974087721525 => {}
                    _ => {
                        if len_0 == 0 as libc::c_int {
                            current_block = 5306725316661857025;
                        } else {
                            (*csv).field[len_0 as usize] = '\0' as i32 as libc::c_char;
                            if _glp_str2num(((*csv).field).as_mut_ptr(), &mut temp) != 0
                            {
                                (*csv).what = 3 as libc::c_int;
                            }
                            current_block = 17318156935761887089;
                        }
                    }
                }
            }
            match current_block {
                17318156935761887089 => {}
                5306725316661857025 => {}
                _ => {
                    glp_printf(
                        b"%s:%d: field too long\n\0" as *const u8 as *const libc::c_char,
                        (*csv).fname,
                        (*csv).count,
                    );
                    longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                }
            }
        }
        match current_block {
            17318156935761887089 => {}
            _ => {
                glp_printf(
                    b"%s:%d: empty field not allowed\n\0" as *const u8
                        as *const libc::c_char,
                    (*csv).fname,
                    (*csv).count,
                );
                longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
            }
        }
    };
}
unsafe extern "C" fn csv_open_file(
    mut dca: *mut TABDCA,
    mut mode: libc::c_int,
) -> *mut csv {
    let mut csv: *mut csv = 0 as *mut csv;
    csv = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<csv>() as libc::c_ulong as libc::c_int,
    ) as *mut csv;
    (*csv).mode = mode;
    (*csv).fname = 0 as *mut libc::c_char;
    (*csv).fp = 0 as *mut FILE;
    if _setjmp(((*csv).jump).as_mut_ptr()) != 0 {
        if !((*csv).fname).is_null() {
            glp_free((*csv).fname as *mut libc::c_void);
        }
        if !((*csv).fp).is_null() {
            fclose((*csv).fp);
        }
        glp_free(csv as *mut libc::c_void);
        return 0 as *mut csv;
    } else {
        (*csv).count = 0 as libc::c_int;
        (*csv).c = '\n' as i32;
        (*csv).what = 0 as libc::c_int;
        (*csv).field[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*csv).nf = 0 as libc::c_int;
        if _glp_mpl_tab_num_args(dca) < 2 as libc::c_int {
            glp_printf(
                b"csv_driver: file name not specified\n\0" as *const u8
                    as *const libc::c_char,
            );
            longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        (*csv)
            .fname = glp_alloc(
            1 as libc::c_int,
            (strlen(_glp_mpl_tab_get_arg(dca, 2 as libc::c_int)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*csv).fname, _glp_mpl_tab_get_arg(dca, 2 as libc::c_int));
        if mode == 'R' as i32 {
            let mut k: libc::c_int = 0;
            (*csv).fp = fopen((*csv).fname, b"r\0" as *const u8 as *const libc::c_char);
            if ((*csv).fp).is_null() {
                glp_printf(
                    b"csv_driver: unable to open %s - %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*csv).fname,
                    _glp_xstrerr(*__errno_location()),
                );
                longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            (*csv).nskip = 0 as libc::c_int;
            read_field(csv);
            ((*csv).what == 1 as libc::c_int
                || {
                    glp_assert_(
                        b"csv->what == CSV_EOR\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                        254 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ((*csv).nf == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"csv->nf == 0\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                        256 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            loop {
                read_field(csv);
                if (*csv).what == 1 as libc::c_int {
                    break;
                }
                if (*csv).what != 3 as libc::c_int {
                    glp_printf(
                        b"%s:%d: invalid field name\n\0" as *const u8
                            as *const libc::c_char,
                        (*csv).fname,
                        (*csv).count,
                    );
                    longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                }
                if (*csv).nf == 50 as libc::c_int {
                    glp_printf(
                        b"%s:%d: too many fields\n\0" as *const u8
                            as *const libc::c_char,
                        (*csv).fname,
                        (*csv).count,
                    );
                    longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                }
                (*csv).nf += 1;
                (*csv).nf;
                k = _glp_mpl_tab_num_flds(dca);
                while k >= 1 as libc::c_int {
                    if strcmp(_glp_mpl_tab_get_name(dca, k), ((*csv).field).as_mut_ptr())
                        == 0 as libc::c_int
                    {
                        break;
                    }
                    k -= 1;
                    k;
                }
                (*csv).ref_0[(*csv).nf as usize] = k;
            }
            k = _glp_mpl_tab_num_flds(dca);
            while k >= 1 as libc::c_int {
                if strcmp(
                    _glp_mpl_tab_get_name(dca, k),
                    b"RECNO\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    break;
                }
                k -= 1;
                k;
            }
            (*csv).ref_0[0 as libc::c_int as usize] = k;
        } else if mode == 'W' as i32 {
            let mut k_0: libc::c_int = 0;
            let mut nf: libc::c_int = 0;
            (*csv).fp = fopen((*csv).fname, b"w\0" as *const u8 as *const libc::c_char);
            if ((*csv).fp).is_null() {
                glp_printf(
                    b"csv_driver: unable to create %s - %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*csv).fname,
                    _glp_xstrerr(*__errno_location()),
                );
                longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            nf = _glp_mpl_tab_num_flds(dca);
            k_0 = 1 as libc::c_int;
            while k_0 <= nf {
                fprintf(
                    (*csv).fp,
                    b"%s%c\0" as *const u8 as *const libc::c_char,
                    _glp_mpl_tab_get_name(dca, k_0),
                    if k_0 < nf { ',' as i32 } else { '\n' as i32 },
                );
                k_0 += 1;
                k_0;
            }
            (*csv).count += 1;
            (*csv).count;
        } else {
            (mode != mode
                || {
                    glp_assert_(
                        b"mode != mode\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                        305 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        return csv;
    };
}
unsafe extern "C" fn csv_read_record(
    mut dca: *mut TABDCA,
    mut csv: *mut csv,
) -> libc::c_int {
    let mut current_block: u64;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    ((*csv).mode == 'R' as i32
        || {
            glp_assert_(
                b"csv->mode == 'R'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if _setjmp(((*csv).jump).as_mut_ptr()) != 0 {
        ret = 1 as libc::c_int;
    } else {
        if (*csv).ref_0[0 as libc::c_int as usize] > 0 as libc::c_int {
            _glp_mpl_tab_set_num(
                dca,
                (*csv).ref_0[0 as libc::c_int as usize],
                ((*csv).count - (*csv).nskip - 1 as libc::c_int) as libc::c_double,
            );
        }
        k = 1 as libc::c_int;
        loop {
            if !(k <= (*csv).nf) {
                current_block = 15768484401365413375;
                break;
            }
            read_field(csv);
            if (*csv).what == 0 as libc::c_int {
                (k == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"k == 1\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                            335 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                ret = -(1 as libc::c_int);
                current_block = 5089123208677676726;
                break;
            } else {
                if (*csv).what == 1 as libc::c_int {
                    let mut lack: libc::c_int = (*csv).nf - k + 1 as libc::c_int;
                    if lack == 1 as libc::c_int {
                        glp_printf(
                            b"%s:%d: one field missing\n\0" as *const u8
                                as *const libc::c_char,
                            (*csv).fname,
                            (*csv).count,
                        );
                    } else {
                        glp_printf(
                            b"%s:%d: %d fields missing\n\0" as *const u8
                                as *const libc::c_char,
                            (*csv).fname,
                            (*csv).count,
                            lack,
                        );
                    }
                    longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                } else if (*csv).what == 2 as libc::c_int {
                    if (*csv).ref_0[k as usize] > 0 as libc::c_int {
                        let mut num: libc::c_double = 0.;
                        (_glp_str2num(((*csv).field).as_mut_ptr(), &mut num)
                            == 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"str2num(csv->field, &num) == 0\0" as *const u8
                                        as *const libc::c_char,
                                    b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                                    354 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        _glp_mpl_tab_set_num(dca, (*csv).ref_0[k as usize], num);
                    }
                } else if (*csv).what == 3 as libc::c_int {
                    if (*csv).ref_0[k as usize] > 0 as libc::c_int {
                        _glp_mpl_tab_set_str(
                            dca,
                            (*csv).ref_0[k as usize],
                            ((*csv).field).as_mut_ptr(),
                        );
                    }
                } else {
                    (csv != csv
                        || {
                            glp_assert_(
                                b"csv != csv\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                                364 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                k += 1;
                k;
            }
        }
        match current_block {
            5089123208677676726 => {}
            _ => {
                read_field(csv);
                ((*csv).what != 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"csv->what != CSV_EOF\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                            368 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*csv).what != 1 as libc::c_int {
                    glp_printf(
                        b"%s:%d: too many fields\n\0" as *const u8
                            as *const libc::c_char,
                        (*csv).fname,
                        (*csv).count,
                    );
                    longjmp(((*csv).jump).as_mut_ptr(), 0 as libc::c_int);
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn csv_write_record(
    mut dca: *mut TABDCA,
    mut csv: *mut csv,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut nf: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    ((*csv).mode == 'W' as i32
        || {
            glp_assert_(
                b"csv->mode == 'W'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    nf = _glp_mpl_tab_num_flds(dca);
    k = 1 as libc::c_int;
    while k <= nf {
        match _glp_mpl_tab_get_type(dca, k) {
            78 => {
                fprintf(
                    (*csv).fp,
                    b"%.*g\0" as *const u8 as *const libc::c_char,
                    15 as libc::c_int,
                    _glp_mpl_tab_get_num(dca, k),
                );
            }
            83 => {
                fputc('"' as i32, (*csv).fp);
                c = _glp_mpl_tab_get_str(dca, k);
                while *c as libc::c_int != '\0' as i32 {
                    if *c as libc::c_int == '"' as i32 {
                        fputc('"' as i32, (*csv).fp);
                        fputc('"' as i32, (*csv).fp);
                    } else {
                        fputc(*c as libc::c_int, (*csv).fp);
                    }
                    c = c.offset(1);
                    c;
                }
                fputc('"' as i32, (*csv).fp);
            }
            _ => {
                (dca != dca
                    || {
                        glp_assert_(
                            b"dca != dca\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                            399 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        fputc(if k < nf { ',' as i32 } else { '\n' as i32 }, (*csv).fp);
        k += 1;
        k;
    }
    (*csv).count += 1;
    (*csv).count;
    if ferror((*csv).fp) != 0 {
        glp_printf(
            b"%s:%d: write error - %s\n\0" as *const u8 as *const libc::c_char,
            (*csv).fname,
            (*csv).count,
            _glp_xstrerr(*__errno_location()),
        );
        ret = 1 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn csv_close_file(
    mut dca: *mut TABDCA,
    mut csv: *mut csv,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    (dca == dca
        || {
            glp_assert_(
                b"dca == dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                419 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*csv).mode == 'W' as i32 {
        fflush((*csv).fp);
        if ferror((*csv).fp) != 0 {
            glp_printf(
                b"%s:%d: write error - %s\n\0" as *const u8 as *const libc::c_char,
                (*csv).fname,
                (*csv).count,
                _glp_xstrerr(*__errno_location()),
            );
            ret = 1 as libc::c_int;
        }
    }
    glp_free((*csv).fname as *mut libc::c_void);
    fclose((*csv).fp);
    glp_free(csv as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn read_byte(mut dbf: *mut dbf) -> libc::c_int {
    let mut b: libc::c_int = 0;
    b = fgetc((*dbf).fp);
    if ferror((*dbf).fp) != 0 {
        glp_printf(
            b"%s:0x%X: read error - %s\n\0" as *const u8 as *const libc::c_char,
            (*dbf).fname,
            (*dbf).offset,
            _glp_xstrerr(*__errno_location()),
        );
        longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
    }
    if feof((*dbf).fp) != 0 {
        glp_printf(
            b"%s:0x%X: unexpected end of file\n\0" as *const u8 as *const libc::c_char,
            (*dbf).fname,
            (*dbf).offset,
        );
        longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
    }
    (0 as libc::c_int <= b && b <= 0xff as libc::c_int
        || {
            glp_assert_(
                b"0x00 <= b && b <= 0xFF\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*dbf).offset += 1;
    (*dbf).offset;
    return b;
}
unsafe extern "C" fn read_header(mut dca: *mut TABDCA, mut dbf: *mut dbf) {
    let mut b: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut recl: libc::c_int = 0;
    let mut name: [libc::c_char; 11] = [0; 11];
    j = 1 as libc::c_int;
    while j <= 10 as libc::c_int {
        read_byte(dbf);
        j += 1;
        j;
    }
    recl = read_byte(dbf);
    recl += read_byte(dbf) << 8 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= 20 as libc::c_int {
        read_byte(dbf);
        j += 1;
        j;
    }
    ((*dbf).nf == 0 as libc::c_int
        || {
            glp_assert_(
                b"dbf->nf == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                511 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    loop {
        b = read_byte(dbf);
        if b == 0xd as libc::c_int {
            break;
        }
        if (*dbf).nf == 50 as libc::c_int {
            glp_printf(
                b"%s:0x%X: too many fields\n\0" as *const u8 as *const libc::c_char,
                (*dbf).fname,
                (*dbf).offset,
            );
            longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        (*dbf).nf += 1;
        (*dbf).nf;
        name[0 as libc::c_int as usize] = b as libc::c_char;
        j = 1 as libc::c_int;
        while j < 10 as libc::c_int {
            b = read_byte(dbf);
            name[j as usize] = b as libc::c_char;
            j += 1;
            j;
        }
        name[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        b = read_byte(dbf);
        if b != 0 as libc::c_int {
            glp_printf(
                b"%s:0x%X: invalid field name\n\0" as *const u8 as *const libc::c_char,
                (*dbf).fname,
                (*dbf).offset,
            );
            longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        k = _glp_mpl_tab_num_flds(dca);
        while k >= 1 as libc::c_int {
            if strcmp(_glp_mpl_tab_get_name(dca, k), name.as_mut_ptr())
                == 0 as libc::c_int
            {
                break;
            }
            k -= 1;
            k;
        }
        (*dbf).ref_0[(*dbf).nf as usize] = k;
        b = read_byte(dbf);
        if !(b == 'C' as i32 || b == 'N' as i32) {
            glp_printf(
                b"%s:0x%X: invalid field type\n\0" as *const u8 as *const libc::c_char,
                (*dbf).fname,
                (*dbf).offset,
            );
            longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        (*dbf).type_0[(*dbf).nf as usize] = b;
        j = 1 as libc::c_int;
        while j <= 4 as libc::c_int {
            read_byte(dbf);
            j += 1;
            j;
        }
        b = read_byte(dbf);
        if b == 0 as libc::c_int {
            glp_printf(
                b"%s:0x%X: invalid field length\n\0" as *const u8 as *const libc::c_char,
                (*dbf).fname,
                (*dbf).offset,
            );
            longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        if b > 100 as libc::c_int {
            glp_printf(
                b"%s:0x%X: field too long\n\0" as *const u8 as *const libc::c_char,
                (*dbf).fname,
                (*dbf).offset,
            );
            longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        (*dbf).len[(*dbf).nf as usize] = b;
        recl -= b;
        j = 1 as libc::c_int;
        while j <= 15 as libc::c_int {
            read_byte(dbf);
            j += 1;
            j;
        }
    }
    if recl != 1 as libc::c_int {
        glp_printf(
            b"%s:0x%X: invalid file header\n\0" as *const u8 as *const libc::c_char,
            (*dbf).fname,
            (*dbf).offset,
        );
        longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
    }
    k = _glp_mpl_tab_num_flds(dca);
    while k >= 1 as libc::c_int {
        if strcmp(
            _glp_mpl_tab_get_name(dca, k),
            b"RECNO\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            break;
        }
        k -= 1;
        k;
    }
    (*dbf).ref_0[0 as libc::c_int as usize] = k;
}
unsafe extern "C" fn parse_third_arg(mut dca: *mut TABDCA, mut dbf: *mut dbf) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    (*dbf).nf = _glp_mpl_tab_num_flds(dca);
    arg = _glp_mpl_tab_get_arg(dca, 3 as libc::c_int);
    j = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= (*dbf).nf {
        's_165: {
            if *arg.offset(j as isize) as libc::c_int == '\0' as i32 {
                glp_printf(
                    b"xBASE driver: field %s: specification missing\n\0" as *const u8
                        as *const libc::c_char,
                    _glp_mpl_tab_get_name(dca, k),
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            if *arg.offset(j as isize) as libc::c_int == 'C' as i32
                || *arg.offset(j as isize) as libc::c_int == 'N' as i32
            {
                (*dbf).type_0[k as usize] = *arg.offset(j as isize) as libc::c_int;
                j += 1;
                j;
            } else {
                glp_printf(
                    b"xBASE driver: field %s: invalid field type\n\0" as *const u8
                        as *const libc::c_char,
                    _glp_mpl_tab_get_name(dca, k),
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            if *arg.offset(j as isize) as libc::c_int == '(' as i32 {
                j += 1;
                j;
                temp = 0 as libc::c_int;
                while *(*__ctype_b_loc())
                    .offset(*arg.offset(j as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    if temp > 100 as libc::c_int {
                        break;
                    }
                    temp = 10 as libc::c_int * temp
                        + (*arg.offset(j as isize) as libc::c_int - '0' as i32);
                    j += 1;
                    j;
                }
                if !(1 as libc::c_int <= temp && temp <= 100 as libc::c_int) {
                    glp_printf(
                        b"xBASE driver: field %s: invalid field length\n\0" as *const u8
                            as *const libc::c_char,
                        _glp_mpl_tab_get_name(dca, k),
                    );
                    longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
                }
                (*dbf).len[k as usize] = temp;
                if (*dbf).type_0[k as usize] == 'N' as i32
                    && *arg.offset(j as isize) as libc::c_int == ',' as i32
                {
                    j += 1;
                    j;
                    temp = 0 as libc::c_int;
                    while *(*__ctype_b_loc())
                        .offset(*arg.offset(j as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        if temp > (*dbf).len[k as usize] {
                            break;
                        }
                        temp = 10 as libc::c_int * temp
                            + (*arg.offset(j as isize) as libc::c_int - '0' as i32);
                        j += 1;
                        j;
                    }
                    if temp > (*dbf).len[k as usize] {
                        glp_printf(
                            b"xBASE driver: field %s: invalid field precision\n\0"
                                as *const u8 as *const libc::c_char,
                            _glp_mpl_tab_get_name(dca, k),
                        );
                        longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
                    }
                    (*dbf).prec[k as usize] = temp;
                } else {
                    (*dbf).prec[k as usize] = 0 as libc::c_int;
                }
                if *arg.offset(j as isize) as libc::c_int == ')' as i32 {
                    j += 1;
                    j;
                    break 's_165;
                }
            }
            glp_printf(
                b"xBASE driver: field %s: invalid field format\n\0" as *const u8
                    as *const libc::c_char,
                _glp_mpl_tab_get_name(dca, k),
            );
            longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        k += 1;
        k;
    }
}
unsafe extern "C" fn write_byte(mut dbf: *mut dbf, mut b: libc::c_int) {
    fputc(b, (*dbf).fp);
    (*dbf).offset += 1;
    (*dbf).offset;
}
unsafe extern "C" fn write_header(mut dca: *mut TABDCA, mut dbf: *mut dbf) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    write_byte(dbf, 0x3 as libc::c_int);
    write_byte(dbf, 70 as libc::c_int);
    write_byte(dbf, 1 as libc::c_int);
    write_byte(dbf, 1 as libc::c_int);
    j = 1 as libc::c_int;
    while j <= 4 as libc::c_int {
        write_byte(dbf, 0xff as libc::c_int);
        j += 1;
        j;
    }
    temp = 32 as libc::c_int + (*dbf).nf * 32 as libc::c_int + 1 as libc::c_int;
    write_byte(dbf, temp);
    write_byte(dbf, temp >> 8 as libc::c_int);
    temp = 1 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= (*dbf).nf {
        temp += (*dbf).len[k as usize];
        k += 1;
        k;
    }
    write_byte(dbf, temp);
    write_byte(dbf, temp >> 8 as libc::c_int);
    j = 1 as libc::c_int;
    while j <= 20 as libc::c_int {
        write_byte(dbf, 0 as libc::c_int);
        j += 1;
        j;
    }
    k = 1 as libc::c_int;
    while k <= (*dbf).nf {
        name = _glp_mpl_tab_get_name(dca, k);
        j = 0 as libc::c_int;
        while j < 10 as libc::c_int
            && *name.offset(j as isize) as libc::c_int != '\0' as i32
        {
            write_byte(dbf, *name.offset(j as isize) as libc::c_int);
            j += 1;
            j;
        }
        j = j;
        while j < 11 as libc::c_int {
            write_byte(dbf, 0 as libc::c_int);
            j += 1;
            j;
        }
        write_byte(dbf, (*dbf).type_0[k as usize]);
        j = 1 as libc::c_int;
        while j <= 4 as libc::c_int {
            write_byte(dbf, 0 as libc::c_int);
            j += 1;
            j;
        }
        write_byte(dbf, (*dbf).len[k as usize]);
        write_byte(dbf, (*dbf).prec[k as usize]);
        j = 1 as libc::c_int;
        while j <= 14 as libc::c_int {
            write_byte(dbf, 0 as libc::c_int);
            j += 1;
            j;
        }
        k += 1;
        k;
    }
    write_byte(dbf, 0xd as libc::c_int);
}
unsafe extern "C" fn dbf_open_file(
    mut dca: *mut TABDCA,
    mut mode: libc::c_int,
) -> *mut dbf {
    let mut dbf: *mut dbf = 0 as *mut dbf;
    dbf = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<dbf>() as libc::c_ulong as libc::c_int,
    ) as *mut dbf;
    (*dbf).mode = mode;
    (*dbf).fname = 0 as *mut libc::c_char;
    (*dbf).fp = 0 as *mut FILE;
    if _setjmp(((*dbf).jump).as_mut_ptr()) != 0 {
        if !((*dbf).fname).is_null() {
            glp_free((*dbf).fname as *mut libc::c_void);
        }
        if !((*dbf).fp).is_null() {
            fclose((*dbf).fp);
        }
        glp_free(dbf as *mut libc::c_void);
        return 0 as *mut dbf;
    } else {
        (*dbf).offset = 0 as libc::c_int;
        (*dbf).count = 0 as libc::c_int;
        (*dbf).nf = 0 as libc::c_int;
        if _glp_mpl_tab_num_args(dca) < 2 as libc::c_int {
            glp_printf(
                b"xBASE driver: file name not specified\n\0" as *const u8
                    as *const libc::c_char,
            );
            longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
        }
        (*dbf)
            .fname = glp_alloc(
            1 as libc::c_int,
            (strlen(_glp_mpl_tab_get_arg(dca, 2 as libc::c_int)))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*dbf).fname, _glp_mpl_tab_get_arg(dca, 2 as libc::c_int));
        if mode == 'R' as i32 {
            (*dbf).fp = fopen((*dbf).fname, b"rb\0" as *const u8 as *const libc::c_char);
            if ((*dbf).fp).is_null() {
                glp_printf(
                    b"xBASE driver: unable to open %s - %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*dbf).fname,
                    _glp_xstrerr(*__errno_location()),
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            read_header(dca, dbf);
        } else if mode == 'W' as i32 {
            if _glp_mpl_tab_num_args(dca) < 3 as libc::c_int {
                glp_printf(
                    b"xBASE driver: file format not specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            parse_third_arg(dca, dbf);
            (*dbf).fp = fopen((*dbf).fname, b"wb\0" as *const u8 as *const libc::c_char);
            if ((*dbf).fp).is_null() {
                glp_printf(
                    b"xBASE driver: unable to create %s - %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*dbf).fname,
                    _glp_xstrerr(*__errno_location()),
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            write_header(dca, dbf);
        } else {
            (mode != mode
                || {
                    glp_assert_(
                        b"mode != mode\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                        760 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        return dbf;
    };
}
unsafe extern "C" fn dbf_read_record(
    mut dca: *mut TABDCA,
    mut dbf: *mut dbf,
) -> libc::c_int {
    let mut b: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 101] = [0; 101];
    ((*dbf).mode == 'R' as i32
        || {
            glp_assert_(
                b"dbf->mode == 'R'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                774 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if _setjmp(((*dbf).jump).as_mut_ptr()) != 0 {
        ret = 1 as libc::c_int;
    } else {
        b = read_byte(dbf);
        if b == 0x1a as libc::c_int {
            ret = -(1 as libc::c_int);
        } else {
            if b != 0x20 as libc::c_int {
                glp_printf(
                    b"%s:0x%X: invalid record flag\n\0" as *const u8
                        as *const libc::c_char,
                    (*dbf).fname,
                    (*dbf).offset,
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            if (*dbf).ref_0[0 as libc::c_int as usize] > 0 as libc::c_int {
                _glp_mpl_tab_set_num(
                    dca,
                    (*dbf).ref_0[0 as libc::c_int as usize],
                    ((*dbf).count + 1 as libc::c_int) as libc::c_double,
                );
            }
            k = 1 as libc::c_int;
            while k <= (*dbf).nf {
                j = 0 as libc::c_int;
                while j < (*dbf).len[k as usize] {
                    buf[j as usize] = read_byte(dbf) as libc::c_char;
                    j += 1;
                    j;
                }
                buf[(*dbf).len[k as usize] as usize] = '\0' as i32 as libc::c_char;
                if (*dbf).type_0[k as usize] == 'C' as i32 {
                    if (*dbf).ref_0[k as usize] > 0 as libc::c_int {
                        _glp_mpl_tab_set_str(
                            dca,
                            (*dbf).ref_0[k as usize],
                            _glp_strtrim(buf.as_mut_ptr()),
                        );
                    }
                } else if (*dbf).type_0[k as usize] == 'N' as i32 {
                    if (*dbf).ref_0[k as usize] > 0 as libc::c_int {
                        let mut num: libc::c_double = 0.;
                        _glp_strspx(buf.as_mut_ptr());
                        (_glp_str2num(buf.as_mut_ptr(), &mut num) == 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"str2num(buf, &num) == 0\0" as *const u8
                                        as *const libc::c_char,
                                    b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                                    811 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        _glp_mpl_tab_set_num(dca, (*dbf).ref_0[k as usize], num);
                    }
                } else {
                    (dbf != dbf
                        || {
                            glp_assert_(
                                b"dbf != dbf\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                                816 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                k += 1;
                k;
            }
            (*dbf).count += 1;
            (*dbf).count;
        }
    }
    return ret;
}
unsafe extern "C" fn dbf_write_record(
    mut dca: *mut TABDCA,
    mut dbf: *mut dbf,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ((*dbf).mode == 'W' as i32
        || {
            glp_assert_(
                b"dbf->mode == 'W'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                827 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if _setjmp(((*dbf).jump).as_mut_ptr()) != 0 {
        ret = 1 as libc::c_int;
    } else {
        write_byte(dbf, 0x20 as libc::c_int);
        ((*dbf).nf == _glp_mpl_tab_num_flds(dca)
            || {
                glp_assert_(
                    b"dbf->nf == mpl_tab_num_flds(dca)\0" as *const u8
                        as *const libc::c_char,
                    b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                    834 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = 1 as libc::c_int;
        while k <= (*dbf).nf {
            if (*dbf).type_0[k as usize] == 'C' as i32 {
                let mut str: *const libc::c_char = 0 as *const libc::c_char;
                if _glp_mpl_tab_get_type(dca, k) == 'N' as i32 {
                    sprintf(
                        buf.as_mut_ptr(),
                        b"%.*g\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        _glp_mpl_tab_get_num(dca, k),
                    );
                    str = buf.as_mut_ptr();
                } else if _glp_mpl_tab_get_type(dca, k) == 'S' as i32 {
                    str = _glp_mpl_tab_get_str(dca, k);
                } else {
                    (dca != dca
                        || {
                            glp_assert_(
                                b"dca != dca\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                                846 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                if strlen(str) as libc::c_int > (*dbf).len[k as usize] {
                    glp_printf(
                        b"xBASE driver: field %s: cannot convert %.15s... to field format\n\0"
                            as *const u8 as *const libc::c_char,
                        _glp_mpl_tab_get_name(dca, k),
                        str,
                    );
                    longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
                }
                j = 0 as libc::c_int;
                while j < (*dbf).len[k as usize]
                    && *str.offset(j as isize) as libc::c_int != '\0' as i32
                {
                    write_byte(dbf, *str.offset(j as isize) as libc::c_int);
                    j += 1;
                    j;
                }
                j = j;
                while j < (*dbf).len[k as usize] {
                    write_byte(dbf, ' ' as i32);
                    j += 1;
                    j;
                }
            } else if (*dbf).type_0[k as usize] == 'N' as i32 {
                's_156: {
                    let mut num: libc::c_double = _glp_mpl_tab_get_num(dca, k);
                    if !(fabs(num) > 1e20f64) {
                        sprintf(
                            buf.as_mut_ptr(),
                            b"%*.*f\0" as *const u8 as *const libc::c_char,
                            (*dbf).len[k as usize],
                            (*dbf).prec[k as usize],
                            num,
                        );
                        (strlen(buf.as_mut_ptr())
                            < ::std::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong
                            || {
                                glp_assert_(
                                    b"strlen(buf) < sizeof(buf)\0" as *const u8
                                        as *const libc::c_char,
                                    b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                                    866 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        if !(strlen(buf.as_mut_ptr()) as libc::c_int
                            != (*dbf).len[k as usize])
                        {
                            j = 0 as libc::c_int;
                            while j < (*dbf).len[k as usize] {
                                write_byte(dbf, buf[j as usize] as libc::c_int);
                                j += 1;
                                j;
                            }
                            break 's_156;
                        }
                    }
                    glp_printf(
                        b"xBASE driver: field %s: cannot convert %g to field format\n\0"
                            as *const u8 as *const libc::c_char,
                        _glp_mpl_tab_get_name(dca, k),
                        num,
                    );
                    longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
                }
            } else {
                (dbf != dbf
                    || {
                        glp_assert_(
                            b"dbf != dbf\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                            872 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            k += 1;
            k;
        }
        (*dbf).count += 1;
        (*dbf).count;
    }
    return ret;
}
unsafe extern "C" fn dbf_close_file(
    mut dca: *mut TABDCA,
    mut dbf: *mut dbf,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    (dca == dca
        || {
            glp_assert_(
                b"dca == dca\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                882 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*dbf).mode == 'W' as i32 {
        if _setjmp(((*dbf).jump).as_mut_ptr()) != 0 {
            ret = 1 as libc::c_int;
        } else {
            write_byte(dbf, 0x1a as libc::c_int);
            (*dbf).offset = 4 as libc::c_int;
            if fseek((*dbf).fp, (*dbf).offset as libc::c_long, 0 as libc::c_int) != 0 {
                glp_printf(
                    b"%s:0x%X: seek error - %s\n\0" as *const u8 as *const libc::c_char,
                    (*dbf).fname,
                    (*dbf).offset,
                    _glp_xstrerr(*__errno_location()),
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
            write_byte(dbf, (*dbf).count);
            write_byte(dbf, (*dbf).count >> 8 as libc::c_int);
            write_byte(dbf, (*dbf).count >> 16 as libc::c_int);
            write_byte(dbf, (*dbf).count >> 24 as libc::c_int);
            fflush((*dbf).fp);
            if ferror((*dbf).fp) != 0 {
                glp_printf(
                    b"%s:0x%X: write error - %s\n\0" as *const u8 as *const libc::c_char,
                    (*dbf).fname,
                    (*dbf).offset,
                    _glp_xstrerr(*__errno_location()),
                );
                longjmp(((*dbf).jump).as_mut_ptr(), 0 as libc::c_int);
            }
        }
    }
    glp_free((*dbf).fname as *mut libc::c_void);
    fclose((*dbf).fp);
    glp_free(dbf as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn _glp_mpl_tab_drv_open(
    mut mpl: *mut MPL,
    mut mode: libc::c_int,
) {
    let mut dca: *mut TABDCA = (*mpl).dca;
    ((*dca).id == 0 as libc::c_int
        || {
            glp_assert_(
                b"dca->id == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                932 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (((*dca).link).is_null()
        || {
            glp_assert_(
                b"dca->link == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                933 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*dca).na >= 1 as libc::c_int
        || {
            glp_assert_(
                b"dca->na >= 1\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                934 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if strcmp(
        *((*dca).arg).offset(1 as libc::c_int as isize),
        b"CSV\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*dca).id = 1 as libc::c_int;
        (*dca).link = csv_open_file(dca, mode) as *mut libc::c_void;
    } else if strcmp(
        *((*dca).arg).offset(1 as libc::c_int as isize),
        b"xBASE\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*dca).id = 2 as libc::c_int;
        (*dca).link = dbf_open_file(dca, mode) as *mut libc::c_void;
    } else if strcmp(
        *((*dca).arg).offset(1 as libc::c_int as isize),
        b"ODBC\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            *((*dca).arg).offset(1 as libc::c_int as isize),
            b"iODBC\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        (*dca).id = 3 as libc::c_int;
        (*dca).link = _glp_db_iodbc_open(dca, mode);
    } else if strcmp(
        *((*dca).arg).offset(1 as libc::c_int as isize),
        b"MySQL\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*dca).id = 4 as libc::c_int;
        (*dca).link = _glp_db_mysql_open(dca, mode);
    } else {
        glp_printf(
            b"Invalid table driver '%s'\n\0" as *const u8 as *const libc::c_char,
            *((*dca).arg).offset(1 as libc::c_int as isize),
        );
    }
    if ((*dca).link).is_null() {
        _glp_mpl_error(
            mpl,
            b"error on opening table %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*(*(*mpl).stmt).u.tab).name,
        );
    }
}
pub unsafe extern "C" fn _glp_mpl_tab_drv_read(mut mpl: *mut MPL) -> libc::c_int {
    let mut dca: *mut TABDCA = (*mpl).dca;
    let mut ret: libc::c_int = 0;
    match (*dca).id {
        1 => {
            ret = csv_read_record(dca, (*dca).link as *mut csv);
        }
        2 => {
            ret = dbf_read_record(dca, (*dca).link as *mut dbf);
        }
        3 => {
            ret = _glp_db_iodbc_read(dca, (*dca).link);
        }
        4 => {
            ret = _glp_db_mysql_read(dca, (*dca).link);
        }
        _ => {
            (dca != dca
                || {
                    glp_assert_(
                        b"dca != dca\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                        977 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if ret > 0 as libc::c_int {
        _glp_mpl_error(
            mpl,
            b"error on reading data from table %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*(*(*mpl).stmt).u.tab).name,
        );
    }
    return ret;
}
pub unsafe extern "C" fn _glp_mpl_tab_drv_write(mut mpl: *mut MPL) {
    let mut dca: *mut TABDCA = (*mpl).dca;
    let mut ret: libc::c_int = 0;
    match (*dca).id {
        1 => {
            ret = csv_write_record(dca, (*dca).link as *mut csv);
        }
        2 => {
            ret = dbf_write_record(dca, (*dca).link as *mut dbf);
        }
        3 => {
            ret = _glp_db_iodbc_write(dca, (*dca).link);
        }
        4 => {
            ret = _glp_db_mysql_write(dca, (*dca).link);
        }
        _ => {
            (dca != dca
                || {
                    glp_assert_(
                        b"dca != dca\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                        1002 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if ret != 0 {
        _glp_mpl_error(
            mpl,
            b"error on writing data to table %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*(*(*mpl).stmt).u.tab).name,
        );
    }
}
pub unsafe extern "C" fn _glp_mpl_tab_drv_close(mut mpl: *mut MPL) {
    let mut dca: *mut TABDCA = (*mpl).dca;
    let mut ret: libc::c_int = 0;
    match (*dca).id {
        1 => {
            ret = csv_close_file(dca, (*dca).link as *mut csv);
        }
        2 => {
            ret = dbf_close_file(dca, (*dca).link as *mut dbf);
        }
        3 => {
            ret = _glp_db_iodbc_close(dca, (*dca).link);
        }
        4 => {
            ret = _glp_db_mysql_close(dca, (*dca).link);
        }
        _ => {
            (dca != dca
                || {
                    glp_assert_(
                        b"dca != dca\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl6.c\0" as *const u8 as *const libc::c_char,
                        1027 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (*dca).id = 0 as libc::c_int;
    (*dca).link = 0 as *mut libc::c_void;
    if ret != 0 {
        _glp_mpl_error(
            mpl,
            b"error on closing table %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*(*(*mpl).stmt).u.tab).name,
        );
    }
}
