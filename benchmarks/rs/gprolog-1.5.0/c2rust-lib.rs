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
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod BipsFD {
pub mod fd_bool_c;
pub mod fd_infos_c;
pub mod fd_math_c;
pub mod fd_prime_c;
pub mod fd_symbolic_c;
pub mod fd_values_c;
pub mod math_supp;
pub mod oper_supp;
} // mod BipsFD
pub mod BipsPl {
pub mod all_solut_c;
pub mod arith_inl_c;
pub mod assert_c;
pub mod atom_c;
pub mod bc_supp;
pub mod c_supp;
pub mod call_args_c;
pub mod callinf_supp;
pub mod char_io_c;
pub mod const_io_c;
pub mod consult_c;
pub mod control_c;
pub mod debugger_c;
pub mod dynam_supp;
pub mod error_supp;
pub mod expand_c;
pub mod file_c;
pub mod flag_c;
pub mod flag_supp;
pub mod foreign_supp;
pub mod format_c;
pub mod g_var_inl_c;
pub mod le_interf_c;
pub mod list_c;
pub mod oper_c;
pub mod os_interf_c;
pub mod parse_supp;
pub mod pred_c;
pub mod pred_supp;
pub mod pretty_c;
pub mod random_c;
pub mod read_c;
pub mod scan_supp;
pub mod sockets_c;
pub mod sort_c;
pub mod src_rdr_c;
pub mod stat_c;
pub mod stream_c;
pub mod stream_supp;
pub mod term_inl_c;
pub mod term_supp;
pub mod throw_c;
pub mod top_level_c;
pub mod type_inl_c;
pub mod write_c;
pub mod write_supp;
} // mod BipsPl
pub mod EngineFD {
pub mod fd_inst;
pub mod fd_range;
} // mod EngineFD
pub mod EnginePl {
pub mod atom;
pub mod cpp_headers;
pub mod engine;
pub mod engine1;
pub mod hash;
pub mod hash_fct;
pub mod if_no_fd;
pub mod machine;
pub mod machine1;
pub mod main;
pub mod mem_alloc;
pub mod misc;
pub mod obj_chain;
pub mod oper;
pub mod pl_config;
pub mod pred;
pub mod stacks_sigsegv;
pub mod wam_inst;
} // mod EnginePl
pub mod Linedit {
pub mod ctrl_c;
pub mod linedit;
pub mod terminal;
} // mod Linedit
pub mod Ma2Asm {
pub mod ma2asm;
pub mod ma2asm_inst;
pub mod ma_parser;
} // mod Ma2Asm
pub mod TopComp {
pub mod hexfilter;
pub mod top_comp;
pub mod top_level_main;
} // mod TopComp
pub mod Wam2Ma {
pub mod wam2ma;
pub mod wam_parser;
} // mod Wam2Ma
