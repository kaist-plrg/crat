use ::libc;
pub static mut azStatus: [*const libc::c_char; 8] = [
    b"Conversation complete\0" as *const u8 as *const libc::c_char,
    b"Port unavailable\0" as *const u8 as *const libc::c_char,
    b"Dial failed\0" as *const u8 as *const libc::c_char,
    b"Login failed\0" as *const u8 as *const libc::c_char,
    b"Handshake failed\0" as *const u8 as *const libc::c_char,
    b"Call failed\0" as *const u8 as *const libc::c_char,
    b"Talking\0" as *const u8 as *const libc::c_char,
    b"Wrong time to call\0" as *const u8 as *const libc::c_char,
];
