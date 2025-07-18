use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_M_Host_Name_From_Name(host_name: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_M_Host_Name_From_Adr(host_address: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_M_Absolute_Path_Name(src: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Rd_C_Int_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Check_For_Un_Atom(start_word: WamWord);
    fn Pl_Check_For_Un_Variable(start_word: WamWord);
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_domain_os_path: libc::c_int;
    static mut pl_domain_socket_domain: libc::c_int;
    static mut pl_domain_socket_address: libc::c_int;
    fn Pl_Os_Error(ret_val: libc::c_int);
    fn Pl_Err_Instantiation();
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Add_Stream_For_Stdio_Desc(
        f: *mut FILE,
        atom_path: libc::c_int,
        mode: libc::c_int,
        text: Bool,
        force_eof_reset: Bool,
    ) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type size_t = libc::c_ulong;
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
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct StmProp {
    #[bitfield(name = "mode", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "input", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "output", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "text", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "reposition", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "eof_action", ty = "libc::c_uint", bits = "6..=7")]
    #[bitfield(name = "buffering", ty = "libc::c_uint", bits = "8..=9")]
    #[bitfield(name = "special_close", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "other", ty = "libc::c_uint", bits = "11..=15")]
    pub mode_input_output_text_reposition_eof_action_buffering_special_close_other: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PbStk {
    pub buff: [libc::c_int; 8],
    pub ptr: *mut libc::c_int,
    pub nb_elems: libc::c_int,
}
pub type StmFct = Option::<unsafe extern "C" fn() -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_lst {
    pub stm: libc::c_int,
    pub next: PStmLst,
}
pub type PStmLst = *mut stm_lst;
pub type StmLst = stm_lst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_inf {
    pub atom_file_name: libc::c_int,
    pub file: PlLong,
    pub prop: StmProp,
    pub mirror: *mut StmLst,
    pub mirror_of: *mut StmLst,
    pub fct_getc: StmFct,
    pub fct_putc: StmFct,
    pub fct_flush: StmFct,
    pub fct_close: StmFct,
    pub fct_tell: StmFct,
    pub fct_seek: StmFct,
    pub fct_clearerr: StmFct,
    pub eof_reached: Bool,
    pub pb_char: PbStk,
    pub char_count: PlLong,
    pub line_count: PlLong,
    pub line_pos: PlLong,
    pub pb_line_pos: PbStk,
}
pub type StmInf = stm_inf;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut atom_AF_UNIX: libc::c_int = 0;
static mut atom_AF_INET: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Socket_2(
    mut domain_word: WamWord,
    mut socket_word: WamWord,
) -> Bool {
    let mut domain: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut proto: libc::c_int = 0 as libc::c_int;
    domain = Pl_Rd_Atom_Check(domain_word);
    if domain != atom_AF_UNIX && domain != atom_AF_INET {
        Pl_Err_Domain(pl_domain_socket_domain, domain_word);
    }
    Pl_Check_For_Un_Variable(socket_word);
    if domain == atom_AF_UNIX {
        sock = socket(1 as libc::c_int, SOCK_STREAM as libc::c_int, proto);
    } else {
        sock = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, proto);
    }
    let mut _tst: libc::c_int = sock;
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    opt = 1 as libc::c_int;
    let mut _tst_0: libc::c_int = setsockopt(
        sock,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut opt as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if _tst_0 < 0 as libc::c_int {
        Pl_Os_Error(_tst_0);
        return 0 as libc::c_int;
    }
    return Pl_Get_Integer(sock as PlLong, socket_word);
}
pub unsafe extern "C" fn Pl_Socket_Close_1(mut socket_word: WamWord) -> Bool {
    let mut sock: libc::c_int = 0;
    sock = Pl_Rd_C_Int_Check(socket_word);
    if sock < 2 as libc::c_int {
        *__errno_location() = 9 as libc::c_int;
        let mut _tst: libc::c_int = -(1 as libc::c_int);
        if _tst < 0 as libc::c_int {
            Pl_Os_Error(_tst);
            return 0 as libc::c_int;
        }
    } else {
        let mut _tst_0: libc::c_int = close(sock);
        if _tst_0 < 0 as libc::c_int {
            Pl_Os_Error(_tst_0);
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Socket_Bind_2(
    mut socket_word: WamWord,
    mut address_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    let mut sock: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut l: socklen_t = 0;
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut adr_un: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut adr_in: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    static mut atom_host_name: libc::c_int = -(1 as libc::c_int);
    sock = Pl_Rd_C_Int_Check(socket_word);
    let mut deref_last_word: WamWord = 0;
    word = address_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong {
        current_block = 4634199788181062148;
    } else {
        current_block = 5143058163439228106;
    }
    loop {
        match current_block {
            4634199788181062148 => {
                Pl_Err_Domain(pl_domain_socket_address, word);
                current_block = 5143058163439228106;
            }
            _ => {
                stc_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                if (1 as libc::c_int as PlULong)
                    << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                    | atom_AF_UNIX as libc::c_ulong
                    == *stc_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
                {
                    path_name = Pl_Rd_String_Check(
                        *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                    );
                    path_name = Pl_M_Absolute_Path_Name(path_name);
                    if path_name.is_null() {
                        Pl_Err_Domain(
                            pl_domain_os_path,
                            *stc_adr
                                .offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                        );
                    }
                    adr_un.sun_family = 1 as libc::c_int as sa_family_t;
                    strcpy((adr_un.sun_path).as_mut_ptr(), path_name);
                    unlink(path_name);
                    let mut _tst: libc::c_int = bind(
                        sock,
                        &mut adr_un as *mut sockaddr_un as *mut sockaddr,
                        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong
                            as socklen_t,
                    );
                    if _tst < 0 as libc::c_int {
                        Pl_Os_Error(_tst);
                        return 0 as libc::c_int;
                    }
                    return 1 as libc::c_int;
                }
                if (2 as libc::c_int as PlULong)
                    << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                    | atom_AF_INET as libc::c_ulong
                    != *stc_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
                {
                    current_block = 4634199788181062148;
                } else {
                    break;
                }
            }
        }
    }
    let mut deref_last_word_0: WamWord = 0;
    word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
    loop {
        deref_last_word_0 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_0) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        if atom_host_name < 0 as libc::c_int {
            atom_host_name = Pl_Create_Allocate_Atom(
                Pl_M_Host_Name_From_Name(0 as *mut libc::c_char),
            );
        }
        Pl_Get_Atom(atom_host_name, word);
    } else {
        Pl_Rd_Atom_Check(word);
    }
    port = 0 as libc::c_int;
    let mut deref_last_word_1: WamWord = 0;
    word = *stc_adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize);
    loop {
        deref_last_word_1 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_1) {
            break;
        }
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        port = Pl_Rd_C_Int_Check(word);
    }
    adr_in.sin_port = __bswap_16(port as libc::c_ushort);
    adr_in.sin_family = 2 as libc::c_int as sa_family_t;
    adr_in.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    let mut _tst_0: libc::c_int = bind(
        sock,
        &mut adr_in as *mut sockaddr_in as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if _tst_0 < 0 as libc::c_int {
        Pl_Os_Error(_tst_0);
        return 0 as libc::c_int;
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        return 1 as libc::c_int;
    }
    l = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    let mut _tst_1: libc::c_int = getsockname(
        sock,
        &mut adr_in as *mut sockaddr_in as *mut sockaddr,
        &mut l,
    );
    if _tst_1 < 0 as libc::c_int {
        Pl_Os_Error(_tst_1);
        return 0 as libc::c_int;
    }
    port = __bswap_16(adr_in.sin_port) as libc::c_int;
    return Pl_Get_Integer(port as PlLong, word);
}
pub unsafe extern "C" fn Pl_Socket_Connect_4(
    mut socket_word: WamWord,
    mut address_word: WamWord,
    mut stm_in_word: WamWord,
    mut stm_out_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    let mut sock: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut host_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut adr_un: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut adr_in: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut host_entry: *mut hostent = 0 as *mut hostent;
    let mut stm_in: libc::c_int = 0;
    let mut stm_out: libc::c_int = 0;
    let mut stream_name: [libc::c_char; 256] = [0; 256];
    sock = Pl_Rd_C_Int_Check(socket_word);
    let mut deref_last_word: WamWord = 0;
    word = address_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong {
        current_block = 15510551920695145309;
    } else {
        current_block = 17833034027772472439;
    }
    loop {
        match current_block {
            15510551920695145309 => {
                Pl_Err_Domain(pl_domain_socket_address, word);
                current_block = 17833034027772472439;
            }
            _ => {
                stc_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                if (1 as libc::c_int as PlULong)
                    << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                    | atom_AF_UNIX as libc::c_ulong
                    == *stc_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
                {
                    path_name = Pl_Rd_String_Check(
                        *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                    );
                    path_name = Pl_M_Absolute_Path_Name(path_name);
                    if path_name.is_null() {
                        Pl_Err_Domain(
                            pl_domain_os_path,
                            *stc_adr
                                .offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                        );
                    }
                    adr_un.sun_family = 1 as libc::c_int as sa_family_t;
                    strcpy((adr_un.sun_path).as_mut_ptr(), path_name);
                    let mut _tst: libc::c_int = connect(
                        sock,
                        &mut adr_un as *mut sockaddr_un as *mut sockaddr,
                        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong
                            as socklen_t,
                    );
                    if _tst < 0 as libc::c_int {
                        Pl_Os_Error(_tst);
                        return 0 as libc::c_int;
                    }
                    sprintf(
                        stream_name.as_mut_ptr(),
                        b"socket_stream(connect('AF_UNIX'('%s')),%d)\0" as *const u8
                            as *const libc::c_char,
                        path_name,
                        sock,
                    );
                    break;
                } else {
                    if (2 as libc::c_int as PlULong)
                        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                        | atom_AF_INET as libc::c_ulong
                        != *stc_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
                    {
                        current_block = 15510551920695145309;
                        continue;
                    }
                    host_name = Pl_Rd_String_Check(
                        *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                    );
                    port = Pl_Rd_C_Int_Check(
                        *stc_adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize),
                    );
                    host_entry = gethostbyname(host_name);
                    if host_entry.is_null() {
                        return 0 as libc::c_int;
                    }
                    adr_in.sin_family = 2 as libc::c_int as sa_family_t;
                    adr_in.sin_port = __bswap_16(port as libc::c_ushort);
                    memcpy(
                        &mut adr_in.sin_addr as *mut in_addr as *mut libc::c_void,
                        *((*host_entry).h_addr_list).offset(0 as libc::c_int as isize)
                            as *const libc::c_void,
                        (*host_entry).h_length as libc::c_ulong,
                    );
                    let mut _tst_0: libc::c_int = connect(
                        sock,
                        &mut adr_in as *mut sockaddr_in as *mut sockaddr,
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as socklen_t,
                    );
                    if _tst_0 < 0 as libc::c_int {
                        Pl_Os_Error(_tst_0);
                        return 0 as libc::c_int;
                    }
                    sprintf(
                        stream_name.as_mut_ptr(),
                        b"socket_stream(connect('AF_INET'('%s',%d)),%d)\0" as *const u8
                            as *const libc::c_char,
                        host_name,
                        port,
                        sock,
                    );
                    break;
                }
            }
        }
    }
    if Create_Socket_Streams(sock, stream_name.as_mut_ptr(), &mut stm_in, &mut stm_out)
        == 0
    {
        return 0 as libc::c_int;
    }
    Pl_Get_Integer(stm_in as PlLong, stm_in_word);
    Pl_Get_Integer(stm_out as PlLong, stm_out_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Socket_Listen_2(
    mut socket_word: WamWord,
    mut length_word: WamWord,
) -> Bool {
    let mut sock: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    sock = Pl_Rd_C_Int_Check(socket_word);
    length = Pl_Rd_C_Int_Check(length_word);
    let mut _tst: libc::c_int = listen(sock, length);
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Socket_Accept_4(
    mut socket_word: WamWord,
    mut client_word: WamWord,
    mut stm_in_word: WamWord,
    mut stm_out_word: WamWord,
) -> Bool {
    let mut sock: libc::c_int = 0;
    let mut cli_sock: libc::c_int = 0;
    let mut l: socklen_t = 0;
    let mut adr_in: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut stm_in: libc::c_int = 0;
    let mut stm_out: libc::c_int = 0;
    let mut cli_ip_adr: *mut libc::c_char = b"AF_UNIX\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut stream_name: [libc::c_char; 256] = [0; 256];
    l = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    sock = Pl_Rd_C_Int_Check(socket_word);
    cli_sock = accept(sock, &mut adr_in as *mut sockaddr_in as *mut sockaddr, &mut l);
    let mut _tst: libc::c_int = cli_sock;
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    if adr_in.sin_family as libc::c_int == 2 as libc::c_int {
        cli_ip_adr = inet_ntoa(adr_in.sin_addr);
        if cli_ip_adr.is_null() {
            return 0 as libc::c_int;
        }
        Pl_Get_Atom(Pl_Create_Allocate_Atom(cli_ip_adr), client_word);
    }
    sprintf(
        stream_name.as_mut_ptr(),
        b"socket_stream(accept('%s'),%d)\0" as *const u8 as *const libc::c_char,
        cli_ip_adr,
        cli_sock,
    );
    if Create_Socket_Streams(
        cli_sock,
        stream_name.as_mut_ptr(),
        &mut stm_in,
        &mut stm_out,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    Pl_Get_Integer(stm_in as PlLong, stm_in_word);
    Pl_Get_Integer(stm_out as PlLong, stm_out_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Assoc_Socket_Streams_3(
    mut socket_word: WamWord,
    mut stm_in_word: WamWord,
    mut stm_out_word: WamWord,
) -> Bool {
    let mut stm_in: libc::c_int = 0;
    let mut stm_out: libc::c_int = 0;
    let mut stream_name: [libc::c_char; 256] = [0; 256];
    let mut sock: libc::c_int = Pl_Rd_C_Int_Check(socket_word);
    sprintf(
        stream_name.as_mut_ptr(),
        b"socket_stream(assoc(%d))\0" as *const u8 as *const libc::c_char,
        sock,
    );
    if Create_Socket_Streams(sock, stream_name.as_mut_ptr(), &mut stm_in, &mut stm_out)
        == 0
    {
        return 0 as libc::c_int;
    }
    Pl_Get_Integer(stm_in as PlLong, stm_in_word);
    Pl_Get_Integer(stm_out as PlLong, stm_out_word);
    return 1 as libc::c_int;
}
unsafe extern "C" fn Create_Socket_Streams(
    mut sock: libc::c_int,
    mut stream_name: *mut libc::c_char,
    mut stm_in: *mut libc::c_int,
    mut stm_out: *mut libc::c_int,
) -> Bool {
    let mut fd: libc::c_int = 0;
    let mut f_in: *mut FILE = 0 as *mut FILE;
    let mut f_out: *mut FILE = 0 as *mut FILE;
    let mut atom: libc::c_int = 0;
    let mut stm: libc::c_int = 0;
    fd = dup(sock);
    let mut _tst: libc::c_int = fd;
    if _tst < 0 as libc::c_int {
        Pl_Os_Error(_tst);
        return 0 as libc::c_int;
    }
    f_in = fdopen(sock, b"rt\0" as *const u8 as *const libc::c_char);
    if f_in.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    f_out = fdopen(fd, b"wt\0" as *const u8 as *const libc::c_char);
    if f_out.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    atom = Pl_Create_Allocate_Atom(stream_name);
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f_in,
        atom,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let ref mut fresh0 = (**pl_stm_tbl.offset(stm as isize)).prop;
    (*fresh0).set_eof_action(2 as libc::c_int as libc::c_uint);
    let ref mut fresh1 = (**pl_stm_tbl.offset(stm as isize)).prop;
    (*fresh1).set_other(4 as libc::c_int as libc::c_uint);
    *stm_in = stm;
    stm = Pl_Add_Stream_For_Stdio_Desc(
        f_out,
        atom,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let ref mut fresh2 = (**pl_stm_tbl.offset(stm as isize)).prop;
    (*fresh2).set_other(4 as libc::c_int as libc::c_uint);
    *stm_out = stm;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Hostname_Address_2(
    mut host_name_word: WamWord,
    mut host_address_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut host_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_address: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_entry: *mut hostent = 0 as *mut hostent;
    let mut iadr: in_addr = in_addr { s_addr: 0 };
    let mut deref_last_word: WamWord = 0;
    word = host_name_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        host_address = Pl_Rd_String_Check(host_address_word);
        host_name = Pl_M_Host_Name_From_Adr(host_address);
        return (!host_name.is_null()
            && Pl_Un_String_Check(host_name, host_name_word) != 0) as libc::c_int;
    }
    host_name = Pl_Rd_String_Check(word);
    Pl_Check_For_Un_Atom(host_address_word);
    host_entry = gethostbyname(host_name);
    if host_entry.is_null() {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut iadr.s_addr as *mut in_addr_t as *mut libc::c_void,
        *((*host_entry).h_addr_list).offset(0 as libc::c_int as isize)
            as *const libc::c_void,
        (*host_entry).h_length as libc::c_ulong,
    );
    host_address = inet_ntoa(iadr);
    return Pl_Un_String_Check(host_address, host_address_word);
}
