use ::libc;
extern "C" {
    static mut _zero_: bc_num;
    static mut _one_: bc_num;
    fn bc_init_numbers();
    fn bc_free_num(num: *mut bc_num);
    fn bc_copy_num(num: bc_num) -> bc_num;
    fn bc_init_num(num: *mut bc_num);
    fn bc_sub(n1: bc_num, n2: bc_num, result: *mut bc_num, scale_min: libc::c_int);
    fn bc_add(n1: bc_num, n2: bc_num, result: *mut bc_num, scale_min: libc::c_int);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn bc_int2num(num: *mut bc_num, val: libc::c_int);
    fn bc_num2long(num: bc_num) -> libc::c_long;
    fn bc_is_zero(num: bc_num) -> libc::c_char;
    fn bc_is_neg(num: bc_num) -> libc::c_char;
    static mut warn_not_std: libc::c_int;
    static mut std_only: libc::c_int;
    static mut functions: *mut bc_function;
    static mut f_names: *mut *mut libc::c_char;
    static mut f_count: libc::c_int;
    static mut variables: *mut *mut bc_var;
    static mut v_names: *mut *mut libc::c_char;
    static mut v_count: libc::c_int;
    static mut arrays: *mut *mut bc_var_array;
    static mut a_names: *mut *mut libc::c_char;
    static mut a_count: libc::c_int;
    static mut ex_stack: *mut estack_rec;
    static mut fn_stack: *mut fstack_rec;
    static mut i_base: libc::c_int;
    static mut o_base: libc::c_int;
    static mut scale: libc::c_int;
    static mut c_code: libc::c_char;
    fn byte(pc_: *mut program_counter) -> libc::c_uchar;
    fn free(__ptr: *mut libc::c_void);
    fn free_args(args: *mut arg_list);
    fn bc_malloc(_: size_t) -> *mut libc::c_void;
    fn rt_error(mesg: *const libc::c_char, _: ...);
    fn rt_warn(mesg: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type sign = libc::c_uint;
pub const MINUS: sign = 1;
pub const PLUS: sign = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_struct {
    pub n_sign: sign,
    pub n_len: libc::c_int,
    pub n_scale: libc::c_int,
    pub n_refs: libc::c_int,
    pub n_next: bc_num,
    pub n_ptr: *mut libc::c_char,
    pub n_value: *mut libc::c_char,
}
pub type bc_num = *mut bc_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_label_group {
    pub l_adrs: [libc::c_ulong; 64],
    pub l_next: *mut bc_label_group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_list {
    pub av_name: libc::c_int,
    pub arg_is_var: libc::c_int,
    pub next: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_function {
    pub f_defined: libc::c_char,
    pub f_void: libc::c_char,
    pub f_body: *mut libc::c_char,
    pub f_body_size: size_t,
    pub f_code_size: size_t,
    pub f_label: *mut bc_label_group,
    pub f_params: *mut arg_list,
    pub f_autos: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_counter {
    pub pc_func: libc::c_uint,
    pub pc_addr: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_var {
    pub v_value: bc_num,
    pub v_next: *mut bc_var,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_array_node {
    pub n_items: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub n_num: [bc_num; 64],
    pub n_down: [*mut bc_array_node; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_array {
    pub a_tree: *mut bc_array_node,
    pub a_depth: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_var_array {
    pub a_value: *mut bc_array,
    pub a_param: libc::c_char,
    pub a_next: *mut bc_var_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct estack_rec {
    pub s_num: bc_num,
    pub s_next: *mut estack_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fstack_rec {
    pub s_val: libc::c_int,
    pub s_next: *mut fstack_rec,
}
pub unsafe extern "C" fn init_storage() {
    f_count = 0 as libc::c_int;
    more_functions();
    let ref mut fresh0 = *f_names.offset(0 as libc::c_int as isize);
    *fresh0 = strdup(b"(main)\0" as *const u8 as *const libc::c_char);
    v_count = 0 as libc::c_int;
    more_variables();
    a_count = 0 as libc::c_int;
    more_arrays();
    ex_stack = 0 as *mut estack_rec;
    fn_stack = 0 as *mut fstack_rec;
    i_base = 10 as libc::c_int;
    o_base = 10 as libc::c_int;
    scale = 0 as libc::c_int;
    c_code = 0 as libc::c_int as libc::c_char;
    bc_init_numbers();
}
pub unsafe extern "C" fn more_functions() {
    let mut old_count: libc::c_int = 0;
    let mut indx: libc::c_int = 0;
    let mut old_f: *mut bc_function = 0 as *mut bc_function;
    let mut f: *mut bc_function = 0 as *mut bc_function;
    let mut old_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    old_count = f_count;
    old_f = functions;
    old_names = f_names;
    f_count += 32 as libc::c_int;
    functions = bc_malloc(
        (f_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<bc_function>() as libc::c_ulong),
    ) as *mut bc_function;
    f_names = bc_malloc(
        (f_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    indx = 0 as libc::c_int;
    while indx < old_count {
        *functions.offset(indx as isize) = *old_f.offset(indx as isize);
        let ref mut fresh1 = *f_names.offset(indx as isize);
        *fresh1 = *old_names.offset(indx as isize);
        indx += 1;
        indx;
    }
    while indx < f_count {
        f = &mut *functions.offset(indx as isize) as *mut bc_function;
        (*f).f_defined = 0 as libc::c_int as libc::c_char;
        (*f).f_void = 0 as libc::c_int as libc::c_char;
        (*f).f_body = bc_malloc(1024 as libc::c_int as size_t) as *mut libc::c_char;
        (*f).f_body_size = 1024 as libc::c_int as size_t;
        (*f).f_code_size = 0 as libc::c_int as size_t;
        (*f).f_label = 0 as *mut bc_label_group;
        (*f).f_autos = 0 as *mut arg_list;
        (*f).f_params = 0 as *mut arg_list;
        indx += 1;
        indx;
    }
    if old_count != 0 as libc::c_int {
        free(old_f as *mut libc::c_void);
        free(old_names as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn more_variables() {
    let mut indx: libc::c_int = 0;
    let mut old_count: libc::c_int = 0;
    let mut old_var: *mut *mut bc_var = 0 as *mut *mut bc_var;
    let mut old_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    old_count = v_count;
    old_var = variables;
    old_names = v_names;
    v_count += 32 as libc::c_int;
    variables = bc_malloc(
        (v_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut bc_var>() as libc::c_ulong),
    ) as *mut *mut bc_var;
    v_names = bc_malloc(
        (v_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    indx = 3 as libc::c_int;
    while indx < old_count {
        let ref mut fresh2 = *variables.offset(indx as isize);
        *fresh2 = *old_var.offset(indx as isize);
        let ref mut fresh3 = *v_names.offset(indx as isize);
        *fresh3 = *old_names.offset(indx as isize);
        indx += 1;
        indx;
    }
    while indx < v_count {
        let ref mut fresh4 = *variables.offset(indx as isize);
        *fresh4 = 0 as *mut bc_var;
        indx += 1;
        indx;
    }
    if old_count != 0 as libc::c_int {
        free(old_var as *mut libc::c_void);
        free(old_names as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn more_arrays() {
    let mut indx: libc::c_int = 0;
    let mut old_count: libc::c_int = 0;
    let mut old_ary: *mut *mut bc_var_array = 0 as *mut *mut bc_var_array;
    let mut old_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    old_count = a_count;
    old_ary = arrays;
    old_names = a_names;
    a_count += 32 as libc::c_int;
    arrays = bc_malloc(
        (a_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut bc_var_array>() as libc::c_ulong),
    ) as *mut *mut bc_var_array;
    a_names = bc_malloc(
        (a_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    indx = 1 as libc::c_int;
    while indx < old_count {
        let ref mut fresh5 = *arrays.offset(indx as isize);
        *fresh5 = *old_ary.offset(indx as isize);
        let ref mut fresh6 = *a_names.offset(indx as isize);
        *fresh6 = *old_names.offset(indx as isize);
        indx += 1;
        indx;
    }
    while indx < a_count {
        let ref mut fresh7 = *arrays.offset(indx as isize);
        *fresh7 = 0 as *mut bc_var_array;
        indx += 1;
        indx;
    }
    if old_count != 0 as libc::c_int {
        free(old_ary as *mut libc::c_void);
        free(old_names as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn clear_func(mut func: libc::c_int) {
    let mut f: *mut bc_function = 0 as *mut bc_function;
    let mut lg: *mut bc_label_group = 0 as *mut bc_label_group;
    f = &mut *functions.offset(func as isize) as *mut bc_function;
    (*f).f_defined = 0 as libc::c_int as libc::c_char;
    (*f).f_code_size = 0 as libc::c_int as size_t;
    if !((*f).f_autos).is_null() {
        free_args((*f).f_autos);
        (*f).f_autos = 0 as *mut arg_list;
    }
    if !((*f).f_params).is_null() {
        free_args((*f).f_params);
        (*f).f_params = 0 as *mut arg_list;
    }
    while !((*f).f_label).is_null() {
        lg = (*(*f).f_label).l_next;
        free((*f).f_label as *mut libc::c_void);
        (*f).f_label = lg;
    }
}
pub unsafe extern "C" fn fpop() -> libc::c_int {
    let mut temp: *mut fstack_rec = 0 as *mut fstack_rec;
    let mut retval: libc::c_int = 0;
    if !fn_stack.is_null() {
        temp = fn_stack;
        fn_stack = (*temp).s_next;
        retval = (*temp).s_val;
        free(temp as *mut libc::c_void);
    } else {
        retval = 0 as libc::c_int;
        rt_error(
            b"function stack underflow, contact maintainer.\0" as *const u8
                as *const libc::c_char,
        );
    }
    return retval;
}
pub unsafe extern "C" fn fpush(mut val: libc::c_int) {
    let mut temp: *mut fstack_rec = 0 as *mut fstack_rec;
    temp = bc_malloc(::std::mem::size_of::<fstack_rec>() as libc::c_ulong)
        as *mut fstack_rec;
    (*temp).s_next = fn_stack;
    (*temp).s_val = val;
    fn_stack = temp;
}
pub unsafe extern "C" fn pop() {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    if !ex_stack.is_null() {
        temp = ex_stack;
        ex_stack = (*temp).s_next;
        bc_free_num(&mut (*temp).s_num);
        free(temp as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn push_copy(mut num: bc_num) {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    temp = bc_malloc(::std::mem::size_of::<estack_rec>() as libc::c_ulong)
        as *mut estack_rec;
    (*temp).s_num = bc_copy_num(num);
    (*temp).s_next = ex_stack;
    ex_stack = temp;
}
pub unsafe extern "C" fn push_num(mut num: bc_num) {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    temp = bc_malloc(::std::mem::size_of::<estack_rec>() as libc::c_ulong)
        as *mut estack_rec;
    (*temp).s_num = num;
    (*temp).s_next = ex_stack;
    ex_stack = temp;
}
pub unsafe extern "C" fn check_stack(mut depth: libc::c_int) -> libc::c_char {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    temp = ex_stack;
    while !temp.is_null() && depth > 0 as libc::c_int {
        temp = (*temp).s_next;
        depth -= 1;
        depth;
    }
    if depth > 0 as libc::c_int {
        rt_error(b"Stack error.\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int as libc::c_char;
    }
    return 1 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn get_var(mut var_name: libc::c_int) -> *mut bc_var {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    var_ptr = *variables.offset(var_name as isize);
    if var_ptr.is_null() {
        let ref mut fresh8 = *variables.offset(var_name as isize);
        *fresh8 = bc_malloc(::std::mem::size_of::<bc_var>() as libc::c_ulong)
            as *mut bc_var;
        var_ptr = *fresh8;
        bc_init_num(&mut (*var_ptr).v_value);
    }
    return var_ptr;
}
pub unsafe extern "C" fn get_array_num(
    mut var_index: libc::c_int,
    mut idx: libc::c_ulong,
) -> *mut bc_num {
    let mut ary_ptr: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut a_var: *mut bc_array = 0 as *mut bc_array;
    let mut temp: *mut bc_array_node = 0 as *mut bc_array_node;
    let mut log: libc::c_int = 0;
    let mut ix: libc::c_uint = 0;
    let mut ix1: libc::c_uint = 0;
    let mut sub: [libc::c_int; 4] = [0; 4];
    ary_ptr = *arrays.offset(var_index as isize);
    if ary_ptr.is_null() {
        let ref mut fresh9 = *arrays.offset(var_index as isize);
        *fresh9 = bc_malloc(::std::mem::size_of::<bc_var_array>() as libc::c_ulong)
            as *mut bc_var_array;
        ary_ptr = *fresh9;
        (*ary_ptr).a_value = 0 as *mut bc_array;
        (*ary_ptr).a_next = 0 as *mut bc_var_array;
        (*ary_ptr).a_param = 0 as libc::c_int as libc::c_char;
    }
    a_var = (*ary_ptr).a_value;
    if a_var.is_null() {
        (*ary_ptr)
            .a_value = bc_malloc(::std::mem::size_of::<bc_array>() as libc::c_ulong)
            as *mut bc_array;
        a_var = (*ary_ptr).a_value;
        (*a_var).a_tree = 0 as *mut bc_array_node;
        (*a_var).a_depth = 0 as libc::c_int as libc::c_short;
    }
    sub[0 as libc::c_int
        as usize] = (idx & 0x3f as libc::c_int as libc::c_ulong) as libc::c_int;
    ix = (idx >> 6 as libc::c_int) as libc::c_uint;
    log = 1 as libc::c_int;
    while ix > 0 as libc::c_int as libc::c_uint || log < (*a_var).a_depth as libc::c_int
    {
        sub[log as usize] = (ix & 0x3f as libc::c_int as libc::c_uint) as libc::c_int;
        ix >>= 6 as libc::c_int;
        log += 1;
        log;
    }
    while log > (*a_var).a_depth as libc::c_int {
        temp = bc_malloc(::std::mem::size_of::<bc_array_node>() as libc::c_ulong)
            as *mut bc_array_node;
        if (*a_var).a_depth as libc::c_int != 0 as libc::c_int {
            (*temp).n_items.n_down[0 as libc::c_int as usize] = (*a_var).a_tree;
            ix = 1 as libc::c_int as libc::c_uint;
            while ix < 64 as libc::c_int as libc::c_uint {
                (*temp).n_items.n_down[ix as usize] = 0 as *mut bc_array_node;
                ix = ix.wrapping_add(1);
                ix;
            }
        } else {
            ix = 0 as libc::c_int as libc::c_uint;
            while ix < 64 as libc::c_int as libc::c_uint {
                (*temp).n_items.n_num[ix as usize] = bc_copy_num(_zero_);
                ix = ix.wrapping_add(1);
                ix;
            }
        }
        (*a_var).a_tree = temp;
        (*a_var).a_depth += 1;
        (*a_var).a_depth;
    }
    temp = (*a_var).a_tree;
    loop {
        let fresh10 = log;
        log = log - 1;
        if !(fresh10 > 1 as libc::c_int) {
            break;
        }
        ix1 = sub[log as usize] as libc::c_uint;
        if ((*temp).n_items.n_down[ix1 as usize]).is_null() {
            (*temp)
                .n_items
                .n_down[ix1
                as usize] = bc_malloc(
                ::std::mem::size_of::<bc_array_node>() as libc::c_ulong,
            ) as *mut bc_array_node;
            temp = (*temp).n_items.n_down[ix1 as usize];
            if log > 1 as libc::c_int {
                ix = 0 as libc::c_int as libc::c_uint;
                while ix < 64 as libc::c_int as libc::c_uint {
                    (*temp).n_items.n_down[ix as usize] = 0 as *mut bc_array_node;
                    ix = ix.wrapping_add(1);
                    ix;
                }
            } else {
                ix = 0 as libc::c_int as libc::c_uint;
                while ix < 64 as libc::c_int as libc::c_uint {
                    (*temp).n_items.n_num[ix as usize] = bc_copy_num(_zero_);
                    ix = ix.wrapping_add(1);
                    ix;
                }
            }
        } else {
            temp = (*temp).n_items.n_down[ix1 as usize];
        }
    }
    return &mut *((*temp).n_items.n_num)
        .as_mut_ptr()
        .offset(*sub.as_mut_ptr().offset(0 as libc::c_int as isize) as isize)
        as *mut bc_num;
}
pub unsafe extern "C" fn store_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    let mut temp: libc::c_long = 0;
    let mut toobig: libc::c_char = 0;
    if var_name > 3 as libc::c_int {
        var_ptr = get_var(var_name);
        if !var_ptr.is_null() {
            bc_free_num(&mut (*var_ptr).v_value);
            (*var_ptr).v_value = bc_copy_num((*ex_stack).s_num);
        }
    } else {
        toobig = 0 as libc::c_int as libc::c_char;
        temp = 0 as libc::c_int as libc::c_long;
        if bc_is_neg((*ex_stack).s_num) != 0 {
            match var_name {
                0 => {
                    rt_warn(
                        b"negative ibase, set to 2\0" as *const u8 as *const libc::c_char,
                    );
                    temp = 2 as libc::c_int as libc::c_long;
                }
                1 => {
                    rt_warn(
                        b"negative obase, set to 2\0" as *const u8 as *const libc::c_char,
                    );
                    temp = 2 as libc::c_int as libc::c_long;
                }
                2 => {
                    rt_warn(
                        b"negative scale, set to 0\0" as *const u8 as *const libc::c_char,
                    );
                    temp = 0 as libc::c_int as libc::c_long;
                }
                _ => {}
            }
        } else {
            temp = bc_num2long((*ex_stack).s_num);
            if bc_is_zero((*ex_stack).s_num) == 0
                && temp == 0 as libc::c_int as libc::c_long
            {
                toobig = 1 as libc::c_int as libc::c_char;
            }
        }
        match var_name {
            0 => {
                if temp < 2 as libc::c_int as libc::c_long && toobig == 0 {
                    i_base = 2 as libc::c_int;
                    rt_warn(
                        b"ibase too small, set to 2\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if temp > 16 as libc::c_int as libc::c_long
                    || toobig as libc::c_int != 0
                {
                    if std_only != 0 {
                        i_base = 16 as libc::c_int;
                        rt_warn(
                            b"ibase too large, set to 16\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if temp > 36 as libc::c_int as libc::c_long
                        || toobig as libc::c_int != 0
                    {
                        i_base = 36 as libc::c_int;
                        rt_warn(
                            b"ibase too large, set to 36\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        if temp >= 16 as libc::c_int as libc::c_long && warn_not_std != 0
                        {
                            rt_warn(
                                b"ibase larger than 16 is non-standard\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        i_base = temp as libc::c_int;
                    }
                } else {
                    i_base = temp as libc::c_int;
                }
            }
            1 => {
                if temp < 2 as libc::c_int as libc::c_long && toobig == 0 {
                    o_base = 2 as libc::c_int;
                    rt_warn(
                        b"obase too small, set to 2\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if temp > 2147483647 as libc::c_int as libc::c_long
                    || toobig as libc::c_int != 0
                {
                    o_base = 2147483647 as libc::c_int;
                    rt_warn(
                        b"obase too large, set to %d\0" as *const u8
                            as *const libc::c_char,
                        2147483647 as libc::c_int,
                    );
                } else {
                    o_base = temp as libc::c_int;
                }
            }
            2 => {
                if temp > 2147483647 as libc::c_int as libc::c_long
                    || toobig as libc::c_int != 0
                {
                    scale = 2147483647 as libc::c_int;
                    rt_warn(
                        b"scale too large, set to %d\0" as *const u8
                            as *const libc::c_char,
                        2147483647 as libc::c_int,
                    );
                } else {
                    scale = temp as libc::c_int;
                }
            }
            _ => {}
        }
    };
}
pub unsafe extern "C" fn store_array(mut var_name: libc::c_int) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut idx: libc::c_long = 0;
    if check_stack(2 as libc::c_int) == 0 {
        return;
    }
    idx = bc_num2long((*(*ex_stack).s_next).s_num);
    if idx < 0 as libc::c_int as libc::c_long
        || idx > 16777215 as libc::c_int as libc::c_long
        || idx == 0 as libc::c_int as libc::c_long
            && bc_is_zero((*(*ex_stack).s_next).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\0" as *const u8 as *const libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name, idx as libc::c_ulong);
        if !num_ptr.is_null() {
            bc_free_num(num_ptr);
            *num_ptr = bc_copy_num((*ex_stack).s_num);
            bc_free_num(&mut (*(*ex_stack).s_next).s_num);
            (*(*ex_stack).s_next).s_num = (*ex_stack).s_num;
            bc_init_num(&mut (*ex_stack).s_num);
            pop();
        }
    };
}
pub unsafe extern "C" fn load_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    match var_name {
        0 => {
            push_copy(_zero_);
            bc_int2num(&mut (*ex_stack).s_num, i_base);
        }
        1 => {
            push_copy(_zero_);
            bc_int2num(&mut (*ex_stack).s_num, o_base);
        }
        2 => {
            push_copy(_zero_);
            bc_int2num(&mut (*ex_stack).s_num, scale);
        }
        _ => {
            var_ptr = *variables.offset(var_name as isize);
            if !var_ptr.is_null() {
                push_copy((*var_ptr).v_value);
            } else {
                push_copy(_zero_);
            }
        }
    };
}
pub unsafe extern "C" fn load_array(mut var_name: libc::c_int) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut idx: libc::c_long = 0;
    if check_stack(1 as libc::c_int) == 0 {
        return;
    }
    idx = bc_num2long((*ex_stack).s_num);
    if idx < 0 as libc::c_int as libc::c_long
        || idx > 16777215 as libc::c_int as libc::c_long
        || idx == 0 as libc::c_int as libc::c_long && bc_is_zero((*ex_stack).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\0" as *const u8 as *const libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name, idx as libc::c_ulong);
        if !num_ptr.is_null() {
            pop();
            push_copy(*num_ptr);
        }
    };
}
pub unsafe extern "C" fn decr_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    match var_name {
        0 => {
            if i_base > 2 as libc::c_int {
                i_base -= 1;
                i_base;
            } else {
                rt_warn(b"ibase too small in --\0" as *const u8 as *const libc::c_char);
            }
        }
        1 => {
            if o_base > 2 as libc::c_int {
                o_base -= 1;
                o_base;
            } else {
                rt_warn(b"obase too small in --\0" as *const u8 as *const libc::c_char);
            }
        }
        2 => {
            if scale > 0 as libc::c_int {
                scale -= 1;
                scale;
            } else {
                rt_warn(
                    b"scale can not be negative in -- \0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        _ => {
            var_ptr = get_var(var_name);
            if !var_ptr.is_null() {
                bc_sub(
                    (*var_ptr).v_value,
                    _one_,
                    &mut (*var_ptr).v_value,
                    0 as libc::c_int,
                );
            }
        }
    };
}
pub unsafe extern "C" fn decr_array(mut var_name: libc::c_int) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut idx: libc::c_long = 0;
    if check_stack(1 as libc::c_int) == 0 {
        return;
    }
    idx = bc_num2long((*ex_stack).s_num);
    if idx < 0 as libc::c_int as libc::c_long
        || idx > 16777215 as libc::c_int as libc::c_long
        || idx == 0 as libc::c_int as libc::c_long && bc_is_zero((*ex_stack).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\0" as *const u8 as *const libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name, idx as libc::c_ulong);
        if !num_ptr.is_null() {
            pop();
            bc_sub(*num_ptr, _one_, num_ptr, 0 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn incr_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    match var_name {
        0 => {
            if i_base < 16 as libc::c_int {
                i_base += 1;
                i_base;
            } else {
                rt_warn(b"ibase too big in ++\0" as *const u8 as *const libc::c_char);
            }
        }
        1 => {
            if o_base < 2147483647 as libc::c_int {
                o_base += 1;
                o_base;
            } else {
                rt_warn(b"obase too big in ++\0" as *const u8 as *const libc::c_char);
            }
        }
        2 => {
            if scale < 2147483647 as libc::c_int {
                scale += 1;
                scale;
            } else {
                rt_warn(b"Scale too big in ++\0" as *const u8 as *const libc::c_char);
            }
        }
        _ => {
            var_ptr = get_var(var_name);
            if !var_ptr.is_null() {
                bc_add(
                    (*var_ptr).v_value,
                    _one_,
                    &mut (*var_ptr).v_value,
                    0 as libc::c_int,
                );
            }
        }
    };
}
pub unsafe extern "C" fn incr_array(mut var_name: libc::c_int) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut idx: libc::c_long = 0;
    if check_stack(1 as libc::c_int) == 0 {
        return;
    }
    idx = bc_num2long((*ex_stack).s_num);
    if idx < 0 as libc::c_int as libc::c_long
        || idx > 16777215 as libc::c_int as libc::c_long
        || idx == 0 as libc::c_int as libc::c_long && bc_is_zero((*ex_stack).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\0" as *const u8 as *const libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name, idx as libc::c_ulong);
        if !num_ptr.is_null() {
            pop();
            bc_add(*num_ptr, _one_, num_ptr, 0 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn auto_var(mut name: libc::c_int) {
    let mut v_temp: *mut bc_var = 0 as *mut bc_var;
    let mut a_temp: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut ix: libc::c_int = 0;
    if name > 0 as libc::c_int {
        ix = name;
        v_temp = bc_malloc(::std::mem::size_of::<bc_var>() as libc::c_ulong)
            as *mut bc_var;
        (*v_temp).v_next = *variables.offset(ix as isize);
        bc_init_num(&mut (*v_temp).v_value);
        let ref mut fresh11 = *variables.offset(ix as isize);
        *fresh11 = v_temp;
    } else {
        ix = -name;
        a_temp = bc_malloc(::std::mem::size_of::<bc_var_array>() as libc::c_ulong)
            as *mut bc_var_array;
        (*a_temp).a_next = *arrays.offset(ix as isize);
        (*a_temp).a_value = 0 as *mut bc_array;
        (*a_temp).a_param = 0 as libc::c_int as libc::c_char;
        let ref mut fresh12 = *arrays.offset(ix as isize);
        *fresh12 = a_temp;
    };
}
pub unsafe extern "C" fn free_a_tree(
    mut root: *mut bc_array_node,
    mut depth: libc::c_int,
) {
    let mut ix: libc::c_int = 0;
    if !root.is_null() {
        if depth > 1 as libc::c_int {
            ix = 0 as libc::c_int;
            while ix < 64 as libc::c_int {
                free_a_tree(
                    (*root).n_items.n_down[ix as usize],
                    depth - 1 as libc::c_int,
                );
                ix += 1;
                ix;
            }
        } else {
            ix = 0 as libc::c_int;
            while ix < 64 as libc::c_int {
                bc_free_num(
                    &mut *((*root).n_items.n_num).as_mut_ptr().offset(ix as isize),
                );
                ix += 1;
                ix;
            }
        }
        free(root as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn pop_vars(mut list: *mut arg_list) {
    let mut v_temp: *mut bc_var = 0 as *mut bc_var;
    let mut a_temp: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut ix: libc::c_int = 0;
    while !list.is_null() {
        ix = (*list).av_name;
        if ix > 0 as libc::c_int {
            v_temp = *variables.offset(ix as isize);
            if !v_temp.is_null() {
                let ref mut fresh13 = *variables.offset(ix as isize);
                *fresh13 = (*v_temp).v_next;
                bc_free_num(&mut (*v_temp).v_value);
                free(v_temp as *mut libc::c_void);
            }
        } else {
            ix = -ix;
            a_temp = *arrays.offset(ix as isize);
            if !a_temp.is_null() {
                let ref mut fresh14 = *arrays.offset(ix as isize);
                *fresh14 = (*a_temp).a_next;
                if (*a_temp).a_param == 0 && !((*a_temp).a_value).is_null() {
                    free_a_tree(
                        (*(*a_temp).a_value).a_tree,
                        (*(*a_temp).a_value).a_depth as libc::c_int,
                    );
                    free((*a_temp).a_value as *mut libc::c_void);
                }
                free(a_temp as *mut libc::c_void);
            }
        }
        list = (*list).next;
    }
}
unsafe extern "C" fn copy_tree(
    mut ary_node: *mut bc_array_node,
    mut depth: libc::c_int,
) -> *mut bc_array_node {
    let mut res: *mut bc_array_node = bc_malloc(
        ::std::mem::size_of::<bc_array_node>() as libc::c_ulong,
    ) as *mut bc_array_node;
    let mut i: libc::c_int = 0;
    if depth > 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if !((*ary_node).n_items.n_down[i as usize]).is_null() {
                (*res)
                    .n_items
                    .n_down[i
                    as usize] = copy_tree(
                    (*ary_node).n_items.n_down[i as usize],
                    depth - 1 as libc::c_int,
                );
            } else {
                (*res).n_items.n_down[i as usize] = 0 as *mut bc_array_node;
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if !((*ary_node).n_items.n_num[i as usize]).is_null() {
                (*res)
                    .n_items
                    .n_num[i
                    as usize] = bc_copy_num((*ary_node).n_items.n_num[i as usize]);
            } else {
                (*res).n_items.n_num[i as usize] = 0 as bc_num;
            }
            i += 1;
            i;
        }
    }
    return res;
}
unsafe extern "C" fn copy_array(mut ary: *mut bc_array) -> *mut bc_array {
    let mut res: *mut bc_array = bc_malloc(
        ::std::mem::size_of::<bc_array>() as libc::c_ulong,
    ) as *mut bc_array;
    (*res).a_depth = (*ary).a_depth;
    (*res).a_tree = copy_tree((*ary).a_tree, (*ary).a_depth as libc::c_int);
    return res;
}
pub unsafe extern "C" fn process_params(
    mut progctr: *mut program_counter,
    mut func: libc::c_int,
) {
    let mut ch: libc::c_char = 0;
    let mut params: *mut arg_list = 0 as *mut arg_list;
    let mut ix: libc::c_int = 0;
    let mut ix1: libc::c_int = 0;
    let mut v_temp: *mut bc_var = 0 as *mut bc_var;
    let mut a_src: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut a_dest: *mut bc_var_array = 0 as *mut bc_var_array;
    params = (*functions.offset(func as isize)).f_params;
    loop {
        ch = byte(progctr) as libc::c_char;
        if !(ch as libc::c_int != ':' as i32) {
            break;
        }
        if !params.is_null() {
            if ch as libc::c_int == '0' as i32 && (*params).av_name > 0 as libc::c_int {
                ix = (*params).av_name;
                v_temp = bc_malloc(::std::mem::size_of::<bc_var>() as libc::c_ulong)
                    as *mut bc_var;
                (*v_temp).v_next = *variables.offset(ix as isize);
                (*v_temp).v_value = (*ex_stack).s_num;
                bc_init_num(&mut (*ex_stack).s_num);
                let ref mut fresh15 = *variables.offset(ix as isize);
                *fresh15 = v_temp;
            } else if ch as libc::c_int == '1' as i32
                && (*params).av_name < 0 as libc::c_int
            {
                ix = bc_num2long((*ex_stack).s_num) as libc::c_int;
                get_array_num(ix, 0 as libc::c_int as libc::c_ulong);
                auto_var((*params).av_name);
                ix1 = -(*params).av_name;
                if ix == ix1 {
                    a_src = (**arrays.offset(ix as isize)).a_next;
                } else {
                    a_src = *arrays.offset(ix as isize);
                }
                a_dest = *arrays.offset(ix1 as isize);
                if (*params).arg_is_var != 0 {
                    (*a_dest).a_param = 1 as libc::c_int as libc::c_char;
                    (*a_dest).a_value = (*a_src).a_value;
                } else {
                    (*a_dest).a_param = 0 as libc::c_int as libc::c_char;
                    (*a_dest).a_value = copy_array((*a_src).a_value);
                }
            } else {
                if (*params).av_name < 0 as libc::c_int {
                    rt_error(
                        b"Parameter type mismatch parameter %s.\0" as *const u8
                            as *const libc::c_char,
                        *a_names.offset(-(*params).av_name as isize),
                    );
                } else {
                    rt_error(
                        b"Parameter type mismatch, parameter %s.\0" as *const u8
                            as *const libc::c_char,
                        *v_names.offset((*params).av_name as isize),
                    );
                }
                params = params.offset(1);
                params;
            }
            pop();
        } else {
            rt_error(b"Parameter number mismatch\0" as *const u8 as *const libc::c_char);
            return;
        }
        params = (*params).next;
    }
    if !params.is_null() {
        rt_error(b"Parameter number mismatch\0" as *const u8 as *const libc::c_char);
    }
}
