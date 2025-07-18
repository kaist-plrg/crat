use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type BOOL = libc::c_int;
pub type WORD = libc::c_uint;
pub type PSTR = *mut libc::c_char;
pub type PINT = *mut libc::c_int;
pub type PVOID = *mut libc::c_void;
pub type PFILE = *mut FILE;
pub type PBUF = PSTR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTBUF {
    pub pfileIn: PFILE,
    pub pbufOrg: PBUF,
    pub lBufSize: libc::c_long,
    pub pbufCur: PBUF,
    pub iLineNum: libc::c_int,
    pub iLNPrev: libc::c_int,
    pub cErrors: libc::c_int,
    pub pInfo: PVOID,
    pub pTempInfo: PVOID,
}
pub type PINPUTBUF = *mut tagINPUTBUF;
pub type PSTRLEX = [libc::c_char; 1023];
pub unsafe extern "C" fn ReportError(
    mut pibIn: PINPUTBUF,
    mut wCode: WORD,
    mut szMsg: PSTR,
    mut szAltMsg: PSTR,
) {
    let mut cNull: libc::c_char = '\0' as i32 as libc::c_char;
    let mut bFatal: BOOL = (wCode & 0x8000 as libc::c_int as libc::c_uint) as BOOL;
    let mut bWarning: BOOL = (wCode & 0x4000 as libc::c_int as libc::c_uint) as BOOL;
    wCode &= !(0x8000 as libc::c_int | 0x4000 as libc::c_int) as libc::c_uint;
    if szMsg.is_null() {
        szMsg = &mut cNull;
    }
    if wCode != 0 {
        if bWarning != 0 {
            printf(b"*** Warning: \0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"*** Error: \0" as *const u8 as *const libc::c_char);
            bFatal
                |= (!pibIn.is_null()
                    && {
                        let fresh0 = (*pibIn).cErrors;
                        (*pibIn).cErrors = (*pibIn).cErrors + 1;
                        fresh0 > 20 as libc::c_int
                    }) as libc::c_int;
        }
    }
    if !pibIn.is_null() {
        if !((*pibIn).pfileIn).is_null() || (*pibIn).iLNPrev != 0 {
            printf(
                b"line %d: \0" as *const u8 as *const libc::c_char,
                (*pibIn).iLineNum,
            );
        } else if wCode != 0x2 as libc::c_int as libc::c_uint {
            let mut szTmp: PSTRLEX = [0; 1023];
            szTmp[(0x3ff as libc::c_int - 1 as libc::c_int)
                as usize] = '\0' as i32 as libc::c_char;
            printf(
                b"'%s'...\n  \0" as *const u8 as *const libc::c_char,
                strncpy(
                    szTmp.as_mut_ptr(),
                    (*pibIn).pbufOrg as *const libc::c_char,
                    (0x3ff as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                ),
            );
        }
    }
    let mut current_block_46: u64;
    match wCode {
        0 => {
            current_block_46 = 9353995356876505083;
        }
        1 => {
            current_block_46 = 13102332982549100014;
        }
        2 => {
            printf(
                b"File not found \"%s\".\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        3 => {
            printf(
                b"Cannot open file \"%s\".\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        17 => {
            printf(
                b"Unexpected character '%c' in input file.\0" as *const u8
                    as *const libc::c_char,
                *szMsg as libc::c_int,
            );
            current_block_46 = 9353995356876505083;
        }
        18 => {
            printf(
                b"Unexpected escape sequence '%s' in input file.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        19 => {
            printf(
                b"Unexpected number %s in input file.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        20 => {
            printf(
                b"Expected '%c' before '%c'.\0" as *const u8 as *const libc::c_char,
                *szMsg.offset(1 as libc::c_int as isize) as libc::c_int,
                *szMsg.offset(0 as libc::c_int as isize) as libc::c_int,
            );
            current_block_46 = 9353995356876505083;
        }
        21 => {
            printf(b"Expected <%s>\0" as *const u8 as *const libc::c_char, szMsg);
            if !szAltMsg.is_null() {
                printf(b" before '%s'\0" as *const u8 as *const libc::c_char, szAltMsg);
            }
            current_block_46 = 9353995356876505083;
        }
        257 => {
            printf(
                b"'%s' used in invalid context.\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        258 => {
            printf(
                b"Duplicate declaration of model variable '%s'.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        270 => {
            printf(
                b"Only one '%s' section is allowed.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        4 => {
            printf(
                b"Out of memory in %s() !\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        259 => {
            printf(b"'%s' redefined.\0" as *const u8 as *const libc::c_char, szMsg);
            current_block_46 = 9353995356876505083;
        }
        260 => {
            printf(
                b"Equation is too long.  Possibly missing terminator.\0" as *const u8
                    as *const libc::c_char,
            );
            current_block_46 = 9353995356876505083;
        }
        261 => {
            printf(
                b"Invalid state identifier '%s'.\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        262 => {
            printf(
                b"Undefined identifier '%s'.\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        264 => {
            printf(
                b"Input '%s' is not initialized.\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        263 => {
            printf(
                b"State variable '%s' has no dynamics.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        269 => {
            printf(
                b"Output variable '%s' is not computed anywhere.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        265 => {
            printf(
                b"Too many %s declarations. Limit is %d.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
                *(szAltMsg as PINT),
            );
            current_block_46 = 9353995356876505083;
        }
        266 => {
            printf(b"Positive number expected.\0" as *const u8 as *const libc::c_char);
            current_block_46 = 9353995356876505083;
        }
        267 => {
            printf(
                b"Name %s exceed %d characters.\0" as *const u8 as *const libc::c_char,
                szMsg,
                80 as libc::c_int,
            );
            current_block_46 = 9353995356876505083;
        }
        268 => {
            printf(
                b"Unbalanced () or equation too long at this line or above.\0"
                    as *const u8 as *const libc::c_char,
            );
            current_block_46 = 9353995356876505083;
        }
        271 => {
            printf(
                b"End keyword is missing in file %s.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_46 = 9353995356876505083;
        }
        _ => {
            printf(
                b"Unknown error code %x: %s\0" as *const u8 as *const libc::c_char,
                wCode,
                szMsg,
            );
            current_block_46 = 13102332982549100014;
        }
    }
    match current_block_46 {
        13102332982549100014 => {
            printf(b"Initialization error.\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !szAltMsg.is_null() && wCode != 0x15 as libc::c_int as libc::c_uint {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, szAltMsg);
    }
    if bFatal != 0 {
        printf(
            b"One or more fatal errors: Exiting...\n\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(wCode as libc::c_int);
    }
}
