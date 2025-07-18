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


extern crate libc;
pub mod lib {
pub mod argmatch;
pub mod asnprintf;
pub mod backup_find;
pub mod backupfile;
pub mod basename;
pub mod basename_lgpl;
pub mod bitrotate;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strncasecmp;
pub mod chdir_long;
pub mod chmodat;
pub mod chownat;
pub mod cloexec;
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
pub mod full_write;
pub mod getprogname;
pub mod gettime;
pub mod gl_linked_list;
pub mod gl_list;
pub mod gl_xlist;
pub mod hard_locale;
pub mod hash;
pub mod localcharset;
pub mod localtime_buffer;
pub mod malloca;
pub mod mbrtowc;
pub mod mktime;
pub mod nstrftime;
pub mod openat_die;
pub mod openat_proc;
pub mod opendir_safer;
pub mod parse_datetime;
pub mod pipe_safer;
pub mod printf_args;
pub mod printf_parse;
pub mod progname;
pub mod quotearg;
pub mod renameat2;
pub mod safe_write;
pub mod save_cwd;
pub mod stat_time;
pub mod statat;
pub mod stripslash;
pub mod tempname;
pub mod time_rz;
pub mod timespec;
pub mod unistd;
pub mod utimens;
pub mod vasnprintf;
pub mod verror;
pub mod wctype_h;
pub mod xalloc_die;
pub mod xasprintf;
pub mod xmalloc;
pub mod xmemdup0;
pub mod xsize;
pub mod xstrndup;
pub mod xvasprintf;
} // mod lib
pub mod src {
pub mod inp;
pub mod merge;
pub mod patch;
pub mod pch;
pub mod safe;
pub mod util;
pub mod version;
} // mod src
