use ::libc;
pub type uerr_t = libc::c_uint;
pub const METALINK_SIZE_ERROR: uerr_t = 62;
pub const RETR_WITH_METALINK: uerr_t = 61;
pub const METALINK_MISSING_RESOURCE: uerr_t = 60;
pub const METALINK_SIG_ERROR: uerr_t = 59;
pub const METALINK_CHKSUM_ERROR: uerr_t = 58;
pub const METALINK_RETR_ERROR: uerr_t = 57;
pub const METALINK_PARSE_ERROR: uerr_t = 56;
pub const TIMECONV_ERR: uerr_t = 55;
pub const WARC_TMP_FWRITEERR: uerr_t = 54;
pub const WARC_TMP_FOPENERR: uerr_t = 53;
pub const WARC_ERR: uerr_t = 52;
pub const UNKNOWNATTR: uerr_t = 51;
pub const ATTRMISSING: uerr_t = 50;
pub const CLOSEFAILED: uerr_t = 49;
pub const NEWLOCATION_KEEP_POST: uerr_t = 48;
pub const UNLINKERR: uerr_t = 47;
pub const VERIFCERTERR: uerr_t = 46;
pub const SSLINITFAILED: uerr_t = 45;
pub const WRITEFAILED: uerr_t = 44;
pub const QUOTEXC: uerr_t = 43;
pub const AUTHFAILED: uerr_t = 42;
pub const PROXERR: uerr_t = 41;
pub const RETRBADPATTERN: uerr_t = 40;
pub const RANGEERR: uerr_t = 39;
pub const FILEBADFILE: uerr_t = 38;
pub const TRYLIMEXC: uerr_t = 37;
pub const READERR: uerr_t = 36;
pub const RETRFINISHED: uerr_t = 35;
pub const RETRUNNEEDED: uerr_t = 34;
pub const CONTNOTSUPPORTED: uerr_t = 33;
pub const FTPNOAUTH: uerr_t = 32;
pub const FTPNOPROT: uerr_t = 31;
pub const FTPNOPBSZ: uerr_t = 30;
pub const FTPNOPASV: uerr_t = 29;
pub const FTPINVPASV: uerr_t = 28;
pub const WRONGCODE: uerr_t = 27;
pub const RECLEVELEXC: uerr_t = 26;
pub const RETROK: uerr_t = 25;
pub const HERR: uerr_t = 24;
pub const GATEWAYTIMEOUT: uerr_t = 23;
pub const HEOF: uerr_t = 22;
pub const FWRITEERR: uerr_t = 21;
pub const FOPEN_EXCL_ERR: uerr_t = 20;
pub const FOPENERR: uerr_t = 19;
pub const URLERROR: uerr_t = 18;
pub const FTPRESTFAIL: uerr_t = 17;
pub const FTPRETRINT: uerr_t = 16;
pub const FTPSRVERR: uerr_t = 15;
pub const FTPRERR: uerr_t = 14;
pub const FTPUNKNOWNTYPE: uerr_t = 13;
pub const FTPNSFOD: uerr_t = 12;
pub const FTPSYSERR: uerr_t = 11;
pub const FTPPORTERR: uerr_t = 10;
pub const FTPLOGREFUSED: uerr_t = 9;
pub const FTPLOGINC: uerr_t = 8;
pub const FTPOK: uerr_t = 7;
pub const NEWLOCATION: uerr_t = 6;
pub const CONIMPOSSIBLE: uerr_t = 5;
pub const CONSSLERR: uerr_t = 4;
pub const CONERROR: uerr_t = 3;
pub const CONSOCKERR: uerr_t = 2;
pub const HOSTERR: uerr_t = 1;
pub const NOCONERROR: uerr_t = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const WGET_EXIT_UNKNOWN: C2RustUnnamed = 9;
pub const WGET_EXIT_SERVER_ERROR: C2RustUnnamed = 8;
pub const WGET_EXIT_PROTOCOL_ERROR: C2RustUnnamed = 7;
pub const WGET_EXIT_SERVER_AUTH_FAIL: C2RustUnnamed = 6;
pub const WGET_EXIT_SSL_AUTH_FAIL: C2RustUnnamed = 5;
pub const WGET_EXIT_NETWORK_FAIL: C2RustUnnamed = 4;
pub const WGET_EXIT_IO_FAIL: C2RustUnnamed = 3;
pub const WGET_EXIT_PARSE_ERROR: C2RustUnnamed = 2;
pub const WGET_EXIT_GENERIC_ERROR: C2RustUnnamed = 1;
pub const WGET_EXIT_SUCCESS: C2RustUnnamed = 0;
static mut final_exit_status: libc::c_int = WGET_EXIT_SUCCESS as libc::c_int;
unsafe extern "C" fn get_status_for_err(mut err: uerr_t) -> libc::c_int {
    match err as libc::c_uint {
        25 => return WGET_EXIT_SUCCESS as libc::c_int,
        19 | 20 | 21 | 44 | 47 | 49 | 38 => return WGET_EXIT_IO_FAIL as libc::c_int,
        0 | 1 | 2 | 3 | 4 | 5 | 14 | 28 | 36 | 37 => {
            return WGET_EXIT_NETWORK_FAIL as libc::c_int;
        }
        46 => return WGET_EXIT_SSL_AUTH_FAIL as libc::c_int,
        8 | 9 | 42 => return WGET_EXIT_SERVER_AUTH_FAIL as libc::c_int,
        22 | 24 | 50 => return WGET_EXIT_PROTOCOL_ERROR as libc::c_int,
        27 | 10 | 11 | 12 | 13 | 15 | 16 | 17 | 29 | 33 | 39 | 40 | 41 | 23 => {
            return WGET_EXIT_SERVER_ERROR as libc::c_int;
        }
        18 | 43 | 45 | 51 | _ => return WGET_EXIT_UNKNOWN as libc::c_int,
    };
}
pub unsafe extern "C" fn inform_exit_status(mut err: uerr_t) {
    let mut new_status: libc::c_int = get_status_for_err(err);
    if new_status != WGET_EXIT_SUCCESS as libc::c_int
        && (final_exit_status == WGET_EXIT_SUCCESS as libc::c_int
            || new_status < final_exit_status)
    {
        final_exit_status = new_status;
    }
}
pub unsafe extern "C" fn get_exit_status() -> libc::c_int {
    return if final_exit_status == WGET_EXIT_UNKNOWN as libc::c_int {
        1 as libc::c_int
    } else {
        final_exit_status
    };
}
