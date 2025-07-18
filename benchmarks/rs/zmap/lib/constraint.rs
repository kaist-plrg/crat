use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn log_debug(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _constraint {
    pub root: *mut node_t,
    pub radix: *mut uint32_t,
    pub radix_len: size_t,
    pub painted: libc::c_int,
    pub paint_value: value_t,
}
pub type value_t = uint32_t;
pub type node_t = node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub l: *mut node,
    pub r: *mut node,
    pub value: value_t,
    pub count: uint64_t,
}
pub type constraint_t = _constraint;
unsafe extern "C" fn _create_leaf(mut value: value_t) -> *mut node_t {
    let mut node: *mut node_t = xmalloc(::std::mem::size_of::<node_t>() as libc::c_ulong)
        as *mut node_t;
    (*node).l = 0 as *mut node;
    (*node).r = 0 as *mut node;
    (*node).value = value;
    return node;
}
unsafe extern "C" fn _destroy_subtree(mut node: *mut node_t) {
    if node.is_null() {
        return;
    }
    _destroy_subtree((*node).l);
    _destroy_subtree((*node).r);
    free(node as *mut libc::c_void);
}
unsafe extern "C" fn _convert_to_leaf(mut node: *mut node_t) {
    if !node.is_null() {} else {
        __assert_fail(
            b"node\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void _convert_to_leaf(node_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3194: {
        if !node.is_null() {} else {
            __assert_fail(
                b"node\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void _convert_to_leaf(node_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*node).l).is_null() {} else {
        __assert_fail(
            b"!IS_LEAF(node)\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void _convert_to_leaf(node_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3140: {
        if !((*node).l).is_null() {} else {
            __assert_fail(
                b"!IS_LEAF(node)\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void _convert_to_leaf(node_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    _destroy_subtree((*node).l);
    _destroy_subtree((*node).r);
    (*node).l = 0 as *mut node;
    (*node).r = 0 as *mut node;
}
unsafe extern "C" fn _set_recurse(
    mut node: *mut node_t,
    mut prefix: uint32_t,
    mut len: libc::c_int,
    mut value: value_t,
) {
    if !node.is_null() {} else {
        __assert_fail(
            b"node\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void _set_recurse(node_t *, uint32_t, int, value_t)\0"))
                .as_ptr(),
        );
    }
    'c_3447: {
        if !node.is_null() {} else {
            __assert_fail(
                b"node\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void _set_recurse(node_t *, uint32_t, int, value_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int <= len && len <= 256 as libc::c_int {} else {
        __assert_fail(
            b"0 <= len && len <= 256\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void _set_recurse(node_t *, uint32_t, int, value_t)\0"))
                .as_ptr(),
        );
    }
    'c_3399: {
        if 0 as libc::c_int <= len && len <= 256 as libc::c_int {} else {
            __assert_fail(
                b"0 <= len && len <= 256\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                126 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void _set_recurse(node_t *, uint32_t, int, value_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if len == 0 as libc::c_int {
        if !((*node).l).is_null() {
            _convert_to_leaf(node);
        }
        (*node).value = value;
        return;
    }
    if ((*node).l).is_null() {
        if (*node).value == value {
            return;
        }
        (*node).l = _create_leaf((*node).value);
        (*node).r = _create_leaf((*node).value);
    }
    if prefix & 0x80000000 as libc::c_uint != 0 {
        _set_recurse(
            (*node).r,
            prefix << 1 as libc::c_int,
            len - 1 as libc::c_int,
            value,
        );
    } else {
        _set_recurse(
            (*node).l,
            prefix << 1 as libc::c_int,
            len - 1 as libc::c_int,
            value,
        );
    }
    if ((*(*node).r).l).is_null() && ((*(*node).l).l).is_null()
        && (*(*node).r).value == (*(*node).l).value
    {
        (*node).value = (*(*node).l).value;
        _convert_to_leaf(node);
    }
}
pub unsafe extern "C" fn constraint_set(
    mut con: *mut constraint_t,
    mut prefix: uint32_t,
    mut len: libc::c_int,
    mut value: value_t,
) {
    if !con.is_null() {} else {
        __assert_fail(
            b"con\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void constraint_set(constraint_t *, uint32_t, int, value_t)\0"))
                .as_ptr(),
        );
    }
    'c_3479: {
        if !con.is_null() {} else {
            __assert_fail(
                b"con\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void constraint_set(constraint_t *, uint32_t, int, value_t)\0"))
                    .as_ptr(),
            );
        }
    };
    _set_recurse((*con).root, prefix, len, value);
    (*con).painted = 0 as libc::c_int;
}
unsafe extern "C" fn _lookup_ip(
    mut root: *mut node_t,
    mut address: uint32_t,
) -> libc::c_int {
    if !root.is_null() {} else {
        __assert_fail(
            b"root\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int _lookup_ip(node_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_3590: {
        if !root.is_null() {} else {
            __assert_fail(
                b"root\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                183 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int _lookup_ip(node_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut node: *mut node_t = root;
    let mut mask: uint32_t = 0x80000000 as libc::c_uint;
    loop {
        if ((*node).l).is_null() {
            return (*node).value as libc::c_int;
        }
        if address & mask != 0 {
            node = (*node).r;
        } else {
            node = (*node).l;
        }
        mask >>= 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn constraint_lookup_ip(
    mut con: *mut constraint_t,
    mut address: uint32_t,
) -> value_t {
    if !con.is_null() {} else {
        __assert_fail(
            b"con\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"value_t constraint_lookup_ip(constraint_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_3623: {
        if !con.is_null() {} else {
            __assert_fail(
                b"con\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"value_t constraint_lookup_ip(constraint_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return _lookup_ip((*con).root, address) as value_t;
}
unsafe extern "C" fn _lookup_index(
    mut root: *mut node_t,
    mut n: uint64_t,
) -> libc::c_int {
    if !root.is_null() {} else {
        __assert_fail(
            b"root\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"int _lookup_index(node_t *, uint64_t)\0"))
                .as_ptr(),
        );
    }
    'c_4006: {
        if !root.is_null() {} else {
            __assert_fail(
                b"root\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"int _lookup_index(node_t *, uint64_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut node: *mut node_t = root;
    let mut ip: uint32_t = 0 as libc::c_int as uint32_t;
    let mut mask: uint32_t = 0x80000000 as libc::c_uint;
    loop {
        if ((*node).l).is_null() {
            return (ip as libc::c_ulong | n) as libc::c_int;
        }
        if n < (*(*node).l).count {
            node = (*node).l;
        } else {
            n = (n as libc::c_ulong).wrapping_sub((*(*node).l).count) as uint64_t
                as uint64_t;
            node = (*node).r;
            ip |= mask;
        }
        mask >>= 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn constraint_lookup_index(
    mut con: *mut constraint_t,
    mut index: uint64_t,
    mut value: value_t,
) -> uint32_t {
    if !con.is_null() {} else {
        __assert_fail(
            b"con\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"uint32_t constraint_lookup_index(constraint_t *, uint64_t, value_t)\0"))
                .as_ptr(),
        );
    }
    'c_4515: {
        if !con.is_null() {} else {
            __assert_fail(
                b"con\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                236 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"uint32_t constraint_lookup_index(constraint_t *, uint64_t, value_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*con).painted == 0 || (*con).paint_value != value {
        constraint_paint_value(con, value);
    }
    let mut radix_idx: uint64_t = index
        .wrapping_div(
            ((1 as libc::c_int) << 32 as libc::c_int - 20 as libc::c_int)
                as libc::c_ulong,
        );
    if radix_idx < (*con).radix_len {
        let mut radix_offset: uint32_t = index
            .wrapping_rem(
                ((1 as libc::c_int) << 32 as libc::c_int - 20 as libc::c_int)
                    as libc::c_ulong,
            ) as uint32_t;
        return *((*con).radix).offset(radix_idx as isize) | radix_offset;
    }
    index = (index as libc::c_ulong)
        .wrapping_sub(
            ((*con).radix_len)
                .wrapping_mul(
                    ((1 as libc::c_int) << 32 as libc::c_int - 20 as libc::c_int)
                        as libc::c_ulong,
                ),
        ) as uint64_t as uint64_t;
    if index < (*(*con).root).count {} else {
        __assert_fail(
            b"index < con->root->count\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"uint32_t constraint_lookup_index(constraint_t *, uint64_t, value_t)\0"))
                .as_ptr(),
        );
    }
    'c_4039: {
        if index < (*(*con).root).count {} else {
            __assert_fail(
                b"index < con->root->count\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                253 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"uint32_t constraint_lookup_index(constraint_t *, uint64_t, value_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return _lookup_index((*con).root, index) as uint32_t;
}
unsafe extern "C" fn _count_ips_recurse(
    mut node: *mut node_t,
    mut value: value_t,
    mut size: uint64_t,
    mut paint: libc::c_int,
    mut exclude_radix: libc::c_int,
) -> uint64_t {
    if !node.is_null() {} else {
        __assert_fail(
            b"node\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"uint64_t _count_ips_recurse(node_t *, value_t, uint64_t, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_3801: {
        if !node.is_null() {} else {
            __assert_fail(
                b"node\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                266 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"uint64_t _count_ips_recurse(node_t *, value_t, uint64_t, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut n: uint64_t = 0;
    if ((*node).l).is_null() {
        if (*node).value == value {
            n = size;
            if exclude_radix != 0
                && size
                    >= ((1 as libc::c_int) << 32 as libc::c_int - 20 as libc::c_int)
                        as libc::c_ulong
            {
                n = 0 as libc::c_int as uint64_t;
            }
        } else {
            n = 0 as libc::c_int as uint64_t;
        }
    } else {
        n = (_count_ips_recurse(
            (*node).l,
            value,
            size >> 1 as libc::c_int,
            paint,
            exclude_radix,
        ))
            .wrapping_add(
                _count_ips_recurse(
                    (*node).r,
                    value,
                    size >> 1 as libc::c_int,
                    paint,
                    exclude_radix,
                ),
            );
    }
    if paint != 0 {
        (*node).count = n;
    }
    return n;
}
unsafe extern "C" fn _lookup_node(
    mut root: *mut node_t,
    mut prefix: uint32_t,
    mut len: libc::c_int,
) -> *mut node_t {
    if !root.is_null() {} else {
        __assert_fail(
            b"root\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"node_t *_lookup_node(node_t *, uint32_t, int)\0"))
                .as_ptr(),
        );
    }
    'c_4385: {
        if !root.is_null() {} else {
            __assert_fail(
                b"root\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                297 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"node_t *_lookup_node(node_t *, uint32_t, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int <= len && len <= 32 as libc::c_int {} else {
        __assert_fail(
            b"0 <= len && len <= 32\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"node_t *_lookup_node(node_t *, uint32_t, int)\0"))
                .as_ptr(),
        );
    }
    'c_4337: {
        if 0 as libc::c_int <= len && len <= 32 as libc::c_int {} else {
            __assert_fail(
                b"0 <= len && len <= 32\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"node_t *_lookup_node(node_t *, uint32_t, int)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut node: *mut node_t = root;
    let mut mask: uint32_t = 0x80000000 as libc::c_uint;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        if ((*node).l).is_null() {
            return node;
        }
        if prefix & mask != 0 {
            node = (*node).r;
        } else {
            node = (*node).l;
        }
        mask >>= 1 as libc::c_int;
        i += 1;
        i;
    }
    return node;
}
pub unsafe extern "C" fn constraint_paint_value(
    mut con: *mut constraint_t,
    mut value: value_t,
) {
    if !con.is_null() {} else {
        __assert_fail(
            b"con\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[libc::c_char; 53],
            >(b"void constraint_paint_value(constraint_t *, value_t)\0"))
                .as_ptr(),
        );
    }
    'c_4470: {
        if !con.is_null() {} else {
            __assert_fail(
                b"con\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void constraint_paint_value(constraint_t *, value_t)\0"))
                    .as_ptr(),
            );
        }
    };
    log_debug(
        b"constraint\0" as *const u8 as *const libc::c_char,
        b"Painting value %lu\0" as *const u8 as *const libc::c_char,
        value,
    );
    _count_ips_recurse(
        (*con).root,
        value,
        (1 as libc::c_int as uint64_t) << 32 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: uint32_t = 0;
    (*con).radix_len = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as uint32_t;
    while i < ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint {
        let mut prefix: uint32_t = i << 32 as libc::c_int - 20 as libc::c_int;
        let mut node: *mut node_t = _lookup_node((*con).root, prefix, 20 as libc::c_int);
        if ((*node).l).is_null() && (*node).value == value {
            let fresh0 = (*con).radix_len;
            (*con).radix_len = ((*con).radix_len).wrapping_add(1);
            *((*con).radix).offset(fresh0 as isize) = prefix;
        }
        i = i.wrapping_add(1);
        i;
    }
    log_debug(
        b"constraint\0" as *const u8 as *const libc::c_char,
        b"%lu IPs in radix array, %lu IPs in tree\0" as *const u8 as *const libc::c_char,
        ((*con).radix_len)
            .wrapping_mul(
                ((1 as libc::c_int) << 32 as libc::c_int - 20 as libc::c_int)
                    as libc::c_ulong,
            ),
        (*(*con).root).count,
    );
    (*con).painted = 1 as libc::c_int;
    (*con).paint_value = value;
}
pub unsafe extern "C" fn constraint_count_ips(
    mut con: *mut constraint_t,
    mut value: value_t,
) -> uint64_t {
    if !con.is_null() {} else {
        __assert_fail(
            b"con\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"uint64_t constraint_count_ips(constraint_t *, value_t)\0"))
                .as_ptr(),
        );
    }
    'c_3866: {
        if !con.is_null() {} else {
            __assert_fail(
                b"con\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                349 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"uint64_t constraint_count_ips(constraint_t *, value_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*con).painted != 0 && (*con).paint_value == value {
        return ((*(*con).root).count)
            .wrapping_add(
                ((*con).radix_len)
                    .wrapping_mul(
                        ((1 as libc::c_int) << 32 as libc::c_int - 20 as libc::c_int)
                            as libc::c_ulong,
                    ),
            )
    } else {
        return _count_ips_recurse(
            (*con).root,
            value,
            (1 as libc::c_int as uint64_t) << 32 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        )
    };
}
pub unsafe extern "C" fn constraint_init(mut value: value_t) -> *mut constraint_t {
    let mut con: *mut constraint_t = xmalloc(
        ::std::mem::size_of::<constraint_t>() as libc::c_ulong,
    ) as *mut constraint_t;
    (*con).root = _create_leaf(value);
    (*con)
        .radix = xcalloc(
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        ((1 as libc::c_int) << 20 as libc::c_int) as size_t,
    ) as *mut uint32_t;
    (*con).painted = 0 as libc::c_int;
    return con;
}
pub unsafe extern "C" fn constraint_free(mut con: *mut constraint_t) {
    if !con.is_null() {} else {
        __assert_fail(
            b"con\0" as *const u8 as *const libc::c_char,
            b"constraint.c\0" as *const u8 as *const libc::c_char,
            373 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void constraint_free(constraint_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3013: {
        if !con.is_null() {} else {
            __assert_fail(
                b"con\0" as *const u8 as *const libc::c_char,
                b"constraint.c\0" as *const u8 as *const libc::c_char,
                373 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void constraint_free(constraint_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    log_debug(
        b"constraint\0" as *const u8 as *const libc::c_char,
        b"Cleaning up\0" as *const u8 as *const libc::c_char,
    );
    _destroy_subtree((*con).root);
    free((*con).radix as *mut libc::c_void);
    free(con as *mut libc::c_void);
}
