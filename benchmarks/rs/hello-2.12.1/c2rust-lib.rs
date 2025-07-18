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
pub mod basename;
pub mod basename_lgpl;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strncasecmp;
pub mod cloexec;
pub mod close_stream;
pub mod closeout;
pub mod dirname;
pub mod dirname_lgpl;
pub mod exitfail;
pub mod fcntl;
pub mod fd_hook;
pub mod getprogname;
pub mod hard_locale;
pub mod localcharset;
pub mod malloca;
pub mod mbchar;
pub mod mbiter;
pub mod mbrtowc;
pub mod mbslen;
pub mod mbsstr;
pub mod mbuiter;
pub mod progname;
pub mod propername;
pub mod quotearg;
pub mod setlocale_null;
pub mod stat_time;
pub mod striconv;
pub mod stripslash;
pub mod strnlen1;
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
pub mod xmalloc;
pub mod xstriconv;
pub mod xstrndup;
} // mod lib
pub mod src {
pub mod hello;
} // mod src
