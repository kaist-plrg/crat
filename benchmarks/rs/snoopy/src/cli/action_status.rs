use ::libc;
extern "C" {
    fn printMessage(message: *const libc::c_char);
    fn printDiagValue(message: *const libc::c_char, value: *const libc::c_char);
    fn printWarning(message: *const libc::c_char);
    fn fatalError(message: *const libc::c_char);
    fn libsnoopySo_getFilePath() -> *mut libc::c_char;
    fn etcLdSoPreload_readFile() -> *mut libc::c_char;
    fn etcLdSoPreload_findNonCommentLineContainingString(
        content: *const libc::c_char,
        searchString: *const libc::c_char,
    ) -> *const libc::c_char;
    fn etcLdSoPreload_findEntry(
        content: *const libc::c_char,
        entry: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut g_etcLdSoPreloadPath: *const libc::c_char;
    fn snoopy_util_string_getLineLength(
        lineStartPtr: *const libc::c_char,
    ) -> libc::c_int;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dl_iterate_phdr(
        __callback: Option::<
            unsafe extern "C" fn(
                *mut dl_phdr_info,
                size_t,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        __data: *mut libc::c_void,
    ) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type execveAddrT = Option::<unsafe extern "C" fn() -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solibsOrderT {
    pub libcPos: libc::c_int,
    pub libsnoopyPos: libc::c_int,
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dl_phdr_info {
    pub dlpi_addr: Elf64_Addr,
    pub dlpi_name: *const libc::c_char,
    pub dlpi_phdr: *const Elf64_Phdr,
    pub dlpi_phnum: Elf64_Half,
    pub dlpi_adds: libc::c_ulonglong,
    pub dlpi_subs: libc::c_ulonglong,
    pub dlpi_tls_modid: size_t,
    pub dlpi_tls_data: *mut libc::c_void,
}
pub type Elf64_Half = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
pub type Elf64_Xword = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type Elf64_Addr = uint64_t;
pub type Elf64_Off = uint64_t;
pub type Elf64_Word = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
unsafe extern "C" fn dl_iterate_phdr_callback(
    mut i: *mut dl_phdr_info,
    mut size: size_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    static mut callCount: libc::c_int = 0 as libc::c_int;
    let mut solibsOrder: *mut solibsOrderT = 0 as *mut solibsOrderT;
    let mut strPos: *const libc::c_char = 0 as *const libc::c_char;
    solibsOrder = data as *mut solibsOrderT;
    callCount += 1;
    callCount;
    let mut libsnoopyFileName: *const libc::c_char = b"/libsnoopy.so\0" as *const u8
        as *const libc::c_char;
    strPos = strstr((*i).dlpi_name, libsnoopyFileName);
    if !strPos.is_null()
        && *strPos.offset(strlen(libsnoopyFileName) as isize) as libc::c_int
            == '\0' as i32
    {
        (*solibsOrder).libsnoopyPos = callCount;
    }
    let mut libcFileName: *const libc::c_char = b"/libc.so.6\0" as *const u8
        as *const libc::c_char;
    strPos = strstr((*i).dlpi_name, libcFileName);
    if !strPos.is_null()
        && *strPos.offset(strlen(libcFileName) as isize) as libc::c_int == '\0' as i32
    {
        (*solibsOrder).libcPos = callCount;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopy_cli_action_status() -> libc::c_int {
    let mut libsnoopySoPath: *const libc::c_char = 0 as *const libc::c_char;
    let mut etcLdSoPreloadContent: *const libc::c_char = 0 as *const libc::c_char;
    let mut linePtr: *const libc::c_char = 0 as *const libc::c_char;
    let mut entryPtr: *const libc::c_char = 0 as *const libc::c_char;
    libsnoopySoPath = libsnoopySo_getFilePath();
    etcLdSoPreloadContent = etcLdSoPreload_readFile();
    linePtr = etcLdSoPreload_findNonCommentLineContainingString(
        etcLdSoPreloadContent,
        b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
    );
    if linePtr.is_null() {
        printMessage(
            b"/etc/ld.so.preload:            NOT OK - Snoopy is not enabled.\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        linePtr = linePtr.offset(snoopy_util_string_getLineLength(linePtr) as isize);
        linePtr = etcLdSoPreload_findNonCommentLineContainingString(
            linePtr,
            b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
        );
        if !linePtr.is_null() {
            printDiagValue(
                b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
                g_etcLdSoPreloadPath,
            );
            printDiagValue(
                b"Search string\0" as *const u8 as *const libc::c_char,
                b"libsnoopy.so\0" as *const u8 as *const libc::c_char,
            );
            fatalError(
                b"Multiple Snoopy references found, aborting.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        entryPtr = etcLdSoPreload_findEntry(etcLdSoPreloadContent, libsnoopySoPath);
        if entryPtr.is_null() {
            printMessage(
                b"/etc/ld.so.preload:            NOT OK - 'libsnoopy.so' found, but not with the expected path - is another Snoopy instance enabled?\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            printMessage(
                b"/etc/ld.so.preload:            OK - Snoopy is enabled.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    let mut envValue: *const libc::c_char = 0 as *const libc::c_char;
    envValue = getenv(b"LD_PRELOAD\0" as *const u8 as *const libc::c_char);
    if envValue.is_null() {
        printMessage(
            b"LD_PRELOAD environment var:    NOT OK - Not set.\0" as *const u8
                as *const libc::c_char,
        );
    } else if !(strstr(envValue, b"libsnoopy.so\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        printMessage(
            b"LD_PRELOAD environment var:    OK - Snoopy reference found.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printMessage(
            b"LD_PRELOAD environment var:    NOT SET - Set, but no Snoopy reference found.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut solibsOrder: solibsOrderT = {
        let mut init = solibsOrderT {
            libcPos: 0 as libc::c_int,
            libsnoopyPos: 0 as libc::c_int,
        };
        init
    };
    dl_iterate_phdr(
        Some(
            dl_iterate_phdr_callback
                as unsafe extern "C" fn(
                    *mut dl_phdr_info,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut solibsOrder as *mut solibsOrderT as *mut libc::c_void,
    );
    if solibsOrder.libcPos == 0 as libc::c_int {
        printWarning(
            b"Current process (shared libs): Unable to find libc.so.6\0" as *const u8
                as *const libc::c_char,
        );
    } else if solibsOrder.libsnoopyPos == 0 as libc::c_int {
        printMessage(
            b"Current process (shared libs): NOT OK - Unable to find libsnoopy.so.\0"
                as *const u8 as *const libc::c_char,
        );
    } else if solibsOrder.libsnoopyPos < solibsOrder.libcPos {
        printMessage(
            b"Current process (shared libs): OK - libsnoopy.so is loaded in front of libc.so.6.\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        printMessage(
            b"Current process (shared libs): NOT OK - libsnoopy.so is loaded _after_ libc.so.6.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    let mut execveAddrLibc: execveAddrT = None;
    let mut execveAddrCur: execveAddrT = None;
    let mut libcPath: *const libc::c_char = b"libc.so.6\0" as *const u8
        as *const libc::c_char;
    let mut libcHandle: *mut libc::c_void = dlopen(libcPath, 0x1 as libc::c_int);
    let ref mut fresh0 = *(&mut execveAddrCur as *mut execveAddrT
        as *mut *mut libc::c_void);
    *fresh0 = dlsym(
        0 as *mut libc::c_void,
        b"execve\0" as *const u8 as *const libc::c_char,
    );
    let ref mut fresh1 = *(&mut execveAddrLibc as *mut execveAddrT
        as *mut *mut libc::c_void);
    *fresh1 = dlsym(libcHandle, b"execve\0" as *const u8 as *const libc::c_char);
    if execveAddrLibc == execveAddrCur {
        printMessage(
            b"Current process (execve addr): NOT OK - execve() symbol not overloaded, Snoopy is not loaded.\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        printMessage(
            b"Current process (execve addr): OK - execve() symbol address changed, looks like Snoopy is loaded.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
