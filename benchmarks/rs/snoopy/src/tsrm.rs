use ::libc;
extern "C" {
    fn snoopy_configuration_setUninitialized(CFG: *mut snoopy_configuration_t);
    fn snoopy_inputdatastorage_setUninitialized(IDS: *mut snoopy_inputdatastorage_t);
    fn pthread_self() -> pthread_t;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: libc::c_int,
    ) -> libc::c_int;
    fn snoopy_util_list_push(
        list: *mut list_t,
        newNodeValue: *mut libc::c_void,
    ) -> libc::c_int;
    fn snoopy_util_list_remove(
        list: *mut list_t,
        nodeToRemove: *mut listNode_t,
    ) -> *mut libc::c_void;
    fn snoopy_util_list_fetchNextNode(
        list: *mut list_t,
        curNode: *mut listNode_t,
    ) -> *mut listNode_t;
    fn free(__ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_configuration_t {
    pub initialized: libc::c_int,
    pub configfile_enabled: libc::c_int,
    pub configfile_path: *mut libc::c_char,
    pub configfile_found: libc::c_int,
    pub configfile_parsed: libc::c_int,
    pub error_logging_enabled: libc::c_int,
    pub message_format: *mut libc::c_char,
    pub message_format_malloced: libc::c_int,
    pub filtering_enabled: libc::c_int,
    pub filter_chain: *mut libc::c_char,
    pub filter_chain_malloced: libc::c_int,
    pub output: *mut libc::c_char,
    pub output_malloced: libc::c_int,
    pub output_arg: *mut libc::c_char,
    pub output_arg_malloced: libc::c_int,
    pub syslog_facility: libc::c_int,
    pub syslog_level: libc::c_int,
    pub syslog_ident_format_malloced: libc::c_int,
    pub syslog_ident_format: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_inputdatastorage_t {
    pub initialized: libc::c_int,
    pub filename: *const libc::c_char,
    pub argv: *const *mut libc::c_char,
    pub envp: *const *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_once_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub type snoopy_tsrm_threadId_t = pthread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snoopy_tsrm_threadData_t {
    pub threadId: snoopy_tsrm_threadId_t,
    pub configuration: *mut snoopy_configuration_t,
    pub inputdatastorage: *mut snoopy_inputdatastorage_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_t {
    pub count: libc::c_int,
    pub first: *mut listNode_t,
    pub last: *mut listNode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listNode_t {
    pub next: *mut listNode_t,
    pub prev: *mut listNode_t,
    pub value: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
}
pub static mut snoopy_tsrm_init_onceControl: pthread_once_t = 0 as libc::c_int;
pub static mut snoopy_tsrm_threadRepo_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub static mut snoopy_tsrm_threadRepo_mutexAttr: pthread_mutexattr_t = pthread_mutexattr_t {
    __size: [0; 4],
};
pub static mut snoopy_tsrm_threadRepo_data: list_t = {
    let mut init = list_t {
        count: 0 as libc::c_int,
        first: 0 as *const listNode_t as *mut listNode_t,
        last: 0 as *const listNode_t as *mut listNode_t,
    };
    init
};
pub static mut snoopy_tsrm_threadRepo: *mut list_t = unsafe {
    &snoopy_tsrm_threadRepo_data as *const list_t as *mut list_t
};
pub unsafe extern "C" fn snoopy_tsrm_ctor() {
    let mut curTid: snoopy_tsrm_threadId_t = 0;
    let mut tData: *mut snoopy_tsrm_threadData_t = 0 as *mut snoopy_tsrm_threadData_t;
    pthread_once(
        &mut snoopy_tsrm_init_onceControl,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(snoopy_tsrm_init),
            ),
        ),
    );
    curTid = snoopy_tsrm_getCurrentThreadId();
    pthread_mutex_lock(&mut snoopy_tsrm_threadRepo_mutex);
    if 0 as libc::c_int == snoopy_tsrm_doesThreadRepoEntryExist(curTid, 1 as libc::c_int)
    {
        tData = snoopy_tsrm_createNewThreadData(curTid);
        snoopy_util_list_push(snoopy_tsrm_threadRepo, tData as *mut libc::c_void);
    }
    pthread_mutex_unlock(&mut snoopy_tsrm_threadRepo_mutex);
}
pub unsafe extern "C" fn snoopy_tsrm_dtor() {
    let mut tRepoEntry: *mut listNode_t = 0 as *mut listNode_t;
    let mut tData: *mut snoopy_tsrm_threadData_t = 0 as *mut snoopy_tsrm_threadData_t;
    tRepoEntry = snoopy_tsrm_getCurrentThreadRepoEntry();
    if tRepoEntry.is_null() {
        return;
    }
    pthread_mutex_lock(&mut snoopy_tsrm_threadRepo_mutex);
    tData = snoopy_util_list_remove(snoopy_tsrm_threadRepo, tRepoEntry)
        as *mut snoopy_tsrm_threadData_t;
    pthread_mutex_unlock(&mut snoopy_tsrm_threadRepo_mutex);
    free((*tData).inputdatastorage as *mut libc::c_void);
    free((*tData).configuration as *mut libc::c_void);
    free(tData as *mut libc::c_void);
}
pub unsafe extern "C" fn snoopy_tsrm_init() {
    pthread_mutexattr_init(&mut snoopy_tsrm_threadRepo_mutexAttr);
    pthread_mutexattr_settype(
        &mut snoopy_tsrm_threadRepo_mutexAttr,
        PTHREAD_MUTEX_RECURSIVE as libc::c_int,
    );
    pthread_mutex_init(
        &mut snoopy_tsrm_threadRepo_mutex,
        &mut snoopy_tsrm_threadRepo_mutexAttr,
    );
}
pub unsafe extern "C" fn snoopy_tsrm_doesThreadRepoEntryExist(
    mut threadId: snoopy_tsrm_threadId_t,
    mut mutex_already_locked: libc::c_int,
) -> libc::c_int {
    let mut tData: *const snoopy_tsrm_threadData_t = 0
        as *const snoopy_tsrm_threadData_t;
    let mut retVal: libc::c_int = 0 as libc::c_int;
    if 1 as libc::c_int != mutex_already_locked {
        pthread_mutex_lock(&mut snoopy_tsrm_threadRepo_mutex);
    }
    let mut curNode: *mut listNode_t = 0 as *mut listNode_t;
    loop {
        curNode = snoopy_util_list_fetchNextNode(snoopy_tsrm_threadRepo, curNode);
        if curNode.is_null() {
            break;
        }
        if ((*curNode).value).is_null() {
            continue;
        }
        tData = (*curNode).value as *const snoopy_tsrm_threadData_t;
        if !(0 as libc::c_int != pthread_equal(threadId, (*tData).threadId)) {
            continue;
        }
        retVal = 1 as libc::c_int;
        break;
    }
    if 1 as libc::c_int != mutex_already_locked {
        pthread_mutex_unlock(&mut snoopy_tsrm_threadRepo_mutex);
    }
    return retVal;
}
pub unsafe extern "C" fn snoopy_tsrm_createNewThreadData(
    mut threadId: snoopy_tsrm_threadId_t,
) -> *mut snoopy_tsrm_threadData_t {
    let mut tData: *mut snoopy_tsrm_threadData_t = 0 as *mut snoopy_tsrm_threadData_t;
    tData = malloc(::std::mem::size_of::<snoopy_tsrm_threadData_t>() as libc::c_ulong)
        as *mut snoopy_tsrm_threadData_t;
    (*tData)
        .configuration = malloc(
        ::std::mem::size_of::<snoopy_configuration_t>() as libc::c_ulong,
    ) as *mut snoopy_configuration_t;
    (*tData)
        .inputdatastorage = malloc(
        ::std::mem::size_of::<snoopy_inputdatastorage_t>() as libc::c_ulong,
    ) as *mut snoopy_inputdatastorage_t;
    (*tData).threadId = threadId;
    snoopy_configuration_setUninitialized((*tData).configuration);
    snoopy_inputdatastorage_setUninitialized((*tData).inputdatastorage);
    return tData;
}
pub unsafe extern "C" fn snoopy_tsrm_getCurrentThreadId() -> snoopy_tsrm_threadId_t {
    return pthread_self();
}
pub unsafe extern "C" fn snoopy_tsrm_getCurrentThreadRepoEntry() -> *mut listNode_t {
    let mut myThreadId: snoopy_tsrm_threadId_t = 0;
    let mut tRepoEntry: *mut listNode_t = 0 as *mut listNode_t;
    let mut tData: *const snoopy_tsrm_threadData_t = 0
        as *const snoopy_tsrm_threadData_t;
    myThreadId = snoopy_tsrm_getCurrentThreadId();
    pthread_mutex_lock(&mut snoopy_tsrm_threadRepo_mutex);
    let mut curNode: *mut listNode_t = 0 as *mut listNode_t;
    loop {
        curNode = snoopy_util_list_fetchNextNode(snoopy_tsrm_threadRepo, curNode);
        if curNode.is_null() {
            break;
        }
        if ((*curNode).value).is_null() {
            continue;
        }
        tData = (*curNode).value as *const snoopy_tsrm_threadData_t;
        if !(0 as libc::c_int != pthread_equal(myThreadId, (*tData).threadId)) {
            continue;
        }
        tRepoEntry = curNode;
        break;
    }
    pthread_mutex_unlock(&mut snoopy_tsrm_threadRepo_mutex);
    return tRepoEntry;
}
pub unsafe extern "C" fn snoopy_tsrm_getCurrentThreadData() -> *mut snoopy_tsrm_threadData_t {
    let mut tRepoEntry: *mut listNode_t = 0 as *mut listNode_t;
    tRepoEntry = snoopy_tsrm_getCurrentThreadRepoEntry();
    if tRepoEntry.is_null() {
        return 0 as *mut snoopy_tsrm_threadData_t;
    }
    return (*tRepoEntry).value as *mut snoopy_tsrm_threadData_t;
}
pub unsafe extern "C" fn snoopy_tsrm_get_configuration() -> *mut snoopy_configuration_t {
    let mut tData: *mut snoopy_tsrm_threadData_t = 0 as *mut snoopy_tsrm_threadData_t;
    tData = snoopy_tsrm_getCurrentThreadData();
    return (*tData).configuration;
}
pub unsafe extern "C" fn snoopy_tsrm_get_inputdatastorage() -> *mut snoopy_inputdatastorage_t {
    let mut tData: *mut snoopy_tsrm_threadData_t = 0 as *mut snoopy_tsrm_threadData_t;
    tData = snoopy_tsrm_getCurrentThreadData();
    return (*tData).inputdatastorage;
}
pub unsafe extern "C" fn snoopy_tsrm_get_threadCount() -> libc::c_int {
    let mut threadCount: libc::c_int = 0;
    pthread_mutex_lock(&mut snoopy_tsrm_threadRepo_mutex);
    threadCount = (*snoopy_tsrm_threadRepo).count;
    pthread_mutex_unlock(&mut snoopy_tsrm_threadRepo_mutex);
    return threadCount;
}
