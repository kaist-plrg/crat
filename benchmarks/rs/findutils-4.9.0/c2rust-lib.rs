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
pub mod find {
pub mod exec;
pub mod finddata;
pub mod fstype;
pub mod ftsfind;
pub mod parser;
pub mod pred;
pub mod print;
pub mod sharefile;
pub mod tree;
pub mod util;
} // mod find
pub mod gl {
pub mod lib {
pub mod allocator;
pub mod areadlink;
pub mod areadlinkat;
pub mod argmatch;
pub mod argv_iter;
pub mod asnprintf;
pub mod basename;
pub mod basename_lgpl;
pub mod bitrotate;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strcasestr;
pub mod c_strncasecmp;
pub mod c_strstr;
pub mod canonicalize;
pub mod careadlinkat;
pub mod chdir_long;
pub mod cloexec;
pub mod close_stream;
pub mod closein;
pub mod closeout;
pub mod creat_safer;
pub mod cycle_check;
pub mod dirname;
pub mod dirname_lgpl;
pub mod dup_safer;
pub mod dup_safer_flag;
pub mod exitfail;
pub mod fclose;
pub mod fcntl;
pub mod fd_hook;
pub mod fd_safer;
pub mod fd_safer_flag;
pub mod fflush;
pub mod file_set;
pub mod filemode;
pub mod filenamecat_lgpl;
pub mod fopen_safer;
pub mod fpurge;
pub mod freadahead;
pub mod freading;
pub mod free;
pub mod fseek;
pub mod fseeko;
pub mod fts;
pub mod getfilecon;
pub mod getprogname;
pub mod gettime;
pub mod glthread {
pub mod lock;
pub mod threadlib;
} // mod glthread
pub mod hard_locale;
pub mod hash;
pub mod hash_pjw;
pub mod hash_triple_simple;
pub mod human;
pub mod i_ring;
pub mod ialloc;
pub mod idcache;
pub mod localcharset;
pub mod malloc {
pub mod dynarray_at_failure;
pub mod dynarray_emplace_enlarge;
pub mod dynarray_finalize;
pub mod dynarray_resize;
pub mod dynarray_resize_clear;
pub mod scratch_buffer_dupfree;
pub mod scratch_buffer_grow;
pub mod scratch_buffer_grow_preserve;
pub mod scratch_buffer_set_array_size;
} // mod malloc
pub mod malloca;
pub mod math;
pub mod mbchar;
pub mod mbrtowc;
pub mod mbscasestr;
pub mod mbslen;
pub mod mbsstr;
pub mod mbswidth;
pub mod mbuiter;
pub mod mktime;
pub mod modechange;
pub mod mountlist;
pub mod nstrftime;
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
pub mod progname;
pub mod quotearg;
pub mod regex;
pub mod safe_read;
pub mod save_cwd;
pub mod savedir;
pub mod se_context;
pub mod se_label;
pub mod se_selinux;
pub mod selinux_at;
pub mod setlocale_null;
pub mod sockets;
pub mod stat_time;
pub mod statat;
pub mod strerror_r;
pub mod stripslash;
pub mod strnlen1;
pub mod sys_socket;
pub mod time_rz;
pub mod timespec;
pub mod unistd;
pub mod uniwidth {
pub mod width;
} // mod uniwidth
pub mod vasnprintf;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod wctype_h;
pub mod xalloc_die;
pub mod xgetcwd;
pub mod xmalloc;
pub mod xsize;
pub mod xstrtod;
pub mod xstrtol;
pub mod xstrtol_error;
pub mod xstrtoul;
pub mod xstrtoumax;
pub mod yesno;
} // mod lib
} // mod gl
pub mod gnulib_tests {
pub mod binary_io;
pub mod dtotimespec;
pub mod glthread {
pub mod thread;
} // mod glthread
pub mod imaxtostr;
pub mod inttostr;
pub mod ioctl;
pub mod locale;
pub mod localename;
pub mod localename_table;
pub mod nanosleep;
pub mod offtostr;
pub mod priv_set;
pub mod pthread_sigmask;
pub mod sig_handler;
pub mod tempname;
pub mod test_localcharset;
pub mod timespec_add;
pub mod timespec_sub;
pub mod tmpdir;
pub mod uinttostr;
pub mod umaxtostr;
pub mod unlinkdir;
} // mod gnulib_tests
pub mod lib {
pub mod bugreports;
pub mod buildcmd;
pub mod dircallback;
pub mod extendbuf;
pub mod fdleak;
pub mod findutils_version;
pub mod listfile;
pub mod printquoted;
pub mod qmark;
pub mod regextype;
pub mod safe_atoi;
pub mod splitstring;
} // mod lib
pub mod locate {
pub mod frcode;
pub mod locate;
pub mod word_io;
} // mod locate
pub mod xargs {
pub mod xargs;
} // mod xargs
