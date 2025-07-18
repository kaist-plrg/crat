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
pub mod dtotimespec;
pub mod glthread {
pub mod thread;
} // mod glthread
pub mod hash_pjw;
pub mod ioctl;
pub mod locale;
pub mod nanosleep;
pub mod printf_args;
pub mod printf_parse;
pub mod pthread_sigmask;
pub mod sockets;
pub mod strerror_r;
pub mod sys_socket;
pub mod test_localcharset;
pub mod time;
pub mod timespec_add;
pub mod timespec_sub;
pub mod vasnprintf;
pub mod xsize;
pub mod xstrtol_error;
} // mod gnulib_tests
pub mod lib {
pub mod allocator;
pub mod areadlink;
pub mod argmatch;
pub mod basename;
pub mod basename_lgpl;
pub mod binary_io;
pub mod bitrotate;
pub mod btowc;
pub mod c_ctype;
pub mod c_stack;
pub mod c_strcasecmp;
pub mod c_strncasecmp;
pub mod careadlinkat;
pub mod cloexec;
pub mod cmpbuf;
pub mod dirname;
pub mod dirname_lgpl;
pub mod exclude;
pub mod exitfail;
pub mod fcntl;
pub mod fd_hook;
pub mod file_type;
pub mod filenamecat;
pub mod filenamecat_lgpl;
pub mod fopen;
pub mod free;
pub mod getprogname;
pub mod gettime;
pub mod glthread {
pub mod lock;
pub mod threadlib;
} // mod glthread
pub mod hard_locale;
pub mod hash;
pub mod ialloc;
pub mod imaxtostr;
pub mod inttostr;
pub mod localcharset;
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
pub mod mbrtowc;
pub mod mbscasecmp;
pub mod mbslen;
pub mod mbsrtowcs;
pub mod mbsrtowcs_state;
pub mod mbsstr;
pub mod mbuiter;
pub mod mktime;
pub mod nstrftime;
pub mod offtostr;
pub mod progname;
pub mod propername;
pub mod quotearg;
pub mod regex;
pub mod setlocale_null;
pub mod sh_quote;
pub mod sigsegv;
pub mod stackvma;
pub mod stat_time;
pub mod stdopen;
pub mod striconv;
pub mod stripslash;
pub mod strnlen1;
pub mod strtoll;
pub mod system_quote;
pub mod tempname;
pub mod time_rz;
pub mod timegm;
pub mod timespec;
pub mod trim;
pub mod uinttostr;
pub mod umaxtostr;
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
pub mod xfreopen;
pub mod xmalloc;
pub mod xmalloca;
pub mod xreadlink;
pub mod xstdopen;
pub mod xstriconv;
pub mod xstrtoimax;
pub mod xstrtol;
pub mod xstrtoul;
} // mod lib
pub mod src {
pub mod analyze;
pub mod cmp;
pub mod context;
pub mod diff;
pub mod diff3;
pub mod dir;
pub mod ed;
pub mod ifdef;
pub mod io;
pub mod normal;
pub mod sdiff;
pub mod side;
pub mod util;
pub mod version;
} // mod src
