#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod lib {
pub mod asnprintf;
pub mod basename_lgpl;
pub mod chdir_long;
pub mod cloexec;
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
pub mod getprogname;
pub mod gettime;
pub mod glob;
pub mod glob_pattern_p;
pub mod globfree;
pub mod glthread {
pub mod lock;
pub mod threadlib;
} // mod glthread
pub mod hard_locale;
pub mod localcharset;
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
pub mod math;
pub mod mbrtowc;
pub mod openat_die;
pub mod openat_proc;
pub mod pipe_safer;
pub mod printf_args;
pub mod printf_frexp;
pub mod printf_frexpl;
pub mod printf_parse;
pub mod regex;
pub mod save_cwd;
pub mod setlocale_null;
pub mod sig_handler;
pub mod stat_time;
pub mod stripslash;
pub mod strnlen1;
pub mod tempname;
pub mod timespec;
pub mod unistd;
pub mod uniwidth {
pub mod width;
} // mod uniwidth
pub mod utimens;
pub mod vasnprintf;
pub mod wctype_h;
pub mod xsize;
} // mod lib
pub mod src {
pub mod browser;
pub mod chars;
pub mod color;
pub mod cut;
pub mod files;
pub mod global;
pub mod help;
pub mod history;
pub mod nano;
pub mod prompt;
pub mod r#move;
pub mod rcfile;
pub mod search;
pub mod text;
pub mod utils;
pub mod winio;
} // mod src
