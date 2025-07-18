use ::libc;
pub static mut version_string: *const libc::c_char = b"1.21.4\0" as *const u8
    as *const libc::c_char;
pub static mut compilation_string: *const libc::c_char = b"gcc -DHAVE_CONFIG_H -DSYSTEM_WGETRC=\"/usr/local/etc/wgetrc\" -DLOCALEDIR=\"/usr/local/share/locale\" -I. -I../lib -I../lib -DNDEBUG -g -O2\0"
    as *const u8 as *const libc::c_char;
pub static mut link_string: *const libc::c_char = b"gcc -DNDEBUG -g -O2 -lpcre -luuid -lidn2 -lnettle -lz ../lib/libgnu.a -lunistring \0"
    as *const u8 as *const libc::c_char;
