use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    static mut pl_m_architecture: [libc::c_char; 0];
    static mut pl_m_os_version: [libc::c_char; 0];
    fn Pl_M_Create_Shell_Command(cmd: *mut libc::c_char) -> *mut *mut libc::c_char;
    fn Pl_M_Shell(cmd: *mut libc::c_char) -> libc::c_int;
    fn Pl_M_Spawn(arg: *mut *mut libc::c_char) -> libc::c_int;
    fn Pl_M_Spawn_Redirect(
        arg: *mut *mut libc::c_char,
        detach: libc::c_int,
        f_in: *mut *mut FILE,
        f_out: *mut *mut FILE,
        f_err: *mut *mut FILE,
    ) -> libc::c_int;
    fn Pl_M_Get_Status(pid: libc::c_int) -> libc::c_int;
    fn Pl_M_Mktemp(tmp_template: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_M_Tempnam(dir: *mut libc::c_char, pfx: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_M_Host_Name_From_Name(host_name: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_M_Get_Working_Dir() -> *mut libc::c_char;
    fn Pl_M_Set_Working_Dir(path: *mut libc::c_char) -> Bool;
    fn Pl_M_Absolute_Path_Name(src: *mut libc::c_char) -> *mut libc::c_char;
    static mut pl_glob_buff: [libc::c_char; 0];
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Check_For_Un_List(start_word: WamWord);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Positive_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_atom_read: libc::c_int;
    static mut pl_atom_write: libc::c_int;
    fn Pl_Add_Stream_For_Stdio_Desc(
        f: *mut FILE,
        atom_path: libc::c_int,
        mode: libc::c_int,
        text: Bool,
        force_eof_reset: Bool,
    ) -> libc::c_int;
    fn Pl_Flush_All_Streams();
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Io_Fileno_Of_Stream(stm: libc::c_int) -> libc::c_int;
    fn Pl_Rd_C_Int_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_C_Int_Positive_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Number_Check(start_word: WamWord) -> libc::c_double;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    static mut pl_sys_var: [PlLong; 0];
    static mut pl_type_compound: libc::c_int;
    static mut pl_type_list: libc::c_int;
    static mut pl_domain_io_mode: libc::c_int;
    static mut pl_domain_not_less_than_zero: libc::c_int;
    static mut pl_domain_date_time: libc::c_int;
    static mut pl_domain_selectable_item: libc::c_int;
    static mut pl_domain_os_file_permission: libc::c_int;
    static mut pl_domain_os_path: libc::c_int;
    fn Pl_Os_Error(ret_val: libc::c_int);
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Err_System(pl_atom_error: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AtomProp {
    #[bitfield(name = "length", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "op_mask", ty = "libc::c_uint", bits = "16..=19")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "20..=21")]
    #[bitfield(name = "needs_quote", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "needs_scan", ty = "libc::c_uint", bits = "23..=23")]
    pub length_op_mask_type_0_needs_quote_needs_scan: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AtomInf {
    pub name: *mut libc::c_char,
    pub hash: libc::c_uint,
    pub prop: AtomProp,
    pub info: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfSig {
    pub atom: libc::c_int,
    pub sig: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct StmProp {
    #[bitfield(name = "mode", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "input", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "output", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "text", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "reposition", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "eof_action", ty = "libc::c_uint", bits = "6..=7")]
    #[bitfield(name = "buffering", ty = "libc::c_uint", bits = "8..=9")]
    #[bitfield(name = "special_close", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "other", ty = "libc::c_uint", bits = "11..=15")]
    pub mode_input_output_text_reposition_eof_action_buffering_special_close_other: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PbStk {
    pub buff: [libc::c_int; 8],
    pub ptr: *mut libc::c_int,
    pub nb_elems: libc::c_int,
}
pub type StmFct = Option::<unsafe extern "C" fn() -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_lst {
    pub stm: libc::c_int,
    pub next: PStmLst,
}
pub type PStmLst = *mut stm_lst;
pub type StmLst = stm_lst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_inf {
    pub atom_file_name: libc::c_int,
    pub file: PlLong,
    pub prop: StmProp,
    pub mirror: *mut StmLst,
    pub mirror_of: *mut StmLst,
    pub fct_getc: StmFct,
    pub fct_putc: StmFct,
    pub fct_flush: StmFct,
    pub fct_close: StmFct,
    pub fct_tell: StmFct,
    pub fct_seek: StmFct,
    pub fct_clearerr: StmFct,
    pub eof_reached: Bool,
    pub pb_char: PbStk,
    pub char_count: PlLong,
    pub line_count: PlLong,
    pub line_pos: PlLong,
    pub pb_line_pos: PbStk,
}
pub type StmInf = stm_inf;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut atom_dt: libc::c_int = 0;
static mut atom_execute: libc::c_int = 0;
static mut atom_search: libc::c_int = 0;
static mut atom_regular: libc::c_int = 0;
static mut atom_directory: libc::c_int = 0;
static mut atom_fifo: libc::c_int = 0;
static mut atom_socket: libc::c_int = 0;
static mut atom_character_device: libc::c_int = 0;
static mut atom_block_device: libc::c_int = 0;
static mut atom_unknown: libc::c_int = 0;
static mut tsig: [InfSig; 255] = [InfSig { atom: 0, sig: 0 }; 255];
static mut nb_sig: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Make_Directory_1(mut path_name_word: WamWord) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    let mut _tst: libc::c_int = mkdir(path_name, 0o777 as libc::c_int as __mode_t);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Delete_Directory_1(mut path_name_word: WamWord) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    let mut _tst: libc::c_int = rmdir(path_name);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Working_Directory_1(mut path_name_word: WamWord) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Pl_M_Get_Working_Dir();
    return Pl_Un_String_Check(path_name, path_name_word);
}
pub unsafe extern "C" fn Pl_Change_Directory_1(mut path_name_word: WamWord) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    *__errno_location() = -(1 as libc::c_int);
    if Pl_M_Set_Working_Dir(path_name) == 0 {
        let mut _tst: libc::c_int = -(1 as libc::c_int);
        if _tst < 0 as libc::c_int {
            Pl_Os_Error(_tst);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Directory_Files_2(
    mut path_name_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: Bool = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut cur_entry: *mut dirent = 0 as *mut dirent;
    Pl_Check_For_Un_List(list_word);
    path_name = Get_Path_Name(path_name_word);
    dir = opendir(path_name);
    if dir.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    loop {
        cur_entry = readdir(dir);
        if cur_entry.is_null() {
            current_block = 11050875288958768710;
            break;
        }
        name = ((*cur_entry).d_name).as_mut_ptr();
        if Pl_Get_List(list_word) == 0
            || Pl_Unify_Atom(Pl_Create_Allocate_Atom(name)) == 0
        {
            res = 0 as libc::c_int;
            current_block = 12715564572477067435;
            break;
        } else {
            list_word = Pl_Unify_Variable();
        }
    }
    match current_block {
        11050875288958768710 => {
            res = Pl_Get_Nil(list_word);
        }
        _ => {}
    }
    closedir(dir);
    return res;
}
pub unsafe extern "C" fn Pl_Rename_File_2(
    mut path_name1_word: WamWord,
    mut path_name2_word: WamWord,
) -> Bool {
    let mut path_name1: [libc::c_char; 4096] = [0; 4096];
    let mut path_name2: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(path_name1.as_mut_ptr(), Get_Path_Name(path_name1_word));
    path_name2 = Get_Path_Name(path_name2_word);
    let mut _tst: libc::c_int = rename(path_name1.as_mut_ptr(), path_name2);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unlink_1(mut path_name_word: WamWord) {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    unlink(path_name);
}
pub unsafe extern "C" fn Pl_Delete_File_1(mut path_name_word: WamWord) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    let mut _tst: libc::c_int = unlink(path_name);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_File_Exists_1(mut path_name_word: WamWord) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    if access(path_name, 0 as libc::c_int) != 0 {
        if *__errno_location() == 2 as libc::c_int
            || *__errno_location() == 20 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        let mut _tst: libc::c_int = -(1 as libc::c_int);
        if _tst < 0 as libc::c_int {
            Pl_Os_Error(_tst);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_File_Permission_2(
    mut path_name_word: WamWord,
    mut perm_list_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_perm_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode: libc::c_int = 0;
    let mut perm: libc::c_int = 0 as libc::c_int;
    let mut file_info: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut res: libc::c_int = 0;
    let mut is_a_directory: Bool = 0;
    path_name = Get_Path_Name(path_name_word);
    res = stat(path_name, &mut file_info);
    if res == -(1 as libc::c_int) && *__errno_location() != 2 as libc::c_int
        && *__errno_location() != 20 as libc::c_int
    {
        let mut _tst: libc::c_int = -(1 as libc::c_int);
        if _tst < 0 as libc::c_int {
            Pl_Os_Error(_tst);
            return 0 as libc::c_int;
        }
    }
    mode = file_info.st_mode as libc::c_int;
    is_a_directory = (res == 0 as libc::c_int
        && mode & 0o170000 as libc::c_int == 0o40000 as libc::c_int) as libc::c_int;
    let mut deref_last_word: WamWord = 0;
    word = perm_list_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && word as libc::c_ulong
            != (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
    {
        perm |= Flag_Of_Permission(word, is_a_directory);
    } else {
        save_perm_list_word = perm_list_word;
        loop {
            let mut deref_last_word_0: WamWord = 0;
            word = perm_list_word;
            loop {
                deref_last_word_0 = word;
                tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                    as WamWord;
                if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                    break;
                }
                word = *(word as *mut WamWord);
                if !(word != deref_last_word_0) {
                    break;
                }
            }
            if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                Pl_Err_Instantiation();
            }
            if word as libc::c_ulong
                == (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong)
            {
                break;
            }
            if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
                Pl_Err_Type(pl_type_list, save_perm_list_word);
            }
            lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            perm
                |= Flag_Of_Permission(
                    *lst_adr.offset(0 as libc::c_int as isize),
                    is_a_directory,
                );
            perm_list_word = *lst_adr
                .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        }
    }
    return (res == 0 as libc::c_int && perm > 0 as libc::c_int && mode | perm == mode)
        as libc::c_int;
}
unsafe extern "C" fn Flag_Of_Permission(
    mut perm_word: WamWord,
    mut is_a_directory: Bool,
) -> libc::c_int {
    let mut atom: libc::c_int = 0;
    atom = Pl_Rd_Atom_Check(perm_word);
    if atom == pl_atom_read {
        return 0o400 as libc::c_int;
    }
    if atom == pl_atom_write {
        return 0o200 as libc::c_int;
    }
    if atom == atom_execute {
        return if is_a_directory != 0 {
            -(1 as libc::c_int)
        } else {
            0o100 as libc::c_int
        };
    }
    if atom == atom_search {
        return if is_a_directory != 0 {
            0o100 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    Pl_Err_Domain(pl_domain_os_file_permission, perm_word);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_File_Prop_Absolute_File_Name_2(
    mut absolute_path_name_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    let mut _tst: libc::c_int = access(path_name, 0 as libc::c_int);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return Pl_Un_String_Check(path_name, absolute_path_name_word);
}
pub unsafe extern "C" fn Pl_File_Prop_Real_File_Name_2(
    mut real_path_name_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut path_name: *mut libc::c_char = Get_Path_Name(path_name_word);
    let mut real_path_name: [libc::c_char; 4096] = [0; 4096];
    if (realpath(path_name, real_path_name.as_mut_ptr())).is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    return Pl_Un_String_Check(real_path_name.as_mut_ptr(), real_path_name_word);
}
pub unsafe extern "C" fn Pl_File_Prop_Type_2(
    mut type_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_info: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut atom: libc::c_int = 0;
    path_name = Get_Path_Name(path_name_word);
    let mut _tst: libc::c_int = stat(path_name, &mut file_info);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    if file_info.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        atom = atom_regular;
    } else if file_info.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        atom = atom_directory;
    } else if file_info.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        atom = atom_fifo;
    } else if file_info.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint
    {
        atom = atom_socket;
    } else if file_info.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        atom = atom_character_device;
    } else if file_info.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        atom = atom_block_device;
    } else {
        atom = atom_unknown;
    }
    return Pl_Un_Atom_Check(atom, type_word);
}
pub unsafe extern "C" fn Pl_File_Prop_Size_2(
    mut size_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_info: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    path_name = Get_Path_Name(path_name_word);
    let mut _tst: libc::c_int = stat(path_name, &mut file_info);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return Pl_Un_Positive_Check(file_info.st_size as libc::c_int as PlLong, size_word);
}
pub unsafe extern "C" fn Pl_Check_Prop_Perm_And_File_2(
    mut perm_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Get_Path_Name(path_name_word);
    let mut deref_last_word: WamWord = 0;
    word = perm_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        Flag_Of_Permission(perm_word, 0 as libc::c_int);
    }
    let mut _tst: libc::c_int = access(path_name, 0 as libc::c_int);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_File_Prop_Date_2(
    mut date_time_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_info: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut t: *mut time_t = 0 as *mut time_t;
    path_name = Get_Path_Name(path_name_word);
    let mut _tst: libc::c_int = stat(path_name, &mut file_info);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    match *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize) {
        0 => {
            t = &mut file_info.st_ctim.tv_sec;
        }
        1 => {
            t = &mut file_info.st_atim.tv_sec;
        }
        _ => {
            t = &mut file_info.st_mtim.tv_sec;
        }
    }
    return Date_Time_To_Prolog(t, date_time_word);
}
pub unsafe extern "C" fn Pl_Temporary_Name_2(
    mut template_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut template: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    template = Get_Path_Name(template_word);
    path_name = Pl_M_Mktemp(template);
    if path_name.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    return (!path_name.is_null() && Pl_Un_String_Check(path_name, path_name_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Temporary_File_3(
    mut dir_word: WamWord,
    mut prefix_word: WamWord,
    mut path_name_word: WamWord,
) -> Bool {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    dir = Pl_Rd_String_Check(dir_word);
    if *dir as libc::c_int == '\0' as i32 {
        dir = 0 as *mut libc::c_char;
    } else {
        dir = Get_Path_Name(dir_word);
    }
    prefix = Pl_Rd_String_Check(prefix_word);
    if *prefix as libc::c_int == '\0' as i32 {
        prefix = 0 as *mut libc::c_char;
    }
    path_name = Pl_M_Tempnam(dir, prefix);
    if path_name.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    return (!path_name.is_null() && Pl_Un_String_Check(path_name, path_name_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Date_Time_1(mut date_time_word: WamWord) -> Bool {
    let mut t: time_t = 0;
    t = time(0 as *mut time_t);
    return Date_Time_To_Prolog(&mut t, date_time_word);
}
pub unsafe extern "C" fn Pl_Host_Name_1(mut host_name_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    static mut atom_host_name: libc::c_int = -(1 as libc::c_int);
    if atom_host_name < 0 as libc::c_int {
        atom_host_name = Pl_Create_Allocate_Atom(
            Pl_M_Host_Name_From_Name(0 as *mut libc::c_char),
        );
    }
    let mut deref_last_word: WamWord = 0;
    word = host_name_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        return Pl_Get_Atom(atom_host_name, host_name_word);
    }
    atom = Pl_Rd_Atom_Check(word);
    return (atom == atom_host_name
        || strcmp(
            Pl_M_Host_Name_From_Name((*pl_atom_tbl.offset(atom as isize)).name),
            (*pl_atom_tbl.offset(atom_host_name as isize)).name,
        ) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Os_Version_1(mut os_version_word: WamWord) -> Bool {
    return Pl_Un_String_Check(pl_m_os_version.as_mut_ptr(), os_version_word);
}
pub unsafe extern "C" fn Pl_Architecture_1(mut architecture_word: WamWord) -> Bool {
    return Pl_Un_String_Check(pl_m_architecture.as_mut_ptr(), architecture_word);
}
pub unsafe extern "C" fn Pl_Sleep_1(mut seconds_word: WamWord) {
    let mut us: PlLong = 0;
    us = (Pl_Rd_Number_Check(seconds_word) * 1000000 as libc::c_int as libc::c_double)
        as PlLong;
    if us < 0 as libc::c_int as libc::c_long {
        Pl_Err_Domain(pl_domain_not_less_than_zero, seconds_word);
    }
    usleep(us as __useconds_t);
}
pub unsafe extern "C" fn Pl_Shell_2(
    mut cmd_word: WamWord,
    mut status_word: WamWord,
) -> Bool {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: libc::c_int = 0;
    cmd = Pl_Rd_String_Check(cmd_word);
    if *cmd as libc::c_int == '\0' as i32 {
        cmd = 0 as *mut libc::c_char;
    }
    Pl_Check_For_Un_Integer(status_word);
    Pl_Flush_All_Streams();
    status = Pl_M_Shell(cmd);
    return Pl_Get_Integer(status as PlLong, status_word);
}
pub unsafe extern "C" fn Pl_System_2(
    mut cmd_word: WamWord,
    mut status_word: WamWord,
) -> Bool {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: libc::c_int = 0;
    cmd = Pl_Rd_String_Check(cmd_word);
    Pl_Check_For_Un_Integer(status_word);
    Pl_Flush_All_Streams();
    status = system(cmd);
    return Pl_Get_Integer(status as PlLong, status_word);
}
pub unsafe extern "C" fn Pl_Spawn_3(
    mut cmd_word: WamWord,
    mut list_word: WamWord,
    mut status_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut arg: [*mut libc::c_char; 1024] = [0 as *mut libc::c_char; 1024];
    let mut p: *mut *mut libc::c_char = arg.as_mut_ptr();
    let mut err: [libc::c_char; 64] = [0; 64];
    let mut status: libc::c_int = 0;
    save_list_word = list_word;
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = Pl_Rd_String_Check(cmd_word);
    loop {
        let mut deref_last_word: WamWord = 0;
        word = list_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, save_list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = Pl_Rd_String_Check(*lst_adr.offset(0 as libc::c_int as isize));
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *p = 0 as *mut libc::c_char;
    Pl_Check_For_Un_Integer(status_word);
    Pl_Flush_All_Streams();
    status = Pl_M_Spawn(arg.as_mut_ptr());
    if status == -(1 as libc::c_int) {
        let mut _tst: libc::c_int = status;
        if _tst < 0 as libc::c_int {
            Pl_Os_Error(_tst);
            return 0 as libc::c_int;
        }
    } else if status == -(2 as libc::c_int) {
        sprintf(
            err.as_mut_ptr(),
            b"error trying to execute %s\0" as *const u8 as *const libc::c_char,
            arg[0 as libc::c_int as usize],
        );
        Pl_Err_System(Pl_Create_Allocate_Atom(err.as_mut_ptr()));
        return 0 as libc::c_int;
    }
    return Pl_Get_Integer(status as PlLong, status_word);
}
pub unsafe extern "C" fn Pl_Popen_3(
    mut cmd_word: WamWord,
    mut mode_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut atom: libc::c_int = 0;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut stm: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut open_str: [libc::c_char; 10] = [0; 10];
    cmd = Pl_Rd_String_Check(cmd_word);
    atom = Pl_Rd_Atom_Check(mode_word);
    if atom == pl_atom_read {
        mode = 0 as libc::c_int;
        strcpy(open_str.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    } else if atom == pl_atom_write {
        mode = 1 as libc::c_int;
        strcpy(open_str.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    } else {
        Pl_Err_Domain(pl_domain_io_mode, mode_word);
    }
    Pl_Flush_All_Streams();
    f = popen(cmd, open_str.as_mut_ptr());
    if f.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    sprintf(
        pl_glob_buff.as_mut_ptr(),
        b"popen_stream('%.1024s')\0" as *const u8 as *const libc::c_char,
        cmd,
    );
    atom = Pl_Create_Allocate_Atom(pl_glob_buff.as_mut_ptr());
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f,
        atom,
        mode,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let ref mut fresh2 = (**pl_stm_tbl.offset(stm as isize)).fct_close;
    *fresh2 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
        StmFct,
    >(Some(pclose as unsafe extern "C" fn(*mut FILE) -> libc::c_int));
    return Pl_Get_Integer(stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Exec_5(
    mut cmd_word: WamWord,
    mut stm_in_word: WamWord,
    mut stm_out_word: WamWord,
    mut stm_err_word: WamWord,
    mut pid_word: WamWord,
) -> Bool {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut stm: libc::c_int = 0;
    let mut f_in: *mut FILE = 0 as *mut FILE;
    let mut f_out: *mut FILE = 0 as *mut FILE;
    let mut f_err: *mut FILE = 0 as *mut FILE;
    let mut pid: libc::c_int = 0;
    let mut mask: libc::c_int = *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as libc::c_int;
    let mut atom: libc::c_int = 0;
    let mut err: [libc::c_char; 1024] = [0; 1024];
    cmd = Pl_Rd_String_Check(cmd_word);
    arg = Pl_M_Create_Shell_Command(cmd);
    Pl_Flush_All_Streams();
    pid = Pl_M_Spawn_Redirect(
        arg,
        (mask & 1 as libc::c_int == 0 as libc::c_int) as libc::c_int,
        &mut f_in,
        &mut f_out,
        &mut f_err,
    );
    if pid == -(1 as libc::c_int) && *__errno_location() != 2 as libc::c_int {
        let mut _tst: libc::c_int = pid;
        if _tst < 0 as libc::c_int {
            Pl_Os_Error(_tst);
            return 0 as libc::c_int;
        }
    }
    if pid < 0 as libc::c_int {
        sprintf(
            err.as_mut_ptr(),
            b"error trying to execute %s (maybe not found)\0" as *const u8
                as *const libc::c_char,
            cmd,
        );
        Pl_Err_System(Pl_Create_Allocate_Atom(err.as_mut_ptr()));
        return 0 as libc::c_int;
    }
    if mask & 1 as libc::c_int != 0 {
        Pl_Get_Integer(pid as PlLong, pid_word);
    }
    sprintf(
        pl_glob_buff.as_mut_ptr(),
        b"exec_stream('%.1024s')\0" as *const u8 as *const libc::c_char,
        cmd,
    );
    atom = Pl_Create_Allocate_Atom(pl_glob_buff.as_mut_ptr());
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f_in,
        atom,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    Pl_Get_Integer(stm as PlLong, stm_in_word);
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f_out,
        atom,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let ref mut fresh3 = (**pl_stm_tbl.offset(stm as isize)).prop;
    (*fresh3).set_eof_action(2 as libc::c_int as libc::c_uint);
    Pl_Get_Integer(stm as PlLong, stm_out_word);
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f_err,
        atom,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let ref mut fresh4 = (**pl_stm_tbl.offset(stm as isize)).prop;
    (*fresh4).set_eof_action(2 as libc::c_int as libc::c_uint);
    Pl_Get_Integer(stm as PlLong, stm_err_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Create_Pipe_2(
    mut stm_in_word: WamWord,
    mut stm_out_word: WamWord,
) -> Bool {
    let mut p: [libc::c_int; 2] = [0; 2];
    let mut stm: libc::c_int = 0;
    let mut f_in: *mut FILE = 0 as *mut FILE;
    let mut f_out: *mut FILE = 0 as *mut FILE;
    let mut atom: libc::c_int = 0;
    let mut _tst: libc::c_int = pipe(p.as_mut_ptr());
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    f_in = fdopen(
        p[0 as libc::c_int as usize],
        b"rt\0" as *const u8 as *const libc::c_char,
    );
    if f_in.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    sprintf(
        pl_glob_buff.as_mut_ptr(),
        b"pipe_stream_in\0" as *const u8 as *const libc::c_char,
    );
    atom = Pl_Create_Allocate_Atom(pl_glob_buff.as_mut_ptr());
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f_in,
        atom,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let ref mut fresh5 = (**pl_stm_tbl.offset(stm as isize)).prop;
    (*fresh5).set_eof_action(2 as libc::c_int as libc::c_uint);
    Pl_Get_Integer(stm as PlLong, stm_in_word);
    f_out = fdopen(
        p[1 as libc::c_int as usize],
        b"wt\0" as *const u8 as *const libc::c_char,
    );
    if f_out.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    sprintf(
        pl_glob_buff.as_mut_ptr(),
        b"pipe_stream_out\0" as *const u8 as *const libc::c_char,
    );
    atom = Pl_Create_Allocate_Atom(pl_glob_buff.as_mut_ptr());
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f_out,
        atom,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    Pl_Get_Integer(stm as PlLong, stm_out_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fork_Prolog_1(mut pid_word: WamWord) -> Bool {
    let mut pid: libc::c_int = 0;
    pid = fork();
    let mut _tst: libc::c_int = pid;
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return Pl_Get_Integer(pid as PlLong, pid_word);
}
pub unsafe extern "C" fn Pl_Select_5(
    mut reads_word: WamWord,
    mut ready_reads_word: WamWord,
    mut writes_word: WamWord,
    mut ready_writes_word: WamWord,
    mut time_out_word: WamWord,
) -> Bool {
    let mut time_out: libc::c_double = 0.;
    let mut p: *mut timeval = 0 as *mut timeval;
    let mut t: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut read_set: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut write_set: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut max: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    max = Select_Init_Set(reads_word, &mut read_set, 2 as libc::c_int);
    Pl_Check_For_Un_List(ready_reads_word);
    n = Select_Init_Set(writes_word, &mut write_set, 3 as libc::c_int);
    if n > max {
        max = n;
    }
    Pl_Check_For_Un_List(ready_writes_word);
    time_out = Pl_Rd_Number_Check(time_out_word);
    if time_out <= 0 as libc::c_int as libc::c_double {
        p = 0 as *mut timeval;
    } else {
        t.tv_sec = (time_out / 1000 as libc::c_int as libc::c_double) as PlLong;
        t
            .tv_usec = (fmod(time_out, 1000 as libc::c_int as libc::c_double)
            * 1000 as libc::c_int as libc::c_double) as PlLong;
        p = &mut t;
    }
    let mut _tst: libc::c_int = select(
        max + 1 as libc::c_int,
        &mut read_set,
        &mut write_set,
        0 as *mut fd_set,
        p,
    );
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return (Select_Init_Ready_List(reads_word, &mut read_set, ready_reads_word) != 0
        && Select_Init_Ready_List(writes_word, &mut write_set, ready_writes_word) != 0)
        as libc::c_int;
}
unsafe extern "C" fn Select_Init_Set(
    mut list_word: WamWord,
    mut set: *mut fd_set,
    mut check: libc::c_int,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut stm: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh6 = &mut __d0;
    let fresh7;
    let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh9 = &mut __d1;
    let fresh10;
    let fresh11 = &mut *((*set).__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh6,
        fresh8) => fresh7, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9,
        fresh11) => fresh10, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    save_list_word = list_word;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = list_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, save_list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_0: WamWord = 0;
        word = *lst_adr.offset(0 as libc::c_int as isize);
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            fd = Pl_Rd_C_Int_Positive_Check(word);
        } else {
            stm = Pl_Get_Stream_Or_Alias(word, check);
            fd = Pl_Io_Fileno_Of_Stream(stm);
            if fd < 0 as libc::c_int {
                Pl_Err_Domain(pl_domain_selectable_item, word);
            }
        }
        if fd >= 1024 as libc::c_int {
            *__errno_location() = 9 as libc::c_int;
            let mut _tst: libc::c_int = -(1 as libc::c_int);
            if _tst < 0 as libc::c_int {
                Pl_Os_Error(_tst);
                return 0 as libc::c_int;
            }
        }
        (*set)
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        if fd > max {
            max = fd;
        }
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return max;
}
unsafe extern "C" fn Select_Init_Ready_List(
    mut list_word: WamWord,
    mut set: *mut fd_set,
    mut ready_list_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut stm: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = list_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_0: WamWord = 0;
        word = *lst_adr.offset(0 as libc::c_int as isize);
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            fd = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
        } else {
            stm = Pl_Get_Stream_Or_Alias(word, 0 as libc::c_int);
            fd = if stm < 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                Pl_Io_Fileno_Of_Stream(stm)
            };
        }
        if (*set)
            .__fds_bits[(fd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << fd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask
            != 0 as libc::c_int as libc::c_long
        {
            if Pl_Get_List(ready_list_word) == 0 || Pl_Unify_Value(word) == 0 {
                return 0 as libc::c_int;
            }
            ready_list_word = Pl_Unify_Variable();
        }
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return Pl_Get_Nil(ready_list_word);
}
pub unsafe extern "C" fn Pl_Prolog_Pid_1(mut prolog_pid_word: WamWord) -> Bool {
    let mut prolog_pid: libc::c_int = 0;
    prolog_pid = getpid();
    return Pl_Un_Integer_Check(prolog_pid as PlLong, prolog_pid_word);
}
pub unsafe extern "C" fn Pl_Send_Signal_2(
    mut pid_word: WamWord,
    mut signal_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut pid: libc::c_int = 0;
    let mut sig: libc::c_int = 0;
    let mut atom: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    pid = Pl_Rd_C_Int_Check(pid_word);
    let mut deref_last_word: WamWord = 0;
    word = signal_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        sig = -(1 as libc::c_int);
        i = 0 as libc::c_int;
        while i < nb_sig {
            if tsig[i as usize].atom == atom {
                sig = tsig[i as usize].sig;
                break;
            } else {
                i += 1;
                i;
            }
        }
    } else {
        sig = Pl_Rd_C_Int_Check(word);
    }
    let mut _tst: libc::c_int = kill(pid, sig);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Wait_2(
    mut pid_word: WamWord,
    mut status_word: WamWord,
) -> Bool {
    let mut pid: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    pid = Pl_Rd_C_Int_Check(pid_word);
    Pl_Check_For_Un_Integer(status_word);
    status = Pl_M_Get_Status(pid);
    let mut _tst: libc::c_int = status;
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return Pl_Get_Integer(status as PlLong, status_word);
}
unsafe extern "C" fn Get_Path_Name(mut path_name_word: WamWord) -> *mut libc::c_char {
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    path_name = Pl_Rd_String_Check(path_name_word);
    path_name = Pl_M_Absolute_Path_Name(path_name);
    if path_name.is_null() {
        Pl_Err_Domain(pl_domain_os_path, path_name_word);
    }
    return path_name;
}
unsafe extern "C" fn Date_Time_To_Prolog(
    mut t: *mut time_t,
    mut date_time_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut year_word: WamWord = 0;
    let mut month_word: WamWord = 0;
    let mut day_word: WamWord = 0;
    let mut hour_word: WamWord = 0;
    let mut minute_word: WamWord = 0;
    let mut second_word: WamWord = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut day: libc::c_int = 0;
    let mut month: libc::c_int = 0;
    let mut year: libc::c_int = 0;
    let mut hour: libc::c_int = 0;
    let mut minute: libc::c_int = 0;
    let mut second: libc::c_int = 0;
    tm = localtime(t);
    year = (*tm).tm_year + 1900 as libc::c_int;
    month = (*tm).tm_mon + 1 as libc::c_int;
    day = (*tm).tm_mday;
    hour = (*tm).tm_hour;
    minute = (*tm).tm_min;
    second = (*tm).tm_sec;
    let mut deref_last_word: WamWord = 0;
    word = date_time_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_compound, word);
    }
    if Pl_Get_Structure(atom_dt, 6 as libc::c_int, word) == 0 {
        Pl_Err_Domain(pl_domain_date_time, word);
    }
    year_word = Pl_Unify_Variable();
    month_word = Pl_Unify_Variable();
    day_word = Pl_Unify_Variable();
    hour_word = Pl_Unify_Variable();
    minute_word = Pl_Unify_Variable();
    second_word = Pl_Unify_Variable();
    Pl_Check_For_Un_Integer(year_word);
    Pl_Check_For_Un_Integer(month_word);
    Pl_Check_For_Un_Integer(day_word);
    Pl_Check_For_Un_Integer(hour_word);
    Pl_Check_For_Un_Integer(minute_word);
    Pl_Check_For_Un_Integer(second_word);
    return (Pl_Get_Integer(year as PlLong, year_word) != 0
        && Pl_Get_Integer(month as PlLong, month_word) != 0
        && Pl_Get_Integer(day as PlLong, day_word) != 0
        && Pl_Get_Integer(hour as PlLong, hour_word) != 0
        && Pl_Get_Integer(minute as PlLong, minute_word) != 0
        && Pl_Get_Integer(second as PlLong, second_word) != 0) as libc::c_int;
}
