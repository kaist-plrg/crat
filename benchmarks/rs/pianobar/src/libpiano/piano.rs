use ::libc;
extern "C" {
    pub type gcry_cipher_handle;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn gcry_cipher_open(
        handle: *mut gcry_cipher_hd_t,
        algo: libc::c_int,
        mode: libc::c_int,
        flags: libc::c_uint,
    ) -> gcry_error_t;
    fn gcry_cipher_close(h: gcry_cipher_hd_t);
    fn gcry_cipher_setkey(
        hd: gcry_cipher_hd_t,
        key: *const libc::c_void,
        keylen: size_t,
    ) -> gcry_error_t;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const GPG_ERR_CODE_DIM: C2RustUnnamed = 65536;
pub const GPG_ERR_EXFULL: C2RustUnnamed = 32908;
pub const GPG_ERR_EXDEV: C2RustUnnamed = 32907;
pub const GPG_ERR_EWOULDBLOCK: C2RustUnnamed = 32906;
pub const GPG_ERR_EUSERS: C2RustUnnamed = 32905;
pub const GPG_ERR_EUNATCH: C2RustUnnamed = 32904;
pub const GPG_ERR_EUCLEAN: C2RustUnnamed = 32903;
pub const GPG_ERR_ETXTBSY: C2RustUnnamed = 32902;
pub const GPG_ERR_ETOOMANYREFS: C2RustUnnamed = 32901;
pub const GPG_ERR_ETIMEDOUT: C2RustUnnamed = 32900;
pub const GPG_ERR_ETIME: C2RustUnnamed = 32899;
pub const GPG_ERR_ESTRPIPE: C2RustUnnamed = 32898;
pub const GPG_ERR_ESTALE: C2RustUnnamed = 32897;
pub const GPG_ERR_ESRMNT: C2RustUnnamed = 32896;
pub const GPG_ERR_ESRCH: C2RustUnnamed = 32895;
pub const GPG_ERR_ESPIPE: C2RustUnnamed = 32894;
pub const GPG_ERR_ESOCKTNOSUPPORT: C2RustUnnamed = 32893;
pub const GPG_ERR_ESHUTDOWN: C2RustUnnamed = 32892;
pub const GPG_ERR_ERPCMISMATCH: C2RustUnnamed = 32891;
pub const GPG_ERR_EROFS: C2RustUnnamed = 32890;
pub const GPG_ERR_ERESTART: C2RustUnnamed = 32889;
pub const GPG_ERR_EREMOTEIO: C2RustUnnamed = 32888;
pub const GPG_ERR_EREMOTE: C2RustUnnamed = 32887;
pub const GPG_ERR_EREMCHG: C2RustUnnamed = 32886;
pub const GPG_ERR_ERANGE: C2RustUnnamed = 32885;
pub const GPG_ERR_EPROTOTYPE: C2RustUnnamed = 32884;
pub const GPG_ERR_EPROTONOSUPPORT: C2RustUnnamed = 32883;
pub const GPG_ERR_EPROTO: C2RustUnnamed = 32882;
pub const GPG_ERR_EPROGUNAVAIL: C2RustUnnamed = 32881;
pub const GPG_ERR_EPROGMISMATCH: C2RustUnnamed = 32880;
pub const GPG_ERR_EPROCUNAVAIL: C2RustUnnamed = 32879;
pub const GPG_ERR_EPROCLIM: C2RustUnnamed = 32878;
pub const GPG_ERR_EPIPE: C2RustUnnamed = 32877;
pub const GPG_ERR_EPFNOSUPPORT: C2RustUnnamed = 32876;
pub const GPG_ERR_EPERM: C2RustUnnamed = 32875;
pub const GPG_ERR_EOVERFLOW: C2RustUnnamed = 32874;
pub const GPG_ERR_EOPNOTSUPP: C2RustUnnamed = 32873;
pub const GPG_ERR_ENXIO: C2RustUnnamed = 32872;
pub const GPG_ERR_ENOTUNIQ: C2RustUnnamed = 32871;
pub const GPG_ERR_ENOTTY: C2RustUnnamed = 32870;
pub const GPG_ERR_ENOTSUP: C2RustUnnamed = 32869;
pub const GPG_ERR_ENOTSOCK: C2RustUnnamed = 32868;
pub const GPG_ERR_ENOTNAM: C2RustUnnamed = 32867;
pub const GPG_ERR_ENOTEMPTY: C2RustUnnamed = 32866;
pub const GPG_ERR_ENOTDIR: C2RustUnnamed = 32865;
pub const GPG_ERR_ENOTCONN: C2RustUnnamed = 32864;
pub const GPG_ERR_ENOTBLK: C2RustUnnamed = 32863;
pub const GPG_ERR_ENOSYS: C2RustUnnamed = 32862;
pub const GPG_ERR_ENOSTR: C2RustUnnamed = 32861;
pub const GPG_ERR_ENOSR: C2RustUnnamed = 32860;
pub const GPG_ERR_ENOSPC: C2RustUnnamed = 32859;
pub const GPG_ERR_ENOPROTOOPT: C2RustUnnamed = 32858;
pub const GPG_ERR_ENOPKG: C2RustUnnamed = 32857;
pub const GPG_ERR_ENONET: C2RustUnnamed = 32856;
pub const GPG_ERR_ENOMSG: C2RustUnnamed = 32855;
pub const GPG_ERR_ENOMEM: C2RustUnnamed = 32854;
pub const GPG_ERR_ENOMEDIUM: C2RustUnnamed = 32853;
pub const GPG_ERR_ENOLINK: C2RustUnnamed = 32852;
pub const GPG_ERR_ENOLCK: C2RustUnnamed = 32851;
pub const GPG_ERR_ENOEXEC: C2RustUnnamed = 32850;
pub const GPG_ERR_ENOENT: C2RustUnnamed = 32849;
pub const GPG_ERR_ENODEV: C2RustUnnamed = 32848;
pub const GPG_ERR_ENODATA: C2RustUnnamed = 32847;
pub const GPG_ERR_ENOCSI: C2RustUnnamed = 32846;
pub const GPG_ERR_ENOBUFS: C2RustUnnamed = 32845;
pub const GPG_ERR_ENOANO: C2RustUnnamed = 32844;
pub const GPG_ERR_ENFILE: C2RustUnnamed = 32843;
pub const GPG_ERR_ENETUNREACH: C2RustUnnamed = 32842;
pub const GPG_ERR_ENETRESET: C2RustUnnamed = 32841;
pub const GPG_ERR_ENETDOWN: C2RustUnnamed = 32840;
pub const GPG_ERR_ENEEDAUTH: C2RustUnnamed = 32839;
pub const GPG_ERR_ENAVAIL: C2RustUnnamed = 32838;
pub const GPG_ERR_ENAMETOOLONG: C2RustUnnamed = 32837;
pub const GPG_ERR_EMULTIHOP: C2RustUnnamed = 32836;
pub const GPG_ERR_EMSGSIZE: C2RustUnnamed = 32835;
pub const GPG_ERR_EMLINK: C2RustUnnamed = 32834;
pub const GPG_ERR_EMFILE: C2RustUnnamed = 32833;
pub const GPG_ERR_EMEDIUMTYPE: C2RustUnnamed = 32832;
pub const GPG_ERR_ELOOP: C2RustUnnamed = 32831;
pub const GPG_ERR_ELNRNG: C2RustUnnamed = 32830;
pub const GPG_ERR_ELIBSCN: C2RustUnnamed = 32829;
pub const GPG_ERR_ELIBMAX: C2RustUnnamed = 32828;
pub const GPG_ERR_ELIBEXEC: C2RustUnnamed = 32827;
pub const GPG_ERR_ELIBBAD: C2RustUnnamed = 32826;
pub const GPG_ERR_ELIBACC: C2RustUnnamed = 32825;
pub const GPG_ERR_EL3RST: C2RustUnnamed = 32824;
pub const GPG_ERR_EL3HLT: C2RustUnnamed = 32823;
pub const GPG_ERR_EL2NSYNC: C2RustUnnamed = 32822;
pub const GPG_ERR_EL2HLT: C2RustUnnamed = 32821;
pub const GPG_ERR_EISNAM: C2RustUnnamed = 32820;
pub const GPG_ERR_EISDIR: C2RustUnnamed = 32819;
pub const GPG_ERR_EISCONN: C2RustUnnamed = 32818;
pub const GPG_ERR_EIO: C2RustUnnamed = 32817;
pub const GPG_ERR_EINVAL: C2RustUnnamed = 32816;
pub const GPG_ERR_EINTR: C2RustUnnamed = 32815;
pub const GPG_ERR_EINPROGRESS: C2RustUnnamed = 32814;
pub const GPG_ERR_EILSEQ: C2RustUnnamed = 32813;
pub const GPG_ERR_EIEIO: C2RustUnnamed = 32812;
pub const GPG_ERR_EIDRM: C2RustUnnamed = 32811;
pub const GPG_ERR_EHOSTUNREACH: C2RustUnnamed = 32810;
pub const GPG_ERR_EHOSTDOWN: C2RustUnnamed = 32809;
pub const GPG_ERR_EGREGIOUS: C2RustUnnamed = 32808;
pub const GPG_ERR_EGRATUITOUS: C2RustUnnamed = 32807;
pub const GPG_ERR_EFTYPE: C2RustUnnamed = 32806;
pub const GPG_ERR_EFBIG: C2RustUnnamed = 32805;
pub const GPG_ERR_EFAULT: C2RustUnnamed = 32804;
pub const GPG_ERR_EEXIST: C2RustUnnamed = 32803;
pub const GPG_ERR_EDQUOT: C2RustUnnamed = 32802;
pub const GPG_ERR_EDOTDOT: C2RustUnnamed = 32801;
pub const GPG_ERR_EDOM: C2RustUnnamed = 32800;
pub const GPG_ERR_EDIED: C2RustUnnamed = 32799;
pub const GPG_ERR_EDESTADDRREQ: C2RustUnnamed = 32798;
pub const GPG_ERR_EDEADLOCK: C2RustUnnamed = 32797;
pub const GPG_ERR_EDEADLK: C2RustUnnamed = 32796;
pub const GPG_ERR_ED: C2RustUnnamed = 32795;
pub const GPG_ERR_ECONNRESET: C2RustUnnamed = 32794;
pub const GPG_ERR_ECONNREFUSED: C2RustUnnamed = 32793;
pub const GPG_ERR_ECONNABORTED: C2RustUnnamed = 32792;
pub const GPG_ERR_ECOMM: C2RustUnnamed = 32791;
pub const GPG_ERR_ECHRNG: C2RustUnnamed = 32790;
pub const GPG_ERR_ECHILD: C2RustUnnamed = 32789;
pub const GPG_ERR_ECANCELED: C2RustUnnamed = 32788;
pub const GPG_ERR_EBUSY: C2RustUnnamed = 32787;
pub const GPG_ERR_EBFONT: C2RustUnnamed = 32786;
pub const GPG_ERR_EBADSLT: C2RustUnnamed = 32785;
pub const GPG_ERR_EBADRQC: C2RustUnnamed = 32784;
pub const GPG_ERR_EBADRPC: C2RustUnnamed = 32783;
pub const GPG_ERR_EBADR: C2RustUnnamed = 32782;
pub const GPG_ERR_EBADMSG: C2RustUnnamed = 32781;
pub const GPG_ERR_EBADFD: C2RustUnnamed = 32780;
pub const GPG_ERR_EBADF: C2RustUnnamed = 32779;
pub const GPG_ERR_EBADE: C2RustUnnamed = 32778;
pub const GPG_ERR_EBACKGROUND: C2RustUnnamed = 32777;
pub const GPG_ERR_EAUTH: C2RustUnnamed = 32776;
pub const GPG_ERR_EALREADY: C2RustUnnamed = 32775;
pub const GPG_ERR_EAGAIN: C2RustUnnamed = 32774;
pub const GPG_ERR_EAFNOSUPPORT: C2RustUnnamed = 32773;
pub const GPG_ERR_EADV: C2RustUnnamed = 32772;
pub const GPG_ERR_EADDRNOTAVAIL: C2RustUnnamed = 32771;
pub const GPG_ERR_EADDRINUSE: C2RustUnnamed = 32770;
pub const GPG_ERR_EACCES: C2RustUnnamed = 32769;
pub const GPG_ERR_E2BIG: C2RustUnnamed = 32768;
pub const GPG_ERR_EOF: C2RustUnnamed = 16383;
pub const GPG_ERR_UNKNOWN_ERRNO: C2RustUnnamed = 16382;
pub const GPG_ERR_MISSING_ERRNO: C2RustUnnamed = 16381;
pub const GPG_ERR_SQL_DONE: C2RustUnnamed = 1601;
pub const GPG_ERR_SQL_ROW: C2RustUnnamed = 1600;
pub const GPG_ERR_SQL_WARNING: C2RustUnnamed = 1528;
pub const GPG_ERR_SQL_NOTICE: C2RustUnnamed = 1527;
pub const GPG_ERR_SQL_NOTADB: C2RustUnnamed = 1526;
pub const GPG_ERR_SQL_RANGE: C2RustUnnamed = 1525;
pub const GPG_ERR_SQL_FORMAT: C2RustUnnamed = 1524;
pub const GPG_ERR_SQL_AUTH: C2RustUnnamed = 1523;
pub const GPG_ERR_SQL_NOLFS: C2RustUnnamed = 1522;
pub const GPG_ERR_SQL_MISUSE: C2RustUnnamed = 1521;
pub const GPG_ERR_SQL_MISMATCH: C2RustUnnamed = 1520;
pub const GPG_ERR_SQL_CONSTRAINT: C2RustUnnamed = 1519;
pub const GPG_ERR_SQL_TOOBIG: C2RustUnnamed = 1518;
pub const GPG_ERR_SQL_SCHEMA: C2RustUnnamed = 1517;
pub const GPG_ERR_SQL_EMPTY: C2RustUnnamed = 1516;
pub const GPG_ERR_SQL_PROTOCOL: C2RustUnnamed = 1515;
pub const GPG_ERR_SQL_CANTOPEN: C2RustUnnamed = 1514;
pub const GPG_ERR_SQL_FULL: C2RustUnnamed = 1513;
pub const GPG_ERR_SQL_NOTFOUND: C2RustUnnamed = 1512;
pub const GPG_ERR_SQL_CORRUPT: C2RustUnnamed = 1511;
pub const GPG_ERR_SQL_IOERR: C2RustUnnamed = 1510;
pub const GPG_ERR_SQL_INTERRUPT: C2RustUnnamed = 1509;
pub const GPG_ERR_SQL_READONLY: C2RustUnnamed = 1508;
pub const GPG_ERR_SQL_NOMEM: C2RustUnnamed = 1507;
pub const GPG_ERR_SQL_LOCKED: C2RustUnnamed = 1506;
pub const GPG_ERR_SQL_BUSY: C2RustUnnamed = 1505;
pub const GPG_ERR_SQL_ABORT: C2RustUnnamed = 1504;
pub const GPG_ERR_SQL_PERM: C2RustUnnamed = 1503;
pub const GPG_ERR_SQL_INTERNAL: C2RustUnnamed = 1502;
pub const GPG_ERR_SQL_ERROR: C2RustUnnamed = 1501;
pub const GPG_ERR_SQL_OK: C2RustUnnamed = 1500;
pub const GPG_ERR_USER_16: C2RustUnnamed = 1039;
pub const GPG_ERR_USER_15: C2RustUnnamed = 1038;
pub const GPG_ERR_USER_14: C2RustUnnamed = 1037;
pub const GPG_ERR_USER_13: C2RustUnnamed = 1036;
pub const GPG_ERR_USER_12: C2RustUnnamed = 1035;
pub const GPG_ERR_USER_11: C2RustUnnamed = 1034;
pub const GPG_ERR_USER_10: C2RustUnnamed = 1033;
pub const GPG_ERR_USER_9: C2RustUnnamed = 1032;
pub const GPG_ERR_USER_8: C2RustUnnamed = 1031;
pub const GPG_ERR_USER_7: C2RustUnnamed = 1030;
pub const GPG_ERR_USER_6: C2RustUnnamed = 1029;
pub const GPG_ERR_USER_5: C2RustUnnamed = 1028;
pub const GPG_ERR_USER_4: C2RustUnnamed = 1027;
pub const GPG_ERR_USER_3: C2RustUnnamed = 1026;
pub const GPG_ERR_USER_2: C2RustUnnamed = 1025;
pub const GPG_ERR_USER_1: C2RustUnnamed = 1024;
pub const GPG_ERR_LDAP_PROX_AUTH_DENIED: C2RustUnnamed = 891;
pub const GPG_ERR_LDAP_ASSERTION_FAILED: C2RustUnnamed = 890;
pub const GPG_ERR_LDAP_CANNOT_CANCEL: C2RustUnnamed = 889;
pub const GPG_ERR_LDAP_TOO_LATE: C2RustUnnamed = 888;
pub const GPG_ERR_LDAP_NO_SUCH_OPERATION: C2RustUnnamed = 887;
pub const GPG_ERR_LDAP_CANCELLED: C2RustUnnamed = 886;
pub const GPG_ERR_LDAP_CUP_RELOAD: C2RustUnnamed = 885;
pub const GPG_ERR_LDAP_CUP_UNSUP_SCHEME: C2RustUnnamed = 884;
pub const GPG_ERR_LDAP_CUP_INV_DATA: C2RustUnnamed = 883;
pub const GPG_ERR_LDAP_CUP_SEC_VIOLATION: C2RustUnnamed = 882;
pub const GPG_ERR_LDAP_CUP_RESOURCE_LIMIT: C2RustUnnamed = 881;
pub const GPG_ERR_LDAP_OTHER: C2RustUnnamed = 848;
pub const GPG_ERR_LDAP_VLV: C2RustUnnamed = 844;
pub const GPG_ERR_LDAP_AFFECTS_MULT_DSAS: C2RustUnnamed = 839;
pub const GPG_ERR_LDAP_RESULTS_TOO_LARGE: C2RustUnnamed = 838;
pub const GPG_ERR_LDAP_NO_OBJ_CLASS_MODS: C2RustUnnamed = 837;
pub const GPG_ERR_LDAP_ALREADY_EXISTS: C2RustUnnamed = 836;
pub const GPG_ERR_LDAP_NOT_ALLOW_ON_RDN: C2RustUnnamed = 835;
pub const GPG_ERR_LDAP_NOT_ALLOW_NONLEAF: C2RustUnnamed = 834;
pub const GPG_ERR_LDAP_OBJ_CLS_VIOLATION: C2RustUnnamed = 833;
pub const GPG_ERR_LDAP_NAMING_VIOLATION: C2RustUnnamed = 832;
pub const GPG_ERR_LDAP_LOOP_DETECT: C2RustUnnamed = 822;
pub const GPG_ERR_LDAP_UNWILL_TO_PERFORM: C2RustUnnamed = 821;
pub const GPG_ERR_LDAP_UNAVAILABLE: C2RustUnnamed = 820;
pub const GPG_ERR_LDAP_BUSY: C2RustUnnamed = 819;
pub const GPG_ERR_LDAP_INSUFFICIENT_ACC: C2RustUnnamed = 818;
pub const GPG_ERR_LDAP_INV_CREDENTIALS: C2RustUnnamed = 817;
pub const GPG_ERR_LDAP_BAD_AUTH: C2RustUnnamed = 816;
pub const GPG_ERR_LDAP_X_PROXY_AUTH_FAIL: C2RustUnnamed = 815;
pub const GPG_ERR_LDAP_ALIAS_DEREF: C2RustUnnamed = 804;
pub const GPG_ERR_LDAP_IS_LEAF: C2RustUnnamed = 803;
pub const GPG_ERR_LDAP_INV_DN_SYNTAX: C2RustUnnamed = 802;
pub const GPG_ERR_LDAP_ALIAS_PROBLEM: C2RustUnnamed = 801;
pub const GPG_ERR_LDAP_NO_SUCH_OBJ: C2RustUnnamed = 800;
pub const GPG_ERR_LDAP_INV_SYNTAX: C2RustUnnamed = 789;
pub const GPG_ERR_LDAP_TYPE_VALUE_EXISTS: C2RustUnnamed = 788;
pub const GPG_ERR_LDAP_CONST_VIOLATION: C2RustUnnamed = 787;
pub const GPG_ERR_LDAP_BAD_MATCHING: C2RustUnnamed = 786;
pub const GPG_ERR_LDAP_UNDEFINED_TYPE: C2RustUnnamed = 785;
pub const GPG_ERR_LDAP_NO_SUCH_ATTRIBUTE: C2RustUnnamed = 784;
pub const GPG_ERR_LDAP_SASL_BIND_INPROG: C2RustUnnamed = 782;
pub const GPG_ERR_LDAP_CONFIDENT_RQRD: C2RustUnnamed = 781;
pub const GPG_ERR_LDAP_UNAVAIL_CRIT_EXTN: C2RustUnnamed = 780;
pub const GPG_ERR_LDAP_ADMINLIMIT: C2RustUnnamed = 779;
pub const GPG_ERR_LDAP_REFERRAL: C2RustUnnamed = 778;
pub const GPG_ERR_LDAP_PARTIAL_RESULTS: C2RustUnnamed = 777;
pub const GPG_ERR_LDAP_STRONG_AUTH_RQRD: C2RustUnnamed = 776;
pub const GPG_ERR_LDAP_UNSUPPORTED_AUTH: C2RustUnnamed = 775;
pub const GPG_ERR_LDAP_COMPARE_TRUE: C2RustUnnamed = 774;
pub const GPG_ERR_LDAP_COMPARE_FALSE: C2RustUnnamed = 773;
pub const GPG_ERR_LDAP_SIZELIMIT: C2RustUnnamed = 772;
pub const GPG_ERR_LDAP_TIMELIMIT: C2RustUnnamed = 771;
pub const GPG_ERR_LDAP_PROTOCOL: C2RustUnnamed = 770;
pub const GPG_ERR_LDAP_OPERATIONS: C2RustUnnamed = 769;
pub const GPG_ERR_LDAP_SUCCESS: C2RustUnnamed = 768;
pub const GPG_ERR_LDAP_SERVER_DOWN: C2RustUnnamed = 767;
pub const GPG_ERR_LDAP_LOCAL: C2RustUnnamed = 766;
pub const GPG_ERR_LDAP_ENCODING: C2RustUnnamed = 765;
pub const GPG_ERR_LDAP_DECODING: C2RustUnnamed = 764;
pub const GPG_ERR_LDAP_TIMEOUT: C2RustUnnamed = 763;
pub const GPG_ERR_LDAP_AUTH_UNKNOWN: C2RustUnnamed = 762;
pub const GPG_ERR_LDAP_FILTER: C2RustUnnamed = 761;
pub const GPG_ERR_LDAP_USER_CANCELLED: C2RustUnnamed = 760;
pub const GPG_ERR_LDAP_PARAM: C2RustUnnamed = 759;
pub const GPG_ERR_LDAP_NO_MEMORY: C2RustUnnamed = 758;
pub const GPG_ERR_LDAP_CONNECT: C2RustUnnamed = 757;
pub const GPG_ERR_LDAP_NOT_SUPPORTED: C2RustUnnamed = 756;
pub const GPG_ERR_LDAP_CONTROL_NOT_FOUND: C2RustUnnamed = 755;
pub const GPG_ERR_LDAP_NO_RESULTS: C2RustUnnamed = 754;
pub const GPG_ERR_LDAP_CLIENT_LOOP: C2RustUnnamed = 752;
pub const GPG_ERR_LDAP_REFERRAL_LIMIT: C2RustUnnamed = 751;
pub const GPG_ERR_LDAP_X_CONNECTING: C2RustUnnamed = 750;
pub const GPG_ERR_LDAP_OTHER_GENERAL: C2RustUnnamed = 729;
pub const GPG_ERR_LDAP_X_GENERAL: C2RustUnnamed = 728;
pub const GPG_ERR_LDAP_E_GENERAL: C2RustUnnamed = 727;
pub const GPG_ERR_LDAP_UPDATE_GENERAL: C2RustUnnamed = 726;
pub const GPG_ERR_LDAP_SERVICE_GENERAL: C2RustUnnamed = 725;
pub const GPG_ERR_LDAP_SECURITY_GENERAL: C2RustUnnamed = 724;
pub const GPG_ERR_LDAP_NAME_GENERAL: C2RustUnnamed = 723;
pub const GPG_ERR_LDAP_ATTR_GENERAL: C2RustUnnamed = 722;
pub const GPG_ERR_LDAP_GENERAL: C2RustUnnamed = 721;
pub const GPG_ERR_DNS_TIMEOUT: C2RustUnnamed = 718;
pub const GPG_ERR_DNS_VERIFY: C2RustUnnamed = 717;
pub const GPG_ERR_DNS_CLOSED: C2RustUnnamed = 716;
pub const GPG_ERR_DNS_NO_ANSWER: C2RustUnnamed = 715;
pub const GPG_ERR_DNS_NO_QUERY: C2RustUnnamed = 714;
pub const GPG_ERR_DNS_ADDRESS: C2RustUnnamed = 713;
pub const GPG_ERR_DNS_SECTION: C2RustUnnamed = 712;
pub const GPG_ERR_DNS_UNKNOWN: C2RustUnnamed = 711;
pub const GPG_ERR_SYSTEM_BUG: C2RustUnnamed = 666;
pub const GPG_ERR_SERVICE: C2RustUnnamed = 319;
pub const GPG_ERR_NO_SERVICE: C2RustUnnamed = 318;
pub const GPG_ERR_KEYBOXD: C2RustUnnamed = 317;
pub const GPG_ERR_NO_KEYBOXD: C2RustUnnamed = 316;
pub const GPG_ERR_BAD_AUTH: C2RustUnnamed = 315;
pub const GPG_ERR_NO_AUTH: C2RustUnnamed = 314;
pub const GPG_ERR_WRONG_NAME: C2RustUnnamed = 313;
pub const GPG_ERR_TRY_LATER: C2RustUnnamed = 312;
pub const GPG_ERR_ALREADY_FETCHED: C2RustUnnamed = 311;
pub const GPG_ERR_INV_ORDER: C2RustUnnamed = 310;
pub const GPG_ERR_UNKNOWN_FLAG: C2RustUnnamed = 309;
pub const GPG_ERR_TOO_OLD: C2RustUnnamed = 308;
pub const GPG_ERR_TOO_YOUNG: C2RustUnnamed = 307;
pub const GPG_ERR_DUP_NAME: C2RustUnnamed = 306;
pub const GPG_ERR_NAME_EXISTS: C2RustUnnamed = 305;
pub const GPG_ERR_USER_ID_EXISTS: C2RustUnnamed = 304;
pub const GPG_ERR_MISSING_ENVVAR: C2RustUnnamed = 303;
pub const GPG_ERR_WINDOW_TOO_LARGE: C2RustUnnamed = 302;
pub const GPG_ERR_WINDOW_TOO_SMALL: C2RustUnnamed = 301;
pub const GPG_ERR_ENGINE_TOO_OLD: C2RustUnnamed = 300;
pub const GPG_ERR_ASS_UNKNOWN_INQUIRE: C2RustUnnamed = 281;
pub const GPG_ERR_ASS_PARAMETER: C2RustUnnamed = 280;
pub const GPG_ERR_ASS_NO_OUTPUT: C2RustUnnamed = 279;
pub const GPG_ERR_ASS_NO_INPUT: C2RustUnnamed = 278;
pub const GPG_ERR_ASS_CANCELED: C2RustUnnamed = 277;
pub const GPG_ERR_ASS_SYNTAX: C2RustUnnamed = 276;
pub const GPG_ERR_ASS_UNKNOWN_CMD: C2RustUnnamed = 275;
pub const GPG_ERR_ASS_UNEXPECTED_CMD: C2RustUnnamed = 274;
pub const GPG_ERR_ASS_TOO_MUCH_DATA: C2RustUnnamed = 273;
pub const GPG_ERR_ASS_WRITE_ERROR: C2RustUnnamed = 271;
pub const GPG_ERR_ASS_READ_ERROR: C2RustUnnamed = 270;
pub const GPG_ERR_ASS_SERVER_START: C2RustUnnamed = 269;
pub const GPG_ERR_ASS_NOT_A_CLIENT: C2RustUnnamed = 268;
pub const GPG_ERR_ASS_NOT_A_SERVER: C2RustUnnamed = 267;
pub const GPG_ERR_ASS_NO_INQUIRE_CB: C2RustUnnamed = 266;
pub const GPG_ERR_ASS_NO_DATA_CB: C2RustUnnamed = 265;
pub const GPG_ERR_ASS_NESTED_COMMANDS: C2RustUnnamed = 264;
pub const GPG_ERR_ASS_LINE_TOO_LONG: C2RustUnnamed = 263;
pub const GPG_ERR_ASS_INCOMPLETE_LINE: C2RustUnnamed = 262;
pub const GPG_ERR_ASS_INV_VALUE: C2RustUnnamed = 261;
pub const GPG_ERR_ASS_INV_RESPONSE: C2RustUnnamed = 260;
pub const GPG_ERR_ASS_CONNECT_FAILED: C2RustUnnamed = 259;
pub const GPG_ERR_ASS_ACCEPT_FAILED: C2RustUnnamed = 258;
pub const GPG_ERR_ASS_GENERAL: C2RustUnnamed = 257;
pub const GPG_ERR_FALSE: C2RustUnnamed = 256;
pub const GPG_ERR_TRUE: C2RustUnnamed = 255;
pub const GPG_ERR_INV_LOCK_OBJ: C2RustUnnamed = 254;
pub const GPG_ERR_KEY_ON_CARD: C2RustUnnamed = 253;
pub const GPG_ERR_KEY_DISABLED: C2RustUnnamed = 252;
pub const GPG_ERR_FORBIDDEN: C2RustUnnamed = 251;
pub const GPG_ERR_BOGUS_STRING: C2RustUnnamed = 250;
pub const GPG_ERR_BAD_HS_CLIENT_KEX: C2RustUnnamed = 249;
pub const GPG_ERR_BAD_HS_SERVER_KEX: C2RustUnnamed = 248;
pub const GPG_ERR_BAD_HS_FINISHED: C2RustUnnamed = 247;
pub const GPG_ERR_BAD_HS_SERVER_HELLO_DONE: C2RustUnnamed = 246;
pub const GPG_ERR_BAD_HS_SERVER_HELLO: C2RustUnnamed = 245;
pub const GPG_ERR_BAD_HS_CLIENT_HELLO: C2RustUnnamed = 244;
pub const GPG_ERR_BAD_HS_CHANGE_CIPHER: C2RustUnnamed = 243;
pub const GPG_ERR_BAD_HS_CERT_VER: C2RustUnnamed = 242;
pub const GPG_ERR_BAD_HS_CERT_REQ: C2RustUnnamed = 241;
pub const GPG_ERR_BAD_HS_CERT: C2RustUnnamed = 240;
pub const GPG_ERR_UNKNOWN_IDENTITY: C2RustUnnamed = 239;
pub const GPG_ERR_BAD_TICKET: C2RustUnnamed = 238;
pub const GPG_ERR_TICKET_EXPIRED: C2RustUnnamed = 237;
pub const GPG_ERR_CLOSE_NOTIFY: C2RustUnnamed = 236;
pub const GPG_ERR_MISSING_CLIENT_CERT: C2RustUnnamed = 235;
pub const GPG_ERR_NO_CIPHER: C2RustUnnamed = 234;
pub const GPG_ERR_FATAL_ALERT: C2RustUnnamed = 233;
pub const GPG_ERR_WOULD_WRAP: C2RustUnnamed = 232;
pub const GPG_ERR_COMPR_FAILED: C2RustUnnamed = 231;
pub const GPG_ERR_UNEXPECTED_MSG: C2RustUnnamed = 230;
pub const GPG_ERR_BAD_MAC: C2RustUnnamed = 229;
pub const GPG_ERR_INV_RECORD: C2RustUnnamed = 228;
pub const GPG_ERR_CERT_TOO_LARGE: C2RustUnnamed = 227;
pub const GPG_ERR_NO_CERT_CHAIN: C2RustUnnamed = 226;
pub const GPG_ERR_OBJ_TERM_STATE: C2RustUnnamed = 225;
pub const GPG_ERR_REQUEST_TOO_LONG: C2RustUnnamed = 224;
pub const GPG_ERR_REQUEST_TOO_SHORT: C2RustUnnamed = 223;
pub const GPG_ERR_LEGACY_KEY: C2RustUnnamed = 222;
pub const GPG_ERR_NO_KEY: C2RustUnnamed = 221;
pub const GPG_ERR_NO_NAME: C2RustUnnamed = 220;
pub const GPG_ERR_SERVER_FAILED: C2RustUnnamed = 219;
pub const GPG_ERR_DB_CORRUPTED: C2RustUnnamed = 218;
pub const GPG_ERR_SUBKEYS_EXP_OR_REV: C2RustUnnamed = 217;
pub const GPG_ERR_SEXP_BAD_OCT_CHAR: C2RustUnnamed = 213;
pub const GPG_ERR_SEXP_ODD_HEX_NUMBERS: C2RustUnnamed = 212;
pub const GPG_ERR_SEXP_BAD_HEX_CHAR: C2RustUnnamed = 211;
pub const GPG_ERR_SEXP_UNEXPECTED_PUNC: C2RustUnnamed = 210;
pub const GPG_ERR_SEXP_UNMATCHED_DH: C2RustUnnamed = 209;
pub const GPG_ERR_SEXP_NESTED_DH: C2RustUnnamed = 208;
pub const GPG_ERR_SEXP_ZERO_PREFIX: C2RustUnnamed = 207;
pub const GPG_ERR_SEXP_BAD_QUOTATION: C2RustUnnamed = 206;
pub const GPG_ERR_SEXP_BAD_CHARACTER: C2RustUnnamed = 205;
pub const GPG_ERR_SEXP_NOT_CANONICAL: C2RustUnnamed = 204;
pub const GPG_ERR_SEXP_UNMATCHED_PAREN: C2RustUnnamed = 203;
pub const GPG_ERR_SEXP_STRING_TOO_LONG: C2RustUnnamed = 202;
pub const GPG_ERR_SEXP_INV_LEN_SPEC: C2RustUnnamed = 201;
pub const GPG_ERR_BUFFER_TOO_SHORT: C2RustUnnamed = 200;
pub const GPG_ERR_UNFINISHED: C2RustUnnamed = 199;
pub const GPG_ERR_FULLY_CANCELED: C2RustUnnamed = 198;
pub const GPG_ERR_MAC_ALGO: C2RustUnnamed = 197;
pub const GPG_ERR_BROKEN_SECKEY: C2RustUnnamed = 196;
pub const GPG_ERR_BROKEN_PUBKEY: C2RustUnnamed = 195;
pub const GPG_ERR_CRYPT_CTX_CONFLICT: C2RustUnnamed = 194;
pub const GPG_ERR_BAD_CRYPT_CTX: C2RustUnnamed = 193;
pub const GPG_ERR_WRONG_CRYPT_CTX: C2RustUnnamed = 192;
pub const GPG_ERR_NO_CRYPT_CTX: C2RustUnnamed = 191;
pub const GPG_ERR_AMBIGUOUS: C2RustUnnamed = 190;
pub const GPG_ERR_DUP_KEY: C2RustUnnamed = 189;
pub const GPG_ERR_UNKNOWN_CURVE: C2RustUnnamed = 188;
pub const GPG_ERR_INV_CURVE: C2RustUnnamed = 187;
pub const GPG_ERR_NO_KEYSERVER: C2RustUnnamed = 186;
pub const GPG_ERR_MISSING_ISSUER_CERT: C2RustUnnamed = 185;
pub const GPG_ERR_NOT_INITIALIZED: C2RustUnnamed = 184;
pub const GPG_ERR_LIMIT_REACHED: C2RustUnnamed = 183;
pub const GPG_ERR_TOO_MANY: C2RustUnnamed = 182;
pub const GPG_ERR_MISSING_KEY: C2RustUnnamed = 181;
pub const GPG_ERR_NO_ENGINE: C2RustUnnamed = 180;
pub const GPG_ERR_NOT_ENABLED: C2RustUnnamed = 179;
pub const GPG_ERR_NO_PIN: C2RustUnnamed = 178;
pub const GPG_ERR_NO_PASSPHRASE: C2RustUnnamed = 177;
pub const GPG_ERR_NOT_OPERATIONAL: C2RustUnnamed = 176;
pub const GPG_ERR_UNKNOWN_COMMAND: C2RustUnnamed = 175;
pub const GPG_ERR_UNKNOWN_OPTION: C2RustUnnamed = 174;
pub const GPG_ERR_LOCKED: C2RustUnnamed = 173;
pub const GPG_ERR_UNKNOWN_CRIT_EXTN: C2RustUnnamed = 172;
pub const GPG_ERR_UNKNOWN_EXTN: C2RustUnnamed = 171;
pub const GPG_ERR_INV_REQUEST: C2RustUnnamed = 170;
pub const GPG_ERR_INV_MAC: C2RustUnnamed = 169;
pub const GPG_ERR_PROTOCOL_VIOLATION: C2RustUnnamed = 168;
pub const GPG_ERR_NOT_LOCKED: C2RustUnnamed = 167;
pub const GPG_ERR_LOCALE_PROBLEM: C2RustUnnamed = 166;
pub const GPG_ERR_UNKNOWN_NAME: C2RustUnnamed = 165;
pub const GPG_ERR_INV_CERT_OBJ: C2RustUnnamed = 164;
pub const GPG_ERR_UNSUPPORTED_CRL_VERSION: C2RustUnnamed = 163;
pub const GPG_ERR_INV_CRL_OBJ: C2RustUnnamed = 162;
pub const GPG_ERR_INV_TIME: C2RustUnnamed = 161;
pub const GPG_ERR_INV_OID_STRING: C2RustUnnamed = 160;
pub const GPG_ERR_MODULE_NOT_FOUND: C2RustUnnamed = 159;
pub const GPG_ERR_MISSING_ACTION: C2RustUnnamed = 158;
pub const GPG_ERR_DUP_VALUE: C2RustUnnamed = 157;
pub const GPG_ERR_INV_STATE: C2RustUnnamed = 156;
pub const GPG_ERR_ENCODING_PROBLEM: C2RustUnnamed = 155;
pub const GPG_ERR_SIG_EXPIRED: C2RustUnnamed = 154;
pub const GPG_ERR_KEY_EXPIRED: C2RustUnnamed = 153;
pub const GPG_ERR_DECRYPT_FAILED: C2RustUnnamed = 152;
pub const GPG_ERR_PUBKEY_NOT_TRUSTED: C2RustUnnamed = 151;
pub const GPG_ERR_INV_ENGINE: C2RustUnnamed = 150;
pub const GPG_ERR_UNKNOWN_ALGORITHM: C2RustUnnamed = 149;
pub const GPG_ERR_UNSUPPORTED_CMS_VERSION: C2RustUnnamed = 148;
pub const GPG_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed = 147;
pub const GPG_ERR_UNSUPPORTED_CMS_OBJ: C2RustUnnamed = 146;
pub const GPG_ERR_UNKNOWN_CMS_OBJ: C2RustUnnamed = 145;
pub const GPG_ERR_INV_CMS_OBJ: C2RustUnnamed = 144;
pub const GPG_ERR_NO_CMS_OBJ: C2RustUnnamed = 143;
pub const GPG_ERR_NOT_DER_ENCODED: C2RustUnnamed = 142;
pub const GPG_ERR_UNEXPECTED_TAG: C2RustUnnamed = 141;
pub const GPG_ERR_INV_KEYINFO: C2RustUnnamed = 140;
pub const GPG_ERR_INV_LENGTH: C2RustUnnamed = 139;
pub const GPG_ERR_INV_TAG: C2RustUnnamed = 138;
pub const GPG_ERR_IDENTIFIER_NOT_FOUND: C2RustUnnamed = 137;
pub const GPG_ERR_ELEMENT_NOT_FOUND: C2RustUnnamed = 136;
pub const GPG_ERR_INV_BER: C2RustUnnamed = 135;
pub const GPG_ERR_BAD_BER: C2RustUnnamed = 134;
pub const GPG_ERR_INV_CRL: C2RustUnnamed = 133;
pub const GPG_ERR_PIN_NOT_SYNCED: C2RustUnnamed = 132;
pub const GPG_ERR_USE_CONDITIONS: C2RustUnnamed = 131;
pub const GPG_ERR_PIN_BLOCKED: C2RustUnnamed = 130;
pub const GPG_ERR_HARDWARE: C2RustUnnamed = 129;
pub const GPG_ERR_MISSING_VALUE: C2RustUnnamed = 128;
pub const GPG_ERR_WRONG_BLOB_TYPE: C2RustUnnamed = 127;
pub const GPG_ERR_NOTHING_FOUND: C2RustUnnamed = 126;
pub const GPG_ERR_WRONG_KEY_USAGE: C2RustUnnamed = 125;
pub const GPG_ERR_UNSUPPORTED_OPERATION: C2RustUnnamed = 124;
pub const GPG_ERR_CARD_NOT_INITIALIZED: C2RustUnnamed = 123;
pub const GPG_ERR_BAD_PIN_METHOD: C2RustUnnamed = 122;
pub const GPG_ERR_UNSUPPORTED_PROTOCOL: C2RustUnnamed = 121;
pub const GPG_ERR_SCDAEMON: C2RustUnnamed = 120;
pub const GPG_ERR_NO_SCDAEMON: C2RustUnnamed = 119;
pub const GPG_ERR_INV_ID: C2RustUnnamed = 118;
pub const GPG_ERR_INV_INDEX: C2RustUnnamed = 117;
pub const GPG_ERR_NO_POLICY_MATCH: C2RustUnnamed = 116;
pub const GPG_ERR_CONFIGURATION: C2RustUnnamed = 115;
pub const GPG_ERR_NOT_CONFIRMED: C2RustUnnamed = 114;
pub const GPG_ERR_NO_PKCS15_APP: C2RustUnnamed = 113;
pub const GPG_ERR_CARD_NOT_PRESENT: C2RustUnnamed = 112;
pub const GPG_ERR_INV_CARD: C2RustUnnamed = 111;
pub const GPG_ERR_CARD_REMOVED: C2RustUnnamed = 110;
pub const GPG_ERR_CARD_RESET: C2RustUnnamed = 109;
pub const GPG_ERR_CARD: C2RustUnnamed = 108;
pub const GPG_ERR_AMBIGUOUS_NAME: C2RustUnnamed = 107;
pub const GPG_ERR_CORRUPTED_PROTECTION: C2RustUnnamed = 106;
pub const GPG_ERR_UNSUPPORTED_PROTECTION: C2RustUnnamed = 105;
pub const GPG_ERR_UNKNOWN_SEXP: C2RustUnnamed = 104;
pub const GPG_ERR_UNSUPPORTED_CERT: C2RustUnnamed = 103;
pub const GPG_ERR_CERT_TOO_YOUNG: C2RustUnnamed = 102;
pub const GPG_ERR_CERT_EXPIRED: C2RustUnnamed = 101;
pub const GPG_ERR_BAD_CA_CERT: C2RustUnnamed = 100;
pub const GPG_ERR_CANCELED: C2RustUnnamed = 99;
pub const GPG_ERR_NOT_TRUSTED: C2RustUnnamed = 98;
pub const GPG_ERR_LINE_TOO_LONG: C2RustUnnamed = 97;
pub const GPG_ERR_CRL_TOO_OLD: C2RustUnnamed = 96;
pub const GPG_ERR_NO_CRL_KNOWN: C2RustUnnamed = 95;
pub const GPG_ERR_CERT_REVOKED: C2RustUnnamed = 94;
pub const GPG_ERR_DIRMNGR: C2RustUnnamed = 93;
pub const GPG_ERR_NO_DIRMNGR: C2RustUnnamed = 92;
pub const GPG_ERR_WRONG_CARD: C2RustUnnamed = 91;
pub const GPG_ERR_INV_PARAMETER: C2RustUnnamed = 90;
pub const GPG_ERR_BAD_DATA: C2RustUnnamed = 89;
pub const GPG_ERR_INV_NAME: C2RustUnnamed = 88;
pub const GPG_ERR_BAD_PIN: C2RustUnnamed = 87;
pub const GPG_ERR_PIN_ENTRY: C2RustUnnamed = 86;
pub const GPG_ERR_NO_PIN_ENTRY: C2RustUnnamed = 85;
pub const GPG_ERR_UNSUPPORTED_ALGORITHM: C2RustUnnamed = 84;
pub const GPG_ERR_INV_SEXP: C2RustUnnamed = 83;
pub const GPG_ERR_INV_SESSION_KEY: C2RustUnnamed = 82;
pub const GPG_ERR_ASSUAN: C2RustUnnamed = 81;
pub const GPG_ERR_ASSUAN_SERVER_FAULT: C2RustUnnamed = 80;
pub const GPG_ERR_INV_DATA: C2RustUnnamed = 79;
pub const GPG_ERR_AGENT: C2RustUnnamed = 78;
pub const GPG_ERR_NO_AGENT: C2RustUnnamed = 77;
pub const GPG_ERR_INV_RESPONSE: C2RustUnnamed = 76;
pub const GPG_ERR_INCOMPLETE_LINE: C2RustUnnamed = 75;
pub const GPG_ERR_TRUNCATED: C2RustUnnamed = 74;
pub const GPG_ERR_INV_HANDLE: C2RustUnnamed = 73;
pub const GPG_ERR_INV_FLAG: C2RustUnnamed = 72;
pub const GPG_ERR_INV_CIPHER_MODE: C2RustUnnamed = 71;
pub const GPG_ERR_CONFLICT: C2RustUnnamed = 70;
pub const GPG_ERR_NOT_IMPLEMENTED: C2RustUnnamed = 69;
pub const GPG_ERR_NO_OBJ: C2RustUnnamed = 68;
pub const GPG_ERR_TOO_LARGE: C2RustUnnamed = 67;
pub const GPG_ERR_TOO_SHORT: C2RustUnnamed = 66;
pub const GPG_ERR_INV_OBJ: C2RustUnnamed = 65;
pub const GPG_ERR_EOF_GCRYPT: C2RustUnnamed = 64;
pub const GPG_ERR_INTERNAL: C2RustUnnamed = 63;
pub const GPG_ERR_TIMEOUT: C2RustUnnamed = 62;
pub const GPG_ERR_INV_OP: C2RustUnnamed = 61;
pub const GPG_ERR_NOT_SUPPORTED: C2RustUnnamed = 60;
pub const GPG_ERR_BUG: C2RustUnnamed = 59;
pub const GPG_ERR_NO_DATA: C2RustUnnamed = 58;
pub const GPG_ERR_MISSING_CERT: C2RustUnnamed = 57;
pub const GPG_ERR_BAD_CERT_CHAIN: C2RustUnnamed = 56;
pub const GPG_ERR_INV_VALUE: C2RustUnnamed = 55;
pub const GPG_ERR_UNUSABLE_SECKEY: C2RustUnnamed = 54;
pub const GPG_ERR_UNUSABLE_PUBKEY: C2RustUnnamed = 53;
pub const GPG_ERR_NOT_PROCESSED: C2RustUnnamed = 52;
pub const GPG_ERR_NOT_ENCRYPTED: C2RustUnnamed = 51;
pub const GPG_ERR_SELFTEST_FAILED: C2RustUnnamed = 50;
pub const GPG_ERR_UNKNOWN_HOST: C2RustUnnamed = 49;
pub const GPG_ERR_NETWORK: C2RustUnnamed = 48;
pub const GPG_ERR_INV_URI: C2RustUnnamed = 47;
pub const GPG_ERR_BAD_URI: C2RustUnnamed = 46;
pub const GPG_ERR_INV_ARG: C2RustUnnamed = 45;
pub const GPG_ERR_INV_KEYLEN: C2RustUnnamed = 44;
pub const GPG_ERR_WEAK_KEY: C2RustUnnamed = 43;
pub const GPG_ERR_TRIBUTE_TO_D_A: C2RustUnnamed = 42;
pub const GPG_ERR_WRONG_PUBKEY_ALGO: C2RustUnnamed = 41;
pub const GPG_ERR_KEYSERVER: C2RustUnnamed = 40;
pub const GPG_ERR_TIME_CONFLICT: C2RustUnnamed = 39;
pub const GPG_ERR_UNEXPECTED: C2RustUnnamed = 38;
pub const GPG_ERR_INV_USER_ID: C2RustUnnamed = 37;
pub const GPG_ERR_BAD_CERT: C2RustUnnamed = 36;
pub const GPG_ERR_TRUSTDB: C2RustUnnamed = 35;
pub const GPG_ERR_INV_KEYRING: C2RustUnnamed = 34;
pub const GPG_ERR_RESOURCE_LIMIT: C2RustUnnamed = 33;
pub const GPG_ERR_SIG_CLASS: C2RustUnnamed = 32;
pub const GPG_ERR_INV_PASSPHRASE: C2RustUnnamed = 31;
pub const GPG_ERR_BAD_MPI: C2RustUnnamed = 30;
pub const GPG_ERR_SYNTAX: C2RustUnnamed = 29;
pub const GPG_ERR_VALUE_NOT_FOUND: C2RustUnnamed = 28;
pub const GPG_ERR_NOT_FOUND: C2RustUnnamed = 27;
pub const GPG_ERR_NO_VALUE: C2RustUnnamed = 26;
pub const GPG_ERR_INV_ATTR: C2RustUnnamed = 25;
pub const GPG_ERR_NO_SIGNATURE_SCHEME: C2RustUnnamed = 24;
pub const GPG_ERR_NO_ENCRYPTION_SCHEME: C2RustUnnamed = 23;
pub const GPG_ERR_NO_ENCODING_METHOD: C2RustUnnamed = 22;
pub const GPG_ERR_NO_PRIME: C2RustUnnamed = 21;
pub const GPG_ERR_COMPR_ALGO: C2RustUnnamed = 20;
pub const GPG_ERR_BAD_KEY: C2RustUnnamed = 19;
pub const GPG_ERR_WRONG_SECKEY: C2RustUnnamed = 18;
pub const GPG_ERR_NO_SECKEY: C2RustUnnamed = 17;
pub const GPG_ERR_NO_USER_ID: C2RustUnnamed = 16;
pub const GPG_ERR_INV_ARMOR: C2RustUnnamed = 15;
pub const GPG_ERR_INV_PACKET: C2RustUnnamed = 14;
pub const GPG_ERR_KEYRING_OPEN: C2RustUnnamed = 13;
pub const GPG_ERR_CIPHER_ALGO: C2RustUnnamed = 12;
pub const GPG_ERR_BAD_PASSPHRASE: C2RustUnnamed = 11;
pub const GPG_ERR_CHECKSUM: C2RustUnnamed = 10;
pub const GPG_ERR_NO_PUBKEY: C2RustUnnamed = 9;
pub const GPG_ERR_BAD_SIGNATURE: C2RustUnnamed = 8;
pub const GPG_ERR_BAD_SECKEY: C2RustUnnamed = 7;
pub const GPG_ERR_BAD_PUBKEY: C2RustUnnamed = 6;
pub const GPG_ERR_DIGEST_ALGO: C2RustUnnamed = 5;
pub const GPG_ERR_PUBKEY_ALGO: C2RustUnnamed = 4;
pub const GPG_ERR_UNKNOWN_VERSION: C2RustUnnamed = 3;
pub const GPG_ERR_UNKNOWN_PACKET: C2RustUnnamed = 2;
pub const GPG_ERR_GENERAL: C2RustUnnamed = 1;
pub const GPG_ERR_NO_ERROR: C2RustUnnamed = 0;
pub type gpg_error_t = libc::c_uint;
pub type gcry_error_t = gpg_error_t;
pub type gcry_cipher_hd_t = *mut gcry_cipher_handle;
pub type gcry_cipher_algos = libc::c_uint;
pub const GCRY_CIPHER_CHACHA20: gcry_cipher_algos = 316;
pub const GCRY_CIPHER_GOST28147: gcry_cipher_algos = 315;
pub const GCRY_CIPHER_SALSA20R12: gcry_cipher_algos = 314;
pub const GCRY_CIPHER_SALSA20: gcry_cipher_algos = 313;
pub const GCRY_CIPHER_CAMELLIA256: gcry_cipher_algos = 312;
pub const GCRY_CIPHER_CAMELLIA192: gcry_cipher_algos = 311;
pub const GCRY_CIPHER_CAMELLIA128: gcry_cipher_algos = 310;
pub const GCRY_CIPHER_SEED: gcry_cipher_algos = 309;
pub const GCRY_CIPHER_RFC2268_128: gcry_cipher_algos = 308;
pub const GCRY_CIPHER_RFC2268_40: gcry_cipher_algos = 307;
pub const GCRY_CIPHER_SERPENT256: gcry_cipher_algos = 306;
pub const GCRY_CIPHER_SERPENT192: gcry_cipher_algos = 305;
pub const GCRY_CIPHER_SERPENT128: gcry_cipher_algos = 304;
pub const GCRY_CIPHER_TWOFISH128: gcry_cipher_algos = 303;
pub const GCRY_CIPHER_DES: gcry_cipher_algos = 302;
pub const GCRY_CIPHER_ARCFOUR: gcry_cipher_algos = 301;
pub const GCRY_CIPHER_TWOFISH: gcry_cipher_algos = 10;
pub const GCRY_CIPHER_AES256: gcry_cipher_algos = 9;
pub const GCRY_CIPHER_AES192: gcry_cipher_algos = 8;
pub const GCRY_CIPHER_AES: gcry_cipher_algos = 7;
pub const GCRY_CIPHER_DES_SK: gcry_cipher_algos = 6;
pub const GCRY_CIPHER_SAFER_SK128: gcry_cipher_algos = 5;
pub const GCRY_CIPHER_BLOWFISH: gcry_cipher_algos = 4;
pub const GCRY_CIPHER_CAST5: gcry_cipher_algos = 3;
pub const GCRY_CIPHER_3DES: gcry_cipher_algos = 2;
pub const GCRY_CIPHER_IDEA: gcry_cipher_algos = 1;
pub const GCRY_CIPHER_NONE: gcry_cipher_algos = 0;
pub type gcry_cipher_modes = libc::c_uint;
pub const GCRY_CIPHER_MODE_XTS: gcry_cipher_modes = 13;
pub const GCRY_CIPHER_MODE_CFB8: gcry_cipher_modes = 12;
pub const GCRY_CIPHER_MODE_OCB: gcry_cipher_modes = 11;
pub const GCRY_CIPHER_MODE_POLY1305: gcry_cipher_modes = 10;
pub const GCRY_CIPHER_MODE_GCM: gcry_cipher_modes = 9;
pub const GCRY_CIPHER_MODE_CCM: gcry_cipher_modes = 8;
pub const GCRY_CIPHER_MODE_AESWRAP: gcry_cipher_modes = 7;
pub const GCRY_CIPHER_MODE_CTR: gcry_cipher_modes = 6;
pub const GCRY_CIPHER_MODE_OFB: gcry_cipher_modes = 5;
pub const GCRY_CIPHER_MODE_STREAM: gcry_cipher_modes = 4;
pub const GCRY_CIPHER_MODE_CBC: gcry_cipher_modes = 3;
pub const GCRY_CIPHER_MODE_CFB: gcry_cipher_modes = 2;
pub const GCRY_CIPHER_MODE_ECB: gcry_cipher_modes = 1;
pub const GCRY_CIPHER_MODE_NONE: gcry_cipher_modes = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoListHead {
    pub next: *mut PianoListHead,
}
pub type PianoListHead_t = PianoListHead;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoUserInfo {
    pub listenerId: *mut libc::c_char,
    pub authToken: *mut libc::c_char,
}
pub type PianoUserInfo_t = PianoUserInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoStation {
    pub head: PianoListHead_t,
    pub isCreator: libc::c_char,
    pub isQuickMix: libc::c_char,
    pub useQuickMix: libc::c_char,
    pub name: *mut libc::c_char,
    pub id: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
}
pub type PianoStation_t = PianoStation;
pub type PianoSongRating_t = libc::c_uint;
pub const PIANO_RATE_TIRED: PianoSongRating_t = 3;
pub const PIANO_RATE_BAN: PianoSongRating_t = 2;
pub const PIANO_RATE_LOVE: PianoSongRating_t = 1;
pub const PIANO_RATE_NONE: PianoSongRating_t = 0;
pub type PianoAudioFormat_t = libc::c_uint;
pub const PIANO_AF_MP3: PianoAudioFormat_t = 2;
pub const PIANO_AF_AACPLUS: PianoAudioFormat_t = 1;
pub const PIANO_AF_UNKNOWN: PianoAudioFormat_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoSong {
    pub head: PianoListHead_t,
    pub artist: *mut libc::c_char,
    pub stationId: *mut libc::c_char,
    pub album: *mut libc::c_char,
    pub audioUrl: *mut libc::c_char,
    pub coverArt: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
    pub feedbackId: *mut libc::c_char,
    pub detailUrl: *mut libc::c_char,
    pub trackToken: *mut libc::c_char,
    pub fileGain: libc::c_float,
    pub length: libc::c_uint,
    pub rating: PianoSongRating_t,
    pub audioFormat: PianoAudioFormat_t,
}
pub type PianoSong_t = PianoSong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoArtist {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
    pub seedId: *mut libc::c_char,
    pub score: libc::c_int,
}
pub type PianoArtist_t = PianoArtist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoGenre {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub musicId: *mut libc::c_char,
}
pub type PianoGenre_t = PianoGenre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoGenreCategory {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub genres: *mut PianoGenre_t,
}
pub type PianoGenreCategory_t = PianoGenreCategory;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoPartner {
    pub in_0: gcry_cipher_hd_t,
    pub out: gcry_cipher_hd_t,
    pub authToken: *mut libc::c_char,
    pub device: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub id: libc::c_uint,
}
pub type PianoPartner_t = PianoPartner;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoHandle {
    pub user: PianoUserInfo_t,
    pub stations: *mut PianoStation_t,
    pub genreStations: *mut PianoGenreCategory_t,
    pub partner: PianoPartner_t,
    pub timeOffset: libc::c_int,
}
pub type PianoHandle_t = PianoHandle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoSearchResult {
    pub songs: *mut PianoSong_t,
    pub artists: *mut PianoArtist_t,
}
pub type PianoSearchResult_t = PianoSearchResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoStationInfo_t {
    pub songSeeds: *mut PianoSong_t,
    pub artistSeeds: *mut PianoArtist_t,
    pub stationSeeds: *mut PianoStation_t,
    pub feedback: *mut PianoSong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoStationMode_t {
    pub head: PianoListHead_t,
    pub name: *mut libc::c_char,
    pub description: *mut libc::c_char,
    pub isAlgorithmic: bool,
    pub isTakeover: bool,
    pub active: bool,
    pub id: libc::c_int,
}
pub type PianoRequestType_t = libc::c_uint;
pub const PIANO_REQUEST_SET_STATION_MODE: PianoRequestType_t = 26;
pub const PIANO_REQUEST_GET_STATION_MODES: PianoRequestType_t = 25;
pub const PIANO_REQUEST_CHANGE_SETTINGS: PianoRequestType_t = 24;
pub const PIANO_REQUEST_GET_SETTINGS: PianoRequestType_t = 23;
pub const PIANO_REQUEST_DELETE_SEED: PianoRequestType_t = 22;
pub const PIANO_REQUEST_DELETE_FEEDBACK: PianoRequestType_t = 21;
pub const PIANO_REQUEST_GET_STATION_INFO: PianoRequestType_t = 20;
pub const PIANO_REQUEST_BOOKMARK_ARTIST: PianoRequestType_t = 19;
pub const PIANO_REQUEST_BOOKMARK_SONG: PianoRequestType_t = 18;
pub const PIANO_REQUEST_EXPLAIN: PianoRequestType_t = 16;
pub const PIANO_REQUEST_TRANSFORM_STATION: PianoRequestType_t = 15;
pub const PIANO_REQUEST_GET_GENRE_STATIONS: PianoRequestType_t = 14;
pub const PIANO_REQUEST_SET_QUICKMIX: PianoRequestType_t = 13;
pub const PIANO_REQUEST_ADD_TIRED_SONG: PianoRequestType_t = 12;
pub const PIANO_REQUEST_ADD_SEED: PianoRequestType_t = 11;
pub const PIANO_REQUEST_CREATE_STATION: PianoRequestType_t = 10;
pub const PIANO_REQUEST_SEARCH: PianoRequestType_t = 9;
pub const PIANO_REQUEST_DELETE_STATION: PianoRequestType_t = 8;
pub const PIANO_REQUEST_RENAME_STATION: PianoRequestType_t = 7;
pub const PIANO_REQUEST_ADD_FEEDBACK: PianoRequestType_t = 5;
pub const PIANO_REQUEST_RATE_SONG: PianoRequestType_t = 4;
pub const PIANO_REQUEST_GET_PLAYLIST: PianoRequestType_t = 3;
pub const PIANO_REQUEST_GET_STATIONS: PianoRequestType_t = 2;
pub const PIANO_REQUEST_LOGIN: PianoRequestType_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PianoRequest {
    pub type_0: PianoRequestType_t,
    pub secure: bool,
    pub data: *mut libc::c_void,
    pub urlPath: [libc::c_char; 1024],
    pub postData: *mut libc::c_char,
    pub responseData: *mut libc::c_char,
}
pub type PianoRequest_t = PianoRequest;
pub type PianoReturn_t = libc::c_uint;
pub const PIANO_RET_P_RATE_LIMIT: PianoReturn_t = 2063;
pub const PIANO_RET_P_ZIP_CODE_INVALID: PianoReturn_t = 2048;
pub const PIANO_RET_P_USER_NOT_AUTHORIZED: PianoReturn_t = 2028;
pub const PIANO_RET_P_LISTENER_NOT_AUTHORIZED: PianoReturn_t = 2027;
pub const PIANO_RET_P_USER_ALREADY_USED_TRIAL: PianoReturn_t = 2061;
pub const PIANO_RET_P_USERNAME_ALREADY_EXISTS: PianoReturn_t = 2037;
pub const PIANO_RET_P_URL_PARAM_MISSING_USER_ID: PianoReturn_t = 1029;
pub const PIANO_RET_P_URL_PARAM_MISSING_PARTNER_ID: PianoReturn_t = 1028;
pub const PIANO_RET_P_URL_PARAM_MISSING_METHOD: PianoReturn_t = 1026;
pub const PIANO_RET_P_URL_PARAM_MISSING_AUTH_TOKEN: PianoReturn_t = 1027;
pub const PIANO_RET_P_UPGRADE_DEVICE_MODEL_INVALID: PianoReturn_t = 2039;
pub const PIANO_RET_P_STATION_DOES_NOT_EXIST: PianoReturn_t = 2030;
pub const PIANO_RET_P_SECURE_PROTOCOL_REQUIRED: PianoReturn_t = 1030;
pub const PIANO_RET_P_READ_ONLY_MODE: PianoReturn_t = 2024;
pub const PIANO_RET_P_PARTNER_NOT_AUTHORIZED: PianoReturn_t = 2034;
pub const PIANO_RET_P_PARAMETER_VALUE_INVALID: PianoReturn_t = 1034;
pub const PIANO_RET_P_PARAMETER_TYPE_MISMATCH: PianoReturn_t = 1032;
pub const PIANO_RET_P_PARAMETER_MISSING: PianoReturn_t = 1033;
pub const PIANO_RET_P_MAX_STATIONS_REACHED: PianoReturn_t = 2029;
pub const PIANO_RET_P_MAINTENANCE_MODE: PianoReturn_t = 1025;
pub const PIANO_RET_P_LICENSING_RESTRICTIONS: PianoReturn_t = 1036;
pub const PIANO_RET_P_INVALID_USERNAME: PianoReturn_t = 2035;
pub const PIANO_RET_P_INVALID_SPONSOR: PianoReturn_t = 2060;
pub const PIANO_RET_P_INVALID_PASSWORD: PianoReturn_t = 2036;
pub const PIANO_RET_P_INVALID_PARTNER_LOGIN: PianoReturn_t = 2026;
pub const PIANO_RET_P_INVALID_GENDER: PianoReturn_t = 2051;
pub const PIANO_RET_P_INVALID_COUNTRY_CODE: PianoReturn_t = 2051;
pub const PIANO_RET_P_INVALID_AUTH_TOKEN: PianoReturn_t = 2025;
pub const PIANO_RET_P_INSUFFICIENT_CONNECTIVITY: PianoReturn_t = 1037;
pub const PIANO_RET_P_EXPLICIT_PIN_MALFORMED: PianoReturn_t = 2044;
pub const PIANO_RET_P_EXPLICIT_PIN_INCORRECT: PianoReturn_t = 2042;
pub const PIANO_RET_P_DEVICE_NOT_FOUND: PianoReturn_t = 2033;
pub const PIANO_RET_P_DEVICE_MODEL_INVALID: PianoReturn_t = 2047;
pub const PIANO_RET_P_DEVICE_DISABLED: PianoReturn_t = 2058;
pub const PIANO_RET_P_DEVICE_ALREADY_ASSOCIATED_TO_ACCOUNT: PianoReturn_t = 2038;
pub const PIANO_RET_P_DAILY_TRIAL_LIMIT_REACHED: PianoReturn_t = 2059;
pub const PIANO_RET_P_COMPLIMENTARY_PERIOD_ALREADY_IN_USE: PianoReturn_t = 2031;
pub const PIANO_RET_P_CERTIFICATE_REQUIRED: PianoReturn_t = 1031;
pub const PIANO_RET_P_CALL_NOT_ALLOWED: PianoReturn_t = 2032;
pub const PIANO_RET_P_BIRTH_YEAR_TOO_YOUNG: PianoReturn_t = 2050;
pub const PIANO_RET_P_BIRTH_YEAR_INVALID: PianoReturn_t = 2049;
pub const PIANO_RET_P_API_VERSION_NOT_SUPPORTED: PianoReturn_t = 1035;
pub const PIANO_RET_P_INTERNAL: PianoReturn_t = 1024;
pub const PIANO_RET_GCRY_ERR: PianoReturn_t = 7;
pub const PIANO_RET_QUALITY_UNAVAILABLE: PianoReturn_t = 6;
pub const PIANO_RET_INVALID_LOGIN: PianoReturn_t = 5;
pub const PIANO_RET_OUT_OF_MEMORY: PianoReturn_t = 4;
pub const PIANO_RET_CONTINUE_REQUEST: PianoReturn_t = 3;
pub const PIANO_RET_INVALID_RESPONSE: PianoReturn_t = 2;
pub const PIANO_RET_OK: PianoReturn_t = 1;
pub const PIANO_RET_ERR: PianoReturn_t = 0;
pub unsafe extern "C" fn PianoInit(
    mut ph: *mut PianoHandle_t,
    mut partnerUser: *const libc::c_char,
    mut partnerPassword: *const libc::c_char,
    mut device: *const libc::c_char,
    mut inkey: *const libc::c_char,
    mut outkey: *const libc::c_char,
) -> PianoReturn_t {
    memset(
        ph as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoHandle_t>() as libc::c_ulong,
    );
    (*ph).partner.user = strdup(partnerUser);
    (*ph).partner.password = strdup(partnerPassword);
    (*ph).partner.device = strdup(device);
    if gcry_cipher_open(
        &mut (*ph).partner.in_0,
        GCRY_CIPHER_BLOWFISH as libc::c_int,
        GCRY_CIPHER_MODE_ECB as libc::c_int,
        0 as libc::c_int as libc::c_uint,
    ) != GPG_ERR_NO_ERROR as libc::c_int as libc::c_uint
    {
        return PIANO_RET_GCRY_ERR;
    }
    if gcry_cipher_setkey(
        (*ph).partner.in_0,
        inkey as *const libc::c_uchar as *const libc::c_void,
        strlen(inkey),
    ) != GPG_ERR_NO_ERROR as libc::c_int as libc::c_uint
    {
        return PIANO_RET_GCRY_ERR;
    }
    if gcry_cipher_open(
        &mut (*ph).partner.out,
        GCRY_CIPHER_BLOWFISH as libc::c_int,
        GCRY_CIPHER_MODE_ECB as libc::c_int,
        0 as libc::c_int as libc::c_uint,
    ) != GPG_ERR_NO_ERROR as libc::c_int as libc::c_uint
    {
        return PIANO_RET_GCRY_ERR;
    }
    if gcry_cipher_setkey(
        (*ph).partner.out,
        outkey as *const libc::c_uchar as *const libc::c_void,
        strlen(outkey),
    ) != GPG_ERR_NO_ERROR as libc::c_int as libc::c_uint
    {
        return PIANO_RET_GCRY_ERR;
    }
    return PIANO_RET_OK;
}
unsafe extern "C" fn PianoDestroyArtists(mut artists: *mut PianoArtist_t) {
    let mut curArtist: *mut PianoArtist_t = 0 as *mut PianoArtist_t;
    let mut lastArtist: *mut PianoArtist_t = 0 as *mut PianoArtist_t;
    curArtist = artists;
    while !curArtist.is_null() {
        free((*curArtist).name as *mut libc::c_void);
        free((*curArtist).musicId as *mut libc::c_void);
        free((*curArtist).seedId as *mut libc::c_void);
        lastArtist = curArtist;
        curArtist = (*curArtist).head.next as *mut PianoArtist_t;
        free(lastArtist as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn PianoDestroySearchResult(
    mut searchResult: *mut PianoSearchResult_t,
) {
    PianoDestroyArtists((*searchResult).artists);
    PianoDestroyPlaylist((*searchResult).songs);
}
pub unsafe extern "C" fn PianoDestroyStation(mut station: *mut PianoStation_t) {
    free((*station).name as *mut libc::c_void);
    free((*station).id as *mut libc::c_void);
    free((*station).seedId as *mut libc::c_void);
    memset(
        station as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoStation_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn PianoDestroyStations(mut stations: *mut PianoStation_t) {
    let mut curStation: *mut PianoStation_t = 0 as *mut PianoStation_t;
    let mut lastStation: *mut PianoStation_t = 0 as *mut PianoStation_t;
    curStation = stations;
    while !curStation.is_null() {
        lastStation = curStation;
        curStation = (*curStation).head.next as *mut PianoStation_t;
        PianoDestroyStation(lastStation);
        free(lastStation as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn PianoDestroyPlaylist(mut playlist: *mut PianoSong_t) {
    let mut curSong: *mut PianoSong_t = 0 as *mut PianoSong_t;
    let mut lastSong: *mut PianoSong_t = 0 as *mut PianoSong_t;
    curSong = playlist;
    while !curSong.is_null() {
        free((*curSong).audioUrl as *mut libc::c_void);
        free((*curSong).coverArt as *mut libc::c_void);
        free((*curSong).artist as *mut libc::c_void);
        free((*curSong).musicId as *mut libc::c_void);
        free((*curSong).title as *mut libc::c_void);
        free((*curSong).stationId as *mut libc::c_void);
        free((*curSong).album as *mut libc::c_void);
        free((*curSong).feedbackId as *mut libc::c_void);
        free((*curSong).seedId as *mut libc::c_void);
        free((*curSong).detailUrl as *mut libc::c_void);
        free((*curSong).trackToken as *mut libc::c_void);
        lastSong = curSong;
        curSong = (*curSong).head.next as *mut PianoSong_t;
        free(lastSong as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn PianoDestroyStationInfo(mut info: *mut PianoStationInfo_t) {
    PianoDestroyPlaylist((*info).feedback);
    PianoDestroyPlaylist((*info).songSeeds);
    PianoDestroyArtists((*info).artistSeeds);
    PianoDestroyStations((*info).stationSeeds);
}
unsafe extern "C" fn PianoDestroyGenres(mut genres: *mut PianoGenre_t) {
    let mut curGenre: *mut PianoGenre_t = 0 as *mut PianoGenre_t;
    let mut lastGenre: *mut PianoGenre_t = 0 as *mut PianoGenre_t;
    curGenre = genres;
    while !curGenre.is_null() {
        free((*curGenre).name as *mut libc::c_void);
        free((*curGenre).musicId as *mut libc::c_void);
        lastGenre = curGenre;
        curGenre = (*curGenre).head.next as *mut PianoGenre_t;
        free(lastGenre as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn PianoDestroyUserInfo(mut user: *mut PianoUserInfo_t) {
    free((*user).authToken as *mut libc::c_void);
    free((*user).listenerId as *mut libc::c_void);
}
unsafe extern "C" fn PianoDestroyPartner(mut partner: *mut PianoPartner_t) {
    free((*partner).user as *mut libc::c_void);
    free((*partner).password as *mut libc::c_void);
    free((*partner).device as *mut libc::c_void);
    free((*partner).authToken as *mut libc::c_void);
    gcry_cipher_close((*partner).in_0);
    gcry_cipher_close((*partner).out);
    memset(
        partner as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoPartner_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn PianoDestroyStationMode(modes: *mut PianoStationMode_t) {
    let mut curMode: *mut PianoStationMode_t = modes;
    while !curMode.is_null() {
        free((*curMode).name as *mut libc::c_void);
        free((*curMode).description as *mut libc::c_void);
        let lastMode: *mut PianoStationMode_t = curMode;
        curMode = (*curMode).head.next as *mut PianoStationMode_t;
        free(lastMode as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn PianoDestroy(mut ph: *mut PianoHandle_t) {
    PianoDestroyUserInfo(&mut (*ph).user);
    PianoDestroyStations((*ph).stations);
    PianoDestroyPartner(&mut (*ph).partner);
    let mut curGenreCat: *mut PianoGenreCategory_t = (*ph).genreStations;
    let mut lastGenreCat: *mut PianoGenreCategory_t = 0 as *mut PianoGenreCategory_t;
    while !curGenreCat.is_null() {
        PianoDestroyGenres((*curGenreCat).genres);
        free((*curGenreCat).name as *mut libc::c_void);
        lastGenreCat = curGenreCat;
        curGenreCat = (*curGenreCat).head.next as *mut PianoGenreCategory_t;
        free(lastGenreCat as *mut libc::c_void);
    }
    memset(
        ph as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoHandle_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn PianoDestroyRequest(mut req: *mut PianoRequest_t) {
    free((*req).postData as *mut libc::c_void);
    memset(
        req as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<PianoRequest_t>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn PianoFindStationById(
    stations: *mut PianoStation_t,
    searchStation: *const libc::c_char,
) -> *mut PianoStation_t {
    if searchStation.is_null() {
        return 0 as *mut PianoStation_t;
    }
    let mut currStation: *mut PianoStation_t = stations;
    while !currStation.is_null() {
        if strcmp((*currStation).id, searchStation) == 0 as libc::c_int {
            return currStation;
        }
        currStation = (*currStation).head.next as *mut libc::c_void
            as *mut PianoStation_t;
    }
    return 0 as *mut PianoStation_t;
}
pub unsafe extern "C" fn PianoErrorToStr(mut ret: PianoReturn_t) -> *const libc::c_char {
    match ret as libc::c_uint {
        1 => return b"Everything is fine :)\0" as *const u8 as *const libc::c_char,
        0 => return b"Unknown.\0" as *const u8 as *const libc::c_char,
        2 => return b"Invalid response.\0" as *const u8 as *const libc::c_char,
        3 => return b"Fix your program.\0" as *const u8 as *const libc::c_char,
        4 => return b"Out of memory.\0" as *const u8 as *const libc::c_char,
        5 => {
            return b"Wrong email address or password.\0" as *const u8
                as *const libc::c_char;
        }
        6 => {
            return b"Selected audio quality is not available.\0" as *const u8
                as *const libc::c_char;
        }
        7 => {
            return b"libgcrypt initialization failed.\0" as *const u8
                as *const libc::c_char;
        }
        1024 => return b"Internal error.\0" as *const u8 as *const libc::c_char,
        2032 => return b"Call not allowed.\0" as *const u8 as *const libc::c_char,
        2025 => return b"Invalid auth token.\0" as *const u8 as *const libc::c_char,
        1025 => return b"Maintenance mode.\0" as *const u8 as *const libc::c_char,
        2029 => {
            return b"Max number of stations reached.\0" as *const u8
                as *const libc::c_char;
        }
        2024 => {
            return b"Read only mode. Try again later.\0" as *const u8
                as *const libc::c_char;
        }
        2030 => return b"Station does not exist.\0" as *const u8 as *const libc::c_char,
        2026 => return b"Invalid partner login.\0" as *const u8 as *const libc::c_char,
        1036 => {
            return b"Pandora is not available in your country. Set up a control proxy (see manpage).\0"
                as *const u8 as *const libc::c_char;
        }
        2034 => {
            return b"Invalid partner credentials.\0" as *const u8 as *const libc::c_char;
        }
        2027 => return b"Listener not authorized.\0" as *const u8 as *const libc::c_char,
        2063 => {
            return b"Access denied. Try again later.\0" as *const u8
                as *const libc::c_char;
        }
        _ => return b"No error message available.\0" as *const u8 as *const libc::c_char,
    };
}
