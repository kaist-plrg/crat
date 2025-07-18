use ::libc;
extern "C" {
    fn fs_get_string_by_index(
        fs: *mut fieldset_t,
        index: libc::c_int,
    ) -> *mut libc::c_char;
    fn fs_get_uint64_by_index(fs: *mut fieldset_t, index: libc::c_int) -> uint64_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_def {
    pub name: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub desc: *const libc::c_char,
}
pub type fielddef_t = field_def;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fielddef_set {
    pub fielddefs: [fielddef_t; 128],
    pub len: libc::c_int,
}
pub type fielddefset_t = fielddef_set;
#[derive(Copy, Clone)]
#[repr(C)]
pub union field_val {
    pub ptr: *mut libc::c_void,
    pub num: uint64_t,
}
pub type field_val_t = field_val;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
    pub len: size_t,
    pub value: field_val_t,
}
pub type field_t = field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fieldset {
    pub len: libc::c_int,
    pub fields: [field_t; 128],
    pub fds: *mut fielddefset_t,
    pub inner_type: libc::c_int,
    pub type_0: libc::c_int,
    pub free_: libc::c_int,
}
pub type fieldset_t = fieldset;
pub type operation = libc::c_uint;
pub const GT_EQ: operation = 7;
pub const LT_EQ: operation = 6;
pub const OR: operation = 5;
pub const AND: operation = 4;
pub const NEQ: operation = 3;
pub const EQ: operation = 2;
pub const LT: operation = 1;
pub const GT: operation = 0;
pub type node_type = libc::c_uint;
pub const INT: node_type = 3;
pub const STRING: node_type = 2;
pub const FIELD: node_type = 1;
pub const OP: node_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_id {
    pub index: libc::c_int,
    pub fieldname: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union node_value {
    pub field: field_id,
    pub string_literal: *mut libc::c_char,
    pub int_literal: uint64_t,
    pub op: operation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub left_child: *mut node_st,
    pub right_child: *mut node_st,
    pub type_0: node_type,
    pub value: node_value,
}
pub type node_t = node_st;
unsafe extern "C" fn alloc_node() -> *mut node_t {
    let mut node: *mut node_t = xmalloc(::std::mem::size_of::<node_t>() as libc::c_ulong)
        as *mut node_t;
    return node;
}
unsafe extern "C" fn eval_gt_node(
    mut node: *mut node_t,
    mut fields: *mut fieldset_t,
) -> libc::c_int {
    let mut index: libc::c_int = (*(*node).left_child).value.field.index;
    let mut expected: uint64_t = (*(*node).right_child).value.int_literal;
    let mut actual: uint64_t = fs_get_uint64_by_index(fields, index);
    return (actual > expected) as libc::c_int;
}
unsafe extern "C" fn eval_lt_node(
    mut node: *mut node_t,
    mut fields: *mut fieldset_t,
) -> libc::c_int {
    let mut index: libc::c_int = (*(*node).left_child).value.field.index;
    let mut expected: uint64_t = (*(*node).right_child).value.int_literal;
    let mut actual: uint64_t = fs_get_uint64_by_index(fields, index);
    return (actual < expected) as libc::c_int;
}
unsafe extern "C" fn eval_eq_node(
    mut node: *mut node_t,
    mut fields: *mut fieldset_t,
) -> libc::c_int {
    let mut literal: *mut node_t = (*node).right_child;
    let mut index: libc::c_int = (*(*node).left_child).value.field.index;
    let mut expected: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut actual: *mut libc::c_char = 0 as *mut libc::c_char;
    match (*literal).type_0 as libc::c_uint {
        2 => {
            expected = (*literal).value.string_literal;
            actual = fs_get_string_by_index(fields, index);
            return (strcmp(expected, actual) == 0 as libc::c_int) as libc::c_int;
        }
        3 => {
            return (fs_get_uint64_by_index(fields, index)
                == (*literal).value.int_literal) as libc::c_int;
        }
        _ => {
            printf(b"wat\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn eval_lt_eq_node(
    mut node: *mut node_t,
    mut fields: *mut fieldset_t,
) -> libc::c_int {
    return (eval_gt_node(node, fields) == 0) as libc::c_int;
}
unsafe extern "C" fn eval_gt_eq_node(
    mut node: *mut node_t,
    mut fields: *mut fieldset_t,
) -> libc::c_int {
    return (eval_lt_node(node, fields) == 0) as libc::c_int;
}
pub unsafe extern "C" fn make_op_node(mut op: operation) -> *mut node_t {
    let mut node: *mut node_t = alloc_node();
    (*node).type_0 = OP;
    (*node).value.op = op;
    return node;
}
pub unsafe extern "C" fn make_field_node(
    mut fieldname: *mut libc::c_char,
) -> *mut node_t {
    let mut node: *mut node_t = alloc_node();
    (*node).type_0 = FIELD;
    (*node).value.field.fieldname = fieldname;
    return node;
}
pub unsafe extern "C" fn make_string_node(
    mut literal: *mut libc::c_char,
) -> *mut node_t {
    let mut node: *mut node_t = alloc_node();
    (*node).type_0 = STRING;
    (*node).value.string_literal = literal;
    return node;
}
pub unsafe extern "C" fn make_int_node(mut literal: libc::c_int) -> *mut node_t {
    let mut node: *mut node_t = alloc_node();
    (*node).type_0 = INT;
    (*node).value.int_literal = literal as uint64_t;
    return node;
}
pub unsafe extern "C" fn evaluate_expression(
    mut root: *mut node_t,
    mut fields: *mut fieldset_t,
) -> libc::c_int {
    if root.is_null() {
        return 1 as libc::c_int;
    }
    match (*root).type_0 as libc::c_uint {
        1 | 2 | 3 => return 1 as libc::c_int,
        0 | _ => {}
    }
    match (*root).value.op as libc::c_uint {
        0 => return eval_gt_node(root, fields),
        1 => return eval_lt_node(root, fields),
        2 => return eval_eq_node(root, fields),
        3 => return (eval_eq_node(root, fields) == 0) as libc::c_int,
        6 => return eval_lt_eq_node(root, fields),
        7 => return eval_gt_eq_node(root, fields),
        4 => {
            return (evaluate_expression((*root).left_child, fields) != 0
                && evaluate_expression((*root).right_child, fields) != 0) as libc::c_int;
        }
        5 => {
            return (evaluate_expression((*root).left_child, fields) != 0
                || evaluate_expression((*root).right_child, fields) != 0) as libc::c_int;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn print_expression(mut root: *mut node_t) {
    if root.is_null() {
        return;
    }
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        b"( \0" as *const u8 as *const libc::c_char,
    );
    print_expression((*root).left_child);
    match (*root).type_0 as libc::c_uint {
        0 => {
            printf(
                b" %i \0" as *const u8 as *const libc::c_char,
                (*root).value.op as libc::c_uint,
            );
        }
        1 => {
            printf(
                b" (%s\0" as *const u8 as *const libc::c_char,
                (*root).value.field.fieldname,
            );
        }
        2 => {
            printf(
                b"%s) \0" as *const u8 as *const libc::c_char,
                (*root).value.string_literal,
            );
        }
        3 => {
            printf(
                b" %llu) \0" as *const u8 as *const libc::c_char,
                (*root).value.int_literal as libc::c_ulonglong,
            );
        }
        _ => {}
    }
    print_expression((*root).right_child);
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        b" )\0" as *const u8 as *const libc::c_char,
    );
}
