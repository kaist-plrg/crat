use ::libc;
extern "C" {
    pub type PKVM;
    pub type PkHandle;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn pkRegisterBuiltinFn(
        vm: *mut PKVM,
        name: *const libc::c_char,
        fn_0: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
    fn pkRealloc(
        vm: *mut PKVM,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn pkReleaseHandle(vm: *mut PKVM, handle: *mut PkHandle);
    fn pkNewModule(vm: *mut PKVM, name: *const libc::c_char) -> *mut PkHandle;
    fn pkRegisterModule(vm: *mut PKVM, module: *mut PkHandle);
    fn pkModuleAddFunction(
        vm: *mut PKVM,
        module: *mut PkHandle,
        name: *const libc::c_char,
        fptr: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
    fn pkNewClass(
        vm: *mut PKVM,
        name: *const libc::c_char,
        base_class: *mut PkHandle,
        module: *mut PkHandle,
        new_fn: pkNewInstanceFn,
        delete_fn: pkDeleteInstanceFn,
        docstring: *const libc::c_char,
    ) -> *mut PkHandle;
    fn pkClassAddMethod(
        vm: *mut PKVM,
        cls: *mut PkHandle,
        name: *const libc::c_char,
        fptr: pkNativeFn,
        arity: libc::c_int,
        docstring: *const libc::c_char,
    );
    fn pkModuleAddSource(
        vm: *mut PKVM,
        module: *mut PkHandle,
        source: *const libc::c_char,
    );
    fn pkSetRuntimeError(vm: *mut PKVM, message: *const libc::c_char);
    fn pkSetRuntimeErrorFmt(vm: *mut PKVM, fmt: *const libc::c_char, _: ...);
    fn pkGetSelf(vm: *const PKVM) -> *mut libc::c_void;
    fn pkGetArgc(vm: *const PKVM) -> libc::c_int;
    fn pkCheckArgcRange(
        vm: *mut PKVM,
        argc: libc::c_int,
        min: libc::c_int,
        max: libc::c_int,
    ) -> bool;
    fn pkValidateSlotNumber(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut libc::c_double,
    ) -> bool;
    fn pkValidateSlotInteger(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut int32_t,
    ) -> bool;
    fn pkValidateSlotString(
        vm: *mut PKVM,
        slot: libc::c_int,
        value: *mut *const libc::c_char,
        length: *mut uint32_t,
    ) -> bool;
    fn pkReserveSlots(vm: *mut PKVM, count: libc::c_int);
    fn pkSetSlotNumber(vm: *mut PKVM, index: libc::c_int, value: libc::c_double);
    fn pkSetSlotString(vm: *mut PKVM, index: libc::c_int, value: *const libc::c_char);
    fn pkSetSlotStringLength(
        vm: *mut PKVM,
        index: libc::c_int,
        value: *const libc::c_char,
        length: uint32_t,
    );
    fn pkSetSlotHandle(vm: *mut PKVM, index: libc::c_int, handle: *mut PkHandle);
    fn pkNewInstance(
        vm: *mut PKVM,
        cls: libc::c_int,
        index: libc::c_int,
        argc: libc::c_int,
        argv: libc::c_int,
    ) -> bool;
    fn pkCallMethod(
        vm: *mut PKVM,
        instance: libc::c_int,
        method: *const libc::c_char,
        argc: libc::c_int,
        argv: libc::c_int,
        ret: libc::c_int,
    ) -> bool;
    fn pkGetAttribute(
        vm: *mut PKVM,
        instance: libc::c_int,
        name: *const libc::c_char,
        index: libc::c_int,
    ) -> bool;
    fn pkSetAttribute(
        vm: *mut PKVM,
        instance: libc::c_int,
        name: *const libc::c_char,
        value: libc::c_int,
    ) -> bool;
    fn pkImportModule(
        vm: *mut PKVM,
        path: *const libc::c_char,
        index: libc::c_int,
    ) -> bool;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn pkByteBufferClear(self_0: *mut pkByteBuffer, vm: *mut PKVM);
    fn pkByteBufferInit(self_0: *mut pkByteBuffer);
    fn pkByteBufferWrite(self_0: *mut pkByteBuffer, vm: *mut PKVM, data: uint8_t);
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type pkNativeFn = Option::<unsafe extern "C" fn(*mut PKVM) -> ()>;
pub type pkNewInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void,
>;
pub type pkDeleteInstanceFn = Option::<
    unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkByteBuffer {
    pub data: *mut uint8_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub type FileAccessMode = libc::c_uint;
pub const FMODE_APPEND_BIN_EXT: FileAccessMode = 28;
pub const FMODE_WRITE_BIN_EXT: FileAccessMode = 26;
pub const FMODE_READ_BIN_EXT: FileAccessMode = 25;
pub const FMODE_APPEND_BIN: FileAccessMode = 20;
pub const FMODE_WRITE_BIN: FileAccessMode = 18;
pub const FMODE_READ_BIN: FileAccessMode = 17;
pub const FMODE_APPEND_EXT: FileAccessMode = 12;
pub const FMODE_WRITE_EXT: FileAccessMode = 10;
pub const FMODE_READ_EXT: FileAccessMode = 9;
pub const _FMODE_BIN: FileAccessMode = 16;
pub const _FMODE_EXT: FileAccessMode = 8;
pub const FMODE_APPEND: FileAccessMode = 4;
pub const FMODE_WRITE: FileAccessMode = 2;
pub const FMODE_READ: FileAccessMode = 1;
pub const FMODE_NONE: FileAccessMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct File {
    pub fp: *mut FILE,
    pub mode: FileAccessMode,
    pub closed: bool,
}
static mut _pk_doc__ioWrite: *const libc::c_char = b"io.write(stream:Var, bytes:String) -> Null\n\nWarning: the function is subjected to be changed anytime soon.\nWrite [bytes] string to the stream. stream should be any of io.stdin, io.stdout, io.stderr.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _ioWrite(mut vm: *mut PKVM) {
    let mut stream: libc::c_double = 0.;
    if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut stream) {
        return;
    }
    if stream != 0 as libc::c_int as libc::c_double
        && stream != 1 as libc::c_int as libc::c_double
        && stream != 2 as libc::c_int as libc::c_double
    {
        pkSetRuntimeErrorFmt(
            vm,
            b"Invalid stream (%g). Only use any of io.stdin, io.stdout, io.stderr.\0"
                as *const u8 as *const libc::c_char,
            stream,
        );
        return;
    }
    let mut bytes: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: uint32_t = 0;
    if !pkValidateSlotString(vm, 2 as libc::c_int, &mut bytes, &mut length) {
        return;
    }
    match stream as libc::c_int {
        0 => {
            pkSetRuntimeError(
                vm,
                b"Cannot write to stdin.\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        1 => {
            fwrite(
                bytes as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                length as libc::c_ulong,
                stdout,
            );
            return;
        }
        2 => {
            fwrite(
                bytes as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                length as libc::c_ulong,
                stderr,
            );
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn _ioFlush(mut vm: *mut PKVM) {
    fflush(stdout);
}
static mut _pk_doc__ioFlush: *const libc::c_char = b"io.flush() -> Null\n\nWarning: the function is subjected to be changed anytime soon.\nFlush stdout buffer.\0"
    as *const u8 as *const libc::c_char;
static mut _pk_doc__ioGetc: *const libc::c_char = b"io.getc() -> String\n\nRead a single character from stdin and return it.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _ioGetc(mut vm: *mut PKVM) {
    let mut c: libc::c_char = fgetc(stdin) as libc::c_char;
    pkSetSlotStringLength(vm, 0 as libc::c_int, &mut c, 1 as libc::c_int as uint32_t);
}
pub unsafe extern "C" fn _fileNew(mut vm: *mut PKVM) -> *mut libc::c_void {
    let mut file: *mut File = pkRealloc(
        vm,
        0 as *mut libc::c_void,
        ::std::mem::size_of::<File>() as libc::c_ulong,
    ) as *mut File;
    (*file).closed = 1 as libc::c_int != 0;
    (*file).mode = FMODE_NONE;
    (*file).fp = 0 as *mut FILE;
    return file as *mut libc::c_void;
}
pub unsafe extern "C" fn _fileDelete(mut vm: *mut PKVM, mut ptr: *mut libc::c_void) {
    let mut file: *mut File = ptr as *mut File;
    if !(*file).closed {
        fclose((*file).fp) != 0 as libc::c_int;
        (*file).closed = 1 as libc::c_int != 0;
        (*file).fp = 0 as *mut FILE;
    }
    pkRealloc(vm, file as *mut libc::c_void, 0 as libc::c_int as size_t);
}
static mut _pk_doc__fileOpen: *const libc::c_char = b"io.File.open(path:String, mode:String) -> Null\n\nOpens a file at the [path] with the [mode]. Path should be either absolute or relative to the current working directory. and [mode] can be'r', 'w', 'a' in combination with 'b' (binary) and/or '+' (extended).\n```\n mode | If already exists | If does not exist |\n -----+-------------------+-------------------|\n 'r'  |  read from start  |   failure to open |\n 'w'  |  destroy contents |   create new      |\n 'a'  |  write to end     |   create new      |\n 'r+' |  read from start  |   error           |\n 'w+' |  destroy contents |   create new      |\n 'a+' |  write to end     |   create new      |\n```\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fileOpen(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = pkGetArgc(vm);
    if !pkCheckArgcRange(vm, argc, 1 as libc::c_int, 2 as libc::c_int) {
        return;
    }
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut path, 0 as *mut uint32_t) {
        return;
    }
    let mut mode_str: *const libc::c_char = b"r\0" as *const u8 as *const libc::c_char;
    let mut mode: FileAccessMode = FMODE_READ;
    if argc == 2 as libc::c_int {
        if !pkValidateSlotString(
            vm,
            2 as libc::c_int,
            &mut mode_str,
            0 as *mut uint32_t,
        ) {
            return;
        }
        if strcmp(mode_str, b"r\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_READ;
        } else if strcmp(mode_str, b"w\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_WRITE;
        } else if strcmp(mode_str, b"a\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_APPEND;
        } else if strcmp(mode_str, b"r+\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_READ_EXT;
        } else if strcmp(mode_str, b"w+\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_WRITE_EXT;
        } else if strcmp(mode_str, b"a+\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_APPEND_EXT;
        } else if strcmp(mode_str, b"rb\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_READ_BIN;
        } else if strcmp(mode_str, b"wb\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_WRITE_BIN;
        } else if strcmp(mode_str, b"ab\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_APPEND_BIN;
        } else if strcmp(mode_str, b"rb+\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_READ_BIN_EXT;
        } else if strcmp(mode_str, b"wb+\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_WRITE_BIN_EXT;
        } else if strcmp(mode_str, b"ab+\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode = FMODE_APPEND_BIN_EXT;
        } else {
            pkSetRuntimeError(
                vm,
                b"Invalid mode string.\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    let mut fp: *mut FILE = fopen(path, mode_str);
    if !fp.is_null() {
        let mut self_0: *mut File = pkGetSelf(vm) as *mut File;
        (*self_0).fp = fp;
        (*self_0).mode = mode;
        (*self_0).closed = 0 as libc::c_int != 0;
    } else {
        pkSetRuntimeError(
            vm,
            b"Error opening the file.\0" as *const u8 as *const libc::c_char,
        );
    };
}
static mut _pk_doc__fileRead: *const libc::c_char = b"io.File.read(count:Number) -> String\n\nReads [count] number of bytes from the file and return it as String.If the count is -1 it'll read till the end of file and return it.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fileRead(mut vm: *mut PKVM) {
    let mut is_read_failed: bool = false;
    let mut argc: libc::c_int = pkGetArgc(vm);
    if !pkCheckArgcRange(vm, argc, 0 as libc::c_int, 1 as libc::c_int) {
        return;
    }
    let mut count: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if argc == 1 as libc::c_int {
        let mut count_: libc::c_double = 0.;
        if !pkValidateSlotNumber(vm, 1 as libc::c_int, &mut count_) {
            return;
        }
        if floor(count_) != count_ {
            pkSetRuntimeError(
                vm,
                b"Expected an integer.\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        if count_ < 0 as libc::c_int as libc::c_double
            && count_ != -(1 as libc::c_int) as libc::c_double
        {
            pkSetRuntimeError(
                vm,
                b"Read bytes count should be either > 0 or == -1.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        count = count_ as libc::c_long;
    }
    let mut file: *mut File = pkGetSelf(vm) as *mut File;
    if (*file).closed {
        pkSetRuntimeError(
            vm,
            b"Cannot read from a closed file.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*file).mode as libc::c_uint & FMODE_READ as libc::c_int as libc::c_uint == 0
        && _FMODE_EXT as libc::c_int as libc::c_uint & (*file).mode as libc::c_uint == 0
    {
        pkSetRuntimeError(
            vm,
            b"File is not readable.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if count == -(1 as libc::c_int) as libc::c_long {
        let mut current: libc::c_long = ftell((*file).fp);
        fseek((*file).fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
        count = ftell((*file).fp);
        fseek((*file).fp, current, 0 as libc::c_int);
    }
    let mut buff: *mut libc::c_char = pkRealloc(
        vm,
        0 as *mut libc::c_void,
        (count as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    clearerr((*file).fp);
    let mut read: size_t = fread(
        buff as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        count as libc::c_ulong,
        (*file).fp,
    );
    if ferror((*file).fp) != 0 {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.fread errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
    } else {
        is_read_failed = read > count as libc::c_ulong;
        if !is_read_failed {
            *buff.offset(read as isize) = '\0' as i32 as libc::c_char;
        }
        if read == 0 as libc::c_int as libc::c_ulong {
            pkSetSlotStringLength(
                vm,
                0 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint32_t,
            );
        } else if is_read_failed {
            pkSetRuntimeError(
                vm,
                b"C.fread() failed.\0" as *const u8 as *const libc::c_char,
            );
        } else {
            pkSetSlotStringLength(vm, 0 as libc::c_int, buff, read as uint32_t);
        }
    }
    pkRealloc(vm, buff as *mut libc::c_void, 0 as libc::c_int as size_t);
}
static mut _pk_doc__fileGetLine: *const libc::c_char = b"io.File.getline() -> String\n\nReads a line from the file and return it as string. This function can only be used for files that are opened with text mode.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fileGetLine(mut vm: *mut PKVM) {
    let mut current_block: u64;
    let mut file: *mut File = pkGetSelf(vm) as *mut File;
    if (*file).closed {
        pkSetRuntimeError(
            vm,
            b"Cannot read from a closed file.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*file).mode as libc::c_uint & FMODE_READ as libc::c_int as libc::c_uint == 0
        && _FMODE_EXT as libc::c_int as libc::c_uint & (*file).mode as libc::c_uint == 0
    {
        pkSetRuntimeError(
            vm,
            b"File is not readable.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*file).mode as libc::c_uint & _FMODE_BIN as libc::c_int as libc::c_uint != 0 {
        pkSetRuntimeError(
            vm,
            b"Cannot getline binary files.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut buff: pkByteBuffer = pkByteBuffer {
        data: 0 as *mut uint8_t,
        count: 0,
        capacity: 0,
    };
    pkByteBufferInit(&mut buff);
    let mut c: libc::c_char = 0;
    loop {
        c = fgetc((*file).fp) as libc::c_char;
        if c as libc::c_int == -(1 as libc::c_int) {
            if !(ferror((*file).fp) != 0) {
                current_block = 7149356873433890176;
                break;
            }
            pkSetRuntimeErrorFmt(
                vm,
                b"C.fgetc errno:%i - %s.\0" as *const u8 as *const libc::c_char,
                *__errno_location(),
                strerror(*__errno_location()),
            );
            current_block = 16781988807444545348;
            break;
        } else {
            pkByteBufferWrite(&mut buff, vm, c as uint8_t);
            if c as libc::c_int == '\n' as i32 {
                current_block = 7149356873433890176;
                break;
            }
        }
    }
    match current_block {
        7149356873433890176 => {
            pkSetSlotStringLength(
                vm,
                0 as libc::c_int,
                buff.data as *const libc::c_char,
                buff.count,
            );
        }
        _ => {}
    }
    pkByteBufferClear(&mut buff, vm);
}
static mut _pk_doc__fileWrite: *const libc::c_char = b"io.File.write(data:String) -> Null\n\nWrite the [data] to the file. Since pocketlang string support any validbyte value in it's string, binary data can also be written with strings.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fileWrite(mut vm: *mut PKVM) {
    let mut file: *mut File = pkGetSelf(vm) as *mut File;
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut length: uint32_t = 0;
    if !pkValidateSlotString(vm, 1 as libc::c_int, &mut text, &mut length) {
        return;
    }
    if (*file).closed {
        pkSetRuntimeError(
            vm,
            b"Cannot write to a closed file.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*file).mode as libc::c_uint & FMODE_WRITE as libc::c_int as libc::c_uint == 0
        && (*file).mode as libc::c_uint & FMODE_APPEND as libc::c_int as libc::c_uint
            == 0
        && (*file).mode as libc::c_uint & _FMODE_EXT as libc::c_int as libc::c_uint == 0
    {
        pkSetRuntimeError(
            vm,
            b"File is not writable.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    clearerr((*file).fp);
    fwrite(
        text as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        length as size_t,
        (*file).fp,
    );
    if ferror((*file).fp) != 0 {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.fwrite errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        return;
    }
}
static mut _pk_doc__fileClose: *const libc::c_char = b"io.File.close()\n\nCloses the opend file.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fileClose(mut vm: *mut PKVM) {
    let mut file: *mut File = pkGetSelf(vm) as *mut File;
    if (*file).closed {
        pkSetRuntimeError(
            vm,
            b"File already closed.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if fclose((*file).fp) != 0 as libc::c_int {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.fclose errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        return;
    }
    (*file).fp = 0 as *mut FILE;
    (*file).closed = 1 as libc::c_int != 0;
}
static mut _pk_doc__fileSeek: *const libc::c_char = b"io.File.seek(offset:Number, whence:Number) -> Null\n\nMove the file read/write offset. where [offset] is the offset from [whence] which should be any of the bellow three.\n  0: Begining of the file.\n  1: Current position.\n  2: End of the file.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fileSeek(mut vm: *mut PKVM) {
    let mut argc: libc::c_int = pkGetArgc(vm);
    if !pkCheckArgcRange(vm, argc, 1 as libc::c_int, 2 as libc::c_int) {
        return;
    }
    let mut offset: int32_t = 0 as libc::c_int;
    let mut whence: int32_t = 0 as libc::c_int;
    if !pkValidateSlotInteger(vm, 1 as libc::c_int, &mut offset) {
        return;
    }
    if argc == 2 as libc::c_int {
        if !pkValidateSlotInteger(vm, 2 as libc::c_int, &mut whence) {
            return;
        }
        if whence < 0 as libc::c_int || whence > 2 as libc::c_int {
            pkSetRuntimeErrorFmt(
                vm,
                b"Invalid whence value (%i).\0" as *const u8 as *const libc::c_char,
                whence,
            );
            return;
        }
    }
    let mut file: *mut File = pkGetSelf(vm) as *mut File;
    if (*file).closed {
        pkSetRuntimeError(
            vm,
            b"Cannot seek from a closed file.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if fseek((*file).fp, offset as libc::c_long, whence) != 0 as libc::c_int {
        pkSetRuntimeErrorFmt(
            vm,
            b"C.fseek errno:%i - %s.\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        return;
    }
}
static mut _pk_doc__fileTell: *const libc::c_char = b"io.File.tell() -> Number\n\nReturns the read/write position of the file.\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _fileTell(mut vm: *mut PKVM) {
    let mut file: *mut File = pkGetSelf(vm) as *mut File;
    if (*file).closed {
        pkSetRuntimeError(
            vm,
            b"Cannot tell from a closed file.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    pkSetSlotNumber(vm, 0 as libc::c_int, ftell((*file).fp) as libc::c_double);
}
static mut _pk_doc__open: *const libc::c_char = b"open(path:String, mode:String) -> Null\n\nOpens a file at the [path] with the [mode]. Path should be either absolute or relative to the current working directory. and [mode] can be'r', 'w', 'a' in combination with 'b' (binary) and/or '+' (extended).\n```\n mode | If already exists | If does not exist |\n -----+-------------------+-------------------|\n 'r'  |  read from start  |   failure to open |\n 'w'  |  destroy contents |   create new      |\n 'a'  |  write to end     |   create new      |\n 'r+' |  read from start  |   error           |\n 'w+' |  destroy contents |   create new      |\n 'a+' |  write to end     |   create new      |\n```\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn _open(mut vm: *mut PKVM) {
    pkReserveSlots(vm, 3 as libc::c_int);
    let mut argc: libc::c_int = pkGetArgc(vm);
    if !pkCheckArgcRange(vm, argc, 1 as libc::c_int, 2 as libc::c_int) {
        return;
    }
    if argc == 1 as libc::c_int {
        pkSetSlotString(
            vm,
            2 as libc::c_int,
            b"r\0" as *const u8 as *const libc::c_char,
        );
    }
    if !pkImportModule(
        vm,
        b"io\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) {
        return;
    }
    if !pkGetAttribute(
        vm,
        0 as libc::c_int,
        b"File\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) {
        return;
    }
    if !pkNewInstance(
        vm,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ) {
        return;
    }
    if !pkCallMethod(
        vm,
        0 as libc::c_int,
        b"open\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
    ) {
        return;
    }
}
pub unsafe extern "C" fn registerModuleIO(mut vm: *mut PKVM) {
    let mut io: *mut PkHandle = pkNewModule(
        vm,
        b"io\0" as *const u8 as *const libc::c_char,
    );
    pkRegisterBuiltinFn(
        vm,
        b"open\0" as *const u8 as *const libc::c_char,
        Some(_open as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__open,
    );
    pkReserveSlots(vm, 2 as libc::c_int);
    pkSetSlotHandle(vm, 0 as libc::c_int, io);
    pkSetSlotNumber(vm, 1 as libc::c_int, 0 as libc::c_int as libc::c_double);
    pkSetAttribute(
        vm,
        0 as libc::c_int,
        b"stdin\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    pkSetSlotNumber(vm, 1 as libc::c_int, 1 as libc::c_int as libc::c_double);
    pkSetAttribute(
        vm,
        0 as libc::c_int,
        b"stdout\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    pkSetSlotNumber(vm, 1 as libc::c_int, 2 as libc::c_int as libc::c_double);
    pkSetAttribute(
        vm,
        0 as libc::c_int,
        b"stderr\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    pkModuleAddFunction(
        vm,
        io,
        b"write\0" as *const u8 as *const libc::c_char,
        Some(_ioWrite as unsafe extern "C" fn(*mut PKVM) -> ()),
        2 as libc::c_int,
        _pk_doc__ioWrite,
    );
    pkModuleAddFunction(
        vm,
        io,
        b"flush\0" as *const u8 as *const libc::c_char,
        Some(_ioFlush as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__ioFlush,
    );
    pkModuleAddFunction(
        vm,
        io,
        b"getc\0" as *const u8 as *const libc::c_char,
        Some(_ioGetc as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__ioGetc,
    );
    let mut cls_file: *mut PkHandle = pkNewClass(
        vm,
        b"File\0" as *const u8 as *const libc::c_char,
        0 as *mut PkHandle,
        io,
        Some(_fileNew as unsafe extern "C" fn(*mut PKVM) -> *mut libc::c_void),
        Some(_fileDelete as unsafe extern "C" fn(*mut PKVM, *mut libc::c_void) -> ()),
        b"A simple file type.\0" as *const u8 as *const libc::c_char,
    );
    pkClassAddMethod(
        vm,
        cls_file,
        b"open\0" as *const u8 as *const libc::c_char,
        Some(_fileOpen as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__fileOpen,
    );
    pkClassAddMethod(
        vm,
        cls_file,
        b"read\0" as *const u8 as *const libc::c_char,
        Some(_fileRead as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__fileRead,
    );
    pkClassAddMethod(
        vm,
        cls_file,
        b"write\0" as *const u8 as *const libc::c_char,
        Some(_fileWrite as unsafe extern "C" fn(*mut PKVM) -> ()),
        1 as libc::c_int,
        _pk_doc__fileWrite,
    );
    pkClassAddMethod(
        vm,
        cls_file,
        b"getline\0" as *const u8 as *const libc::c_char,
        Some(_fileGetLine as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__fileGetLine,
    );
    pkClassAddMethod(
        vm,
        cls_file,
        b"close\0" as *const u8 as *const libc::c_char,
        Some(_fileClose as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__fileClose,
    );
    pkClassAddMethod(
        vm,
        cls_file,
        b"seek\0" as *const u8 as *const libc::c_char,
        Some(_fileSeek as unsafe extern "C" fn(*mut PKVM) -> ()),
        -(1 as libc::c_int),
        _pk_doc__fileSeek,
    );
    pkClassAddMethod(
        vm,
        cls_file,
        b"tell\0" as *const u8 as *const libc::c_char,
        Some(_fileTell as unsafe extern "C" fn(*mut PKVM) -> ()),
        0 as libc::c_int,
        _pk_doc__fileTell,
    );
    pkReleaseHandle(vm, cls_file);
    pkModuleAddSource(
        vm,
        io,
        b"def readfile(filepath)\n  \"Reads a file and return it's content as string\"  fp = File()\n  fp.open(filepath, 'r')\n  text = fp.read()\n  fp.close()\n  return text\nend\n\0"
            as *const u8 as *const libc::c_char,
    );
    pkRegisterModule(vm, io);
    pkReleaseHandle(vm, io);
}
