use ::libc;
pub static mut tli_rcsid: [libc::c_char; 47] = unsafe {
    *::std::mem::transmute::<
        &[u8; 47],
        &[libc::c_char; 47],
    >(b"$Id: tli.c,v 1.8 2002/03/05 19:10:42 ian Rel $\0")
};
