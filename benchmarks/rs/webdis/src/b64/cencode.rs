use ::libc;
pub type base64_encodestep = libc::c_uint;
pub const step_C: base64_encodestep = 2;
pub const step_B: base64_encodestep = 1;
pub const step_A: base64_encodestep = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encodestate {
    pub step: base64_encodestep,
    pub result: libc::c_char,
    pub stepcount: libc::c_int,
}
pub static mut CHARS_PER_LINE: libc::c_int = 4096 as libc::c_int;
pub unsafe extern "C" fn base64_init_encodestate(mut state_in: *mut base64_encodestate) {
    (*state_in).step = step_A;
    (*state_in).result = 0 as libc::c_int as libc::c_char;
    (*state_in).stepcount = 0 as libc::c_int;
}
pub unsafe extern "C" fn base64_encode_value(
    mut value_in: libc::c_char,
) -> libc::c_char {
    static mut encoding: *const libc::c_char = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0"
        as *const u8 as *const libc::c_char;
    if value_in as libc::c_int > 63 as libc::c_int {
        return '=' as i32 as libc::c_char;
    }
    return *encoding.offset(value_in as libc::c_int as isize);
}
pub unsafe extern "C" fn base64_encode_block(
    mut plaintext_in: *const libc::c_char,
    mut length_in: libc::c_int,
    mut code_out: *mut libc::c_char,
    mut state_in: *mut base64_encodestate,
) -> libc::c_int {
    let mut plainchar: *const libc::c_char = plaintext_in;
    let plaintextend: *const libc::c_char = plaintext_in.offset(length_in as isize);
    let mut codechar: *mut libc::c_char = code_out;
    let mut result: libc::c_char = 0;
    let mut fragment: libc::c_char = 0;
    result = (*state_in).result;
    's_139: {
        let mut current_block_33: u64;
        match (*state_in).step as libc::c_uint {
            0 => {
                current_block_33 = 8353443426042145887;
            }
            1 => {
                current_block_33 = 5930550006011605575;
            }
            2 => {
                current_block_33 = 13766659360152472789;
            }
            _ => {
                break 's_139;
            }
        }
        loop {
            match current_block_33 {
                8353443426042145887 => {
                    if plainchar == plaintextend {
                        (*state_in).result = result;
                        (*state_in).step = step_A;
                        return codechar.offset_from(code_out) as libc::c_long
                            as libc::c_int;
                    }
                    let fresh0 = plainchar;
                    plainchar = plainchar.offset(1);
                    fragment = *fresh0;
                    result = ((fragment as libc::c_int & 0xfc as libc::c_int)
                        >> 2 as libc::c_int) as libc::c_char;
                    let fresh1 = codechar;
                    codechar = codechar.offset(1);
                    *fresh1 = base64_encode_value(result);
                    result = ((fragment as libc::c_int & 0x3 as libc::c_int)
                        << 4 as libc::c_int) as libc::c_char;
                    current_block_33 = 5930550006011605575;
                }
                5930550006011605575 => {
                    if plainchar == plaintextend {
                        (*state_in).result = result;
                        (*state_in).step = step_B;
                        return codechar.offset_from(code_out) as libc::c_long
                            as libc::c_int;
                    }
                    let fresh2 = plainchar;
                    plainchar = plainchar.offset(1);
                    fragment = *fresh2;
                    result = (result as libc::c_int
                        | (fragment as libc::c_int & 0xf0 as libc::c_int)
                            >> 4 as libc::c_int) as libc::c_char;
                    let fresh3 = codechar;
                    codechar = codechar.offset(1);
                    *fresh3 = base64_encode_value(result);
                    result = ((fragment as libc::c_int & 0xf as libc::c_int)
                        << 2 as libc::c_int) as libc::c_char;
                    current_block_33 = 13766659360152472789;
                }
                _ => {
                    if plainchar == plaintextend {
                        (*state_in).result = result;
                        (*state_in).step = step_C;
                        return codechar.offset_from(code_out) as libc::c_long
                            as libc::c_int;
                    }
                    let fresh4 = plainchar;
                    plainchar = plainchar.offset(1);
                    fragment = *fresh4;
                    result = (result as libc::c_int
                        | (fragment as libc::c_int & 0xc0 as libc::c_int)
                            >> 6 as libc::c_int) as libc::c_char;
                    let fresh5 = codechar;
                    codechar = codechar.offset(1);
                    *fresh5 = base64_encode_value(result);
                    result = ((fragment as libc::c_int & 0x3f as libc::c_int)
                        >> 0 as libc::c_int) as libc::c_char;
                    let fresh6 = codechar;
                    codechar = codechar.offset(1);
                    *fresh6 = base64_encode_value(result);
                    (*state_in).stepcount += 1;
                    (*state_in).stepcount;
                    if (*state_in).stepcount == CHARS_PER_LINE / 4 as libc::c_int {
                        let fresh7 = codechar;
                        codechar = codechar.offset(1);
                        *fresh7 = '\n' as i32 as libc::c_char;
                        (*state_in).stepcount = 0 as libc::c_int;
                    }
                    current_block_33 = 8353443426042145887;
                }
            }
        }
    }
    return codechar.offset_from(code_out) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn base64_encode_blockend(
    mut code_out: *mut libc::c_char,
    mut state_in: *mut base64_encodestate,
) -> libc::c_int {
    let mut codechar: *mut libc::c_char = code_out;
    match (*state_in).step as libc::c_uint {
        1 => {
            let fresh8 = codechar;
            codechar = codechar.offset(1);
            *fresh8 = base64_encode_value((*state_in).result);
            let fresh9 = codechar;
            codechar = codechar.offset(1);
            *fresh9 = '=' as i32 as libc::c_char;
            let fresh10 = codechar;
            codechar = codechar.offset(1);
            *fresh10 = '=' as i32 as libc::c_char;
        }
        2 => {
            let fresh11 = codechar;
            codechar = codechar.offset(1);
            *fresh11 = base64_encode_value((*state_in).result);
            let fresh12 = codechar;
            codechar = codechar.offset(1);
            *fresh12 = '=' as i32 as libc::c_char;
        }
        0 | _ => {}
    }
    let fresh13 = codechar;
    codechar = codechar.offset(1);
    *fresh13 = '\n' as i32 as libc::c_char;
    return codechar.offset_from(code_out) as libc::c_long as libc::c_int;
}
