use ::libc;
extern "C" {
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_Dummy_Ptr(p: *mut libc::c_void) -> *mut libc::c_void;
}
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_fd_variable_to_string: Option::<
    unsafe extern "C" fn() -> *mut libc::c_char,
> = None;
pub static mut pl_fd_copy_variable: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_fd_variable_size: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_fd_unify_with_fd_var: Option::<unsafe extern "C" fn() -> Bool> = None;
pub static mut pl_fd_unify_with_integer: Option::<unsafe extern "C" fn() -> Bool> = None;
pub static mut pl_fd_reset_solver: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_fd_init_solver: Option::<unsafe extern "C" fn() -> ()> = None;
pub unsafe extern "C" fn Pl_Fd_Init_Solver() {
    if pl_fd_init_solver.is_none() {
        pl_fd_unify_with_integer = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Fd_Solver_Missing as unsafe extern "C" fn() -> ()));
        pl_fd_unify_with_fd_var = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Fd_Solver_Missing as unsafe extern "C" fn() -> ()));
        pl_fd_variable_size = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(Some(Fd_Solver_Missing as unsafe extern "C" fn() -> ()));
        pl_fd_copy_variable = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(Some(Fd_Solver_Missing as unsafe extern "C" fn() -> ()));
        pl_fd_variable_to_string = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> *mut libc::c_char>,
        >(Some(Fd_Solver_Missing as unsafe extern "C" fn() -> ()));
        return;
    }
    ::std::mem::transmute::<_, fn()>((Some(pl_fd_init_solver.unwrap())).unwrap())();
}
pub unsafe extern "C" fn Pl_Fd_Reset_Solver() {
    let mut copy_of_pl_fd_reset_solver: Option::<unsafe extern "C" fn() -> ()> = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Pl_Dummy_Ptr(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                *mut libc::c_void,
            >(pl_fd_reset_solver),
        ),
    );
    if copy_of_pl_fd_reset_solver.is_none() {
        return;
    }
    ::std::mem::transmute::<
        _,
        fn(),
    >((Some(copy_of_pl_fd_reset_solver.unwrap())).unwrap())();
}
unsafe extern "C" fn Fd_Solver_Missing() {
    Pl_Fatal_Error(
        b"if_no_fd.c: FD Solver not linked\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
