use ::libc;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub key: int64_t,
    pub data: *mut libc::c_void,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbtree {
    pub root: *mut rbnode,
    pub sentinel: *mut rbnode,
}
pub unsafe extern "C" fn rbtree_node_init(mut node: *mut rbnode) {
    (*node).left = 0 as *mut rbnode;
    (*node).right = 0 as *mut rbnode;
    (*node).parent = 0 as *mut rbnode;
    (*node).key = 0 as libc::c_ulonglong as int64_t;
    (*node).data = 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn rbtree_init(mut tree: *mut rbtree, mut node: *mut rbnode) {
    rbtree_node_init(node);
    (*node).color = 0 as libc::c_int as uint8_t;
    (*tree).root = node;
    (*tree).sentinel = node;
}
unsafe extern "C" fn rbtree_node_min(
    mut node: *mut rbnode,
    mut sentinel: *const rbnode,
) -> *mut rbnode {
    while (*node).left != sentinel as *mut rbnode {
        node = (*node).left;
    }
    return node;
}
pub unsafe extern "C" fn rbtree_min(mut tree: *const rbtree) -> *mut rbnode {
    let mut node: *mut rbnode = (*tree).root;
    let mut sentinel: *const rbnode = (*tree).sentinel;
    if node == sentinel as *mut rbnode {
        return 0 as *mut rbnode;
    }
    return rbtree_node_min(node, sentinel);
}
unsafe extern "C" fn rbtree_left_rotate(
    mut root: *mut *mut rbnode,
    mut sentinel: *mut rbnode,
    mut node: *mut rbnode,
) {
    let mut temp: *mut rbnode = 0 as *mut rbnode;
    temp = (*node).right;
    (*node).right = (*temp).left;
    if (*temp).left != sentinel {
        (*(*temp).left).parent = node;
    }
    (*temp).parent = (*node).parent;
    if node == *root {
        *root = temp;
    } else if node == (*(*node).parent).left {
        (*(*node).parent).left = temp;
    } else {
        (*(*node).parent).right = temp;
    }
    (*temp).left = node;
    (*node).parent = temp;
}
unsafe extern "C" fn rbtree_right_rotate(
    mut root: *mut *mut rbnode,
    mut sentinel: *mut rbnode,
    mut node: *mut rbnode,
) {
    let mut temp: *mut rbnode = 0 as *mut rbnode;
    temp = (*node).left;
    (*node).left = (*temp).right;
    if (*temp).right != sentinel {
        (*(*temp).right).parent = node;
    }
    (*temp).parent = (*node).parent;
    if node == *root {
        *root = temp;
    } else if node == (*(*node).parent).right {
        (*(*node).parent).right = temp;
    } else {
        (*(*node).parent).left = temp;
    }
    (*temp).right = node;
    (*node).parent = temp;
}
pub unsafe extern "C" fn rbtree_insert(mut tree: *mut rbtree, mut node: *mut rbnode) {
    let mut root: *mut *mut rbnode = &mut (*tree).root;
    let mut sentinel: *mut rbnode = (*tree).sentinel;
    let mut temp: *mut rbnode = 0 as *mut rbnode;
    let mut p: *mut *mut rbnode = 0 as *mut *mut rbnode;
    if *root == sentinel {
        (*node).parent = 0 as *mut rbnode;
        (*node).left = sentinel;
        (*node).right = sentinel;
        (*node).color = 0 as libc::c_int as uint8_t;
        *root = node;
        return;
    }
    temp = *root;
    loop {
        p = if (*node).key < (*temp).key {
            &mut (*temp).left
        } else {
            &mut (*temp).right
        };
        if *p == sentinel {
            break;
        }
        temp = *p;
    }
    *p = node;
    (*node).parent = temp;
    (*node).left = sentinel;
    (*node).right = sentinel;
    (*node).color = 1 as libc::c_int as uint8_t;
    while node != *root && (*(*node).parent).color as libc::c_int != 0 {
        if (*node).parent == (*(*(*node).parent).parent).left {
            temp = (*(*(*node).parent).parent).right;
            if (*temp).color != 0 {
                (*(*node).parent).color = 0 as libc::c_int as uint8_t;
                (*temp).color = 0 as libc::c_int as uint8_t;
                (*(*(*node).parent).parent).color = 1 as libc::c_int as uint8_t;
                node = (*(*node).parent).parent;
            } else {
                if node == (*(*node).parent).right {
                    node = (*node).parent;
                    rbtree_left_rotate(root, sentinel, node);
                }
                (*(*node).parent).color = 0 as libc::c_int as uint8_t;
                (*(*(*node).parent).parent).color = 1 as libc::c_int as uint8_t;
                rbtree_right_rotate(root, sentinel, (*(*node).parent).parent);
            }
        } else {
            temp = (*(*(*node).parent).parent).left;
            if (*temp).color != 0 {
                (*(*node).parent).color = 0 as libc::c_int as uint8_t;
                (*temp).color = 0 as libc::c_int as uint8_t;
                (*(*(*node).parent).parent).color = 1 as libc::c_int as uint8_t;
                node = (*(*node).parent).parent;
            } else {
                if node == (*(*node).parent).left {
                    node = (*node).parent;
                    rbtree_right_rotate(root, sentinel, node);
                }
                (*(*node).parent).color = 0 as libc::c_int as uint8_t;
                (*(*(*node).parent).parent).color = 1 as libc::c_int as uint8_t;
                rbtree_left_rotate(root, sentinel, (*(*node).parent).parent);
            }
        }
    }
    (**root).color = 0 as libc::c_int as uint8_t;
}
pub unsafe extern "C" fn rbtree_delete(mut tree: *mut rbtree, mut node: *mut rbnode) {
    let mut root: *mut *mut rbnode = &mut (*tree).root;
    let mut sentinel: *mut rbnode = (*tree).sentinel;
    let mut subst: *mut rbnode = 0 as *mut rbnode;
    let mut temp: *mut rbnode = 0 as *mut rbnode;
    let mut w: *mut rbnode = 0 as *mut rbnode;
    let mut red: uint8_t = 0;
    if (*node).left == sentinel {
        temp = (*node).right;
        subst = node;
    } else if (*node).right == sentinel {
        temp = (*node).left;
        subst = node;
    } else {
        subst = rbtree_node_min((*node).right, sentinel);
        temp = (*subst).right;
    }
    if subst == *root {
        *root = temp;
        (*temp).color = 0 as libc::c_int as uint8_t;
        rbtree_node_init(node);
        return;
    }
    red = (*subst).color;
    if subst == (*(*subst).parent).left {
        (*(*subst).parent).left = temp;
    } else {
        (*(*subst).parent).right = temp;
    }
    if subst == node {
        (*temp).parent = (*subst).parent;
    } else {
        if (*subst).parent == node {
            (*temp).parent = subst;
        } else {
            (*temp).parent = (*subst).parent;
        }
        (*subst).left = (*node).left;
        (*subst).right = (*node).right;
        (*subst).parent = (*node).parent;
        (*subst).color = (*node).color;
        if node == *root {
            *root = subst;
        } else if node == (*(*node).parent).left {
            (*(*node).parent).left = subst;
        } else {
            (*(*node).parent).right = subst;
        }
        if (*subst).left != sentinel {
            (*(*subst).left).parent = subst;
        }
        if (*subst).right != sentinel {
            (*(*subst).right).parent = subst;
        }
    }
    rbtree_node_init(node);
    if red != 0 {
        return;
    }
    while temp != *root && (*temp).color == 0 {
        if temp == (*(*temp).parent).left {
            w = (*(*temp).parent).right;
            if (*w).color != 0 {
                (*w).color = 0 as libc::c_int as uint8_t;
                (*(*temp).parent).color = 1 as libc::c_int as uint8_t;
                rbtree_left_rotate(root, sentinel, (*temp).parent);
                w = (*(*temp).parent).right;
            }
            if (*(*w).left).color == 0 && (*(*w).right).color == 0 {
                (*w).color = 1 as libc::c_int as uint8_t;
                temp = (*temp).parent;
            } else {
                if (*(*w).right).color == 0 {
                    (*(*w).left).color = 0 as libc::c_int as uint8_t;
                    (*w).color = 1 as libc::c_int as uint8_t;
                    rbtree_right_rotate(root, sentinel, w);
                    w = (*(*temp).parent).right;
                }
                (*w).color = (*(*temp).parent).color;
                (*(*temp).parent).color = 0 as libc::c_int as uint8_t;
                (*(*w).right).color = 0 as libc::c_int as uint8_t;
                rbtree_left_rotate(root, sentinel, (*temp).parent);
                temp = *root;
            }
        } else {
            w = (*(*temp).parent).left;
            if (*w).color != 0 {
                (*w).color = 0 as libc::c_int as uint8_t;
                (*(*temp).parent).color = 1 as libc::c_int as uint8_t;
                rbtree_right_rotate(root, sentinel, (*temp).parent);
                w = (*(*temp).parent).left;
            }
            if (*(*w).left).color == 0 && (*(*w).right).color == 0 {
                (*w).color = 1 as libc::c_int as uint8_t;
                temp = (*temp).parent;
            } else {
                if (*(*w).left).color == 0 {
                    (*(*w).right).color = 0 as libc::c_int as uint8_t;
                    (*w).color = 1 as libc::c_int as uint8_t;
                    rbtree_left_rotate(root, sentinel, w);
                    w = (*(*temp).parent).left;
                }
                (*w).color = (*(*temp).parent).color;
                (*(*temp).parent).color = 0 as libc::c_int as uint8_t;
                (*(*w).left).color = 0 as libc::c_int as uint8_t;
                rbtree_right_rotate(root, sentinel, (*temp).parent);
                temp = *root;
            }
        }
    }
    (*temp).color = 0 as libc::c_int as uint8_t;
}
