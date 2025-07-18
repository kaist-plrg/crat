use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn MD5_Update(
        c: *mut MD5_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn MD5_Init(c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Final(md: *mut libc::c_uchar, c: *mut MD5_CTX) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    static mut config: shairport_cfg;
    fn debug(level: libc::c_int, format: *mut libc::c_char, _: ...);
    fn warn(format: *mut libc::c_char, _: ...);
    fn die(format: *mut libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub type MD5_CTX = MD5state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_output {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_backend {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shairport_cfg {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metadata {
    pub artist: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub album: *mut libc::c_char,
    pub artwork: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub genre: *mut libc::c_char,
}
pub static mut player_meta: metadata = metadata {
    artist: 0 as *const libc::c_char as *mut libc::c_char,
    title: 0 as *const libc::c_char as *mut libc::c_char,
    album: 0 as *const libc::c_char as *mut libc::c_char,
    artwork: 0 as *const libc::c_char as *mut libc::c_char,
    comment: 0 as *const libc::c_char as *mut libc::c_char,
    genre: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut fd: libc::c_int = -(1 as libc::c_int);
static mut dirty: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn metadata_set(
    mut field: *mut *mut libc::c_char,
    mut value: *const libc::c_char,
) {
    if !(*field).is_null() {
        if strcmp(*field, value) == 0 {
            return;
        }
        free(*field as *mut libc::c_void);
    }
    *field = strdup(value);
    dirty = 1 as libc::c_int;
}
pub unsafe extern "C" fn metadata_open() {
    if (config.meta_dir).is_null() {
        return;
    }
    let fn_0: [libc::c_char; 12] = *::std::mem::transmute::<
        &[u8; 12],
        &[libc::c_char; 12],
    >(b"now_playing\0");
    let mut pl: size_t = (strlen(config.meta_dir))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(fn_0.as_ptr()));
    let mut path: *mut libc::c_char = malloc(
        pl.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    snprintf(
        path,
        pl.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        config.meta_dir,
        fn_0.as_ptr(),
    );
    if mkfifo(path, 0o644 as libc::c_int as __mode_t) != 0
        && *__errno_location() != 17 as libc::c_int
    {
        die(
            b"Could not create metadata FIFO %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            path,
        );
    }
    fd = open(path, 0o1 as libc::c_int | 0o4000 as libc::c_int);
    if fd < 0 as libc::c_int {
        debug(
            1 as libc::c_int,
            b"Could not open metadata FIFO %s. Will try again later.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            path,
        );
    }
    free(path as *mut libc::c_void);
}
unsafe extern "C" fn metadata_close() {
    close(fd);
    fd = -(1 as libc::c_int);
}
unsafe extern "C" fn print_one(
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    write(fd, name as *const libc::c_void, strlen(name));
    write(
        fd,
        b"=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    if !value.is_null() {
        write(fd, value as *const libc::c_void, strlen(value));
    }
    write(
        fd,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn metadata_write() {
    let mut ret: libc::c_int = 0;
    if fd < 0 as libc::c_int {
        metadata_open();
    }
    if fd < 0 as libc::c_int {
        return;
    }
    if dirty == 0 {
        return;
    }
    dirty = 0 as libc::c_int;
    print_one(b"artist\0" as *const u8 as *const libc::c_char, player_meta.artist);
    print_one(b"title\0" as *const u8 as *const libc::c_char, player_meta.title);
    print_one(b"album\0" as *const u8 as *const libc::c_char, player_meta.album);
    print_one(b"artwork\0" as *const u8 as *const libc::c_char, player_meta.artwork);
    print_one(b"genre\0" as *const u8 as *const libc::c_char, player_meta.genre);
    print_one(b"comment\0" as *const u8 as *const libc::c_char, player_meta.comment);
    ret = write(
        fd,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) as libc::c_int;
    if ret < 1 as libc::c_int {
        metadata_close();
    }
}
pub unsafe extern "C" fn metadata_cover_image(
    mut buf: *const libc::c_char,
    mut len: libc::c_int,
    mut ext: *const libc::c_char,
) {
    if (config.meta_dir).is_null() {
        return;
    }
    if !buf.is_null() {
        debug(
            1 as libc::c_int,
            b"Cover Art set\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        debug(
            1 as libc::c_int,
            b"Cover Art cleared\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    let mut img_md5: [uint8_t; 16] = [0; 16];
    let mut ctx: MD5_CTX = MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    MD5_Init(&mut ctx);
    MD5_Update(&mut ctx, buf as *const libc::c_void, len as size_t);
    MD5_Final(img_md5.as_mut_ptr(), &mut ctx);
    let mut img_md5_str: [libc::c_char; 33] = [0; 33];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        sprintf(
            &mut *img_md5_str.as_mut_ptr().offset((i * 2 as libc::c_int) as isize)
                as *mut libc::c_char,
            b"%02x\0" as *const u8 as *const libc::c_char,
            img_md5[i as usize] as libc::c_int,
        );
        i += 1;
        i;
    }
    let mut dir: *mut libc::c_char = config.meta_dir;
    let mut prefix: *mut libc::c_char = b"cover-\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let mut pl: size_t = (strlen(dir))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(prefix))
        .wrapping_add(strlen(img_md5_str.as_mut_ptr()))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(ext));
    let mut path: *mut libc::c_char = malloc(
        pl.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    snprintf(
        path,
        pl.wrapping_add(1 as libc::c_int as libc::c_ulong),
        b"%s/%s%s.%s\0" as *const u8 as *const libc::c_char,
        dir,
        prefix,
        img_md5_str.as_mut_ptr(),
        ext,
    );
    let mut cover_fd: libc::c_int = open(
        path,
        0o1 as libc::c_int | 0o100 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if cover_fd < 0 as libc::c_int {
        warn(
            b"Could not open file %s for writing cover art\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            path,
        );
        return;
    }
    if write(cover_fd, buf as *const libc::c_void, len as size_t) < len as libc::c_long {
        warn(
            b"writing %s failed\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            path,
        );
        free(path as *mut libc::c_void);
        return;
    }
    close(cover_fd);
    debug(
        1 as libc::c_int,
        b"Cover Art file is %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        path,
    );
    metadata_set(
        &mut player_meta.artwork,
        path.offset(strlen(dir) as isize).offset(1 as libc::c_int as isize),
    );
    free(path as *mut libc::c_void);
}
