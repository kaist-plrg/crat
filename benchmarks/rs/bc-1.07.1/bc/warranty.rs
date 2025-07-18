use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub unsafe extern "C" fn welcome() {
    printf(
        b"This is free software with ABSOLUTELY NO WARRANTY.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"For details type `warranty'. \n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn show_bc_version() {
    printf(
        b"%s %s\n%s\n\0" as *const u8 as *const libc::c_char,
        b"bc\0" as *const u8 as *const libc::c_char,
        b"1.07.1\0" as *const u8 as *const libc::c_char,
        b"Copyright 1991-1994, 1997, 1998, 2000, 2004, 2006, 2008, 2012-2017 Free Software Foundation, Inc.\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn warranty(mut prefix: *const libc::c_char) {
    printf(b"\n%s\0" as *const u8 as *const libc::c_char, prefix);
    show_bc_version();
    printf(
        b"\n    This program is free software; you can redistribute it and/or modify\n    it under the terms of the GNU General Public License as published by\n    the Free Software Foundation; either version 3 of the License , or\n    (at your option) any later version.\n\n    This program is distributed in the hope that it will be useful,\n    but WITHOUT ANY WARRANTY; without even the implied warranty of\n    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n    GNU General Public License for more details.\n\n    You should have received a copy of the GNU General Public License\n    along with this program. If not, write to\n\n       The Free Software Foundation, Inc.\n       51 Franklin Street, Fifth Floor\n       Boston, MA 02110-1335  USA\n\n\0"
            as *const u8 as *const libc::c_char,
    );
}
