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
pub mod fflush;
pub mod file_has_acl;
pub mod fpurge;
pub mod freading;
pub mod fseek;
pub mod fseeko;
pub mod glthread {
pub mod thread;
} // mod glthread
pub mod ioctl;
pub mod locale;
pub mod localename;
pub mod localename_table;
pub mod nanosleep;
pub mod pthread_sigmask;
pub mod read_file;
pub mod sockets;
pub mod strerror_r;
pub mod sys_socket;
pub mod test_localcharset;
pub mod vma_iter;
} // mod gnulib_tests
pub mod lib {
pub mod acl_errno_valid;
pub mod acl_internal;
pub mod basename_lgpl;
pub mod binary_io;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strncasecmp;
pub mod cloexec;
pub mod close_stream;
pub mod closeout;
pub mod copy_acl;
pub mod dfa;
pub mod dirname_lgpl;
pub mod exitfail;
pub mod fcntl;
pub mod fd_hook;
pub mod free;
pub mod get_permissions;
pub mod getfilecon;
pub mod getprogname;
pub mod glthread {
pub mod lock;
pub mod threadlib;
} // mod glthread
pub mod hard_locale;
pub mod ialloc;
pub mod localcharset;
pub mod localeinfo;
pub mod malloc {
pub mod dynarray_at_failure;
pub mod dynarray_emplace_enlarge;
pub mod dynarray_finalize;
pub mod dynarray_resize;
pub mod dynarray_resize_clear;
pub mod scratch_buffer_grow;
pub mod scratch_buffer_grow_preserve;
pub mod scratch_buffer_set_array_size;
} // mod malloc
pub mod malloca;
pub mod mbrlen;
pub mod mbrtowc;
pub mod obstack;
pub mod progname;
pub mod qcopy_acl;
pub mod qset_acl;
pub mod quotearg;
pub mod regex;
pub mod se_context;
pub mod se_label;
pub mod se_selinux;
pub mod set_acl;
pub mod set_permissions;
pub mod setlocale_null;
pub mod stat_time;
pub mod stripslash;
pub mod tempname;
pub mod unistd;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod wctype_h;
pub mod xalloc_die;
pub mod xmalloc;
} // mod lib
pub mod sed {
pub mod compile;
pub mod debug;
pub mod execute;
pub mod mbcs;
pub mod regexp;
pub mod sed;
pub mod utils;
pub mod version;
} // mod sed
