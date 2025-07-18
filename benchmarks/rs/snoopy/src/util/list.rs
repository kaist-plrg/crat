use ::libc;
extern "C" {
    fn snoopy_error_handler(errorMsg: *const libc::c_char);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listNode_t {
    pub next: *mut listNode_t,
    pub prev: *mut listNode_t,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub count: libc::c_int,
    pub first: *mut listNode_t,
    pub last: *mut listNode_t,
}
pub unsafe extern "C" fn snoopy_util_list_push(
    mut list: *mut list_t,
    mut newNodeValue: *mut libc::c_void,
) -> libc::c_int {
    let mut newNode: *mut listNode_t = 0 as *mut listNode_t;
    newNode = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<listNode_t>() as libc::c_ulong,
    ) as *mut listNode_t;
    if newNode.is_null() {
        snoopy_error_handler(
            b"Unable to allocate memory for a new doubly linked list node\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*newNode).value = newNodeValue;
    if ((*list).last).is_null() {
        (*list).first = newNode;
        (*list).last = newNode;
        (*newNode).prev = 0 as *mut listNode_t;
        (*newNode).next = 0 as *mut listNode_t;
    } else {
        (*(*list).last).next = newNode;
        (*newNode).prev = (*list).last;
        (*newNode).next = 0 as *mut listNode_t;
        (*list).last = newNode;
    }
    (*list).count += 1;
    (*list).count;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_util_list_remove(
    mut list: *mut list_t,
    mut nodeToRemove: *mut listNode_t,
) -> *mut libc::c_void {
    let mut retVal: *mut libc::c_void = 0 as *mut libc::c_void;
    if ((*list).first).is_null() || ((*list).last).is_null() {
        snoopy_error_handler(
            b"The doubly linked list is empty\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    if nodeToRemove.is_null() {
        snoopy_error_handler(
            b"No node given, unable to remove NULL\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    if nodeToRemove == (*list).first && nodeToRemove == (*list).last {
        (*list).first = 0 as *mut listNode_t;
        (*list).last = 0 as *mut listNode_t;
    } else if nodeToRemove == (*list).first {
        (*list).first = (*nodeToRemove).next;
    } else if nodeToRemove == (*list).last {
        (*list).last = (*nodeToRemove).prev;
        (*(*list).last).next = 0 as *mut listNode_t;
    } else {
        let mut nodeAfter: *mut listNode_t = (*nodeToRemove).next;
        let mut nodeBefore: *mut listNode_t = (*nodeToRemove).prev;
        (*nodeAfter).prev = nodeBefore;
        (*nodeBefore).next = nodeAfter;
    }
    (*list).count -= 1;
    (*list).count;
    retVal = (*nodeToRemove).value;
    free(nodeToRemove as *mut libc::c_void);
    return retVal;
}
pub unsafe extern "C" fn snoopy_util_list_fetchNextNode(
    mut list: *mut list_t,
    mut curNode: *mut listNode_t,
) -> *mut listNode_t {
    if ((*list).first).is_null() || ((*list).last).is_null() {
        return 0 as *mut listNode_t;
    }
    if curNode.is_null() {
        return (*list).first;
    }
    return (*curNode).next;
}
