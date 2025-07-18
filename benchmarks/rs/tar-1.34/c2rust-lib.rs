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
extern crate f128;#[macro_use]
extern crate num_traits;
extern crate libc;
pub mod gnu {
pub mod acl_errno_valid;
pub mod acl_internal;
pub mod allocator;
pub mod areadlink;
pub mod areadlink_with_size;
pub mod areadlinkat;
pub mod areadlinkat_with_size;
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
pub mod backup_find;
pub mod backupfile;
pub mod basename;
pub mod basename_lgpl;
pub mod bitrotate;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strncasecmp;
pub mod careadlinkat;
pub mod chdir_long;
pub mod cloexec;
pub mod close_stream;
pub mod closeout;
pub mod creat_safer;
pub mod dirname;
pub mod dirname_lgpl;
pub mod dup_safer;
pub mod dup_safer_flag;
pub mod exclude;
pub mod exitfail;
pub mod fchmodat;
pub mod fcntl;
pub mod fd_hook;
pub mod fd_safer;
pub mod fd_safer_flag;
pub mod fdutimensat;
pub mod file_has_acl;
pub mod filenamecat_lgpl;
pub mod fprintftime;
pub mod free;
pub mod full_write;
pub mod get_permissions;
pub mod getfilecon;
pub mod getopt;
pub mod getopt1;
pub mod getprogname;
pub mod gettime;
pub mod hard_locale;
pub mod hash;
pub mod human;
pub mod imaxtostr;
pub mod inttostr;
pub mod lchmod;
pub mod localcharset;
pub mod malloc {
pub mod scratch_buffer_dupfree;
pub mod scratch_buffer_grow;
pub mod scratch_buffer_grow_preserve;
pub mod scratch_buffer_set_array_size;
} // mod malloc
pub mod malloca;
pub mod mbchar;
pub mod mbrtowc;
pub mod mbscasecmp;
pub mod mbuiter;
pub mod mktime;
pub mod modechange;
pub mod nstrftime;
pub mod obstack;
pub mod offtostr;
pub mod open_safer;
pub mod openat_die;
pub mod openat_proc;
pub mod openat_safer;
pub mod opendir_safer;
pub mod opendirat;
pub mod parse_datetime;
pub mod pipe_safer;
pub mod printf_args;
pub mod printf_parse;
pub mod priv_set;
pub mod progname;
pub mod quotearg;
pub mod regex;
pub mod renameatu;
pub mod safe_read;
pub mod safe_write;
pub mod save_cwd;
pub mod savedir;
pub mod se_context;
pub mod se_label;
pub mod se_selinux;
pub mod selinux_at;
pub mod set_permissions;
pub mod setlocale_null;
pub mod stat_time;
pub mod statat;
pub mod stdopen;
pub mod stripslash;
pub mod strnlen1;
pub mod tempname;
pub mod time_rz;
pub mod timespec;
pub mod timespec_sub;
pub mod uinttostr;
pub mod umaxtostr;
pub mod unistd;
pub mod uniwidth {
pub mod width;
} // mod uniwidth
pub mod unlinkdir;
pub mod utimens;
pub mod vasnprintf;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod wctype_h;
pub mod xalloc_die;
pub mod xasprintf;
pub mod xgetcwd;
pub mod xmalloc;
pub mod xsize;
pub mod xstrndup;
pub mod xstrtol;
pub mod xstrtoul;
pub mod xstrtoumax;
pub mod xvasprintf;
} // mod gnu
pub mod lib {
pub mod paxerror;
pub mod paxexit_status;
pub mod paxnames;
pub mod rtapelib;
pub mod wordsplit;
pub mod xattr_at;
} // mod lib
pub mod rmt {
pub mod rmt;
} // mod rmt
pub mod src {
pub mod buffer;
pub mod checkpoint;
pub mod compare;
pub mod create;
pub mod delete;
pub mod exclist;
pub mod exit;
pub mod extract;
pub mod incremen;
pub mod list;
pub mod map;
pub mod misc;
pub mod names;
pub mod sparse;
pub mod suffix;
pub mod system;
pub mod tar;
pub mod transform;
pub mod unlink;
pub mod update;
pub mod utf8;
pub mod warning;
pub mod xattrs;
pub mod xheader;
} // mod src
