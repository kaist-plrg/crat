use ::libc;
extern "C" {
    fn getpid() -> __pid_t;
    fn set_filter_pattern(pattern: *mut libc::c_char, search_type: libc::c_int);
    fn quit(status: libc::c_int);
    fn raw_mode(on: libc::c_int);
    fn get_term();
    fn init();
    fn deinit();
    fn bell();
    fn clear_bot();
    fn calc_jump_sline();
    fn calc_shift_count();
    fn intread();
    fn flush();
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    static mut sc_width: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut screen_trashed: libc::c_int;
    static mut wscroll: libc::c_int;
    static mut reading: libc::c_int;
    static mut quit_on_intr: libc::c_int;
    static mut secure: libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub static mut sigs: libc::c_int = 0;
unsafe extern "C" fn u_interrupt(mut type_0: libc::c_int) {
    bell();
    signal(
        2 as libc::c_int,
        Some(u_interrupt as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sigs |= 0o1 as libc::c_int;
    set_filter_pattern(0 as *mut libc::c_char, 0 as libc::c_int);
    if reading != 0 {
        intread();
    }
}
unsafe extern "C" fn stop(mut type_0: libc::c_int) {
    signal(20 as libc::c_int, Some(stop as unsafe extern "C" fn(libc::c_int) -> ()));
    sigs |= 0o2 as libc::c_int;
    if reading != 0 {
        intread();
    }
}
pub unsafe extern "C" fn winch(mut type_0: libc::c_int) {
    signal(28 as libc::c_int, Some(winch as unsafe extern "C" fn(libc::c_int) -> ()));
    sigs |= 0o4 as libc::c_int;
    if reading != 0 {
        intread();
    }
}
unsafe extern "C" fn terminate(mut type_0: libc::c_int) {
    quit(15 as libc::c_int);
}
pub unsafe extern "C" fn init_signals(mut on: libc::c_int) {
    if on != 0 {
        signal(
            2 as libc::c_int,
            Some(u_interrupt as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            20 as libc::c_int,
            if secure != 0 {
                ::std::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t)
            } else {
                Some(stop as unsafe extern "C" fn(libc::c_int) -> ())
            },
        );
        signal(
            28 as libc::c_int,
            Some(winch as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            3 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        signal(
            15 as libc::c_int,
            Some(terminate as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    } else {
        signal(2 as libc::c_int, None);
        signal(20 as libc::c_int, None);
        signal(
            28 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        signal(3 as libc::c_int, None);
        signal(15 as libc::c_int, None);
    };
}
pub unsafe extern "C" fn psignals() {
    let mut tsignals: libc::c_int = 0;
    tsignals = sigs;
    if tsignals == 0 as libc::c_int {
        return;
    }
    sigs = 0 as libc::c_int;
    if tsignals & 0o2 as libc::c_int != 0 {
        signal(
            22 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        clear_bot();
        deinit();
        flush();
        raw_mode(0 as libc::c_int);
        signal(22 as libc::c_int, None);
        signal(20 as libc::c_int, None);
        kill(getpid(), 20 as libc::c_int);
        signal(20 as libc::c_int, Some(stop as unsafe extern "C" fn(libc::c_int) -> ()));
        raw_mode(1 as libc::c_int);
        init();
        screen_trashed = 1 as libc::c_int;
        tsignals |= 0o4 as libc::c_int;
    }
    if tsignals & 0o4 as libc::c_int != 0 {
        let mut old_width: libc::c_int = 0;
        let mut old_height: libc::c_int = 0;
        old_width = sc_width;
        old_height = sc_height;
        get_term();
        if sc_width != old_width || sc_height != old_height {
            wscroll = (sc_height + 1 as libc::c_int) / 2 as libc::c_int;
            calc_jump_sline();
            calc_shift_count();
        }
        screen_trashed = 1 as libc::c_int;
    }
    if tsignals & 0o1 as libc::c_int != 0 {
        if quit_on_intr != 0 {
            quit(1 as libc::c_int + 1 as libc::c_int);
        }
    }
}
