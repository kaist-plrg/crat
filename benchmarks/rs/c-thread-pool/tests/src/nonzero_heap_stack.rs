use ::libc;
extern "C" {
    pub type thpool_;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_self() -> pthread_t;
    fn thpool_init(num_threads: libc::c_int) -> threadpool;
    fn thpool_add_work(
        _: threadpool,
        function_p: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        arg_p: *mut libc::c_void,
    ) -> libc::c_int;
    fn thpool_destroy(_: threadpool);
}
pub type pthread_t = libc::c_ulong;
pub type threadpool = *mut thpool_;
pub unsafe extern "C" fn task() {
    printf(
        b"Thread #%u working on task\n\0" as *const u8 as *const libc::c_char,
        pthread_self() as libc::c_int,
    );
}
pub unsafe extern "C" fn nonzero_stack() {
    let mut buf: [libc::c_char; 40096] = [0; 40096];
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0x80 as libc::c_int,
        40096 as libc::c_int as libc::c_ulong,
    );
}
pub unsafe extern "C" fn nonzero_heap() {
    let mut i: libc::c_int = 0;
    let mut ptrs: [*mut libc::c_void; 200] = [0 as *mut libc::c_void; 200];
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        ptrs[i
            as usize] = malloc(
            ((i + 1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong,
        );
        if !(ptrs[i as usize]).is_null() {
            memset(
                ptrs[i as usize],
                0x80 as libc::c_int,
                ((i + 1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong,
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        free(ptrs[i as usize]);
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    nonzero_stack();
    nonzero_heap();
    puts(b"Making threadpool with 4 threads\0" as *const u8 as *const libc::c_char);
    let mut thpool: threadpool = thpool_init(4 as libc::c_int);
    puts(b"Adding 20 tasks to threadpool\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        thpool_add_work(
            thpool,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    *mut libc::c_void,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(task),
                    ),
                ),
            ),
            0 as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    puts(b"Killing threadpool\0" as *const u8 as *const libc::c_char);
    thpool_destroy(thpool);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
