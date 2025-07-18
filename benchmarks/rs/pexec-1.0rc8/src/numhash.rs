use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct numhashnode {
    pub node: C2RustUnnamed,
    pub nchild: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub leaves: *mut numhashnode,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct numhashtable {
    pub table: numhashnode,
    pub depth: libc::c_int,
    pub bitsize: libc::c_int,
}
pub unsafe extern "C" fn numhash_search(
    mut nt: *mut numhashtable,
    mut value: libc::c_int,
    mut ret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut off: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut nh: *mut numhashnode = 0 as *mut numhashnode;
    if nt.is_null() {
        return 0 as libc::c_int;
    }
    off = 0 as libc::c_int;
    nh = &mut (*nt).table;
    if ((*nt).bitsize as libc::c_ulong)
        < (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        value = value & ((1 as libc::c_int) << (*nt).bitsize) - 1 as libc::c_int;
    }
    loop {
        bit = value
            >> (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth - off)
            & ((1 as libc::c_int) << (*nt).depth) - 1 as libc::c_int;
        if ((*nh).node.leaves).is_null() {
            return 0 as libc::c_int;
        }
        off += 1;
        off;
        nh = &mut *((*nh).node.leaves).offset(bit as isize) as *mut numhashnode;
        if !(off * (*nt).depth < (*nt).bitsize) {
            break;
        }
    }
    if !ret.is_null() {
        *ret = (*nh).node.data;
    }
    return (*nh).nchild;
}
pub unsafe extern "C" fn numhash_init(
    mut nt: *mut numhashtable,
    mut bitsize: libc::c_int,
    mut depth: libc::c_int,
) -> libc::c_int {
    (*nt).table.node.leaves = 0 as *mut numhashnode;
    (*nt).table.node.data = 0 as *mut libc::c_void;
    (*nt).table.nchild = 0 as libc::c_int;
    (*nt).depth = depth;
    (*nt).bitsize = bitsize;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn numhash_add(
    mut nt: *mut numhashtable,
    mut value: libc::c_int,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut off: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut bitsize: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut tnode: libc::c_int = 0;
    let mut nodesize: libc::c_int = 0;
    let mut nh: *mut numhashnode = 0 as *mut numhashnode;
    let mut tree: [*mut numhashnode; 64] = [0 as *mut numhashnode; 64];
    if nt.is_null() {
        return -(1 as libc::c_int);
    }
    off = 0 as libc::c_int;
    nh = &mut (*nt).table;
    depth = (*nt).depth;
    bitsize = (*nt).bitsize;
    tnode = 0 as libc::c_int;
    if (bitsize as libc::c_ulong)
        < (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        value = value & ((1 as libc::c_int) << bitsize) - 1 as libc::c_int;
    }
    loop {
        shift = depth * ((bitsize - 1 as libc::c_int) / depth - off);
        bit = value >> shift & ((1 as libc::c_int) << depth) - 1 as libc::c_int;
        if ((*nh).node.leaves).is_null() {
            nodesize = (1 as libc::c_int) << depth;
            (*nh)
                .node
                .leaves = malloc(
                (::std::mem::size_of::<numhashnode>() as libc::c_ulong)
                    .wrapping_mul(nodesize as libc::c_ulong),
            ) as *mut numhashnode;
            if ((*nh).node.leaves).is_null() && 1 as libc::c_int > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"numhash.c: %s.\n\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"memory exhausted\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                abort();
            }
            memset(
                (*nh).node.leaves as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<numhashnode>() as libc::c_ulong)
                    .wrapping_mul(nodesize as libc::c_ulong),
            );
        }
        off += 1;
        off;
        tree[tnode as usize] = nh;
        tnode += 1;
        tnode;
        nh = &mut *((*nh).node.leaves).offset(bit as isize) as *mut numhashnode;
        if !(off * depth < bitsize) {
            break;
        }
    }
    (*nh).node.data = data;
    if (*nh).nchild != 0 {
        return 0 as libc::c_int
    } else {
        (*nh).nchild = 1 as libc::c_int;
        while tnode > 0 as libc::c_int {
            tnode -= 1;
            tnode;
            (*tree[tnode as usize]).nchild += 1;
            (*tree[tnode as usize]).nchild;
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn numhash_total(mut nt: *mut numhashtable) -> libc::c_int {
    return (*nt).table.nchild;
}
pub unsafe extern "C" fn numhash_remove(
    mut nt: *mut numhashtable,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut off: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    let mut bitsize: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut tnode: libc::c_int = 0;
    let mut nh: *mut numhashnode = 0 as *mut numhashnode;
    let mut tree: [*mut numhashnode; 64] = [0 as *mut numhashnode; 64];
    if nt.is_null() {
        return 0 as libc::c_int;
    }
    off = 0 as libc::c_int;
    nh = &mut (*nt).table;
    depth = (*nt).depth;
    bitsize = (*nt).bitsize;
    tnode = 0 as libc::c_int;
    if (bitsize as libc::c_ulong)
        < (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        value = value & ((1 as libc::c_int) << bitsize) - 1 as libc::c_int;
    }
    loop {
        shift = depth * ((bitsize - 1 as libc::c_int) / depth - off);
        bit = value >> shift & ((1 as libc::c_int) << depth) - 1 as libc::c_int;
        if ((*nh).node.leaves).is_null() {
            return 0 as libc::c_int;
        }
        off += 1;
        off;
        tree[tnode as usize] = nh;
        tnode += 1;
        tnode;
        nh = &mut *((*nh).node.leaves).offset(bit as isize) as *mut numhashnode;
        if !(off * depth < bitsize) {
            break;
        }
    }
    if (*nh).nchild <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*nh).nchild = 0 as libc::c_int;
    while tnode > 0 as libc::c_int {
        tnode -= 1;
        tnode;
        nh = tree[tnode as usize];
        (*nh).nchild -= 1;
        (*nh).nchild;
        if (*nh).nchild <= 0 as libc::c_int {
            free((*nh).node.leaves as *mut libc::c_void);
            (*nh).node.leaves = 0 as *mut numhashnode;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn numhash_get_terminal_free(
    mut nt: *mut numhashtable,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut capacity: libc::c_int = 0;
    let mut cdepth: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut nh: *mut numhashnode = 0 as *mut numhashnode;
    if (1 as libc::c_int) << (*nt).bitsize <= (*nt).table.nchild {
        return -(1 as libc::c_int);
    }
    off = 0 as libc::c_int;
    nh = &mut (*nt).table;
    if ((*nh).node.leaves).is_null() {
        return 0 as libc::c_int;
    }
    off = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while !nh.is_null() {
        shift = (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth - off);
        cdepth = if (*nt).bitsize - shift < (*nt).depth {
            (*nt).bitsize - shift
        } else {
            (*nt).depth
        };
        capacity = (1 as libc::c_int) << shift;
        if dir >= 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (1 as libc::c_int) << cdepth {
                if (*((*nh).node.leaves).offset(i as isize)).nchild <= 0 as libc::c_int {
                    return k
                } else {
                    if (*((*nh).node.leaves).offset(i as isize)).nchild < capacity {
                        break;
                    }
                    k += capacity;
                    i += 1;
                    i;
                }
            }
            if i == (1 as libc::c_int) << cdepth {
                return -(1 as libc::c_int);
            }
        } else {
            i = ((1 as libc::c_int) << cdepth) - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                if (*((*nh).node.leaves).offset(i as isize)).nchild <= 0 as libc::c_int {
                    return ((1 as libc::c_int) << (*nt).bitsize) - 1 as libc::c_int - k
                } else {
                    if (*((*nh).node.leaves).offset(i as isize)).nchild < capacity {
                        break;
                    }
                    k += capacity;
                    i -= 1;
                    i;
                }
            }
            if i < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        nh = &mut *((*nh).node.leaves).offset(i as isize) as *mut numhashnode;
        off += 1;
        off;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn numhash_get_smallest_free(
    mut nt: *mut numhashtable,
) -> libc::c_int {
    return numhash_get_terminal_free(nt, 1 as libc::c_int);
}
pub unsafe extern "C" fn numhash_get_largest_free(
    mut nt: *mut numhashtable,
) -> libc::c_int {
    return numhash_get_terminal_free(nt, -(1 as libc::c_int));
}
unsafe extern "C" fn numhash_get_terminal_used(
    mut nt: *mut numhashtable,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut capacity: libc::c_int = 0;
    let mut cdepth: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut tnd: libc::c_int = 0;
    let mut nh: *mut numhashnode = 0 as *mut numhashnode;
    if (*nt).table.nchild <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    off = 0 as libc::c_int;
    nh = &mut (*nt).table;
    if ((*nh).node.leaves).is_null() {
        return -(1 as libc::c_int);
    }
    tnd = ((*nt).bitsize - 1 as libc::c_int) / (*nt).depth;
    off = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while !nh.is_null() {
        shift = (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth - off);
        cdepth = if (*nt).bitsize - shift < (*nt).depth {
            (*nt).bitsize - shift
        } else {
            (*nt).depth
        };
        capacity = (1 as libc::c_int) << shift;
        if dir >= 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (1 as libc::c_int) << cdepth {
                if (*((*nh).node.leaves).offset(i as isize)).nchild != 0 && off < tnd {
                    break;
                }
                if (*((*nh).node.leaves).offset(i as isize)).nchild != 0 {
                    return k
                } else {
                    k += capacity;
                }
                i += 1;
                i;
            }
            if i == (1 as libc::c_int) << cdepth {
                return -(1 as libc::c_int);
            }
        } else {
            i = ((1 as libc::c_int) << cdepth) - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                if (*((*nh).node.leaves).offset(i as isize)).nchild != 0 && off < tnd {
                    break;
                }
                if (*((*nh).node.leaves).offset(i as isize)).nchild != 0 {
                    return ((1 as libc::c_int) << (*nt).bitsize) - 1 as libc::c_int - k
                } else {
                    k += capacity;
                }
                i -= 1;
                i;
            }
            if i < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        nh = &mut *((*nh).node.leaves).offset(i as isize) as *mut numhashnode;
        off += 1;
        off;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn numhash_get_smallest_used(
    mut nt: *mut numhashtable,
) -> libc::c_int {
    return numhash_get_terminal_used(nt, 1 as libc::c_int);
}
pub unsafe extern "C" fn numhash_get_largest_used(
    mut nt: *mut numhashtable,
) -> libc::c_int {
    return numhash_get_terminal_used(nt, -(1 as libc::c_int));
}
unsafe extern "C" fn numhash_local_walk(
    mut nh: *mut numhashnode,
    mut off: libc::c_int,
    mut num: libc::c_int,
    mut depth: libc::c_int,
    mut bitsize: libc::c_int,
    mut callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut shift: libc::c_int = 0;
    let mut cdepth: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if ((*nh).node.leaves).is_null() {
        return 0 as libc::c_int;
    }
    shift = depth * ((bitsize - 1 as libc::c_int) / depth - off);
    cdepth = if bitsize - shift < depth { bitsize - shift } else { depth };
    ret = 0 as libc::c_int;
    if shift > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << cdepth {
            if !((*((*nh).node.leaves).offset(i as isize)).node.leaves).is_null() {
                ret
                    += numhash_local_walk(
                        &mut *((*nh).node.leaves).offset(i as isize),
                        off + 1 as libc::c_int,
                        num | i << shift,
                        depth,
                        bitsize,
                        callback,
                        param,
                    );
            }
            i += 1;
            i;
        }
    } else if callback.is_some() {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << cdepth {
            if (*((*nh).node.leaves).offset(i as isize)).nchild > 0 as libc::c_int {
                callback
                    .unwrap()(
                    num + i,
                    (*((*nh).node.leaves).offset(i as isize)).node.data,
                    param,
                );
                ret += 1;
                ret;
            }
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << cdepth {
            if (*((*nh).node.leaves).offset(i as isize)).nchild > 0 as libc::c_int {
                ret += 1;
                ret;
            }
            i += 1;
            i;
        }
    }
    return ret;
}
unsafe extern "C" fn numhash_local_walk_desc(
    mut nh: *mut numhashnode,
    mut off: libc::c_int,
    mut num: libc::c_int,
    mut depth: libc::c_int,
    mut bitsize: libc::c_int,
    mut callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut shift: libc::c_int = 0;
    let mut cdepth: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if ((*nh).node.leaves).is_null() {
        return 0 as libc::c_int;
    }
    shift = depth * ((bitsize - 1 as libc::c_int) / depth - off);
    cdepth = if bitsize - shift < depth { bitsize - shift } else { depth };
    ret = 0 as libc::c_int;
    if shift > 0 as libc::c_int {
        i = ((1 as libc::c_int) << cdepth) - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if !((*((*nh).node.leaves).offset(i as isize)).node.leaves).is_null() {
                ret
                    += numhash_local_walk_desc(
                        &mut *((*nh).node.leaves).offset(i as isize),
                        off + 1 as libc::c_int,
                        num | i << shift,
                        depth,
                        bitsize,
                        callback,
                        param,
                    );
            }
            i -= 1;
            i;
        }
    } else if callback.is_some() {
        i = ((1 as libc::c_int) << cdepth) - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if (*((*nh).node.leaves).offset(i as isize)).nchild > 0 as libc::c_int {
                callback
                    .unwrap()(
                    num + i,
                    (*((*nh).node.leaves).offset(i as isize)).node.data,
                    param,
                );
                ret += 1;
                ret;
            }
            i -= 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << cdepth {
            if (*((*nh).node.leaves).offset(i as isize)).nchild > 0 as libc::c_int {
                ret += 1;
                ret;
            }
            i += 1;
            i;
        }
    }
    return ret;
}
pub unsafe extern "C" fn numhash_walk(
    mut nt: *mut numhashtable,
    mut callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = numhash_local_walk(
        &mut (*nt).table,
        0 as libc::c_int,
        0 as libc::c_int,
        (*nt).depth,
        (*nt).bitsize,
        callback,
        param,
    );
    return ret;
}
pub unsafe extern "C" fn numhash_walk_asc(
    mut nt: *mut numhashtable,
    mut callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    return numhash_walk(nt, callback, param);
}
pub unsafe extern "C" fn numhash_walk_desc(
    mut nt: *mut numhashtable,
    mut callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut param: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = numhash_local_walk_desc(
        &mut (*nt).table,
        0 as libc::c_int,
        0 as libc::c_int,
        (*nt).depth,
        (*nt).bitsize,
        callback,
        param,
    );
    return ret;
}
pub unsafe extern "C" fn numhash_walk_dir(
    mut nt: *mut numhashtable,
    mut callback: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut param: *mut libc::c_void,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if dir >= 0 as libc::c_int {
        ret = numhash_local_walk(
            &mut (*nt).table,
            0 as libc::c_int,
            0 as libc::c_int,
            (*nt).depth,
            (*nt).bitsize,
            callback,
            param,
        );
    } else {
        ret = numhash_local_walk_desc(
            &mut (*nt).table,
            0 as libc::c_int,
            0 as libc::c_int,
            (*nt).depth,
            (*nt).bitsize,
            callback,
            param,
        );
    }
    return ret;
}
pub unsafe extern "C" fn numhash_first(mut nt: *mut numhashtable) -> libc::c_int {
    return numhash_get_terminal_used(nt, 1 as libc::c_int);
}
pub unsafe extern "C" fn numhash_first_wdata(
    mut nt: *mut numhashtable,
    mut dret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = numhash_get_terminal_used(nt, 1 as libc::c_int);
    if c >= 0 as libc::c_int && !dret.is_null() {
        numhash_search(nt, c, dret);
    }
    return c;
}
pub unsafe extern "C" fn numhash_last(mut nt: *mut numhashtable) -> libc::c_int {
    return numhash_get_terminal_used(nt, -(1 as libc::c_int));
}
pub unsafe extern "C" fn numhash_last_wdata(
    mut nt: *mut numhashtable,
    mut dret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = numhash_get_terminal_used(nt, -(1 as libc::c_int));
    if c >= 0 as libc::c_int && !dret.is_null() {
        numhash_search(nt, c, dret);
    }
    return c;
}
unsafe extern "C" fn numhash_local_first(
    mut nt: *mut numhashtable,
    mut nh: *mut numhashnode,
    mut fcc: libc::c_int,
    mut off: libc::c_int,
    mut dret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ntn: libc::c_int = 0;
    let mut lcb: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if ((*nh).node.leaves).is_null() {
        return -(1 as libc::c_int);
    }
    ntn = (1 as libc::c_int)
        << (if off != 0 {
            (*nt).depth
        } else {
            (*nt).bitsize
                - (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth)
        });
    lcb = (1 as libc::c_int)
        << (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth - off);
    ret = fcc;
    i = 0 as libc::c_int;
    while i < ntn {
        if (*((*nh).node.leaves).offset(i as isize)).nchild != 0 {
            if off < ((*nt).bitsize - 1 as libc::c_int) / (*nt).depth {
                nh = &mut *((*nh).node.leaves).offset(i as isize) as *mut numhashnode;
                off += 1;
                off;
                ntn = (1 as libc::c_int)
                    << (if off != 0 {
                        (*nt).depth
                    } else {
                        (*nt).bitsize
                            - (*nt).depth
                                * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth)
                    });
                i = 0 as libc::c_int;
                lcb = lcb >> (*nt).depth;
            } else {
                if !dret.is_null() {
                    *dret = (*((*nh).node.leaves).offset(i as isize)).node.data;
                }
                return ret;
            }
        } else {
            i += 1;
            i;
            ret += lcb;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn numhash_local_last(
    mut nt: *mut numhashtable,
    mut nh: *mut numhashnode,
    mut fcc: libc::c_int,
    mut off: libc::c_int,
    mut dret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ntn: libc::c_int = 0;
    let mut lcb: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if ((*nh).node.leaves).is_null() {
        return -(1 as libc::c_int);
    }
    ntn = (1 as libc::c_int)
        << (if off != 0 {
            (*nt).depth
        } else {
            (*nt).bitsize
                - (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth)
        });
    lcb = (1 as libc::c_int)
        << (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth - off);
    ret = fcc + lcb * (ntn - 1 as libc::c_int);
    i = ntn - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (*((*nh).node.leaves).offset(i as isize)).nchild != 0 {
            if off < ((*nt).bitsize - 1 as libc::c_int) / (*nt).depth {
                nh = &mut *((*nh).node.leaves).offset(i as isize) as *mut numhashnode;
                off += 1;
                off;
                ntn = (1 as libc::c_int)
                    << (if off != 0 {
                        (*nt).depth
                    } else {
                        (*nt).bitsize
                            - (*nt).depth
                                * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth)
                    });
                i = ntn - 1 as libc::c_int;
                lcb = lcb >> (*nt).depth;
                ret += lcb * (ntn - 1 as libc::c_int);
            } else {
                if !dret.is_null() {
                    *dret = (*((*nh).node.leaves).offset(i as isize)).node.data;
                }
                return ret;
            }
        } else {
            i -= 1;
            i;
            ret -= lcb;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn numhash_local_nextprev(
    mut nt: *mut numhashtable,
    mut nh0: *mut numhashnode,
    mut value: libc::c_int,
    mut off: libc::c_int,
    mut fcc: libc::c_int,
    mut ret: *mut libc::c_int,
    mut dir: libc::c_int,
    mut dret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut nh: *mut numhashnode = 0 as *mut numhashnode;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut is_real_leaf: libc::c_int = 0;
    let mut ntn: libc::c_int = 0;
    let mut lcb: libc::c_int = 0;
    if ((*nh0).node.leaves).is_null() {
        return 0 as libc::c_int;
    }
    k = value >> (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth - off)
        & ((1 as libc::c_int) << (*nt).depth) - 1 as libc::c_int;
    nh = &mut *((*nh0).node.leaves).offset(k as isize) as *mut numhashnode;
    lcb = (1 as libc::c_int)
        << (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth - off);
    is_real_leaf = (off < ((*nt).bitsize - 1 as libc::c_int) / (*nt).depth)
        as libc::c_int;
    if !((*nh).node.leaves).is_null() && is_real_leaf != 0 {
        c = numhash_local_nextprev(
            nt,
            nh,
            value,
            off + 1 as libc::c_int,
            fcc + k * lcb,
            ret,
            dir,
            dret,
        );
        if c != 0 {
            return 1 as libc::c_int;
        }
    }
    ntn = (1 as libc::c_int)
        << (if off != 0 {
            (*nt).depth
        } else {
            (*nt).bitsize
                - (*nt).depth * (((*nt).bitsize - 1 as libc::c_int) / (*nt).depth)
        });
    if dir >= 0 as libc::c_int {
        i = k + 1 as libc::c_int;
        while i < ntn {
            nh = &mut *((*nh0).node.leaves).offset(i as isize) as *mut numhashnode;
            if is_real_leaf == 0 && (*nh).nchild != 0 {
                *ret = fcc + i;
                if !dret.is_null() {
                    *dret = (*nh).node.data;
                }
                return 1 as libc::c_int;
            } else if is_real_leaf != 0 && (*nh).nchild != 0 {
                *ret = numhash_local_first(
                    nt,
                    nh,
                    fcc + i * lcb,
                    off + 1 as libc::c_int,
                    dret,
                );
                return 1 as libc::c_int;
            }
            i += 1;
            i;
        }
    } else {
        i = k - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            nh = &mut *((*nh0).node.leaves).offset(i as isize) as *mut numhashnode;
            if is_real_leaf == 0 && (*nh).nchild != 0 {
                *ret = fcc + i;
                if !dret.is_null() {
                    *dret = (*nh).node.data;
                }
                return 1 as libc::c_int;
            } else if is_real_leaf != 0 && (*nh).nchild != 0 {
                *ret = numhash_local_last(
                    nt,
                    nh,
                    fcc + i * lcb,
                    off + 1 as libc::c_int,
                    dret,
                );
                return 1 as libc::c_int;
            }
            i -= 1;
            i;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn numhash_next_wdata(
    mut nt: *mut numhashtable,
    mut value: libc::c_int,
    mut dret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = numhash_local_nextprev(
        nt,
        &mut (*nt).table,
        value,
        0 as libc::c_int,
        0 as libc::c_int,
        &mut ret,
        1 as libc::c_int,
        dret,
    );
    if c != 0 { return ret } else { return -(1 as libc::c_int) };
}
pub unsafe extern "C" fn numhash_prev_wdata(
    mut nt: *mut numhashtable,
    mut value: libc::c_int,
    mut dret: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = numhash_local_nextprev(
        nt,
        &mut (*nt).table,
        value,
        0 as libc::c_int,
        0 as libc::c_int,
        &mut ret,
        -(1 as libc::c_int),
        dret,
    );
    if c != 0 { return ret } else { return -(1 as libc::c_int) };
}
pub unsafe extern "C" fn numhash_next(
    mut nt: *mut numhashtable,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = numhash_local_nextprev(
        nt,
        &mut (*nt).table,
        value,
        0 as libc::c_int,
        0 as libc::c_int,
        &mut ret,
        1 as libc::c_int,
        0 as *mut *mut libc::c_void,
    );
    if c != 0 { return ret } else { return -(1 as libc::c_int) };
}
pub unsafe extern "C" fn numhash_prev(
    mut nt: *mut numhashtable,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    c = numhash_local_nextprev(
        nt,
        &mut (*nt).table,
        value,
        0 as libc::c_int,
        0 as libc::c_int,
        &mut ret,
        -(1 as libc::c_int),
        0 as *mut *mut libc::c_void,
    );
    if c != 0 { return ret } else { return -(1 as libc::c_int) };
}
pub unsafe extern "C" fn numhash_loop_start(
    mut nt: *mut numhashtable,
    mut dir: libc::c_int,
    mut ret: *mut *mut libc::c_void,
) -> libc::c_int {
    if dir >= 0 as libc::c_int {
        return numhash_first_wdata(nt, ret)
    } else {
        return numhash_last_wdata(nt, ret)
    };
}
pub unsafe extern "C" fn numhash_loop_next(
    mut nt: *mut numhashtable,
    mut dir: libc::c_int,
    mut key: libc::c_int,
    mut ret: *mut *mut libc::c_void,
) -> libc::c_int {
    if dir >= 0 as libc::c_int {
        return numhash_next_wdata(nt, key, ret)
    } else {
        return numhash_prev_wdata(nt, key, ret)
    };
}
unsafe extern "C" fn numhash_local_free(
    mut nh: *mut numhashnode,
    mut off: libc::c_int,
    mut depth: libc::c_int,
    mut bitsize: libc::c_int,
) -> libc::c_int {
    let mut shift: libc::c_int = 0;
    let mut cdepth: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if ((*nh).node.leaves).is_null() {
        return 0 as libc::c_int;
    }
    shift = depth * ((bitsize - 1 as libc::c_int) / depth - off);
    cdepth = if bitsize - shift < depth { bitsize - shift } else { depth };
    if shift > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (1 as libc::c_int) << cdepth {
            if !((*((*nh).node.leaves).offset(i as isize)).node.leaves).is_null() {
                numhash_local_free(
                    &mut *((*nh).node.leaves).offset(i as isize),
                    off + 1 as libc::c_int,
                    depth,
                    bitsize,
                );
            }
            i += 1;
            i;
        }
    }
    free((*nh).node.leaves as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn numhash_free(mut nt: *mut numhashtable) -> libc::c_int {
    numhash_local_free(&mut (*nt).table, 0 as libc::c_int, (*nt).depth, (*nt).bitsize);
    numhash_init(nt, (*nt).bitsize, (*nt).depth);
    return 0 as libc::c_int;
}
