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
#![feature(rustc_private)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod buffer;
pub mod charsetConv;
pub mod codepages;
pub mod config;
pub mod copyfile;
pub mod device;
pub mod devices;
pub mod dirCache;
pub mod directory;
pub mod direntry;
pub mod dos2unix;
pub mod expand;
pub mod fat;
pub mod fat_free;
pub mod file;
pub mod file_name;
pub mod floppyd;
pub mod floppyd_installtest;
pub mod floppyd_io;
pub mod force_io;
pub mod hash;
pub mod init;
pub mod lba;
pub mod llong;
pub mod lockdev;
pub mod mainloop;
pub mod mattrib;
pub mod mbadblocks;
pub mod mcat;
pub mod mcd;
pub mod mcopy;
pub mod mdel;
pub mod mdir;
pub mod mdoctorfat;
pub mod mdu;
pub mod mformat;
pub mod minfo;
pub mod misc;
pub mod missFuncs;
pub mod mk_direntry;
pub mod mkmanifest;
pub mod mlabel;
pub mod mmd;
pub mod mmount;
pub mod mmove;
pub mod mpartition;
pub mod mshortname;
pub mod mshowfat;
pub mod mtools;
pub mod mzip;
pub mod offset;
pub mod old_dos;
pub mod open_image;
pub mod partition;
pub mod patchlevel;
pub mod plain_io;
pub mod precmd;
pub mod privileges;
pub mod r#match;
pub mod remap;
pub mod scsi;
pub mod scsi_io;
pub mod signal;
pub mod stream;
pub mod streamcache;
pub mod strtonum;
pub mod swap;
pub mod tty;
pub mod unix2dos;
pub mod unixdir;
pub mod vfat;
pub mod xdf_io;
