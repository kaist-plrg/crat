use ::libc;
extern "C" {
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn random() -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn rbt_comp_ttl_gt(
        v1: *mut libc::c_void,
        v2: *mut libc::c_void,
        argv: *mut libc::c_void,
    ) -> libc::c_int;
    fn dns_error(_: libc::c_int, _: *mut libc::c_char);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type ushort = libc::c_ushort;
pub type uint = libc::c_uint;
pub type pthread_spinlock_t = libc::c_int;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uchar = libc::c_uchar;
pub type hashval_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _packet_type {
    pub label_count: uint8_t,
    pub domain: [uchar; 256],
    pub label: [*mut uint8_t; 64],
    pub label_offsets: [uint8_t; 64],
    pub label_len: [uint8_t; 64],
    pub hash: [hashval_t; 64],
}
pub type packet_type = _packet_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttlnode {
    pub exp: uint,
    pub dlen: ushort,
    pub type_0: ushort,
    pub hash: *mut hashval_t,
    pub lowerdomain: *mut packet_type,
    pub data: *mut uchar,
}
pub type comprbt = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub parent: *mut rbnode,
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub color: libc::c_int,
    pub key: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub root: *mut rbnode,
    pub nil: rbnode,
    pub lock: pthread_spinlock_t,
    pub size: uint,
    pub c: Option::<comprbt>,
    pub argv: *mut libc::c_void,
}
unsafe extern "C" fn left_rotate(mut rbt: *mut rbtree, mut node: *mut rbnode) {
    let mut tmp: *mut rbnode = (*node).right;
    (*node).right = (*tmp).left;
    if (*tmp).left != &mut (*rbt).nil as *mut rbnode {
        (*(*tmp).left).parent = node;
    }
    (*tmp).parent = (*node).parent;
    if (*node).parent == &mut (*rbt).nil as *mut rbnode {
        (*rbt).root = tmp;
    } else if node == (*(*node).parent).left {
        (*(*node).parent).left = tmp;
    } else {
        (*(*node).parent).right = tmp;
    }
    (*tmp).left = node;
    (*node).parent = tmp;
}
unsafe extern "C" fn right_rotate(mut rbt: *mut rbtree, mut node: *mut rbnode) {
    let mut tmp: *mut rbnode = (*node).left;
    (*node).left = (*tmp).right;
    if (*tmp).right != &mut (*rbt).nil as *mut rbnode {
        (*(*tmp).right).parent = node;
    }
    (*tmp).parent = (*node).parent;
    if (*node).parent == &mut (*rbt).nil as *mut rbnode {
        (*rbt).root = tmp;
    } else if node == (*(*node).parent).left {
        (*(*node).parent).left = tmp;
    } else {
        (*(*node).parent).right = tmp;
    }
    (*tmp).right = node;
    (*node).parent = tmp;
}
unsafe extern "C" fn insert_fixup(mut rbt: *mut rbtree, mut nd: *mut rbnode) {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    while (*(*nd).parent).color == 1 as libc::c_int {
        if (*nd).parent == (*(*(*nd).parent).parent).left {
            tmp = (*(*(*nd).parent).parent).right;
            if (*tmp).color == 1 as libc::c_int {
                (*tmp).color = 0 as libc::c_int;
                (*(*nd).parent).color = (*tmp).color;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                nd = (*(*nd).parent).parent;
            } else {
                if nd == (*(*nd).parent).right {
                    nd = (*nd).parent;
                    left_rotate(rbt, nd);
                }
                (*(*nd).parent).color = 0 as libc::c_int;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                right_rotate(rbt, (*(*nd).parent).parent);
            }
        } else {
            tmp = (*(*(*nd).parent).parent).left;
            if (*tmp).color == 1 as libc::c_int {
                (*tmp).color = 0 as libc::c_int;
                (*(*nd).parent).color = (*tmp).color;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                nd = (*(*nd).parent).parent;
            } else {
                if nd == (*(*nd).parent).left {
                    nd = (*nd).parent;
                    right_rotate(rbt, nd);
                }
                (*(*nd).parent).color = 0 as libc::c_int;
                (*(*(*nd).parent).parent).color = 1 as libc::c_int;
                left_rotate(rbt, (*(*nd).parent).parent);
            }
        }
    }
    (*(*rbt).root).color = 0 as libc::c_int;
}
pub unsafe extern "C" fn find_node(
    mut rbt: *mut rbtree,
    mut key: *mut libc::c_void,
) -> *mut rbnode {
    let mut nd: *mut rbnode = &mut (*rbt).nil;
    let mut i: libc::c_int = 0;
    nd = (*rbt).root;
    while nd != &mut (*rbt).nil as *mut rbnode {
        i = ((*rbt).c).unwrap()((*nd).key, key, (*rbt).argv);
        if i > 0 as libc::c_int {
            nd = (*nd).left;
        }
        if i < 0 as libc::c_int {
            nd = (*nd).right;
        }
        if nd == &mut (*rbt).nil as *mut rbnode {
            break;
        }
        if i == 0 as libc::c_int {
            return nd;
        }
    }
    return 0 as *mut rbnode;
}
pub unsafe extern "C" fn insert_node(
    mut rbt: *mut rbtree,
    mut pnd: *mut rbnode,
) -> libc::c_int {
    let mut tmp: *mut rbnode = &mut (*rbt).nil;
    let mut itor: *mut rbnode = (*rbt).root;
    let mut nd: *mut rbnode = malloc(::std::mem::size_of::<rbnode>() as libc::c_ulong)
        as *mut rbnode;
    if nd.is_null() {
        return -(1 as libc::c_int);
    }
    *nd = *pnd;
    while itor != &mut (*rbt).nil as *mut rbnode {
        tmp = itor;
        if ((*rbt).c).unwrap()((*itor).key, (*nd).key, (*rbt).argv) > 0 as libc::c_int {
            itor = (*itor).left;
        } else {
            itor = (*itor).right;
        }
    }
    (*nd).parent = tmp;
    if tmp == &mut (*rbt).nil as *mut rbnode {
        (*rbt).root = nd;
    } else if ((*rbt).c).unwrap()((*tmp).key, (*nd).key, (*rbt).argv) > 0 as libc::c_int
    {
        (*tmp).left = nd;
    } else {
        (*tmp).right = nd;
    }
    (*nd).right = &mut (*rbt).nil;
    (*nd).left = (*nd).right;
    (*nd).color = 1 as libc::c_int;
    insert_fixup(rbt, nd);
    (*rbt).size = ((*rbt).size).wrapping_add(1);
    (*rbt).size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn rbt_successor(
    mut rbt: *mut rbtree,
    mut nd: *mut rbnode,
) -> *mut rbnode {
    let mut min: *mut rbnode = &mut (*rbt).nil;
    if (*nd).right != &mut (*rbt).nil as *mut rbnode {
        min = (*nd).right;
        while (*min).left != &mut (*rbt).nil as *mut rbnode {
            min = (*min).left;
        }
        return min;
    }
    min = (*nd).parent;
    while min != &mut (*rbt).nil as *mut rbnode && nd == (*min).right {
        nd = min;
        min = (*min).parent;
    }
    return min;
}
unsafe extern "C" fn delete_fixup(mut rbt: *mut rbtree, mut nd: *mut rbnode) {
    let mut tmp: *mut rbnode = &mut (*rbt).nil;
    while nd != (*rbt).root && (*nd).color == 0 as libc::c_int {
        if nd == (*(*nd).parent).left {
            tmp = (*(*nd).parent).right;
            if (*tmp).color == 1 as libc::c_int {
                (*tmp).color = 0 as libc::c_int;
                (*(*nd).parent).color = 1 as libc::c_int;
                left_rotate(rbt, (*nd).parent);
                tmp = (*(*nd).parent).right;
            }
            if (*(*tmp).left).color == 0 as libc::c_int
                && (*(*tmp).right).color == 0 as libc::c_int
            {
                (*tmp).color = 1 as libc::c_int;
                nd = (*nd).parent;
            } else {
                if (*(*tmp).right).color == 0 as libc::c_int {
                    (*(*tmp).left).color = 0 as libc::c_int;
                    (*tmp).color = 1 as libc::c_int;
                    right_rotate(rbt, tmp);
                    tmp = (*(*nd).parent).right;
                }
                (*tmp).color = (*(*nd).parent).color;
                (*(*nd).parent).color = 0 as libc::c_int;
                (*(*tmp).right).color = 0 as libc::c_int;
                left_rotate(rbt, (*nd).parent);
                nd = (*rbt).root;
            }
        } else {
            tmp = (*(*nd).parent).left;
            if (*tmp).color == 1 as libc::c_int {
                (*tmp).color = 0 as libc::c_int;
                (*(*nd).parent).color = 1 as libc::c_int;
                right_rotate(rbt, (*nd).parent);
                tmp = (*(*nd).parent).left;
            }
            if (*(*tmp).right).color == 0 as libc::c_int
                && (*(*tmp).left).color == 0 as libc::c_int
            {
                (*tmp).color = 1 as libc::c_int;
                nd = (*nd).parent;
            } else {
                if (*(*tmp).left).color == 0 as libc::c_int {
                    (*(*tmp).right).color = 0 as libc::c_int;
                    (*tmp).color = 1 as libc::c_int;
                    left_rotate(rbt, tmp);
                    tmp = (*(*nd).parent).left;
                }
                (*tmp).color = (*(*nd).parent).color;
                (*(*nd).parent).color = 0 as libc::c_int;
                (*(*tmp).left).color = 0 as libc::c_int;
                right_rotate(rbt, (*nd).parent);
                nd = (*rbt).root;
            }
        }
    }
    (*nd).color = 0 as libc::c_int;
}
pub unsafe extern "C" fn min_node(mut rbt: *mut rbtree) -> *mut rbnode {
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    let mut ret: *mut rbnode = 0 as *mut rbnode;
    tmp = (*rbt).root;
    ret = &mut (*rbt).nil;
    if tmp == &mut (*rbt).nil as *mut rbnode {
        return 0 as *mut rbnode;
    }
    while tmp != &mut (*rbt).nil as *mut rbnode {
        ret = tmp;
        tmp = (*tmp).left;
    }
    if ret == &mut (*rbt).nil as *mut rbnode {
        return 0 as *mut rbnode;
    }
    return ret;
}
pub unsafe extern "C" fn delete_node(
    mut rbt: *mut rbtree,
    mut nd: *mut rbnode,
) -> *mut libc::c_void {
    let mut val: *mut ttlnode = 0 as *mut ttlnode;
    let mut ret: *mut rbnode = nd;
    let mut tmp: *mut rbnode = 0 as *mut rbnode;
    let mut itor: *mut rbnode = 0 as *mut rbnode;
    if nd.is_null() || rbt.is_null() {
        return 0 as *mut libc::c_void;
    }
    val = (*nd).key as *mut ttlnode;
    if (*nd).left == &mut (*rbt).nil as *mut rbnode
        || (*nd).right == &mut (*rbt).nil as *mut rbnode
    {
        tmp = nd;
    } else {
        tmp = rbt_successor(rbt, nd);
    }
    if (*tmp).left != &mut (*rbt).nil as *mut rbnode {
        itor = (*tmp).left;
    } else {
        itor = (*tmp).right;
    }
    (*itor).parent = (*tmp).parent;
    if (*tmp).parent == &mut (*rbt).nil as *mut rbnode {
        (*rbt).root = itor;
    } else if tmp == (*(*tmp).parent).left {
        (*(*tmp).parent).left = itor;
    } else {
        (*(*tmp).parent).right = itor;
    }
    if tmp != itor {
        (*nd).key = (*tmp).key;
    }
    if (*tmp).color == 0 as libc::c_int {
        delete_fixup(rbt, itor);
    }
    if ret.is_null() {
        printf(b"ret is null\n\0" as *const u8 as *const libc::c_char);
    }
    free(tmp as *mut libc::c_void);
    (*rbt).size = ((*rbt).size).wrapping_sub(1);
    (*rbt).size;
    return val as *mut libc::c_void;
}
pub unsafe extern "C" fn create_rbtree(
    mut c: Option::<comprbt>,
    mut argv: *mut libc::c_void,
) -> *mut rbtree {
    let mut rbt: *mut rbtree = malloc(::std::mem::size_of::<rbtree>() as libc::c_ulong)
        as *mut rbtree;
    if rbt.is_null() {
        return 0 as *mut rbtree;
    }
    (*rbt).argv = argv;
    (*rbt).c = c;
    (*rbt).size = 0 as libc::c_int as uint;
    pthread_spin_init(&mut (*rbt).lock, 0 as libc::c_int);
    (*rbt).nil.parent = &mut (*rbt).nil;
    (*rbt).nil.left = &mut (*rbt).nil;
    (*rbt).nil.right = &mut (*rbt).nil;
    (*rbt).nil.color = 0 as libc::c_int;
    (*rbt).nil.key = 0 as *mut libc::c_void;
    (*rbt).root = &mut (*rbt).nil;
    return rbt;
}
pub unsafe extern "C" fn get_rbt_size(mut rbt: *mut rbtree) -> uint {
    return (*rbt).size;
}
pub unsafe extern "C" fn free_rbtree(mut rbt: *mut rbtree) -> libc::c_int {
    if get_rbt_size(rbt) > 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    free(rbt as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn rbtree_test() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut slice: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut node: rbnode = rbnode {
        parent: 0 as *mut rbnode,
        left: 0 as *mut rbnode,
        right: 0 as *mut rbnode,
        color: 0,
        key: 0 as *mut libc::c_void,
    };
    let mut pn: *mut rbnode = 0 as *mut rbnode;
    let mut tn: *mut ttlnode = 0 as *mut ttlnode;
    let mut rbt: *mut rbtree = 0 as *mut rbtree;
    rbt = create_rbtree(
        Some(
            rbt_comp_ttl_gt
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    if rbt.is_null() {
        dns_error(
            0 as libc::c_int,
            b"create rbtree\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    node = (*rbt).nil;
    slice = 8000000 as libc::c_int;
    j = 0 as libc::c_int;
    while j < slice {
        len = (random() % 30 as libc::c_int as libc::c_long) as libc::c_int;
        tn = malloc(
            (::std::mem::size_of::<ttlnode>() as libc::c_ulong)
                .wrapping_add(len as libc::c_ulong),
        ) as *mut ttlnode;
        if tn.is_null() {
            printf(b"oom\n\0" as *const u8 as *const libc::c_char);
        }
        (*tn).exp = j as uint;
        i = 0 as libc::c_int;
        while i < len {
            *((*tn).data).offset(i as isize) = ('a' as i32 + i) as uchar;
            i += 1;
            i;
        }
        node.key = tn as *mut libc::c_void;
        ret = insert_node(rbt, &mut node);
        if ret != 0 as libc::c_int {
            printf(b"insert error\n\0" as *const u8 as *const libc::c_char);
        }
        j += 1;
        j;
    }
    printf(b"insert all\n\0" as *const u8 as *const libc::c_char);
    sleep(2 as libc::c_int as libc::c_uint);
    j = 0 as libc::c_int;
    while j < slice {
        pn = min_node(rbt);
        if !pn.is_null() {
            tn = delete_node(rbt, pn) as *mut ttlnode;
            free(tn as *mut libc::c_void);
        } else {
            printf(b"error\n\0" as *const u8 as *const libc::c_char);
        }
        j += 1;
        j;
    }
    printf(b"delete all\n\0" as *const u8 as *const libc::c_char);
    sleep(5 as libc::c_int as libc::c_uint);
    if free_rbtree(rbt) != 0 as libc::c_int {
        dns_error(
            0 as libc::c_int,
            b"free\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
