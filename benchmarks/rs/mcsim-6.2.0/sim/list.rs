use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
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
pub type BOOL = libc::c_int;
pub type WORD = libc::c_uint;
pub type PSTR = *mut libc::c_char;
pub type PVOID = *mut libc::c_void;
pub type PFILE = *mut FILE;
pub type PBUF = PSTR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTBUF {
    pub pfileIn: PFILE,
    pub pbufOrg: PBUF,
    pub pbufCur: PBUF,
    pub iLineNum: libc::c_int,
    pub iLNPrev: libc::c_int,
    pub cErrors: libc::c_int,
    pub pInfo: PVOID,
}
pub type PINPUTBUF = *mut tagINPUTBUF;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagLISTELEM {
    pub pData: PVOID,
    pub pleNext: *mut tagLISTELEM,
}
pub type LISTELEM = tagLISTELEM;
pub type PLISTELEM = *mut tagLISTELEM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagLIST {
    pub pleHead: PLISTELEM,
    pub pleTail: PLISTELEM,
    pub iSize: libc::c_int,
}
pub type LIST = tagLIST;
pub type PLIST = *mut tagLIST;
pub type PFI_FORLISTCALLBACK = Option::<
    unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int,
>;
pub type PFI_FORLISTCALLBACK3 = Option::<
    unsafe extern "C" fn(PVOID, PVOID, PVOID, PVOID) -> (),
>;
pub type PFV_FREELISTCALLBACK = Option::<unsafe extern "C" fn(PVOID) -> ()>;
pub unsafe extern "C" fn InitList() -> PLIST {
    let mut plist: PLIST = malloc(::std::mem::size_of::<LIST>() as libc::c_ulong)
        as PLIST;
    if !plist.is_null() {
        (*plist).pleHead = 0 as PLISTELEM;
        (*plist).pleTail = 0 as PLISTELEM;
        (*plist).iSize = 0 as libc::c_int;
    } else {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"InitList\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    return plist;
}
pub unsafe extern "C" fn FreeList(
    mut pplist: *mut PLIST,
    mut pfvFreeData: PFV_FREELISTCALLBACK,
    mut bAndData: BOOL,
) {
    let mut plist: PLIST = *pplist;
    if plist.is_null() {
        return;
    }
    while !((*plist).pleHead).is_null() {
        if pfvFreeData.is_some() {
            (Some(pfvFreeData.unwrap())).unwrap()((*(*plist).pleHead).pData);
        } else if bAndData != 0 && !((*(*plist).pleHead).pData).is_null() {
            free((*(*plist).pleHead).pData);
        }
        (*plist).pleTail = (*plist).pleHead;
        (*plist).pleHead = (*(*plist).pleHead).pleNext;
        free((*plist).pleTail as *mut libc::c_void);
    }
    free(plist as *mut libc::c_void);
    *pplist = 0 as PLIST;
}
pub unsafe extern "C" fn QueueListItem(mut plist: PLIST, mut pData: PVOID) {
    let mut pNewElem: PLISTELEM = 0 as *mut tagLISTELEM;
    if plist.is_null() {
        return;
    }
    pNewElem = malloc(::std::mem::size_of::<LISTELEM>() as libc::c_ulong) as PLISTELEM;
    if pNewElem.is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"QueueListItem\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    (*pNewElem).pData = pData;
    (*pNewElem).pleNext = 0 as *mut tagLISTELEM;
    if !((*plist).pleTail).is_null() {
        (*(*plist).pleTail).pleNext = pNewElem;
    } else {
        (*plist).pleHead = pNewElem;
    }
    (*plist).pleTail = pNewElem;
    (*plist).iSize += 1;
    (*plist).iSize;
}
pub unsafe extern "C" fn ForAllList(
    mut plist: PLIST,
    mut pfiCallback: PFI_FORLISTCALLBACK,
    mut pUserInfo: PVOID,
) -> libc::c_int {
    let mut iTotal: libc::c_int = 0 as libc::c_int;
    let mut ple: PLISTELEM = 0 as *mut tagLISTELEM;
    if plist.is_null() || pfiCallback.is_none() {
        return 0 as libc::c_int;
    }
    ple = (*plist).pleHead;
    while !ple.is_null() {
        iTotal += (Some(pfiCallback.unwrap())).unwrap()((*ple).pData, pUserInfo);
        ple = (*ple).pleNext;
    }
    return iTotal;
}
pub unsafe extern "C" fn ForAllList3(
    mut plist: PLIST,
    mut pfiCallback: PFI_FORLISTCALLBACK3,
    mut pUserInfo1: PVOID,
    mut pUserInfo2: PVOID,
    mut pUserInfo3: PVOID,
) {
    let mut ple: PLISTELEM = 0 as *mut tagLISTELEM;
    if plist.is_null() || pfiCallback.is_none() {
        return;
    }
    ple = (*plist).pleHead;
    while !ple.is_null() {
        (Some(pfiCallback.unwrap()))
            .unwrap()((*ple).pData, pUserInfo1, pUserInfo2, pUserInfo3);
        ple = (*ple).pleNext;
    }
}
