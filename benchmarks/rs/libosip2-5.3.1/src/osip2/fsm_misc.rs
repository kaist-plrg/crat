use ::libc;
pub type _state_t = libc::c_uint;
pub const DIALOG_CLOSE: _state_t = 22;
pub const DIALOG_CONFIRMED: _state_t = 21;
pub const DIALOG_EARLY: _state_t = 20;
pub const NIST_TERMINATED: _state_t = 19;
pub const NIST_COMPLETED: _state_t = 18;
pub const NIST_PROCEEDING: _state_t = 17;
pub const NIST_TRYING: _state_t = 16;
pub const NIST_PRE_TRYING: _state_t = 15;
pub const NICT_TERMINATED: _state_t = 14;
pub const NICT_COMPLETED: _state_t = 13;
pub const NICT_PROCEEDING: _state_t = 12;
pub const NICT_TRYING: _state_t = 11;
pub const NICT_PRE_TRYING: _state_t = 10;
pub const IST_TERMINATED: _state_t = 9;
pub const IST_CONFIRMED: _state_t = 8;
pub const IST_COMPLETED: _state_t = 7;
pub const IST_PROCEEDING: _state_t = 6;
pub const IST_PRE_PROCEEDING: _state_t = 5;
pub const ICT_TERMINATED: _state_t = 4;
pub const ICT_COMPLETED: _state_t = 3;
pub const ICT_PROCEEDING: _state_t = 2;
pub const ICT_CALLING: _state_t = 1;
pub const ICT_PRE_CALLING: _state_t = 0;
pub type state_t = _state_t;
pub type type_t = libc::c_uint;
pub const UNKNOWN_EVT: type_t = 23;
pub const KILL_TRANSACTION: type_t = 22;
pub const SND_STATUS_3456XX: type_t = 21;
pub const SND_STATUS_2XX: type_t = 20;
pub const SND_STATUS_1XX: type_t = 19;
pub const SND_REQUEST: type_t = 18;
pub const SND_REQACK: type_t = 17;
pub const SND_REQINVITE: type_t = 16;
pub const RCV_STATUS_3456XX: type_t = 15;
pub const RCV_STATUS_2XX: type_t = 14;
pub const RCV_STATUS_1XX: type_t = 13;
pub const RCV_REQUEST: type_t = 12;
pub const RCV_REQACK: type_t = 11;
pub const RCV_REQINVITE: type_t = 10;
pub const TIMEOUT_J: type_t = 9;
pub const TIMEOUT_I: type_t = 8;
pub const TIMEOUT_H: type_t = 7;
pub const TIMEOUT_G: type_t = 6;
pub const TIMEOUT_K: type_t = 5;
pub const TIMEOUT_F: type_t = 4;
pub const TIMEOUT_E: type_t = 3;
pub const TIMEOUT_D: type_t = 2;
pub const TIMEOUT_B: type_t = 1;
pub const TIMEOUT_A: type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osip_statemachine {
    pub transitions: *mut _transition_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _transition_t {
    pub state: state_t,
    pub type_0: type_t,
    pub method: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub next: *mut _transition_t,
    pub parent: *mut _transition_t,
}
pub type osip_statemachine_t = osip_statemachine;
pub type transition_t = _transition_t;
unsafe extern "C" fn fsm_findmethod(
    mut type_0: type_t,
    mut state: state_t,
    mut statemachine: *mut osip_statemachine_t,
) -> *mut transition_t {
    let mut transition: *mut transition_t = 0 as *mut transition_t;
    transition = (*statemachine).transitions;
    while !transition.is_null() {
        if (*transition).type_0 as libc::c_uint == type_0 as libc::c_uint
            && (*transition).state as libc::c_uint == state as libc::c_uint
        {
            return transition;
        }
        transition = (*transition).next;
    }
    return 0 as *mut transition_t;
}
pub unsafe extern "C" fn fsm_callmethod(
    mut type_0: type_t,
    mut state: state_t,
    mut statemachine: *mut osip_statemachine_t,
    mut sipevent: *mut libc::c_void,
    mut transaction: *mut libc::c_void,
) -> libc::c_int {
    let mut transition: *mut transition_t = 0 as *mut transition_t;
    transition = fsm_findmethod(type_0, state, statemachine);
    if transition.is_null() {
        return -(1 as libc::c_int);
    }
    ((*transition).method).unwrap()(transaction, sipevent);
    return 0 as libc::c_int;
}
