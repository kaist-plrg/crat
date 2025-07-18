use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn looking_at(sym: *const tinysym, start: *const libc::c_char) -> bool;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn str_save(s: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn nice_getopt(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        longopts: *const option,
    ) -> libc::c_int;
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    static mut peer_super: symdef;
    fn one_beyond_last_dir_sep(name: *const libc::c_char) -> *const libc::c_char;
    fn find_peer_prog(prog: *mut symdef) -> *const libc::c_char;
    static ya_ci: yacmd;
    static ya_co: yacmd;
    static ya_rcs: yacmd;
    static ya_rcsclean: yacmd;
    static ya_rcsdiff: yacmd;
    static ya_rcsmerge: yacmd;
    static ya_rlog: yacmd;
}
pub type __uint8_t = libc::c_uchar;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
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
pub type off_t = __off_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbuf {
    pub string: *const libc::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta {
    pub num: *const libc::c_char,
    pub date: *const libc::c_char,
    pub author: *const libc::c_char,
    pub lockedby: *const libc::c_char,
    pub state: *const libc::c_char,
    pub log: *mut atat,
    pub text: *mut atat,
    pub name: *const libc::c_char,
    pub pretty_log: cbuf,
    pub branches: *mut wlink,
    pub commitid: *const libc::c_char,
    pub ilk: *mut delta,
    pub selector: bool,
    pub neck: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symdef {
    pub meaningful: *const libc::c_char,
    pub underlying: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const libc::c_char,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const libc::c_char,
    pub name: *const libc::c_char,
    pub desc: *const libc::c_char,
    pub help: *const libc::c_char,
    pub tyag: libc::c_int,
}
pub type maker = libc::c_uint;
pub const effective: maker = 2;
pub const real: maker = 1;
pub const notmade: maker = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sff {
    pub filename: *const libc::c_char,
    pub disposition: maker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct behavior {
    pub invdir: *const libc::c_char,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_Locker_in_Id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: libc::c_int,
    pub stick_with_euid: bool,
    pub ruid: libc::c_int,
    pub euid: libc::c_int,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: libc::c_int,
    pub pe: *const libc::c_char,
    pub zone_offset: zone_offset,
    pub username: *mut libc::c_char,
    pub now: timespec,
    pub fixed_SIGCHLD: bool,
    pub Oerrloop: bool,
    pub cwd: *mut libc::c_char,
    pub mem_limit: off_t,
    pub sff: *mut sff,
    pub isr: *mut isr_scratch,
    pub ephemstuff: *mut ephemstuff,
    pub maketimestuff: *mut maketimestuff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_offset {
    pub valid: bool,
    pub seconds: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut libc::c_char,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub valid: bool,
    pub author: *mut libc::c_char,
    pub date: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub rev: *mut libc::c_char,
    pub state: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repo {
    pub head: *const libc::c_char,
    pub branch: *const libc::c_char,
    pub access_count: size_t,
    pub access: *mut link,
    pub symbols_count: size_t,
    pub symbols: *mut link,
    pub locks_count: size_t,
    pub locks: *mut link,
    pub strict: bool,
    pub integrity: *mut atat,
    pub comment: *mut atat,
    pub expand: libc::c_int,
    pub deltas_count: size_t,
    pub deltas: *mut wlink,
    pub desc: *mut atat,
    pub neck: off_t,
    pub lockdefs: *mut lockdef,
    pub ht: *mut hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repository {
    pub filename: *const libc::c_char,
    pub fd_lock: libc::c_int,
    pub stat: stat,
    pub r: *mut repo,
    pub tip: *mut delta,
    pub log_lead: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow {
    pub from: *mut fro,
    pub rewr: *mut FILE,
    pub to: *mut FILE,
    pub res: *mut FILE,
    pub result: *const libc::c_char,
    pub erroneous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top {
    pub program: *const program,
    pub behavior: behavior,
    pub manifestation: manifestation,
    pub repository: repository,
    pub flow: flow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type submain_t = unsafe extern "C" fn(
    *const libc::c_char,
    libc::c_int,
    *mut *mut libc::c_char,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yacmd {
    pub func: Option::<submain_t>,
    pub aka: *const uint8_t,
    pub pr: *mut program,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynamic_root {
    pub top: *mut top,
    pub single: *mut divvy,
    pub plexus: *mut divvy,
}
pub const ddc_aliases: ddc_option_values = 2;
pub const ddc_commands: ddc_option_values = 1;
pub type ddc_option_values = libc::c_uint;
pub const ddc_unrecognized: ddc_option_values = 0;
static mut super_blurb: [libc::c_char; 25] = unsafe {
    *::std::mem::transmute::<
        &[u8; 25],
        &[libc::c_char; 25],
    >(b"Dispatch an RCS command.\0")
};
static mut super_help: [libc::c_char; 262] = unsafe {
    *::std::mem::transmute::<
        &[u8; 262],
        &[libc::c_char; 262],
    >(
        b"[options] command [command-arg ...]\nOptions:\n  --commands       Display available commands and exit.\n  --aliases        Display command aliases and exit.\n  --help COMMAND   Display help for COMMAND.\n\nTo display help for the legacy interface, use:\n  --help frob\n\0",
    )
};
unsafe extern "C" fn droot_global_to_stack(mut dr: *mut dynamic_root) {
    (*dr).top = top;
    (*dr).single = single;
    (*dr).plexus = plexus;
}
unsafe extern "C" fn droot_stack_to_global(mut dr: *mut dynamic_root) {
    top = (*dr).top;
    single = (*dr).single;
    plexus = (*dr).plexus;
}
unsafe extern "C" fn dispatch(
    mut exitval: *mut libc::c_int,
    mut sub: Option::<submain_t>,
    mut cmd: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut super_0: dynamic_root = dynamic_root {
        top: 0 as *mut top,
        single: 0 as *mut divvy,
        plexus: 0 as *mut divvy,
    };
    droot_global_to_stack(&mut super_0);
    *exitval = sub.unwrap()(cmd, argc, argv);
    droot_stack_to_global(&mut super_0);
}
static mut avail: [*const yacmd; 7] = unsafe {
    [
        &ya_ci as *const yacmd,
        &ya_co as *const yacmd,
        &ya_rcs as *const yacmd,
        &ya_rcsclean as *const yacmd,
        &ya_rcsdiff as *const yacmd,
        &ya_rcsmerge as *const yacmd,
        &ya_rlog as *const yacmd,
    ]
};
static mut n_avail: size_t = 0;
unsafe extern "C" fn recognize(mut maybe: *const libc::c_char) -> Option::<submain_t> {
    let mut mlen: size_t = strlen(maybe);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n_avail {
        let mut y: *const yacmd = avail[i as usize];
        let mut aka: *const uint8_t = (*y).aka;
        let fresh0 = aka;
        aka = aka.offset(1);
        let mut count: size_t = *fresh0 as size_t;
        loop {
            let fresh1 = count;
            count = count.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            let mut sym: *mut tinysym = aka as *mut tinysym;
            if mlen == (*sym).len as libc::c_ulong
                && looking_at(sym, maybe) as libc::c_int != 0
            {
                return (*y).func;
            }
            aka = aka.offset(((*sym).len as libc::c_int + 1 as libc::c_int) as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return None;
}
unsafe extern "C" fn string_from_sym(
    mut dest: *mut libc::c_char,
    mut sym: *const tinysym,
) {
    let mut len: size_t = (if 64 as libc::c_int > (*sym).len as libc::c_int {
        (*sym).len as libc::c_int
    } else {
        64 as libc::c_int - 1 as libc::c_int
    }) as size_t;
    let mut end: *mut libc::c_char = mempcpy(
        dest as *mut libc::c_void,
        ((*sym).bytes).as_ptr() as *const libc::c_void,
        len,
    ) as *mut libc::c_char;
    *end = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn display_commands() {
    printf(
        b"%-10s  %s\n\0" as *const u8 as *const libc::c_char,
        b"(command)\0" as *const u8 as *const libc::c_char,
        b"(description)\0" as *const u8 as *const libc::c_char,
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_avail {
        let mut y: *const yacmd = avail[i as usize];
        let mut aka: *const uint8_t = (*y).aka;
        let mut name: [libc::c_char; 64] = [0; 64];
        aka = aka.offset(1);
        string_from_sym(name.as_mut_ptr(), aka as *mut tinysym);
        printf(
            b" %-10s  %s\n\0" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            (*(*y).pr).desc,
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn display_aliases() {
    printf(
        b"%-10s  %s\n\0" as *const u8 as *const libc::c_char,
        b"(command)\0" as *const u8 as *const libc::c_char,
        b"(aliases)\0" as *const u8 as *const libc::c_char,
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_avail {
        let mut y: *const yacmd = avail[i as usize];
        let mut aka: *const uint8_t = (*y).aka;
        let fresh2 = aka;
        aka = aka.offset(1);
        let mut count: size_t = *fresh2 as size_t;
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < count {
            let mut sym: *mut tinysym = aka as *mut tinysym;
            let mut name: [libc::c_char; 64] = [0; 64];
            string_from_sym(name.as_mut_ptr(), sym);
            match j {
                0 => {
                    printf(
                        b" %-10s \0" as *const u8 as *const libc::c_char,
                        name.as_mut_ptr(),
                    );
                }
                1 => {
                    printf(
                        b" %s\0" as *const u8 as *const libc::c_char,
                        name.as_mut_ptr(),
                    );
                }
                _ => {
                    printf(
                        b", %s\0" as *const u8 as *const libc::c_char,
                        name.as_mut_ptr(),
                    );
                }
            }
            aka = aka.offset((1 as libc::c_int + (*sym).len as libc::c_int) as isize);
            j = j.wrapping_add(1);
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
}
pub static mut top: *mut top = 0 as *const top as *mut top;
unsafe extern "C" fn all_options_short_p(mut argv: *mut *mut libc::c_char) -> bool {
    let mut ok: bool = false;
    let mut i: libc::c_int = 0;
    ok = 1 as libc::c_int != 0;
    i = 1 as libc::c_int;
    while !(*argv.offset(i as isize)).is_null() {
        if '-' as i32
            != *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int
        {
            break;
        }
        if '-' as i32
            == *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int
        {
            ok = 0 as libc::c_int != 0;
            break;
        } else {
            i += 1;
            i;
        }
    }
    return ok;
}
static mut hint: [libc::c_char; 14] = unsafe {
    *::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b" (try --help)\0")
};
unsafe extern "C" fn huh(mut what: *const libc::c_char, mut argv1: *const libc::c_char) {
    generic_fatal(
        0 as *const libc::c_char,
        b"unknown %s: %s%s\0" as *const u8 as *const libc::c_char,
        what,
        argv1,
        hint.as_ptr(),
    );
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: super_blurb.as_ptr(),
            help: super_help.as_ptr(),
            tyag: 0 as libc::c_int,
        };
        init
    }
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut sub: Option::<submain_t> = None;
    let mut exitval: libc::c_int = 0 as libc::c_int;
    if 3 as libc::c_int == argc
        && strcmp(
            b"--help\0" as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        ) == 0
    {
        let mut tmp: *mut libc::c_char = *argv.offset(2 as libc::c_int as isize);
        let ref mut fresh3 = *argv.offset(2 as libc::c_int as isize);
        *fresh3 = *argv.offset(1 as libc::c_int as isize);
        let ref mut fresh4 = *argv.offset(1 as libc::c_int as isize);
        *fresh4 = tmp;
    }
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = peer_super.meaningful;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    if 2 as libc::c_int > argc
        || '-' as i32
            == *(*argv.offset(1 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int
            && all_options_short_p(argv) as libc::c_int != 0
    {
        current_block = 3525469301105603487;
    } else {
        if '-' as i32
            == *(*argv.offset(1 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int
        {
            let mut allddc: [option; 3] = [
                {
                    let mut init = option {
                        name: b"commands\0" as *const u8 as *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: ddc_commands as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = option {
                        name: b"aliases\0" as *const u8 as *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: ddc_aliases as libc::c_int,
                    };
                    init
                },
                {
                    let mut init = option {
                        name: 0 as *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 0 as libc::c_int,
                    };
                    init
                },
            ];
            match nice_getopt(argc, argv, allddc.as_mut_ptr()) {
                1 => {
                    display_commands();
                    current_block = 9386053737632237638;
                }
                2 => {
                    display_aliases();
                    current_block = 9386053737632237638;
                }
                _ => {
                    huh(
                        b"option\0" as *const u8 as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize),
                    );
                    current_block = 5948590327928692120;
                }
            }
        } else {
            current_block = 5948590327928692120;
        }
        match current_block {
            9386053737632237638 => {}
            _ => {
                cmd = *argv.offset(1 as libc::c_int as isize);
                sub = recognize(cmd);
                if sub.is_some() {
                    let ref mut fresh5 = *argv.offset(1 as libc::c_int as isize);
                    *fresh5 = if !(one_beyond_last_dir_sep(
                        *argv.offset(0 as libc::c_int as isize),
                    ))
                        .is_null()
                    {
                        *argv.offset(0 as libc::c_int as isize)
                    } else {
                        str_save(find_peer_prog(&mut peer_super))
                    };
                    dispatch(
                        &mut exitval,
                        sub,
                        cmd,
                        argc - 1 as libc::c_int,
                        argv.offset(1 as libc::c_int as isize),
                    );
                    current_block = 9386053737632237638;
                } else if !(strchr(cmd, '/' as i32)).is_null()
                    || !(0 as libc::c_int > access(cmd, 4 as libc::c_int))
                {
                    current_block = 3525469301105603487;
                } else {
                    huh(
                        b"command\0" as *const u8 as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize),
                    );
                    current_block = 9386053737632237638;
                }
            }
        }
    }
    match current_block {
        3525469301105603487 => {
            cmd = b"rcs\0" as *const u8 as *const libc::c_char;
            sub = recognize(cmd);
            dispatch(&mut exitval, sub, cmd, argc, argv);
        }
        _ => {}
    }
    gnurcs_goodbye();
    return exitval;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    n_avail = (::std::mem::size_of::<[*const yacmd; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const yacmd>() as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
