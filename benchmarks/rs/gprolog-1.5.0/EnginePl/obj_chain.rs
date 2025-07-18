use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjInf {
    pub fct_obj_init: Option::<unsafe extern "C" fn() -> ()>,
    pub fct_exec_system: Option::<unsafe extern "C" fn() -> ()>,
    pub fct_exec_user: Option::<unsafe extern "C" fn() -> ()>,
}
static mut obj_tbl: [ObjInf; 1024] = [ObjInf {
    fct_obj_init: None,
    fct_exec_system: None,
    fct_exec_user: None,
}; 1024];
static mut nb_obj: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn Pl_Find_Linked_Objects() {
    let mut i: libc::c_int = 0;
    i = nb_obj;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if (obj_tbl[i as usize].fct_obj_init).is_some() {
            ::std::mem::transmute::<
                _,
                fn(),
            >(
                (Some(
                    ((*obj_tbl.as_mut_ptr().offset(i as isize)).fct_obj_init).unwrap(),
                ))
                    .unwrap(),
            )();
        }
    }
    i = 0 as libc::c_int;
    while i < nb_obj {
        if (obj_tbl[i as usize].fct_exec_system).is_some() {
            ::std::mem::transmute::<
                _,
                fn(),
            >(
                (Some(
                    ((*obj_tbl.as_mut_ptr().offset(i as isize)).fct_exec_system).unwrap(),
                ))
                    .unwrap(),
            )();
        }
        i += 1;
        i;
    }
    i = nb_obj;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if (obj_tbl[i as usize].fct_exec_user).is_some() {
            ::std::mem::transmute::<
                _,
                fn(),
            >(
                (Some(
                    ((*obj_tbl.as_mut_ptr().offset(i as isize)).fct_exec_user).unwrap(),
                ))
                    .unwrap(),
            )();
        }
    };
}
pub unsafe extern "C" fn Pl_New_Object(
    mut fct_obj_init: Option::<unsafe extern "C" fn() -> ()>,
    mut fct_exec_system: Option::<unsafe extern "C" fn() -> ()>,
    mut fct_exec_user: Option::<unsafe extern "C" fn() -> ()>,
) {
    obj_tbl[nb_obj as usize].fct_obj_init = fct_obj_init;
    obj_tbl[nb_obj as usize].fct_exec_system = fct_exec_system;
    obj_tbl[nb_obj as usize].fct_exec_user = fct_exec_user;
    nb_obj += 1;
    nb_obj;
}
