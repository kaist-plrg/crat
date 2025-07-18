use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Pl_M_Absolute_Path_Name(src: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_M_Is_Absolute_File_Name(path: *mut libc::c_char) -> Bool;
    fn Pl_M_Decompose_File_Name(
        path: *mut libc::c_char,
        del_trail_slashes: Bool,
        base: *mut *mut libc::c_char,
        suffix: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Check_For_Un_Atom(start_word: WamWord);
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_String(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    static mut pl_domain_os_path: libc::c_int;
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Absolute_File_Name_2(
    mut path1_word: WamWord,
    mut path2_word: WamWord,
) -> Bool {
    let mut path1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path2: *mut libc::c_char = 0 as *mut libc::c_char;
    path1 = (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(path1_word) as isize)).name;
    path2 = Pl_M_Absolute_Path_Name(path1);
    if path2.is_null() {
        Pl_Err_Domain(pl_domain_os_path, path1_word);
    }
    return Pl_Un_String_Check(path2, path2_word);
}
pub unsafe extern "C" fn Pl_Is_Absolute_File_Name_1(mut path_word: WamWord) -> Bool {
    let mut path: *mut libc::c_char = (*pl_atom_tbl
        .offset(Pl_Rd_Atom_Check(path_word) as isize))
        .name;
    return Pl_M_Is_Absolute_File_Name(path);
}
pub unsafe extern "C" fn Pl_Is_Relative_File_Name_1(mut path_word: WamWord) -> Bool {
    return (Pl_Is_Absolute_File_Name_1(path_word) == 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Decompose_File_Name_4(
    mut path_word: WamWord,
    mut dir_word: WamWord,
    mut prefix_word: WamWord,
    mut suffix_word: WamWord,
) -> Bool {
    let mut path: *mut libc::c_char = (*pl_atom_tbl
        .offset(Pl_Rd_Atom_Check(path_word) as isize))
        .name;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    Pl_Check_For_Un_Atom(dir_word);
    Pl_Check_For_Un_Atom(prefix_word);
    Pl_Check_For_Un_Atom(suffix_word);
    dir = Pl_M_Decompose_File_Name(path, 0 as libc::c_int, &mut base, &mut suffix);
    if Pl_Un_String(dir, dir_word) == 0 {
        return 0 as libc::c_int;
    }
    c = *suffix;
    *suffix = '\0' as i32 as libc::c_char;
    if Pl_Un_String(base, prefix_word) == 0 {
        return 0 as libc::c_int;
    }
    *suffix = c;
    return Pl_Un_String(suffix, suffix_word);
}
pub unsafe extern "C" fn Pl_Prolog_File_Name_2(
    mut path1_word: WamWord,
    mut path2_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = 0;
    let mut path1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut suffixes: [libc::c_char; 19] = *::std::mem::transmute::<
        &[u8; 19],
        &mut [libc::c_char; 19],
    >(b"|.pl|.pro|.prolog|\0");
    atom = Pl_Rd_Atom_Check(path1_word);
    path1 = (*pl_atom_tbl.offset(atom as isize)).name;
    path1 = Pl_M_Absolute_Path_Name(path1);
    if path1.is_null() {
        Pl_Err_Domain(pl_domain_os_path, path1_word);
    }
    if !(strcmp(path1, b"user\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int)
    {
        let mut _ptr: *mut libc::c_char = 0 as *mut libc::c_char;
        p = 0 as *mut libc::c_char;
        _ptr = path1;
        while *_ptr != 0 {
            if *_ptr as libc::c_int == '/' as i32 || *_ptr as libc::c_int == '/' as i32 {
                p = _ptr;
            }
            _ptr = _ptr.offset(1);
            _ptr;
        }
        if (strchr(if !p.is_null() { p } else { path1 }, '.' as i32)).is_null() {
            strcpy(pl_glob_buff.as_mut_ptr(), path1);
            len = strlen(path1) as libc::c_int;
            q = suffixes.as_mut_ptr();
            loop {
                p = q.offset(1 as libc::c_int as isize);
                if *p as libc::c_int == '\0' as i32 {
                    p = b".pl\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    break;
                } else {
                    q = strchr(p, '|' as i32);
                    *q = '\0' as i32 as libc::c_char;
                    strcpy(pl_glob_buff.as_mut_ptr().offset(len as isize), p);
                    if !(access(pl_glob_buff.as_mut_ptr(), 0 as libc::c_int) != 0) {
                        break;
                    }
                }
            }
            sprintf(
                pl_glob_buff.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                (*pl_atom_tbl.offset(atom as isize)).name,
                p,
            );
            return Pl_Un_String_Check(pl_glob_buff.as_mut_ptr(), path2_word);
        }
    }
    return Pl_Un_Atom_Check(atom, path2_word);
}
pub unsafe extern "C" fn Pl_Prolog_File_Suffix_1(mut suffix_word: WamWord) -> Bool {
    let mut atom: libc::c_int = 0;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    atom = Pl_Rd_Atom_Check(suffix_word);
    suffix = (*pl_atom_tbl.offset(atom as isize)).name;
    return (Find_Suffix(
        b"|.pl|.pro|.prolog|\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        suffix,
    ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
unsafe extern "C" fn Find_Suffix(
    mut suffixes: *mut libc::c_char,
    mut suffix: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strstr(suffixes, suffix);
    if !p.is_null()
        && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '|' as i32
        && *p.offset(strlen(suffix) as isize) as libc::c_int == '|' as i32
    {
        return p;
    }
    return 0 as *mut libc::c_char;
}
