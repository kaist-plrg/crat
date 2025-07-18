use ::libc;
use ::c2rust_bitfields;
extern "C" {
    static mut pl_stk_tbl: [InfStack; 0];
    static mut pl_max_atom: PlULong;
    static mut pl_nb_atom: PlULong;
    fn Pl_M_User_Time() -> PlLong;
    fn Pl_M_System_Time() -> PlLong;
    fn Pl_M_Real_Time() -> PlLong;
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_stdout: libc::c_int;
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfStack {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub env_var_name: *mut libc::c_char,
    pub p_def_size: *mut PlLong,
    pub default_size: libc::c_int,
    pub size: libc::c_int,
    pub stack: *mut WamWord,
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
static mut last_user_time: PlLong = 0 as libc::c_int as PlLong;
static mut last_system_time: PlLong = 0 as libc::c_int as PlLong;
static mut last_cpu_time: PlLong = 0 as libc::c_int as PlLong;
static mut last_real_time: PlLong = 0 as libc::c_int as PlLong;
pub unsafe extern "C" fn Pl_Statistics_0() {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(pl_stm_stdout as isize);
    let mut used: libc::c_int = 0;
    let mut free: libc::c_int = 0;
    let mut t: [PlLong; 4] = [0; 4];
    let mut l: [PlLong; 4] = [0; 4];
    static mut n: [*mut libc::c_char; 4] = [
        b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"system\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cpu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"real\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    Pl_Stream_Printf(
        pstm,
        b"Memory               limit         in use            free\n\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        Stack_Size(i, &mut used, &mut free);
        if !(used + free == 0 as libc::c_int) {
            used /= 1024 as libc::c_int;
            free /= 1024 as libc::c_int;
            Pl_Stream_Printf(
                pstm,
                b"   %-6s stack %10d Kb   %10d Kb   %10d Kb\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).name,
                used + free,
                used,
                free,
            );
        }
        i += 1;
        i;
    }
    Pl_Stream_Printf(
        pstm,
        b"   atom   table %10d atoms%10d atoms%10d atoms\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        pl_max_atom,
        pl_nb_atom,
        pl_max_atom.wrapping_sub(pl_nb_atom),
    );
    t[0 as libc::c_int as usize] = Pl_M_User_Time();
    l[0 as libc::c_int as usize] = t[0 as libc::c_int as usize] - last_user_time;
    t[1 as libc::c_int as usize] = Pl_M_System_Time();
    l[1 as libc::c_int as usize] = t[1 as libc::c_int as usize] - last_system_time;
    t[2 as libc::c_int
        as usize] = t[0 as libc::c_int as usize] + t[1 as libc::c_int as usize];
    l[2 as libc::c_int as usize] = t[2 as libc::c_int as usize] - last_cpu_time;
    t[3 as libc::c_int as usize] = Pl_M_Real_Time();
    l[3 as libc::c_int as usize] = t[3 as libc::c_int as usize] - last_real_time;
    last_user_time = t[0 as libc::c_int as usize];
    last_system_time = t[1 as libc::c_int as usize];
    last_cpu_time = t[2 as libc::c_int as usize];
    last_real_time = t[3 as libc::c_int as usize];
    Pl_Stream_Printf(
        pstm,
        b"\nTimes              since start      since last\n\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        Pl_Stream_Printf(
            pstm,
            b"   %-6s time %11.3f sec %11.3f sec\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            n[i as usize],
            t[i as usize] as libc::c_double / 1000.0f64,
            l[i as usize] as libc::c_double / 1000.0f64,
        );
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn Pl_Statistics_User_Time_2(
    mut since_start_word: WamWord,
    mut since_last_word: WamWord,
) -> Bool {
    let mut user_time: PlLong = 0;
    let mut since_start: libc::c_int = 0;
    let mut since_last: libc::c_int = 0;
    user_time = Pl_M_User_Time();
    since_start = user_time as libc::c_int;
    since_last = (user_time - last_user_time) as libc::c_int;
    last_user_time = user_time;
    return (Pl_Un_Integer_Check(since_start as PlLong, since_start_word) != 0
        && Pl_Un_Integer_Check(since_last as PlLong, since_last_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_System_Time_2(
    mut since_start_word: WamWord,
    mut since_last_word: WamWord,
) -> Bool {
    let mut system_time: PlLong = 0;
    let mut since_start: libc::c_int = 0;
    let mut since_last: libc::c_int = 0;
    system_time = Pl_M_System_Time();
    since_start = system_time as libc::c_int;
    since_last = (system_time - last_system_time) as libc::c_int;
    last_system_time = system_time;
    return (Pl_Un_Integer_Check(since_start as PlLong, since_start_word) != 0
        && Pl_Un_Integer_Check(since_last as PlLong, since_last_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_Cpu_Time_2(
    mut since_start_word: WamWord,
    mut since_last_word: WamWord,
) -> Bool {
    let mut cpu_time: PlLong = 0;
    let mut since_start: libc::c_int = 0;
    let mut since_last: libc::c_int = 0;
    cpu_time = Pl_M_User_Time() + Pl_M_System_Time();
    since_start = cpu_time as libc::c_int;
    since_last = (cpu_time - last_cpu_time) as libc::c_int;
    last_cpu_time = cpu_time;
    return (Pl_Un_Integer_Check(since_start as PlLong, since_start_word) != 0
        && Pl_Un_Integer_Check(since_last as PlLong, since_last_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_Real_Time_2(
    mut since_start_word: WamWord,
    mut since_last_word: WamWord,
) -> Bool {
    let mut real_time: PlLong = 0;
    let mut since_start: libc::c_int = 0;
    let mut since_last: libc::c_int = 0;
    real_time = Pl_M_Real_Time();
    since_start = real_time as libc::c_int;
    since_last = (real_time - last_real_time) as libc::c_int;
    last_real_time = real_time;
    return (Pl_Un_Integer_Check(since_start as PlLong, since_start_word) != 0
        && Pl_Un_Integer_Check(since_last as PlLong, since_last_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_Local_Stack_2(
    mut used_word: WamWord,
    mut free_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut free: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack
            == (*pl_stk_tbl.as_mut_ptr().offset(3 as libc::c_int as isize)).stack
        {
            Stack_Size(i, &mut used, &mut free);
        }
        i += 1;
        i;
    }
    return (Pl_Un_Integer_Check(used as PlLong, used_word) != 0
        && Pl_Un_Integer_Check(free as PlLong, free_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_Global_Stack_2(
    mut used_word: WamWord,
    mut free_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut free: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack
            == (*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack
        {
            Stack_Size(i, &mut used, &mut free);
        }
        i += 1;
        i;
    }
    return (Pl_Un_Integer_Check(used as PlLong, used_word) != 0
        && Pl_Un_Integer_Check(free as PlLong, free_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_Trail_Stack_2(
    mut used_word: WamWord,
    mut free_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut free: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack
            == (*pl_stk_tbl.as_mut_ptr().offset(0 as libc::c_int as isize)).stack
        {
            Stack_Size(i, &mut used, &mut free);
        }
        i += 1;
        i;
    }
    return (Pl_Un_Integer_Check(used as PlLong, used_word) != 0
        && Pl_Un_Integer_Check(free as PlLong, free_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_Cstr_Stack_2(
    mut used_word: WamWord,
    mut free_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut free: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack
            == (*pl_stk_tbl.as_mut_ptr().offset(1 as libc::c_int as isize)).stack
        {
            Stack_Size(i, &mut used, &mut free);
        }
        i += 1;
        i;
    }
    return (Pl_Un_Integer_Check(used as PlLong, used_word) != 0
        && Pl_Un_Integer_Check(free as PlLong, free_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Statistics_Atoms_2(
    mut used_word: WamWord,
    mut free_word: WamWord,
) -> Bool {
    return (Pl_Un_Integer_Check(pl_nb_atom as PlLong, used_word) != 0
        && Pl_Un_Integer_Check(pl_max_atom.wrapping_sub(pl_nb_atom) as PlLong, free_word)
            != 0) as libc::c_int;
}
unsafe extern "C" fn Stack_Size(
    mut stack_nb: libc::c_int,
    mut used: *mut libc::c_int,
    mut free: *mut libc::c_int,
) {
    *used = (if stack_nb == 0 as libc::c_int {
        TR
    } else {
        (if stack_nb == 1 as libc::c_int {
            *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 3 as libc::c_int) as isize)
        } else {
            (if stack_nb == 2 as libc::c_int {
                H
            } else {
                (if B
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
                {
                    B
                } else {
                    *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
                })
            })
        })
    })
        .offset_from((*pl_stk_tbl.as_mut_ptr().offset(stack_nb as isize)).stack)
        as libc::c_long as libc::c_int;
    *free = (*pl_stk_tbl.as_mut_ptr().offset(stack_nb as isize)).size - *used;
    if (*pl_stk_tbl.as_mut_ptr().offset(stack_nb as isize)).stack
        == (*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack
    {
        *used += 256 as libc::c_int + 8 as libc::c_int;
    }
    *used = (*used as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong) as libc::c_int
        as libc::c_int;
    *free = (*free as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong) as libc::c_int
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_User_Time_1(mut since_start_word: WamWord) -> Bool {
    return Pl_Un_Integer_Check(Pl_M_User_Time(), since_start_word);
}
pub unsafe extern "C" fn Pl_System_Time_1(mut since_start_word: WamWord) -> Bool {
    return Pl_Un_Integer_Check(Pl_M_System_Time(), since_start_word);
}
pub unsafe extern "C" fn Pl_Cpu_Time_1(mut since_start_word: WamWord) -> Bool {
    return Pl_Un_Integer_Check(Pl_M_User_Time() + Pl_M_System_Time(), since_start_word);
}
pub unsafe extern "C" fn Pl_Real_Time_1(mut since_start_word: WamWord) -> Bool {
    return Pl_Un_Integer_Check(Pl_M_Real_Time(), since_start_word);
}
