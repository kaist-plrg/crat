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
pub mod gnulib_tests {
pub mod asnprintf;
pub mod c_strcasestr;
pub mod glthread {
pub mod thread;
} // mod glthread
pub mod hash_pjw;
pub mod imaxtostr;
pub mod inttostr;
pub mod ioctl;
pub mod locale;
pub mod localename;
pub mod localename_table;
pub mod nanosleep;
pub mod offtostr;
pub mod printf_args;
pub mod printf_parse;
pub mod pthread_sigmask;
pub mod sockets;
pub mod strerror_r;
pub mod sys_socket;
pub mod test_localcharset;
pub mod time;
pub mod uinttostr;
pub mod umaxtostr;
pub mod vasnprintf;
pub mod xsize;
pub mod xstrtol_error;
} // mod gnulib_tests
pub mod lib {
pub mod argmatch;
pub mod basename_lgpl;
pub mod binary_io;
pub mod bitrotate;
pub mod btowc;
pub mod c_ctype;
pub mod c_stack;
pub mod c_strcasecmp;
pub mod c_strncasecmp;
pub mod chdir_long;
pub mod cloexec;
pub mod close_stream;
pub mod closeout;
pub mod colorize;
pub mod creat_safer;
pub mod cycle_check;
pub mod dfa;
pub mod dirfd;
pub mod dirname_lgpl;
pub mod dup_safer;
pub mod dup_safer_flag;
pub mod exclude;
pub mod exitfail;
pub mod fcntl;
pub mod fd_hook;
pub mod fd_safer;
pub mod fd_safer_flag;
pub mod filenamecat_lgpl;
pub mod fopen;
pub mod free;
pub mod fts;
pub mod getprogname;
pub mod glthread {
pub mod lock;
pub mod threadlib;
} // mod glthread
pub mod hard_locale;
pub mod hash;
pub mod i_ring;
pub mod ialloc;
pub mod localcharset;
pub mod localeinfo;
pub mod malloc {
pub mod dynarray_at_failure;
pub mod dynarray_emplace_enlarge;
pub mod dynarray_finalize;
pub mod dynarray_resize;
pub mod dynarray_resize_clear;
} // mod malloc
pub mod malloca;
pub mod mbchar;
pub mod mbiter;
pub mod mbrlen;
pub mod mbrtowc;
pub mod mbscasecmp;
pub mod mbslen;
pub mod mbsrtowcs;
pub mod mbsrtowcs_state;
pub mod mbsstr;
pub mod mbuiter;
pub mod memchr2;
pub mod obstack;
pub mod open_safer;
pub mod openat_die;
pub mod openat_proc;
pub mod openat_safer;
pub mod opendirat;
pub mod pipe_safer;
pub mod propername;
pub mod quotearg;
pub mod regex;
pub mod safe_read;
pub mod save_cwd;
pub mod setlocale_null;
pub mod sigsegv;
pub mod stackvma;
pub mod stat_time;
pub mod striconv;
pub mod stripslash;
pub mod strnlen1;
pub mod strtoll;
pub mod strtoull;
pub mod trim;
pub mod unistd;
pub mod unistr {
pub mod u8_mbtoucr;
pub mod u8_uctomb;
pub mod u8_uctomb_aux;
} // mod unistr
pub mod uniwidth {
pub mod width;
} // mod uniwidth
pub mod version_etc;
pub mod version_etc_fsf;
pub mod wctype_h;
pub mod xalloc_die;
pub mod xbinary_io;
pub mod xmalloc;
pub mod xstriconv;
pub mod xstrtoimax;
pub mod xstrtol;
pub mod xstrtoul;
} // mod lib
pub mod src {
pub mod dfasearch;
pub mod grep;
pub mod kwsearch;
pub mod kwset;
pub mod searchutils;
} // mod src
