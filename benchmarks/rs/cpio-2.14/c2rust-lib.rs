#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod gnu {
pub mod argmatch;
pub mod argp_ba;
pub mod argp_eexst;
pub mod argp_fmtstream;
pub mod argp_fs_xinl;
pub mod argp_help;
pub mod argp_parse;
pub mod argp_pin;
pub mod argp_pv;
pub mod argp_pvh;
pub mod argp_version_etc;
pub mod argp_xinl;
pub mod asnprintf;
pub mod basename;
pub mod basename_lgpl;
pub mod bitrotate;
pub mod btowc;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strncasecmp;
pub mod chdir_long;
pub mod cloexec;
pub mod close_stream;
pub mod closeout;
pub mod dirfd;
pub mod dirname;
pub mod dirname_lgpl;
pub mod dup_safer;
pub mod dup_safer_flag;
pub mod exitfail;
pub mod fcntl;
pub mod fd_hook;
pub mod fd_safer;
pub mod fd_safer_flag;
pub mod filenamecat_lgpl;
pub mod free;
pub mod full_write;
pub mod getopt;
pub mod getopt1;
pub mod getprogname;
pub mod gettime;
pub mod hard_locale;
pub mod hash;
pub mod ialloc;
pub mod imaxtostr;
pub mod inttostr;
pub mod localcharset;
pub mod malloca;
pub mod mbrtowc;
pub mod mbsrtowcs;
pub mod mbsrtowcs_state;
pub mod mktime;
pub mod nstrftime;
pub mod obstack;
pub mod offtostr;
pub mod openat_die;
pub mod openat_proc;
pub mod opendir_safer;
pub mod parse_datetime;
pub mod pipe_safer;
pub mod printf_args;
pub mod printf_parse;
pub mod progname;
pub mod quotearg;
pub mod safe_read;
pub mod safe_write;
pub mod save_cwd;
pub mod savedir;
pub mod setlocale_null;
pub mod stat_time;
pub mod stripslash;
pub mod strnlen1;
pub mod strtol;
pub mod strtoll;
pub mod strtoull;
pub mod time_rz;
pub mod timegm;
pub mod timespec;
pub mod uinttostr;
pub mod umaxtostr;
pub mod unistd;
pub mod utimens;
pub mod vasnprintf;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod wctype_h;
pub mod xalloc_die;
pub mod xgetcwd;
pub mod xmalloc;
pub mod xsize;
} // mod gnu
pub mod lib {
pub mod paxerror;
pub mod paxexit;
pub mod paxexit_status;
pub mod paxnames;
pub mod rtapelib;
pub mod sysdep;
} // mod lib
pub mod rmt {
pub mod rmt;
} // mod rmt
pub mod src {
pub mod copyin;
pub mod copyout;
pub mod copypass;
pub mod defer;
pub mod dstring;
pub mod fatal;
pub mod filemode;
pub mod global;
pub mod idcache;
pub mod main;
pub mod makepath;
pub mod tar;
pub mod userspec;
pub mod util;
} // mod src
