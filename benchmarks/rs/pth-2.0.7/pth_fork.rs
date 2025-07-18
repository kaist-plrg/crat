use ::libc;
extern "C" {
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn __pth_scheduler_drop();
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_atfork_st {
    pub prepare: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub parent: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub child: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
static mut pth_atfork_list: [pth_atfork_st; 128] = [pth_atfork_st {
    prepare: None,
    parent: None,
    child: None,
    arg: 0 as *const libc::c_void as *mut libc::c_void,
}; 128];
static mut pth_atfork_idx: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn pth_atfork_push(
    mut prepare: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut parent: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut child: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    if pth_atfork_idx > 128 as libc::c_int - 1 as libc::c_int {
        *__errno_location() = 12 as libc::c_int;
        return 0 as libc::c_int;
    }
    pth_atfork_list[pth_atfork_idx as usize].prepare = prepare;
    pth_atfork_list[pth_atfork_idx as usize].parent = parent;
    pth_atfork_list[pth_atfork_idx as usize].child = child;
    pth_atfork_list[pth_atfork_idx as usize].arg = arg;
    pth_atfork_idx += 1;
    pth_atfork_idx;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_atfork_pop() -> libc::c_int {
    if pth_atfork_idx <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    pth_atfork_idx -= 1;
    pth_atfork_idx;
    return (0 as libc::c_int == 0) as libc::c_int;
}
pub unsafe extern "C" fn pth_fork() -> pid_t {
    let mut pid: pid_t = 0;
    let mut i: libc::c_int = 0;
    i = pth_atfork_idx - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (pth_atfork_list[i as usize].prepare).is_some() {
            (pth_atfork_list[i as usize].prepare)
                .unwrap()(pth_atfork_list[i as usize].arg);
        }
        i -= 1;
        i;
    }
    pid = fork();
    if pid == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if pid != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= pth_atfork_idx - 1 as libc::c_int {
            if (pth_atfork_list[i as usize].parent).is_some() {
                (pth_atfork_list[i as usize].parent)
                    .unwrap()(pth_atfork_list[i as usize].arg);
            }
            i += 1;
            i;
        }
    } else {
        __pth_scheduler_drop();
        i = 0 as libc::c_int;
        while i <= pth_atfork_idx - 1 as libc::c_int {
            if (pth_atfork_list[i as usize].child).is_some() {
                (pth_atfork_list[i as usize].child)
                    .unwrap()(pth_atfork_list[i as usize].arg);
            }
            i += 1;
            i;
        }
    }
    return pid;
}
