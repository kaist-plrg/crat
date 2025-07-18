#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(rustc_private)]


extern crate libc;
pub mod lib {
pub mod inih {
pub mod src {
pub mod ini;
} // mod src
} // mod inih
} // mod lib
pub mod src {
pub mod action {
pub mod log_message_dispatch;
pub mod log_syscall_exec;
} // mod action
pub mod cli {
pub mod action_conf;
pub mod action_disable;
pub mod action_enable;
pub mod action_status;
pub mod action_version;
pub mod cli_subroutines;
pub mod snoopy;
} // mod cli
pub mod configfile;
pub mod configuration;
pub mod datasource {
pub mod cgroup;
pub mod cmdline;
pub mod cwd;
pub mod datetime;
pub mod domain;
pub mod egid;
pub mod egroup;
pub mod env;
pub mod env_all;
pub mod euid;
pub mod eusername;
pub mod failure;
pub mod filename;
pub mod gid;
pub mod group;
pub mod hostname;
pub mod login;
pub mod noop;
pub mod pid;
pub mod ppid;
pub mod rpname;
pub mod sid;
pub mod snoopy_configure_command;
pub mod snoopy_literal;
pub mod snoopy_threads;
pub mod snoopy_version;
pub mod systemd_unit_name;
pub mod tid;
pub mod tid_kernel;
pub mod timestamp;
pub mod timestamp_ms;
pub mod timestamp_us;
pub mod tty;
pub mod tty__common;
pub mod tty_uid;
pub mod tty_username;
pub mod uid;
pub mod username;
} // mod datasource
pub mod datasourceregistry;
pub mod entrypoint {
pub mod cli;
pub mod execve_wrapper;
pub mod execve_wrapper_test_configfile_env;
pub mod test_cli;
} // mod entrypoint
pub mod error;
pub mod filter {
pub mod exclude_spawns_of;
pub mod exclude_uid;
pub mod noop;
pub mod only_root;
pub mod only_tty;
pub mod only_uid;
} // mod filter
pub mod filtering;
pub mod filterregistry;
pub mod genericregistry;
pub mod init_deinit;
pub mod inputdatastorage;
pub mod message;
pub mod output {
pub mod devlogoutput;
pub mod devnulloutput;
pub mod devttyoutput;
pub mod fileoutput;
pub mod noopoutput;
pub mod socketoutput;
pub mod stderroutput;
pub mod stdoutoutput;
} // mod output
pub mod outputregistry;
pub mod tsrm;
pub mod util {
pub mod file;
pub mod list;
pub mod parser;
pub mod pwd;
pub mod string;
pub mod syslog;
pub mod systemd;
} // mod util
} // mod src
pub mod tests {
pub mod bin {
pub mod action_run;
pub mod action_run_configfile;
pub mod action_run_datasource;
pub mod action_run_everything;
pub mod action_run_filter;
pub mod action_run_filterchain;
pub mod action_run_messageformat;
pub mod action_run_output;
pub mod action_stress;
pub mod action_stress_threads;
pub mod action_stress_threadsexec;
pub mod action_unit;
pub mod action_unit_action;
pub mod action_unit_action_log_message_dispatch;
pub mod action_unit_action_log_syscall_exec;
pub mod action_unit_error;
pub mod action_unit_ext_ini;
pub mod action_unit_filterregistry;
pub mod action_unit_outputregistry;
pub mod action_unit_util;
pub mod action_unit_util_syslog;
pub mod action_unit_util_systemd;
pub mod snoopy_test;
pub mod spaceparent;
} // mod bin
} // mod tests
