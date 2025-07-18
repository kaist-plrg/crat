use ::libc;
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
    fn merge(
        tostdout: bool,
        edarg: *const libc::c_char,
        three_manifestations: *mut symdef,
    ) -> libc::c_int;
    fn thank_you_and_goodnight(how: libc::c_int);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const libc::c_char);
    fn display_version(prog: *const program, flags: libc::c_int);
    fn check_hv(argc: libc::c_int, argv: *mut *mut libc::c_char, prog: *const program);
    fn generic_error(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
    fn generic_fatal(who: *const libc::c_char, fmt: *const libc::c_char, _: ...);
}
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
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
static mut merge_blurb: [libc::c_char; 22] = unsafe {
    *::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"Three-way file merge.\0")
};
static mut merge_help: [libc::c_char; 494] = unsafe {
    *::std::mem::transmute::<
        &[u8; 494],
        &[libc::c_char; 494],
    >(
        b"[options] receiving-sibling parent other-sibling\nOptions:\n  -A            Use `diff3 -A' style.\n  -E            Use `diff3 -E' style (default).\n  -e            Use `diff3 -e' style.\n  -p            Write to stdout instead of overwriting RECEIVING-SIBLING.\n  -q            Quiet mode; suppress conflict warnings.\n  -L LABEL      (up to three times) Specify the conflict labels for\n                RECEIVING-SIBLING, PARENT and OTHER-SIBLING, respectively.\n  -V            Obsolete; do not use.\n\0",
    )
};
pub static mut top: *mut top = 0 as *const top as *mut top;
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const libc::c_char,
            name: 0 as *const libc::c_char,
            desc: merge_blurb.as_ptr(),
            help: merge_help.as_ptr(),
            tyag: (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        };
        init
    }
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut a: *const libc::c_char = 0 as *const libc::c_char;
    let mut three_manifestations: [symdef; 3] = [symdef {
        meaningful: 0 as *const libc::c_char,
        underlying: 0 as *const libc::c_char,
    }; 3];
    let mut edarg: *const libc::c_char = 0 as *const libc::c_char;
    let mut labels: libc::c_int = 0;
    let mut exitstatus: libc::c_int = 0;
    let mut tostdout: bool = 0 as libc::c_int != 0;
    program.invoke = *argv.offset(0 as libc::c_int as isize);
    program.name = b"merge\0" as *const u8 as *const libc::c_char;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    labels = 0 as libc::c_int;
    let mut current_block_24: u64;
    loop {
        argv = argv.offset(1);
        a = *argv;
        if !(!a.is_null()
            && {
                let fresh0 = a;
                a = a.offset(1);
                *fresh0 as libc::c_int == '-' as i32
            })
        {
            break;
        }
        let fresh1 = a;
        a = a.offset(1);
        match *fresh1 as libc::c_int {
            65 | 69 | 101 => {
                if !edarg.is_null()
                    && *edarg.offset(1 as libc::c_int as isize) as libc::c_int
                        != *(*argv).offset(1 as libc::c_int as isize) as libc::c_int
                {
                    generic_error(
                        0 as *const libc::c_char,
                        b"%s and %s are incompatible\0" as *const u8
                            as *const libc::c_char,
                        edarg,
                        *argv,
                    );
                }
                edarg = *argv;
                current_block_24 = 7172762164747879670;
            }
            112 => {
                tostdout = 1 as libc::c_int != 0;
                current_block_24 = 7172762164747879670;
            }
            113 => {
                (*top).behavior.quiet = 1 as libc::c_int != 0;
                current_block_24 = 7172762164747879670;
            }
            76 => {
                if 3 as libc::c_int <= labels {
                    generic_fatal(
                        0 as *const libc::c_char,
                        b"too many -L options\0" as *const u8 as *const libc::c_char,
                    );
                }
                argv = argv.offset(1);
                let fresh2 = labels;
                labels = labels + 1;
                three_manifestations[fresh2 as usize].meaningful = *argv;
                if (three_manifestations[fresh2 as usize].meaningful).is_null() {
                    generic_fatal(
                        0 as *const libc::c_char,
                        b"-L needs following argument\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                argc -= 1;
                argc;
                current_block_24 = 7172762164747879670;
            }
            86 => {
                if *a.offset(0 as libc::c_int as isize) != 0 {
                    bad_option(a.offset(-(2 as libc::c_int as isize)));
                } else {
                    display_version(&mut program, 1 as libc::c_int);
                }
                gnurcs_goodbye();
                return if *a.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
            }
            _ => {
                bad_option(a.offset(-(2 as libc::c_int as isize)));
                current_block_24 = 7815301370352969686;
            }
        }
        match current_block_24 {
            7172762164747879670 => {
                if *a != 0 {
                    bad_option(a.offset(-(2 as libc::c_int as isize)));
                }
            }
            _ => {}
        }
        argc -= 1;
        argc;
    }
    if argc != 4 as libc::c_int {
        generic_fatal(
            0 as *const libc::c_char,
            b"%s arguments\0" as *const u8 as *const libc::c_char,
            if argc < 4 as libc::c_int {
                b"not enough\0" as *const u8 as *const libc::c_char
            } else {
                b"too many\0" as *const u8 as *const libc::c_char
            },
        );
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        three_manifestations[i as usize].underlying = *argv.offset(i as isize);
        if labels <= i {
            three_manifestations[i as usize]
                .meaningful = three_manifestations[i as usize].underlying;
        }
        i += 1;
        i;
    }
    if (*top).flow.erroneous {
        thank_you_and_goodnight((*(*top).program).tyag);
    }
    exitstatus = merge(tostdout, edarg, three_manifestations.as_mut_ptr());
    gnurcs_goodbye();
    return exitstatus;
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
